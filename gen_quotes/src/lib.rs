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
#[allow(clippy::single_call_fn)] // shared with prefix-aware token quote wrapper to keep parse+panic-id flow in one place
fn quote_literal_ts<Dsp>(prefix: &str, quote_ch: char, v: &Dsp, panic_id: &str) -> Ts2
where
    Dsp: Display + ?Sized,
{
    quote_literal(prefix, quote_ch, v)
        .parse::<Ts2>()
        .unwrap_or_else(|_| panic!("{panic_id}"))
}
fn quote_str_with_prefix<Dsp>(prefix: &str, quote_ch: char, v: &Dsp) -> String
where
    Dsp: Display + ?Sized,
{
    quote_literal(prefix, quote_ch, v)
}
fn quote_ts_with_prefix<Dsp>(prefix: &str, quote_ch: char, v: &Dsp, panic_id: &str) -> Ts2
where
    Dsp: Display + ?Sized,
{
    quote_literal_ts(prefix, quote_ch, v, panic_id)
}
#[must_use]
pub fn single_quotes_str(v: &str) -> String {
    quote_str_with_prefix(NO_PREFIX, SINGLE_QUOTE, v)
}
#[must_use]
pub fn single_quotes_ts(v: &str) -> Ts2 {
    quote_ts_with_prefix(NO_PREFIX, SINGLE_QUOTE, v, "ec1e77d5")
}
#[must_use]
pub fn dq_str<Dsp>(v: &Dsp) -> String
where
    Dsp: Display + ?Sized,
{
    quote_str_with_prefix(NO_PREFIX, DQ, v)
}
#[must_use]
pub fn dq_ts<Dsp>(v: &Dsp) -> Ts2
where
    Dsp: Display + ?Sized,
{
    quote_ts_with_prefix(NO_PREFIX, DQ, v, "0391ac99")
}
#[must_use]
pub fn binary_single_quotes_str(v: &str) -> String {
    quote_str_with_prefix(BINARY_PREFIX, SINGLE_QUOTE, v)
}
#[must_use]
pub fn binary_single_quotes_ts(v: &str) -> Ts2 {
    quote_ts_with_prefix(BINARY_PREFIX, SINGLE_QUOTE, v, "8bce26e7")
}
#[must_use]
pub fn binary_dq_str<Dsp>(v: &Dsp) -> String
where
    Dsp: Display + ?Sized,
{
    quote_str_with_prefix(BINARY_PREFIX, DQ, v)
}
#[must_use]
pub fn binary_dq_ts<Dsp>(v: &Dsp) -> Ts2
where
    Dsp: Display + ?Sized,
{
    quote_ts_with_prefix(BINARY_PREFIX, DQ, v, "5dc6f142")
}
#[cfg(test)]
mod tests {
    use super::{
        binary_dq_str, binary_dq_ts, binary_single_quotes_str, binary_single_quotes_ts, dq_str,
        dq_ts, single_quotes_str, single_quotes_ts,
    };
    fn assert_quote_str(actual: &str, expected: &str) {
        assert_eq!(actual, expected);
    }
    fn assert_quote_ts(actual: &proc_macro2::TokenStream, expected: &str) {
        assert_eq!(actual.to_string(), expected);
    }
    #[test]
    fn quote_str_helpers_return_expected_literals() {
        assert_quote_str(&single_quotes_str("abc"), "'abc'");
        assert_quote_str(&dq_str(&"abc"), "\"abc\"");
        assert_quote_str(&binary_single_quotes_str("abc"), "b'abc'");
        assert_quote_str(&binary_dq_str(&"abc"), "b\"abc\"");
    }
    #[test]
    fn quote_ts_helpers_return_expected_tokens() {
        assert_quote_ts(&single_quotes_ts("a"), "'a'");
        assert_quote_ts(&dq_ts(&"abc"), "\"abc\"");
        assert_quote_ts(&binary_single_quotes_ts("a"), "b'a'");
        assert_quote_ts(&binary_dq_ts(&"abc"), "b\"abc\"");
    }
    #[test]
    fn quote_helpers_support_non_string_display_inputs() {
        assert_quote_str(&dq_str(&42i32), "\"42\"");
        assert_quote_str(&binary_dq_str(&42i32), "b\"42\"");
        assert_quote_ts(&dq_ts(&42i32), "\"42\"");
        assert_quote_ts(&binary_dq_ts(&42i32), "b\"42\"");
    }
    #[test]
    fn quote_helpers_handle_empty_input() {
        use std::panic::catch_unwind;
        assert_quote_str(&single_quotes_str(""), "''");
        assert_quote_str(&dq_str(&""), "\"\"");
        assert_quote_str(&binary_single_quotes_str(""), "b''");
        assert_quote_str(&binary_dq_str(&""), "b\"\"");
        match catch_unwind(|| single_quotes_ts("")) {
            Err(_) => (),
            Ok(_) => panic!("a71c4d2e"),
        }
        assert_quote_ts(&dq_ts(&""), "\"\"");
        match catch_unwind(|| binary_single_quotes_ts("")) {
            Err(_) => (),
            Ok(_) => panic!("f13b9c70"),
        }
        assert_quote_ts(&binary_dq_ts(&""), "b\"\"");
    }
}
