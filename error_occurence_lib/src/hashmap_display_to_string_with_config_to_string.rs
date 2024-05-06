pub trait HashMapDisplayToStringWithConfigToString<'a, ConfigGeneric> {
    fn hashmap_display_to_string_with_config_to_string(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}

// impl<'a, HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher, ConfigGeneric>
//     HashMapDisplayToStringWithConfigToString<'a, ConfigGeneric>
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric: crate::to_string_with_config::ToStringWithConfig<'a>,
//     ConfigGeneric:
//         app_state::GetSourcePlaceType + app_state::GetTimezone,
// {
//     fn hashmap_display_to_string_with_config_to_string(
//         &self,
//         config: &ConfigGeneric,
//     ) -> std::string::String {
//         crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
//             std::string::String::new(),
//             |mut acc, (key, value)| {
//                 acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
//                     key,
//                     value.to_string_with_config(config),
//                 ));
//                 acc
//             },
//         ))
//     }
// }
