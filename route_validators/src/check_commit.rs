use axum::http::{
    HeaderMap, StatusCode,
    header::{HeaderValue, ToStrError},
};
use git_info::{GetGitCommitLink, PROJECT_GIT_INFO};
use http_logic::GetAxumHttpStatusCode;
use loc_lib::{Location, loc, loc::Loc};
use naming::CommitSc;
use optml::Optml;
use thiserror::Error;
#[derive(Debug, Error, Location, Optml)]
pub enum CommitEr {
    CommitNotEq {
        #[eo_to_err_string_serde]
        commit_not_eq: String,
        #[eo_to_err_string_serde]
        commit_to_use: String,
        loc: Loc,
    },
    CommitToStrConversion {
        loc: Loc,
        #[eo_to_err_string]
        commit_to_str_conversion: ToStrError,
    },
    NoCommitHeader {
        #[eo_to_err_string_serde]
        no_commit_header: String,
        loc: Loc,
    },
}
impl GetAxumHttpStatusCode for CommitEr {
    fn get_axum_http_status_code(&self) -> StatusCode {
        match self {
            Self::CommitNotEq { .. }
            | Self::CommitToStrConversion { .. }
            | Self::NoCommitHeader { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
pub fn check_commit(
    // app_state: &dyn app_state::GetEnableApiGitCommitCheck,
    enable_api_git_commit_check: bool,
    headers: &HeaderMap<HeaderValue>,
) -> Result<(), CommitEr> {
    if
    // app_state.get_enable_api_git_commit_check()
    enable_api_git_commit_check {
        headers.get(CommitSc.to_string()).map_or_else(
            || {
                Err(CommitEr::NoCommitHeader {
                    no_commit_header: String::from("no_commit_header"),
                    loc: loc!(),
                })
            },
            |v_9c98ee60| match v_9c98ee60.to_str() {
                Ok(v_16408fd2) => {
                    if v_16408fd2 == PROJECT_GIT_INFO.commit {
                        Ok(())
                    } else {
                        Err(CommitEr::CommitNotEq {
                            commit_not_eq: String::from("different project commit provided, services must work only with eq project commits"),
                            commit_to_use: GetGitCommitLink::get_git_commit_link(&PROJECT_GIT_INFO),
                            loc: loc!(),
                        })
                    }
                }
                Err(er) => Err(CommitEr::CommitToStrConversion {
                    commit_to_str_conversion: er,
                    loc: loc!(),
                }),
            },
        )
    } else {
        Ok(())
    }
}
