pub use naming::GITHUB_URL;
use serde_derive::{Deserialize, Serialize};
pub const PROJECT_GIT_INFO: ProjectGitInfo<'_> =
    compile_time_git_info::compile_time_project_git_info!();
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Default)]
pub struct ProjectGitInfo<'commit_lifetime> {
    pub commit: &'commit_lifetime str,
}

// pub const PROJECT_GIT_INFO: crate::common::git::project_git_info::ProjectGitInfo =
//     crate::common::git::project_git_info::ProjectGitInfo {
//         commit: "moch",
//     };

pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> String;
}

impl<T> GetGitCommitLink for T
where
    T: GetGitCommitId, //todo wtf
{
    fn get_git_commit_link(&self) -> String {
        format!("{}/tree/{}", GITHUB_URL, self.get_git_commit_id())
    }
}

pub trait GetGitCommitId {
    //todo remove
    fn get_git_commit_id(&self) -> String;
}

// pub trait GetGitRepoLink {
//     fn get_git_repo_link(&self) -> String;
// }

// pub trait GetGitAuthor<'lifetime> {
//     fn get_git_author(&self) -> &'lifetime str;
// }

// pub trait GetGitAuthorEmail<'lifetime> {
//     fn get_git_author_email(&self) -> &'lifetime str;
// }

// pub trait GetGitCommitUnixTime<'lifetime> {
//     fn get_git_commit_unix_time(&self) -> &'lifetime str;
// }

// pub trait GetGitTimezone<'lifetime> {
//     fn get_git_timezone(&self) -> &'lifetime str;
// }

// pub trait GetGitMessage<'lifetime> {
//     fn get_git_message(&self) -> &'lifetime str;
// }

impl GetGitCommitId for ProjectGitInfo<'_> {
    //todo
    fn get_git_commit_id(&self) -> String {
        self.commit.to_owned()
    }
}
