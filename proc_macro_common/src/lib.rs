pub mod panic_location;
pub mod global_variables;
pub mod naming_conventions;
pub mod generate_quotes;

pub fn generate_function_name_upper_camel_case_token_stream(
    proc_macro_name_stringified: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let value = crate::naming_conventions::ToUpperCamelCaseStringified::to_upper_camel_case_stringified(&proc_macro_name_stringified);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn generate_function_name_snake_case_token_stream(
    proc_macro_name_upper_camel_case_stringified: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let value = crate::naming_conventions::ToSnakeCaseStringified::to_snake_case_stringified(&proc_macro_name_upper_camel_case_stringified);
    value.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn trait_path_token_stream() -> proc_macro2::TokenStream {
    quote::quote! {proc_macro_common::naming_conventions}
}
pub fn std_string_string_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{std::string::String}
}
pub fn error_value_snake_case_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{e}
}
pub fn str_ref_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{&str}
}
pub fn thiserror_error_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{thiserror::Error}
}
pub fn error_occurence_lib_error_occurence_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{error_occurence_lib::ErrorOccurence}
}
pub fn utoipa_to_schema_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{utoipa::ToSchema}
}
pub fn serde_serialize_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{serde::Serialize}
}
pub fn serde_deserialize_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{serde::Deserialize}
}
pub fn sqlx_row_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{sqlx::Row}
}
pub fn http_status_code_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{http::StatusCode}
}
pub fn reqwest_header_header_map_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{reqwest::header::HeaderMap}
}
pub fn reqwest_error_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{reqwest::Error}
}
pub fn axum_response_into_response_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{axum::response::IntoResponse}//todo maybe remove impl
}
pub fn axum_extract_rejection_json_rejection_token_stream() -> proc_macro2::TokenStream {
    quote::quote!{axum::extract::rejection::JsonRejection}
}