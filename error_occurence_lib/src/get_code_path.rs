pub fn get_code_path<'a>(
    source_place_type: &config_lib::source_place_type::SourcePlaceType,
    code_occurence: &(impl crate::get_file::GetFile
          + crate::get_line::GetLine
          + crate::get_column::GetColumn
          + crate::get_git_source_file_link::GetGitSourceFileLink<'a>),
) -> std::string::String {
    match source_place_type {
        config_lib::source_place_type::SourcePlaceType::Source => crate::form_error_path::FormErrorPathDirectory::form_error_path_directory(code_occurence),
        config_lib::source_place_type::SourcePlaceType::Github => crate::form_error_path::FormErrorPathGithub::form_error_path_github(code_occurence)
    }
}
