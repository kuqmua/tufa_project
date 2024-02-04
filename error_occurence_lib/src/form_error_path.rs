pub trait FormErrorPathDirectory {
    fn form_error_path_directory(&self) -> std::string::String;
}

impl<T> FormErrorPathDirectory for T 
where T: crate::get_file::GetFile 
    + crate::get_line::GetLine 
    + crate::get_column::GetColumn
{
    fn form_error_path_directory(&self) -> std::string::String {
        format!(
            "{}:{}:{}",
            self.get_file(), self.get_line(), self.get_column()
        )
    }
}

pub trait FormErrorPathGithub {
    fn form_error_path_github(&self) -> std::string::String;
}

impl<T> crate::form_error_path::FormErrorPathGithub for T 
where T: crate::get_commit::GetCommit
    + crate::get_file::GetFile 
    + crate::get_line::GetLine 
{
    fn form_error_path_github(&self) -> std::string::String {
        format!(
            "{}/blob/{}/{}#L{}",
            naming_constants::GITHUB_URL, self.get_commit(), self.get_file(), self.get_line()
        )
    }
}