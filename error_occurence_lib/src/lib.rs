pub use error_occurence::ErrorOccurence;
pub mod code_occurence;
pub mod display_foreign_type;
pub mod error_log;
pub mod error_occurence_named;
pub mod error_occurence_unnamed;
pub mod hashmap_display_display_foreign_type_into_hashmap_display_string;
pub mod hashmap_display_display_foreign_type_into_hashmap_string_string;
pub mod hashmap_display_display_foreign_type_to_string;
pub mod hashmap_display_display_into_hashmap_display_string;
pub mod hashmap_display_display_into_hashmap_string_display;
pub mod hashmap_display_display_into_hashmap_string_string;
pub mod hashmap_display_display_to_string;
pub mod hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type;
pub mod hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string;
pub mod hashmap_display_foreign_type_display_foreign_type_to_string;
pub mod hashmap_display_foreign_type_display_into_hashmap_string_display;
pub mod hashmap_display_foreign_type_display_into_hashmap_string_string;
pub mod hashmap_display_foreign_type_display_to_string;
pub mod hashmap_display_foreign_type_to_string_with_config_to_string;
pub mod hashmap_display_foreign_type_to_string_without_config_to_string;
pub mod hashmap_display_to_string_with_config_to_string;
pub mod hashmap_display_to_string_without_config_to_string;
pub mod helpers;
pub mod lines_space_backslash;
pub mod source_to_string_with_config;
pub mod source_to_string_without_config;
#[cfg(test)]
pub mod test;
pub mod to_string_with_config;
pub mod to_string_without_config;
pub mod vec_display_foreign_type_into_vec_string;
pub mod vec_display_foreign_type_to_string;
pub mod vec_display_into_vec_string;
pub mod vec_display_to_string;
pub mod vec_to_string_with_config_to_string;
pub mod vec_to_string_without_config_to_string;

pub trait IntoSerdeSerializeSerdeDeserialize {
    fn into_serde_serialize_serde_deserialize(
        self,
    ) -> impl serde::Serialize + serde::Deserialize<'static>;
}

#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_string(),
            line!(),
            column!(),
            None,
        )
    }};
}