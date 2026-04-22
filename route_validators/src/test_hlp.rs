use crate::GetAxumHttpStatusCode;
use axum::http::{
    HeaderMap, StatusCode,
    header::{HeaderValue, IntoHeaderName},
};
use std::{
    task::{Context, Poll, Waker},
    thread::yield_now,
};
const MAX_BLOCK_ON_POLLS: usize = 4096;
const EXPECT_OK_ER_ID: &str = "db9d2f63";
const EXPECT_ER_ER_ID: &str = "2f755472";
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
                poll_count = poll_count.checked_add(1).expect("3b1d9f0a");
                assert!(
                    poll_count < MAX_BLOCK_ON_POLLS,
                    "cf6e91ab block_on exceeded poll limit"
                );
                yield_now();
            }
        }
    }
}
#[track_caller]
pub(crate) fn panic_unexpected_variant(exp_id: &'static str) -> ! {
    panic!("4fe6f2e6 id={exp_id}");
}
pub(crate) fn expect_ok<T, E>(v: Result<T, E>, exp_id: &'static str) -> T {
    v.unwrap_or_else(|_| panic!("{EXPECT_OK_ER_ID} unexpected Err for expect_ok, id={exp_id}"))
}
pub(crate) fn expect_er<T, E>(v: Result<T, E>, exp_id: &'static str) -> E {
    v.map_or_else(
        |er| er,
        |_| panic!("{EXPECT_ER_ER_ID} unexpected Ok for expect_er, id={exp_id}"),
    )
}
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
pub(crate) fn mk_headers_with_entry(name: impl IntoHeaderName, value: HeaderValue) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let prev = headers.insert(name, value);
    assert!(prev.is_none());
    headers
}
#[cfg(test)]
mod tests {
    use super::{assert_err_status_code, block_on, expect_er, expect_ok, mk_headers_with_entry};
    use axum::http::{StatusCode, header::HeaderValue};
    use std::{future::poll_fn, panic::catch_unwind, task::Poll};
    #[test]
    fn block_on_panics_for_never_ready_future() {
        let panic_res = catch_unwind(|| block_on(poll_fn(|_| Poll::<u8>::Pending)));
        drop(panic_res.expect_err("1fc8c9f0"));
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
    fn mk_headers_with_entry_inserts_value_for_case_insensitive_name() {
        let headers = mk_headers_with_entry("Commit", HeaderValue::from_static("deadbeef"));
        let actual = headers.get("commit");
        assert_eq!(actual, Some(&HeaderValue::from_static("deadbeef")));
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
    }
}
