use crate::interchange_object::InterchangeObjectDescriptor;
use crate::file::AAFFile;
use crate::types::OMPropertyId;
use crate::properties::PropertyValue;

use std::io::{Read, Seek};
use std::path::PathBuf;

enum AAFValue {
    Data(Vec<u8>),
    Stream(PathBuf),
    Object(AAFObject),
    Vector(Vec<AAFObject>),
    Set(Vec<AAFObject>),
    ObjectRef(PathBuf),
    VectorRef(Vec<PathBuf>),
    SetRef(Vec<PathBuf>)
}

impl AAFValue {
    fn from_property_value<F: Read + Seek>(pv: PropertyValue, 
        in_file:&mut AAFFile<F>) -> Self {

        match pv {
            PropertyValue::Data(d) => AAFValue::Data(*d),
            PropertyValue::Stream(p) => AAFValue::Stream(p),
            PropertyValue::Single(o) => {
                AAFValue::Object(AAFObject::load(in_file, &o)) 
            },
            PropertyValue::Vector(v) => {
                AAFValue::Vector(v.into_iter().map(|o| {
                    AAFObject::load(in_file, &o)
                }).collect())
            },
            PropertyValue::Set(s) => {
                 AAFValue::Set(s.into_iter().map(|o| {
                    AAFObject::load(in_file, o)
                }).collect())
            },
        }
    }
}

struct AAFObject {
    object_descriptor: InterchangeObjectDescriptor,
    properties: Vec<(OMPropertyId, AAFValue)>
}

impl AAFObject {
    fn load<F:Read + Seek>(from_file: &mut AAFFile<F>, 
        obj: &InterchangeObjectDescriptor) -> Self {

        let properties = from_file.all_property_ids(object);

        let reduced = properties.into_iter().map( |pid| {
            match prop {

            }
        })

    }
}

impl<F: Read + Seek> AAFObject<F> {

}
