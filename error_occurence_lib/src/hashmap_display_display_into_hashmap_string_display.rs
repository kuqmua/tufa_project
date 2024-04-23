pub trait HashMapDisplayDisplayIntoHashMapStringDisplay<HashMapValueGeneric> {
    fn hashmap_display_display_into_hashmap_string_display(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapDisplayDisplayIntoHashMapStringDisplay<HashMapValueGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_display_display_into_hashmap_string_display(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric> {
        self.into_iter().map(|(key, value)| (key.to_string(), value)).collect()
    }
}
