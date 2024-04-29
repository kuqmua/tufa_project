pub trait HashMapDisplayToStdStringStringIntoHashMapStringString {
    fn hashmap_display_to_std_string_string_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, std::string::String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher> HashMapDisplayToStdStringStringIntoHashMapStringString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: std::fmt::Display + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: to_std_string_string::ToStdStringString,
{
    fn hashmap_display_to_std_string_string_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, std::string::String> {
        self.into_iter()
            .map(|(key, value)| (key.to_string(), value.to_std_string_string()))
            .collect()
    }
}
