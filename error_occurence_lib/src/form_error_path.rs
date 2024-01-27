pub trait FormErrorPathDirectory {
    fn form_error_path_directory(&self) -> std::string::String;
}

impl<SelfGeneric> FormErrorPathDirectory for SelfGeneric
where
    SelfGeneric: crate::get_file::GetFile + crate::get_line::GetLine + crate::get_column::GetColumn,
{
    fn form_error_path_directory(&self) -> std::string::String {
        format!(
            "src/{}:{}:{}", //todo "src" - hardcode, for some reason vscode stops following just {}:{}:{} path(without prefix "src")
            self.get_file(),
            self.get_line(),
            self.get_column()
        )
    }
}

pub trait FormErrorPathGithub {
    fn form_error_path_github(&self) -> std::string::String;
}

impl<'a, SelfGeneric> FormErrorPathGithub for SelfGeneric
where
    SelfGeneric: crate::get_file::GetFile
        + crate::get_line::GetLine
        + crate::get_column::GetColumn
        + crate::get_git_source_file_link::GetGitSourceFileLink<'a>,
{
    fn form_error_path_github(&self) -> std::string::String {
        let backslash = "/";
        let file = self.get_file();
        match file.find(backslash) {
            Some(index) => {
                self.get_git_source_file_link(&file[index + backslash.len()..], *self.get_line())
            }
            None => {
                std::string::String::from("cant find backslash symbol in file path of location")
            }
        }
    }
}
