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
static LOC_DISPLAY_TIMEZONE: OnceLock<Option<FixedOffset>> = OnceLock::new();
const LOC_DISPLAY_UTC_OFFSET_SECS: i32 = 10_800;
const INCORRECT_DATETIME_MSG: &str = "incorrect datetime";
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, JsonSchema, Optml)]
pub struct Occr {
    pub file: String,
    pub line: u32,
    pub col: u32,
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
    col: u32,
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl Loc {
    #[allow(clippy::single_call_fn)] // shared cached offset accessor is reused by formatter and tests
    fn loc_display_timezone() -> Option<&'static FixedOffset> {
        LOC_DISPLAY_TIMEZONE
            .get_or_init(|| FixedOffset::east_opt(LOC_DISPLAY_UTC_OFFSET_SECS))
            .as_ref()
    }
    fn fmt_with_occr(
        &self,
        f: &mut Formatter<'_>,
        mut fmt_primary: impl FnMut(&mut Formatter<'_>) -> FmtResult,
        mut fmt_occr: impl FnMut(&mut Formatter<'_>, &Occr) -> FmtResult,
    ) -> FmtResult {
        fmt_primary(f)?;
        if let Some(v) = self.occr.as_ref() {
            f.write_str(" (")?;
            fmt_occr(f, v)?;
            f.write_str(")")
        } else {
            Ok(())
        }
    }
    fn fmt_github_loc(&self, f: &mut Formatter<'_>, file: &str, line: u32) -> FmtResult {
        write!(f, "{}/blob/{}/{}#L{}", GITHUB_URL, self.commit, file, line)
    }
    fn fmt_src_loc(f: &mut Formatter<'_>, file: &str, line: u32, col: u32) -> FmtResult {
        write!(f, "{file}:{line}:{col}")
    }
    #[allow(clippy::single_call_fn)] // centralizes datetime + timezone composition so formatting can stay branch-light and tests can target conversion separately
    fn datetime_with_tz(&self) -> Option<DateTime<FixedOffset>> {
        let epoch = UNIX_EPOCH.checked_add(self.duration)?;
        let offset = Self::loc_display_timezone()?;
        Some(DateTime::<Utc>::from(epoch).with_timezone(offset))
    }
    fn fmt_datetime(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.datetime_with_tz() {
            Some(v) => write!(f, "{}", v.format("%Y-%m-%d %H:%M:%S")),
            None => f.write_str(INCORRECT_DATETIME_MSG),
        }
    }
    fn fmt_github_place(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_with_occr(
            f,
            |fmtr| self.fmt_github_loc(fmtr, &self.file, self.line),
            |fmtr, v| self.fmt_github_loc(fmtr, &v.file, v.line),
        )
    }
    fn fmt_place(&self, src_place_type: SrcPlaceType, f: &mut Formatter<'_>) -> FmtResult {
        match src_place_type {
            SrcPlaceType::Src => self.fmt_src_place(f),
            SrcPlaceType::Github => self.fmt_github_place(f),
        }
    }
    fn fmt_src_place(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_with_occr(
            f,
            |fmtr| Self::fmt_src_loc(fmtr, &self.file, self.line, self.col),
            |fmtr, v| Self::fmt_src_loc(fmtr, &v.file, v.line, v.col),
        )
    }
    #[must_use]
    pub fn new(file: String, line: u32, col: u32, occr: Option<Occr>) -> Self {
        Self {
            file,
            line,
            col,
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
        self.fmt_place(
            *SRC_PLACE_TYPE.get_or_init(SrcPlaceType::from_env_or_dflt),
            f,
        )?;
        f.write_str(" ")?;
        self.fmt_datetime(f)
    }
}
#[cfg(test)]
#[allow(clippy::arbitrary_source_item_ordering)]
mod tests {
    use super::{GITHUB_URL, INCORRECT_DATETIME_MSG, LOC_DISPLAY_UTC_OFFSET_SECS, Loc, Occr};
    use app_state::SrcPlaceType;
    use std::{
        fmt::{Display, Formatter, Result as FmtResult},
        time::Duration,
    };
    struct DatetimeFmt<'loc_lt> {
        loc: &'loc_lt Loc,
    }
    impl Display for DatetimeFmt<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            self.loc.fmt_datetime(f)
        }
    }
    struct PlaceFmt<'loc_lt> {
        loc: &'loc_lt Loc,
        src_place_type: SrcPlaceType,
    }
    impl Display for PlaceFmt<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            self.loc.fmt_place(self.src_place_type, f)
        }
    }
    fn test_loc(duration: Duration, occr: Option<Occr>) -> Loc {
        Loc {
            file: String::from("src/lib.rs"),
            commit: String::from("abc123"),
            duration,
            occr,
            line: 10,
            col: 20,
        }
    }
    fn test_occr() -> Occr {
        Occr {
            file: String::from("src/er.rs"),
            line: 30,
            col: 40,
        }
    }
    fn fmt_place(loc: &Loc, src_place_type: SrcPlaceType) -> String {
        format!(
            "{:}",
            PlaceFmt {
                loc,
                src_place_type
            }
        )
    }
    #[test]
    fn fmt_place_src_without_occr() {
        let loc = test_loc(Duration::from_secs(0), None);
        assert_eq!(fmt_place(&loc, SrcPlaceType::Src), "src/lib.rs:10:20");
    }
    #[test]
    fn fmt_place_src_with_occr() {
        let loc = test_loc(Duration::from_secs(0), Some(test_occr()));
        assert_eq!(
            fmt_place(&loc, SrcPlaceType::Src),
            "src/lib.rs:10:20 (src/er.rs:30:40)"
        );
    }
    #[test]
    fn fmt_place_github_without_occr() {
        let loc = test_loc(Duration::from_secs(0), None);
        assert_eq!(
            fmt_place(&loc, SrcPlaceType::Github),
            format!("{GITHUB_URL}/blob/abc123/src/lib.rs#L10")
        );
    }
    #[test]
    fn fmt_place_github_with_occr() {
        let loc = test_loc(Duration::from_secs(0), Some(test_occr()));
        assert_eq!(
            fmt_place(&loc, SrcPlaceType::Github),
            format!(
                "{GITHUB_URL}/blob/abc123/src/lib.rs#L10 ({GITHUB_URL}/blob/abc123/src/er.rs#L30)"
            )
        );
    }
    #[test]
    fn fmt_datetime_returns_fallback_for_overflowed_duration() {
        let loc = test_loc(Duration::MAX, None);
        assert_eq!(
            format!("{}", DatetimeFmt { loc: &loc }),
            INCORRECT_DATETIME_MSG
        );
    }
    #[test]
    fn datetime_with_tz_returns_expected_epoch_time_for_zero_duration() {
        let loc = test_loc(Duration::from_secs(0), None);
        let date_time = loc.datetime_with_tz().expect("f5c41dd8");
        assert_eq!(
            date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            "1970-01-01 03:00:00"
        );
    }
    #[test]
    fn loc_display_timezone_uses_expected_offset() {
        let offset = Loc::loc_display_timezone().expect("5c53d969");
        assert_eq!(offset.local_minus_utc(), LOC_DISPLAY_UTC_OFFSET_SECS);
    }
}
