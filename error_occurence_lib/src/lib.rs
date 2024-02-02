pub use error_occurence::ErrorOccurence;

pub mod error_log;
pub mod error_occurence_named;
pub mod error_occurence_unnamed;
// pub mod hashmap_display_display_foreign_type_into_hashmap_display_string;
// pub mod hashmap_display_display_foreign_type_into_hashmap_string_string;
// pub mod hashmap_display_display_foreign_type_to_string;
// pub mod hashmap_display_display_into_hashmap_display_string;
// pub mod hashmap_display_display_into_hashmap_string_display;
// pub mod hashmap_display_display_into_hashmap_string_string;
// pub mod hashmap_display_display_to_string;
// pub mod hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type;
// pub mod hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string;
// pub mod hashmap_display_foreign_type_display_foreign_type_to_string;
// pub mod hashmap_display_foreign_type_display_into_hashmap_string_display;
// pub mod hashmap_display_foreign_type_display_into_hashmap_string_string;
// pub mod hashmap_display_foreign_type_display_to_string;
// pub mod hashmap_display_foreign_type_to_string_with_config_to_string;
// pub mod hashmap_display_foreign_type_to_string_without_config_to_string;
// pub mod hashmap_display_to_string_with_config_to_string;
// pub mod hashmap_display_to_string_without_config_to_string;
pub mod lines_space_backslash;
// #[cfg(test)]
// pub mod test;
// pub mod vec_display_foreign_type_into_vec_string;
// pub mod vec_display_foreign_type_to_string;
// pub mod vec_display_into_vec_string;
// pub mod vec_display_to_string;
// pub mod vec_to_string_with_config_to_string;
// pub mod vec_to_string_without_config_to_string;

pub mod to_string_with_config;
pub mod to_string_without_config;
pub mod helpers;
pub mod source_to_string_with_config;
pub mod source_to_string_without_config;
pub mod get_code_occurence;
pub mod code_occurence_prepare_for_log;
pub mod get_code_path_without_config;
pub mod get_column;
pub mod get_duration;
pub mod get_file;
pub mod get_line;
pub mod form_error_path;
pub mod get_git_source_file_link;
pub mod git_fields;
pub mod git_info;
pub mod display_foreign_type;
pub mod get_code_path;
pub mod code_occurence;
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
#[cfg(test)]
pub mod test;
pub mod vec_display_foreign_type_into_vec_string;
pub mod vec_display_foreign_type_to_string;
pub mod vec_display_into_vec_string;
pub mod vec_display_to_string;
pub mod vec_to_string_with_config_to_string;
pub mod vec_to_string_without_config_to_string;


pub trait IntoSerializeDeserialize {
    fn into_serialize_deserialize(self) -> impl serde::Serialize + serde::Deserialize<'static>;
}