pub trait DisplayForeignType {
    fn display_foreign_type(&self) -> std::string::String;
}

impl DisplayForeignType for tracing::dispatcher::SetGlobalDefaultError {
    fn display_foreign_type(&self) -> std::string::String {
        std::string::String::from("tracing::dispatcher::SetGlobalDefaultError")
    }
}

impl DisplayForeignType for tracing::log::SetLoggerError {
    fn display_foreign_type(&self) -> std::string::String {
        std::string::String::from("tracing::log::SetLoggerError")
    }
}

impl DisplayForeignType for reqwest::header::HeaderMap {
    fn display_foreign_type(&self) -> std::string::String {
        format!("{self:#?}")
    }
}

impl DisplayForeignType for http_body::SizeHint {
    fn display_foreign_type(&self) -> std::string::String {
        format!("{self:#?}")
    }
}