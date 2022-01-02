use crate::interchange_object::InterchangeObjectDescriptor;
use crate::types::OMPropertyId;
use crate::properties::PropertyValue;
use crate::file::AAFFile;

use std::io::{Read, Seek};

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


