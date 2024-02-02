#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
pub struct CodeOccurence {
    file: std::string::String,
    line: u32,
    column: u32,
    commit: std::string::String,
    project_part: std::string::String,
    #[schema(value_type = StdTimeDuration)]
    duration: std::time::Duration,
}

impl CodeOccurence {
    #[must_use]
    pub fn new(
        commit: std::string::String,
        project_part: std::string::String,
        file: std::string::String,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            file,
            line,
            column,
            commit,
            project_part,
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

impl<'a> crate::get_git_source_file_link::GetGitSourceFileLink<'a> for CodeOccurence {
    fn get_git_source_file_link(&self, file: &str, line: u32) -> std::string::String {
        format!(
            "https://github.com/kuqmua/tufa_project/blob/{}/{}/{file}#L{line}",
            self.commit,
            self.project_part
        )
    }
}