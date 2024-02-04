#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
pub struct CodeOccurence {
    file: std::string::String,
    line: u32,
    column: u32,
    commit: std::string::String,
    #[schema(value_type = StdTimeDuration)]
    duration: std::time::Duration,
}

impl CodeOccurence {
    #[must_use]
    pub fn new(
        commit: std::string::String,
        file: std::string::String,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            file,
            line,
            column,
            commit,
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("cannot convert time to unix_epoch"),
        }
    }
}

impl crate::get_file::GetFile for CodeOccurence {
    fn get_file(&self) -> &str {
        &self.file
    }
}

impl crate::get_line::GetLine for CodeOccurence {
    fn get_line(&self) -> &u32 {
        &self.line
    }
}

impl crate::get_column::GetColumn for CodeOccurence {
    fn get_column(&self) -> &u32 {
        &self.column
    }
}

impl crate::get_duration::GetDuration for CodeOccurence {
    fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
}

impl std::fmt::Display for CodeOccurence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            crate::code_occurence_prepare_for_log::CodeOccurencePrepareForLogWithoutConfig::code_occurence_prepare_for_log_without_config(self)
        )
    }
}

impl crate::form_error_path::FormErrorPathDirectory for CodeOccurence {
    fn form_error_path_directory(&self) -> std::string::String {
        format!(
            "{}:{}:{}",
            self.file, self.line, self.column
        )
    }
}

impl crate::form_error_path::FormErrorPathGithub for CodeOccurence {
    fn form_error_path_github(&self) -> std::string::String {
        format!(
            "{}/blob/{}/{}#L{}",
            naming_constants::GITHUB_URL, self.commit, self.file, self.line
        )
    }
}