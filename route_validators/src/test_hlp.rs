use crate::GetAxumHttpStatusCode;
use axum::http::{
    HeaderMap, StatusCode,
    header::{AsHeaderName, HeaderValue, IntoHeaderName},
};
use std::{
    fmt::Debug,
    panic::{UnwindSafe, catch_unwind},
    task::{Context, Poll, Waker},
    thread::yield_now,
};
const MAX_BLOCK_ON_POLLS: usize = 4096;
const BLOCK_ON_POLL_LIMIT_ER_ID: &str = "cf6e91ab";
const EXPECT_OK_ER_ID: &str = "db9d2f63";
const EXPECT_ER_ER_ID: &str = "2f755472";
const REPLACE_HEADER_MISSING_SRC_ER_ID: &str = "c3a0f7be";
#[allow(clippy::single_call_fn)] // shared insertion guard keeps header setup helpers consistent
fn insert_header_no_prev(headers: &mut HeaderMap, name: impl IntoHeaderName, value: HeaderValue) {
    let prev = headers.insert(name, value);
    assert!(prev.is_none());
}
#[allow(clippy::single_call_fn)] // extracted to keep block_on loop hot path simple and reusable
const fn is_block_on_poll_limit_reached(poll_count: usize) -> bool {
    poll_count >= MAX_BLOCK_ON_POLLS
}
#[allow(clippy::single_call_fn)] // keeps poll-count mutation centralized so block_on loop stays focused on state transitions
fn incr_block_on_poll_count(poll_count: &mut usize) {
    *poll_count = poll_count.saturating_add(1);
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
                incr_block_on_poll_count(&mut poll_count);
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
#[allow(clippy::single_call_fn)] // shared helper keeps variant-mapping panic behavior consistent for owned and borrowed paths
fn map_or_panic_unexpected_variant<R>(map_res: Option<R>, exp_id: &'static str) -> R {
    map_res.unwrap_or_else(|| panic_unexpected_variant(exp_id))
}
#[track_caller]
pub(crate) fn expect_variant<T, R>(
    v: T,
    map: impl FnOnce(T) -> Option<R>,
    exp_id: &'static str,
) -> R {
    map_or_panic_unexpected_variant(map(v), exp_id)
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper centralizes borrowed-variant extraction with consistent panic path across test assertions
pub(crate) fn expect_variant_ref<T, R>(
    v: &T,
    map: impl FnOnce(&T) -> Option<R>,
    exp_id: &'static str,
) -> R {
    map_or_panic_unexpected_variant(map(v), exp_id)
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared panic formatting keeps expectation failures consistent across helpers
fn panic_unexpected_result(
    er_id: &'static str,
    fn_name: &'static str,
    expected: &'static str,
    exp_id: &'static str,
) -> ! {
    panic!("{er_id} unexpected {expected} for {fn_name}, id={exp_id}");
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared panic builder keeps explicit UUID-tagged panic path reusable for header-replace preconditions
fn panic_replace_header_missing_src(exp_id: &'static str) -> ! {
    panic!("{REPLACE_HEADER_MISSING_SRC_ER_ID} missing source header while replacing, id={exp_id}");
}
#[track_caller]
pub(crate) fn expect_ok<T, E>(v: Result<T, E>, exp_id: &'static str) -> T {
    v.unwrap_or_else(|_| panic_unexpected_result(EXPECT_OK_ER_ID, "expect_ok", "Err", exp_id))
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper keeps ok-result equality assertions concise and consistent across validator tests
pub(crate) fn assert_ok_eq<T, E>(v: Result<T, E>, exp_id: &'static str, expected: &T)
where
    T: PartialEq + Debug,
{
    assert_eq!(&expect_ok(v, exp_id), expected);
}
#[track_caller]
pub(crate) fn expect_er<T, E>(v: Result<T, E>, exp_id: &'static str) -> E {
    v.err()
        .unwrap_or_else(|| panic_unexpected_result(EXPECT_ER_ER_ID, "expect_er", "Ok", exp_id))
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared mapper avoids repeating expect_er + variant extraction boilerplate in tests
pub(crate) fn expect_er_mapped<T, E, R>(
    v: Result<T, E>,
    exp_id: &'static str,
    map: impl FnOnce(E, &'static str) -> R,
) -> R {
    map(expect_er(v, exp_id), exp_id)
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper composes result extraction with variant mapping for concise validator tests
pub(crate) fn expect_er_variant<T, E, R>(
    v: Result<T, E>,
    exp_id: &'static str,
    map: impl FnOnce(E) -> Option<R>,
) -> R {
    expect_er_mapped(v, exp_id, |er, mapped_exp_id| {
        expect_variant(er, map, mapped_exp_id)
    })
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper supports variant extraction without moving the error value in tests
pub(crate) fn expect_er_variant_ref<T, E, R>(
    v: Result<T, E>,
    exp_id: &'static str,
    map: impl FnOnce(&E) -> Option<R>,
) -> R {
    expect_er_mapped(v, exp_id, |er, mapped_exp_id| {
        expect_variant_ref(&er, map, mapped_exp_id)
    })
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper composes status-code assertion with custom mapping to reduce repetition in variant helpers
fn map_err_after_status_check<T, E, R>(
    v: Result<T, E>,
    exp_id: &'static str,
    expected: StatusCode,
    map: impl FnOnce(E, &'static str) -> R,
) -> R
where
    E: GetAxumHttpStatusCode,
{
    map(assert_err_status_code(v, exp_id, expected), exp_id)
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
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper composes status-code assertion with variant mapping to reduce repetitive test boilerplate
pub(crate) fn assert_err_status_code_variant<T, E, R>(
    v: Result<T, E>,
    exp_id: &'static str,
    expected: StatusCode,
    map: impl FnOnce(E) -> Option<R>,
) -> R
where
    E: GetAxumHttpStatusCode,
{
    map_err_after_status_check(v, exp_id, expected, |er, mapped_exp_id| {
        expect_variant(er, map, mapped_exp_id)
    })
}
#[track_caller]
#[allow(clippy::single_call_fn)] // shared helper supports status+variant assertions while borrowing the error for field reads
pub(crate) fn assert_err_status_code_variant_ref<T, E, R>(
    v: Result<T, E>,
    exp_id: &'static str,
    expected: StatusCode,
    map: impl FnOnce(&E) -> Option<R>,
) -> R
where
    E: GetAxumHttpStatusCode,
{
    map_err_after_status_check(v, exp_id, expected, |er, mapped_exp_id| {
        expect_variant_ref(&er, map, mapped_exp_id)
    })
}
pub(crate) fn mk_headers_with_entry(name: impl IntoHeaderName, value: HeaderValue) -> HeaderMap {
    let mut headers = HeaderMap::new();
    insert_header_no_prev(&mut headers, name, value);
    headers
}
#[track_caller]
pub(crate) fn replace_header_name(
    headers: &mut HeaderMap,
    from_name: impl AsHeaderName,
    to_name: impl IntoHeaderName,
    exp_id: &'static str,
) {
    let value = headers
        .remove(from_name)
        .unwrap_or_else(|| panic_replace_header_missing_src(exp_id));
    insert_header_no_prev(headers, to_name, value);
}
pub(crate) fn non_utf8_header_value() -> HeaderValue {
    HeaderValue::from_bytes(&[0x80]).expect("86eb20cf")
}
#[track_caller]
pub(crate) fn assert_panics(action: impl FnOnce() + UnwindSafe, exp_id: &'static str) {
    let panic_res = catch_unwind(action);
    drop(panic_res.expect_err(exp_id));
}
#[cfg(test)]
mod tests {
    use super::{
        assert_err_status_code, assert_err_status_code_only, assert_err_status_code_variant,
        assert_err_status_code_variant_ref, assert_ok_eq, assert_panics, block_on, expect_er,
        expect_er_mapped, expect_er_variant, expect_er_variant_ref, expect_ok, expect_variant,
        expect_variant_ref, mk_headers_with_entry, non_utf8_header_value, panic_unexpected_variant,
    };
    use axum::http::{StatusCode, header::HeaderName, header::HeaderValue};
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
    fn poll_count_increment_helper_increments_once() {
        let mut poll_count = 0usize;
        super::incr_block_on_poll_count(&mut poll_count);
        assert_eq!(poll_count, 1usize);
    }
    #[test]
    fn expect_ok_returns_inner_value() {
        let v = expect_ok::<u8, u16>(Ok(7), "4f607799");
        assert_eq!(v, 7);
    }
    #[test]
    fn assert_ok_eq_checks_ok_result_value() {
        assert_ok_eq::<u8, u16>(Ok(7), "9665f80a", &7);
    }
    #[test]
    fn expect_er_returns_inner_error() {
        let v = expect_er::<u8, u16>(Err(9), "5cd39e4b");
        assert_eq!(v, 9);
    }
    #[test]
    fn expect_er_mapped_passes_error_and_exp_id_to_mapper() {
        let v =
            expect_er_mapped::<u8, u16, (u16, &'static str)>(Err(9), "8ce7a316", |er, exp_id| {
                (er, exp_id)
            });
        assert_eq!(v, (9, "8ce7a316"));
    }
    #[test]
    fn panic_unexpected_variant_always_panics() {
        assert_panics(|| panic_unexpected_variant("f66647ab"), "b6dba95d");
    }
    #[test]
    fn expect_variant_returns_mapped_value_for_matching_variant() {
        let v = expect_variant(Some(7u8), |v| v, "0dfd9a91");
        assert_eq!(v, 7);
    }
    #[test]
    fn expect_variant_ref_returns_mapped_value_for_matching_variant() {
        let value = Some(7u8);
        let v = expect_variant_ref(&value, |v| *v, "a2fcbad4");
        assert_eq!(v, 7);
    }
    #[test]
    fn expect_variant_panics_for_unexpected_variant() {
        assert_panics(
            || {
                let _: u8 = expect_variant::<Option<u8>, u8>(None, |v| v, "dba097b9");
            },
            "a9651f69",
        );
    }
    #[test]
    fn expect_er_variant_maps_matching_error_variant() {
        #[derive(Debug)]
        enum TestEr {
            A(u8),
        }
        let v = expect_er_variant::<(), TestEr, u8>(Err(TestEr::A(3)), "9bf4ce17", |er| match er {
            TestEr::A(v) => Some(v),
        });
        assert_eq!(v, 3);
    }
    #[test]
    fn expect_er_variant_ref_maps_matching_error_variant_without_move() {
        #[derive(Debug)]
        enum TestEr {
            A(u8),
        }
        let v =
            expect_er_variant_ref::<(), TestEr, u8>(Err(TestEr::A(3)), "8dfc4389", |er| match er {
                TestEr::A(v) => Some(*v),
            });
        assert_eq!(v, 3);
    }
    #[test]
    fn assert_err_status_code_variant_checks_status_and_extracts_variant() {
        #[derive(Debug)]
        enum TestEr {
            A,
        }
        impl super::GetAxumHttpStatusCode for TestEr {
            const AXUM_HTTP_STATUS_CODE: StatusCode = StatusCode::BAD_REQUEST;
        }
        let _: () = assert_err_status_code_variant::<(), TestEr, ()>(
            Err(TestEr::A),
            "c1d74a8e",
            StatusCode::BAD_REQUEST,
            |er| match er {
                TestEr::A => Some(()),
            },
        );
    }
    #[test]
    fn assert_err_status_code_variant_ref_checks_status_and_extracts_variant_without_move() {
        #[derive(Debug)]
        enum TestEr {
            A(u8),
        }
        impl super::GetAxumHttpStatusCode for TestEr {
            const AXUM_HTTP_STATUS_CODE: StatusCode = StatusCode::BAD_REQUEST;
        }
        let v = assert_err_status_code_variant_ref::<(), TestEr, u8>(
            Err(TestEr::A(7)),
            "8afb4ffd",
            StatusCode::BAD_REQUEST,
            |er| match er {
                TestEr::A(v) => Some(*v),
            },
        );
        assert_eq!(v, 7);
    }
    #[test]
    fn mk_headers_with_entry_inserts_value_for_case_insensitive_name() {
        let headers = mk_headers_with_entry("Commit", HeaderValue::from_static("deadbeef"));
        let actual = headers.get("commit");
        assert_eq!(actual, Some(&HeaderValue::from_static("deadbeef")));
    }
    #[test]
    fn replace_header_name_moves_value_to_new_key() {
        let mut headers = mk_headers_with_entry("x-commit", HeaderValue::from_static("deadbeef"));
        super::replace_header_name(
            &mut headers,
            "x-commit",
            HeaderName::from_static("commit"),
            "348c0e57",
        );
        assert!(headers.get("x-commit").is_none());
        assert_eq!(
            headers.get("commit"),
            Some(&HeaderValue::from_static("deadbeef"))
        );
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
