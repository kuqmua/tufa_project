pub trait VecToStringWithoutConfigToString<'a> {
    fn vec_to_string_without_config_to_string(&self) -> std::string::String;
}

impl<'a, VecElementGeneric> VecToStringWithoutConfigToString<'a> for Vec<VecElementGeneric>
where
    VecElementGeneric:
        error_occurence_lib::to_string_without_config::ToStringWithoutConfig<'a>,
{
    fn vec_to_string_without_config_to_string(&self) -> std::string::String {
        error_occurence_lib::helpers::stringified_lines_error_vec(self.iter().fold(
            std::string::String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &error_occurence_lib::helpers::lines_space_backslash_addition(
                        vec_element.to_string_without_config(),
                    ),
                );
                acc
            },
        ))
    }
}

pub trait VecToStringWithoutConfigToStringWithSerializeDeserialize<'a> {
    fn vec_to_string_without_config_to_string_with_serialize_deserialize(&self) -> std::string::String;
}

impl<'a, VecElementGeneric> VecToStringWithoutConfigToStringWithSerializeDeserialize<'a> for Vec<VecElementGeneric>
where
    VecElementGeneric:
        error_occurence_lib::to_string_without_config::ToStringWithoutConfigWithSerializeDeserialize<
            'a,
        >,
{
    fn vec_to_string_without_config_to_string_with_serialize_deserialize(&self) -> std::string::String {
        error_occurence_lib::helpers::stringified_lines_error_vec(self.iter().fold(
            std::string::String::from(""),
            |mut acc, vec_element| {
                acc.push_str(
                    &error_occurence_lib::helpers::lines_space_backslash_addition(
                        vec_element.to_string_without_config_with_serialize_deserialize(),
                    ),
                );
                acc
            },
        ))
    }
}
