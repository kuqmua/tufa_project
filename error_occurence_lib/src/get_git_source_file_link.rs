pub trait GetGitSourceFileLink<'a> {
    fn get_git_source_file_link(&self, file: &str, line: u32) -> std::string::String;
}

impl<'a, SelfGeneric> GetGitSourceFileLink<'a> for SelfGeneric
where
    Self: crate::git_fields::GetGitRepoLink + crate::git_fields::GetGitCommitId,
{
    fn get_git_source_file_link(&self, file: &str, line: u32) -> std::string::String {
        format!(
            "{}/blob/{}/{file}#L{line}",
            self.get_git_repo_link(),
            self.get_git_commit_id()
        )
    }
}
