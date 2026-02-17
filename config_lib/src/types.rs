use dotenv::dotenv;
use enum_extension_lib::EnumExtension;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};
use strum_macros::{Display as StrumDisplay, EnumIter};
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, Default, Clone, Copy, EnumIter, EnumExtension, Serialize, Deserialize, PartialEq, Eq,
)]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    #[default]
    Error,
}
impl FromStr for TracingLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err(format!("Unknown tracing level: {s}")),
        }
    }
}
impl Display for TracingLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_sc())
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, StrumDisplay, Serialize, Deserialize)]
pub enum SourcePlaceType {
    #[default]
    Github,
    Source,
}
impl FromStr for SourcePlaceType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "github" => Ok(Self::Github),
            "source" => Ok(Self::Source),
            _ => Err(format!("Unknown source place type: {s}")),
        }
    }
}
impl SourcePlaceType {
    #[must_use]
    pub fn from_env_or_default() -> Self {
        let fix_message = "You can set environment variable SOURCE_PLACE_TYPE to be equal \"source\" or \"github\"";
        if let Err(error) = dotenv() {
            let default = Self::default();
            eprintln!(
                "using default SourcePlaceType::{default:#?} (failed to dotenv(): {error}) {fix_message}"
            );
            return default;
        }
        let name = "SOURCE_PLACE_TYPE";
        match env::var(name) {
            Ok(env_value) => match <Self as FromStr>::from_str(&env_value) {
                Ok(value) => value,
                Err(error) => {
                    let default = Self::default();
                    eprintln!(
                        "using default SourcePlaceType::{default:#?} (<SourcePlaceType as FromStr>::from_str(&value): {error}) {fix_message}"
                    );
                    default
                }
            },
            Err(error) => {
                let default = Self::default();
                eprintln!(
                    "using default SourcePlaceType::{default:#?} (env::var(\"{name}\"): {error}) {fix_message}"
                );
                default
            }
        }
    }
}
