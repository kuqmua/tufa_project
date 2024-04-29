#[derive(Debug, thiserror::Error, 
    error_occurence_lib::ErrorOccurenceTest
)]
pub enum OneEnum {
    FirstVariant {
        #[eo_display_with_serialize_deserialize]
        first_variant: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}


fn main() {
    println!("1");
}


        // eo_display,
        // eo_display_with_serialize_deserialize,
        // eo_display_foreign_type,
        // eo_display_foreign_type_with_serialize_deserialize,
        // eo_error_occurence,
        // //todo error_occurence version for - after errors after deserialization
        // eo_vec_display,//todo maybe add version without generation \n for each element?
        // eo_vec_display_with_serialize_deserialize,
        // eo_vec_display_foreign_type,
        // eo_vec_display_foreign_type_with_serialize_deserialize,
        // eo_vec_error_occurence,
        // eo_hashmap_key_display_with_serialize_deserialize_value_display,
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize,
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type,
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize,
        // eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence,
        // eo_hashmap_key_display_foreign_type_value_display,
        // eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize,
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type,
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize,
        // eo_hashmap_key_display_foreign_type_value_error_occurence,