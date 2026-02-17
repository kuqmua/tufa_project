use axum::http::{
    HeaderMap, StatusCode,
    header::{HeaderValue, ToStrError},
};
use error_occurence_lib::{ErrorOccurence, code_occurence, code_occurence::CodeOccurence};
use git_info::{GetGitCommitLink, PROJECT_GIT_INFO};
use http_logic::GetAxumHttpStatusCode;
use naming::CommitSc;
use thiserror::Error;
#[derive(Debug, Error, ErrorOccurence)]
pub enum CommitError {
    CommitNotEqual {
        #[eo_to_err_string_serde]
        commit_not_equal: String,
        #[eo_to_err_string_serde]
        commit_to_use: String,
        code_occurence: CodeOccurence,
    },
    CommitToStrConversion {
        #[eo_to_err_string]
        commit_to_str_conversion: ToStrError,
        code_occurence: CodeOccurence,
    },
    NoCommitHeader {
        #[eo_to_err_string_serde]
        no_commit_header: String,
        code_occurence: CodeOccurence,
    },
}
impl GetAxumHttpStatusCode for CommitError {
    fn get_axum_http_status_code(&self) -> StatusCode {
        match self {
            Self::CommitNotEqual { .. }
            | Self::CommitToStrConversion { .. }
            | Self::NoCommitHeader { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
pub fn check_commit(
    // app_state: &dyn app_state::GetEnableApiGitCommitCheck,
    enable_api_git_commit_check: bool,
    headers: &HeaderMap<HeaderValue>,
) -> Result<(), CommitError> {
    if
    // app_state.get_enable_api_git_commit_check()
    enable_api_git_commit_check {
        headers.get(CommitSc.to_string()).map_or_else(
            || {
                Err(CommitError::NoCommitHeader {
                    no_commit_header: String::from("no_commit_header"),
                    code_occurence: code_occurence!(),
                })
            },
            |value_9c98ee60| match value_9c98ee60.to_str() {
                Ok(value_16408fd2) => {
                    if value_16408fd2 == PROJECT_GIT_INFO.commit {
                        Ok(())
                    } else {
                        Err(CommitError::CommitNotEqual {
                            commit_not_equal: String::from("different project commit provided, services must work only with equal project commits"),
                            commit_to_use: GetGitCommitLink::get_git_commit_link(&PROJECT_GIT_INFO),
                            code_occurence: code_occurence!(),
                        })
                    }
                }
                Err(error) => Err(CommitError::CommitToStrConversion {
                    commit_to_str_conversion: error,
                    code_occurence: code_occurence!(),
                }),
            },
        )
    } else {
        Ok(())
    }
}
