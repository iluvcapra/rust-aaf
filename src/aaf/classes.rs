use crate::interchange_object::InterchangeObjectDescriptor;
use crate::file::AAFFile;
use crate::types::*;
use crate::properties::PropertyValue;

use std::io::{Read, Seek};
use uuid::Uuid;


const GENERATION_PID : OMPropertyId = 0x0102; 

pub trait AAFObject<F> {
    
    fn make(file: AAFFile<F>, desc: InterchangeObjectDescriptor) -> Self;
    
    fn get_property_value(&mut self, pid: OMPropertyId) -> Option<PropertyValue>;

    fn generation(&mut self) -> Option<Uuid> {
        let pid = GENERATION_PID;
        self.get_optional_data(pid) 
    }

    fn get_optional_data<T: AAFFrom>(&mut self, pid: OMPropertyId) -> Option<T> {
        match self.get_property_value(pid) {
            Some(PropertyValue::Data(b)) => { Some(b[..].aaf_into()) }
            _ => { None }
        }
    }

    fn get_required_data<T: AAFFrom>(&mut self, pid: OMPropertyId) -> T {
        self.get_optional_data(pid)
            .expect(
                &format!("Required property (pid {}) not found", pid)
                )
    }
}

pub struct Header<F> {
    file: AAFFile<F>,
    object: InterchangeObjectDescriptor
}

impl<F: Read + Seek> AAFObject<F> for Header<F> { 

    fn make(file: AAFFile<F>, object: InterchangeObjectDescriptor) -> Self {
        Self { file: file, object: object }
    }

    fn get_property_value(&mut self, pid : OMPropertyId) -> Option<PropertyValue> {
        self.file.get_value(&self.object, pid)
    }
}

impl<F> Header<F> where F: Read + Seek {

    pub fn byte_order(&mut self) -> AAFUInt16 {
        let pid = 0x3b01;
        self.get_required_data(pid)
    }

    pub fn last_modified(&mut self) -> TimeStamp {
        let pid = 0x3b02;
        self.get_required_data(pid)
     }

    pub fn version(&mut self) -> VersionType {
        let pid = 0x3b05; 
        self.get_required_data(pid)
    }
    
    pub fn object_model_version(&mut self) -> Option<AAFUInt32> {
        let pid = 0x3b07;
        self.get_optional_data(pid)
    }

    pub fn operational_pattern(&mut self) -> Option<Uuid> {
        let pid = 0x3b09;
        self.get_optional_data(pid)
    }

    pub fn content(&mut self) -> ContentStorage<F> {
        todo!()
    }

    pub fn dictionary(&mut self) -> Dictionary<F> {
        todo!()
    }

    pub fn identification_list(&mut self) -> Vec<Identification<F>> {
        todo!()
    }
}

pub struct MetaDictionary<F> {
    file: AAFFile<F>,
    object: InterchangeObjectDescriptor
}

pub struct Dictionary<F> {
    file: AAFFile<F>,
    object: InterchangeObjectDescriptor
}

pub struct ContentStorage<F> {
    file: AAFFile<F>,
    object: InterchangeObjectDescriptor
}

pub struct Identification<F> {
    file: AAFFile<F>,
    object: InterchangeObjectDescriptor
}
