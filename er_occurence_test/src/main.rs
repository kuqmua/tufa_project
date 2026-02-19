// todo is there is no point to add additional info to enum like this
// eo_er_occurence_field: {
//     eo_display_with_serde_field: value
// }
// https://github.com/kuqmua/tufa_project/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/er_occurence_test/src/main.rs#L85 2024-05-06 09:17:23
// impl display like this this
// eo_er_occurence_field
// https://github.com/kuqmua/tufa_project/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/er_occurence_test/src/main.rs#L85 2024-05-06 09:17:23
use er_occurence_lib::{ErOccurence, ToErrString, code_occurence, code_occurence::CodeOccurence};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
#[derive(Debug, Error, ErOccurence)]
pub enum ErOne {
    //use ToErrString for hashmap keys instead of Display
    //todo even for String in serialize deserialize version of er must be using ToErrString impl instead of std::fmt::Display
    //todo test on using only code_occurence as pnly field in named variant
    Variant {
        #[eo_to_err_string]
        eo_display_field: DisplayStruct, //IN SERIALIZE DESERIALIZE String
        #[eo_to_err_string_serde]
        eo_serde: SerdeStruct,
        #[eo_er_occurence]
        eo_er_occurence_field: ErTwo, //IN SERIALIZE DESERIALIZE nested
        #[eo_vec_to_err_string] //todo remove wrapper under Vec
        eo_vec_display_field: Vec<DisplayStruct>, //IN SERIALIZE DESERIALIZE Vec<String>
        #[eo_vec_to_err_string_serde]
        eo_vec_serde: Vec<SerdeStruct>,
        #[eo_vec_er_occurence]
        eo_vec_er_occurence_field: Vec<ErUnnamedOne>, //IN SERIALIZE DESERIALIZE Vec<nested>
        #[eo_hashmap_key_string_value_to_err_string]
        hashmap_string_string: HashMap<String, DisplayStruct>,
        #[eo_hashmap_key_string_value_to_err_string_serde]
        hashmap_string_serde: HashMap<String, SerdeStruct>,
        #[eo_hashmap_key_string_value_er_occurence]
        hashmap_string_er_occurence: HashMap<String, ErUnnamedOne>,
        code_occurence: CodeOccurence,
    },
}
#[derive(Debug, Error, ErOccurence)]
pub enum ErTwo {
    Another {
        #[eo_to_err_string_serde]
        sdasdasd: String,
        code_occurence: CodeOccurence,
    },
    Variant {
        #[eo_to_err_string_serde]
        eo_display_with_serde_field: String,
        code_occurence: CodeOccurence,
    },
}
#[derive(Debug, Error, ErOccurence)]
pub enum ErUnnamedOne {
    Something(ErTwo),
}
#[derive(Debug)]
pub struct DisplayStruct {
    pub display: String,
    pub something: bool,
}
//todo or maybe two different traits - display foreign type and convert into serializable and deserializable type
impl ToErrString for DisplayStruct {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SerdeStruct {
    pub one: String,
    pub two: bool,
    pub three: u32,
}
impl ToErrString for SerdeStruct {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
fn main() {
    let er = ErOne::Variant {
        eo_display_field: DisplayStruct {
            display: String::from("value"),
            something: true,
        },
        eo_serde: SerdeStruct {
            one: String::from("value"),
            two: true,
            three: 42,
        },
        eo_er_occurence_field: ErTwo::Variant {
            eo_display_with_serde_field: String::from("value"),
            code_occurence: code_occurence!(),
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
            SerdeStruct {
                one: String::from("value"),
                two: true,
                three: 42,
            },
            SerdeStruct {
                one: String::from("97697697"),
                two: false,
                three: 422,
            },
        ],
        eo_vec_er_occurence_field: vec![
            ErUnnamedOne::Something(ErTwo::Variant {
                eo_display_with_serde_field: String::from("value"),
                code_occurence: code_occurence!(),
            }),
            ErUnnamedOne::Something(ErTwo::Variant {
                eo_display_with_serde_field: String::from("123"),
                code_occurence: code_occurence!(),
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
                SerdeStruct {
                    one: String::from("valusdfgdsgdsfgde"),
                    two: true,
                    three: 42,
                },
            ),
            (
                String::from("ksdfgdsfgsdfgey"),
                SerdeStruct {
                    one: String::from("valsdfgdsgdue"),
                    two: true,
                    three: 42,
                },
            ),
        ]),
        hashmap_string_er_occurence: HashMap::from([
            (
                String::from("ksdfgadsfgsdfgdfgey"),
                ErUnnamedOne::Something(ErTwo::Variant {
                    eo_display_with_serde_field: String::from("vasdfgdgdfglue"),
                    code_occurence: code_occurence!(),
                }),
            ),
            (
                String::from("kesdfgsdgfdfgy"),
                ErUnnamedOne::Something(ErTwo::Variant {
                    eo_display_with_serde_field: String::from("valsdfgdsafgdsgue"),
                    code_occurence: code_occurence!(),
                }),
            ),
        ]),
        code_occurence: code_occurence!(),
    };
    println!("{er:?}");
}
