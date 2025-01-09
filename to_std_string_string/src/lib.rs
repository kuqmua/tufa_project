pub trait ToStdStringString {
    fn to_std_string_string(&self) -> std::string::String;
}
impl ToStdStringString for std::string::String {
    fn to_std_string_string(&self) -> std::string::String {
        self.clone()
    }
}
impl ToStdStringString for std::primitive::i8 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::primitive::i16 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::primitive::i32 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::primitive::i64 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::primitive::u8 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::primitive::u16 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::primitive::u32 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::primitive::u64 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::primitive::f32 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::primitive::f64 {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl ToStdStringString for std::option::Option<std::primitive::i8> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<std::primitive::i16> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<std::primitive::i32> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<std::primitive::i64> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<std::primitive::u8> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<std::primitive::u16> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<std::primitive::u32> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<std::primitive::u64> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<std::primitive::f32> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<std::primitive::f64> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
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
impl ToStdStringString for axum::http::header::ToStrError {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for axum::Error {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for std::primitive::usize {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for time::error::ComponentRange {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::uuid::Error {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for std::io::Error {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::Error {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for serde_json::Error {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for reqwest::Error {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for http::StatusCode {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for axum::extract::rejection::JsonDataError {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::migrate::MigrateError {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for axum::extract::rejection::JsonSyntaxError {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for axum::extract::rejection::JsonRejection {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::chrono::NaiveTime {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::chrono::NaiveDate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::chrono::NaiveDateTime {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::time::Time {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::time::PrimitiveDateTime {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::Decimal {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::BigDecimal {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
// impl ToStdStringString for  {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
// impl ToStdStringString for  {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
