pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> std::string::String;
}

impl<T> GetGitCommitLink for T
where
    T: error_occurence_lib::git_fields::GetGitCommitId,
{
    fn get_git_commit_link(&self) -> std::string::String {
        format!(
            "{}/tree/{}",
            naming_constants::GITHUB_URL, self.get_git_commit_id()
        )
    }
}
