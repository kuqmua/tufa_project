pub fn single_quotes_stringified(inner_content: &str) -> std::string::String {
    format!("\'{inner_content}\'")
}
pub fn single_quotes_token_stream(
    inner_content: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let value = single_quotes_stringified(inner_content);
    value
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {value} {}",
                crate::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
            )
        })
}
pub fn double_quotes_stringified(inner_content: &str) -> std::string::String {
    format!("\"{inner_content}\"")
}
pub fn double_quotes_token_stream(
    inner_content: &str,
    proc_macro_name_upper_camel_case_ident_stringified: &str,
) -> proc_macro2::TokenStream {
    let value = double_quotes_stringified(inner_content);
    value
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| {
            panic!(
                "{proc_macro_name_upper_camel_case_ident_stringified} {value} {}",
                crate::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE
            )
        })
}
