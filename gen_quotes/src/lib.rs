use proc_macro2::TokenStream as Ts2;
use std::fmt::{Display, Write as _};
fn quote_literal(prefix: &str, quote_ch: char, v: &dyn Display) -> String {
    let mut out = String::with_capacity(prefix.len().saturating_add(16));
    out.push_str(prefix);
    out.push(quote_ch);
    write!(&mut out, "{v}").expect("31febb89");
    out.push(quote_ch);
    out
}
fn quote_literal_ts(prefix: &str, quote_ch: char, v: &dyn Display, panic_id: &str) -> Ts2 {
    quote_literal(prefix, quote_ch, v)
        .parse::<Ts2>()
        .unwrap_or_else(|_| panic!("{panic_id}"))
}
#[must_use]
pub fn single_quotes_str(v: &str) -> String {
    quote_literal("", '\'', &v)
}
#[must_use]
pub fn single_quotes_ts(v: &str) -> Ts2 {
    quote_literal_ts("", '\'', &v, "ec1e77d5")
}
#[must_use]
pub fn dq_str(v: &dyn Display) -> String {
    quote_literal("", '\"', v)
}
#[must_use]
pub fn dq_ts(v: &dyn Display) -> Ts2 {
    quote_literal_ts("", '\"', v, "0391ac99")
}
#[must_use]
pub fn binary_single_quotes_str(v: &str) -> String {
    quote_literal("b", '\'', &v)
}
#[must_use]
pub fn binary_single_quotes_ts(v: &str) -> Ts2 {
    quote_literal_ts("b", '\'', &v, "8bce26e7")
}
#[must_use]
pub fn binary_dq_str(v: &dyn Display) -> String {
    quote_literal("b", '\"', v)
}
#[must_use]
pub fn binary_dq_ts(v: &dyn Display) -> Ts2 {
    quote_literal_ts("b", '\"', v, "5dc6f142")
}
#[cfg(test)]
mod tests {
    use super::{
        binary_dq_str, binary_dq_ts, binary_single_quotes_str, binary_single_quotes_ts, dq_str,
        dq_ts, single_quotes_str, single_quotes_ts,
    };
    #[test]
    fn quote_str_helpers_return_expected_literals() {
        assert_eq!(single_quotes_str("abc"), "'abc'");
        assert_eq!(dq_str(&"abc"), "\"abc\"");
        assert_eq!(binary_single_quotes_str("abc"), "b'abc'");
        assert_eq!(binary_dq_str(&"abc"), "b\"abc\"");
    }
    #[test]
    fn quote_ts_helpers_return_expected_tokens() {
        assert_eq!(single_quotes_ts("a").to_string(), "'a'");
        assert_eq!(dq_ts(&"abc").to_string(), "\"abc\"");
        assert_eq!(binary_single_quotes_ts("a").to_string(), "b'a'");
        assert_eq!(binary_dq_ts(&"abc").to_string(), "b\"abc\"");
    }
}
