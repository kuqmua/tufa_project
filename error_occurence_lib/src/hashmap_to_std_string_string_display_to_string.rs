pub trait HashMapToStdStringStringDisplayToString {
    fn hashmap_to_std_string_string_display_to_string(&self) -> std::string::String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher> HashMapToStdStringStringDisplayToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: to_std_string_string::ToStdStringString,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_to_std_string_string_display_to_string(&self) -> std::string::String {
        crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
            std::string::String::new(),
            |mut acc, (key, value)| {
                acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
                    key.to_std_string_string(),
                    value.to_string(),
                ));
                acc
            },
        ))
    }
}
