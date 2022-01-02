use std::fs::File;
/// file.rs
///
use std::io;
use std::io::{Cursor, Read, Seek};
use std::path::{Path, PathBuf};

use byteorder::{LittleEndian, ReadBytesExt};
use cfb;

use crate::aaf::classes::{AAFObject, Header};
use crate::interchange_object::InterchangeObjectDescriptor;
use crate::properties::*;
use crate::types::*;

const AAF_FILE_HEADER_PID: OMPropertyId = 0x0002;
const AAF_FILE_METADICTIONARY_PID: OMPropertyId = 0x0001;
// AAF File uuid b3b398a5-1c90-11d4-8053-080036210804

pub struct AAFEntry {
    parent: InterchangeObjectDescriptor,
    property: OMPropertyId,
    value: Option<PropertyValue>,
    depth: usize,
}

impl AAFEntry {
    pub fn parent(&self) -> &InterchangeObjectDescriptor {
        &self.parent
    }

    pub fn property_id(&self) -> OMPropertyId {
        self.property.clone()
    }

    pub fn value(&self) -> &Option<PropertyValue> {
        &self.value
    }

    pub fn depth(&self) -> usize {
        self.depth.clone()
    }
}

pub struct AAFPropertyIterator<'a, F> {
    file: &'a mut AAFFile<F>,
    stack: Vec<AAFEntry>,
}

impl<'a, F> AAFPropertyIterator<'a, F>
where
    F: Read + Seek,
{
    pub(crate) fn new(file: &'a mut AAFFile<F>, root_object: InterchangeObjectDescriptor) -> Self {
        let mut retval = AAFPropertyIterator {
            file,
            stack: vec![],
        };

        retval.fill_stack(&root_object, 0);
        retval
    }

    fn fill_stack(&mut self, parent: &InterchangeObjectDescriptor, depth: usize) {
        for pid in self.file.all_property_ids(parent) {
            let pv = self.file.get_value(&parent, pid);
            self.stack.push(AAFEntry {
                parent: parent.clone(),
                property: pid,
                value: pv,
                depth: depth + 1,
            })
        }
    }
}

impl<'a, F> Iterator for AAFPropertyIterator<'_, F>
where
    F: Read + Seek,
{
    type Item = AAFEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(move |e| {
            match &e.value {
                Some(PropertyValue::Single(obj)) => {
                    self.fill_stack(&obj, e.depth());
                }
                Some(PropertyValue::Vector(list)) => {
                    for obj in list.into_iter().rev() {
                        self.fill_stack(&obj, e.depth());
                    }
                }
                Some(PropertyValue::Set(list)) => {
                    for obj in list.into_iter().rev() {
                        self.fill_stack(&obj, e.depth());
                    }
                }
                _ => {}
            }
            e
        })
    }
}

/// An AAF file.
pub struct AAFFile<F> {
    f: cfb::CompoundFile<F>,
    weakref_table: Vec<Vec<OMPropertyId>>,
}

impl<F> AAFFile<F> {
    /// An object at a path.
    ///
    /// Panics: If `path` does not exist in storage
    fn object(&self, path: PathBuf) -> InterchangeObjectDescriptor {
        self.f
            .entry(path)
            .map(|entry| InterchangeObjectDescriptor {
                auid: *entry.clsid(),
                path: entry.path().into(),
            })
            .expect("Failed to locate object by path")
    }

    /// The root object.
    pub fn root_object(&self) -> InterchangeObjectDescriptor {
        self.object(PathBuf::from("/"))
    }
}

impl AAFFile<File> {
    /// Open an AAF file at `path`
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<AAFFile<File>> {
        let cfb = cfb::open(path)?;
        Ok(Self::with_cfb(cfb))
    }
}

impl<F: Read + Seek> AAFFile<F> {
    /// Walk the AAF object graph
    ///
    ///
    pub fn walk_properties(&mut self) -> AAFPropertyIterator<F> {
        AAFPropertyIterator::new(self, self.root_object())
    }

    pub fn header(mut self) -> Header<F> {
        if let Some(PropertyValue::Single(obj)) =
            self.get_value(&self.root_object(), AAF_FILE_HEADER_PID)
        {
            Header::make(self, obj)
        } else {
            panic!()
        }
    }

    // pub fn meta_dictionary(&mut self) -> MetaDictionary<F> {
    //     if let Some(PropertyValue::Single(obj)) =
    //         self.get_value(&self.root_object(), AAF_FILE_METADICTIONARY_PID) {
    //         MetaDictionary {
    //             file: self,
    //             object: obj
    //         }
    //     } else {
    //         panic!()
    //     }
    // }

