pub trait HashMapToStdStringStringToStringWithConfigToString<'a, ConfigGeneric> {
    fn hashmap_to_std_string_string_to_string_with_config_to_string(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher, ConfigGeneric>
    HashMapToStdStringStringToStringWithConfigToString<'a, ConfigGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: to_std_string_string::ToStdStringString,
    HashMapValueGeneric: crate::to_string_with_config::ToStringWithConfig<'a>,
    ConfigGeneric:
        app_state::GetSourcePlaceType + app_state::GetTimezone,
{
    fn hashmap_to_std_string_string_to_string_with_config_to_string(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
            std::string::String::new(),
            |mut acc, (key, value)| {
                acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
                    key.to_std_string_string(),
                    value.to_string_with_config(config),
                ));
                acc
            },
        ))
    }
}
