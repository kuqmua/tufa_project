#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence
)]
pub enum One {
    Variant {
        #[eo_display]
        eo_display_field: DisplayStruct,
        #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: std::string::String,
        #[eo_display_foreign_type]
        eo_display_foreign_type_field: DisplayForeignTypeStruct,
        #[eo_display_foreign_type_with_serialize_deserialize]
        eo_display_foreign_type_with_serialize_deserialize_field: DisplayForeignTypeWithSerializeDeserializeStruct,
        #[eo_error_occurence]
        eo_error_occurence_field: Two,
        #[eo_vec_display]
        eo_vec_display_field: std::vec::Vec<DisplayStruct>,
        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_display_with_serialize_deserialize_field: std::vec::Vec<std::string::String>,
        #[eo_vec_display_foreign_type]
        eo_vec_display_foreign_type_field: std::vec::Vec<DisplayForeignTypeStruct>,
        #[eo_vec_display_foreign_type_with_serialize_deserialize]
        eo_vec_display_foreign_type_with_serialize_deserialize_field: std::vec::Vec<DisplayForeignTypeWithSerializeDeserializeStruct>,
        #[eo_vec_error_occurence]
        eo_vec_error_occurence_field: std::vec::Vec<ErrorUnnamed>,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_field: std::collections::HashMap<std::string::String, DisplayStruct>,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_field: ,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_field: ,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize_field: ,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        // eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_display]
        // eo_hashmap_key_display_foreign_type_value_display_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        // eo_hashmap_key_display_foreign_type_value_error_occurence_field: ,

        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },

    // Variant {
    //     #[eo_]
    //     _field: ,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
}


#[derive(Debug)]
pub struct DisplayStruct {
    pub display: std::string::String,
}
impl std::fmt::Display for DisplayStruct {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.display)
    }
}

