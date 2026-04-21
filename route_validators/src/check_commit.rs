use axum::http::{HeaderMap, StatusCode, header::ToStrError};
use git_info::{is_project_commit, project_git_commit_link};
use loc_lib::{Location, loc, loc::Loc};
use optml::Optml;
use thiserror::Error;
const COMMIT_HEADER_NAME: &str = "commit";
const NO_COMMIT_HEADER_MSG: &str = "no_commit_header";
const COMMIT_NOT_EQ_MSG: &str =
    "different project commit provided, services must work only with eq project commits";
#[derive(Debug, Error, Location, Optml)]
pub enum CommitEr {
    CommitNotEq {
        #[eo_to_err_string]
        commit_not_eq: &'static str,
        #[eo_to_err_string]
        commit_to_use: String,
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
pub fn check_commit(
    enable_api_git_commit_check: bool,
    headers: &HeaderMap,
) -> Result<(), CommitEr> {
    if !enable_api_git_commit_check {
        return Ok(());
    }
    let commit = headers
        .get(COMMIT_HEADER_NAME)
        .ok_or_else(|| CommitEr::NoCommitHeader {
            no_commit_header: NO_COMMIT_HEADER_MSG,
            loc: loc!(),
        })?
        .to_str()
        .map_err(|commit_to_str_conversion| CommitEr::CommitToStrConversion {
            commit_to_str_conversion,
            loc: loc!(),
        })?;
    if is_project_commit(commit) {
        return Ok(());
    }
    Err(CommitEr::CommitNotEq {
        commit_not_eq: COMMIT_NOT_EQ_MSG,
        commit_to_use: project_git_commit_link(),
        loc: loc!(),
    })
}
#[cfg(test)]
mod tests {
    use super::{
        COMMIT_HEADER_NAME, COMMIT_NOT_EQ_MSG, CommitEr, NO_COMMIT_HEADER_MSG, check_commit,
    };
    use crate::test_hlp::{assert_status_code, expect_er, expect_ok};
    use axum::http::{HeaderMap, StatusCode, header::HeaderValue};
    use git_info::{PROJECT_GIT_INFO, project_git_commit_link};
    fn mk_headers_with_commit(commit: HeaderValue) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let prev = headers.insert(COMMIT_HEADER_NAME, commit);
        assert!(prev.is_none());
        headers
    }
    fn mk_headers_with_wrong_commit() -> HeaderMap {
        mk_headers_with_commit(HeaderValue::from_static("deadbeef"))
    }
    fn mk_headers_with_project_commit() -> HeaderMap {
        mk_headers_with_commit(HeaderValue::from_str(PROJECT_GIT_INFO.commit).expect("0ad4f301"))
    }
    fn assert_is_bad_request(er: &CommitEr) {
        assert_status_code(er, StatusCode::BAD_REQUEST);
    }
    fn check_commit_ok(
        enable_api_git_commit_check: bool,
        headers: &HeaderMap,
        exp_id: &'static str,
    ) {
        expect_ok(check_commit(enable_api_git_commit_check, headers), exp_id);
    }
    fn check_commit_er(
        enable_api_git_commit_check: bool,
        headers: &HeaderMap,
        exp_id: &'static str,
    ) -> CommitEr {
        expect_er(check_commit(enable_api_git_commit_check, headers), exp_id)
    }
    fn check_commit_not_eq_er(headers: &HeaderMap, exp_id: &'static str) -> (&'static str, String) {
        let er = check_commit_er(true, headers, exp_id);
        match er {
            CommitEr::CommitNotEq {
                commit_not_eq,
                commit_to_use,
                ..
            } => (commit_not_eq, commit_to_use),
            CommitEr::CommitToStrConversion { .. } | CommitEr::NoCommitHeader { .. } => {
                panic!("7f8f24a9");
            }
        }
    }
    #[test]
    fn check_commit_is_skipped_when_validation_is_disabled() {
        let headers = HeaderMap::new();
        check_commit_ok(false, &headers, "f4cab210");
    }
    #[test]
    fn check_commit_skip_mode_ignores_non_utf8_commit_header() {
        let headers = mk_headers_with_commit(HeaderValue::from_bytes(&[0x80]).expect("46076c3e"));
        check_commit_ok(false, &headers, "2f2a7b69");
    }
    #[test]
    fn check_commit_returns_no_header_error_when_header_is_absent() {
        let headers = HeaderMap::new();
        let no_commit_header = match check_commit_er(true, &headers, "c89f19a5") {
            CommitEr::NoCommitHeader {
                no_commit_header, ..
            } => no_commit_header,
            CommitEr::CommitNotEq { .. } | CommitEr::CommitToStrConversion { .. } => {
                panic!("7932cda5");
            }
        };
        assert_eq!(no_commit_header, NO_COMMIT_HEADER_MSG);
    }
    #[test]
    fn check_commit_returns_to_str_error_for_non_utf8_header() {
        let headers = mk_headers_with_commit(HeaderValue::from_bytes(&[0x80]).expect("f77798f0"));
        let er = check_commit_er(true, &headers, "7b9ac2e3");
        match er {
            CommitEr::CommitToStrConversion { .. } => {}
            CommitEr::CommitNotEq { .. } | CommitEr::NoCommitHeader { .. } => {
                panic!("85df9706");
            }
        }
    }
    #[test]
    fn check_commit_returns_mismatch_error_for_wrong_commit() {
        let headers = mk_headers_with_wrong_commit();
        let (commit_not_eq, commit_to_use) = check_commit_not_eq_er(&headers, "14f304d8");
        assert_eq!(commit_not_eq, COMMIT_NOT_EQ_MSG);
        assert!(commit_to_use.contains(PROJECT_GIT_INFO.commit));
    }
    #[test]
    fn check_commit_returns_expected_commit_link_for_wrong_commit() {
        let headers = mk_headers_with_wrong_commit();
        let (_, commit_to_use) = check_commit_not_eq_er(&headers, "3db98d20");
        assert_eq!(commit_to_use, project_git_commit_link());
    }
    #[test]
    fn check_commit_accepts_header_name_with_different_case() {
        let mut headers = mk_headers_with_project_commit();
        let commit = headers.remove(COMMIT_HEADER_NAME).expect("12653c9a");
        let prev = headers.insert("Commit", commit);
        assert!(prev.is_none());
        check_commit_ok(true, &headers, "bb6c239e");
    }
    #[test]
    fn check_commit_returns_ok_for_matching_commit() {
        let headers = mk_headers_with_project_commit();
        check_commit_ok(true, &headers, "c95e27d1");
    }
    #[test]
    fn commit_errors_have_bad_request_status_code() {
        let no_header_er = check_commit_er(true, &HeaderMap::new(), "76314db5");
        assert_is_bad_request(&no_header_er);
        let headers = mk_headers_with_wrong_commit();
        let mismatch_er = check_commit_er(true, &headers, "1cabe205");
        assert_is_bad_request(&mismatch_er);
    }
}
