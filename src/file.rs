use std::io;
use std::fs::File;
use std::io::{Read, Seek, Cursor};
use std::path::{Path, PathBuf};

use byteorder::{LittleEndian, ReadBytesExt};

use encoding::all::UTF_16LE;
use encoding::{DecoderTrap, Encoding};

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

    pub fn interchange_objects(&mut self) -> InterchangeObjectDescriptorIter<cfb::Entries> {
        let entries = self.f.walk();
        InterchangeObjectDescriptorIter(entries)
    }

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
    
    pub fn raw_properties(&mut self, 
        object: &InterchangeObjectDescriptor) 
        -> Vec<RawPropertyValue> {
            let prop_path = object.path.join("properties");
            let mut stream = self.f.open_stream(prop_path)
                .expect("Failed to locate properties IStream");
        RawPropertyValue::from_properties_stream(stream)
    }

    fn weak_refs_table(f: &mut cfb::CompoundFile<F>) -> Vec<Vec<OMPropertyId>> {
        let mut ref_props_stream = f.open_stream(PathBuf::from("/referenced properties"))
            .expect("Failed to open referenced properties stream");

        let _bom = ref_props_stream.read_u8().unwrap() as OMByteOrder;
        let path_count = ref_props_stream.read_u16::<LittleEndian>().unwrap() as OMPropertyCount;
        let pid_count = ref_props_stream.read_u32::<LittleEndian>().unwrap();
        
        let mut retval : Vec<Vec<OMPropertyId>> = vec![];
        let mut this_path : Vec<OMPropertyId> = vec![];

        for _ in 0..pid_count {
            let this_pid = ref_props_stream.read_u16::<LittleEndian>()
                .unwrap() as OMPropertyId;

            if this_pid == 0x0000u16 {
                retval.push(this_path);
                this_path = vec![];
            } else {
                this_path.push(this_pid);
            }
        }
        assert_eq!(path_count as usize, retval.len(),"Weak ref table has inconsistent length");
        retval
    }

    /// returns first free key, last free key, key list
    fn read_strong_vector_index(mut stream: cfb::Stream<F>) -> (u32, u32, Vec<u32>) {
        let entry_count = stream.read_u32::<LittleEndian>().unwrap() as usize;
        let first_free = stream.read_u32::<LittleEndian>().unwrap();
        let last_free = stream.read_u32::<LittleEndian>().unwrap();

        let mut key_list = vec![0u32; entry_count];
        for i in 0..entry_count {
            let entry = stream.read_u32::<LittleEndian>().unwrap();
            key_list[i] = entry;
        }

        (first_free, last_free, key_list)
    }

    /// return first free key, last free key, key_pid, key_list
    /// key list is a Vec of (local_key, ref_count, global_ident>)
    fn read_strong_set_index(
        mut stream: cfb::Stream<F>,
    ) -> (u32, u32, OMPropertyId, Vec<(u32, u32, Box<Vec<u8>>)>) {
        let entry_count = stream.read_u32::<LittleEndian>().unwrap() as usize;
        let first_free = stream.read_u32::<LittleEndian>().unwrap();
        let last_free = stream.read_u32::<LittleEndian>().unwrap();
        let ident_pid = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyId;
        let ident_size = stream.read_u8().unwrap() as OMKeySize;

        let mut key_list: Vec<(u32, u32, Box<Vec<u8>>)> = vec![];
        for _ in 0..entry_count {
            let local_key = stream.read_u32::<LittleEndian>().unwrap();
            let ref_count = stream.read_u32::<LittleEndian>().unwrap();
            let mut buffer = vec![0; ident_size as usize];
            stream.read_exact(&mut buffer).unwrap();
            key_list.push((local_key, ref_count, Box::new(buffer)));
        }

        (first_free, last_free, ident_pid, key_list)
    } 

    fn resolve_weak_reference(&mut self, prop_data: &[u8]) -> InterchangeObjectDescriptor {

        let mut cursor = Cursor::new(prop_data);
        let tag = cursor.read_u16::<LittleEndian>().unwrap() as OMPropertyTag;
        let pid = cursor.read_u16::<LittleEndian>().unwrap() as OMPropertyId;
        let key_size = cursor.read_u8().unwrap() as OMKeySize;
        let mut identification = vec![ 0u8 ; key_size as usize];
        cursor.read_exact(&mut identification);
        
        todo!() 

    }

    pub fn resolve_property_value(
        &mut self,
        object: &InterchangeObjectDescriptor,
        pid: OMPropertyId,
    ) -> PropertyValue { 
        let raw_data = RawPropertyValue::raw_property_value(&mut self, object, pid);

        match property.stored_form {
            SF_DATA => PropertyValue::Data(raw_data),
            SF_DATA_STREAM => {
                let raw_name = &raw_data[0..raw_data.len() - 2];
                let decoded_name = UTF_16LE
                    .decode(raw_name, DecoderTrap::Ignore)
                    .expect("Failed to decode object reference by name");

                let ref_path = object.path.join(decoded_name);
                PropertyValue::Stream(ref_path)
            }
            SF_STRONG_OBJECT_REF => {
                let raw_name = &raw_data[0..raw_data.len() - 2];
                let decoded_name = UTF_16LE
                    .decode(raw_name, DecoderTrap::Ignore)
                    .expect("Failed to decode object reference by name");

                let ref_path = object.path.join(decoded_name);
                self.interchange_object(ref_path)
                    .map(|obj| PropertyValue::Single(obj))
                    .expect("Failed to locate object by path")
            }
            SF_STRONG_OBJECT_REF_VECTOR => {
                let raw_name = &raw_data[0..raw_data.len() - 2];
                let decoded_name = UTF_16LE
                    .decode(raw_name, DecoderTrap::Ignore)
                    .expect("Failed to decode object reference by name");

                let index_name = format!("{} index", decoded_name);

                let ref_path = object.path.join(&index_name);
                let vector_indicies = {
                    let stream = self
                        .f
                        .open_stream(&ref_path)
                        .expect("Failed to open index stream");
                    Self::read_strong_vector_index(stream).2
                };

                let members = vector_indicies
                    .into_iter()
                    .map(|i| {
                        let member_name = format!("{}{{{:x}}}", decoded_name, i);
                        object.path.join(member_name)
                    })
                    .map(|path| {
                        self.interchange_object(path)
                            .expect("Failed to locate index member")
                    })
                    .collect();

                PropertyValue::Vector(members)
            }
            SF_STRONG_OBJECT_REF_SET => {
                let raw_name = &raw_data[0..raw_data.len() - 2];
                let decoded_name = UTF_16LE
                    .decode(raw_name, DecoderTrap::Strict)
                    .expect("Failed to decode object reference by name");

                let index_name = format!("{} index", decoded_name);

                let ref_path = object.path.join(&index_name);
                let set_indicies = {
                    let stream = self
                        .f
                        .open_stream(&ref_path)
                        .expect("Failed to open set index stream");
                    Self::read_strong_set_index(stream).3
                };

                let members = set_indicies
                    .into_iter()
                    .map(|i| {
                        let member_name = format!("{}{{{:x}}}", decoded_name, i.0);
                        object.path.join(member_name)
                    })
                    .map(|path| {
                        self.interchange_object(path)
                            .expect("Failed to locate set member")
                    })
                    .collect();

                PropertyValue::Set(members)
            }
            SF_WEAK_OBJECT_REF => {
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
    fn test_obj_iterator() {
        let test_path = "testmedia/AAF_Test_1/AAF_Test_1.aaf";
        let comp = cfb::open(test_path).unwrap();
        let mut f = AAFFile::with_cfb(comp);

        for i in f.interchange_objects() {
            assert!(i.auid != Uuid::nil());
        }
    }

    #[test]
    fn test_get_properties() {
        let test_path = "testmedia/AAF_Test_1/AAF_Test_1.aaf";
        let comp = cfb::open(test_path).unwrap();
        let mut f = AAFFile::with_cfb(comp);
        let root = f.root_object().unwrap();

        let props = f.raw_properties(&root);

        assert_eq!(props.len(), 2, "Incorrect number of properties detected");

        let _p1 = f.property_by_pid(&root, 0x01).unwrap();
        let _p2 = f.property_by_pid(&root, 0x02).unwrap();
    }
}
