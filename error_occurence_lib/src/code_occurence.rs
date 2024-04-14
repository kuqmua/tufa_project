#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
pub struct MacroOccurence {
    pub file: std::string::String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
pub struct CodeOccurence {
    file: std::string::String,
    line: u32,
    column: u32,
    commit: std::string::String,
    #[schema(value_type = StdTimeDuration)]
    duration: std::time::Duration,
    macro_occurence: std::option::Option<MacroOccurence>,
}

impl CodeOccurence {
    #[must_use]
    pub fn new(
        file: std::string::String,
        line: u32,
        column: u32,
        macro_occurence: std::option::Option<MacroOccurence>,
    ) -> Self {
        Self {
            file,
            line,
            column,
            commit: git_info::PROJECT_GIT_INFO.commit.to_string(),
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default(),
            macro_occurence,
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

pub trait GetMacroOccurence {
    fn get_macro_occurence(&self) -> &std::option::Option<MacroOccurence>;
}

impl GetMacroOccurence for CodeOccurence {
    fn get_macro_occurence(&self) -> &std::option::Option<MacroOccurence> {
        &self.macro_occurence
    }
}

impl std::fmt::Display for CodeOccurence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            CodeOccurencePrepareForLogWithoutConfig::code_occurence_prepare_for_log_without_config(
                self
            )
        )
    }
}

pub trait FormErrorPathDirectory {
    fn form_error_path_directory(&self) -> std::string::String;
}

impl<T> FormErrorPathDirectory for T
where
    T: GetFile + GetLine + GetColumn + GetMacroOccurence,
{
    fn form_error_path_directory(&self) -> std::string::String {
        match self.get_macro_occurence() {
            Some(value) => format!(
                "{}:{}:{} ({}:{}:{})",
                self.get_file(),
                self.get_line(),
                self.get_column(),
                value.file,
                value.line,
                value.column
            ),
            None => format!(
                "{}:{}:{}",
                self.get_file(),
                self.get_line(),
                self.get_column()
            ),
        }
    }
}

pub trait FormErrorPathGithub {
    fn form_error_path_github(&self) -> std::string::String;
}

impl<T> FormErrorPathGithub for T
where
    T: GetCommit + GetFile + GetLine + GetMacroOccurence,
{
    fn form_error_path_github(&self) -> std::string::String {
        match self.get_macro_occurence() {
            Some(value) => format!(
                "{}/blob/{}/{}#L{} ({}/blob/{}/{}#L{})",
                naming_constants::GITHUB_URL,
                self.get_commit(),
                self.get_file(),
                self.get_line(),
                naming_constants::GITHUB_URL,
                self.get_commit(),
                value.file,
                value.line
            ),
            None => format!(
                "{}/blob/{}/{}#L{}",
                naming_constants::GITHUB_URL,
                self.get_commit(),
                self.get_file(),
                self.get_line()
            ),
        }
    }
}

pub trait GetCodeOccurence {
    fn get_code_occurence(&self) -> &CodeOccurence;
}

pub trait GetCodePath {
    fn get_code_path(
        &self,
        source_place_type: &config_lib::config_fields::SourcePlaceType,
    ) -> std::string::String;
}

impl<T> GetCodePath for T
where
    T: FormErrorPathDirectory + FormErrorPathGithub,
{
    fn get_code_path(
        &self,
        source_place_type: &config_lib::config_fields::SourcePlaceType,
    ) -> std::string::String {
        match source_place_type {
            config_lib::config_fields::SourcePlaceType::Source => {
                self.form_error_path_directory()
            }
            config_lib::config_fields::SourcePlaceType::Github => self.form_error_path_github(),
        }
    }
}

pub trait CodeOccurencePrepareForLogWithConfig {
    fn code_occurence_prepare_for_log_with_config<
        ConfigGeneric: config_lib::config_fields::GetTimezone
            + config_lib::config_fields::GetSourcePlaceType
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String;
}

impl<SelfGeneric> CodeOccurencePrepareForLogWithConfig for SelfGeneric
where
    SelfGeneric: GetCodePath + GetDuration,
{
    fn code_occurence_prepare_for_log_with_config<
        ConfigGeneric: config_lib::config_fields::GetTimezone
            + config_lib::config_fields::GetSourcePlaceType
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        prepare_for_log(
            self.get_code_path(config.get_source_place_type()),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(config.get_timezone())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
    }
}

pub trait CodeOccurencePrepareForLogWithoutConfig {
    fn code_occurence_prepare_for_log_without_config(&self) -> std::string::String;
}

impl<SelfGeneric> CodeOccurencePrepareForLogWithoutConfig for SelfGeneric
where
    SelfGeneric: FormErrorPathGithub + GetDuration,
{
    fn code_occurence_prepare_for_log_without_config(&self) -> std::string::String {
        prepare_for_log(
            self.form_error_path_github(),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
    }
}

pub trait CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize {
    fn code_occurence_prepare_for_log_without_config_with_serialize_deserialize(
        &self,
    ) -> std::string::String;
}

impl<SelfGeneric> CodeOccurencePrepareForLogWithoutConfigWithSerializeDeserialize for SelfGeneric
where
    SelfGeneric: FormErrorPathGithub + GetDuration,
{
    fn code_occurence_prepare_for_log_without_config_with_serialize_deserialize(
        &self,
    ) -> std::string::String {
        prepare_for_log(
            self.form_error_path_github(),
            chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH + self.get_duration())
                .with_timezone(&chrono::FixedOffset::east_opt(10800).unwrap())
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        )
    }
}

fn prepare_for_log(path: std::string::String, time: std::string::String) -> std::string::String {
    format!("{path} {time}")
}
