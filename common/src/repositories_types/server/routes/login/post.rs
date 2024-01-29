// #[derive(serde::Deserialize)]
// pub struct FormData {
//     pub username: std::string::String,
//     pub password: secrecy::Secret<String>,
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum LoginErrorNamed {
//     AuthError {
//         #[eo_error_occurence]
//         validate_credentials: crate::repositories_types::server::authentication::password::ValidateCredentialsErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     SessionInsert {
//         #[eo_error_occurence]
//         session_insert: crate::repositories_types::server::session_state::InsertUserIdErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     }
// }
