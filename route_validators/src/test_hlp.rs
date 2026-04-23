use crate::GetAxumHttpStatusCode;
use axum::http::{
    HeaderMap, StatusCode,
    header::{HeaderValue, IntoHeaderName},
};
use std::{
    panic::{UnwindSafe, catch_unwind},
    task::{Context, Poll, Waker},
    thread::yield_now,
};
const MAX_BLOCK_ON_POLLS: usize = 4096;
const BLOCK_ON_POLL_LIMIT_ER_ID: &str = "cf6e91ab";
const EXPECT_OK_ER_ID: &str = "db9d2f63";
const EXPECT_ER_ER_ID: &str = "2f755472";
#[allow(clippy::single_call_fn)] // extracted to keep block_on loop hot path simple and reusable
const fn is_block_on_poll_limit_reached(poll_count: usize) -> bool {
    poll_count >= MAX_BLOCK_ON_POLLS
}
pub(crate) fn block_on<T>(input_future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(input_future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut poll_count = 0usize;
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => {
                return output;
            }
            Poll::Pending => {
                assert!(
                    !is_block_on_poll_limit_reached(poll_count),
                    "{BLOCK_ON_POLL_LIMIT_ER_ID} block_on exceeded poll limit"
                );
                poll_count = poll_count.saturating_add(1);
                yield_now();
            }
        }
    }
}
#[track_caller]
pub(crate) fn panic_unexpected_variant(exp_id: &'static str) -> ! {
    panic!("4fe6f2e6 id={exp_id}");
}
#[track_caller]
fn panic_unexpected_result(
    er_id: &'static str,
    fn_name: &'static str,
    expected: &'static str,
    exp_id: &'static str,
) -> ! {
    panic!("{er_id} unexpected {expected} for {fn_name}, id={exp_id}");
}
#[track_caller]
pub(crate) fn expect_ok<T, E>(v: Result<T, E>, exp_id: &'static str) -> T {
    v.unwrap_or_else(|_| panic_unexpected_result(EXPECT_OK_ER_ID, "expect_ok", "Err", exp_id))
}
#[track_caller]
pub(crate) fn expect_er<T, E>(v: Result<T, E>, exp_id: &'static str) -> E {
    v.map_or_else(
        |er| er,
        |_| panic_unexpected_result(EXPECT_ER_ER_ID, "expect_er", "Ok", exp_id),
    )
}
#[track_caller]
pub(crate) fn assert_err_status_code<T, E>(
    v: Result<T, E>,
    exp_id: &'static str,
    expected: StatusCode,
) -> E
where
    E: GetAxumHttpStatusCode,
{
    let err = expect_er(v, exp_id);
    assert_eq!(err.get_axum_http_status_code(), expected);
    err
}
#[track_caller]
pub(crate) fn assert_err_status_code_only<T, E>(
    v: Result<T, E>,
    exp_id: &'static str,
    expected: StatusCode,
) where
    E: GetAxumHttpStatusCode,
{
    drop(assert_err_status_code(v, exp_id, expected));
}
pub(crate) fn mk_headers_with_entry(name: impl IntoHeaderName, value: HeaderValue) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let prev = headers.insert(name, value);
    assert!(prev.is_none());
    headers
}
pub(crate) fn non_utf8_header_value() -> HeaderValue {
    HeaderValue::from_bytes(&[0x80]).expect("86eb20cf")
}
#[track_caller]
pub(crate) fn assert_panics(action: impl FnOnce() + UnwindSafe, exp_id: &'static str) {
    let panic_res = catch_unwind(action);
    let panic_payload = panic_res.expect_err(exp_id);
    drop(panic_payload);
}
#[cfg(test)]
mod tests {
    use super::{
        assert_err_status_code, assert_err_status_code_only, assert_panics, block_on, expect_er,
        expect_ok, mk_headers_with_entry, non_utf8_header_value, panic_unexpected_variant,
    };
    use axum::http::{StatusCode, header::HeaderValue};
    use std::{future::poll_fn, task::Poll};
    #[test]
    fn block_on_panics_for_never_ready_future() {
        assert_panics(
            || {
                let _ignored = block_on(poll_fn(|_| Poll::<u8>::Pending));
            },
            "1fc8c9f0",
        );
    }
    #[test]
    fn poll_limit_helper_returns_false_below_limit_and_true_at_limit() {
        assert!(!super::is_block_on_poll_limit_reached(0));
        assert!(super::is_block_on_poll_limit_reached(
            super::MAX_BLOCK_ON_POLLS
        ));
    }
    #[test]
    fn expect_ok_returns_inner_value() {
        let v = expect_ok::<u8, u16>(Ok(7), "4f607799");
        assert_eq!(v, 7);
    }
    #[test]
    fn expect_er_returns_inner_error() {
        let v = expect_er::<u8, u16>(Err(9), "5cd39e4b");
        assert_eq!(v, 9);
    }
    #[test]
    fn panic_unexpected_variant_always_panics() {
        assert_panics(|| panic_unexpected_variant("f66647ab"), "b6dba95d");
    }
    #[test]
    fn mk_headers_with_entry_inserts_value_for_case_insensitive_name() {
        let headers = mk_headers_with_entry("Commit", HeaderValue::from_static("deadbeef"));
        let actual = headers.get("commit");
        assert_eq!(actual, Some(&HeaderValue::from_static("deadbeef")));
    }
    #[test]
    fn non_utf8_header_value_creates_non_utf8_header() {
        assert_eq!(
            non_utf8_header_value().to_str().err().map(|_| true),
            Some(true)
        );
    }
    #[test]
    fn assert_err_status_code_returns_error_after_status_check() {
        #[derive(Debug)]
        struct TestErr;
        impl crate::GetAxumHttpStatusCode for TestErr {
            const AXUM_HTTP_STATUS_CODE: StatusCode = StatusCode::BAD_REQUEST;
        }
        let _err = assert_err_status_code::<(), TestErr>(
            Err(TestErr),
            "4a1791d2",
            StatusCode::BAD_REQUEST,
        );
        assert_err_status_code_only::<(), TestErr>(
            Err(TestErr),
            "773c5af2",
            StatusCode::BAD_REQUEST,
        );
    }
}
