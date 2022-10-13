use impl_display::ImplDisplay;

#[derive(Debug, ImplDisplay, Clone, PartialEq, Eq)]
pub enum RequestResult {
    NotExecuted,
    Pending,
    Success,
    Error,
}
