pub trait HashMapToStdStringStringToStdStringStringIntoHashMapStringToStdStringString<
    HashMapValueGeneric,
>
{
    fn hashmap_to_std_string_string_to_std_string_string_into_hashmap_string_to_std_string_string(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric>;
}

impl<HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapToStdStringStringToStdStringStringIntoHashMapStringToStdStringString<
        HashMapValueGeneric,
    > for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric:
        to_std_string_string::ToStdStringString + std::cmp::Eq + std::hash::Hash,
    HashMapValueGeneric: to_std_string_string::ToStdStringString,
{
    fn hashmap_to_std_string_string_to_std_string_string_into_hashmap_string_to_std_string_string(
        self,
    ) -> std::collections::HashMap<String, HashMapValueGeneric> {
        self.into_iter()
            .map(|(key, value)| (key.to_std_string_string(), value))
            .collect()
    }
}
