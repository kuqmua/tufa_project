pub trait CodeOccurencePrepareForLogWithConfig {
    fn code_occurence_prepare_for_log_with_config<
        ConfigGeneric: config_lib::config_fields::GetTimezone
            + config_lib::config_fields::GetSourcePlaceType
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}

impl<SelfGeneric> CodeOccurencePrepareForLogWithConfig for SelfGeneric
where
    SelfGeneric: crate::code_occurence::FormErrorPathDirectory
        + crate::code_occurence::FormErrorPathGithub
        + crate::code_occurence::GetDuration,
{
    fn code_occurence_prepare_for_log_with_config<
        ConfigGeneric: config_lib::config_fields::GetTimezone
            + config_lib::config_fields::GetSourcePlaceType
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        prepare_for_log(
            crate::code_occurence::get_code_path(config.get_source_place_type(), self),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(config.get_timezone())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
    }
}

pub trait CodeOccurencePrepareForLogWithoutConfig {
    fn code_occurence_prepare_for_log_without_config(&self) -> std::string::String;
}

impl<SelfGeneric> CodeOccurencePrepareForLogWithoutConfig for SelfGeneric
where
    SelfGeneric: crate::code_occurence::FormErrorPathGithub + crate::code_occurence::GetDuration,
{
    fn code_occurence_prepare_for_log_without_config(&self) -> std::string::String {
        prepare_for_log(
            self.form_error_path_github(),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
    }
}

pub trait CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize {
    fn code_occurence_prepare_for_log_without_config_with_serialize_deserialize(
        &self,
    ) -> std::string::String;
}

impl<SelfGeneric> CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize for SelfGeneric
where
    SelfGeneric: crate::code_occurence::FormErrorPathGithub + crate::code_occurence::GetDuration,
{
    fn code_occurence_prepare_for_log_without_config_with_serialize_deserialize(
        &self,
    ) -> std::string::String {
        prepare_for_log(
            self.form_error_path_github(),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
    }
}

fn prepare_for_log(path: std::string::String, time: std::string::String) -> std::string::String {
    format!("{path} {time}")
}
