pub trait HashMapDisplayForeignTypeDisplayToString {
    fn hashmap_display_foreign_type_display_to_string(&self) -> std::string::String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher> HashMapDisplayForeignTypeDisplayToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: display_foreign_type::DisplayForeignType,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_foreign_type_display_to_string(&self) -> std::string::String {
        crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
            std::string::String::new(),
            |mut acc, (key, value)| {
                acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
                    key.display_foreign_type(),
                    value.to_string(),
                ));
                acc
            },
        ))
    }
}
