pub trait HashMapToStdStringStringDisplayIntoHashMapStringString {
    fn hashmap_to_std_string_string_display_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, std::string::String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher> HashMapToStdStringStringDisplayIntoHashMapStringString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric:
        to_std_string_string::ToStdStringString + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: std::fmt::Display,
{
    fn hashmap_to_std_string_string_display_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, std::string::String> {
        self.into_iter()
            .map(|(key, value)| (key.to_std_string_string(), value.to_string()))
            .collect()
    }
}
