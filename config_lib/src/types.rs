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
pub enum TracingTypeEnum {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
impl std::default::Default for TracingTypeEnum {
    fn default() -> Self {
        Self::Error
    }
}
impl std::fmt::Display for TracingTypeEnum {
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
        Self::Source
    }
}