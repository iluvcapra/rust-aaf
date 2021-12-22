use crate::interchange_object::InterchangeObjectDescriptor;
use crate::file::AAFFile;
use crate::types::*;
use crate::properties::PropertyValue;

use std::io::{Read, Seek};

pub struct ContentStorage<'a,F> {
    file: &'a mut AAFFile<F>,
    object: InterchangeObjectDescriptor
}

pub struct Header<'a, F> {
    file: &'a mut AAFFile<F>,
    object: InterchangeObjectDescriptor
}

impl<'a, F> Header<'a, F> where F: Read + Seek {
    
    pub fn from(file: &'a mut AAFFile<F>, object: InterchangeObjectDescriptor) -> Self {
        Header { file: file, object: object }
    }

    pub fn byte_order(&mut self) -> AAFUInt16 {
        let pid = 0x3b01; 
        if let Some(PropertyValue::Data(b)) = self.file.get_value(&self.object, pid) {
            b[..].aaf_into()
        } else {
            panic!("Required property Header.ByteOrder not found")
        }
    }

    pub fn last_modified(&mut self) -> TimeStamp {
        let pid = 0x3b02; 
        if let Some(PropertyValue::Data(b)) = self.file.get_value(&self.object, pid) {
            b[..].aaf_into()
        } else {
            panic!("Required property Header.TimeStamp not found")
        }
     }

    pub fn version(&mut self) -> VersionType {
        let pid = 0x3b05; 
        if let Some(PropertyValue::Data(b)) = self.file.get_value(&self.object, pid) {
            b[..].aaf_into()
        } else {
            panic!("Required property Header.VersionType not found")
        }
    }
    
    pub fn object_model_version(&mut self) -> Option<AAFUInt32> {
        let PID = 0; //fixme this is wrong
        match self.file.get_value(&self.object, PID) {
            Some(PropertyValue::Data(b)) => {
                Some(b[..].aaf_into())
            }
            _ => None
        }
    }

    fn content(&mut self) -> ContentStorage<'a, F> {
       todo!()
    }

    fn dictionary(&mut self) -> () {
       todo!()
    } 
}

pub struct MetaDictionary<'a, F> {
    file: &'a mut AAFFile<F>,
    object: InterchangeObjectDescriptor
}



