pub fn single_quotes_stringified(inner_content: &str) -> std::string::String {
    format!("\'{inner_content}\'")
}
pub fn single_quotes_token_stream(inner_content: &str) -> proc_macro2::TokenStream {
    let value = single_quotes_stringified(inner_content);
    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn double_quotes_stringified(inner_content: &dyn std::fmt::Display) -> std::string::String {
    format!("\"{inner_content}\"")
}
pub fn double_quotes_token_stream(inner_content: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    let value = double_quotes_stringified(inner_content);
    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}

pub fn binary_single_quotes_stringified(inner_content: &str) -> std::string::String {
    format!("b\'{inner_content}\'")
}
pub fn binary_single_quotes_token_stream(inner_content: &str) -> proc_macro2::TokenStream {
    let value = binary_single_quotes_stringified(inner_content);
    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
pub fn binary_double_quotes_stringified(inner_content: &dyn std::fmt::Display) -> std::string::String {
    format!("b\"{inner_content}\"")
}
pub fn binary_double_quotes_token_stream(inner_content: &dyn std::fmt::Display) -> proc_macro2::TokenStream {
    let value = binary_double_quotes_stringified(inner_content);
    value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
}
