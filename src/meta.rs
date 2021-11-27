// use uuid::Uuid;

// type StringArray = Vec<String>;
// type Int64Array = Vec<u64>
// type Ref = u64;
// type AUIDArray = Vec<Uuid>

// struct ClassDefinition {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     parent_class: Option<Ref>,
//     properties: Vec<Ref>,
//     concrete: bool
// }

// struct PropertyDefinition {
//     idenitification: Uuid,
//     name: String,
//     description: String,
//     optional: bool,
//     local_identification: u16,
//     unique: bool
// }

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
