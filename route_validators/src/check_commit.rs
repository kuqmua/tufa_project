#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CheckCommitErrorNamed {
    CommitNotEqual {
        #[eo_to_std_string_string_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitToStrConversion {
        #[eo_to_std_string_string]
        commit_to_str_conversion: axum::http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitHeader {
        #[eo_to_std_string_string_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl http_logic::GetAxumHttpStatusCode for CheckCommitErrorNamed {
    fn get_axum_http_status_code(&self) -> axum::http::StatusCode {
        match self {
            Self::CommitNotEqual { .. }
            | Self::CommitToStrConversion { .. }
            | Self::NoCommitHeader { .. } => axum::http::StatusCode::BAD_REQUEST,
        }
    }
}

pub fn check_commit(
    // app_state: &dyn app_state::GetEnableApiGitCommitCheck,
    enable_api_git_commit_check: bool,
    headers: &axum::http::HeaderMap<axum::http::header::HeaderValue>,
) -> Result<(), CheckCommitErrorNamed> {
    if
    // app_state.get_enable_api_git_commit_check()
    enable_api_git_commit_check {
        headers.get(naming::CommitSnakeCase.to_string()).map_or_else(
            || {
                Err(CheckCommitErrorNamed::NoCommitHeader {
                    no_commit_header: std::string::String::from("no_commit_header"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            },
            |value| match value.to_str() {
                Ok(value) => {
                    if value == git_info::PROJECT_GIT_INFO.commit {
                        Ok(())
                    } else {
                        Err(CheckCommitErrorNamed::CommitNotEqual {
                            commit_not_equal: std::string::String::from("different project commit provided, services must work only with equal project commits"),
                            commit_to_use: git_info::GetGitCommitLink::get_git_commit_link(&git_info::PROJECT_GIT_INFO),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        })
                    }
                }
                Err(error) => Err(CheckCommitErrorNamed::CommitToStrConversion {
                    commit_to_str_conversion: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                }),
            },
        )
    } else {
        Ok(())
    }
}
