pub trait VecToStringWithConfigToString<'a, ConfigGeneric: ?Sized> {
    fn vec_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String;
}

impl<'a, VecElementGeneric, ConfigGeneric> VecToStringWithConfigToString<'a, ConfigGeneric>
    for Vec<VecElementGeneric>
where
    VecElementGeneric:
        crate::common::error_logs_logic::to_string_with_config::ToStringWithConfig<'a>,
    ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
        + crate::common::config::config_fields::GetTimezone
        + ?Sized
{
    fn vec_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String {
        crate::common::error_logs_logic::helpers::stringified_lines_error_vec(self.iter().fold(
            std::string::String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &crate::common::error_logs_logic::helpers::lines_space_backslash_addition(
                        vec_element.to_string_with_config(config),
                    ),
                );
                acc
            },
        ))
    }
}
