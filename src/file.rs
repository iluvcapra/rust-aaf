use std::io;
use std::fs::File;
use std::io::{Read, Seek, Cursor};
use std::path::{Path, PathBuf};

use byteorder::{LittleEndian, ReadBytesExt};

// use encoding::all::UTF_16LE;
// use encoding::{DecoderTrap, Encoding};

use crate::interchange_object::{InterchangeObjectDescriptor, InterchangeObjectDescriptorIter};
use crate::properties::*;
use crate::types::{OMKeySize, OMPropertyId, OMByteOrder, OMPropertyCount, OMPropertyTag};

use cfb;

/// An AAF file.
pub struct AAFFile<F> {
    f: cfb::CompoundFile<F>,
    weakref_table: Vec<Vec<OMPropertyId>>
}


impl<F> AAFFile<F> { 
    pub fn interchange_object(&self, path: PathBuf) -> Option<InterchangeObjectDescriptor> {
        self.f
            .entry(path)
            .map(|entry| InterchangeObjectDescriptor {
                auid: *entry.clsid(),
                path: entry.path().into(),
            })
            .ok()
    }

    pub fn root_object(&self) -> Option<InterchangeObjectDescriptor> {
        self.interchange_object(PathBuf::from("/"))
    }
}

impl AAFFile<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<AAFFile<File>> {
        let cfb = cfb::open(path)?;
        Ok( Self::with_cfb(cfb) )
    }
}

impl<F: Read + Seek> AAFFile<F> {

    /// A new `AAFFile` with a `cfb::CompoundFile`
    pub fn with_cfb(mut cfb: cfb::CompoundFile<F>) -> Self {
        let weakref_table = Self::weak_refs_table(&mut cfb);
        Self { f: cfb , weakref_table: weakref_table }
    }

    fn weak_refs_table(f: &mut cfb::CompoundFile<F>) -> Vec<Vec<OMPropertyId>> {
        let ref_props_stream = f.open_stream(PathBuf::from("/referenced properties"))
            .expect("Failed to open referenced properties stream");
        
        ReferencedPropertiesTable::from_stream(ref_props_stream).pid_paths
    }

    pub fn raw_properties(
        &mut self, 
        object: &InterchangeObjectDescriptor
        ) -> Vec<RawProperty> {
        let properties_path = object.path.join("properties");
        let mut stream = self.f.open_stream(&properties_path).expect(&format!(
            "Failed to open `properties` stream for object {:?}",
            object
        ));

        let mut buf: Vec<u8> = vec![];
        stream.read_to_end(&mut buf).expect("Error reading properties IStream");
        RawProperty::from_properties_istream(&mut buf)
    }

    pub fn raw_property_by_pid(
        &mut self,
        object: &InterchangeObjectDescriptor,
        pid: OMPropertyId,
    ) -> Option<RawProperty> {
        self.raw_properties(object).into_iter().find(|p| p.pid == pid)
    }

    pub fn resolve_property_value(
        &mut self,
        object: &InterchangeObjectDescriptor,
        property: &RawProperty,
    ) -> PropertyValue {
        let raw_data = Self::raw_property_by_pid(self, object, property.pid)
            .unwrap().raw_value;

        match property.stored_form {
            SF_DATA => PropertyValue::Data(raw_data),
            SF_DATA_STREAM => {
                let decoded_name = property.raw_string_value();
                let ref_path = object.path.join(decoded_name);
                PropertyValue::Stream(ref_path)
            }
            SF_STRONG_OBJECT_REF => {
                let decoded_name = property.raw_string_value();
                let ref_path = object.path.join(decoded_name);
                self.interchange_object(ref_path)
                    .map(|obj| PropertyValue::Single(obj))
                    .expect("Failed to locate object by path")
            }
            SF_STRONG_OBJECT_REF_VECTOR => {
                let decoded_name = property.raw_string_value();
                let index_name = format!("{} index", decoded_name);
                let index_path = object.path.join(index_name);
                let index_stream = self.f.open_stream(index_path).unwrap();
                let vector_index = StrongVectorReferenceIndex::from_stream(index_stream);
                let members = vector_index.member_paths(decoded_name, &object.path).into_iter()
                    .map(|path| {
                        self.interchange_object(path)
                            .expect("Failed to locate index member")
                    })
                    .collect();

                PropertyValue::Vector(members)
            }
            SF_STRONG_OBJECT_REF_SET => {
                let decoded_name = property.raw_string_value();
                let index_name = format!("{} index", decoded_name);
                let index_path = object.path.join(index_name);
                let index_stream = self.f.open_stream(index_path).unwrap();
                let set_index = StrongSetReferenceIndex::from_stream(index_stream);
                let members = set_index.member_paths(decoded_name, &object.path)
                    .into_iter() 
                    .map(|path| {
                        self.interchange_object(path)
                            .expect("Failed to locate set member")
                    })
                    .collect();

                PropertyValue::Set(members)
            }
            SF_WEAK_OBJECT_REF => {
                let index_path = property.index_path(&object.path);
                let index_stream = self.f.open_stream(index_path).unwrap();
                todo!()
            }
            SF_WEAK_OBJECT_REF_VECTOR => {
                todo!()
            }
            SF_WEAK_OBJECT_REF_SET => {
                todo!()
            }
            _ => panic!("Unrecgonized stored form found."),
        }
    }
}

