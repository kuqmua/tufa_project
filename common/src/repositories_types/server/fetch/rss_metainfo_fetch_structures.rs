#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum NoItemsErrorNamed {
    ThereIsTag {
        #[eo_to_std_string_string_serialize_deserialize]
        tag: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ConversionFromStrError {
        #[eo_to_std_string_string_serialize_deserialize]
        string: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoTag {
        #[eo_to_std_string_string_serialize_deserialize]
        no_tag: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl NoItemsErrorNamed {
    pub const fn get_stringified_kind(error: &Self) -> &'static str {
        match error {
            Self::ThereIsTag { .. } => stringify!(Self::ThereIsTag),
            Self::ConversionFromStrError { .. } => {
                stringify!(Self::ConversionFromStrError)
            }
            Self::NoTag { .. } => stringify!(Self::NoTag),
        }
    }
    // pub fn into_json_with_link_and_provider_kind(
    //     link: &str,
    //     no_items_error: &NoItemsErrorNamed,
    //     pk: &crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind,
    // ) -> serde_json::Value {
    //     match no_items_error {
    //         NoItemsErrorNamed::ThereIsTag{ tag, .. } => {
    //             serde_json::json!({
    //                 "error_kind": NoItemsErrorNamed::get_stringified_kind(no_items_error),
    //                 "link": link,
    //                 "tag": tag,
    //                 "part_of": format!("{pk}"),
    //                 "date": chrono::Local::now().to_string()
    //             })
    //         }
    //         NoItemsErrorNamed::ConversionFromStrError{ string, error, .. } => serde_json::json!({
    //             "error_kind": NoItemsErrorNamed::get_stringified_kind(no_items_error),
    //             "link": link,
    //             "string": string,
    //             "error": error,
    //             "part_of": format!("{pk}"),
    //             "date": chrono::Local::now().to_string()
    //         }),
    //         NoItemsErrorNamed::NoTag{ no_tag, .. } => serde_json::json!({
    //             "error_kind": NoItemsErrorNamed::get_stringified_kind(no_items_error),
    //             "link": link,
    //             "tag": no_tag,
    //             "part_of": format!("{pk}"),
    //             "date": chrono::Local::now().to_string()
    //         }),
    //     }
    // }
}
