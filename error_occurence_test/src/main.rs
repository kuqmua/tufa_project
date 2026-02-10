// todo is there is no point to add additional info to enum like this
// eo_error_occurence_field: {
//     eo_display_with_serialize_deserialize_field: value
// }
// https://github.com/kuqmua/tufa_project/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/error_occurence_test/src/main.rs#L85 2024-05-06 09:17:23
// impl display like this this
// eo_error_occurence_field
// https://github.com/kuqmua/tufa_project/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/error_occurence_test/src/main.rs#L85 2024-05-06 09:17:23

use error_occurence_lib::code_occurence::CodeOccurence;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum ErrorNamedOne {
    //use ToStdStringString for hashmap keys instead of Display
    //todo even for String in serialize deserialize version of error must be using ToStdStringString impl instead of std::fmt::Display
    //todo test on using only code_occurence as pnly field in named variant
    Variant {
        #[eo_to_std_string_string]
        eo_display_field: DisplayStruct, //IN SERIALIZE DESERIALIZE String
        #[eo_to_std_string_string_serialize_deserialize]
        eo_serde: SerializeDeserializeStruct,
        #[eo_error_occurence]
        eo_error_occurence_field: ErrorNamedTwo, //IN SERIALIZE DESERIALIZE nested
        #[eo_vec_to_std_string_string] //todo remove wrapper under Vec
        eo_vec_display_field: Vec<DisplayStruct>, //IN SERIALIZE DESERIALIZE Vec<String>
        #[eo_vec_to_std_string_string_serialize_deserialize]
        eo_vec_serde: Vec<SerializeDeserializeStruct>,
        #[eo_vec_error_occurence]
        eo_vec_error_occurence_field: Vec<ErrorUnnamedOne>, //IN SERIALIZE DESERIALIZE Vec<nested>
        #[eo_hashmap_key_std_string_string_value_to_std_string_string]
        hashmap_string_string: HashMap<String, DisplayStruct>,
        #[eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize]
        hashmap_string_serde: HashMap<String, SerializeDeserializeStruct>,
        #[eo_hashmap_key_std_string_string_value_error_occurence]
        hashmap_string_error_occurence: HashMap<String, ErrorUnnamedOne>,

        code_occurence: CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum ErrorNamedTwo {
    Another {
        #[eo_to_std_string_string_serialize_deserialize]
        sdasdasd: String,
        code_occurence: CodeOccurence,
    },
    Variant {
        #[eo_to_std_string_string_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: String,
        code_occurence: CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum ErrorUnnamedOne {
    Something(ErrorNamedTwo),
}

#[derive(Debug)]
pub struct DisplayStruct {
    pub display: String,
    pub something: bool,
}

//todo or maybe two different traits - display foreign type and convert into serializable and deserializable type
impl error_occurence_lib::ToStdStringString for DisplayStruct {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SerializeDeserializeStruct {
    pub one: String,
    pub two: bool,
    pub three: u32,
}

impl error_occurence_lib::ToStdStringString for SerializeDeserializeStruct {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}

fn main() {
    let error = ErrorNamedOne::Variant {
        eo_display_field: DisplayStruct {
            display: String::from("value"),
            something: true,
        },
        eo_serde: SerializeDeserializeStruct {
            one: String::from("value"),
            two: true,
            three: 42,
        },
        eo_error_occurence_field: ErrorNamedTwo::Variant {
            eo_display_with_serialize_deserialize_field: String::from("value"),
            code_occurence: error_occurence_lib::code_occurence!(),
        },
        eo_vec_display_field: vec![
            DisplayStruct {
                display: String::from("08708789"),
                something: true,
            },
            DisplayStruct {
                display: String::from("7565757"),
                something: true,
            },
        ],
        eo_vec_serde: vec![
            SerializeDeserializeStruct {
                one: String::from("value"),
                two: true,
                three: 42,
            },
            SerializeDeserializeStruct {
                one: String::from("97697697"),
                two: false,
                three: 422,
            },
        ],
        eo_vec_error_occurence_field: vec![
            ErrorUnnamedOne::Something(ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field: String::from("value"),
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
            ErrorUnnamedOne::Something(ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field: String::from("123"),
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        ],
        hashmap_string_string: HashMap::from([
            (
                String::from("kesdfsfdsfsd"),
                DisplayStruct {
                    display: String::from("vasfdsdfsdflue"),
                    something: true,
                },
            ),
            (
                String::from("ksdfsdfsdfsdfey"),
                DisplayStruct {
                    display: String::from("valsfdsfdsfdsue"),
                    something: true,
                },
            ),
        ]),
        hashmap_string_serde: HashMap::from([
            (
                String::from("kdfgsdfgdsfgey"),
                SerializeDeserializeStruct {
                    one: String::from("valusdfgdsgdsfgde"),
                    two: true,
                    three: 42,
                },
            ),
            (
                String::from("ksdfgdsfgsdfgey"),
                SerializeDeserializeStruct {
                    one: String::from("valsdfgdsgdue"),
                    two: true,
                    three: 42,
                },
            ),
        ]),
        hashmap_string_error_occurence: HashMap::from([
            (
                String::from("ksdfgadsfgsdfgdfgey"),
                ErrorUnnamedOne::Something(ErrorNamedTwo::Variant {
                    eo_display_with_serialize_deserialize_field: String::from("vasdfgdgdfglue"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }),
            ),
            (
                String::from("kesdfgsdgfdfgy"),
                ErrorUnnamedOne::Something(ErrorNamedTwo::Variant {
                    eo_display_with_serialize_deserialize_field: String::from("valsdfgdsafgdsgue"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }),
            ),
        ]),

        code_occurence: error_occurence_lib::code_occurence!(),
    };
    println!("{error:?}");
}
// ///////////
