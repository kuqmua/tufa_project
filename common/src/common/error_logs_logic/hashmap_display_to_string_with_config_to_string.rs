pub trait HashMapDisplayToStringWithConfigToString<'a, ConfigGeneric> {
    fn hashmap_display_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric, ConfigGeneric>
    HashMapDisplayToStringWithConfigToString<'a, ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric:
        crate::common::error_logs_logic::to_string_with_config::ToStringWithConfig<'a>,
    ConfigGeneric: crate::common::config::config_fields::GetSourcePlaceType
        + crate::common::config::config_fields::GetTimezone,
{
    fn hashmap_display_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String {
        crate::common::error_logs_logic::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &crate::common::error_logs_logic::helpers::stringified_lines_error_hashmap_element(
                        key,
                        value.to_string_with_config(config),
                    ),
                );
                acc
            })
        )
    }
}
