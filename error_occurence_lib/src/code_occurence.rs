

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

pub trait FormErrorPathDirectory {
    fn form_error_path_directory(&self) -> std::string::String;
}

impl<T> FormErrorPathDirectory for T 
where T: GetFile + GetLine + GetColumn
{
    fn form_error_path_directory(&self) -> std::string::String {
        format!(
            "{}:{}:{}",
            self.get_file(), self.get_line(), self.get_column()
        )
    }
}

pub trait FormErrorPathGithub {
    fn form_error_path_github(&self) -> std::string::String;
}

impl<T> FormErrorPathGithub for T 
where T: GetCommit + GetFile + GetLine 
{
    fn form_error_path_github(&self) -> std::string::String {
        format!(
            "{}/blob/{}/{}#L{}",
            naming_constants::GITHUB_URL, self.get_commit(), self.get_file(), self.get_line()
        )
    }
}

pub trait GetCodeOccurence {
    fn get_code_occurence(&self) -> &CodeOccurence;
}

pub trait GetCodePath {
    fn get_code_path(&self, source_place_type: &config_lib::source_place_type::SourcePlaceType) -> std::string::String;
}

impl<T> GetCodePath for T 
where T: FormErrorPathDirectory + FormErrorPathGithub
{
    fn get_code_path(&self, source_place_type: &config_lib::source_place_type::SourcePlaceType) -> std::string::String {
        match source_place_type {
            config_lib::source_place_type::SourcePlaceType::Source => self.form_error_path_directory(),
            config_lib::source_place_type::SourcePlaceType::Github => self.form_error_path_github(),
        }
    }
}

// pub fn get_code_path(
//     source_place_type: &config_lib::source_place_type::SourcePlaceType,
//     code_occurence: &(impl FormErrorPathDirectory + FormErrorPathGithub),
// ) -> std::string::String {
//     match source_place_type {
//         config_lib::source_place_type::SourcePlaceType::Source => FormErrorPathDirectory::form_error_path_directory(code_occurence),
//         config_lib::source_place_type::SourcePlaceType::Github => FormErrorPathGithub::form_error_path_github(code_occurence),
//     }
// }