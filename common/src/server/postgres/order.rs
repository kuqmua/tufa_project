#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, from_str::FromStr)]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "{}", naming_constants::ASC),
            Order::Desc => write!(f, "{}", naming_constants::DESC),
        }
    }
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}
