// #[derive(Debug, sqlx::Type)]
// #[sqlx(type_name = "header_pair")]
// pub struct HeaderPairRecord {
//     pub name: std::string::String,
//     pub value: Vec<u8>,
// }

// impl sqlx::postgres::PgHasArrayType for HeaderPairRecord {
//     fn array_type_info() -> sqlx::postgres::PgTypeInfo {
//         sqlx::postgres::PgTypeInfo::with_name("_header_pair")
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum GetSavedResponseErrorNamed {
//     PostgresSelect {
//         #[eo_to_std_string_string]
//         postgres_select: sqlx::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     TryFromIntError {
//         #[eo_to_std_string_string]
//         try_from_int_error: std::num::TryFromIntError,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     InvalidStatusCode {
//         #[eo_to_std_string_string]
//         invalid_status_code: http::status::InvalidStatusCode,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum SaveResponseErrorNamed {
//     BodyToBytes {
//         #[eo_to_std_string_string]
//         body_to_bytes: actix_web::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     PostgtesUpdate {
//         #[eo_to_std_string_string]
//         postgres_update: sqlx::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     PostgtesTransactionCommit {
//         #[eo_to_std_string_string]
//         postgres_transaction_commit: sqlx::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// pub enum NextAction {
//     ReturnSavedResponse(actix_web::HttpResponse),
//     StartProcessing(sqlx::Transaction<'static, sqlx::Postgres>),
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum TryProcessingErrorNamed {
//     PostgresPoolBegin {
//         #[eo_to_std_string_string]
//         pool_begin_error: sqlx::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     PostgresInsert {
//         #[eo_to_std_string_string]
//         insert: sqlx::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     GetSavedResponse {
//         #[eo_error_occurence]
//         get_saved_response: GetSavedResponseErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     SavedResponseIsNone {
//         #[eo_to_std_string_string_serialize_deserialize]
//         message: &'a str,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
