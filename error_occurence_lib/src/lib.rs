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