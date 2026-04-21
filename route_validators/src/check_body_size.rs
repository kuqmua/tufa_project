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
pub async fn check_body_size(body: Body, limit: usize) -> Result<Bytes, BodySizeEr> {
    let size_hint = HttpBody::size_hint(&body);
    to_bytes(body, limit)
        .await
        .map_err(|er| BodySizeEr::ReachedMaximumSizeOfBody {
            er,
            maximum_size_of_body_limit_in_bytes: limit,
            size_hint,
            loc: loc!(),
        })
}
#[cfg(test)]
mod tests {
    use super::{BodySizeEr, check_body_size};
    use crate::test_hlp::{assert_status_code, block_on, expect_er, expect_ok};
    use axum::{body::Body, http::StatusCode};
    use bytes::Bytes;
    fn get_limit_from_er(er: &BodySizeEr) -> usize {
        match er {
            BodySizeEr::ReachedMaximumSizeOfBody {
                maximum_size_of_body_limit_in_bytes,
                ..
            } => *maximum_size_of_body_limit_in_bytes,
        }
    }
    fn check_body_size_ok(body: Body, limit: usize, exp_id: &'static str) -> Bytes {
        expect_ok(block_on(check_body_size(body, limit)), exp_id)
    }
    fn check_body_size_er(body: Body, limit: usize, exp_id: &'static str) -> BodySizeEr {
        expect_er(block_on(check_body_size(body, limit)), exp_id)
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
        let er = check_body_size_er(Body::from("oversized"), 2, "ddf0983a");
        assert_eq!(get_limit_from_er(&er), 2);
    }
    #[test]
    fn check_body_size_returns_error_when_body_not_empty_and_limit_is_zero() {
        let er = check_body_size_er(Body::from("x"), 0, "7da3cae4");
        assert_eq!(get_limit_from_er(&er), 0);
    }
    #[test]
    fn body_size_error_maps_to_payload_too_large() {
        let er = check_body_size_er(Body::from("too-big"), 1, "7ed49ba1");
        assert_status_code(&er, StatusCode::PAYLOAD_TOO_LARGE);
    }
}
