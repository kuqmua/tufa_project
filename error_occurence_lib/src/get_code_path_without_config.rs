// pub trait GetCodePathWithoutConfig {
//     fn get_code_path_without_config(&self) -> std::string::String;
// }

// impl<SelfGeneric> GetCodePathWithoutConfig for SelfGeneric
// where
//     SelfGeneric: crate::get_file::GetFile
//         + crate::get_line::GetLine
//         + crate::get_column::GetColumn,
// {
//     fn get_code_path_without_config(&self) -> std::string::String {
//         format!(
//             "src/{}:{}:{}", //todo "src" - hardcode, for some reason vscode stops following just {}:{}:{} path(without prefix "src")
//             self.get_file(),
//             self.get_line(),
//             self.get_column()
//         )
//     }
// }
