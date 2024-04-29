#[derive(
    Debug,
    thiserror::Error,
    // error_occurence_lib::ErrorOccurence
)]
pub enum ErrorNamedOne {
    Variant {

        TODO REMOVE ALL ATTRIBUTES EXCEPT eo_error_occurence RELATED 

        // #[eo_display]
        eo_display_field: DisplayStruct,
        // #[eo_display_with_serialize_deserialize]
        // eo_display_with_serialize_deserialize_field: std::string::String,
        // #[eo_to_std_string_string]
        // eo_to_std_string_string_field: ToStdStringStringStruct,
        // #[eo_to_std_string_string_with_serialize_deserialize]
        // eo_to_std_string_string_with_serialize_deserialize_field: ToStdStringStringWithSerializeDeserializeStruct,
        // #[eo_error_occurence]
        eo_error_occurence_field: ErrorNamedTwo,
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
impl error_occurence_lib::SourceToStringWithConfig<'_> for DisplayStruct {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::SourceToStringWithoutConfig<'_> for DisplayStruct {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl DisplayStruct {
    pub fn into_serialize_deserialize_version(self) -> std::string::String {
        self.to_string()
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
    // error_occurence_lib::ErrorOccurence
)]
pub enum ErrorNamedTwo {
    Variant {
        // #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}

#[derive(Debug, thiserror::Error, 
    // error_occurence_lib::ErrorOccurence
)]
pub enum ErrorUnnamedOne {
    // #[eo_error_occurence]
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
        eo_error_occurence_field: ErrorNamedTwo::Variant {
            eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
            code_occurence: error_occurence_lib::code_occurence!(),
        },
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
impl error_occurence_lib::SourceToStringWithConfig<'_> for ErrorNamedOne {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                eo_error_occurence_field,
                code_occurence: _unused_argument_2,
            } => {
                format!
                ("{{{}{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash::lines_space_backslash(
                    & format!("eo_display_field: {}", 
                        eo_display_field
                        // error_occurence_lib::to_string_with_config::ToStringWithConfig:: to_string_with_config(eo_display_field, config)
                    )
                ),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash::lines_space_backslash(
                    & format!(
                        "eo_error_occurence_field: {}",
                        error_occurence_lib::to_string_with_config::ToStringWithConfig:: to_string_with_config(eo_error_occurence_field, config)
                    )
                ))
            }
        }
    }
}
impl error_occurence_lib::SourceToStringWithoutConfig<'_> for ErrorNamedOne {
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                eo_error_occurence_field,
                code_occurence: _unused_argument_2,
            } => {
                format!
                ("{{
{}{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                ("eo_display_field: {}", eo_display_field)),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                ("eo_error_occurence_field: {}",
                error_occurence_lib::to_string_without_config::ToStringWithoutConfig
                :: to_string_without_config(eo_error_occurence_field))))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedOne {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field: _unused_argument_0,
                eo_error_occurence_field: _unused_argument_1,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl error_occurence_lib::SourceToStringWithoutConfig<'_>
    for ErrorNamedOneWithSerializeDeserialize
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field,
                eo_error_occurence_field,
                code_occurence: _unused_argument_2,
            } => {
                format!
                ("{{
{}{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                ("eo_display_field: {}", eo_display_field)),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                ("eo_error_occurence_field: {}",
                error_occurence_lib::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize
                ::
                to_string_without_config_with_serialize_deserialize(eo_error_occurence_field))))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedOneWithSerializeDeserialize {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field: _unused_argument_0,
                eo_error_occurence_field: _unused_argument_1,
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
                eo_error_occurence_field,
                code_occurence,
            } => ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field: { eo_display_field.to_string() },
                eo_error_occurence_field: {
                    eo_error_occurence_field.into_serialize_deserialize_version()
                },
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
impl ErrorNamedOne {
    fn _compile_time_check_error_occurence_members(&self) {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field: _unused_argument_0,
                eo_error_occurence_field,
                code_occurence: _unused_argument_2,
            } => {
                error_occurence_lib::error_occurence_named::ErrorOccurenceNamed
                :: error_occurence_named(eo_error_occurence_field);
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum ErrorNamedOneWithSerializeDeserialize {
    Variant {
        eo_display_field: std::string::String,
        eo_error_occurence_field: ErrorNamedTwoWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

////////////

impl error_occurence_lib::SourceToStringWithConfig<'_> for ErrorNamedTwo {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self {
            ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{
{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                ("eo_display_with_serialize_deserialize_field: {}",
                eo_display_with_serialize_deserialize_field)))
            }
        }
    }
}
impl error_occurence_lib::SourceToStringWithoutConfig<'_> for ErrorNamedTwo {
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{
{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                ("eo_display_with_serialize_deserialize_field: {}",
                eo_display_with_serialize_deserialize_field)))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedTwo {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field: _unused_argument_0,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl error_occurence_lib::SourceToStringWithoutConfig<'_>
    for ErrorNamedTwoWithSerializeDeserialize
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedTwoWithSerializeDeserialize::Variant {
                eo_display_with_serialize_deserialize_field,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{
{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                ("eo_display_with_serialize_deserialize_field: {}",
                eo_display_with_serialize_deserialize_field)))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedTwoWithSerializeDeserialize {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedTwoWithSerializeDeserialize::Variant {
                eo_display_with_serialize_deserialize_field: _unused_argument_0,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl ErrorNamedTwo {
    pub fn into_serialize_deserialize_version(self) -> ErrorNamedTwoWithSerializeDeserialize {
        match self {
            ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field,
                code_occurence,
            } => ErrorNamedTwoWithSerializeDeserialize::Variant {
                eo_display_with_serialize_deserialize_field: {
                    eo_display_with_serialize_deserialize_field
                },
                code_occurence: code_occurence,
            },
        }
    }
}
impl std::fmt::Display for ErrorNamedTwo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        error_occurence_lib::to_string_without_config::ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl std::fmt::Display for ErrorNamedTwoWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        error_occurence_lib::to_string_without_config::ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl error_occurence_lib::error_occurence_named::ErrorOccurenceNamed for ErrorNamedTwo {
    fn error_occurence_named(&self) {}
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum ErrorNamedTwoWithSerializeDeserialize {
    Variant {
        eo_display_with_serialize_deserialize_field: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
////////////////////
impl error_occurence_lib::to_string_with_config::ToStringWithConfig<'_> for ErrorUnnamedOne {
    fn to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self {
            ErrorUnnamedOne::Something(i) => i.to_string_with_config(config),
        }
    }
}
impl error_occurence_lib::to_string_without_config::ToStringWithoutConfig<'_> for ErrorUnnamedOne {
    fn to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorUnnamedOne::Something(i) => i.to_string_without_config(),
        }
    }
}
impl
    error_occurence_lib::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize<'_>
    for ErrorUnnamedOneWithSerializeDeserialize
{
    fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String {
        match self {
            ErrorUnnamedOneWithSerializeDeserialize::Something(i) => {
                i.to_string_without_config_with_serialize_deserialize()
            }
        }
    }
}
impl ErrorUnnamedOne {
    pub fn into_serialize_deserialize_version(self) -> ErrorUnnamedOneWithSerializeDeserialize {
        match self {
            ErrorUnnamedOne::Something(i) => ErrorUnnamedOneWithSerializeDeserialize::Something(
                i.into_serialize_deserialize_version(),
            ),
        }
    }
}
impl std::fmt::Display for ErrorUnnamedOne {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        error_occurence_lib::to_string_without_config::ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl std::fmt::Display for ErrorUnnamedOneWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        error_occurence_lib::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize
        :: to_string_without_config_with_serialize_deserialize(self))
    }
}
impl error_occurence_lib::error_occurence_unnamed::ErrorOccurenceUnnamed for ErrorUnnamedOne {
    fn error_occurence_unnamed(&self) {}
}
impl ErrorUnnamedOne {
    fn _compile_time_check_error_occurence_members(&self) {
        match self {
            ErrorUnnamedOne::Something(i) => {
                error_occurence_lib::error_occurence_named::ErrorOccurenceNamed
                :: error_occurence_named(i);
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum ErrorUnnamedOneWithSerializeDeserialize {
    Something(ErrorNamedTwoWithSerializeDeserialize),
}
