#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
)]
pub enum CheckCommitErrorNamed {
    CommitNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitToStrConversion {
        #[eo_display]
        commit_to_str_conversion: axum::http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl axum::response::IntoResponse for CheckCommitErrorNamed {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            Self::CommitNotEqual {
                commit_not_equal: _,
                commit_to_use: _,
                code_occurence: _,
            } => axum::http::StatusCode::BAD_REQUEST,
            Self::CommitToStrConversion {
                commit_to_str_conversion: _,
                code_occurence: _,
            } => axum::http::StatusCode::BAD_REQUEST,
            Self::NoCommitHeader {
                no_commit_header: _,
                code_occurence: _,
            } => axum::http::StatusCode::BAD_REQUEST,
        };
        let mut res = axum::Json(self.into_serialize_deserialize_version()).into_response(); 
        *res.status_mut() = status_code;
        res
    }
}

pub fn check_commit(
    app_state: &dyn config_lib::config_fields::GetEnableApiGitCommitCheck,
    headers: &axum::http::HeaderMap<axum::http::header::HeaderValue>,
) -> Result<(), CheckCommitErrorNamed> {
    match app_state.get_enable_api_git_commit_check() {
        true => match headers.get(<naming_constants::Commit as naming_constants::Naming>::snake_case_stringified()) {
            Some(value) => match value.to_str() {
                Ok(value) => match value == git_info::PROJECT_GIT_INFO.commit {
                    true => Ok(()),
                    false => Err(CheckCommitErrorNamed::CommitNotEqual {
                        commit_not_equal: std::string::String::from("different project commit provided, services must work only with equal project commits"),
                        commit_to_use: crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(&git_info::PROJECT_GIT_INFO),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    })
                }
                Err(e) => Err(CheckCommitErrorNamed::CommitToStrConversion {
                    commit_to_str_conversion: e,
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            }
            None => Err(CheckCommitErrorNamed::NoCommitHeader {
                no_commit_header: std::string::String::from("no_commit_header"),
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
        false => Ok(())
    }
}
