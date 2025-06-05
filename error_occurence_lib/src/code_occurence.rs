#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct MacroOccurence {
    pub file: std::string::String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
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
    pub fn new(file: std::string::String, line: u32, column: u32, macro_occurence: std::option::Option<MacroOccurence>) -> Self {
        Self {
            file,
            line,
            column,
            commit: git_info::PROJECT_GIT_INFO.commit.to_owned(),
            duration: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default(),
            macro_occurence,
        }
    }
}
static SOURCE_PLACE_TYPE: std::sync::OnceLock<app_state::SourcePlaceType> = std::sync::OnceLock::new();
impl std::fmt::Display for CodeOccurence {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} {}",
            &match SOURCE_PLACE_TYPE.get_or_init(app_state::SourcePlaceType::from_env_or_default) {
                app_state::SourcePlaceType::Source => self
                    .macro_occurence
                    .as_ref()
                    .map_or_else(|| format!("{}:{}:{}", self.file, self.line, self.column), |value| format!("{}:{}:{} ({}:{}:{})", self.file, self.line, self.column, value.file, value.line, value.column)),
                app_state::SourcePlaceType::Github => self.macro_occurence.as_ref().map_or_else(
                    || format!("{}/blob/{}/{}#L{}", naming::GITHUB_URL, self.commit, self.file, self.line),
                    |value| format!("{}/blob/{}/{}#L{} ({}/blob/{}/{}#L{})", naming::GITHUB_URL, self.commit, self.file, self.line, naming::GITHUB_URL, self.commit, value.file, value.line)
                ),
            },
            match (std::time::UNIX_EPOCH.checked_add(self.duration), chrono::FixedOffset::east_opt(10800)) {
                (Some(epoch), Some(offset)) => chrono::DateTime::<chrono::Utc>::from(epoch).with_timezone(&offset).format("%Y-%m-%d %H:%M:%S").to_string(),
                _ => "incorrect datetime".to_string(),
            }
        )
    }
}
