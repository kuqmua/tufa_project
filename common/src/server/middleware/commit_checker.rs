pub type CommitCheckerAppState =
    std::sync::Arc<(dyn config_lib::config_fields::GetEnableApiGitCommitCheck + Send + Sync)>;

pub async fn commit_checker(
    //todo maybe check not wrapper repo commit id but instead create array with git modules repos ids what actually synced with current wrapper repo commit id
    axum::extract::State(app_state): axum::extract::State<CommitCheckerAppState>,
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, axum::response::Response> {
    match app_state.get_enable_api_git_commit_check() {
        true => match req
            .headers()
            .get(<naming_constants::Commit as naming_constants::Naming>::snake_case_stringified())
            .and_then(|header| header.to_str().ok())
        {
            Some(commit_checker_header) => match commit_checker_header
                == git_info::PROJECT_GIT_INFO
                    .commit
            {
                true => Ok(next.run(req).await),
                false => Err(axum::response::IntoResponse::into_response((
                    axum::http::StatusCode::BAD_REQUEST,
                    axum::Json(
                        crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed::CommitExtractorNotEqual {
                            commit_not_equal: std::string::String::from("different project commit provided, services must work only with equal project commits"),
                            commit_to_use: crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(&git_info::PROJECT_GIT_INFO),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        }.into_serialize_deserialize_version()
                    ),
                )))
            },
            None => Err(axum::response::IntoResponse::into_response((
                axum::http::StatusCode::BAD_REQUEST,
                axum::Json(
                    crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed::NoCommitExtractorHeader {
                        no_commit_header: std::string::String::from("commit header is not provided"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }.into_serialize_deserialize_version()
                ),
            )))
        },
        false => Ok(next.run(req).await),
    }
}

#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
)]
pub enum CommitExtractorCheckErrorNamed {
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn commit_checker_wrapper(
    app_state: CommitCheckerAppState,
    req: &axum::http::Request<axum::body::Body>,
) -> Result<(), CommitExtractorCheckErrorNamed> {
    match app_state.get_enable_api_git_commit_check() {
        true => match req
            .headers()
            .get(<naming_constants::Commit as naming_constants::Naming>::snake_case_stringified())
            .and_then(|header| header.to_str().ok())
        {
            Some(commit_checker_header) => match commit_checker_header == git_info::PROJECT_GIT_INFO.commit {
                true => Ok(()),
                false => Err(CommitExtractorCheckErrorNamed::CommitExtractorNotEqual {
                    commit_not_equal: std::string::String::from("different project commit provided, services must work only with equal project commits"),
                    commit_to_use: crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(&git_info::PROJECT_GIT_INFO),
                    code_occurence: error_occurence_lib::code_occurence!(),
                })
            },
            None => Err(CommitExtractorCheckErrorNamed::NoCommitExtractorHeader {
                no_commit_header: std::string::String::from("commit header is not provided"),
                code_occurence: error_occurence_lib::code_occurence!(),
            }),
        },
        false => Ok(()),
    }
}
