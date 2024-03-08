#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker, //(rust analyzer does not work if type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker macro works for some reason)
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::server::routes::api::cats::TryCreateMany
    // ,
    // crate::repositories_types::server::routes::api::cats::TryCreateOne,
    // crate::repositories_types::server::routes::api::cats::TryReadMany,
    // crate::repositories_types::server::routes::api::cats::TryReadOne,
    // crate::repositories_types::server::routes::api::cats::TryUpdateMany,
    // crate::repositories_types::server::routes::api::cats::TryUpdateOne,
    // crate::repositories_types::server::routes::api::cats::TryDeleteMany,
    // crate::repositories_types::server::routes::api::cats::TryDeleteOne
)]
pub enum JsonExtractorErrorNamed {
    #[tvfrr_400_bad_request]
    JsonDataError {
        #[eo_display_with_serialize_deserialize]
        json_data_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    JsonSyntaxError {
        #[eo_display_with_serialize_deserialize]
        json_syntax_error: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl std::convert::From<axum::extract::rejection::JsonRejection> for JsonExtractorErrorNamed {
    fn from(e: axum::extract::rejection::JsonRejection) -> JsonExtractorErrorNamed {
        match e {
            axum::extract::rejection::JsonRejection::JsonDataError(json_data_error) => JsonExtractorErrorNamed::serde_json_error_response(json_data_error),
            axum::extract::rejection::JsonRejection::JsonSyntaxError(json_syntax_error) => JsonExtractorErrorNamed::serde_json_error_response(json_syntax_error),
            axum::extract::rejection::JsonRejection::MissingJsonContentType(_) => Self::MissingJsonContentType {
                missing_json_content_type: crate::server::routes::helpers::hardcode::MISSING_CONTENT_TYPE_APPLICATION_JSON_HEADER.to_string(),
                code_occurence: crate::code_occurence!(),
            },
            axum::extract::rejection::JsonRejection::BytesRejection(_) => {
                Self::BytesRejection {
                    bytes_rejection:
                        crate::server::routes::helpers::hardcode::FAILED_TO_BUFFER_REQUEST_BODY
                            .to_string(),
                    code_occurence: crate::code_occurence!(),
                }
            }
            _ => Self::UnexpectedCase {
                unexpected_case: crate::server::routes::helpers::hardcode::UNKNOWN_ERROR
                    .to_string(),
                code_occurence: crate::code_occurence!(),
            },
        }
    }
}

// attempt to extract the inner `serde_path_to_error::Error<serde_json::Error>`,
// if that succeeds we can provide a more specific error.
//
// `Json` uses `serde_path_to_error` so the error will be wrapped in `serde_path_to_error::Error`.
impl JsonExtractorErrorNamed {
    fn serde_json_error_response<E>(err: E) -> Self
    where
        E: std::error::Error + 'static,
    {
        if let Some(find_error_source_err) =
            find_error_source::<serde_path_to_error::Error<serde_json::Error>>(&err)
        {
            JsonExtractorErrorNamed::JsonDataError {
                json_data_error: format!("{err}: {}", find_error_source_err.inner()),
                code_occurence: crate::code_occurence!(),
            }
        } else {
            JsonExtractorErrorNamed::UnexpectedCase {
                unexpected_case: crate::server::routes::helpers::hardcode::UNKNOWN_ERROR
                    .to_string(),
                code_occurence: crate::code_occurence!(),
            }
        }
    }
}

// attempt to downcast `err` into a `T` and if that fails recursively try and
// downcast `err`'s source
fn find_error_source<'a, T>(err: &'a (dyn std::error::Error + 'static)) -> Option<&'a T>
where
    T: std::error::Error + 'static,
{
    if let Some(err) = err.downcast_ref::<T>() {
        Some(err)
    } else if let Some(source) = err.source() {
        find_error_source(source)
    } else {
        None
    }
}

pub trait JsonValueResultExtractor<OkGeneric, ErrorGeneric> {
    fn try_extract_value(
        self,
        app_state: &axum::extract::State<
            postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
        >,
    ) -> Result<OkGeneric, ErrorGeneric>;
}

impl<OkGeneric, ErrorGeneric> JsonValueResultExtractor<OkGeneric, ErrorGeneric>
    for Result<axum::Json<OkGeneric>, axum::extract::rejection::JsonRejection>
where
    ErrorGeneric: std::convert::From<
            crate::server::routes::helpers::json_extractor_error::JsonExtractorErrorNamed,
        > + axum::response::IntoResponse,
{
    fn try_extract_value(
        self,
        app_state: &axum::extract::State<
            postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
        >,
    ) -> Result<OkGeneric, ErrorGeneric> {
        match self {
            Ok(axum::Json(payload)) => Ok(payload),
            Err(err) => {
                let error = crate::server::routes::helpers::json_extractor_error::JsonExtractorErrorNamed::from(err);
                error_occurence_lib::error_log::ErrorLog::error_log(&error, app_state.as_ref());
                Err(ErrorGeneric::from(error))
            }
        }
    }
}

// ////////////////////////////todo remove it (rust analyzer does not work if type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker macro works for some reason)
