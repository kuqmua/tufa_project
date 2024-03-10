#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub enum ConjunctiveOperator {
    Or,
    And,
}

impl std::fmt::Display for ConjunctiveOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConjunctiveOperator::Or => write!(f, "{}", naming_constants::OR),
            ConjunctiveOperator::And => {
                write!(f, "{}", naming_constants::AND)
            }
        }
    }
}
