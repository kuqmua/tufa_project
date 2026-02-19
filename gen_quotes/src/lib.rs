use proc_macro2::TokenStream as Ts2;
use std::fmt::Display;
#[must_use]
pub fn single_quotes_str(v: &str) -> String {
    format!("\'{v}\'")
}
#[must_use]
pub fn single_quotes_ts(v: &str) -> Ts2 {
    single_quotes_str(v).parse::<Ts2>().expect("ec1e77d5")
}
pub fn dq_str(v: &dyn Display) -> String {
    format!("\"{v}\"")
}
pub fn dq_ts(v: &dyn Display) -> Ts2 {
    dq_str(v).parse::<Ts2>().expect("0391ac99")
}
#[must_use]
pub fn binary_single_quotes_str(v: &str) -> String {
    format!("b\'{v}\'")
}
#[must_use]
pub fn binary_single_quotes_ts(v: &str) -> Ts2 {
    binary_single_quotes_str(v)
        .parse::<Ts2>()
        .expect("8bce26e7")
}
#[must_use]
pub fn binary_dq_str(v: &dyn Display) -> String {
    format!("b\"{v}\"")
}
#[must_use]
pub fn binary_dq_ts(v: &dyn Display) -> Ts2 {
    binary_dq_str(v).parse::<Ts2>().expect("5dc6f142")
}
