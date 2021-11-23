extern crate cfb;
extern crate encoding;

mod interchange_object;
mod properties;
mod types;
mod file;

use crate::interchange_object::InterchangeObjectDescriptor;
use crate::file::AAFFile;

fn main() {
    let test_path =  "testmedia/AAF_Test_1/AAF_Test_1.aaf";
    let comp = cfb::open(test_path).unwrap();
    
    let mut f = AAFFile::with_cfb(comp);
    
    let root = f.root_object().unwrap();
    
    println!("Root object: {:?}", root); 
    
    let props = f.properties(&root);
    
    println!("Properties: {:?}", props);

    let header_property = f.property_by_pid(&root, 0x01).unwrap();

    let header = f.resolve_property_value(&root, &header_property);

    println!("Content: {:?}", header);


    // { 
    //     for e in f.interchange_objects() {
    //         objects.push(e)
    //     }
    // }
    
    // let mut objects_properties : Vec<(InterchangeObjectDescriptor, 
    //         Vec<PropertyDescriptor>)> = vec![];

    // {
    //     for e in objects.into_iter() {
    //         let properties = f.properties(&e);
    //         objects_properties.push((e, properties));
    //     }
    // }

    // for e in objects_properties.into_iter() {
    //     println!("Object: {:?}", e.0);
    //     for p in e.1.into_iter() {
    //         println!("- {:?}", p);
    //     }
    // }
}
