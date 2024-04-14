pub trait SourceToStringWithConfig<'a> {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType
            + config_lib::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}
