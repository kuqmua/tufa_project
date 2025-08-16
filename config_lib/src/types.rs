#[derive(
    Debug,
    Clone,
    Copy,
    strum_macros::EnumIter,
    enum_extension_lib::EnumExtension,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    from_str::FromStr,
)]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
impl std::default::Default for TracingLevel {
    fn default() -> Self {
        Self::Error
    }
}
impl std::fmt::Display for TracingLevel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_snake_case())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::Display,
    serde::Serialize,
    serde::Deserialize,
    from_str::FromStr,
)]
pub enum SourcePlaceType {
    Source,
    Github,
}
impl std::default::Default for SourcePlaceType {
    fn default() -> Self {
        Self::Github
    }
}
impl SourcePlaceType {
    pub fn from_env_or_default() -> Self {
        let fix_message = "You can set environment variable SOURCE_PLACE_TYPE to be equal \"source\" or \"github\"";
        if let Err(error) = dotenv::dotenv() {
            let default = Self::default();
            eprintln!(
                "using default SourcePlaceType::{default:#?} (failed to dotenv::dotenv(): {error}) {fix_message}"
            );
            return default;
        }
        let name = "SOURCE_PLACE_TYPE";
        match std::env::var(name) {
            Ok(value) => match <Self as std::str::FromStr>::from_str(&value) {
                Ok(value) => value,
                Err(error) => {
                    let default = Self::default();
                    eprintln!(
                        "using default SourcePlaceType::{default:#?} (<SourcePlaceType as std::str::FromStr>::from_str(&value): {error}) {fix_message}"
                    );
                    default
                }
            },
            Err(error) => {
                let default = Self::default();
                eprintln!(
                    "using default SourcePlaceType::{default:#?} (std::env::var(\"{name}\"): {error}) {fix_message}"
                );
                default
            }
        }
    }
}
