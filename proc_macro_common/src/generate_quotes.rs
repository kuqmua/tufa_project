pub fn generate_quotes_stringified(
    inner_content: &str,
) -> std::string::String {
    format!("\"{inner_content}\"")
}

pub fn generate_quotes_token_stream(
    inner_content: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let value_stringified = generate_quotes_stringified(inner_content);
    value_stringified.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_upper_camel_case_ident_stringified} {value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}