    /// All of the `OMPropertyId`s available in the AAFFile for the given object
    pub fn all_property_ids(&mut self, object: &InterchangeObjectDescriptor) -> Vec<OMPropertyId> {
        let props = self.raw_properties(&object);
        props.into_iter().map(|p| p.pid).collect()
    }

    /// Get the value of an object property.
    pub fn get_value(
        &mut self,
        object: &InterchangeObjectDescriptor,
        pid: OMPropertyId,
    ) -> Option<PropertyValue> {
        let prop = self.raw_property_by_pid(object, pid)?;
        self.resolve_property_value(object, &prop)
    }

    /// A new `AAFFile` with a `cfb::CompoundFile`
    fn with_cfb(mut cfb: cfb::CompoundFile<F>) -> Self {
        let weakref_table = Self::weak_refs_table(&mut cfb);
        // let session = Session { };
        Self {
            f: cfb,
            weakref_table: weakref_table,
            // session,
        }
    }

    /// Retrive and parse the `referenced properties` table for a given cfb file
    fn weak_refs_table(f: &mut cfb::CompoundFile<F>) -> Vec<Vec<OMPropertyId>> {
        let ref_props_stream = f
            .open_stream(PathBuf::from("/referenced properties"))
            .expect("Failed to open referenced properties stream");

        ReferencedPropertiesTable::from_stream(ref_props_stream).pid_paths
    }

    fn resolve_weak_reference(&mut self, weak_ref: WeakObjectReference) -> PropertyValue {
        let pid_path = self.weakref_table[weak_ref.tag as usize].to_vec();

        let mut obj = self.root_object();

        for pid in &pid_path[0..pid_path.len() - 1] {
            let p1 = self.raw_property_by_pid(&obj, *pid).unwrap();
            let pv = self.resolve_property_value(&obj, &p1).unwrap();
            obj = pv.unwrap_object();
        }

        let pfinal = self
            .raw_property_by_pid(&obj, pid_path[pid_path.len() - 1])
            .unwrap();
        let found = self
            .resolve_property_value(&obj, &pfinal)
            .unwrap()
            .unwrap_set()
            .into_iter()
            .find(|i| {
                let ident = self
                    .raw_property_by_pid(&i, weak_ref.key_pid)
                    .unwrap()
                    .raw_value;
                *ident == weak_ref.identification
            })
            .unwrap();

        PropertyValue::Reference(found)
    }

    /// All of the raw properties for a given InterchangeObjectDescriptor
    fn raw_properties(&mut self, object: &InterchangeObjectDescriptor) -> Vec<RawProperty> {
        let properties_path = object.path.join("properties");
        let mut stream = self.f.open_stream(&properties_path).expect(&format!(
            "Failed to open `properties` stream for object {:?}",
            object
        ));

        let mut buf: Vec<u8> = vec![];
        stream
            .read_to_end(&mut buf)
            .expect("Error reading properties IStream");
        RawProperty::from_properties_istream(&mut buf)
    }

    /// Retrive a raw property for an InterchangeObjectDescriptor
    fn raw_property_by_pid(
        &mut self,
        object: &InterchangeObjectDescriptor,
        pid: OMPropertyId,
    ) -> Option<RawProperty> {
        self.raw_properties(object)
            .into_iter()
            .find(|p| p.pid == pid)
    }

    fn resolve_property_value(
        &mut self,
        object: &InterchangeObjectDescriptor,
        property: &RawProperty,
    ) -> Option<PropertyValue> {
        let raw_data = Self::raw_property_by_pid(self, object, property.pid)?.raw_value;

        match property.stored_form {
            SF_DATA => Some(PropertyValue::Data(raw_data)),
            SF_DATA_STREAM => {
                let decoded_name = property.raw_string_value();
                let ref_path = object.path.join(decoded_name);
                Some(PropertyValue::Stream(ref_path))
            }
            SF_STRONG_OBJECT_REF => {
                let decoded_name = property.raw_string_value();
                let ref_path = object.path.join(decoded_name);
                Some(PropertyValue::Single(self.object(ref_path)))
            }
            SF_STRONG_OBJECT_REF_VECTOR => {
                let decoded_name = property.raw_string_value();
                let index_name = format!("{} index", decoded_name);
                let index_path = object.path.join(index_name);
                let index_stream = self.f.open_stream(index_path).unwrap();
                let vector_index = StrongVectorReferenceIndex::from_istream(index_stream);

                let members = vector_index
                    .member_paths(decoded_name, &object.path)
                    .into_iter()
                    .map(|path| self.object(path))
                    .collect();

                Some(PropertyValue::Vector(members))
            }
            SF_STRONG_OBJECT_REF_SET => {
                let decoded_name = property.raw_string_value();
                let index_name = format!("{} index", decoded_name);
                let index_path = object.path.join(index_name);
                let index_stream = self.f.open_stream(index_path).unwrap();
                let set_index = StrongSetReferenceIndex::from_istream(index_stream);

                let members = set_index
                    .member_paths(decoded_name, &object.path)
                    .into_iter()
                    .map(|path| self.object(path))
                    .collect();

                Some(PropertyValue::Set(members))
            }
            SF_WEAK_OBJECT_REF => {
                let weak_ref = WeakObjectReference::from_data(&property.raw_value);
                Some(self.resolve_weak_reference(weak_ref))
            }
            SF_WEAK_OBJECT_REF_VECTOR | SF_WEAK_OBJECT_REF_SET => {
                let decoded_name = property.raw_string_value();
                let index_name = format!("{} index", decoded_name);
                let index_path = object.path.join(index_name);
                let index_stream = self.f.open_stream(index_path).unwrap();
                let weak_vec_refs =
                    WeakCollectionReference::from_istream(index_stream).into_weak_references();

                let refs = weak_vec_refs
                    .into_iter()
                    .map(|r| self.resolve_weak_reference(r).unwrap_reference())
                    .collect();
                if property.stored_form == SF_WEAK_OBJECT_REF_VECTOR {
                    Some(PropertyValue::ReferenceVector(refs))
                } else {
                    Some(PropertyValue::ReferenceSet(refs))
                }
            }
            _ => panic!("Unrecgonized stored form found."),
        }
    }
}

