pub trait VecToStringWithConfigToString<'a, ConfigGeneric: ?Sized> {
    fn vec_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String;
}

impl<'a, VecElementGeneric, ConfigGeneric> VecToStringWithConfigToString<'a, ConfigGeneric>
    for Vec<VecElementGeneric>
where
    VecElementGeneric:
        error_occurence_lib::to_string_with_config::ToStringWithConfig<'a>,
    ConfigGeneric: config_lib::config_fields::GetSourcePlaceType
        + config_lib::config_fields::GetTimezone
        + ?Sized
{
    fn vec_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String {
        error_occurence_lib::helpers::stringified_lines_error_vec(self.iter().fold(
            std::string::String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &error_occurence_lib::helpers::lines_space_backslash_addition(
                        vec_element.to_string_with_config(config),
                    ),
                );
                acc
            },
        ))
    }
}
