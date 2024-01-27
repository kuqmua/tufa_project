pub trait ToStringWithoutConfig<'a> {
    fn to_string_without_config(&self) -> std::string::String;
}

impl<'a, SelfGeneric> ToStringWithoutConfig<'a> for SelfGeneric
where
    SelfGeneric: error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'a>
        + crate::common::error_logs_logic::get_code_occurence::GetCodeOccurence,
{
    fn to_string_without_config(&self) -> std::string::String {
        crate::common::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config(),
            self.get_code_occurence(),
        )
    }
}
// //implemented coz you cant deserialize field into &'a GitInfo(not implememnted in serde)
pub trait ToStringWithoutConfigWithSerializeDeserialize<'a> {
    fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String;
}

impl<'a, SelfGeneric> ToStringWithoutConfigWithSerializeDeserialize<'a> for SelfGeneric
where
    SelfGeneric: error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'a>
        + crate::common::error_logs_logic::get_code_occurence::GetCodeOccurence,
{
    fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String {
        crate::common::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_without_config(),
            self.get_code_occurence(),
        )
    }
}