struct StrongVectorReferenceIndex {
    entry_count: u32,
    first_free_key: u32,
    last_free_key: u32,
    local_keys: Vec<u32>
}

impl StrongVectorReferenceIndex {
    fn from_stream<T: Read+Seek>(mut stream: T) -> Self {
        let entry_count = stream.read_u32::<LittleEndian>().unwrap() as usize;
        let first_free_key = stream.read_u32::<LittleEndian>().unwrap();
        let last_free_key = stream.read_u32::<LittleEndian>().unwrap();

        let mut local_keys = vec![0u32; entry_count];
        for i in 0..entry_count {
            let entry = stream.read_u32::<LittleEndian>().unwrap();
            local_keys[i] = entry;
        }
        StrongVectorReferenceIndex { entry_count: entry_count as u32, first_free_key, last_free_key, local_keys }
    }

    fn member_paths(&self, property_name: String, parent_path: &PathBuf) -> Vec<PathBuf> {
        self.local_keys.iter()
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
    identification: Vec<u8>
}

struct StrongSetReferenceIndex {
    entry_count: u32,
    first_free_key: u32,
    last_free_key: u32,
    key_pid: OMPropertyId,
    key_size: OMKeySize,
    local_keys: Vec<StrongSetReferenceIndexEntry>
}

impl StrongSetReferenceIndex { 
   fn from_stream<T:Read+Seek>(mut stream: T) -> Self {
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
            let obj = StrongSetReferenceIndexEntry { local_key, reference_count, identification };
            local_keys.push(obj);
        }
        Self { entry_count: entry_count as u32, first_free_key, last_free_key, key_pid, key_size, local_keys }
    }

   fn member_paths(&self, property_name: String, parent_path: &PathBuf) -> Vec<PathBuf> {
        self.local_keys.iter()
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
    key_size: OMKeySize,
    identification: Vec<u8>
}

impl WeakObjectReference {
    fn from_stream<T:Read + Seek>(mut stream: T) -> Self {
        let tag = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyTag;
        let key_pid = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyId;
        let key_size = stream.read_u8().unwrap() as OMKeySize;
        let mut identification = vec![ 0u8 ; key_size as usize];
        stream.read_exact(&mut identification)
            .expect("Failed to read reference identification length");
        
        WeakObjectReference { tag, key_pid, key_size, identification }
    }
}

struct WeakVectorReference {
    entry_count: u32,
    tag: OMPropertyTag,
    key_pid: OMPropertyId,
    key_size: OMKeySize,
    identification : Vec<u8>
}

impl WeakVectorReference {
    fn from_stream<T: Read + Seek>(mut stream: T) -> Self {
        let entry_count = stream.read_u32::<LittleEndian>().unwrap();
        let tag = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyTag;
        let key_pid = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyId;
        let key_size = stream.read_u8().unwrap() as OMKeySize;
        let mut identification = vec![ 0u8; key_size as usize ];
        stream.read_exact(&mut identification)
            .expect("Failed to read all WeakVectorReference fields");

        WeakVectorReference { entry_count , tag, key_pid, key_size, identification }
    }
}

struct ReferencedPropertiesTable {
    byte_order: OMByteOrder,
    path_count: OMPropertyCount,
    pid_count: u32,
    pid_paths: Vec<Vec<OMPropertyId>>
}

impl ReferencedPropertiesTable {
    pub fn from_stream<T: Read + Seek>(mut stream: T) -> Self {
        let byte_order = stream.read_u8().unwrap() as OMByteOrder;
        assert_eq!(byte_order, 0x4c, "BOM is invalid");

        let path_count = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyCount;
        let pid_count = stream.read_u32::<LittleEndian>().unwrap();
        
        let mut pid_paths : Vec<Vec<OMPropertyId>> = vec![];
        let mut this_path : Vec<OMPropertyId> = vec![];

        for _ in 0..pid_count {
            let this_pid = stream.read_u16::<LittleEndian>()
                .unwrap() as OMPropertyId;

            if this_pid == 0x0000u16 {
                pid_paths.push(this_path);
                this_path = vec![];
            } else {
                this_path.push(this_pid);
            }
        }

        assert_eq!(path_count as usize, pid_paths.len(),"Weak ref table has inconsistent length");
        
        Self { byte_order, path_count, pid_count, pid_paths }
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
        let _root = f.root_object().unwrap();
    }

    #[test]
    fn test_get_properties() {
        let test_path = "testmedia/AAF_Test_1/AAF_Test_1.aaf";
        let comp = cfb::open(test_path).unwrap();
        let mut f = AAFFile::with_cfb(comp);
        let root = f.root_object().unwrap();

        let props = f.raw_properties(&root);

        assert_eq!(props.len(), 2, "Incorrect number of properties detected");

        let _p1 = f.raw_property_by_pid(&root, 0x01).unwrap();
        let _p2 = f.raw_property_by_pid(&root, 0x02).unwrap();
    }
}
