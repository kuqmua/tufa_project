// #[test]
// pub fn test_code_occurence() {
//     let git_info = crate::common::git::git_info::GitInfo::default();
//     let code_occurence = crate::common::code_occurence::CodeOccurence::new(
//         git_info.to_git_info_without_lifetime(),
//         file!().to_string(),
//         line!(),
//         column!(),
//         {
//             app_state::GetServerPort::get_server_port(
//                 *crate::config_mods::config_struct::ConfigUnchecked::default(),
//             )
//         },
//     );
//     let e = ErrorNamed::Something {
//         eo_display: crate::common::error_logs_logic::test::DisplayStruct {
//             display_struct: std::string::String::from("String")
//         },
//         eo_display_lifetime: crate::common::error_logs_logic::test::DisplayStructLifetime {
//             display_struct_lifetime: std::string::String::from("str")
//         },

//         eo_str_display_with_serialize_deserialize: std::string::String::from("str"),
//         eo_string_display_with_serialize_deserialize: std::string::String::from("String"),
//         eo_display_with_serialize_deserialize: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {},
//         eo_display_with_serialize_deserialize_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//             display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str")
//         },

//         eo_to_std_string_string: crate::common::error_logs_logic::test::ToStdStringStringStruct {
//             to_std_string_string_struct: std::string::String::from("String")
//         },
//         eo_to_std_string_string_lifetime: crate::common::error_logs_logic::test::ToStdStringStringStructLifetime {
//             to_std_string_string_struct: std::string::String::from("str")
//         },

//         eo_to_std_string_string_with_serialize_deserialize: crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct {
//             to_std_string_string_serialize_deserialize_struct: std::string::String::from("String")
//         },
//         eo_to_std_string_string_with_serialize_deserialize_lifetime: crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime {
//             to_std_string_string_serialize_deserialize_struct: std::string::String::from("str")
//         },

//         eo_error_occurence: crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//             string: std::string::String::from("String"),
//             display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//             code_occurence: code_occurence.clone(),
//         },

//         eo_vec_str_display_with_serialize_deserialize: vec![std::string::String::from("str")],
//         eo_vec_string_display_with_serialize_deserialize:  vec![std::string::String::from("String")],
//         eo_vec_display: vec![crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") }],
//         eo_vec_display_lifetime: vec![crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") }],

//         eo_vec_display_with_serialize_deserialize: vec![crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {}],
//         eo_vec_display_with_serialize_deserialize_lifetime: vec![crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime { display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") }],

//         eo_vec_to_std_string_string: vec![crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") }],
//         eo_vec_to_std_string_string_lifetime: vec![crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") }],

//         eo_vec_to_std_string_string_with_serialize_deserialize: vec![crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct { to_std_string_string_serialize_deserialize_struct: std::string::String::from("String") }],
//         eo_vec_to_std_string_string_with_serialize_deserialize_lifetime: vec![crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime{ to_std_string_string_serialize_deserialize_struct: std::string::String::from("str") }],

//         eo_vec_error_occurence: vec![crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//             string: std::string::String::from("String"),
//             display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//             code_occurence: code_occurence.clone(),
//         })],

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
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
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {},
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {},
//         )]),
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//         )]),

//         eo_hashmap_key_str_value_to_std_string_string: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_string_value_to_std_string_string: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_str_value_to_std_string_string_lifetime: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_string_value_to_std_string_string_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_str_value_to_std_string_string_with_serialize_deserialize: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct { to_std_string_string_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_string_value_to_std_string_string_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct { to_std_string_string_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_str_value_to_std_string_string_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime { to_std_string_string_serialize_deserialize_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_string_value_to_std_string_string_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime { to_std_string_string_serialize_deserialize_struct: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),

//         eo_hashmap_key_to_std_string_string_value_display: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_display: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_to_std_string_string_value_display_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_display_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_to_std_string_string_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             "str",
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             "str",
//         )]),
//         eo_hashmap_key_to_std_string_string_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             std::string::String::from("String"),
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             std::string::String::from("String"),
//         )]),
//         eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct{},
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct{},
//         )]),
//         eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_to_std_string_string_value_to_std_string_string: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_to_std_string_string: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_to_std_string_string_value_to_std_string_string_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_to_std_string_string_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct { to_std_string_string_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_to_std_string_string_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct { to_std_string_string_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime { to_std_string_string_serialize_deserialize_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_to_std_string_string_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime { to_std_string_string_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),

//         eo_hashmap_key_to_std_string_string_value_error_occurence: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStruct { to_std_string_string_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),
//         eo_hashmap_key_to_std_string_string_lifetime_value_error_occurence: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime { to_std_string_string_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
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
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.display_struct)
//     }
// }

// pub struct DisplayStructLifetime {
//     display_struct_lifetime: std::string::String,
// }
// impl std::fmt::Display for DisplayStructLifetime {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.display_struct_lifetime)
//     }
// }

