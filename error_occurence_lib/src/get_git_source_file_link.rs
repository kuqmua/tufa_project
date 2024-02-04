pub trait GetGitSourceFileLink {
    fn get_git_source_file_link(&self, file: &str, line: u32) -> std::string::String;
}
