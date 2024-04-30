pub trait SourceToStringWithConfig<'a> {
    fn source_to_string_with_config<
        ConfigGeneric: app_state::GetSourcePlaceType
            + app_state::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}

impl SourceToStringWithConfig<'_> for std::string::String {
    fn source_to_string_with_config<
        ConfigGeneric: app_state::GetSourcePlaceType + app_state::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        (*self).clone()
    }
}