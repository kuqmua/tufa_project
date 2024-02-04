pub fn get_code_path(
    source_place_type: &config_lib::source_place_type::SourcePlaceType,
    code_occurence: &(impl crate::form_error_path::FormErrorPathDirectory
          + crate::form_error_path::FormErrorPathGithub),
) -> std::string::String {
    match source_place_type {
        config_lib::source_place_type::SourcePlaceType::Source => {
            crate::form_error_path::FormErrorPathDirectory::form_error_path_directory(
                code_occurence,
            )
        }
        config_lib::source_place_type::SourcePlaceType::Github => {
            crate::form_error_path::FormErrorPathGithub::form_error_path_github(code_occurence)
        }
    }
}
