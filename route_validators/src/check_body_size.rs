use axum::{
    Error as AxumEr,
    body::{Body, HttpBody, to_bytes},
    http::StatusCode,
};
use bytes::Bytes;
use http_body::SizeHint;
use loc_lib::{Location, loc, loc::Loc};
use optml::Optml;
use thiserror::Error;
#[derive(Debug, Error, Location, Optml)]
pub enum BodySizeEr {
    ReachedMaximumSizeOfBody {
        #[eo_to_err_string]
        er: AxumEr,
        #[eo_to_err_string_serde]
        maximum_size_of_body_limit_in_bytes: usize,
        #[eo_to_err_string]
        size_hint: SizeHint,
        loc: Loc,
    },
}
impl crate::GetAxumHttpStatusCode for BodySizeEr {
    const AXUM_HTTP_STATUS_CODE: StatusCode = StatusCode::PAYLOAD_TOO_LARGE;
}
impl BodySizeEr {
    #[cfg(test)]
    fn expect_reached_maximum_size(self) -> (usize, Option<u64>) {
        match self {
            Self::ReachedMaximumSizeOfBody {
                maximum_size_of_body_limit_in_bytes,
                size_hint,
                ..
            } => (maximum_size_of_body_limit_in_bytes, size_hint.upper()),
        }
    }
    #[allow(clippy::single_call_fn)] // keeps body-size error construction reusable and testable in one place
    fn reached_maximum_size_of_body(
        er: AxumEr,
        maximum_size_of_body_limit_in_bytes: usize,
        size_hint: SizeHint,
    ) -> Self {
        Self::ReachedMaximumSizeOfBody {
            er,
            maximum_size_of_body_limit_in_bytes,
            size_hint,
            loc: loc!(),
        }
    }
}
#[allow(clippy::single_call_fn)] // keeps conversion from axum body error to domain error reusable for future body validators
fn map_body_size_er(er: AxumEr, limit: usize, size_hint: SizeHint) -> BodySizeEr {
    BodySizeEr::reached_maximum_size_of_body(er, limit, size_hint)
}
pub async fn check_body_size(body: Body, limit: usize) -> Result<Bytes, BodySizeEr> {
    let size_hint = HttpBody::size_hint(&body);
    to_bytes(body, limit)
        .await
        .map_err(|er: AxumEr| map_body_size_er(er, limit, size_hint))
}
#[cfg(test)]
mod tests {
    use super::check_body_size;
    use crate::test_hlp::{assert_err_status_code_only, block_on, expect_er, expect_ok};
    use axum::{body::Body, http::StatusCode};
    use bytes::Bytes;
    fn check_body_size_ok(body: Body, limit: usize, exp_id: &'static str) -> Bytes {
        expect_ok(block_on(check_body_size(body, limit)), exp_id)
    }
    fn expect_reached_max_size(
        body: Body,
        limit: usize,
        exp_id: &'static str,
    ) -> (usize, Option<u64>) {
        expect_er(block_on(check_body_size(body, limit)), exp_id).expect_reached_maximum_size()
    }
    fn assert_reached_max_size_limit(body: Body, limit: usize, exp_id: &'static str) {
        let (maximum_size_of_body_limit_in_bytes, _) = expect_reached_max_size(body, limit, exp_id);
        assert_eq!(maximum_size_of_body_limit_in_bytes, limit);
    }
    #[test]
    fn check_body_size_returns_bytes_when_body_fits_limit() {
        let bytes = check_body_size_ok(Body::from("ok"), 8, "2fb3e958");
        assert_eq!(bytes, "ok");
    }
    #[test]
    fn check_body_size_returns_bytes_when_size_eq_limit() {
        let bytes = check_body_size_ok(Body::from("ok"), 2, "1736f4db");
        assert_eq!(bytes, "ok");
    }
    #[test]
    fn check_body_size_returns_bytes_for_empty_body_with_zero_limit() {
        let bytes = check_body_size_ok(Body::empty(), 0, "44c8ad59");
        assert!(bytes.is_empty());
    }
    #[test]
    fn check_body_size_returns_error_when_body_exceeds_limit() {
        assert_reached_max_size_limit(Body::from("oversized"), 2, "ddf0983a");
    }
    #[test]
    fn check_body_size_returns_error_when_body_not_empty_and_limit_is_zero() {
        assert_reached_max_size_limit(Body::from("x"), 0, "7da3cae4");
    }
    #[test]
    fn check_body_size_error_contains_expected_non_zero_size_hint_for_static_body() {
        let (_, size_hint_upper) = expect_reached_max_size(Body::from("oversized"), 2, "cc0f2f3e");
        assert_eq!(size_hint_upper, Some(9));
        assert_eq!(size_hint_upper.map(|v| v > 0), Some(true));
    }
    #[test]
    fn body_size_error_maps_to_payload_too_large() {
        assert_err_status_code_only(
            block_on(check_body_size(Body::from("too-big"), 1)),
            "7ed49ba1",
            StatusCode::PAYLOAD_TOO_LARGE,
        );
    }
    #[test]
    fn body_size_error_keeps_limit_when_limit_is_one() {
        assert_reached_max_size_limit(Body::from("ab"), 1, "1fe7a3b4");
    }
}
