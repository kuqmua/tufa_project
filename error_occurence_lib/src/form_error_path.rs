pub trait FormErrorPathDirectory {
    fn form_error_path_directory(&self) -> std::string::String;
}

impl<T> FormErrorPathDirectory for T 
where T: crate::code_occurence::GetFile 
    + crate::code_occurence::GetLine 
    + crate::code_occurence::GetColumn
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
where T: crate::code_occurence::GetCommit
    + crate::code_occurence::GetFile 
    + crate::code_occurence::GetLine 
{
    fn form_error_path_github(&self) -> std::string::String {
        format!(
            "{}/blob/{}/{}#L{}",
            naming_constants::GITHUB_URL, self.get_commit(), self.get_file(), self.get_line()
        )
    }
}