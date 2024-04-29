#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence
)]
pub enum ErrorNamedOne {
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
        eo_error_occurence_field: ErrorNamedTwo,
        #[eo_vec_display]
        eo_vec_display_field: std::vec::Vec<DisplayStruct>,
        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_display_with_serialize_deserialize_field: std::vec::Vec<std::string::String>,
        #[eo_vec_display_foreign_type]
        eo_vec_display_foreign_type_field: std::vec::Vec<DisplayForeignTypeStruct>,
        #[eo_vec_display_foreign_type_with_serialize_deserialize]
        eo_vec_display_foreign_type_with_serialize_deserialize_field: std::vec::Vec<DisplayForeignTypeWithSerializeDeserializeStruct>,
        #[eo_vec_error_occurence]
        eo_vec_error_occurence_field: std::vec::Vec<ErrorUnnamedOne>,

        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
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
pub enum ErrorNamedTwo {
    Variant {
        #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum ErrorUnnamedOne {
    #[eo_error_occurence]
    Something(ErrorNamedTwo),
}

fn main() {
    println!("1");
    test_code_occurence();
    println!("2");
}

pub fn test_code_occurence() {
    let e = ErrorNamedOne::Variant {
        eo_display_field: DisplayStruct {
            display: std::string::String::from("value")
        },
        eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
        eo_display_foreign_type_field: DisplayForeignTypeStruct {
            display_foreign_type: std::string::String::from("value"),
        },
        eo_display_foreign_type_with_serialize_deserialize_field: DisplayForeignTypeWithSerializeDeserializeStruct {
            display_foreign_type_with_serialize_deserialize: std::string::String::from("value"),
        },
        eo_error_occurence_field: ErrorNamedTwo::Variant {
            eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
            code_occurence: error_occurence_lib::code_occurence!(),
        },
        eo_vec_display_field: vec![
            DisplayStruct {
                display: std::string::String::from("value")
            }
        ],
        eo_vec_display_with_serialize_deserialize_field: vec![
            std::string::String::from("value")
        ],
        eo_vec_display_foreign_type_field: vec![
            DisplayForeignTypeStruct {
                display_foreign_type: std::string::String::from("value")
            }
        ],
        eo_vec_display_foreign_type_with_serialize_deserialize_field: vec![
            DisplayForeignTypeWithSerializeDeserializeStruct {
                display_foreign_type_with_serialize_deserialize: std::string::String::from("value"),
            }
        ],
        eo_vec_error_occurence_field: vec![
            ErrorUnnamedOne::Something(
                ErrorNamedTwo::Variant {
                    eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }
            )
        ],

        code_occurence: error_occurence_lib::code_occurence!(),
    };

    // let git_info = crate::common::git::git_info::GitInfo::default();
    // let code_occurence = error_occurence_lib::code_occurence::CodeOccurence::new(
    //     git_info.to_git_info_without_lifetime(),
    //     file!().to_string(),
    //     line!(),
    //     column!(),
    //     {
    //         app_state::GetServerPort::get_server_port(
    //             *crate::config_mods::config_struct::ConfigUnchecked::default(),
    //         )
    //     },
    // );
       
    // let e = ErrorNamed::Something {
    //     // eo_display: DisplayStruct {
    //     //     display_struct: std::string::String::from("String")
    //     // },
    //     // eo_display_lifetime: DisplayStructLifetime {
    //     //     display_struct_lifetime: std::string::String::from("str")
    //     // },

    //     // eo_str_display_with_serialize_deserialize: std::string::String::from("str"),
    //     // eo_string_display_with_serialize_deserialize: std::string::String::from("String"),
    //     // eo_display_with_serialize_deserialize: DisplayWithSerializeDeserializeStruct {},
    //     // eo_display_with_serialize_deserialize_lifetime: DisplayWithSerializeDeserializeStructLifetime {
    //     //     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str")
    //     // },

    //     // eo_display_foreign_type: DisplayForeignTypeStruct {
    //     //     display_foreign_type_struct: std::string::String::from("String")
    //     // },
    //     // eo_display_foreign_type_lifetime: DisplayForeignTypeStructLifetime {
    //     //     display_foreign_type_struct: std::string::String::from("str")
    //     // },

    //     // eo_display_foreign_type_with_serialize_deserialize: DisplayForeignTypeSerializeDeserializeStruct {
    //     //     display_foreign_type_serialize_deserialize_struct: std::string::String::from("String")
    //     // },
    //     // eo_display_foreign_type_with_serialize_deserialize_lifetime: DisplayForeignTypeSerializeDeserializeStructLifetime {
    //     //     display_foreign_type_serialize_deserialize_struct: std::string::String::from("str")
    //     // },

    //     // eo_error_occurence: InnerErrorNamed::Something {
    //     //     string: std::string::String::from("String"),
    //     //     display_with_serialize_deserialize_struct_lifetime: DisplayWithSerializeDeserializeStructLifetime {
    //     //         display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
    //     //     },
    //     //     code_occurence: code_occurence.clone(),
    //     // },

    //     // eo_vec_str_display_with_serialize_deserialize: vec![std::string::String::from("str")],
    //     // eo_vec_string_display_with_serialize_deserialize:  vec![std::string::String::from("String")],
    //     // eo_vec_display: vec![DisplayStruct { display_struct: std::string::String::from("String") }],
    //     // eo_vec_display_lifetime: vec![DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") }],

    //     // eo_vec_display_with_serialize_deserialize: vec![DisplayWithSerializeDeserializeStruct {}],
    //     // eo_vec_display_with_serialize_deserialize_lifetime: vec![DisplayWithSerializeDeserializeStructLifetime { display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") }],

    //     // eo_vec_display_foreign_type: vec![DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") }],
    //     // eo_vec_display_foreign_type_lifetime: vec![DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") }],

    //     // eo_vec_display_foreign_type_with_serialize_deserialize: vec![DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") }],
    //     // eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: vec![DisplayForeignTypeSerializeDeserializeStructLifetime{ display_foreign_type_serialize_deserialize_struct: std::string::String::from("str") }],

    //     // eo_vec_error_occurence: vec![ErrorUnnamed::Something(InnerErrorNamed::Something {
    //     //     string: std::string::String::from("String"),
    //     //     display_with_serialize_deserialize_struct_lifetime: DisplayWithSerializeDeserializeStructLifetime {
    //     //         display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
    //     //     },
    //     //     code_occurence: code_occurence.clone(),
    //     // })],

    //     // code_occurence,
    // };
    // println!("{e}");
    // let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
    // println!("{e_serialize_deserialize_version}");
    // let e_json = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
    // println!("{e_json}");
    // let e_deserialized: ErrorNamedWithSerializeDeserialize = serde_json::from_str(&e_json).unwrap();
    // println!("{e_deserialized}");
}

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

//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
