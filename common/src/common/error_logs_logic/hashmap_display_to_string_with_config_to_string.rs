pub trait HashMapDisplayToStringWithConfigToString<'a, ConfigGeneric> {
    fn hashmap_display_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric>
    HashMapDisplayToStringWithConfigToString<'a, ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        error_occurence_lib::to_string_with_config::ToStringWithConfig<'a>,
    ConfigGeneric: config_lib::config_fields::GetSourcePlaceType
        + config_lib::config_fields::GetTimezone,
{
    fn hashmap_display_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String {
        error_occurence_lib::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &error_occurence_lib::helpers::stringified_lines_error_hashmap_element(
                        key,
                        value.to_string_with_config(config),
                    ),
                );
                acc
            })
        )
    }
}
