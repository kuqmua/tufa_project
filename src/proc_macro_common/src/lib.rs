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