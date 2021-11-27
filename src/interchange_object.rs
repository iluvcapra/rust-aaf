use std::fmt;
use std::path::PathBuf;
use uuid::Uuid;


pub trait InterchangeObject {
    fn auid(&self) -> Uuid;
    fn path(&self) -> PathBuf;
}

pub struct InterchangeObjectDescriptor {
    pub auid : Uuid,
    pub path : PathBuf,
}

impl fmt::Debug for InterchangeObjectDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InterchangeObjectDescriptor")
            .field("auid", &self.auid)
            .field("path", &self.path)
            .finish()
    }
}

