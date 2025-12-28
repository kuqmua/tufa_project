pub fn single_quotes_stringified(inner_content: &str) -> String {
    format!("\'{inner_content}\'")
}
pub fn single_quotes_token_stream(inner_content: &str) -> proc_macro2::TokenStream {
    let value = single_quotes_stringified(inner_content);
    value
        .parse::<proc_macro2::TokenStream>()
        .expect("ec1e77d5-3e00-4342-9464-85ab822beede")
}
pub fn double_quotes_stringified(inner_content: &dyn std::fmt::Display) -> String {
    format!("\"{inner_content}\"")
}
pub fn double_quotes_token_stream(
    inner_content: &dyn std::fmt::Display,
) -> proc_macro2::TokenStream {
    let value = double_quotes_stringified(inner_content);
    value
        .parse::<proc_macro2::TokenStream>()
        .expect("0391ac99-9d0e-4182-ba3d-91445b775aaa")
}

pub fn binary_single_quotes_stringified(inner_content: &str) -> String {
    format!("b\'{inner_content}\'")
}
pub fn binary_single_quotes_token_stream(inner_content: &str) -> proc_macro2::TokenStream {
    let value = binary_single_quotes_stringified(inner_content);
    value
        .parse::<proc_macro2::TokenStream>()
        .expect("8bce26e7-50d9-47cf-9572-f3756ebc1617")
}
pub fn binary_double_quotes_stringified(inner_content: &dyn std::fmt::Display) -> String {
    format!("b\"{inner_content}\"")
}
pub fn binary_double_quotes_token_stream(
    inner_content: &dyn std::fmt::Display,
) -> proc_macro2::TokenStream {
    let value = binary_double_quotes_stringified(inner_content);
    value
        .parse::<proc_macro2::TokenStream>()
        .expect("5dc6f142-e1af-44b4-8098-aacf10e2d959")
}
