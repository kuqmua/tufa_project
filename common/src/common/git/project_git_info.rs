#[derive(
    Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Eq, Hash, PartialEq, Default,
)]
pub struct ProjectGitInfo<'a> {
    pub commit: &'a str,
}

impl<'a> error_occurence_lib::git_fields::GetGitCommitId for ProjectGitInfo<'a> {
    //todo
    fn get_git_commit_id(&self) -> std::string::String {
        self.commit.to_string()
    }
}
