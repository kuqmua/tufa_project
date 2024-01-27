pub trait ToStringWithConfig<'a> {
    fn to_string_with_config<
        ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
            + crate::common::config::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}

impl<'a, SelfGeneric> ToStringWithConfig<'a> for SelfGeneric
where
    SelfGeneric:
        crate::common::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
                'a,
            > + crate::common::error_logs_logic::get_code_occurence::GetCodeOccurence,
{
    fn to_string_with_config<
        ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
            + crate::common::config::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        crate::common::error_logs_logic::helpers::source_and_code_occurence_formatter(
            self.source_to_string_with_config(config),
            crate::common::error_logs_logic::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithConfig::code_occurence_prepare_for_log_with_config(
                self.get_code_occurence(),
                config
            )
        )
    }
}
