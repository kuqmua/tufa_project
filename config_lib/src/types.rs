use dotenv::dotenv;
use enum_extension_lib::EnumExtension;
use optimal_pack::OptimalPack;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};
use strum_macros::{Display as StrumDisplay, EnumIter};
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    EnumIter,
    EnumExtension,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    OptimalPack,
)]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    #[default]
    Er,
}
impl FromStr for TracingLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "er" => Ok(Self::Er),
            _ => Err(format!("Unknown tracing level: {s}")),
        }
    }
}
impl Display for TracingLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_sc())
    }
}
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, StrumDisplay, Serialize, Deserialize, OptimalPack,
)]
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
    pub fn from_env_or_dflt() -> Self {
        let fix_message = "You can set environment variable SOURCE_PLACE_TYPE to be equal \"source\" or \"github\"";
        if let Err(er) = dotenv() {
            let dflt = Self::default();
            eprintln!(
                "using dflt SourcePlaceType::{dflt:#?} (failed to dotenv(): {er}) {fix_message}"
            );
            return dflt;
        }
        let name = "SOURCE_PLACE_TYPE";
        match env::var(name) {
            Ok(v) => match <Self as FromStr>::from_str(&v) {
                Ok(v0) => v0,
                Err(er) => {
                    let dflt = Self::default();
                    eprintln!(
                        "using dflt SourcePlaceType::{dflt:#?} (<SourcePlaceType as FromStr>::from_str(&v): {er}) {fix_message}"
                    );
                    dflt
                }
            },
            Err(er) => {
                let dflt = Self::default();
                eprintln!(
                    "using dflt SourcePlaceType::{dflt:#?} (env::var(\"{name}\"): {er}) {fix_message}"
                );
                dflt
            }
        }
    }
}
