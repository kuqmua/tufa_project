use app_state::SourcePlaceType;
use chrono::{DateTime, FixedOffset, Utc};
use git_info::PROJECT_GIT_INFO;
use naming::GITHUB_URL;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    sync::OnceLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use utoipa::ToSchema;
static SOURCE_PLACE_TYPE: OnceLock<SourcePlaceType> = OnceLock::new();
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct MacroOccurence {
    pub file: String,
    pub line: u32,
    pub column: u32,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, JsonSchema)]
pub struct Loc {
    #[allow(clippy::arbitrary_source_item_ordering)]
    file: String,
    line: u32,
    column: u32,
    commit: String,
    #[schema(value_type = StdTimeDuration)]
    duration: Duration,
    macro_occurence: Option<MacroOccurence>,
}
impl Loc {
    #[must_use]
    pub fn new(
        file: String,
        line: u32,
        column: u32,
        macro_occurence: Option<MacroOccurence>,
    ) -> Self {
        Self {
            file,
            line,
            column,
            commit: PROJECT_GIT_INFO.commit.to_owned(),
            duration: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default(),
            macro_occurence,
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, ToSchema)] //todo check somehow what its equal to std::time::Duration
pub struct StdTimeDuration {
    pub secs: u64,
    pub nanos: u32,
}
impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} {}",
            &match SOURCE_PLACE_TYPE.get_or_init(SourcePlaceType::from_env_or_default) {
                SourcePlaceType::Source => self.macro_occurence.as_ref().map_or_else(
                    || format!("{}:{}:{}", self.file, self.line, self.column),
                    |value_efd00048| format!(
                        "{}:{}:{} ({}:{}:{})",
                        self.file,
                        self.line,
                        self.column,
                        value_efd00048.file,
                        value_efd00048.line,
                        value_efd00048.column
                    )
                ),
                SourcePlaceType::Github => self.macro_occurence.as_ref().map_or_else(
                    || format!(
                        "{}/blob/{}/{}#L{}",
                        GITHUB_URL, self.commit, self.file, self.line
                    ),
                    |value_2ad91ca0| format!(
                        "{}/blob/{}/{}#L{} ({}/blob/{}/{}#L{})",
                        GITHUB_URL,
                        self.commit,
                        self.file,
                        self.line,
                        GITHUB_URL,
                        self.commit,
                        value_2ad91ca0.file,
                        value_2ad91ca0.line
                    )
                ),
            },
            match (
                UNIX_EPOCH.checked_add(self.duration),
                FixedOffset::east_opt(10800)
            ) {
                (Some(epoch), Some(offset)) => DateTime::<Utc>::from(epoch)
                    .with_timezone(&offset)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
                _ => "incorrect datetime".to_owned(),
            }
        )
    }
}
