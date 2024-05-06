pub trait VecToStringWithConfigToString<'a, ConfigGeneric: ?Sized> {
    fn vec_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String;
}

// impl<'a, VecElementGeneric, ConfigGeneric> VecToStringWithConfigToString<'a, ConfigGeneric>
//     for Vec<VecElementGeneric>
// where
//     VecElementGeneric: crate::to_string_with_config::ToStringWithConfig<'a>,
//     ConfigGeneric: app_state::GetSourcePlaceType
//         + app_state::GetTimezone
//         + ?Sized,
// {
//     fn vec_to_string_with_config_to_string(&self, config: &ConfigGeneric) -> std::string::String {
//         crate::helpers::stringified_lines_error_vec(self.iter().fold(
//             std::string::String::from(""),
//             |mut acc, vec_element| {
//                 acc.push_str(&crate::helpers::lines_space_backslash_addition(
//                     vec_element.to_string_with_config(config),
//                 ));
//                 acc
//             },
//         ))
//     }
// }
