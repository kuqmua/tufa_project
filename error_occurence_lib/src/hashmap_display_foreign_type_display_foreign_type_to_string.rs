pub trait HashMapDisplayForeignTypeDisplayForeignTypeToString {
    fn hashmap_display_foreign_type_display_foreign_type_to_string(&self) -> std::string::String;
}

impl<HashMapKeyGeneric, HashMapValueGeneric> HashMapDisplayForeignTypeDisplayForeignTypeToString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric>
where
    HashMapKeyGeneric: crate::display_foreign_type::DisplayForeignType,
    HashMapValueGeneric: crate::display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_foreign_type_display_foreign_type_to_string(&self) -> std::string::String {
        crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
            String::from(""),
            |mut acc, (key, value)| {
                acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
                    key.display_foreign_type(),
                    value.display_foreign_type(),
                ));
                acc
            },
        ))
    }
}
