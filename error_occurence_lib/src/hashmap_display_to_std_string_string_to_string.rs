pub trait HashMapDisplayToStdStringStringToString {
    fn hashmap_display_to_std_string_string_to_string(&self) -> std::string::String;
}

// impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher> HashMapDisplayToStdStringStringToString
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
// where
//     HashMapKeyGeneric: std::fmt::Display,
//     HashMapValueGeneric: to_std_string_string::ToStdStringString,
// {
//     fn hashmap_display_to_std_string_string_to_string(&self) -> std::string::String {
//         crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
//             std::string::String::new(),
//             |mut acc, (key, value)| {
//                 acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
//                     key,
//                     value.to_std_string_string(),
//                 ));
//                 acc
//             },
//         ))
//     }
// }
