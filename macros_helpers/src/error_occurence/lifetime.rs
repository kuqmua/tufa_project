#[derive(Debug)]
pub enum Lifetime {
    Specified(String),
    NotSpecified,
}

impl std::fmt::Display for crate::error_occurence::lifetime::Lifetime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Specified(value) => {
                write!(formatter, "'{value}")
            }
            Self::NotSpecified => write!(formatter, ""),
        }
    }
}
