#[derive(
    Debug,
    thiserror::Error,
    // error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedOne {
    Variant {
        // TODO REMOVE ALL ATTRIBUTES EXCEPT eo_error_occurence RELATED AND MAYBE USE SourceToStringWithConfig AND SourceToStringWithoutConfig
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
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct StdStringString(std::string::String);
impl std::fmt::Display for StdStringString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdStringString {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.0.clone()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdStringString {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.0.clone()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdStringString {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
impl StdStringString {
    pub fn into_serialize_deserialize_version(self) -> StdStringStringWithSerializeDeserialize {
        StdStringStringWithSerializeDeserialize(self.0)
    }
}
//
#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct StdStringStringWithSerializeDeserialize(std::string::String);
impl std::fmt::Display for StdStringStringWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for StdStringStringWithSerializeDeserialize {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.0.clone()
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdStringStringWithSerializeDeserialize {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.0.clone()
    }
}
impl error_occurence_lib::code_occurence::GetOption for StdStringStringWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        None
    }
}
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
                |mut acc, vec_element| {
                    acc.push_str(&format!(" {vec_element}\n"));
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
                |mut acc, vec_element| {
                    acc.push_str(&format!(" {vec_element}\n"));
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
                |mut acc, vec_element| {
                    acc.push_str(&format!(" {vec_element}\n"));
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
                |mut acc, vec_element| {
                    acc.push_str(
                        &vec_element
                        .to_string()
                        .lines()
                        .fold(std::string::String::new(), |mut acc, line| {
                            acc.push_str(&format!(" {line}\n"));
                            acc
                        })
                    );
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
pub struct StdCollectionsHashMapStdStringStringDisplayStruct(std::collections::HashMap<StdStringString, DisplayStruct>);
impl std::fmt::Display for StdCollectionsHashMapStdStringStringDisplayStruct {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::error_occurence_hashmap_formatter(self.0.iter().fold(
                std::string::String::new(),
                |mut acc, (key, value)| {
                    acc.push_str(&error_occurence_lib::helpers::stringified_lines_error_hashmap_element(
                        key,
                        value.to_string(),
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
    std::collections::HashMap<StdStringStringWithSerializeDeserialize, DisplayStructWithSerializeDeserialize>
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
                        key,
                        value.to_string(),
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
pub struct StdCollectionsHashMapStdStringStringErrorUnnamedOne(std::collections::HashMap<StdStringString, ErrorUnnamedOne>);
impl std::fmt::Display for StdCollectionsHashMapStdStringStringErrorUnnamedOne {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            error_occurence_lib::helpers::error_occurence_hashmap_formatter(self.0.iter().fold(
                std::string::String::new(),
                |mut acc, (key, value)| {
                    acc.push_str(&error_occurence_lib::helpers::stringified_lines_error_hashmap_element(
                        key,
                        value.to_string(),
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
    std::collections::HashMap<StdStringStringWithSerializeDeserialize, ErrorUnnamedOneWithSerializeDeserialize>
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
                        key,
                        value.to_string(),
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
}
impl std::fmt::Display for DisplayStruct {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.display)
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
            display: StdStringStringWithSerializeDeserialize (self.display.to_string())
        }
    }
}
//
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisplayStructWithSerializeDeserialize {
    pub display: StdStringStringWithSerializeDeserialize,
}
impl std::fmt::Display for DisplayStructWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.display)
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
    // error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedTwo {
    Variant {
        // #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: StdStringString,
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
            display: std::string::String::from("value")
        },
        eo_error_occurence_field: ErrorNamedTwo::Variant {
            eo_display_with_serialize_deserialize_field: StdStringString(std::string::String::from("value")),
            code_occurence: error_occurence_lib::code_occurence!(),
        },
        eo_vec_display_field: StdVecVecDisplayStruct(vec![
            DisplayStruct {
                display: std::string::String::from("value")
            }
        ]),
        eo_vec_error_occurence_field: StdVecVecErrorUnnamedOne(vec![
            ErrorUnnamedOne::Something(
                ErrorNamedTwo::Variant {
                    eo_display_with_serialize_deserialize_field: StdStringString(std::string::String::from("value")),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }
            )
        ]),
        hashmap_string_string: StdCollectionsHashMapStdStringStringDisplayStruct(
            std::collections::HashMap::from([
                (
                    StdStringString(std::string::String::from("key")),
                    DisplayStruct {
                        display: std::string::String::from("value")
                    }
                ),
            ])
        ),
        hashmap_string_error_occurence: StdCollectionsHashMapStdStringStringErrorUnnamedOne(
            std::collections::HashMap::from([
                (
                    StdStringString(std::string::String::from("key")),
                    ErrorUnnamedOne::Something(
                        ErrorNamedTwo::Variant {
                            eo_display_with_serialize_deserialize_field: StdStringString(std::string::String::from("value")),
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
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for ErrorNamedOne {
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
                eo_vec_display_field,
                eo_vec_error_occurence_field,
                hashmap_string_string,
                hashmap_string_error_occurence,
                code_occurence: _unused_argument_2,
            } => {
                format!(
                    "{{\n{}{}{}{}{}{}}}",
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
                    ),
                    error_occurence_lib::lines_space_backslash::LinesSpaceBackslash::lines_space_backslash(
                        & format!(
                            "eo_vec_display_field: {}",
                            ToStringWithConfig:: to_string_with_config(eo_vec_display_field, config)
                        )
                    ),
                    error_occurence_lib::lines_space_backslash::LinesSpaceBackslash::lines_space_backslash(
                        & format!(
                            "eo_vec_error_occurence_field: {}",
                            ToStringWithConfig:: to_string_with_config(eo_vec_error_occurence_field, config)
                        )
                    ),
                    error_occurence_lib::lines_space_backslash::LinesSpaceBackslash::lines_space_backslash(
                        & format!(
                            "hashmap_string_string: {}",
                            ToStringWithConfig:: to_string_with_config(hashmap_string_string, config)
                        )
                    ),
                    error_occurence_lib::lines_space_backslash::LinesSpaceBackslash::lines_space_backslash(
                        & format!(
                            "hashmap_string_error_occurence: {}",
                            ToStringWithConfig:: to_string_with_config(hashmap_string_error_occurence, config)
                        )
                    ),
                )
            }
        }
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for ErrorNamedOne {
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                eo_error_occurence_field,
                eo_vec_display_field,
                eo_vec_error_occurence_field,
                hashmap_string_string,
                hashmap_string_error_occurence,
                code_occurence: _unused_argument_2,
            } => {
                format!
                ("{{\n{}{}{}{}{}{}}}",
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
                ),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(
                    & format!(
                        "eo_vec_display_field: {}",
                        ToStringWithoutConfig:: to_string_without_config(eo_vec_display_field)
                    )
                ),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(
                    & format!(
                        "eo_vec_error_occurence_field: {}",
                        ToStringWithoutConfig:: to_string_without_config(eo_vec_error_occurence_field)
                    )
                ),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(
                    & format!(
                        "hashmap_string_string: {}",
                        ToStringWithoutConfig:: to_string_without_config(hashmap_string_string)
                    )
                ),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(
                    & format!(
                        "hashmap_string_error_occurence: {}",
                        ToStringWithoutConfig:: to_string_without_config(hashmap_string_error_occurence)
                    )
                ),
            )

            }
        }
    }
}
impl error_occurence_lib::code_occurence::GetOption for ErrorNamedOne {
    fn get_option(&self) -> std::option::Option<&error_occurence_lib::code_occurence::CodeOccurence> {
        match self {
            ErrorNamedOne::Variant {
                code_occurence,
                ..
            } => Some(code_occurence),
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
                eo_error_occurence_field,
                eo_vec_display_field,
                eo_vec_error_occurence_field,
                hashmap_string_string,
                hashmap_string_error_occurence,
                code_occurence: _unused_argument_2,
            } => {
                format!
                ("{{\n{}{}{}{}{}{}}}",
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
                )),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                (
                    "eo_vec_display_field: {}",
                    ToStringWithoutConfigWithSerializeDeserialize::to_string_without_config_with_serialize_deserialize(eo_vec_display_field)
                )),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                (
                    "eo_vec_error_occurence_field: {}",
                    ToStringWithoutConfigWithSerializeDeserialize::to_string_without_config_with_serialize_deserialize(eo_vec_error_occurence_field)
                )),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                (
                    "hashmap_string_string: {}",
                    ToStringWithoutConfigWithSerializeDeserialize::to_string_without_config_with_serialize_deserialize(hashmap_string_string)
                )),
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(& format!
                (
                    "hashmap_string_error_occurence: {}",
                    ToStringWithoutConfigWithSerializeDeserialize::to_string_without_config_with_serialize_deserialize(hashmap_string_error_occurence)
                )),
            )
            }
        }
    }
}
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
impl ErrorNamedOne {
    pub fn into_serialize_deserialize_version(self) -> ErrorNamedOneWithSerializeDeserialize {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                eo_error_occurence_field,
                eo_vec_display_field,
                eo_vec_error_occurence_field,
                hashmap_string_string,
                hashmap_string_error_occurence,
                code_occurence,
            } => ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field: { 
                    eo_display_field.into_serialize_deserialize_version()
                },
                eo_error_occurence_field: {
                    eo_error_occurence_field.into_serialize_deserialize_version()
                },
                eo_vec_display_field: {
                    eo_vec_display_field.into_serialize_deserialize_version()
                },
                eo_vec_error_occurence_field: {
                    eo_vec_error_occurence_field.into_serialize_deserialize_version()
                },
                hashmap_string_string: {
                    hashmap_string_string.into_serialize_deserialize_version()
                },
                hashmap_string_error_occurence: {
                    hashmap_string_error_occurence.into_serialize_deserialize_version()
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
                // eo_display_field: _unused_argument_0,
                eo_error_occurence_field,
                // eo_vec_display_field: _unused_argument_1,
                // code_occurence: _unused_argument_2,
                ..
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
        eo_display_field: DisplayStructWithSerializeDeserialize,
        eo_error_occurence_field: ErrorNamedTwoWithSerializeDeserialize,
        eo_vec_display_field: StdVecVecDisplayStructWithSerializeDeserialize,
        eo_vec_error_occurence_field: StdVecVecErrorUnnamedOneWithSerializeDeserialize,
        hashmap_string_string: StdCollectionsHashMapStdStringStringDisplayStructWithSerializeDeserialize,
        hashmap_string_error_occurence: StdCollectionsHashMapStdStringStringErrorUnnamedOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

////////////

impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for ErrorNamedTwo {
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
                ("{{\n{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(
                    & format!(
                        "eo_display_with_serialize_deserialize_field: {}",
                        ToStringWithConfig:: to_string_with_config(eo_display_with_serialize_deserialize_field, config)
                    )
                ))
            }
        }
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_> for ErrorNamedTwo {
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{\n{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(
                    & format!(
                        "eo_display_with_serialize_deserialize_field: {}",
                        ToStringWithoutConfig:: to_string_without_config(eo_display_with_serialize_deserialize_field)
                    )
                ))
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
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_>
    for ErrorNamedTwoWithSerializeDeserialize
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedTwoWithSerializeDeserialize::Variant {
                eo_display_with_serialize_deserialize_field,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{\n{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                ::
                lines_space_backslash(
                    & format!(
                        "eo_display_with_serialize_deserialize_field: {}",
                        ToStringWithoutConfigWithSerializeDeserialize::to_string_without_config_with_serialize_deserialize(eo_display_with_serialize_deserialize_field)
                    )
                ))
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
                eo_display_with_serialize_deserialize_field: eo_display_with_serialize_deserialize_field.into_serialize_deserialize_version(),
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
        eo_display_with_serialize_deserialize_field: StdStringStringWithSerializeDeserialize,
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
