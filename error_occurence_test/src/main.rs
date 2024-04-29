#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedOne {
    Variant {
        #[eo_display]
        eo_display_field: DisplayStruct,
        // #[eo_display_with_serialize_deserialize]
        // eo_display_with_serialize_deserialize_field: std::string::String,
        // #[eo_to_std_string_string]
        // eo_to_std_string_string_field: ToStdStringStringStruct,
        // #[eo_to_std_string_string_with_serialize_deserialize]
        // eo_to_std_string_string_with_serialize_deserialize_field: ToStdStringStringWithSerializeDeserializeStruct,
        // #[eo_error_occurence]
        // eo_error_occurence_field: ErrorNamedTwo,
        // #[eo_vec_display]
        // eo_vec_display_field: std::vec::Vec<DisplayStruct>,
        // #[eo_vec_display_with_serialize_deserialize]
        // eo_vec_display_with_serialize_deserialize_field: std::vec::Vec<std::string::String>,
        // #[eo_vec_to_std_string_string]
        // eo_vec_to_std_string_string_field: std::vec::Vec<ToStdStringStringStruct>,
        // #[eo_vec_to_std_string_string_with_serialize_deserialize]
        // eo_vec_to_std_string_string_with_serialize_deserialize_field: std::vec::Vec<ToStdStringStringWithSerializeDeserializeStruct>,
        // #[eo_vec_error_occurence]
        // eo_vec_error_occurence_field: std::vec::Vec<ErrorUnnamedOne>,

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
pub struct ToStdStringStringStruct {
    pub to_std_string_string: std::string::String,
}
impl error_occurence_lib::ToStdStringString for ToStdStringStringStruct {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ToStdStringStringWithSerializeDeserializeStruct {
    pub to_std_string_string_with_serialize_deserialize: std::string::String,
}
impl error_occurence_lib::ToStdStringString for ToStdStringStringWithSerializeDeserializeStruct {
    fn to_std_string_string(&self) -> std::string::String {
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
        // eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
        // eo_to_std_string_string_field: ToStdStringStringStruct {
        //     to_std_string_string: std::string::String::from("value"),
        // },
        // eo_to_std_string_string_with_serialize_deserialize_field: ToStdStringStringWithSerializeDeserializeStruct {
        //     to_std_string_string_with_serialize_deserialize: std::string::String::from("value"),
        // },
        // eo_error_occurence_field: ErrorNamedTwo::Variant {
        //     eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
        //     code_occurence: error_occurence_lib::code_occurence!(),
        // },
        // eo_vec_display_field: vec![
        //     DisplayStruct {
        //         display: std::string::String::from("value")
        //     }
        // ],
        // eo_vec_display_with_serialize_deserialize_field: vec![
        //     std::string::String::from("value")
        // ],
        // eo_vec_to_std_string_string_field: vec![
        //     ToStdStringStringStruct {
        //         to_std_string_string: std::string::String::from("value")
        //     }
        // ],
        // eo_vec_to_std_string_string_with_serialize_deserialize_field: vec![
        //     ToStdStringStringWithSerializeDeserializeStruct {
        //         to_std_string_string_with_serialize_deserialize: std::string::String::from("value"),
        //     }
        // ],
        // eo_vec_error_occurence_field: vec![
        //     ErrorUnnamedOne::Something(
        //         ErrorNamedTwo::Variant {
        //             eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
        //             code_occurence: error_occurence_lib::code_occurence!(),
        //         }
        //     )
        // ],

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



//////////////////////////