struct StrongVectorReferenceIndex {
    _entry_count: u32,
    _first_free_key: u32,
    _last_free_key: u32,
    local_keys: Vec<u32>,
}

impl StrongVectorReferenceIndex {
    fn from_istream<T: Read + Seek>(mut stream: T) -> Self {
        let entry_count = stream.read_u32::<LittleEndian>().unwrap() as usize;
        let first_free_key = stream.read_u32::<LittleEndian>().unwrap();
        let last_free_key = stream.read_u32::<LittleEndian>().unwrap();

        let mut local_keys = vec![0u32; entry_count];
        for i in 0..entry_count {
            let entry = stream.read_u32::<LittleEndian>().unwrap();
            local_keys[i] = entry;
        }
        StrongVectorReferenceIndex {
            _entry_count: entry_count as u32,
            _first_free_key: first_free_key,
            _last_free_key: last_free_key,
            local_keys,
        }
    }

    fn member_paths(&self, property_name: String, parent_path: &PathBuf) -> Vec<PathBuf> {
        self.local_keys
            .iter()
            .map(|i| {
                let member_name = format!("{}{{{:x}}}", property_name, i);
                parent_path.join(member_name)
            })
            .collect()
    }
}

struct StrongSetReferenceIndexEntry {
    local_key: u32,
    reference_count: u32,
    identification: Vec<u8>,
}

struct StrongSetReferenceIndex {
    _entry_count: u32,
    first_free_key: u32,
    last_free_key: u32,
    key_pid: OMPropertyId,
    key_size: OMKeySize,
    local_keys: Vec<StrongSetReferenceIndexEntry>,
}

impl StrongSetReferenceIndex {
    fn from_istream<T: Read + Seek>(mut stream: T) -> Self {
        let entry_count = stream.read_u32::<LittleEndian>().unwrap() as usize;
        let first_free_key = stream.read_u32::<LittleEndian>().unwrap();
        let last_free_key = stream.read_u32::<LittleEndian>().unwrap();
        let key_pid = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyId;
        let key_size = stream.read_u8().unwrap() as OMKeySize;

        let mut local_keys: Vec<StrongSetReferenceIndexEntry> = vec![];
        for _ in 0..entry_count {
            let local_key = stream.read_u32::<LittleEndian>().unwrap();
            let reference_count = stream.read_u32::<LittleEndian>().unwrap();
            let mut identification = vec![0; key_size as usize];
            stream.read_exact(&mut identification).unwrap();
            let obj = StrongSetReferenceIndexEntry {
                local_key,
                reference_count,
                identification,
            };
            local_keys.push(obj);
        }
        Self {
            _entry_count: entry_count as u32,
            first_free_key,
            last_free_key,
            key_pid,
            key_size,
            local_keys,
        }
    }

    fn member_paths(&self, property_name: String, parent_path: &PathBuf) -> Vec<PathBuf> {
        self.local_keys
            .iter()
            .map(|i| {
                let member_name = format!("{}{{{:x}}}", property_name, i.local_key);
                parent_path.join(member_name)
            })
            .collect()
    }
}

struct WeakObjectReference {
    tag: OMPropertyTag,
    key_pid: OMPropertyId,
    _key_size: OMKeySize,
    identification: Vec<u8>,
}

