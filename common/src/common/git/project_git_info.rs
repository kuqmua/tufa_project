#[derive(
    Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Eq, Hash, PartialEq, Default,
)]
pub struct ProjectGitInfo<'a> {
    pub commit: &'a str,
}

impl<'a> error_occurence_lib::git_fields::GetGitCommitId for ProjectGitInfo<'a> {
    fn get_git_commit_id(&self) -> std::string::String {
        self.commit.to_string()
    }
}

impl<'a> crate::common::git::get_git_commit_link::GetGitCommitLink for ProjectGitInfo<'a> {
    fn get_git_commit_link(&self) -> std::string::String {
        format!(
            "https://github.com/kuqmua/tufa_project/tree/{}", //todo get git_name from .git directory
            self.commit,
        )
    }
}