#[derive(Debug)]
pub struct DisplayForeignTypeStruct {
    pub display_foreign_type: std::string::String,
}
impl error_occurence_lib::DisplayForeignType for DisplayForeignTypeStruct {
    fn display_foreign_type(&self) -> std::string::String {
        format!("{self:#?}")
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisplayForeignTypeWithSerializeDeserializeStruct {
    pub display_foreign_type_with_serialize_deserialize: std::string::String,
}
impl error_occurence_lib::DisplayForeignType for DisplayForeignTypeWithSerializeDeserializeStruct {
    fn display_foreign_type(&self) -> std::string::String {
        format!("{self:#?}")
    }
}


#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence
)]
pub enum Two {
    Variant {
        #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}


#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum InnerErrorNamed {
    Something {
        #[eo_display_with_serialize_deserialize]
        string: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum ErrorUnnamed {
    #[eo_error_occurence]
    Something(InnerErrorNamed),
}

fn main() {
    println!("1");
}



        // eo_display,
        // eo_display_with_serialize_deserialize,
        // eo_display_foreign_type,
        // eo_display_foreign_type_with_serialize_deserialize,
        // eo_error_occurence,
        // //todo error_occurence version for - after errors after deserialization
        // eo_vec_display,//todo maybe add version without generation \n for each element?
        // eo_vec_display_with_serialize_deserialize,
        // eo_vec_display_foreign_type,
        // eo_vec_display_foreign_type_with_serialize_deserialize,
        // eo_vec_error_occurence,
        // eo_hashmap_key_display_with_serialize_deserialize_value_display,
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize,
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type,
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize,
        // eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence,
        // eo_hashmap_key_display_foreign_type_value_display,
        // eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize,
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type,
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize,
        // eo_hashmap_key_display_foreign_type_value_error_occurence,




// pub fn test_code_occurence() {
//     let git_info = crate::common::git::git_info::GitInfo::default();
//     // let code_occurence = error_occurence_lib::code_occurence::CodeOccurence::new(
//     //     git_info.to_git_info_without_lifetime(),
//     //     file!().to_string(),
//     //     line!(),
//     //     column!(),
//     //     {
//     //         app_state::GetServerPort::get_server_port(
//     //             *crate::config_mods::config_struct::ConfigUnchecked::default(),
//     //         )
//     //     },
//     // );
//     let e = ErrorNamed::Something {
//         eo_display: DisplayStruct {
//             display_struct: std::string::String::from("String")
//         },
//         eo_display_lifetime: DisplayStructLifetime {
//             display_struct_lifetime: std::string::String::from("str")
//         },

//         eo_str_display_with_serialize_deserialize: std::string::String::from("str"),
//         eo_string_display_with_serialize_deserialize: std::string::String::from("String"),
//         eo_display_with_serialize_deserialize: DisplayWithSerializeDeserializeStruct {},
//         eo_display_with_serialize_deserialize_lifetime: DisplayWithSerializeDeserializeStructLifetime {
//             display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str")
//         },

//         eo_display_foreign_type: DisplayForeignTypeStruct {
//             display_foreign_type_struct: std::string::String::from("String")
//         },
//         eo_display_foreign_type_lifetime: DisplayForeignTypeStructLifetime {
//             display_foreign_type_struct: std::string::String::from("str")
//         },

//         eo_display_foreign_type_with_serialize_deserialize: DisplayForeignTypeSerializeDeserializeStruct {
//             display_foreign_type_serialize_deserialize_struct: std::string::String::from("String")
//         },
//         eo_display_foreign_type_with_serialize_deserialize_lifetime: DisplayForeignTypeSerializeDeserializeStructLifetime {
//             display_foreign_type_serialize_deserialize_struct: std::string::String::from("str")
//         },

//         eo_error_occurence: InnerErrorNamed::Something {
//             string: std::string::String::from("String"),
//             display_with_serialize_deserialize_struct_lifetime: DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//             code_occurence: code_occurence.clone(),
//         },

//         eo_vec_str_display_with_serialize_deserialize: vec![std::string::String::from("str")],
//         eo_vec_string_display_with_serialize_deserialize:  vec![std::string::String::from("String")],
//         eo_vec_display: vec![DisplayStruct { display_struct: std::string::String::from("String") }],
//         eo_vec_display_lifetime: vec![DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") }],

//         eo_vec_display_with_serialize_deserialize: vec![DisplayWithSerializeDeserializeStruct {}],
//         eo_vec_display_with_serialize_deserialize_lifetime: vec![DisplayWithSerializeDeserializeStructLifetime { display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") }],

//         eo_vec_display_foreign_type: vec![DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") }],
//         eo_vec_display_foreign_type_lifetime: vec![DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") }],

//         eo_vec_display_foreign_type_with_serialize_deserialize: vec![DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") }],
//         eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: vec![DisplayForeignTypeSerializeDeserializeStructLifetime{ display_foreign_type_serialize_deserialize_struct: std::string::String::from("str") }],

//         eo_vec_error_occurence: vec![ErrorUnnamed::Something(InnerErrorNamed::Something {
//             string: std::string::String::from("String"),
//             display_with_serialize_deserialize_struct_lifetime: DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//             code_occurence: code_occurence.clone(),
//         })],

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
//             "str",
//             DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
//             "str",
//             DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             "str",
//             "str",
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             "str",
//         )]),
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             "str",
//             std::string::String::from("String")
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             std::string::String::from("String")
//         )]),

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             "str",
//             DisplayWithSerializeDeserializeStruct {},
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             DisplayWithSerializeDeserializeStruct {},
//         )]),
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             "str",
//             DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//         )]),

//         eo_hashmap_key_str_value_display_foreign_type: std::collections::HashMap::from([(
//             "str",
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_string_value_display_foreign_type: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_str_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
//             "str",
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_string_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
//             "str",
//             DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             "str",
//             DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
//             "str",
//             ErrorUnnamed::Something(InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             ErrorUnnamed::Something(InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),

//         eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_display_foreign_type_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             "str",
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             "str",
//         )]),
//         eo_hashmap_key_display_foreign_type_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             std::string::String::from("String"),
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             std::string::String::from("String"),
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             DisplayWithSerializeDeserializeStruct{},
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             DisplayWithSerializeDeserializeStruct{},
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),

//         eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap::from([(
//             DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             ErrorUnnamed::Something(InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap::from([(
//             DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             ErrorUnnamed::Something(InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),

//         code_occurence,
//     };
//     println!("{e}");
//     let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
//     println!("{e_serialize_deserialize_version}");
//     let e_json = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
//     println!("{e_json}");
//     let e_deserialized: ErrorNamedWithSerializeDeserialize = serde_json::from_str(&e_json).unwrap();
//     println!("{e_deserialized}");
// }

// pub struct DisplayStruct {
//     display_struct: std::string::String,
// }
// impl std::fmt::Display for DisplayStruct {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{}", self.display_struct)
//     }
// }

// pub struct DisplayStructLifetime {
//     display_struct_lifetime: std::string::String,
// }
// impl std::fmt::Display for DisplayStructLifetime {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{}", self.display_struct_lifetime)
//     }
// }

// #[derive(PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
// pub struct DisplayWithSerializeDeserializeStruct {}
// impl std::fmt::Display for DisplayWithSerializeDeserializeStruct {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "DisplayWithSerializeDeserializeStruct")
//     }
// }

// #[derive(PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
// pub struct DisplayWithSerializeDeserializeStructLifetime {
//     display_with_serialize_deserialize_struct_lifetime: std::string::String,
// }
// impl std::fmt::Display for DisplayWithSerializeDeserializeStructLifetime {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "DisplayWithSerializeDeserializeStructLifetime")
//     }
// }

// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct DisplayForeignTypeSerializeDeserializeStruct {
//     display_foreign_type_serialize_deserialize_struct: std::string::String,
// }
// impl error_occurence_lib::DisplayForeignType
//     for DisplayForeignTypeSerializeDeserializeStruct
// {
//     fn display_foreign_type(&self) -> std::string::String {
//         std::string::String::from("DisplayForeignTypeSerializeDeserializeStruct")
//     }
// }
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct DisplayForeignTypeSerializeDeserializeStructLifetime {
//     display_foreign_type_serialize_deserialize_struct: std::string::String,
// }
// impl error_occurence_lib::DisplayForeignType
//     for DisplayForeignTypeSerializeDeserializeStructLifetime
// {
//     fn display_foreign_type(&self) -> std::string::String {
//         std::string::String::from("DisplayForeignTypeSerializeDeserializeStructLifetime")
//     }
// }

// #[derive(Hash, Eq, PartialEq)]
// pub struct DisplayForeignTypeStruct {
//     display_foreign_type_struct: std::string::String,
// }
// impl error_occurence_lib::DisplayForeignType for DisplayForeignTypeStruct {
//     fn display_foreign_type(&self) -> std::string::String {
//         std::string::String::from("DisplayForeignTypeStruct")
//     }
// }
// #[derive(Hash, Eq, PartialEq)]
// pub struct DisplayForeignTypeStructLifetime {
//     display_foreign_type_struct: std::string::String,
// }
// impl error_occurence_lib::DisplayForeignType for DisplayForeignTypeStructLifetime {
//     fn display_foreign_type(&self) -> std::string::String {
//         std::string::String::from("DisplayForeignTypeStruct")
//     }
// }

// #[derive(thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum ErrorNamed {
//     Something {
//         #[eo_display]
//         eo_display: DisplayStruct,
//         #[eo_display]
//         eo_display_lifetime: DisplayStructLifetime,

//         #[eo_display_with_serialize_deserialize]
//         eo_str_display_with_serialize_deserialize: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         eo_string_display_with_serialize_deserialize: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         eo_display_with_serialize_deserialize: DisplayWithSerializeDeserializeStruct,
//         #[eo_display_with_serialize_deserialize]
//         eo_display_with_serialize_deserialize_lifetime: DisplayWithSerializeDeserializeStructLifetime,

//         #[eo_display_foreign_type]
//         eo_display_foreign_type: DisplayForeignTypeStruct,
//         #[eo_display_foreign_type]
//         eo_display_foreign_type_lifetime: DisplayForeignTypeStructLifetime,

//         #[eo_display_foreign_type_with_serialize_deserialize]
//         eo_display_foreign_type_with_serialize_deserialize: DisplayForeignTypeSerializeDeserializeStruct,
//         #[eo_display_foreign_type_with_serialize_deserialize]
//         eo_display_foreign_type_with_serialize_deserialize_lifetime: DisplayForeignTypeSerializeDeserializeStructLifetime,

//         #[eo_error_occurence]
//         eo_error_occurence: InnerErrorNamed,

//         #[eo_vec_display]
//         eo_vec_display: std::vec::Vec<DisplayStruct>,
//         #[eo_vec_display]
//         eo_vec_display_lifetime: std::vec::Vec<DisplayStructLifetime>,

//         #[eo_vec_display_with_serialize_deserialize]
//         eo_vec_str_display_with_serialize_deserialize: std::vec::Vec<std::string::String>,
//         #[eo_vec_display_with_serialize_deserialize]
//         eo_vec_string_display_with_serialize_deserialize: std::vec::Vec<std::string::String>,
//         #[eo_vec_display_with_serialize_deserialize]
//         eo_vec_display_with_serialize_deserialize: std::vec::Vec<DisplayWithSerializeDeserializeStruct>,
//         #[eo_vec_display_with_serialize_deserialize]
//         eo_vec_display_with_serialize_deserialize_lifetime: std::vec::Vec<DisplayWithSerializeDeserializeStructLifetime>,

//         #[eo_vec_display_foreign_type]
//         eo_vec_display_foreign_type: std::vec::Vec<DisplayForeignTypeStruct>,
//         #[eo_vec_display_foreign_type]
//         eo_vec_display_foreign_type_lifetime: std::vec::Vec<DisplayForeignTypeStructLifetime>,

//         #[eo_vec_display_foreign_type_with_serialize_deserialize]
//         eo_vec_display_foreign_type_with_serialize_deserialize: std::vec::Vec<DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_vec_display_foreign_type_with_serialize_deserialize]
//         eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: std::vec::Vec<DisplayForeignTypeSerializeDeserializeStructLifetime>,

//         #[eo_vec_error_occurence]
//         eo_vec_error_occurence: std::vec::Vec<ErrorUnnamed>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap<std::string::String, DisplayStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap<std::string::String, DisplayStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<std::string::String, DisplayStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<std::string::String, DisplayStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<String, DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<String, DisplayWithSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, DisplayWithSerializeDeserializeStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
//         eo_hashmap_key_str_value_display_foreign_type: std::collections::HashMap<String, DisplayForeignTypeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
//         eo_hashmap_key_string_value_display_foreign_type: std::collections::HashMap<std::string::String, DisplayForeignTypeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
//         eo_hashmap_key_str_value_display_foreign_type_lifetime: std::collections::HashMap<String, DisplayForeignTypeStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
//         eo_hashmap_key_string_value_display_foreign_type_lifetime: std::collections::HashMap<std::string::String, DisplayForeignTypeStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<String, DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<std::string::String, DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<String, DisplayForeignTypeSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, DisplayForeignTypeSerializeDeserializeStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<String, ErrorUnnamed>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<std::string::String, ErrorUnnamed>,

//         #[eo_hashmap_key_display_foreign_type_value_display]
//         eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap<DisplayForeignTypeStruct, DisplayStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap<DisplayForeignTypeStructLifetime, DisplayStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display]
//         eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap<DisplayForeignTypeStruct, DisplayStructLifetime>,
//         #[eo_hashmap_key_display_foreign_type_value_display]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap<DisplayForeignTypeStructLifetime, DisplayStructLifetime>,

//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_str_display_with_serialize_deserialize: std::collections::HashMap<DisplayForeignTypeStruct, std::string::String>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap<DisplayForeignTypeStructLifetime, std::string::String>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_string_display_with_serialize_deserialize: std::collections::HashMap<DisplayForeignTypeStruct, std::string::String>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap<DisplayForeignTypeStructLifetime, std::string::String>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap<DisplayForeignTypeStruct, DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap<DisplayForeignTypeStructLifetime, DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<DisplayForeignTypeStruct, DisplayWithSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<
//             DisplayForeignTypeStructLifetime,
//             DisplayWithSerializeDeserializeStructLifetime,
//         >,

//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap<DisplayForeignTypeStruct, DisplayForeignTypeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap<DisplayForeignTypeStructLifetime, DisplayForeignTypeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap<DisplayForeignTypeStruct, DisplayForeignTypeStructLifetime>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap<DisplayForeignTypeStructLifetime, DisplayForeignTypeStructLifetime>,

//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<DisplayForeignTypeStruct, DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<DisplayForeignTypeStructLifetime, DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<DisplayForeignTypeStruct, DisplayForeignTypeSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<DisplayForeignTypeStructLifetime, DisplayForeignTypeSerializeDeserializeStructLifetime>,

//         #[eo_hashmap_key_display_foreign_type_value_error_occurence]
//         eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap<DisplayForeignTypeStruct, ErrorUnnamed>,
//         #[eo_hashmap_key_display_foreign_type_value_error_occurence]
//         eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap<DisplayForeignTypeStructLifetime, ErrorUnnamed>,

//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
