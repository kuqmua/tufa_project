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
    SelfGeneric:
        error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<
                'a,
            > + error_occurence_lib::get_code_occurence::GetCodeOccurence,
{
    fn to_string_with_config<
        ConfigGeneric: config_lib::config_fields::GetSourcePlaceType
            + config_lib::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        error_occurence_lib::helpers::source_and_code_occurence_formatter(
            self.source_to_string_with_config(config),
            error_occurence_lib::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithConfig::code_occurence_prepare_for_log_with_config(
                self.get_code_occurence(),
                config
            )
        )
    }
}
