#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurenceTest
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
    println!("{e}");
    let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
    println!("--------------------------------------------------------------------------------------------------");
    println!("{e_serialize_deserialize_version}");
    let e_serialize_deserialize_version_json_string = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
    println!("{e_serialize_deserialize_version_json_string}");
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    let e_serialize_deserialize_version_deserialized: ErrorNamedOneWithSerializeDeserialize = serde_json::from_str(&e_serialize_deserialize_version_json_string).unwrap();
    println!("{e_serialize_deserialize_version_deserialized}");
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
}



// {
//  eo_display_field: value
//  eo_display_with_serialize_deserialize_field: value
//  eo_display_foreign_type_field: DisplayForeignTypeStruct {
//      display_foreign_type: "value",
//  }
//  eo_display_foreign_type_with_serialize_deserialize_field: DisplayForeignTypeWithSerializeDeserializeStruct {
//      display_foreign_type_with_serialize_deserialize: "value",
//  }
//  eo_error_occurence_field: {
//   eo_display_with_serialize_deserialize_field: value
//  }
//  https://github.com/kuqmua/tufa_project/blob/491d61409e9874f36e541815c92ecb16088155e3/error_occurence_test/src/main.rs#L103 2024-04-29 16:54:51
//  eo_vec_display_field: [
//   value
//  ]
//  eo_vec_display_with_serialize_deserialize_field: [
//   value
//  ]
//  eo_vec_display_foreign_type_field: [
//   DisplayForeignTypeStruct {
//      display_foreign_type: "value",
//  }
//  ]
//  eo_vec_display_foreign_type_with_serialize_deserialize_field: [
//   DisplayForeignTypeWithSerializeDeserializeStruct {
//      display_foreign_type_with_serialize_deserialize: "value",
//  }
//  ]
//  eo_vec_error_occurence_field: [
//   {
//    eo_display_with_serialize_deserialize_field: value
//   }
//   https://github.com/kuqmua/tufa_project/blob/491d61409e9874f36e541815c92ecb16088155e3/error_occurence_test/src/main.rs#L127 2024-04-29 16:54:51
//  ]
// }
// https://github.com/kuqmua/tufa_project/blob/491d61409e9874f36e541815c92ecb16088155e3/error_occurence_test/src/main.rs#L132 2024-04-29 16:54:51
// --------------------------------------------------------------------------------------------------
// {
//  eo_display_field: value
//  eo_display_with_serialize_deserialize_field: value
//  eo_display_foreign_type_field: DisplayForeignTypeStruct {
//      display_foreign_type: "value",
//  }
//  eo_display_foreign_type_with_serialize_deserialize_field: DisplayForeignTypeWithSerializeDeserializeStruct {
//      display_foreign_type_with_serialize_deserialize: "value",
//  }
//  eo_error_occurence_field: {
//   eo_display_with_serialize_deserialize_field: value
//  }
//  https://github.com/kuqmua/tufa_project/blob/491d61409e9874f36e541815c92ecb16088155e3/error_occurence_test/src/main.rs#L103 2024-04-29 16:54:51
//  eo_vec_display_field: [
//   value
//  ]
//  eo_vec_display_with_serialize_deserialize_field: [
//   value
//  ]
//  eo_vec_display_foreign_type_field: [
//   DisplayForeignTypeStruct {
//      display_foreign_type: "value",
//  }
//  ]
//  eo_vec_display_foreign_type_with_serialize_deserialize_field: [
//   DisplayForeignTypeWithSerializeDeserializeStruct {
//      display_foreign_type_with_serialize_deserialize: "value",
//  }
//  ]
//  eo_vec_error_occurence_field: [
//   {
//    eo_display_with_serialize_deserialize_field: value
//   }
//   https://github.com/kuqmua/tufa_project/blob/491d61409e9874f36e541815c92ecb16088155e3/error_occurence_test/src/main.rs#L127 2024-04-29 16:54:51
//  ]
// }
// https://github.com/kuqmua/tufa_project/blob/491d61409e9874f36e541815c92ecb16088155e3/error_occurence_test/src/main.rs#L132 2024-04-29 16:54:51
// {"Variant":{"eo_display_field":"value","eo_display_with_serialize_deserialize_field":"value","eo_display_foreign_type_field":"DisplayForeignTypeStruct {\n    display_foreign_type: \"value\",\n}","eo_display_foreign_type_with_serialize_deserialize_field":{"display_foreign_type_with_serialize_deserialize":"value"},"eo_error_occurence_field":{"Variant":{"eo_display_with_serialize_deserialize_field":"value","code_occurence":{"file":"error_occurence_test/src/main.rs","line":103,"column":29,"commit":"491d61409e9874f36e541815c92ecb16088155e3","duration":{"secs":1714398891,"nanos":355009571},"macro_occurence":null}}},"eo_vec_display_field":["value"],"eo_vec_display_with_serialize_deserialize_field":["value"],"eo_vec_display_foreign_type_field":["DisplayForeignTypeStruct {\n    display_foreign_type: \"value\",\n}"],"eo_vec_display_foreign_type_with_serialize_deserialize_field":[{"display_foreign_type_with_serialize_deserialize":"value"}],"eo_vec_error_occurence_field":[{"Something":{"Variant":{"eo_display_with_serialize_deserialize_field":"value","code_occurence":{"file":"error_occurence_test/src/main.rs","line":127,"column":37,"commit":"491d61409e9874f36e541815c92ecb16088155e3","duration":{"secs":1714398891,"nanos":355012297},"macro_occurence":null}}}}],"code_occurence":{"file":"error_occurence_test/src/main.rs","line":132,"column":25,"commit":"491d61409e9874f36e541815c92ecb16088155e3","duration":{"secs":1714398891,"nanos":355012717},"macro_occurence":null}}}
// +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// {
//  eo_display_field: value
//  eo_display_with_serialize_deserialize_field: value
//  eo_display_foreign_type_field: DisplayForeignTypeStruct {
//      display_foreign_type: "value",
//  }
//  eo_display_foreign_type_with_serialize_deserialize_field: DisplayForeignTypeWithSerializeDeserializeStruct {
//      display_foreign_type_with_serialize_deserialize: "value",
//  }
//  eo_error_occurence_field: {
//   eo_display_with_serialize_deserialize_field: value
//  }
//  https://github.com/kuqmua/tufa_project/blob/491d61409e9874f36e541815c92ecb16088155e3/error_occurence_test/src/main.rs#L103 2024-04-29 16:54:51
//  eo_vec_display_field: [
//   value
//  ]
//  eo_vec_display_with_serialize_deserialize_field: [
//   value
//  ]
//  eo_vec_display_foreign_type_field: [
//   DisplayForeignTypeStruct {
//      display_foreign_type: "value",
//  }
//  ]
//  eo_vec_display_foreign_type_with_serialize_deserialize_field: [
//   DisplayForeignTypeWithSerializeDeserializeStruct {
//      display_foreign_type_with_serialize_deserialize: "value",
//  }
//  ]
//  eo_vec_error_occurence_field: [
//   {
//    eo_display_with_serialize_deserialize_field: value
//   }
//   https://github.com/kuqmua/tufa_project/blob/491d61409e9874f36e541815c92ecb16088155e3/error_occurence_test/src/main.rs#L127 2024-04-29 16:54:51
//  ]
// }
// https://github.com/kuqmua/tufa_project/blob/491d61409e9874f36e541815c92ecb16088155e3/error_occurence_test/src/main.rs#L132 2024-04-29 16:54:51
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@