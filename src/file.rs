use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{Read, Seek};
use crate::interchange_object::{InterchangeObjectDescriptor,InterchangeObjectDescriptorIter};
use crate::properties::PropertyDescriptor;
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
}

impl<F: Read + Seek> AAFFile<F> {

    fn parse_properties_file(mut stream: cfb::Stream<F>) -> Vec<PropertyDescriptor> {
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
        
        let mut retval : Vec<PropertyDescriptor> = Vec::with_capacity(property_count as usize);

        for (pid, stored_form, size) in prop_headers {
            let mut value = vec![0; size as usize];
            stream.read_exact(&mut value).unwrap();
            let prop = PropertyDescriptor { pid, stored_form, value: Box::new(value)} ;
            retval.push(prop);
        }

        retval
    }

    pub fn properties(&mut self, object: &InterchangeObjectDescriptor) 
        -> Vec<PropertyDescriptor> {
        let properties_path = object.path.join("properties");
        let stream = self.f.open_stream(&properties_path).unwrap();
        Self::parse_properties_file(stream)
    }
}
