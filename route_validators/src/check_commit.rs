use axum::http::{
    HeaderMap, StatusCode,
    header::{HeaderValue, ToStrError},
};
use error_occurence_lib::code_occurence::CodeOccurence;
use http_logic::GetAxumHttpStatusCode;
use naming::CommitSc;
use thiserror::Error;

#[derive(Debug, Error, error_occurence_lib::ErrorOccurence)]
pub enum ErrorNamed {
    CommitNotEqual {
        #[eo_to_err_string_serialize_deserialize]
        commit_not_equal: String,
        #[eo_to_err_string_serialize_deserialize]
        commit_to_use: String,
        code_occurence: CodeOccurence,
    },
    CommitToStrConversion {
        #[eo_to_err_string]
        commit_to_str_conversion: ToStrError,
        code_occurence: CodeOccurence,
    },
    NoCommitHeader {
        #[eo_to_err_string_serialize_deserialize]
        no_commit_header: String,
        code_occurence: CodeOccurence,
    },
}

impl GetAxumHttpStatusCode for ErrorNamed {
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
) -> Result<(), ErrorNamed> {
    if
    // app_state.get_enable_api_git_commit_check()
    enable_api_git_commit_check {
        headers.get(CommitSc.to_string()).map_or_else(
            || {
                Err(ErrorNamed::NoCommitHeader {
                    no_commit_header: String::from("no_commit_header"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            },
            |value_9c98ee60| match value_9c98ee60.to_str() {
                Ok(value_16408fd2) => {
                    if value_16408fd2 == git_info::PROJECT_GIT_INFO.commit {
                        Ok(())
                    } else {
                        Err(ErrorNamed::CommitNotEqual {
                            commit_not_equal: String::from("different project commit provided, services must work only with equal project commits"),
                            commit_to_use: git_info::GetGitCommitLink::get_git_commit_link(&git_info::PROJECT_GIT_INFO),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
                Err(error) => Err(ErrorNamed::CommitToStrConversion {
                    commit_to_str_conversion: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                }),
            },
        )
    } else {
        Ok(())
    }
}
