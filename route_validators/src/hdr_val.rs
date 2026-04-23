use axum::http::{
    HeaderMap,
    header::{AsHeaderName, HeaderValue, ToStrError},
};
#[allow(clippy::single_call_fn)] // core helper centralizes required-header transform flow reused by parsing helpers
pub(crate) fn get_required_header_mapped<'headers, E, T>(
    headers: &'headers HeaderMap,
    header_name: impl AsHeaderName,
    no_header_er: impl FnOnce() -> E,
    map: impl FnOnce(&'headers HeaderValue) -> Result<T, E>,
) -> Result<T, E> {
    map(get_required_header(headers, header_name, no_header_er)?)
}
#[allow(clippy::single_call_fn)] // helper centralizes required-header parsing and is reused by this module tests
pub(crate) fn get_required_header<E>(
    headers: &HeaderMap,
    header_name: impl AsHeaderName,
    no_header_er: impl FnOnce() -> E,
) -> Result<&HeaderValue, E> {
    headers.get(header_name).ok_or_else(no_header_er)
}
#[allow(clippy::single_call_fn)] // helper centralizes required-header string parsing and is reused by route validators
pub(crate) fn get_required_header_str<E>(
    headers: &HeaderMap,
    header_name: impl AsHeaderName,
    no_header_er: impl FnOnce() -> E,
    to_str_er: impl FnOnce(ToStrError) -> E,
) -> Result<&str, E> {
    get_required_header_mapped(headers, header_name, no_header_er, |header_value| {
        header_value.to_str().map_err(to_str_er)
    })
}
#[allow(clippy::single_call_fn)] // helper centralizes required-header string parsing and is reused by route validators
pub(crate) fn get_required_header_str_parsed<'headers, E, T>(
    headers: &'headers HeaderMap,
    header_name: impl AsHeaderName,
    no_header_er: impl FnOnce() -> E,
    to_str_er: impl FnOnce(ToStrError) -> E,
    parse: impl FnOnce(&'headers str) -> Result<T, E>,
) -> Result<T, E> {
    parse(get_required_header_str(
        headers,
        header_name,
        no_header_er,
        to_str_er,
    )?)
}
#[cfg(test)]
mod tests {
    use super::{
        get_required_header, get_required_header_mapped, get_required_header_str,
        get_required_header_str_parsed,
    };
    use crate::test_hlp::{mk_headers_with_entry, non_utf8_header_value};
    use axum::http::{
        HeaderMap,
        header::{AsHeaderName, HeaderName, HeaderValue},
    };
    const TEST_HEADER_NAME: HeaderName = HeaderName::from_static("x-test-header");
    #[derive(Debug, PartialEq, Eq)]
    enum TestEr {
        NoHeader,
        ParseBool,
        ToStr,
    }
    fn get_header(headers: &HeaderMap, name: impl AsHeaderName) -> Result<&str, TestEr> {
        get_required_header_str(headers, name, || TestEr::NoHeader, |_| TestEr::ToStr)
    }
    fn get_raw_header(
        headers: &HeaderMap,
        name: impl AsHeaderName,
    ) -> Result<&HeaderValue, TestEr> {
        get_required_header(headers, name, || TestEr::NoHeader)
    }
    fn get_bool_header(headers: &HeaderMap, name: impl AsHeaderName) -> Result<bool, TestEr> {
        get_required_header_str_parsed(
            headers,
            name,
            || TestEr::NoHeader,
            |_to_str_er| TestEr::ToStr,
            |header_value| {
                header_value
                    .parse::<bool>()
                    .map_err(|_parse_bool_er| TestEr::ParseBool)
            },
        )
    }
    #[test]
    fn get_required_header_str_returns_header_when_present_and_utf8() {
        let headers = mk_headers_with_entry(TEST_HEADER_NAME, HeaderValue::from_static("abc"));
        let actual = get_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Ok("abc"));
    }
    #[test]
    fn get_required_header_str_returns_no_header_error_when_absent() {
        let headers = HeaderMap::new();
        let actual = get_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Err(TestEr::NoHeader));
    }
    #[test]
    fn get_required_header_str_returns_to_str_error_for_non_utf8_header() {
        let headers = mk_headers_with_entry(TEST_HEADER_NAME, non_utf8_header_value());
        let actual = get_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Err(TestEr::ToStr));
    }
    #[test]
    fn get_required_header_str_accepts_str_header_name() {
        let headers = mk_headers_with_entry(TEST_HEADER_NAME, HeaderValue::from_static("abc"));
        let actual = get_header(&headers, "x-test-header");
        assert_eq!(actual, Ok("abc"));
    }
    #[test]
    fn get_required_header_returns_header_value_when_present() {
        let headers = mk_headers_with_entry(TEST_HEADER_NAME, HeaderValue::from_static("abc"));
        let actual = get_raw_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Ok(&HeaderValue::from_static("abc")));
    }
    #[test]
    fn get_required_header_returns_no_header_error_when_absent() {
        let headers = HeaderMap::new();
        let actual = get_raw_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Err(TestEr::NoHeader));
    }
    #[test]
    fn get_required_header_parsed_returns_parsed_value_for_valid_header() {
        let headers = mk_headers_with_entry(TEST_HEADER_NAME, HeaderValue::from_static("true"));
        let actual = get_bool_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Ok(true));
    }
    #[test]
    fn get_required_header_parsed_returns_parse_error_for_invalid_header_value() {
        let headers = mk_headers_with_entry(TEST_HEADER_NAME, HeaderValue::from_static("nope"));
        let actual = get_bool_header(&headers, TEST_HEADER_NAME);
        assert_eq!(actual, Err(TestEr::ParseBool));
    }
    #[test]
    fn get_required_header_mapped_applies_mapping_for_present_header() {
        let headers = mk_headers_with_entry(TEST_HEADER_NAME, HeaderValue::from_static("abc"));
        let actual = get_required_header_mapped(
            &headers,
            TEST_HEADER_NAME,
            || TestEr::NoHeader,
            |v| v.to_str().map(str::len).map_err(|_to_str_er| TestEr::ToStr),
        );
        assert_eq!(actual, Ok(3));
    }
}
