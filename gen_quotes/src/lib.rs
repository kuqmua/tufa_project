use proc_macro2::TokenStream as Ts2;
use std::fmt::Display;
#[must_use]
pub fn single_quotes_str(inner_content: &str) -> String {
    format!("\'{inner_content}\'")
}
#[must_use]
pub fn single_quotes_ts(inner_content: &str) -> Ts2 {
    let value = single_quotes_str(inner_content);
    value.parse::<Ts2>().expect("ec1e77d5")
}
pub fn dq_str(inner_content: &dyn Display) -> String {
    format!("\"{inner_content}\"")
}
pub fn dq_ts(inner_content: &dyn Display) -> Ts2 {
    let value = dq_str(inner_content);
    value.parse::<Ts2>().expect("0391ac99")
}
#[must_use]
pub fn binary_single_quotes_str(inner_content: &str) -> String {
    format!("b\'{inner_content}\'")
}
#[must_use]
pub fn binary_single_quotes_ts(inner_content: &str) -> Ts2 {
    let value = binary_single_quotes_str(inner_content);
    value.parse::<Ts2>().expect("8bce26e7")
}
#[must_use]
pub fn binary_dq_str(inner_content: &dyn Display) -> String {
    format!("b\"{inner_content}\"")
}
#[must_use]
pub fn binary_dq_ts(inner_content: &dyn Display) -> Ts2 {
    let value = binary_dq_str(inner_content);
    value.parse::<Ts2>().expect("5dc6f142")
}
