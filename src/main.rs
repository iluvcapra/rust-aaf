use encoding::all::UTF_16LE;
use encoding::{DecoderTrap, Encoding};

mod aaf;
mod file;
mod interchange_object;
mod properties;
mod session;
mod types;

use crate::file::AAFFile;
use crate::interchange_object::InterchangeObjectDescriptor;
use std::io::{Read, Seek};

fn print_object<T>(file: &mut AAFFile<T>, obj: &InterchangeObjectDescriptor)
where
    T: Read + Seek,
{
    let i = file.walk_properties();

    for entry in i {
        let indent = std::iter::repeat("  ")
            .take(entry.depth())
            .collect::<String>();

        println!("{}Parent: {:?}", indent, entry.parent().path);
        println!("{}Prop: {}", indent, entry.property_id());
    }
}

fn main() {
    let test_path = "testmedia/AAF_Test_1/AAF_Test_1.aaf";
    let mut f = AAFFile::open(test_path).expect("error opening file");

    let root = f.root_object();
    print_object(&mut f, &root);
}
