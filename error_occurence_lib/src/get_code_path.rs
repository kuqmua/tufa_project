pub fn get_code_path(
    source_place_type: &config_lib::source_place_type::SourcePlaceType,
    code_occurence: &(impl crate::code_occurence::FormErrorPathDirectory
          + crate::code_occurence::FormErrorPathGithub),
) -> std::string::String {
    match source_place_type {
        config_lib::source_place_type::SourcePlaceType::Source => {
            crate::code_occurence::FormErrorPathDirectory::form_error_path_directory(
                code_occurence,
            )
        }
        config_lib::source_place_type::SourcePlaceType::Github => {
            crate::code_occurence::FormErrorPathGithub::form_error_path_github(code_occurence)
        }
    }
}
