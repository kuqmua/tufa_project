pub trait ToStdStringString {
    fn to_std_string_string(&self) -> std::string::String;
}

impl ToStdStringString for std::string::String {
    fn to_std_string_string(&self) -> std::string::String {
        self.clone()
    }
}

impl ToStdStringString for tracing::dispatcher::SetGlobalDefaultError {
    fn to_std_string_string(&self) -> std::string::String {
        std::string::String::from("tracing::dispatcher::SetGlobalDefaultError")
    }
}

impl ToStdStringString for tracing::log::SetLoggerError {
    fn to_std_string_string(&self) -> std::string::String {
        std::string::String::from("tracing::log::SetLoggerError")
    }
}

impl ToStdStringString for reqwest::header::HeaderMap {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}

impl ToStdStringString for http_body::SizeHint {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}