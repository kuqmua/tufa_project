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
            self.commit
        )
    }
}

//todo remove it 
impl<'a> error_occurence_lib::get_git_source_file_link::GetGitSourceFileLink<'a> for ProjectGitInfo<'a> {
    fn get_git_source_file_link(&self, file: &str, line: u32) -> std::string::String {
        format!("todo")
    }
}