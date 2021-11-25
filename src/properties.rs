#[allow(dead_code)]
#[allow(unused_imports)]

use std::fmt;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::types::{OMByteOrder, OMVersion, OMPropertyId, 
    OMStoredForm, OMPropertyCount, OMPropertySize};
use crate::interchange_object::InterchangeObjectDescriptor;
use crate::file::AAFFile;

use std::path::PathBuf;
use std::io::{Read, Seek};

pub const SF_DATA : OMStoredForm = 0x0082;
pub const SF_DATA_STREAM : OMStoredForm = 0x0042;
pub const SF_STRONG_OBJECT_REF : OMStoredForm = 0x0022;
pub const SF_STRONG_OBJECT_REF_VECTOR : OMStoredForm = 0x0032;
pub const SF_STRONG_OBJECT_REF_SET : OMStoredForm = 0x003a;
pub const SF_WEAK_OBJECT_REF : OMStoredForm = 0x0002;
pub const SF_WEAK_OBJECT_REF_VECTOR : OMStoredForm = 0x0012;
pub const SF_WEAK_OBJECT_REF_SET : OMStoredForm = 0x001a;

// Not Yet Implemented
// pub const SF_WEAK_OBJECT_STORED_OBJ_ID : OMStoredForm = 0x03;
// pub const SF_UNIQUE_OBJ_ID : OMStoredForm = 0x86;
// pub const SF_OPAQUE_STREAM : OMStoredForm = 0x40;

pub enum PropertyValue {
    Data(Box<Vec<u8>>),
    Stream(PathBuf),
    Single(InterchangeObjectDescriptor),
    Vector(Vec<InterchangeObjectDescriptor>),
    Set(Vec<InterchangeObjectDescriptor>)
}

impl PropertyValue {

    pub fn value_for_pid<F:Read+Seek>(file: &mut AAFFile<F>, 
        object: &InterchangeObjectDescriptor,
        pid: OMPropertyId) -> Option<PropertyValue> {
        todo!()
    }
}

impl fmt::Debug for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Data(v) => {
                f.debug_struct("PropertyValue::Data")
                    .field("data", v)
                    .finish()
            },
            Self::Stream(v) => {
                f.debug_struct("PropertyValue::Stream")
                    .field("path", v)
                    .finish()    
            },
            Self::Single(v) => {
                f.debug_struct("PropertyValue::SingleObject")
                    .field("obj", v)
                    .finish()
            },
            Self::Vector(v) => {
                f.debug_struct("PropertyValue::Vector")
                    .field("objects", v)
                    .finish()
            },
            Self::Set(v) => {
                f.debug_struct("PropertyValue::Set")
                    .field("objects", v)
                    .finish()
            }
        }
    }
}


pub struct RawPropertyValue {
    pub pid : OMPropertyId,
    pub stored_form: OMStoredForm,
    pub value: Box<Vec<u8>>
}

impl fmt::Debug for RawPropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PropertyDescriptor")
            .field("pid", &self.pid)
            .field("stored_form",&self.stored_form)
            .field("len(value)", &self.value.len())
            .finish()
    }
}

impl RawPropertyValue {

    pub fn raw_property_value<F:Read + Seek>(
        file: &mut AAFFile<F>,
        object: &InterchangeObjectDescriptor,
        pid: OMPropertyId,
    ) -> Option<Box<Vec<u8>>> {
        let raw_prop = file.raw_properties(object)
            .into_iter().find(|p| p.pid == pid);
        raw_prop.map(|e| e.value)
    }

    pub fn from_properties_stream<F>(mut stream: cfb::Stream<F>) -> Vec<RawPropertyValue>  
        where F: Read + Seek {
        let bom = stream.read_u8().unwrap() as OMByteOrder;
        assert_eq!(bom, 0x4c, "BOM is invalid");

        let _version = stream.read_u8().unwrap() as OMVersion;
        let property_count = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyCount;
        
        let mut prop_headers = Vec::with_capacity(property_count as usize);

        for _ in 0..property_count {
            let pid = stream.read_u16::<LittleEndian>().unwrap() as OMPropertyId;
            let stored_form = stream.read_u16::<LittleEndian>().unwrap() as OMStoredForm; 
            let size = stream.read_u16::<LittleEndian>().unwrap() as OMPropertySize;
            prop_headers.push((pid,stored_form,size));
        }
        
        let mut retval : Vec<RawPropertyValue> = Vec::with_capacity(property_count as usize);

        for (pid, stored_form, size) in prop_headers {
            let mut value = vec![0; size as usize];
            stream.read_exact(&mut value).unwrap();
            let prop = RawPropertyValue { pid, stored_form, value: Box::new(value)} ;
            retval.push(prop);
        }

        retval
    }
}
