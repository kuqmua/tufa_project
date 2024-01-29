pub trait SourceToStringWithConfig<'a> {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::config_fields::GetSourcePlaceType
            + config_lib::config_fields::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}
