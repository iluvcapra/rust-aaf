
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
use crate::properties::*;
use crate::aaf::classes::*;

use std::io::{Read, Seek};

use uuid::Uuid;

fn print_object<T>(file : &mut AAFFile<T>, obj: &InterchangeObjectDescriptor)
    where T: Read + Seek {
    
        let i = file.walk_hard_links();

        for entry in i {
            println!("Path: {:?}", entry.parent);
            println!("Prop: {}", entry.property);
            println!("Value: {:?}", entry.value);
        }

    // fn print_obj_impl<T>(file: &mut AAFFile<T>, 
    //     obj: &InterchangeObjectDescriptor, 
    //     indent: usize, descend: bool) where T: Read + Seek {
        
    //     let indent_str = String::from_utf8(vec![b' '; indent]).unwrap();
            
    //     if descend {
    //         println!("{}Object({:?},\n{}  AUID:{}) {{",indent_str, obj.path, indent_str, obj.auid);
    //         for pid in file.all_property_ids(obj) {
    //             let val = file.get_value(obj, pid).unwrap();

    //             match val {
    //                 PropertyValue::Data(v) => {
    //                     match pid {
    //                         0x06 | 0x1b02 => {
    //                             let sval = UTF_16LE.decode(&*v, DecoderTrap::Strict)
    //                                 .unwrap_or(String::from("(Undecodable)"));
    //                             println!("  {}(pid {:#04x}) = string({})", indent_str, pid, sval); 
    //                         },
    //                         _ => { 
    //                             println!("  {}(pid {:#04x}) = len({:?})", indent_str, pid, v.len()); 
    //                         }
    //                     }
    //                 }
    //                 PropertyValue::Stream(p) => {
    //                     println!("  {}(pid {:#04x}) = stream({:?})", indent_str, pid, p);
    //                 },
    //                 PropertyValue::Single(o) => {
    //                     println!("  {}(pid {:#04x}) => ", indent_str, pid);
    //                     print_obj_impl(file, &o, indent + 4, true);
    //                     println!("  {}", indent_str);
    //                 },
    //                 PropertyValue::Vector(o) => {
    //                     println!("  {}(pid {:#04x}) = [", indent_str, pid);
    //                     for child in o {
    //                         print_obj_impl(file, &child, indent + 4, true);
    //                     }
    //                     println!("  {}]", indent_str);
    //                 },
    //                 PropertyValue::Set(o) => {
    //                     println!("  {}(pid {:#04x}) = (", indent_str, pid);
    //                     for child in o {
    //                         print_obj_impl(file, &child, indent + 4, true);
    //                     }
    //                     println!("  {})", indent_str);
    //                 },
    //                 PropertyValue::Reference(o) => {
    //                     println!("  {}(pid {:#04x}) ~> Ref({:?})", indent_str, pid, o.path);
    //                 },
    //                 PropertyValue::ReferenceVector(o) => {
    //                     println!("  {}(pid {:#04x}) ~> [", indent_str, pid);
    //                     for child in o {
    //                         println!("    {}Ref({:?})", indent_str, child.path);
    //                     }
    //                     println!("  {}]", indent_str);
    //                 },
    //                 PropertyValue::ReferenceSet(o) => {
    //                     println!("  {}(pid {:#04x}) ~> [", indent_str, pid);
    //                     for child in o {
    //                         println!("    {}Ref({:?})", indent_str, child.path);
    //                     }
    //                     println!("  {}]", indent_str);
    //                 }
    //             }
    //         }
    //         println!("{}}}", indent_str);
    //     } else {
    //         println!("{}Object({:?},\n{}  AUID:{})",indent_str, obj.path, 
    //             indent_str, obj.auid);
    //     }
    // }
    // print_obj_impl(file, obj, 0, true);
}

fn main() {
    let test_path =  "testmedia/AAF_Test_1/AAF_Test_1.aaf";
    let mut f = AAFFile::open(test_path)
        .expect("error opening file");
    
    // {
    //     let mut h = f.header();

    //     println!(" Last modified: {:?}", h.last_modified());
    //     println!(" Byte order: {:?}", h.byte_order());
    //     println!(" Version: {:?}", h.version());
    //     println!(" Operational Pattern: {:?}", h.operational_pattern());
    //     println!(" Object model version: {}", h.object_model_version().unwrap_or(0));
    //     println!(" Object generation: {:?}", h.generation());
    //}

    let root = f.root_object();
    print_object(&mut f, &root);
}
