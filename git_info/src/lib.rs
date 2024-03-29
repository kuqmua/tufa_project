#[derive(
    Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Eq, Hash, PartialEq, Default,
)]
pub struct ProjectGitInfo<'a> {
    pub commit: &'a str,
}

pub const PROJECT_GIT_INFO: ProjectGitInfo = compile_time_git_info::compile_time_project_git_info!();

// pub const PROJECT_GIT_INFO: crate::common::git::project_git_info::ProjectGitInfo =
//     crate::common::git::project_git_info::ProjectGitInfo {
//         commit: "moch",
//     };
