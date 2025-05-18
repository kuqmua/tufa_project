#[derive(Clone, PartialEq, Eq, Eq)]
pub enum RequestResult {
    NotExecuted,
    Pending,
    Success,
    Error,
}

impl std::fmt::Display for RequestResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
