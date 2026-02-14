use axum::extract::rejection::{JsonDataError, JsonRejection, JsonSyntaxError};
use http::header::ToStrError;
use reqwest::header::HeaderMap;
use sqlx::{
    migrate::MigrateError,
    types::chrono::{NaiveDate, NaiveDateTime, NaiveTime},
    types::time::{PrimitiveDateTime, Time},
    types::uuid::Error as UuidError,
    types::{BigDecimal, Decimal},
};
use std::io::Error as IoError;
use time::error::ComponentRange;
use tracing::{dispatcher::SetGlobalDefaultError, log::SetLoggerError};

pub trait ToStdStringString {
    fn to_err_string(&self) -> String;
}
impl ToStdStringString for String {
    fn to_err_string(&self) -> String {
        self.clone()
    }
}
impl ToStdStringString for i8 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for i16 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for i32 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for i64 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for u8 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for u16 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for u32 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for u64 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for f32 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for f64 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToStdStringString for Option<i8> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for Option<i16> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for Option<i32> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for Option<i64> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for Option<u8> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for Option<u16> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for Option<u32> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for Option<u64> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for Option<f32> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for Option<f64> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToStdStringString for SetGlobalDefaultError {
    fn to_err_string(&self) -> String {
        String::from("tracing::dispatcher::SetGlobalDefaultError")
    }
}
impl ToStdStringString for SetLoggerError {
    fn to_err_string(&self) -> String {
        String::from("tracing::log::SetLoggerError")
    }
}
impl ToStdStringString for HeaderMap {
    fn to_err_string(&self) -> String {
        format!("{self:#?}")
    }
}
impl ToStdStringString for http_body::SizeHint {
    fn to_err_string(&self) -> String {
        format!("{self:#?}")
    }
}
impl ToStdStringString for ToStrError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for axum::Error {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for usize {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for ComponentRange {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for UuidError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for IoError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for sqlx::Error {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for serde_json::Error {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for reqwest::Error {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for reqwest::StatusCode {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for JsonDataError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for MigrateError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for JsonSyntaxError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for JsonRejection {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for NaiveTime {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for NaiveDate {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for NaiveDateTime {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for Time {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for PrimitiveDateTime {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for Decimal {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToStdStringString for BigDecimal {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
// impl ToStdStringString for  {
//     fn to_err_string(&self) -> String {
//         format!("{self}")
//     }
// }
// impl ToStdStringString for  {
//     fn to_err_string(&self) -> String {
//         format!("{self}")
//     }
// }
