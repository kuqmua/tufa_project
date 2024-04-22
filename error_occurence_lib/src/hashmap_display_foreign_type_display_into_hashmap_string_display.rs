pub trait HashMapDisplayForeignTypeDisplayIntoHashMapStringDisplay<HashMapValueGeneric> {
    fn hashmap_display_foreign_type_display_into_hashmap_string_display(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapDisplayForeignTypeDisplayIntoHashMapStringDisplay<HashMapValueGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric:
        display_foreign_type::DisplayForeignType + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_foreign_type_display_into_hashmap_string_display(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric> {
        self.into_iter()
            .map(|(k, v)| (k.display_foreign_type(), v))
            .collect()
    }
}
