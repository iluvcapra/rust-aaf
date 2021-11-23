use std::path::PathBuf;
use std::io::Cursor;
use std::io::{Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};

use encoding::all::UTF_16LE;
use encoding::{Encoding,DecoderTrap};

use crate::interchange_object::{InterchangeObjectDescriptor,
    InterchangeObjectDescriptorIter, InterchangeObjectReferent};
use crate::properties::*;
use crate::types::{OMByteOrder, OMVersion, OMPropertyId, OMStoredForm, 
    OMPropertySize, OMPropertyCount};

use cfb;

/// An AAF file.
pub struct AAFFile<F> {
    f : cfb::CompoundFile<F>
}

impl<F> AAFFile<F> {
    /// A new `AAFFile` with a `cfb::CompoundFile` 
    pub fn with_cfb(cfb: cfb::CompoundFile<F>) -> Self {
        Self { f: cfb }
    }
}

impl<F> AAFFile<F> {
    pub fn interchange_objects(&mut self) -> InterchangeObjectDescriptorIter<cfb::Entries> {
        let entries = self.f.walk();
        InterchangeObjectDescriptorIter(entries)
    }

    pub fn interchange_object(&self, path: PathBuf) -> Option<InterchangeObjectDescriptor> { 
        self.f.entry(path)
            .map(|entry| { 
                InterchangeObjectDescriptor { 
                    auid: *entry.clsid(), 
                    path: entry.path().into() 
                } 
            } )
            .ok()
        }
}

impl<F: Read + Seek> AAFFile<F> {

    pub fn properties(&mut self, object: &InterchangeObjectDescriptor) -> Vec<PropertyDescriptor> {
        let properties_path = object.path.join("properties");
        let stream = self.f.open_stream(&properties_path)
            .expect(&format!("Failed to open `properties` stream for object {:?}", object));

        PropertyDescriptor::from_properties_stream(stream)
    }
    
    fn get_raw_property_value(&mut self, object: &InterchangeObjectDescriptor, 
        pid: OMPropertyId) -> Box<Vec<u8>> {
        let props = self.properties(object);
        
        props.into_iter().find(|prop| { prop.pid == pid } )
            .take().expect("No proprety exists for given pid").value
    }   

    pub fn resolve_object_ref(&mut self, object: &InterchangeObjectDescriptor, 
        property: &PropertyDescriptor) -> Option<InterchangeObjectReferent> {
    
        let raw_name = Self::get_raw_property_value(self, object, property.pid);
        let decoded_name = UTF_16LE.decode(&raw_name, DecoderTrap::Strict)
                    .expect("Failed to decode object reference by name"); 

        match property.stored_form {
            SF_DATA => None,
            SF_DATA_STREAM => None,
            SF_OPAQUE_STREAM => None,
            SF_STRONG_OBJECT_REF => {
                let ref_path = object.path.join(decoded_name);   
                self.interchange_object(ref_path).map( |obj| {
                    InterchangeObjectReferent::Single(obj)
                })
            },
            SF_STRONG_OBJECT_REF_VECTOR => {
                let index_name = format!("{} index", decoded_name);
                let ref_path = object.path.join(index_name);
                todo!()
            },
            SF_STRONG_OBJECT_REF_SET => {
                todo!()
            },
            SF_WEAK_OBJECT_REF => {
                todo!()
            },
            SF_WEAK_OBJECT_REF_VECTOR => {
                todo!()
            },
            SF_WEAK_OBJECT_REF_SET => {
                todo!()
            }, 
            SF_WEAK_OBJECT_STORED_OBJ_ID => {
                todo!()
            }, 
            _ => panic!("Unrecgonized stored form constant found. Exiting.")
        }
    }

}



