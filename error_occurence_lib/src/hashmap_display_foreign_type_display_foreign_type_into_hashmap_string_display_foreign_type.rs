pub trait HashMapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringDisplayForeignType<
    HashMapValueGeneric,
>
{
    fn hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringDisplayForeignType<
        HashMapValueGeneric,
    > for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric:
        display_foreign_type::DisplayForeignType + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: display_foreign_type::DisplayForeignType,
{
    fn hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric> {
        self.into_iter()
            .map(|(key, value)| (key.display_foreign_type(), value))
            .collect()
    }
}
