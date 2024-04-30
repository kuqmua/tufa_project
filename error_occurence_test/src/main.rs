#[derive(
    Debug,
    thiserror::Error,
    // error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedOne {
    Variant {

        // TODO REMOVE ALL ATTRIBUTES EXCEPT eo_error_occurence RELATED AND MAYBE USE SourceToStringWithConfig AND SourceToStringWithoutConfig


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
impl error_occurence_lib::code_occurence::GetOption for DisplayStruct {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
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
    // error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedTwo {
    Variant {
        // #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}

#[derive(Debug, thiserror::Error, 
    // error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorUnnamedOne {
    // #[eo_error_occurence]
    Something(ErrorNamedTwo),
}

////////////
pub trait ToStringWithConfig<'a> {
    fn to_string_with_config<
        ConfigGeneric: app_state::GetSourcePlaceType
            + app_state::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}

impl<'a, SelfGeneric> ToStringWithConfig<'a> for SelfGeneric
where
    SelfGeneric: error_occurence_lib::SourceToStringWithConfig<'a>
        + error_occurence_lib::code_occurence::GetOption,
{
    fn to_string_with_config<
        ConfigGeneric: app_state::GetSourcePlaceType
            + app_state::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self.get_option() {
            Some(value) => error_occurence_lib::helpers::source_and_code_occurence_formatter(
                self.source_to_string_with_config(config),
                error_occurence_lib::code_occurence::PrepareForLogWithConfig::prepare_for_log_with_config(
                    value,
                    config
                )
            ),
            None => self.source_to_string_with_config(config)
        }
    }
}
///
pub trait ToStringWithoutConfig<'a> {
    fn to_string_without_config(&self) -> std::string::String;
}

impl<'a, SelfGeneric> ToStringWithoutConfig<'a> for SelfGeneric
where
    SelfGeneric: error_occurence_lib::SourceToStringWithoutConfig<'a>
        + error_occurence_lib::code_occurence::GetOption,
{
    fn to_string_without_config(&self) -> std::string::String {
        match self.get_option() {
            Some(value) => error_occurence_lib::helpers::source_and_code_occurence_formatter(
                self.source_to_string_without_config(),
                value,
            ),
            None => self.source_to_string_without_config(),
        }
    }
}
// //implemented coz you cant deserialize field into &'a GitInfo(not implememnted in serde)
pub trait ToStringWithoutConfigWithSerializeDeserialize<'a> {
    fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String;
}

impl<'a, SelfGeneric> ToStringWithoutConfigWithSerializeDeserialize<'a> for SelfGeneric
where
    SelfGeneric: error_occurence_lib::SourceToStringWithoutConfig<'a>
        + error_occurence_lib::code_occurence::GetOption,
{
    fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String {
        match self.get_option() {
            Some(value) => error_occurence_lib::helpers::source_and_code_occurence_formatter(
                self.source_to_string_without_config(),
                value,
            ),
            None => self.source_to_string_without_config(),
        }
    }
}
////////////

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
                        ToStringWithConfig:: to_string_with_config(eo_display_field, config)
                    )
                ),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash::lines_space_backslash(
                    & format!(
                        "eo_error_occurence_field: {}",
                        ToStringWithConfig:: to_string_with_config(eo_error_occurence_field, config)
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
                ("{{{}{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(
                    & format!(
                        "eo_display_field: {}", 
                        ToStringWithoutConfig:: to_string_without_config(eo_display_field)
                    )
                ),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(
                    & format!(
                        "eo_error_occurence_field: {}",
                        ToStringWithoutConfig:: to_string_without_config(eo_error_occurence_field)
                    )
                ))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::GetOption for ErrorNamedOne {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field: _unused_argument_0,
                eo_error_occurence_field: _unused_argument_1,
                code_occurence,
            } => Some(code_occurence),
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
                (
                    "eo_display_field: {}", 
                    ToStringWithoutConfigWithSerializeDeserialize::to_string_without_config_with_serialize_deserialize(eo_display_field)
                )),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                (
                    "eo_error_occurence_field: {}",
                    ToStringWithoutConfigWithSerializeDeserialize::to_string_without_config_with_serialize_deserialize(eo_error_occurence_field)
                )))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::GetOption for ErrorNamedOneWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        match self {
            ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field: _unused_argument_0,
                eo_error_occurence_field: _unused_argument_1,
                code_occurence,
            } => Some(code_occurence),
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
        ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl std::fmt::Display for ErrorNamedOneWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        ToStringWithoutConfig
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
impl error_occurence_lib::code_occurence::GetOption for ErrorNamedTwo {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        match self {
            ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field: _unused_argument_0,
                code_occurence,
            } => Some(code_occurence),
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
impl error_occurence_lib::code_occurence::GetOption for ErrorNamedTwoWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        match self {
            ErrorNamedTwoWithSerializeDeserialize::Variant {
                eo_display_with_serialize_deserialize_field: _unused_argument_0,
                code_occurence,
            } => Some(code_occurence),
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
        ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl std::fmt::Display for ErrorNamedTwoWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        ToStringWithoutConfig
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
impl ToStringWithConfig<'_> for ErrorUnnamedOne {
    fn to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self {
            ErrorUnnamedOne::Something(i) => ToStringWithConfig::to_string_with_config(i, config),
        }
    }
}
impl ToStringWithoutConfig<'_> for ErrorUnnamedOne {
    fn to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorUnnamedOne::Something(i) => ToStringWithoutConfig::to_string_without_config(i),
        }
    }
}
impl
    ToStringWithoutConfigWithSerializeDeserialize<'_>
    for ErrorUnnamedOneWithSerializeDeserialize
{
    fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String {
        match self {
            ErrorUnnamedOneWithSerializeDeserialize::Something(i) => {
                ToStringWithoutConfigWithSerializeDeserialize::to_string_without_config_with_serialize_deserialize(i)
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
        ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl std::fmt::Display for ErrorUnnamedOneWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        ToStringWithoutConfigWithSerializeDeserialize
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