impl WeakObjectReference {
    fn from_data(data: &[u8]) -> Self {
        let cursor = Cursor::new(data);
        Self::from_istream(cursor)
    }
    fn from_istream<T: Read + Seek>(mut stream: T) -> Self {
        let tag = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyTag;
        let key_pid = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyId;
        let key_size = stream.read_u8().unwrap() as OMKeySize;
        let mut identification = vec![0u8; key_size as usize];
        stream
            .read_exact(&mut identification)
            .expect("Failed to read reference identification length");

        WeakObjectReference {
            tag,
            key_pid,
            _key_size: key_size,
            identification,
        }
    }
}

struct WeakCollectionReference {
    entry_count: u32,
    tag: OMPropertyTag,
    key_pid: OMPropertyId,
    key_size: OMKeySize,
    identification_list: Vec<Vec<u8>>,
}

impl WeakCollectionReference {
    fn from_istream<T: Read + Seek>(mut stream: T) -> Self {
        let entry_count = stream.read_u32::<LittleEndian>().unwrap();
        let tag = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyTag;
        let key_pid = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyId;
        let key_size = stream.read_u8().unwrap() as OMKeySize;

        let mut identification_list = vec![];

        for _ in 0..entry_count {
            let mut identification = vec![0u8; key_size as usize];
            stream
                .read_exact(&mut identification)
                .expect("Failed to read all WeakVectorReference fields");

            identification_list.push(identification);
        }

        WeakCollectionReference {
            entry_count,
            tag,
            key_pid,
            key_size,
            identification_list,
        }
    }

    fn into_weak_references(self) -> Vec<WeakObjectReference> {
        let mut retval = vec![];
        for ident in self.identification_list.into_iter() {
            retval.push(WeakObjectReference {
                tag: self.tag,
                key_pid: self.key_pid,
                _key_size: self.key_size,
                identification: ident,
            })
        }
        retval
    }
}

struct ReferencedPropertiesTable {
    byte_order: OMByteOrder,
    path_count: OMPropertyCount,
    pid_count: u32,
    pid_paths: Vec<Vec<OMPropertyId>>,
}

impl ReferencedPropertiesTable {
    pub fn from_stream<T: Read + Seek>(mut stream: T) -> Self {
        let byte_order = stream.read_u8().unwrap() as OMByteOrder;
        assert_eq!(byte_order, 0x4c, "BOM is invalid");

        let path_count = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyCount;
        let pid_count = stream.read_u32::<LittleEndian>().unwrap();

        let mut pid_paths: Vec<Vec<OMPropertyId>> = vec![];
        let mut this_path: Vec<OMPropertyId> = vec![];

        for _ in 0..pid_count {
            let this_pid = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyId;

            if this_pid == 0x0000u16 {
                pid_paths.push(this_path);
                this_path = vec![];
            } else {
                this_path.push(this_pid);
            }
        }

        assert_eq!(
            path_count as usize,
            pid_paths.len(),
            "Weak ref table has inconsistent length"
        );

        Self {
            byte_order,
            path_count,
            pid_count,
            pid_paths,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_get_root() {
        let test_path = "testmedia/AAF_Test_1/AAF_Test_1.aaf";
        let comp = cfb::open(test_path).unwrap();
        let f = AAFFile::with_cfb(comp);
        let _root = f.root_object();
    }

    #[test]
    fn test_get_properties() {
        let test_path = "testmedia/AAF_Test_1/AAF_Test_1.aaf";
        let comp = cfb::open(test_path).unwrap();
        let mut f = AAFFile::with_cfb(comp);
        let root = f.root_object();

        let props = f.raw_properties(&root);

        assert_eq!(props.len(), 2, "Incorrect number of properties detected");

        let _p1 = f.raw_property_by_pid(&root, 0x01);
        let _p2 = f.raw_property_by_pid(&root, 0x02);
    }

    #[test]
    fn test_all_properties() {
        let test_path = "testmedia/AAF_Test_1/AAF_Test_1.aaf";
        let mut f = AAFFile::open(test_path).unwrap();
        let root = f.root_object();

        let all = f.all_property_ids(&root);

        assert_eq!(all.len(), 2, "Found property ids");
    }

    #[test]
    fn test_get_header() {
        let test_path = "testmedia/AAF_Test_1/AAF_Test_1.aaf";
        let mut f = AAFFile::open(test_path).unwrap();

        let mut h = f.header();

        assert_eq!(h.byte_order(), 0x4949);

        assert_eq!(
            h.last_modified(),
            TimeStamp {
                date: (2021, 11, 9),
                time: (15, 28, 58, 0)
            }
        );

        assert_eq!(h.version(), VersionType { major: 1, minor: 1 })
    }
}
