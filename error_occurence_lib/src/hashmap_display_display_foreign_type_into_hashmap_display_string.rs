pub trait HashMapDisplayDisplayForeignTypeIntoHashMapDisplayString<HashMapKeyGeneric> {
    fn hashmap_display_display_foreign_type_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, std::string::String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapDisplayDisplayForeignTypeIntoHashMapDisplayString<HashMapKeyGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_display_foreign_type_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, std::string::String> {
        self.into_iter()
            .map(|(key, value)| (key, value.display_foreign_type()))
            .collect()
    }
}
