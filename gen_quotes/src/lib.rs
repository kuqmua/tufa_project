use proc_macro2::TokenStream as Ts2;
use std::fmt::{Display, Write as _};
const NO_PREFIX: &str = "";
const BINARY_PREFIX: &str = "b";
const SINGLE_QUOTE: char = '\'';
const DQ: char = '\"';
fn quote_literal<Dsp>(prefix: &str, quote_ch: char, v: &Dsp) -> String
where
    Dsp: Display + ?Sized,
{
    let mut out = String::with_capacity(prefix.len().saturating_add(2));
    out.push_str(prefix);
    out.push(quote_ch);
    if out.write_fmt(format_args!("{v}")).is_err() {
        return format!("{prefix}{quote_ch}{v}{quote_ch}");
    }
    out.push(quote_ch);
    out
}
fn quote_literal_ts<Dsp>(prefix: &str, quote_ch: char, v: &Dsp, panic_id: &str) -> Ts2
where
    Dsp: Display + ?Sized,
{
    quote_literal(prefix, quote_ch, v)
        .parse::<Ts2>()
        .unwrap_or_else(|_| panic!("{panic_id}"))
}
#[must_use]
pub fn single_quotes_str(v: &str) -> String {
    quote_literal(NO_PREFIX, SINGLE_QUOTE, v)
}
#[must_use]
pub fn single_quotes_ts(v: &str) -> Ts2 {
    quote_literal_ts(NO_PREFIX, SINGLE_QUOTE, v, "ec1e77d5")
}
#[must_use]
pub fn dq_str<Dsp: Display>(v: &Dsp) -> String {
    quote_literal(NO_PREFIX, DQ, v)
}
#[must_use]
pub fn dq_ts<Dsp: Display>(v: &Dsp) -> Ts2 {
    quote_literal_ts(NO_PREFIX, DQ, v, "0391ac99")
}
#[must_use]
pub fn binary_single_quotes_str(v: &str) -> String {
    quote_literal(BINARY_PREFIX, SINGLE_QUOTE, v)
}
#[must_use]
pub fn binary_single_quotes_ts(v: &str) -> Ts2 {
    quote_literal_ts(BINARY_PREFIX, SINGLE_QUOTE, v, "8bce26e7")
}
#[must_use]
pub fn binary_dq_str<Dsp: Display>(v: &Dsp) -> String {
    quote_literal(BINARY_PREFIX, DQ, v)
}
#[must_use]
pub fn binary_dq_ts<Dsp: Display>(v: &Dsp) -> Ts2 {
    quote_literal_ts(BINARY_PREFIX, DQ, v, "5dc6f142")
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
    #[test]
    fn quote_helpers_support_non_string_display_inputs() {
        assert_eq!(dq_str(&42i32), "\"42\"");
        assert_eq!(binary_dq_str(&42i32), "b\"42\"");
        assert_eq!(dq_ts(&42i32).to_string(), "\"42\"");
        assert_eq!(binary_dq_ts(&42i32).to_string(), "b\"42\"");
    }
}
