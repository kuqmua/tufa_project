#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedOne {
    Variant {
        // todo maybe create 6 wrapper types for IN SERIALIZE DESERIALIZE std::string::String, IN SERIALIZE DESERIALIZE nested, IN SERIALIZE DESERIALIZE std::vec::Vec<std::string::String>,//IN SERIALIZE DESERIALIZE std::vec::Vec<nested> and hashmaps
        // #[eo_string]
        eo_display_field: DisplayStruct,//IN SERIALIZE DESERIALIZE std::string::String
        // #[eo_error_occurence]
        eo_error_occurence_field: ErrorNamedTwo,//IN SERIALIZE DESERIALIZE nested
        // #[eo_vec_string]
        eo_vec_display_field: StdVecVecDisplayStruct,//IN SERIALIZE DESERIALIZE std::vec::Vec<std::string::String>
        // #[eo_vec_error_occurence]
        eo_vec_error_occurence_field: StdVecVecErrorUnnamedOne,//IN SERIALIZE DESERIALIZE std::vec::Vec<nested>
        // #[eo_hashmap_string_string]
        hashmap_string_string: StdCollectionsHashMapStdStringStringDisplayStruct,
        // #[eo_hashmap_string_error_occurence]
        hashmap_string_error_occurence: StdCollectionsHashMapStdStringStringErrorUnnamedOne,

        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

//////////

////////
#[derive(Debug)]
pub struct StdVecVecDisplayStruct(std::vec::Vec<DisplayStruct>);
impl std::fmt::Display for StdVecVecDisplayStruct {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::stringified_lines_error_vec(self.0.iter().fold(
                std::string::String::from(""),
                |mut acc, element| {
                    acc.push_str(&error_occurence_lib::helpers::lines_space_backslash_addition(&element));
                    acc
                },
            ))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdVecVecDisplayStruct {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdVecVecDisplayStruct {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdVecVecDisplayStruct {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
impl StdVecVecDisplayStruct {
    pub fn into_serialize_deserialize_version(self) -> StdVecVecDisplayStructWithSerializeDeserialize {
        StdVecVecDisplayStructWithSerializeDeserialize(self.0.into_iter().map(|element|element.into_serialize_deserialize_version()).collect())
    }
}
//
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StdVecVecDisplayStructWithSerializeDeserialize(std::vec::Vec<DisplayStructWithSerializeDeserialize>);
impl std::fmt::Display for StdVecVecDisplayStructWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::stringified_lines_error_vec(self.0.iter().fold(
                std::string::String::from(""),
                |mut acc, element| {
                    acc.push_str(&error_occurence_lib::helpers::lines_space_backslash_addition(&element));
                    acc
                },
            ))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdVecVecDisplayStructWithSerializeDeserialize {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdVecVecDisplayStructWithSerializeDeserialize {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdVecVecDisplayStructWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
////////
#[derive(Debug)]
pub struct StdVecVecErrorUnnamedOne(std::vec::Vec<ErrorUnnamedOne>);
impl std::fmt::Display for StdVecVecErrorUnnamedOne {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::stringified_lines_error_vec(self.0.iter().fold(
                std::string::String::from(""),
                |mut acc, element| {
                    acc.push_str(&error_occurence_lib::helpers::lines_space_backslash_addition(&element));
                    acc
                },
            ))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdVecVecErrorUnnamedOne {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdVecVecErrorUnnamedOne {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdVecVecErrorUnnamedOne {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
impl StdVecVecErrorUnnamedOne {
    pub fn into_serialize_deserialize_version(self) -> StdVecVecErrorUnnamedOneWithSerializeDeserialize {
        StdVecVecErrorUnnamedOneWithSerializeDeserialize(self.0.into_iter().map(|element|element.into_serialize_deserialize_version()).collect())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StdVecVecErrorUnnamedOneWithSerializeDeserialize(std::vec::Vec<ErrorUnnamedOneWithSerializeDeserialize>);
impl std::fmt::Display for StdVecVecErrorUnnamedOneWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::stringified_lines_error_vec(self.0.iter().fold(
                std::string::String::from(""),
                |mut acc, element| {
                    acc.push_str(&error_occurence_lib::helpers::lines_space_backslash_addition(&element));
                    acc
                },
            ))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdVecVecErrorUnnamedOneWithSerializeDeserialize {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdVecVecErrorUnnamedOneWithSerializeDeserialize {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdVecVecErrorUnnamedOneWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
////////
#[derive(Debug)]
pub struct StdCollectionsHashMapStdStringStringDisplayStruct(std::collections::HashMap<error_occurence_lib::primitive_types_wrappers::StdStringString, DisplayStruct>);
impl std::fmt::Display for StdCollectionsHashMapStdStringStringDisplayStruct {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::error_occurence_hashmap_formatter(self.0.iter().fold(
                std::string::String::new(),
                |mut acc, (key, value)| {
                    acc.push_str(&error_occurence_lib::helpers::stringified_lines_error_hashmap_element(
                        &key,
                        &value,
                    ));
                    acc
                },
            ))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdCollectionsHashMapStdStringStringDisplayStruct {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdCollectionsHashMapStdStringStringDisplayStruct {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdCollectionsHashMapStdStringStringDisplayStruct {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
impl StdCollectionsHashMapStdStringStringDisplayStruct {
    pub fn into_serialize_deserialize_version(self) -> StdCollectionsHashMapStdStringStringDisplayStructWithSerializeDeserialize {
        StdCollectionsHashMapStdStringStringDisplayStructWithSerializeDeserialize(self.0.into_iter().map(
            |(key, value)|(key.into_serialize_deserialize_version(), value.into_serialize_deserialize_version())
        ).collect())
    }
}
//
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StdCollectionsHashMapStdStringStringDisplayStructWithSerializeDeserialize(
    std::collections::HashMap<error_occurence_lib::primitive_types_wrappers::StdStringStringWithSerializeDeserialize, DisplayStructWithSerializeDeserialize>
);
impl std::fmt::Display for StdCollectionsHashMapStdStringStringDisplayStructWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::error_occurence_hashmap_formatter(self.0.iter().fold(
                std::string::String::new(),
                |mut acc, (key, value)| {
                    acc.push_str(&error_occurence_lib::helpers::stringified_lines_error_hashmap_element(
                        &key,
                        &value,
                    ));
                    acc
                },
            ))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdCollectionsHashMapStdStringStringDisplayStructWithSerializeDeserialize {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdCollectionsHashMapStdStringStringDisplayStructWithSerializeDeserialize {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdCollectionsHashMapStdStringStringDisplayStructWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
////////
#[derive(Debug)]
pub struct StdCollectionsHashMapStdStringStringErrorUnnamedOne(std::collections::HashMap<error_occurence_lib::primitive_types_wrappers::StdStringString, ErrorUnnamedOne>);
impl std::fmt::Display for StdCollectionsHashMapStdStringStringErrorUnnamedOne {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::error_occurence_hashmap_formatter(self.0.iter().fold(
                std::string::String::new(),
                |mut acc, (key, value)| {
                    acc.push_str(&error_occurence_lib::helpers::stringified_lines_error_hashmap_element(
                        &key,
                        &value,
                    ));
                    acc
                },
            ))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdCollectionsHashMapStdStringStringErrorUnnamedOne {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdCollectionsHashMapStdStringStringErrorUnnamedOne {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdCollectionsHashMapStdStringStringErrorUnnamedOne {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
impl StdCollectionsHashMapStdStringStringErrorUnnamedOne {
    pub fn into_serialize_deserialize_version(self) -> StdCollectionsHashMapStdStringStringErrorUnnamedOneWithSerializeDeserialize {
        StdCollectionsHashMapStdStringStringErrorUnnamedOneWithSerializeDeserialize(self.0.into_iter().map(
            |(key, value)|(key.into_serialize_deserialize_version(), value.into_serialize_deserialize_version())
        ).collect())
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StdCollectionsHashMapStdStringStringErrorUnnamedOneWithSerializeDeserialize(
    std::collections::HashMap<error_occurence_lib::primitive_types_wrappers::StdStringStringWithSerializeDeserialize, ErrorUnnamedOneWithSerializeDeserialize>
);
impl std::fmt::Display for StdCollectionsHashMapStdStringStringErrorUnnamedOneWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::error_occurence_hashmap_formatter(self.0.iter().fold(
                std::string::String::new(),
                |mut acc, (key, value)| {
                    acc.push_str(&error_occurence_lib::helpers::stringified_lines_error_hashmap_element(
                        &key,
                        &value,
                    ));
                    acc
                },
            ))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdCollectionsHashMapStdStringStringErrorUnnamedOneWithSerializeDeserialize {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdCollectionsHashMapStdStringStringErrorUnnamedOneWithSerializeDeserialize {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdCollectionsHashMapStdStringStringErrorUnnamedOneWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
/////


#[derive(Debug)]
pub struct DisplayStruct {
    pub display: std::string::String,
    pub something: std::primitive::bool,
}
impl std::fmt::Display for DisplayStruct {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::lines_backslash_addition(&format!("{self:#?}"))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for DisplayStruct {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for DisplayStruct {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for DisplayStruct {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
impl DisplayStruct {
    pub fn into_serialize_deserialize_version(self) -> DisplayStructWithSerializeDeserialize {
        DisplayStructWithSerializeDeserialize {
            display: error_occurence_lib::primitive_types_wrappers::StdStringStringWithSerializeDeserialize (self.display),
            something: error_occurence_lib::primitive_types_wrappers::StdPrimitiveBoolWithSerializeDeserialize (self.something),
        }
    }
}
//
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisplayStructWithSerializeDeserialize {
    pub display: error_occurence_lib::primitive_types_wrappers::StdStringStringWithSerializeDeserialize,
    pub something: error_occurence_lib::primitive_types_wrappers::StdPrimitiveBoolWithSerializeDeserialize,
}
impl std::fmt::Display for DisplayStructWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::lines_backslash_addition(&format!("{self:#?}"))
        )
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for DisplayStructWithSerializeDeserialize {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for DisplayStructWithSerializeDeserialize {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl error_occurence_lib::code_occurence::GetOption for DisplayStructWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
//

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
    error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedTwo {
    Variant {
        // #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: error_occurence_lib::primitive_types_wrappers::StdStringString,
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
    SelfGeneric: error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'a>
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
    SelfGeneric: error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'a>
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
    SelfGeneric: error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'a>
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
            display: std::string::String::from("value"),
            something: true,
        },
        eo_error_occurence_field: ErrorNamedTwo::Variant {
            eo_display_with_serialize_deserialize_field: error_occurence_lib::primitive_types_wrappers::StdStringString(std::string::String::from("value")),
            code_occurence: error_occurence_lib::code_occurence!(),
        },
        eo_vec_display_field: StdVecVecDisplayStruct(vec![
            DisplayStruct {
                display: std::string::String::from("value"),
                something: true,
            }
        ]),
        eo_vec_error_occurence_field: StdVecVecErrorUnnamedOne(vec![
            ErrorUnnamedOne::Something(
                ErrorNamedTwo::Variant {
                    eo_display_with_serialize_deserialize_field: error_occurence_lib::primitive_types_wrappers::StdStringString(std::string::String::from("value")),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }
            )
        ]),
        hashmap_string_string: StdCollectionsHashMapStdStringStringDisplayStruct(
            std::collections::HashMap::from([
                (
                    error_occurence_lib::primitive_types_wrappers::StdStringString(std::string::String::from("keysiudfgsidlufgsiadfglisadglifasgdlfiygsaldfglsagdlfhgsaldhfglhsdflhsadflhsaldhfgs\nkeysdfsadfasdfsdfasdfasdf\nkeysadfasfdsdfsfdsfsd")),
                    DisplayStruct {
                        display: std::string::String::from("value"),
                        something: true,
                    }
                ),
            ])
        ),
        hashmap_string_error_occurence: StdCollectionsHashMapStdStringStringErrorUnnamedOne(
            std::collections::HashMap::from([
                (
                    error_occurence_lib::primitive_types_wrappers::StdStringString(std::string::String::from("key")),
                    ErrorUnnamedOne::Something(
                        ErrorNamedTwo::Variant {
                            eo_display_with_serialize_deserialize_field: error_occurence_lib::primitive_types_wrappers::StdStringString(std::string::String::from("value")),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        }
                    )
                ),
            ])
        ),
        

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
impl error_occurence_lib::code_occurence::GetOption for ErrorNamedOneWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        match self {
            ErrorNamedOneWithSerializeDeserialize::Variant {
                code_occurence,
                ..
            } => Some(code_occurence),
        }
    }
}

////////////
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

#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum ErrorUnnamedOneWithSerializeDeserialize {
    Something(ErrorNamedTwoWithSerializeDeserialize),
}