pub trait ToStringWithConfig<'a> {
    fn to_string_with_config<
        ConfigGeneric: config_lib::config_fields::GetSourcePlaceType
            + config_lib::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}

impl<'a, SelfGeneric> ToStringWithConfig<'a> for SelfGeneric
where
    SelfGeneric: crate::source_to_string_with_config::SourceToStringWithConfig<'a>
        + crate::code_occurence::GetCodeOccurence,
{
    fn to_string_with_config<
        ConfigGeneric: config_lib::config_fields::GetSourcePlaceType
            + config_lib::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        crate::helpers::source_and_code_occurence_formatter(
            self.source_to_string_with_config(config),
            crate::code_occurence::CodeOccurencePrepareForLogWithConfig::code_occurence_prepare_for_log_with_config(
                self.get_code_occurence(),
                config
            )
        )
    }
}
