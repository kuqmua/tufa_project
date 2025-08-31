generate_where_element_filters::generate_where_element_filters!();

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum EncodeFormat {
    Base64,
    Hex,
    Escape,
}
impl std::fmt::Display for EncodeFormat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Base64 => write!(formatter, "base64"),
            Self::Hex => write!(formatter, "hex"),
            Self::Escape => write!(formatter, "escape"),
        }
    }
}
impl std::default::Default for EncodeFormat {
    fn default() -> Self {
        Self::Base64
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for EncodeFormat {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self::default()
    }
}