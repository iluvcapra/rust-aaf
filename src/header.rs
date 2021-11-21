use crate::properties::*;
use crate::interchange_object::InterchangeObjectDescriptor;

// Auid 0D010101-0101-2F00-060E-2B3402060101
// Parent: InterchangeObject
// Is concrete: true
//
struct Header {
    interchange_object : InterchangeObjectDescriptor, 
}

impl Header {
    /// byte-order (Int16)
    /// 03010201-0200-0000-060E-2B3401010101
    /// Short form: 0x3b01
    /// Mandatory: true
    /// Property UID is class UID: false,
    /// class: Header
    fn byte_order(&self) -> u16 {
        todo!()
    }
    
    /// last modified
    fn last_modified(&self) -> TimeStamp {
        todo!()
    }

    fn content(&self) -> ContentStorage {
        todo!()
    }

    fn dictionary(&self) -> AAFDictionary {
        todo!()
    }

    fn version(&self) -> VersionType {
        todo!()
    }

    fn identification_list(&self) -> Vec<Identification> {
        todo!()
    }
    
    fn object_model_version(&self) -> u32 {
        todo!()
    }

    fn operational_pattern(&self) -> Uuid {
        todo!()
    }

    fn descriptive_schemes(&self) -> Set<Uuid> {

    }
}
