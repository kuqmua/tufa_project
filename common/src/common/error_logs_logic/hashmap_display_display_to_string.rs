pub trait HashMapDisplayDisplayToString {
    fn hashmap_display_display_to_string(&self) -> std::string::String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayDisplayToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_display_to_string(&self) -> std::string::String {
        error_occurence_lib::helpers::error_occurence_hashmap_formatter(
            self.iter().fold(String::from(""), |mut acc, (key, value)| {
                acc.push_str(
                    &error_occurence_lib::helpers::stringified_lines_error_hashmap_element(
                        key,
                        value.to_string(),
                    ),
                );
                acc
            })
        )
    }
}