// #[derive(PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
// pub struct DisplayWithSerializeDeserializeStruct {}
// impl std::fmt::Display for DisplayWithSerializeDeserializeStruct {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "DisplayWithSerializeDeserializeStruct")
//     }
// }

// #[derive(PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
// pub struct DisplayWithSerializeDeserializeStructLifetime {
//     display_with_serialize_deserialize_struct_lifetime: std::string::String,
// }
// impl std::fmt::Display for DisplayWithSerializeDeserializeStructLifetime {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "DisplayWithSerializeDeserializeStructLifetime")
//     }
// }

// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct ToStdStringStringSerializeDeserializeStruct {
//     to_std_string_string_serialize_deserialize_struct: std::string::String,
// }
// impl crate::common::to_std_string_string::ToStdStringString
//     for ToStdStringStringSerializeDeserializeStruct
// {
//     fn to_std_string_string(&self) -> std::string::String {
//         std::string::String::from("ToStdStringStringSerializeDeserializeStruct")
//     }
// }
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct ToStdStringStringSerializeDeserializeStructLifetime {
//     to_std_string_string_serialize_deserialize_struct: std::string::String,
// }
// impl crate::common::to_std_string_string::ToStdStringString
//     for ToStdStringStringSerializeDeserializeStructLifetime
// {
//     fn to_std_string_string(&self) -> std::string::String {
//         std::string::String::from("ToStdStringStringSerializeDeserializeStructLifetime")
//     }
// }

// #[derive(Hash, Eq, PartialEq)]
// pub struct ToStdStringStringStruct {
//     to_std_string_string_struct: std::string::String,
// }
// impl crate::common::to_std_string_string::ToStdStringString for ToStdStringStringStruct {
//     fn to_std_string_string(&self) -> std::string::String {
//         std::string::String::from("ToStdStringStringStruct")
//     }
// }
// #[derive(Hash, Eq, PartialEq)]
// pub struct ToStdStringStringStructLifetime {
//     to_std_string_string_struct: std::string::String,
// }
// impl crate::common::to_std_string_string::ToStdStringString for ToStdStringStringStructLifetime {
//     fn to_std_string_string(&self) -> std::string::String {
//         std::string::String::from("ToStdStringStringStruct")
//     }
// }

// #[derive(thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum InnerErrorNamed {
//     Something {
//         #[eo_to_std_string_string_serialize_deserialize]
//         string: std::string::String,
//         #[eo_to_std_string_string_serialize_deserialize]
//         display_with_serialize_deserialize_struct_lifetime:
//             DisplayWithSerializeDeserializeStructLifetime,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum ErrorUnnamed {
//     #[eo_error_occurence]
//     Something(crate::common::error_logs_logic::test::InnerErrorNamed),
// }

// #[derive(thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum ErrorNamed {
//     Something {
//         #[eo_to_std_string_string]
//         eo_display: crate::common::error_logs_logic::test::DisplayStruct,
//         #[eo_to_std_string_string]
//         eo_display_lifetime: crate::common::error_logs_logic::test::DisplayStructLifetime,

//         #[eo_to_std_string_string_serialize_deserialize]
//         eo_str_display_with_serialize_deserialize: std::string::String,
//         #[eo_to_std_string_string_serialize_deserialize]
//         eo_string_display_with_serialize_deserialize: std::string::String,
//         #[eo_to_std_string_string_serialize_deserialize]
//         eo_display_with_serialize_deserialize: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct,
//         #[eo_to_std_string_string_serialize_deserialize]
//         eo_display_with_serialize_deserialize_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime,

//         #[eo_to_std_string_string]
//         eo_to_std_string_string: crate::common::error_logs_logic::test::ToStdStringStringStruct,
//         #[eo_to_std_string_string]
//         eo_to_std_string_string_lifetime: crate::common::error_logs_logic::test::ToStdStringStringStructLifetime,

//         #[eo_to_std_string_string_with_serialize_deserialize]
//         eo_to_std_string_string_with_serialize_deserialize: crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct,
//         #[eo_to_std_string_string_with_serialize_deserialize]
//         eo_to_std_string_string_with_serialize_deserialize_lifetime: crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime,

//         #[eo_error_occurence]
//         eo_error_occurence: crate::common::error_logs_logic::test::InnerErrorNamed,

//         #[eo_vec_to_std_string_string]
//         eo_vec_display: std::vec::Vec<crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_vec_to_std_string_string]
//         eo_vec_display_lifetime: std::vec::Vec<crate::common::error_logs_logic::test::DisplayStructLifetime>,

//         #[eo_vec_to_std_string_string_serialize_deserialize]
//         eo_vec_str_display_with_serialize_deserialize: std::vec::Vec<std::string::String>,
//         #[eo_vec_to_std_string_string_serialize_deserialize]
//         eo_vec_string_display_with_serialize_deserialize: std::vec::Vec<std::string::String>,
//         #[eo_vec_to_std_string_string_serialize_deserialize]
//         eo_vec_display_with_serialize_deserialize: std::vec::Vec<crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_vec_to_std_string_string_serialize_deserialize]
//         eo_vec_display_with_serialize_deserialize_lifetime: std::vec::Vec<crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime>,

//         #[eo_vec_to_std_string_string]
//         eo_vec_to_std_string_string: std::vec::Vec<crate::common::error_logs_logic::test::ToStdStringStringStruct>,
//         #[eo_vec_to_std_string_string]
//         eo_vec_to_std_string_string_lifetime: std::vec::Vec<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime>,

//         #[eo_vec_to_std_string_string_with_serialize_deserialize]
//         eo_vec_to_std_string_string_with_serialize_deserialize: std::vec::Vec<crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct>,
//         #[eo_vec_to_std_string_string_with_serialize_deserialize]
//         eo_vec_to_std_string_string_with_serialize_deserialize_lifetime: std::vec::Vec<crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime>,

//         #[eo_vec_error_occurence]
//         eo_vec_error_occurence: std::vec::Vec<crate::common::error_logs_logic::test::ErrorUnnamed>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<String, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<String, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string]
//         eo_hashmap_key_str_value_to_std_string_string: std::collections::HashMap<String, crate::common::error_logs_logic::test::ToStdStringStringStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string]
//         eo_hashmap_key_string_value_to_std_string_string: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::ToStdStringStringStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string]
//         eo_hashmap_key_str_value_to_std_string_string_lifetime: std::collections::HashMap<String, crate::common::error_logs_logic::test::ToStdStringStringStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string]
//         eo_hashmap_key_string_value_to_std_string_string_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::ToStdStringStringStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string_with_serialize_deserialize]
//         eo_hashmap_key_str_value_to_std_string_string_with_serialize_deserialize: std::collections::HashMap<String, crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string_with_serialize_deserialize]
//         eo_hashmap_key_string_value_to_std_string_string_with_serialize_deserialize: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string_with_serialize_deserialize]
//         eo_hashmap_key_str_value_to_std_string_string_with_serialize_deserialize_lifetime: std::collections::HashMap<String, crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_to_std_string_string_with_serialize_deserialize]
//         eo_hashmap_key_string_value_to_std_string_string_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<String, crate::common::error_logs_logic::test::ErrorUnnamed>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::ErrorUnnamed>,

//         #[eo_hashmap_key_to_std_string_string_value_display]
//         eo_hashmap_key_to_std_string_string_value_display: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_hashmap_key_to_std_string_string_value_display]
//         eo_hashmap_key_to_std_string_string_lifetime_value_display: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_hashmap_key_to_std_string_string_value_display]
//         eo_hashmap_key_to_std_string_string_value_display_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, crate::common::error_logs_logic::test::DisplayStructLifetime>,
//         #[eo_hashmap_key_to_std_string_string_value_display]
//         eo_hashmap_key_to_std_string_string_lifetime_value_display_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, crate::common::error_logs_logic::test::DisplayStructLifetime>,

//         #[eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_value_str_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, std::string::String>,
//         #[eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, std::string::String>,
//         #[eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_value_string_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, std::string::String>,
//         #[eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, std::string::String>,
//         #[eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_to_std_string_string_value_display_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<
//             crate::common::error_logs_logic::test::ToStdStringStringStructLifetime,
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime,
//         >,

//         #[eo_hashmap_key_to_std_string_string_value_to_std_string_string]
//         eo_hashmap_key_to_std_string_string_value_to_std_string_string: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, crate::common::error_logs_logic::test::ToStdStringStringStruct>,
//         #[eo_hashmap_key_to_std_string_string_value_to_std_string_string]
//         eo_hashmap_key_to_std_string_string_lifetime_value_to_std_string_string: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, crate::common::error_logs_logic::test::ToStdStringStringStruct>,
//         #[eo_hashmap_key_to_std_string_string_value_to_std_string_string]
//         eo_hashmap_key_to_std_string_string_value_to_std_string_string_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, crate::common::error_logs_logic::test::ToStdStringStringStructLifetime>,
//         #[eo_hashmap_key_to_std_string_string_value_to_std_string_string]
//         eo_hashmap_key_to_std_string_string_lifetime_value_to_std_string_string_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, crate::common::error_logs_logic::test::ToStdStringStringStructLifetime>,

//         #[eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct>,
//         #[eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_lifetime_value_to_std_string_string_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStruct>,
//         #[eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_to_std_string_string_value_to_std_string_string_with_serialize_deserialize]
//         eo_hashmap_key_to_std_string_string_lifetime_value_to_std_string_string_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, crate::common::error_logs_logic::test::ToStdStringStringSerializeDeserializeStructLifetime>,

//         #[eo_hashmap_key_to_std_string_string_value_error_occurence]
//         eo_hashmap_key_to_std_string_string_value_error_occurence: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStruct, crate::common::error_logs_logic::test::ErrorUnnamed>,
//         #[eo_hashmap_key_to_std_string_string_value_error_occurence]
//         eo_hashmap_key_to_std_string_string_lifetime_value_error_occurence: std::collections::HashMap<crate::common::error_logs_logic::test::ToStdStringStringStructLifetime, crate::common::error_logs_logic::test::ErrorUnnamed>,

//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
