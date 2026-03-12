use app_state::SrcPlaceType;
use chrono::{DateTime, FixedOffset, Utc};
use git_info::PROJECT_GIT_INFO;
use naming::GITHUB_URL;
use optml::Optml;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    sync::OnceLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use utoipa::ToSchema;
static SRC_PLACE_TYPE: OnceLock<SrcPlaceType> = OnceLock::new();
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, JsonSchema, Optml)]
pub struct Occr {
    pub file: String,
    pub line: u32,
    pub column: u32,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, JsonSchema, Optml)]
pub struct Loc {
    #[allow(clippy::arbitrary_source_item_ordering)]
    file: String,
    commit: String,
    duration: Duration,
    occr: Option<Occr>,
    line: u32,
    column: u32,
}
impl Loc {
    #[must_use]
    pub fn new(file: String, line: u32, column: u32, occr: Option<Occr>) -> Self {
        Self {
            file,
            line,
            column,
            commit: PROJECT_GIT_INFO.commit.to_owned(),
            duration: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default(),
            occr,
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, ToSchema, Optml)] //todo check somehow what its eq to std::time::Duration
pub struct StdTimeDuration {
    pub secs: u64,
    pub nanos: u32,
}
impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} {}",
            &match SRC_PLACE_TYPE.get_or_init(SrcPlaceType::from_env_or_dflt) {
                SrcPlaceType::Src => self.occr.as_ref().map_or_else(
                    || format!("{}:{}:{}", self.file, self.line, self.column),
                    |v| format!(
                        "{}:{}:{} ({}:{}:{})",
                        self.file, self.line, self.column, v.file, v.line, v.column
                    )
                ),
                SrcPlaceType::Github => self.occr.as_ref().map_or_else(
                    || format!(
                        "{}/blob/{}/{}#L{}",
                        GITHUB_URL, self.commit, self.file, self.line
                    ),
                    |v| format!(
                        "{}/blob/{}/{}#L{} ({}/blob/{}/{}#L{})",
                        GITHUB_URL,
                        self.commit,
                        self.file,
                        self.line,
                        GITHUB_URL,
                        self.commit,
                        v.file,
                        v.line
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
