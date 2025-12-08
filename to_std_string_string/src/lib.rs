pub trait ToStdStringString {
    fn to_std_string_string(&self) -> String;
}
impl ToStdStringString for String {
    fn to_std_string_string(&self) -> String {
        self.clone()
    }
}
impl ToStdStringString for i8 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for i16 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for i32 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for i64 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for u8 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for u16 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for u32 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for u64 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for f32 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for f64 {
    fn to_std_string_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for std::option::Option<i8> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<i16> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<i32> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<i64> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<u8> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<u16> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<u32> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<u64> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<f32> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for std::option::Option<f64> {
    fn to_std_string_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for tracing::dispatcher::SetGlobalDefaultError {
    fn to_std_string_string(&self) -> String {
        String::from("tracing::dispatcher::SetGlobalDefaultError")
    }
}
impl ToStdStringString for tracing::log::SetLoggerError {
    fn to_std_string_string(&self) -> String {
        String::from("tracing::log::SetLoggerError")
    }
}
impl ToStdStringString for reqwest::header::HeaderMap {
    fn to_std_string_string(&self) -> String {
        format!("{self:#?}")
    }
}
impl ToStdStringString for http_body::SizeHint {
    fn to_std_string_string(&self) -> String {
        format!("{self:#?}")
    }
}
impl ToStdStringString for axum::http::header::ToStrError {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for axum::Error {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for usize {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for time::error::ComponentRange {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::uuid::Error {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for std::io::Error {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::Error {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for serde_json::Error {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for reqwest::Error {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for http::StatusCode {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for axum::extract::rejection::JsonDataError {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::migrate::MigrateError {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for axum::extract::rejection::JsonSyntaxError {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for axum::extract::rejection::JsonRejection {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::chrono::NaiveTime {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::chrono::NaiveDate {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::chrono::NaiveDateTime {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::time::Time {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::time::PrimitiveDateTime {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::Decimal {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::types::BigDecimal {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
// impl ToStdStringString for  {
//     fn to_std_string_string(&self) -> String {
//         format!("{self}")
//     }
// }
// impl ToStdStringString for  {
//     fn to_std_string_string(&self) -> String {
//         format!("{self}")
//     }
// }
