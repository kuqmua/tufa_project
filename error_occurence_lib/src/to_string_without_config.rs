// pub trait ToStringWithoutConfig<'a> {
//     fn to_string_without_config(&self) -> std::string::String;
// }

// impl<'a, SelfGeneric> ToStringWithoutConfig<'a> for SelfGeneric
// where
//     SelfGeneric: crate::source_to_string_without_config::SourceToStringWithoutConfig<'a>
//         + crate::code_occurence::Get,
// {
//     fn to_string_without_config(&self) -> std::string::String {
//         crate::helpers::source_and_code_occurence_formatter(
//             self.source_to_string_without_config(),
//             self.get(),
//         )
//     }
// }

pub trait ToStringWithoutConfig<'a> {
    fn to_string_without_config(&self) -> std::string::String;
}

impl<'a, SelfGeneric> ToStringWithoutConfig<'a> for SelfGeneric
where
    SelfGeneric: crate::source_to_string_without_config::SourceToStringWithoutConfig<'a>
        + crate::code_occurence::GetOption,
{
    fn to_string_without_config(&self) -> std::string::String {
        match self.get_option() {
            Some(value) => crate::helpers::source_and_code_occurence_formatter(
                self.source_to_string_without_config(),
                value,
            ),
            None => self.source_to_string_without_config(),
        }
    }
}

// //implemented coz you cant deserialize field into &'a GitInfo(not implememnted in serde)
// pub trait ToStringWithoutConfigWithSerializeDeserialize<'a> {
//     fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String;
// }

// impl<'a, SelfGeneric> ToStringWithoutConfigWithSerializeDeserialize<'a> for SelfGeneric
// where
//     SelfGeneric: crate::source_to_string_without_config::SourceToStringWithoutConfig<'a>
//         + crate::code_occurence::Get,
// {
//     fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String {
//         crate::helpers::source_and_code_occurence_formatter(
//             self.source_to_string_without_config(),
//             self.get(),
//         )
//     }
// }

pub trait ToStringWithoutConfigWithSerializeDeserialize<'a> {
    fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String;
}

impl<'a, SelfGeneric> ToStringWithoutConfigWithSerializeDeserialize<'a> for SelfGeneric
where
    SelfGeneric: crate::source_to_string_without_config::SourceToStringWithoutConfig<'a>
        + crate::code_occurence::GetOption,
{
    fn to_string_without_config_with_serialize_deserialize(&self) -> std::string::String {
        match self.get_option() {
            Some(value) => crate::helpers::source_and_code_occurence_formatter(
                self.source_to_string_without_config(),
                value,
            ),
            None => self.source_to_string_without_config(),
        }
    }
}