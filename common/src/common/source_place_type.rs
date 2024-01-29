pub fn get_code_path<'a>(
    source_place_type: &config_lib::source_place_type::SourcePlaceType,
    code_occurence: &(impl error_occurence_lib::get_file::GetFile
          + error_occurence_lib::get_line::GetLine
          + error_occurence_lib::get_column::GetColumn
          + error_occurence_lib::get_git_source_file_link::GetGitSourceFileLink<'a>),
) -> std::string::String {
    match source_place_type {
        config_lib::source_place_type::SourcePlaceType::Source => error_occurence_lib::form_error_path::FormErrorPathDirectory::form_error_path_directory(code_occurence),
        config_lib::source_place_type::SourcePlaceType::Github => error_occurence_lib::form_error_path::FormErrorPathGithub::form_error_path_github(code_occurence)
    }
}
