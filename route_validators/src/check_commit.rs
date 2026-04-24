use crate::hdr_val::get_required_header_str;
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
#[allow(clippy::single_call_fn)] // dedicated extractor enables reuse in tests and keeps parsing separate from validation
fn get_commit_header_str(headers: &HeaderMap) -> Result<&str, CommitEr> {
    get_required_header_str(
        headers,
        COMMIT_HEADER_NAME,
        CommitEr::no_commit_header,
        CommitEr::commit_to_str_conversion,
    )
}
pub fn check_commit(
    enable_api_git_commit_check: bool,
    headers: &HeaderMap,
) -> Result<(), CommitEr> {
    if !enable_api_git_commit_check {
        return Ok(());
    }
    let commit = get_commit_header_str(headers)?;
    validate_commit_header_value(commit)
}
#[cfg(test)]
mod tests {
    use super::{
        COMMIT_HEADER_NAME, COMMIT_NOT_EQ_MSG, CommitEr, NO_COMMIT_HEADER_MSG, check_commit,
        get_commit_header_str, validate_commit_header_value,
    };
    use crate::test_hlp::{
        assert_err_status_code_only, assert_err_status_code_variant_ref, assert_ok_eq,
        expect_er_variant_ref, expect_ok, mk_headers_with_entry, non_utf8_header_value,
        replace_header_name,
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
    fn mk_headers_with_non_utf8_commit() -> HeaderMap {
        mk_headers_with_commit_header_value(non_utf8_header_value())
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
    #[allow(clippy::single_call_fn)] // shared extractor keeps NoCommitHeader variant matching reusable across tests
    fn no_commit_header_msg(v: &CommitEr) -> Option<&'static str> {
        match v {
            CommitEr::NoCommitHeader {
                no_commit_header, ..
            } => Some(*no_commit_header),
            CommitEr::CommitNotEq { .. } | CommitEr::CommitToStrConversion { .. } => None,
        }
    }
    #[allow(clippy::single_call_fn)] // shared extractor keeps CommitToStrConversion checks concise across tests
    fn is_commit_to_str_conversion(v: &CommitEr) -> Option<()> {
        match v {
            CommitEr::CommitToStrConversion { .. } => Some(()),
            CommitEr::CommitNotEq { .. } | CommitEr::NoCommitHeader { .. } => None,
        }
    }
    #[allow(clippy::single_call_fn)] // shared extractor centralizes CommitNotEq fields used by multiple assertions
    fn commit_not_eq_fields(v: &CommitEr) -> Option<(&'static str, &'static str)> {
        match v {
            CommitEr::CommitNotEq {
                commit_not_eq,
                commit_to_use,
                ..
            } => Some((*commit_not_eq, *commit_to_use)),
            CommitEr::CommitToStrConversion { .. } | CommitEr::NoCommitHeader { .. } => None,
        }
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps NoCommitHeader message checks coupled with variant extraction across tests
    fn assert_no_commit_header_err(headers: &HeaderMap, exp_id: &'static str) {
        let no_commit_header =
            expect_check_commit_err_variant(headers, exp_id, no_commit_header_msg);
        assert_eq!(no_commit_header, NO_COMMIT_HEADER_MSG);
    }
    fn expect_commit_to_str_conversion_err(headers: &HeaderMap, exp_id: &'static str) {
        expect_check_commit_err_variant(headers, exp_id, is_commit_to_str_conversion);
    }
    #[allow(clippy::single_call_fn)] // shared wrapper keeps CommitNotEq extraction reusable and explicit for mismatch assertions
    fn expect_commit_not_eq_err(
        headers: &HeaderMap,
        exp_id: &'static str,
    ) -> (&'static str, &'static str) {
        expect_check_commit_err_variant(headers, exp_id, commit_not_eq_fields)
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps CommitNotEq expectation consistent across mismatch tests
    fn assert_commit_not_eq_fields(
        fields: (&'static str, &'static str),
        exp_commit_not_eq: &'static str,
        exp_commit_to_use: &'static str,
    ) {
        let (commit_not_eq, commit_to_use) = fields;
        assert_eq!(commit_not_eq, exp_commit_not_eq);
        assert_eq!(commit_to_use, exp_commit_to_use);
    }
    #[allow(clippy::single_call_fn)] // shared wrapper keeps wrong-commit assertions concise and stable across tests
    fn assert_wrong_commit_fields(fields: (&'static str, &'static str)) {
        assert_commit_not_eq_fields(fields, COMMIT_NOT_EQ_MSG, project_git_commit_link_ref());
    }
    #[allow(clippy::single_call_fn)] // shared helper keeps wrong-commit check+assert flow reusable across mismatch tests
    fn assert_wrong_commit_err(headers: &HeaderMap, exp_id: &'static str) {
        let fields = expect_commit_not_eq_err(headers, exp_id);
        assert_wrong_commit_fields(fields);
    }
    #[allow(clippy::single_call_fn)] // shared assertion wrapper keeps commit-enabled error mapping reusable across variant-specific helpers
    fn expect_check_commit_err_variant<R>(
        headers: &HeaderMap,
        exp_id: &'static str,
        map: impl FnOnce(&CommitEr) -> Option<R>,
    ) -> R {
        expect_commit_result_err_variant(
            check_commit_enabled(headers),
            exp_id,
            Some(StatusCode::BAD_REQUEST),
            map,
        )
    }
    #[allow(clippy::single_call_fn)] // shared wrapper keeps get_commit_header_str error-variant assertions concise and consistent
    fn expect_get_commit_header_str_err_variant<R>(
        headers: &HeaderMap,
        exp_id: &'static str,
        map: impl FnOnce(&CommitEr) -> Option<R>,
    ) -> R {
        expect_commit_result_err_variant(get_commit_header_str(headers), exp_id, None, map)
    }
    #[allow(clippy::single_call_fn)] // generic helper keeps borrowed-variant assertions reusable for both status-checked and plain extraction paths
    fn expect_commit_result_err_variant<T, R>(
        v: Result<T, CommitEr>,
        exp_id: &'static str,
        expected_status: Option<StatusCode>,
        map: impl FnOnce(&CommitEr) -> Option<R>,
    ) -> R {
        match expected_status {
            Some(status_code) => assert_err_status_code_variant_ref(v, exp_id, status_code, map),
            None => expect_er_variant_ref(v, exp_id, map),
        }
    }
    #[test]
    fn check_commit_is_skipped_when_validation_is_disabled() {
        let headers = HeaderMap::new();
        check_commit_ok(false, &headers, "f4cab210");
    }
    #[test]
    fn check_commit_skip_mode_ignores_non_utf8_commit_header() {
        let headers = mk_headers_with_non_utf8_commit();
        check_commit_ok(false, &headers, "2f2a7b69");
    }
    #[test]
    fn check_commit_returns_no_header_error_when_header_is_absent() {
        let headers = HeaderMap::new();
        assert_no_commit_header_err(&headers, "c89f19a5");
    }
    #[test]
    fn check_commit_returns_to_str_error_for_non_utf8_header() {
        let headers = mk_headers_with_non_utf8_commit();
        expect_commit_to_str_conversion_err(&headers, "7b9ac2e3");
    }
    #[test]
    fn get_commit_header_str_returns_header_value_when_present() {
        let headers = mk_headers_with_project_commit();
        assert_ok_eq(
            get_commit_header_str(&headers),
            "e1d07f53",
            &PROJECT_GIT_INFO.commit,
        );
    }
    #[test]
    fn get_commit_header_str_returns_error_when_header_is_absent() {
        let headers = HeaderMap::new();
        let no_commit_header =
            expect_get_commit_header_str_err_variant(&headers, "72e4a18d", no_commit_header_msg);
        assert_eq!(no_commit_header, NO_COMMIT_HEADER_MSG);
    }
    #[test]
    fn get_commit_header_str_returns_error_for_non_utf8_header() {
        let headers = mk_headers_with_non_utf8_commit();
        expect_get_commit_header_str_err_variant(&headers, "6b4a128f", is_commit_to_str_conversion);
    }
    #[test]
    fn check_commit_returns_mismatch_error_for_wrong_commit() {
        let headers = mk_headers_with_wrong_commit();
        assert_wrong_commit_err(&headers, "14f304d8");
    }
    #[test]
    fn validate_commit_header_value_returns_mismatch_for_wrong_commit() {
        let fields = expect_er_variant_ref(
            validate_commit_header_value(WRONG_COMMIT),
            "6804382f",
            commit_not_eq_fields,
        );
        assert_wrong_commit_fields(fields);
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
        let fields = expect_er_variant_ref(
            check_commit_enabled(&headers),
            "3db98d20",
            commit_not_eq_fields,
        );
        assert_wrong_commit_fields(fields);
    }
    #[test]
    fn check_commit_treats_empty_commit_as_mismatch() {
        let headers = mk_headers_with_commit("");
        assert_wrong_commit_err(&headers, "491ef4d6");
    }
    #[test]
    fn check_commit_accepts_header_name_with_different_case() {
        let mut headers = mk_headers_with_project_commit();
        replace_header_name(&mut headers, COMMIT_HEADER_NAME, "Commit", "12653c9a");
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
        assert_no_commit_header_err(&headers, "76314db5");
        check_commit_bad_request(&headers, "f39bdcc6");
        let non_utf8_headers = mk_headers_with_non_utf8_commit();
        expect_commit_to_str_conversion_err(&non_utf8_headers, "e1c2d84a");
        check_commit_bad_request(&non_utf8_headers, "2e86aa15");
        let mismatch_headers = mk_headers_with_wrong_commit();
        check_commit_bad_request(&mismatch_headers, "1cabe205");
    }
}
