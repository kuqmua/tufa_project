pub trait HashMapDisplayForeignTypeToStringWithoutConfigToString<'a> {
    fn hashmap_display_foreign_type_to_string_without_config_to_string(
        &self,
    ) -> std::string::String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapDisplayForeignTypeToStringWithoutConfigToString<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: display_foreign_type::DisplayForeignType,
    HashMapValueGeneric: crate::to_string_without_config::ToStringWithoutConfig<'a>,
{
    fn hashmap_display_foreign_type_to_string_without_config_to_string(
        &self,
    ) -> std::string::String {
        crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
            String::from(""),
            |mut acc, (key, value)| {
                acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
                    key.display_foreign_type(),
                    value.to_string_without_config(),
                ));
                acc
            },
        ))
    }
}

pub trait HashMapToStringDisplayForeignTypeToStringWithoutConfigWithSerializeDeserialize<'a> {
    fn hashmap_to_string_display_foreign_type_to_string_without_config_with_serialize_deserialize(
        &self,
    ) -> std::string::String;
}

impl<'a, HashMapKeyGeneric, HashMapValueGeneric, S: ::std::hash::BuildHasher>
    HashMapToStringDisplayForeignTypeToStringWithoutConfigWithSerializeDeserialize<'a>
    for std::collections::HashMap<HashMapKeyGeneric, HashMapValueGeneric, S>
where
    HashMapKeyGeneric: display_foreign_type::DisplayForeignType,
    HashMapValueGeneric:
        crate::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize<'a>,
{
    fn hashmap_to_string_display_foreign_type_to_string_without_config_with_serialize_deserialize(
        &self,
    ) -> std::string::String {
        crate::helpers::error_occurence_hashmap_formatter(self.iter().fold(
            String::from(""),
            |mut acc, (key, value)| {
                acc.push_str(&crate::helpers::stringified_lines_error_hashmap_element(
                    key.display_foreign_type(),
                    value.to_string_without_config_with_serialize_deserialize(),
                ));
                acc
            },
        ))
    }
}
