
use encoding::all::UTF_16LE;
use encoding::{Encoding, DecoderTrap};

mod interchange_object;
mod properties;
mod types;
mod file;
mod session;
mod aaf;

use crate::interchange_object::InterchangeObjectDescriptor;
use crate::file::AAFFile;
// use crate::properties::*;
// use crate::aaf::classes::*;

use std::io::{Read, Seek};

// use uuid::Uuid;

fn print_object<T>(file : &mut AAFFile<T>, obj: &InterchangeObjectDescriptor)
    where T: Read + Seek {
    
        let i = file.walk_properties();

        for entry in i {
            println!("Parent: {:?}", entry.parent);
            println!("Prop: {}", entry.property);
            println!("Value: {:?}", entry.value);
        }
}

fn main() {
    let test_path =  "testmedia/AAF_Test_1/AAF_Test_1.aaf";
    let mut f = AAFFile::open(test_path)
        .expect("error opening file"); 

    let root = f.root_object();
    print_object(&mut f, &root);
}
