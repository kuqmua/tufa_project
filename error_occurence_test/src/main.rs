#[derive(
    Debug,
    thiserror::Error,
    // error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedOne {
    Variant {
        // #[eo_display]
        eo_display_field: DisplayStruct,
        // #[eo_display_with_serialize_deserialize]
        // eo_display_with_serialize_deserialize_field: std::string::String,
        // #[eo_display_foreign_type]
        // eo_display_foreign_type_field: DisplayForeignTypeStruct,
        // #[eo_display_foreign_type_with_serialize_deserialize]
        // eo_display_foreign_type_with_serialize_deserialize_field: DisplayForeignTypeWithSerializeDeserializeStruct,
        // #[eo_error_occurence]
        // eo_error_occurence_field: ErrorNamedTwo,
        // #[eo_vec_display]
        // eo_vec_display_field: std::vec::Vec<DisplayStruct>,
        // #[eo_vec_display_with_serialize_deserialize]
        // eo_vec_display_with_serialize_deserialize_field: std::vec::Vec<std::string::String>,
        // #[eo_vec_display_foreign_type]
        // eo_vec_display_foreign_type_field: std::vec::Vec<DisplayForeignTypeStruct>,
        // #[eo_vec_display_foreign_type_with_serialize_deserialize]
        // eo_vec_display_foreign_type_with_serialize_deserialize_field: std::vec::Vec<DisplayForeignTypeWithSerializeDeserializeStruct>,
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
        // eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
        // eo_display_foreign_type_field: DisplayForeignTypeStruct {
        //     display_foreign_type: std::string::String::from("value"),
        // },
        // eo_display_foreign_type_with_serialize_deserialize_field: DisplayForeignTypeWithSerializeDeserializeStruct {
        //     display_foreign_type_with_serialize_deserialize: std::string::String::from("value"),
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
        // eo_vec_display_foreign_type_field: vec![
        //     DisplayForeignTypeStruct {
        //         display_foreign_type: std::string::String::from("value")
        //     }
        // ],
        // eo_vec_display_foreign_type_with_serialize_deserialize_field: vec![
        //     DisplayForeignTypeWithSerializeDeserializeStruct {
        //         display_foreign_type_with_serialize_deserialize: std::string::String::from("value"),
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


///////////////////////////////////
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_>
    for ErrorNamedOne
{
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                code_occurence: _unused_argument_1,
            } => {
                format!(
                    "{{{}}}",
                    error_occurence_lib::lines_space_backslash::LinesSpaceBackslash::lines_space_backslash(
                        &format!("eo_display_field: {}", eo_display_field)
                    )
                )
            }
        }
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_>
    for ErrorNamedOne
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                ("eo_display_field: {}", eo_display_field)))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedOne {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field: _unused_argument_0,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_>
    for ErrorNamedOneWithSerializeDeserialize
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{
{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                ("eo_display_field: {}", eo_display_field)))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedOneWithSerializeDeserialize {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field: _unused_argument_0,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl ErrorNamedOne {
    pub fn into_serialize_deserialize_version(self) -> ErrorNamedOneWithSerializeDeserialize {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                code_occurence,
            } => ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field: { eo_display_field.to_string() },
                code_occurence: code_occurence,
            },
        }
    }
}
impl std::fmt::Display for ErrorNamedOne {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        error_occurence_lib::to_string_without_config::ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl std::fmt::Display for ErrorNamedOneWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        error_occurence_lib::to_string_without_config::ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl error_occurence_lib::error_occurence_named::ErrorOccurenceNamed for ErrorNamedOne {
    fn error_occurence_named(&self) {}
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum ErrorNamedOneWithSerializeDeserialize {
    Variant {
        eo_display_field: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
