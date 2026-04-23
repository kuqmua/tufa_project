use crate::hdr_val::get_required_header_str_parsed;
#[cfg(test)]
use crate::test_hlp::panic_unexpected_variant;
use axum::http::{
    HeaderMap, StatusCode,
    header::{HeaderName, ToStrError},
};
use git_info::validate_project_commit;
use loc_lib::{Location, loc, loc::Loc};
use optml::Optml;
use thiserror::Error;
const COMMIT_HEADER_NAME: HeaderName = HeaderName::from_static("commit");
const NO_COMMIT_HEADER_MSG: &str = "no_commit_header";
const COMMIT_NOT_EQ_MSG: &str =
    "different project commit provided, services must work only with eq project commits";
#[derive(Debug, Error, Location, Optml)]
pub enum CommitEr {
    CommitNotEq {
        #[eo_to_err_string]
        commit_not_eq: &'static str,
        #[eo_to_err_string]
        commit_to_use: &'static str,
        loc: Loc,
    },
    CommitToStrConversion {
        loc: Loc,
        #[eo_to_err_string]
        commit_to_str_conversion: ToStrError,
    },
    NoCommitHeader {
        #[eo_to_err_string]
        no_commit_header: &'static str,
        loc: Loc,
    },
}
impl crate::GetAxumHttpStatusCode for CommitEr {
    const AXUM_HTTP_STATUS_CODE: StatusCode = StatusCode::BAD_REQUEST;
}
impl CommitEr {
    #[allow(clippy::single_call_fn)] // keeps mismatch error construction reusable and explicit
    fn commit_not_eq(commit_to_use: &'static str) -> Self {
        Self::CommitNotEq {
            commit_not_eq: COMMIT_NOT_EQ_MSG,
            commit_to_use,
            loc: loc!(),
        }
    }
    #[allow(clippy::single_call_fn)] // keeps header to-str conversion error construction reusable
    fn commit_to_str_conversion(commit_to_str_conversion: ToStrError) -> Self {
        Self::CommitToStrConversion {
            commit_to_str_conversion,
            loc: loc!(),
        }
    }
    #[cfg(test)]
    fn expect_commit_not_eq(self, exp_id: &'static str) -> (&'static str, &'static str) {
        match self {
            Self::CommitNotEq {
                commit_not_eq,
                commit_to_use,
                ..
            } => (commit_not_eq, commit_to_use),
            Self::CommitToStrConversion { .. } | Self::NoCommitHeader { .. } => {
                panic_unexpected_variant(exp_id)
            }
        }
    }
    #[cfg(test)]
    fn expect_commit_to_str_conversion(self, exp_id: &'static str) {
        match self {
            Self::CommitToStrConversion { .. } => {}
            Self::CommitNotEq { .. } | Self::NoCommitHeader { .. } => {
                panic_unexpected_variant(exp_id)
            }
        }
    }
    #[cfg(test)]
    fn expect_no_commit_header(self, exp_id: &'static str) -> &'static str {
        match self {
            Self::NoCommitHeader {
                no_commit_header, ..
            } => no_commit_header,
            Self::CommitNotEq { .. } | Self::CommitToStrConversion { .. } => {
                panic_unexpected_variant(exp_id)
            }
        }
    }
    #[allow(clippy::single_call_fn)] // keeps missing-commit-header error construction reusable
    fn no_commit_header() -> Self {
        Self::NoCommitHeader {
            no_commit_header: NO_COMMIT_HEADER_MSG,
            loc: loc!(),
        }
    }
}
#[allow(clippy::single_call_fn)] // separates commit-value validation from header parsing for reuse and focused tests
fn validate_commit_header_value(commit: &str) -> Result<(), CommitEr> {
    validate_project_commit(commit).map_err(CommitEr::commit_not_eq)
}
pub fn check_commit(
    enable_api_git_commit_check: bool,
    headers: &HeaderMap,
) -> Result<(), CommitEr> {
    if !enable_api_git_commit_check {
        return Ok(());
    }
    get_required_header_str_parsed(
        headers,
        COMMIT_HEADER_NAME,
        CommitEr::no_commit_header,
        CommitEr::commit_to_str_conversion,
        validate_commit_header_value,
    )
}
#[cfg(test)]
mod tests {
    use super::{
        COMMIT_HEADER_NAME, COMMIT_NOT_EQ_MSG, CommitEr, NO_COMMIT_HEADER_MSG, check_commit,
        validate_commit_header_value,
    };
    use crate::test_hlp::{
        assert_err_status_code_only, expect_er, expect_ok, mk_headers_with_entry,
        non_utf8_header_value,
    };
    use axum::http::{HeaderMap, StatusCode, header::HeaderValue};
    use git_info::{PROJECT_GIT_INFO, is_project_commit, project_git_commit_link_ref};
    const WRONG_COMMIT: &str = "deadbeef";
    fn check_commit_enabled(headers: &HeaderMap) -> Result<(), CommitEr> {
        check_commit(true, headers)
    }
    fn mk_headers_with_commit_header_value(value: HeaderValue) -> HeaderMap {
        mk_headers_with_entry(COMMIT_HEADER_NAME, value)
    }
    fn mk_headers_with_commit(commit: &str) -> HeaderMap {
        mk_headers_with_commit_header_value(HeaderValue::from_str(commit).expect("9f2db59c"))
    }
    fn mk_headers_with_wrong_commit() -> HeaderMap {
        mk_headers_with_commit(WRONG_COMMIT)
    }
    fn mk_headers_with_project_commit() -> HeaderMap {
        mk_headers_with_commit(PROJECT_GIT_INFO.commit)
    }
    fn check_commit_ok(
        enable_api_git_commit_check: bool,
        headers: &HeaderMap,
        exp_id: &'static str,
    ) {
        expect_ok(check_commit(enable_api_git_commit_check, headers), exp_id);
    }
    fn check_commit_enabled_ok(headers: &HeaderMap, exp_id: &'static str) {
        check_commit_ok(true, headers, exp_id);
    }
    fn check_commit_bad_request(headers: &HeaderMap, exp_id: &'static str) {
        assert_err_status_code_only(
            check_commit_enabled(headers),
            exp_id,
            StatusCode::BAD_REQUEST,
        );
    }
    fn expect_commit_er(headers: &HeaderMap, exp_id: &'static str) -> CommitEr {
        expect_er(check_commit_enabled(headers), exp_id)
    }
    fn expect_no_commit_header_err(headers: &HeaderMap, exp_id: &'static str) -> &'static str {
        expect_commit_er(headers, exp_id).expect_no_commit_header(exp_id)
    }
    fn expect_commit_to_str_conversion_err(headers: &HeaderMap, exp_id: &'static str) {
        expect_commit_er(headers, exp_id).expect_commit_to_str_conversion(exp_id);
    }
    fn expect_commit_not_eq_err(
        headers: &HeaderMap,
        exp_id: &'static str,
    ) -> (&'static str, &'static str) {
        expect_commit_er(headers, exp_id).expect_commit_not_eq(exp_id)
    }
    #[test]
    fn check_commit_is_skipped_when_validation_is_disabled() {
        let headers = HeaderMap::new();
        check_commit_ok(false, &headers, "f4cab210");
    }
    #[test]
    fn check_commit_skip_mode_ignores_non_utf8_commit_header() {
        let headers = mk_headers_with_commit_header_value(non_utf8_header_value());
        check_commit_ok(false, &headers, "2f2a7b69");
    }
    #[test]
    fn check_commit_returns_no_header_error_when_header_is_absent() {
        let headers = HeaderMap::new();
        let no_commit_header = expect_no_commit_header_err(&headers, "c89f19a5");
        assert_eq!(no_commit_header, NO_COMMIT_HEADER_MSG);
    }
    #[test]
    fn check_commit_returns_to_str_error_for_non_utf8_header() {
        let headers = mk_headers_with_commit_header_value(non_utf8_header_value());
        expect_commit_to_str_conversion_err(&headers, "7b9ac2e3");
    }
    #[test]
    fn check_commit_returns_mismatch_error_for_wrong_commit() {
        let headers = mk_headers_with_wrong_commit();
        let (commit_not_eq, commit_to_use) = expect_commit_not_eq_err(&headers, "14f304d8");
        assert_eq!(commit_not_eq, COMMIT_NOT_EQ_MSG);
        assert!(commit_to_use.contains(PROJECT_GIT_INFO.commit));
    }
    #[test]
    fn validate_commit_header_value_returns_mismatch_for_wrong_commit() {
        let er = validate_commit_header_value(WRONG_COMMIT).expect_err("6804382f");
        let (commit_not_eq, commit_to_use) = er.expect_commit_not_eq("15f86081");
        assert_eq!(commit_not_eq, COMMIT_NOT_EQ_MSG);
        assert_eq!(commit_to_use, project_git_commit_link_ref());
    }
    #[test]
    fn validate_commit_header_value_accepts_project_commit() {
        expect_ok(
            validate_commit_header_value(PROJECT_GIT_INFO.commit),
            "5ef927d2",
        );
    }
    #[test]
    fn check_commit_returns_expected_commit_link_for_wrong_commit() {
        let headers = mk_headers_with_wrong_commit();
        let (_, commit_to_use) = expect_commit_not_eq_err(&headers, "3db98d20");
        assert_eq!(commit_to_use, project_git_commit_link_ref());
    }
    #[test]
    fn check_commit_treats_empty_commit_as_mismatch() {
        let headers = mk_headers_with_commit("");
        let (commit_not_eq, commit_to_use) = expect_commit_not_eq_err(&headers, "491ef4d6");
        assert_eq!(commit_not_eq, COMMIT_NOT_EQ_MSG);
        assert_eq!(commit_to_use, project_git_commit_link_ref());
    }
    #[test]
    fn check_commit_accepts_header_name_with_different_case() {
        let mut headers = mk_headers_with_project_commit();
        let commit = headers.remove(COMMIT_HEADER_NAME).expect("12653c9a");
        let prev = headers.insert("Commit", commit);
        assert!(prev.is_none());
        check_commit_enabled_ok(&headers, "bb6c239e");
    }
    #[test]
    fn check_commit_returns_ok_for_matching_commit() {
        let headers = mk_headers_with_project_commit();
        check_commit_enabled_ok(&headers, "c95e27d1");
    }
    #[test]
    fn project_commit_is_recognized_by_git_info_helper() {
        assert!(is_project_commit(PROJECT_GIT_INFO.commit));
    }
    #[test]
    fn non_project_commit_is_rejected_by_git_info_helper() {
        assert!(!is_project_commit(WRONG_COMMIT));
    }
    #[test]
    fn commit_errors_have_bad_request_status_code() {
        let headers = HeaderMap::new();
        let no_header_msg = expect_no_commit_header_err(&headers, "76314db5");
        assert_eq!(no_header_msg, NO_COMMIT_HEADER_MSG);
        check_commit_bad_request(&headers, "f39bdcc6");
        let non_utf8_headers = mk_headers_with_commit_header_value(non_utf8_header_value());
        expect_commit_to_str_conversion_err(&non_utf8_headers, "e1c2d84a");
        check_commit_bad_request(&non_utf8_headers, "2e86aa15");
        let mismatch_headers = mk_headers_with_wrong_commit();
        check_commit_bad_request(&mismatch_headers, "1cabe205");
    }
}
