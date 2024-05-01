#[derive(Debug, PartialEq, Eq, Hash)]
pub struct StdStringString(pub std::string::String);
impl std::fmt::Display for StdStringString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", crate::helpers::lines_backslash_addition(&self.0))
    }
}
impl crate::source_to_string_with_config::SourceToStringWithConfig<'_> for StdStringString {
    fn source_to_string_with_config<
        ConfigGeneric: app_state::GetSourcePlaceType + app_state::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.0.clone()
    }
}
impl crate::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdStringString {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.0.clone()
    }
}
impl crate::code_occurence::GetOption for StdStringString {
    fn get_option(&self) -> std::option::Option<&crate::code_occurence::CodeOccurence> {
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
pub struct StdStringStringWithSerializeDeserialize(pub std::string::String);
impl std::fmt::Display for StdStringStringWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", crate::helpers::lines_backslash_addition(&self.0))
    }
}
impl crate::source_to_string_with_config::SourceToStringWithConfig<'_> for StdStringStringWithSerializeDeserialize {
    fn source_to_string_with_config<
        ConfigGeneric: app_state::GetSourcePlaceType + app_state::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.0.clone()
    }
}
impl crate::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdStringStringWithSerializeDeserialize {
    fn source_to_string_without_config(&self) -> std::string::String {
        self.0.clone()
    }
}
impl crate::code_occurence::GetOption for StdStringStringWithSerializeDeserialize {
    fn get_option(&self) -> std::option::Option<&crate::code_occurence::CodeOccurence> {
        None
    }
}