pub trait SourceToStringWithoutConfig<'a> {
    fn source_to_string_without_config(&self) -> std::string::String;
}

impl SourceToStringWithoutConfig<'_> for std::string::String {
    fn source_to_string_without_config(&self) -> std::string::String {
        (*self).clone()
    }
}