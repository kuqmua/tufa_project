pub trait HashMapToStdStringStringToStdStringStringIntoHashMapStringString {
    fn hashmap_to_std_string_string_to_std_string_string_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, std::string::String>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapToStdStringStringToStdStringStringIntoHashMapStringString
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric:
        to_std_string_string::ToStdStringString + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: to_std_string_string::ToStdStringString,
{
    fn hashmap_to_std_string_string_to_std_string_string_into_hashmap_string_string(
        self,
    ) -> std::collections::HashMap<String, std::string::String> {
        self.into_iter()
            .map(|(key, value)| (key.to_std_string_string(), value.to_std_string_string()))
            .collect()
    }
}
