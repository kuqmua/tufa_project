pub use naming::GITHUB_URL;
use serde_derive::{Deserialize, Serialize};
pub const PROJECT_GIT_INFO: ProjectGitInfo<'_> =
    compile_time_git_info::compile_time_project_git_info!();
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Default)]
pub struct ProjectGitInfo<'commit_lifetime> {
    pub commit: &'commit_lifetime str,
}
pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> String;
}
impl<T> GetGitCommitLink for T
where
    T: GetGitCommitId,
{
    fn get_git_commit_link(&self) -> String {
        format!("{}/tree/{}", GITHUB_URL, self.get_git_commit_id())
    }
}
pub trait GetGitCommitId {
    fn get_git_commit_id(&self) -> String;
}
impl GetGitCommitId for ProjectGitInfo<'_> {
    fn get_git_commit_id(&self) -> String {
        self.commit.to_owned()
    }
}
