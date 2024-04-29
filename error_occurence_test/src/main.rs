#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurenceTest
)]
pub enum One {
    EoDisplayVariant {
        #[eo_display_with_serialize_deserialize]
        eo_display_field: std::string::String,
        #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: std::string::String,
        // #[eo_display_foreign_type]
        // eo_display_foreign_type_field: ,
        // #[eo_display_foreign_type_with_serialize_deserialize]
        // eo_display_foreign_type_with_serialize_deserialize_field: ,
        // #[eo_error_occurence]
        // eo_error_occurence_field: ,
        // #[eo_vec_display]
        // eo_vec_display_field: ,
        // #[eo_vec_display_with_serialize_deserialize]
        // eo_vec_display_with_serialize_deserialize_field: ,
        // #[eo_vec_display_foreign_type]
        // eo_vec_display_foreign_type_field: ,
        // #[eo_vec_display_foreign_type_with_serialize_deserialize]
        // eo_vec_display_foreign_type_with_serialize_deserialize_field: ,
        // #[eo_vec_error_occurence]
        // eo_vec_error_occurence_field: ,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_field: ,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_field: ,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_field: ,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        // eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize_field: ,
        // #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        // eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_display]
        // eo_hashmap_key_display_foreign_type_value_display_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_field: ,
        // #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        // eo_hashmap_key_display_foreign_type_value_error_occurence_field: ,

        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },

    // Variant {
    //     #[eo_]
    //     _field: ,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
}

// struct F {
//     k: std::string::String,
// }

// impl error_occurence_lib::DisplayForeignType for F {
//     fn display_foreign_type(&self) -> std::string::String {
//         format!("{self}")
//     }
// }

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