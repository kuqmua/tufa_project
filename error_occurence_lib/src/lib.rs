pub use error_occurence::ErrorOccurenceTest;

pub use error_occurence::ErrorOccurence;
pub use ::to_std_string_string::ToStdStringString;
pub mod code_occurence;
pub mod error_log;
pub mod error_occurence_named;
pub mod error_occurence_unnamed;
pub mod hashmap_display_to_std_string_string_into_hashmap_display_string;
pub mod hashmap_display_to_std_string_string_into_hashmap_string_string;
pub mod hashmap_display_to_std_string_string_to_string;
pub mod hashmap_display_display_into_hashmap_display_string;
pub mod hashmap_display_display_into_hashmap_string_display;
pub mod hashmap_display_display_into_hashmap_string_string;
pub mod hashmap_display_display_to_string;
pub mod hashmap_to_std_string_string_to_std_string_string_into_hashmap_string_to_std_string_string;
pub mod hashmap_to_std_string_string_to_std_string_string_into_hashmap_string_string;
pub mod hashmap_to_std_string_string_to_std_string_string_to_string;
pub mod hashmap_to_std_string_string_display_into_hashmap_string_display;
pub mod hashmap_to_std_string_string_display_into_hashmap_string_string;
pub mod hashmap_to_std_string_string_display_to_string;
pub mod hashmap_to_std_string_string_to_string_with_config_to_string;
pub mod hashmap_to_std_string_string_to_string_without_config_to_string;
pub mod hashmap_display_to_string_with_config_to_string;
pub mod hashmap_display_to_string_without_config_to_string;
pub mod helpers;
pub mod lines_space_backslash;
#[cfg(test)]
pub mod test;
pub mod to_string_with_config;
pub mod to_string_without_config;
pub mod vec_to_std_string_string_into_vec_string;
pub mod vec_to_std_string_string_to_string;
pub mod vec_display_into_vec_string;
pub mod vec_display_to_string;
pub mod vec_to_string_with_config_to_string;
pub mod vec_to_string_without_config_to_string;
pub mod source_to_string_with_config;
pub mod source_to_string_without_config;
pub mod primitive_types_wrappers;

#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            None,
        )
    }};
}

#[derive(Debug)]
pub struct StdVecVec<T>(pub std::vec::Vec<T>);
impl<T> std::fmt::Display for StdVecVec<T> 
    where T: std::fmt::Display
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            crate::helpers::stringified_lines_error_vec(self.0.iter().fold(
                std::string::String::from(""),
                |mut acc, element| {
                    acc.push_str(&crate::helpers::lines_space_backslash_addition(&element));
                    acc
                },
            ))
        )
    }
}
impl<T> crate::source_to_string_with_config::SourceToStringWithConfig<'_> for StdVecVec<T> 
where T: std::fmt::Display
{
    fn source_to_string_with_config<
        ConfigGeneric: app_state::GetSourcePlaceType + app_state::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl<T> crate::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdVecVec<T> 
where T: std::fmt::Display
{
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl<T> crate::code_occurence::GetOption for StdVecVec<T> {
    fn get_option(&self) -> std::option::Option<&crate::code_occurence::CodeOccurence> {
        None
    }
}
// impl<T> StdVecVec<T> {
//     pub fn into_serialize_deserialize_version(self) -> StdVecVecWithSerializeDeserialize<T> {
//         StdVecVecWithSerializeDeserialize(self.0.into_iter().map(|element|element.into_serialize_deserialize_version()).collect())
//     }
// }
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StdVecVecWithSerializeDeserialize<T>(std::vec::Vec<T>);
impl<T> std::fmt::Display for StdVecVecWithSerializeDeserialize<T> 
where T: std::fmt::Display
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter, 
            "{}",
            crate::helpers::stringified_lines_error_vec(self.0.iter().fold(
                std::string::String::from(""),
                |mut acc, element| {
                    acc.push_str(&crate::helpers::lines_space_backslash_addition(&element));
                    acc
                },
            ))
        )
    }
}
impl<T> crate::source_to_string_with_config::SourceToStringWithConfig<'_> for StdVecVecWithSerializeDeserialize<T>
where T: std::fmt::Display
{
    fn source_to_string_with_config<
        ConfigGeneric: app_state::GetSourcePlaceType + app_state::GetTimezone + ?Sized,
    >(
        &self,
        _: &ConfigGeneric,
    ) -> std::string::String {
        self.to_string()
    }
}
impl<T> crate::source_to_string_without_config::SourceToStringWithoutConfig<'_> for StdVecVecWithSerializeDeserialize<T> 
where T: std::fmt::Display
{
    fn source_to_string_without_config(&self) -> std::string::String {
        self.to_string()
    }
}
impl<T> crate::code_occurence::GetOption for StdVecVecWithSerializeDeserialize<T> {
    fn get_option(&self) -> std::option::Option<&crate::code_occurence::CodeOccurence> {
        None
    }
}