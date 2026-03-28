use crate::str_from_enum_macros::impl_from_str_for_enum_helper;
use dotenv::dotenv;
use optml::Optml;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};
use strum_macros::{Display as StrumDisplay, EnumIter};
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
impl FromStr for TracingLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        impl_from_str_for_enum_helper(
            s,
            &[
                ("trace", Self::Trace),
                ("debug", Self::Debug),
                ("info", Self::Info),
                ("warn", Self::Warn),
                ("er", Self::Er),
            ],
        )
    }
}
impl Display for TracingLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            convert_case::Casing::to_case(&format!("{self:?}"), convert_case::Case::Snake)
        )
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
        impl_from_str_for_enum_helper(s, &[("github", Self::Github), ("src", Self::Src)])
    }
}
impl SrcPlaceType {
    #[must_use]
    pub fn from_env_or_dflt() -> Self {
        let fix_msg =
            "You can set environment variable SRC_PLACE_TYPE to be eq \"src\" or \"github\"";
        if let Err(er) = dotenv() {
            let dflt = Self::default();
            eprintln!("using dflt SrcPlaceType::{dflt:#?} (failed to dotenv(): {er}) {fix_msg}");
            return dflt;
        }
        let name = "SRC_PLACE_TYPE";
        match env::var(name) {
            Ok(v) => match <Self as FromStr>::from_str(&v) {
                Ok(v0) => v0,
                Err(er) => {
                    let dflt = Self::default();
                    eprintln!(
                        "using dflt SrcPlaceType::{dflt:#?} (<SrcPlaceType as FromStr>::from_str(&v): {er}) {fix_msg}"
                    );
                    dflt
                }
            },
            Err(er) => {
                let dflt = Self::default();
                eprintln!(
                    "using dflt SrcPlaceType::{dflt:#?} (env::var(\"{name}\"): {er}) {fix_msg}"
                );
                dflt
            }
        }
    }
}
