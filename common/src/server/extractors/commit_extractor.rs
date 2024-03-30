#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::server::routes::api::cats::TryCreateMany
    ,
    crate::repositories_types::server::routes::api::cats::TryCreateOne
    ,
    crate::repositories_types::server::routes::api::cats::TryReadMany
    ,
    crate::repositories_types::server::routes::api::cats::TryReadOne
    ,
    crate::repositories_types::server::routes::api::cats::TryUpdateMany
    ,
    crate::repositories_types::server::routes::api::cats::TryUpdateOne
    // ,
    // crate::repositories_types::server::routes::api::cats::TryDeleteMany
    // ,
    // crate::repositories_types::server::routes::api::cats::TryDeleteOne
)]
pub enum CommitExtractorCheckErrorNamed {
    #[tvfrr_400_bad_request]
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

// //todo make a proc macro for it or maybe put it into error occurence?
impl crate::common::to_default_stringified_json::ToDefaultStringifiedJson
    for CommitExtractorCheckErrorNamedWithSerializeDeserialize
{
    fn to_default_stringified_json(&self) -> std::string::String {
        match self {
            CommitExtractorCheckErrorNamedWithSerializeDeserialize::CommitExtractorNotEqual { commit_not_equal: _, commit_to_use: _, code_occurence: _ } => std::string::String::from(
                "{\"CommitExtractorNotEqual\":{\"commit_not_equal\":\"\",\"commit_to_use\":\"\",\"code_occurence\":{\"file\":\"\",\"line\":0,\"column\":0,\"git_info\":{\"git_commit_id\":\"\",\"git_repo_link\":\"\"},\"duration\":{\"secs\":0,\"nanos\":0}}}}"
            ),
            CommitExtractorCheckErrorNamedWithSerializeDeserialize::CommitExtractorToStrConversion { commit_to_str_conversion: _, code_occurence: _ } => std::string::String::from(
                "{\"CommitExtractorToStrConversion\":{\"commit_to_str_conversion\":\"\",\"code_occurence\":{\"file\":\"\",\"line\":0,\"column\":0,\"git_info\":{\"git_commit_id\":\"\",\"git_repo_link\":\"\"},\"duration\":{\"secs\":0,\"nanos\":0}}}}"
            ),
            CommitExtractorCheckErrorNamedWithSerializeDeserialize::NoCommitExtractorHeader { no_commit_header: _, code_occurence: _ } => std::string::String::from(
                "{\"NoCommitExtractorHeader\":{\"no_commit_header\":\"no_commit_header\",\"code_occurence\":{\"file\":\"\",\"line\":124,\"column\":53,\"git_info\":{\"git_commit_id\":\"\",\"git_repo_link\":\"\"},\"duration\":{\"secs\":0,\"nanos\":0}}}}"
            ),
        }
    }
}
