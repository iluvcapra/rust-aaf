extern crate cfb;
extern crate encoding;

mod interchange_object;
mod properties;
mod types;
mod file;

use crate::interchange_object::InterchangeObjectDescriptor;
use crate::file::AAFFile;
use crate::properties::*;

use std::io::{Read, Seek};

fn print_object<T>(file : &mut AAFFile<T>, obj: &InterchangeObjectDescriptor)
    where T: Read + Seek {
    
    fn print_obj_impl<T>(file: &mut AAFFile<T>, 
        obj: &InterchangeObjectDescriptor, 
        indent: usize) where T: Read + Seek {
        
        let indent_str = String::from_utf8(vec![b' '; indent]).unwrap();
            
        println!("{}Object({:?}) {{",indent_str, obj.path);
        for prop in file.raw_properties(obj) {

            // hiding these options until they're implemented
            if prop.stored_form == SF_WEAK_OBJECT_REF ||
                prop.stored_form == SF_WEAK_OBJECT_REF_SET ||
                    prop.stored_form == SF_WEAK_OBJECT_REF_VECTOR {
                continue;
            }

            let val = file.resolve_property_value(obj, &prop);

            match val {
                PropertyValue::Data(v) => {
                    println!("  {}(pid {:#04x}) = len({:?})", indent_str, prop.pid, v.len()); 
                },
                PropertyValue::Stream(p) => {
                    println!("  {}(pid {:#04x}) = stream({:?})", indent_str, prop.pid, p);
                },
                PropertyValue::Single(o) => {
                    println!("  {}(pid {:#04x}) => ", indent_str, prop.pid);
                    print_obj_impl(file, &o, indent + 4);
                    println!("  {}", indent_str);
                },
                PropertyValue::Vector(o) => {
                    println!("  {}(pid {:#04x}) = [", indent_str, prop.pid);
                    for child in o {
                        print_obj_impl(file, &child, indent + 4);
                    }
                    println!("  {}]", indent_str);
                },
                PropertyValue::Set(o) => {
                    println!("  {}(pid {:#04x}) = (", indent_str, prop.pid);
                    for child in o {
                        print_obj_impl(file, &child, indent + 4);
                    }
                    println!("  {})", indent_str);
                }
            }
        }
        println!("{}}}", indent_str);
    }

    print_obj_impl(file, obj, 0);
}

fn main() {
    let test_path =  "testmedia/AAF_Test_1/AAF_Test_1.aaf";
    let mut f = AAFFile::open(test_path)
        .expect("error opening file");
    
    let root = f.root_object().unwrap();
   
    print_object(&mut f, &root);
}
