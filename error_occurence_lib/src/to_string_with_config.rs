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
    SelfGeneric: crate::source_to_string_with_config::SourceToStringWithConfig<'a>
        + crate::code_occurence::Get,
{
    fn to_string_with_config<
        ConfigGeneric: app_state::GetSourcePlaceType
            + app_state::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        crate::helpers::source_and_code_occurence_formatter(
            self.source_to_string_with_config(config),
            crate::code_occurence::PrepareForLogWithConfig::prepare_for_log_with_config(
                self.get(),
                config
            )
        )
    }
}