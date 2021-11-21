#[allow(dead_code)]
use std::fmt;

use crate::types::{OMPropertyId,OMStoredForm};

pub const SF_DATA : OMStoredForm = 0x0082;
pub const SF_DATA_STREAM : OMStoredForm = 0x0042;
pub const SF_STRONG_OBJECT_REF : OMStoredForm = 0x0022;
pub const SF_STRONG_OBJECT_REF_VECTOR : OMStoredForm = 0x0032;
pub const SF_STRONG_OBJECT_REF_SET : OMStoredForm = 0x003a;
pub const SF_WEAK_OBJECT_REF : OMStoredForm = 0x0002;
pub const SF_WEAK_OBJECT_REF_VECTOR : OMStoredForm = 0x0012;
pub const SF_WEAK_OBJECT_REF_SET : OMStoredForm = 0x001a;
pub const SF_WEAK_OBJECT_STORED_OBJ_ID : OMStoredForm = 0x03;
pub const SF_UNIQUE_OBJ_ID : OMStoredForm = 0x86;
pub const SF_OPAQUE_STREAM : OMStoredForm = 0x40;

pub struct PropertyDescriptor {
    pub pid : OMPropertyId,
    pub stored_form: OMStoredForm,
    pub value: Box<Vec<u8>>
}

impl fmt::Debug for PropertyDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PropertyDescriptor")
            .field("pid", &self.pid)
            .field("stored_form",&self.stored_form)
            .field("len(value)", &self.value.len())
            .finish()
    }
}


