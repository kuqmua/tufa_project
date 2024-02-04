

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
pub struct CodeOccurence {
    file: std::string::String,
    line: u32,
    column: u32,
    commit: std::string::String,
    #[schema(value_type = StdTimeDuration)]
    duration: std::time::Duration,
    additional_string: std::option::Option<std::string::String>
}

impl CodeOccurence {
    #[must_use]
    pub fn new(
        commit: std::string::String,
        file: std::string::String,
        line: u32,
        column: u32,
        additional_string: std::option::Option<std::string::String>
    ) -> Self {
        Self {
            file,
            line,
            column,
            commit,
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(std::time::Duration::default()),
            additional_string,
        }
    }
}

pub trait GetCommit {
    fn get_commit(&self) -> &str;
}

impl GetCommit for CodeOccurence {
    fn get_commit(&self) -> &str {
        &self.commit
    }
}

pub trait GetFile {
    fn get_file(&self) -> &str;
}

impl GetFile for CodeOccurence {
    fn get_file(&self) -> &str {
        &self.file
    }
}

pub trait GetLine {
    fn get_line(&self) -> &u32;
}

impl GetLine for CodeOccurence {
    fn get_line(&self) -> &u32 {
        &self.line
    }
}

pub trait GetColumn {
    fn get_column(&self) -> &u32;
}

impl GetColumn for CodeOccurence {
    fn get_column(&self) -> &u32 {
        &self.column
    }
}

pub trait GetDuration {
    fn get_duration(&self) -> std::time::Duration;
}

impl GetDuration for CodeOccurence {
    fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
}

pub trait GetAdditionalString {
    fn get_additional_string(&self) -> &std::option::Option<std::string::String>;
}

impl GetAdditionalString for CodeOccurence {
    fn get_additional_string(&self) -> &std::option::Option<std::string::String> {
        &self.additional_string
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