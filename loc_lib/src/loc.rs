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
    fn fmt_datetime(&self, f: &mut Formatter<'_>) -> FmtResult {
        match (
            UNIX_EPOCH.checked_add(self.duration),
            FixedOffset::east_opt(10800),
        ) {
            (Some(epoch), Some(offset)) => write!(
                f,
                "{}",
                DateTime::<Utc>::from(epoch)
                    .with_timezone(&offset)
                    .format("%Y-%m-%d %H:%M:%S")
            ),
            _ => f.write_str("incorrect datetime"),
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
    use super::{GITHUB_URL, Loc, Occr};
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
    fn test_loc(occr: Option<Occr>) -> Loc {
        Loc {
            file: String::from("src/lib.rs"),
            commit: String::from("abc123"),
            duration: Duration::from_secs(0),
            occr,
            line: 10,
            col: 20,
        }
    }
    #[test]
    fn fmt_place_src_without_occr() {
        let loc = test_loc(None);
        assert_eq!(
            format!(
                "{}",
                PlaceFmt {
                    loc: &loc,
                    src_place_type: SrcPlaceType::Src
                }
            ),
            "src/lib.rs:10:20"
        );
    }
    #[test]
    fn fmt_place_src_with_occr() {
        let loc = test_loc(Some(Occr {
            file: String::from("src/er.rs"),
            line: 30,
            col: 40,
        }));
        assert_eq!(
            format!(
                "{}",
                PlaceFmt {
                    loc: &loc,
                    src_place_type: SrcPlaceType::Src
                }
            ),
            "src/lib.rs:10:20 (src/er.rs:30:40)"
        );
    }
    #[test]
    fn fmt_place_github_without_occr() {
        let loc = test_loc(None);
        assert_eq!(
            format!(
                "{}",
                PlaceFmt {
                    loc: &loc,
                    src_place_type: SrcPlaceType::Github
                }
            ),
            format!("{GITHUB_URL}/blob/abc123/src/lib.rs#L10")
        );
    }
    #[test]
    fn fmt_place_github_with_occr() {
        let loc = test_loc(Some(Occr {
            file: String::from("src/er.rs"),
            line: 30,
            col: 40,
        }));
        assert_eq!(
            format!(
                "{}",
                PlaceFmt {
                    loc: &loc,
                    src_place_type: SrcPlaceType::Github
                }
            ),
            format!(
                "{GITHUB_URL}/blob/abc123/src/lib.rs#L10 ({GITHUB_URL}/blob/abc123/src/er.rs#L30)"
            )
        );
    }
    #[test]
    fn fmt_datetime_returns_fallback_for_overflowed_duration() {
        let loc = Loc {
            file: String::from("src/lib.rs"),
            commit: String::from("abc123"),
            duration: Duration::MAX,
            occr: None,
            line: 10,
            col: 20,
        };
        assert_eq!(
            format!("{}", DatetimeFmt { loc: &loc }),
            "incorrect datetime"
        );
    }
}
