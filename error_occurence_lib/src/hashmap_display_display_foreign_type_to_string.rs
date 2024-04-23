pub trait HashMapDisplayDisplayForeignTypeToString {
    fn hashmap_display_display_foreign_type_to_string(&self) -> std::string::String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher> HashMapDisplayDisplayForeignTypeToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: std::fmt::Display,
    HashMapValueGeneric: display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_display_foreign_type_to_string(&self) -> std::string::String {
        crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
            std::string::String::new(),
            |mut acc, (key, value)| {
                acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
                    key,
                    value.display_foreign_type(),
                ));
                acc
            },
        ))
    }
}
