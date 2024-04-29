pub trait HashMapToStdStringStringDisplayIntoHashMapStringDisplay<HashMapValueGeneric> {
    fn hashmap_to_std_string_string_display_into_hashmap_string_display(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapToStdStringStringDisplayIntoHashMapStringDisplay<HashMapValueGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric:
        to_std_string_string::ToStdStringString + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_to_std_string_string_display_into_hashmap_string_display(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric> {
        self.into_iter()
            .map(|(key, value)| (key.to_std_string_string(), value))
            .collect()
    }
}
