// pub trait JsonValueResultExtractor<OkGeneric, ErrorGeneric> {
//     fn try_extract_value(
//         self,
//         app_state: &axum::extract::State<crate::repositories_types::server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
//     ) -> Result<OkGeneric, ErrorGeneric>;
// }

// impl<OkGeneric, ErrorGeneric> JsonValueResultExtractor<OkGeneric, ErrorGeneric>
//     for Result<axum::Json<OkGeneric>, axum::extract::rejection::JsonRejection>
// where
//     ErrorGeneric: std::convert::From<
//             crate::server::routes::helpers::json_extractor_error::JsonExtractorErrorNamed,
//         > + axum::response::IntoResponse,
// {
//     fn try_extract_value(
//         self,
//         app_state: &axum::extract::State<crate::repositories_types::server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync>,
//     ) -> Result<OkGeneric, ErrorGeneric> {
//         match self {
//             Ok(axum::Json(payload)) => Ok(payload),
//             Err(err) => {
//                 let error = crate::server::routes::helpers::json_extractor_error::JsonExtractorErrorNamed::from(err);
//                 error_occurence_lib::error_log::ErrorLog::error_log(
//                     &error,
//                     app_state.as_ref(),
//                 );
//                 Err(ErrorGeneric::from(error))
//             }
//         }
//     }
// }
