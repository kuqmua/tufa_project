pub trait SourceToStringWithoutConfig<'a> {
    fn source_to_string_without_config(&self) -> std::string::String;
}

impl SourceToStringWithoutConfig<'_> for std::string::String {
    fn source_to_string_without_config(&self) -> std::string::String {
        (*self).clone()
    }
}

impl SourceToStringWithoutConfig<'_> for std::vec::Vec<std::string::String> {
    fn source_to_string_without_config(&self) -> std::string::String {
        format!(
            "[\n{}]",
            self.iter().fold(
                std::string::String::from(""),
                |mut acc, element| {
                    acc.push_str(&format!(" {element}\n"));
                    acc
                },
            )
        )
    }
}

impl SourceToStringWithoutConfig<'_> for std::collections::HashMap<std::string::String, std::string::String> {
    fn source_to_string_without_config(&self) -> std::string::String {
        format!(
            "{{\n{}}}", 
            self.iter().fold(
                std::string::String::new(),
                |mut acc, (key, value)| {
                    acc.push_str(
                        &format!(
                            "{}: [\n{}]\n", 
                            key, 
                            value
                            .lines()
                            .fold(std::string::String::new(), |mut acc, line| {
                                acc.push_str(&format!(" {line}\n"));
                                acc
                            })
                        )
                    );
                    acc
                },
            )
            .lines()
            .fold(std::string::String::new(), |mut acc, line| {
                acc.push_str(&format!(" {line}\n"));
                acc
            })
        )
    }
}