use cfb;

mod interchange_object;
mod properties;
mod types;
mod file;

use crate::interchange_object::InterchangeObjectDescriptor;
use crate::properties::PropertyDescriptor;
use crate::file::AAFFile;

fn main() {
    let test_path =  "testmedia/AAF_Test_1/AAF_Test_1.aaf";
    let comp = cfb::open(test_path).unwrap();
    
    let mut f = AAFFile::with_cfb(comp);
    let mut objects : Vec<InterchangeObjectDescriptor> = vec![];

    { 
        for e in f.interchange_objects() {
            objects.push(e)
        }
    }
    
    let mut objects_properties : Vec<(InterchangeObjectDescriptor,
                                      Vec<PropertyDescriptor>)> = vec![];

    {
        for e in objects.into_iter() {
            let properties = f.properties(&e);
            objects_properties.push((e, properties));
        }
    }

    for e in objects_properties.into_iter() {
        println!("Object: {:?}", e.0);
        for p in e.1.into_iter() {
            println!("- {:?}", p);
        }
    }
}
