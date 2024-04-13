pub type CommitCheckerAppState =
    std::sync::Arc<(dyn config_lib::config_fields::GetEnableApiGitCommitCheck + Send + Sync)>;

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
        commit_to_str_conversion: axum::http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

// pub async fn commit_checker_wrapper(
//     app_state: CommitCheckerAppState,
//     req: &axum::http::Request<axum::body::Body>,
// ) -> Result<(), CommitExtractorCheckErrorNamed> {
//     match app_state.get_enable_api_git_commit_check() {
//         true => match req
//             .headers()
//             .get(<naming_constants::Commit as naming_constants::Naming>::snake_case_stringified())
//             .and_then(|header| header.to_str().ok())
//         {
//             Some(commit_checker_header) => match commit_checker_header == git_info::PROJECT_GIT_INFO.commit {
//                 true => Ok(()),
//                 false => Err(CommitExtractorCheckErrorNamed::CommitExtractorNotEqual {
//                     commit_not_equal: std::string::String::from("different project commit provided, services must work only with equal project commits"),
//                     commit_to_use: crate::common::git::get_git_commit_link::GetGitCommitLink::get_git_commit_link(&git_info::PROJECT_GIT_INFO),
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 })
//             },
//             None => Err(CommitExtractorCheckErrorNamed::NoCommitExtractorHeader {
//                 no_commit_header: std::string::String::from("commit header is not provided"),
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             }),
//         },
//         false => Ok(()),
//     }
// }
