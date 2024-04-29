pub trait HashMapDisplayToStdStringStringIntoHashMapDisplayString<HashMapKeyGeneric> {
    fn hashmap_display_to_std_string_string_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, std::string::String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapDisplayToStdStringStringIntoHashMapDisplayString<HashMapKeyGeneric>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: to_std_string_string::ToStdStringString,
{
    fn hashmap_display_to_std_string_string_into_hashmap_display_string(
        self,
    ) -> std::collections::HashMap<HashMapKeyGeneric, std::string::String> {
        self.into_iter()
            .map(|(key, value)| (key, value.to_std_string_string()))
            .collect()
    }
}
