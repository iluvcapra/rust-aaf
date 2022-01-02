use crate::interchange_object::InterchangeObjectDescriptor;
use crate::file::AAFFile;
use crate::types::OMPropertyId;
use std::io::{Read, Seek};


pub struct ObjectEntry {
    pub parent: InterchangeObjectDescriptor,
    pub property_id: OMPropertyId,
    pub object: InterchangeObjectDescriptor,
    pub depth: usize
}

pub struct InterchangeObjects<'a,F> {
    file: &'a mut AAFFile<F>,
    stack : Vec<ObjectEntry>
}

impl<'a, F> InterchangeObjects<'a, F>
where F: Read + Seek {
    pub(crate) fn new(file: &'a mut AAFFile<F>, 
        root_object: InterchangeObjectDescriptor) -> Self {
        Self {
            file,
            stack: vec![]
        }
    }

    fn fill_stack(&mut self, object: &InterchangeObjectDescriptor) {
        todo!()
    }
}

impl<'a, F> Iterator for InterchangeObjects<'a, F> {
    type Item = ObjectEntry;

    fn next(&mut self) -> Option<ObjectEntry> {
        if let Some(rval) = self.stack.pop() {
            Some(rval)
        } else {
            None
        }
    }
}
