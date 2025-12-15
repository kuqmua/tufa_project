#[derive(Clone, PartialEq, Eq, Eq)]
pub enum RequestResult {
    NotExecuted,
    Pending,
    Success,
    Error,
}

impl std::fmt::Display for RequestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
