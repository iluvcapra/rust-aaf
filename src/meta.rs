use uuid::Uuid;

use std::io::{Read, Seek};

use crate::interchange_object::InterchangeObjectDescriptor;
use crate::properties::PropertyValue;
use crate::file::AAFFile;
use crate::types::OMPropertyId;

type StringArray = Vec<String>;
type Int64Array = Vec<u64>;
type Ref = u64;
type AUIDArray = Vec<Uuid>;

type Index = usize;

const AAF_FILE_HEADER_PID: OMPropertyId = 0x0002;
const AAF_FILE_METADICTIONARY_PID: OMPropertyId = 0x0001;
// AAF File uuid b3b398a5-1c90-11d4-8053-080036210804

pub struct MetaDictionary {
    pub class_defs : Vec<ClassDefinition>
}

impl MetaDictionary {
    fn load<F:Read + Seek>(file: &mut AAFFile<F>) -> Self {
        
        let object = {
            let root = file.root_object();
            let ov = file.get_value(&root, AAF_FILE_METADICTIONARY_PID);
            ov.unwrap_object()
        };

        let class_defs: Vec<ClassDefinition> = {
            file.get_value(&object, 0x0003)
                .unwrap_set()
                .into_iter()
                .map(|obj| {
                    ClassDefinition::load(file, &obj)
                }).collect()
        };

        Self { class_defs } 
    }
}

pub struct ClassDefinition {
    pub interchange_object: InterchangeObjectDescriptor,
    pub name: String,
    pub description: String,
    pub parent_class: Index,
    pub properties: Vec<PropertyDefinition>,
    pub concrete: bool
}

impl ClassDefinition {
    fn load<F: Read + Seek>(file : &mut AAFFile<F>, 
        obj: &InterchangeObjectDescriptor) -> Self {
        todo!()
    }
}



pub struct PropertyDefinition {
    pub interchange_object: InterchangeObjectDescriptor,
    pub name: String,
    pub description: String,
    pub optional: bool,
    pub local_identification: u16,
    pub unique: bool,
    pub prop_type: ()
}




// struct TypeDefinitionCharacter {
//     idenitification: Uuid,
//     name: String,
//     description: String
// }

// struct TypeDefinitionEnumeration {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     element_type: Ref,
//     element_names: StringArray,
//     element_values: Int64Array
// }

// struct TypeDefinitionExtendibleEnumeration {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     element_names: StringArray,
//     element_values: AUIDArray 
// }

// struct TypeDefinitionFixedArray {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     element_type: Ref,
//     element_count: u32
// }

// struct TypeDefinitionIndirect {
//     idenitification: Uuid,
//     name: String,
//     description: String,
// } 

// struct TypeDefinitionInteger {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     size: u8,
//     signed: bool
// }

// struct TypeDefinitionOpaque {
//     idenitification: Uuid,
//     name: String,
//     description: String,
// }
 
// struct TypeDefinitionRecord {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     member_types: Vec<Ref>,
//     member_names: StringArray
// }

// struct TypeDefinitionRename {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     renamed_type: Ref 
// }

// struct TypeDefinitionSet {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     element_type: Ref 
// }

// struct TypeDefinitionStream {
//     idenitification: Uuid,
//     name: String,
//     description: String,
// }

// struct TypeDefinitionString {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     element_type: Ref
// }

// struct TypeDefinitionStrongObjectReference {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     referenced_type: Ref
// }

// struct TypeDefinitionVariableArray {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     element_type: Ref
// }

// struct TypeDefinitionWeakObjectReference {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     referenced_type: Ref,
//     target_list: AUIDArray
// }

// struct MetaDictionary {
//     class_definitions: Vec<ClassDefinition>,
//     // type_definitions: Vec<
