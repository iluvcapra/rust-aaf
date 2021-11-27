use std::fmt;
use std::path::PathBuf;
use uuid::Uuid;


pub trait InterchangeObject {
    fn auid(&self) -> Uuid;
    fn path(&self) -> PathBuf;
}


// pub struct InterchangeObjectDescriptorIter<I>(pub I);


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

// impl<I: Iterator<Item=cfb::Entry>> Iterator for InterchangeObjectDescriptorIter<I> {    
//     type Item = InterchangeObjectDescriptor;
    
//     fn next(&mut self) -> Option<InterchangeObjectDescriptor> {
//         loop {
//             let next = self.0.next();
//             match next {
//                 Some(elem) if elem.is_storage() => {
//                     return Some(InterchangeObjectDescriptor { auid: *elem.clsid(), 
//                         path: elem.path().into()})
//                 },               
//                 Some(_) => (),
//                 None => return None
//             }
//         }
//     }
// }


