pub trait HashMapToStdStringStringToStringWithoutConfigToString<'a> {
    fn hashmap_to_std_string_string_to_string_without_config_to_string(
        &self,
    ) -> std::string::String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapToStdStringStringToStringWithoutConfigToString<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: to_std_string_string::ToStdStringString,
    HashMapValueGeneric: crate::to_string_without_config::ToStringWithoutConfig<'a>,
{
    fn hashmap_to_std_string_string_to_string_without_config_to_string(
        &self,
    ) -> std::string::String {
        crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
            std::string::String::new(),
            |mut acc, (key, value)| {
                acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
                    key.to_std_string_string(),
                    value.to_string_without_config(),
                ));
                acc
            },
        ))
    }
}

// pub trait HashMapToStringToStdStringStringToStringWithoutConfigWithSerializeDeserialize<'a> {
//     fn hashmap_to_string_to_std_string_string_to_string_without_config_with_serialize_deserialize(
//         &self,
//     ) -> std::string::String;
// }

// impl<'a, HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
//     HashMapToStringToStdStringStringToStringWithoutConfigWithSerializeDeserialize<'a>
//     for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
// where
//     HashMapKeyGeneric: to_std_string_string::ToStdStringString,
//     HashMapValueGeneric:
//         crate::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize<'a>,
// {
//     fn hashmap_to_string_to_std_string_string_to_string_without_config_with_serialize_deserialize(
//         &self,
//     ) -> std::string::String {
//         crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
//             std::string::String::new(),
//             |mut acc, (key, value)| {
//                 acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
//                     key.to_std_string_string(),
//                     value.to_string_without_config_with_serialize_deserialize(),
//                 ));
//                 acc
//             },
//         ))
//     }
// }
