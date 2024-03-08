#[derive(
    Debug,
    Clone,
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

impl Default for SourcePlaceType {
    fn default() -> Self {
        Self::Source
    }
}
