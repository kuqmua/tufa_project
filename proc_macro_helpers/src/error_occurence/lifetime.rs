#[derive(Debug)]
pub enum Lifetime {
    Specified(String),
    NotSpecified,
}

impl std::fmt::Display for crate::error_occurence::lifetime::Lifetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Specified(l) => {
                write!(f, "'{l}")
            }
            Self::NotSpecified => write!(f, ""),
        }
    }
}
