use crate::str_from_enum_macros::impl_from_str_for_enum_helper;
use dotenv::dotenv;
use optml::Optml;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
    sync::OnceLock,
};
use strum_macros::{Display as StrumDisplay, EnumIter};
const TRACING_LEVEL_PARSE_PAIRS: [(&str, TracingLevel); 5] = [
    ("trace", TracingLevel::Trace),
    ("debug", TracingLevel::Debug),
    ("info", TracingLevel::Info),
    ("warn", TracingLevel::Warn),
    ("er", TracingLevel::Er),
];
const SRC_PLACE_TYPE_PARSE_PAIRS: [(&str, SrcPlaceType); 2] =
    [("github", SrcPlaceType::Github), ("src", SrcPlaceType::Src)];
const SRC_PLACE_TYPE_ENV_VAR: &str = "SRC_PLACE_TYPE";
const SRC_PLACE_TYPE_FIX_MSG: &str =
    "You can set environment variable SRC_PLACE_TYPE to be eq \"src\" or \"github\"";
static DOTENV_INIT: OnceLock<()> = OnceLock::new();
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Default, Clone, Copy, EnumIter, Serialize, Deserialize, PartialEq, Eq, Optml)]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    #[default]
    Er,
}
impl TracingLevel {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Er => "er",
        }
    }
}
impl FromStr for TracingLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        impl_from_str_for_enum_helper(s, &TRACING_LEVEL_PARSE_PAIRS)
    }
}
impl Display for TracingLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", (*self).as_str())
    }
}
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, StrumDisplay, Serialize, Deserialize, Optml,
)]
pub enum SrcPlaceType {
    #[default]
    Github,
    Src,
}
impl FromStr for SrcPlaceType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        impl_from_str_for_enum_helper(s, &SRC_PLACE_TYPE_PARSE_PAIRS)
    }
}
impl SrcPlaceType {
    #[must_use]
    pub fn from_env_or_dflt() -> Self {
        let dflt = Self::default();
        let _: &() = DOTENV_INIT.get_or_init(|| {
            if let Err(er) = dotenv() {
                eprintln!("dotenv() failed in SrcPlaceType::from_env_or_dflt: {er}");
            }
        });
        let parsed = Self::parse_src_place_type_from_env_var(env::var(SRC_PLACE_TYPE_ENV_VAR));
        match parsed {
            Ok(v) => v,
            Err(msg) => {
                eprintln!("using dflt SrcPlaceType::{dflt:#?} ({msg}) {SRC_PLACE_TYPE_FIX_MSG}");
                dflt
            }
        }
    }
    #[allow(clippy::single_call_fn)] // parsing helper isolates context-rich error formatting and is reused by tests
    fn parse_src_place_type_env_value(v: &str) -> Result<Self, String> {
        <Self as FromStr>::from_str(v)
            .map_err(|er| format!("<SrcPlaceType as FromStr>::from_str(&v): {er}"))
    }
    #[allow(clippy::single_call_fn)] // helper keeps env-read error context centralized and deterministic for tests
    fn parse_src_place_type_from_env_var(v: Result<String, env::VarError>) -> Result<Self, String> {
        let src_place_type_env_v =
            v.map_err(|er| format!("env::var(\"{SRC_PLACE_TYPE_ENV_VAR}\"): {er}"))?;
        Self::parse_src_place_type_env_value(&src_place_type_env_v)
    }
}
#[cfg(test)]
mod tests {
    use super::{
        SRC_PLACE_TYPE_PARSE_PAIRS, SrcPlaceType, TRACING_LEVEL_PARSE_PAIRS, TracingLevel,
    };
    use std::env;
    use std::str::FromStr as _;
    #[test]
    fn tracing_level_display_is_stable() {
        assert_eq!(TracingLevel::Trace.to_string(), "trace");
        assert_eq!(TracingLevel::Debug.to_string(), "debug");
        assert_eq!(TracingLevel::Info.to_string(), "info");
        assert_eq!(TracingLevel::Warn.to_string(), "warn");
        assert_eq!(TracingLevel::Er.to_string(), "er");
    }
    #[test]
    fn tracing_level_from_str_is_case_insensitive() {
        assert_eq!(TracingLevel::from_str("TRACE"), Ok(TracingLevel::Trace));
        assert_eq!(TracingLevel::from_str("Warn"), Ok(TracingLevel::Warn));
        let _er = TracingLevel::from_str("bad").expect_err("9f8d72a1");
    }
    #[test]
    fn tracing_level_roundtrip_is_stable_for_all_variants() {
        for (name, level) in TRACING_LEVEL_PARSE_PAIRS {
            assert_eq!(TracingLevel::from_str(name), Ok(level));
            assert_eq!(level.to_string(), name);
        }
    }
    #[test]
    fn src_place_type_from_str_roundtrip_is_stable_for_all_variants() {
        for (name, value) in SRC_PLACE_TYPE_PARSE_PAIRS {
            assert_eq!(SrcPlaceType::from_str(name), Ok(value));
        }
    }
    #[test]
    fn src_place_type_from_str_accepts_src_value() {
        assert_eq!(SrcPlaceType::from_str("src"), Ok(SrcPlaceType::Src));
    }
    #[test]
    fn src_place_type_from_str_rejects_unknown_value() {
        let _er = SrcPlaceType::from_str("bad").expect_err("8d6f70bb");
    }
    #[test]
    fn src_place_type_default_is_github() {
        assert_eq!(SrcPlaceType::default(), SrcPlaceType::Github);
    }
    #[test]
    fn src_place_type_parse_error_contains_expected_context() {
        let er = SrcPlaceType::from_str("unknown").expect_err("f2cc7d6b");
        assert!(er.contains("Unknown value"));
        assert!(er.contains("Allowed values:"));
    }
    #[test]
    fn parse_src_place_type_env_value_parses_case_insensitively() {
        let parsed = SrcPlaceType::parse_src_place_type_env_value("GiThUb");
        assert_eq!(parsed, Ok(SrcPlaceType::Github));
    }
    #[test]
    fn parse_src_place_type_env_value_wraps_parse_context() {
        let er = SrcPlaceType::parse_src_place_type_env_value("bad").expect_err("8c9f2a17");
        assert!(er.contains("<SrcPlaceType as FromStr>::from_str(&v):"));
        assert!(er.contains("Unknown value: bad"));
    }
    #[test]
    fn parse_src_place_type_from_env_var_wraps_missing_var_context() {
        let er = SrcPlaceType::parse_src_place_type_from_env_var(Err(env::VarError::NotPresent))
            .expect_err("5a83f2be");
        assert!(er.contains("env::var(\"SRC_PLACE_TYPE\")"));
    }
    #[test]
    fn parse_src_place_type_from_env_var_parses_ok_value() {
        let parsed = SrcPlaceType::parse_src_place_type_from_env_var(Ok(String::from("src")));
        assert_eq!(parsed, Ok(SrcPlaceType::Src));
    }
}
