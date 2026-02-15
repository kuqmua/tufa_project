use axum::{
    Error as AxumError,
    extract::rejection::{JsonDataError, JsonRejection, JsonSyntaxError},
};
use http::header::ToStrError;
use http_body::SizeHint;
use reqwest::{Error, StatusCode, header::HeaderMap};
use serde_json::Error as SerdeJsonError;
use sqlx::{
    Error as SqlxError,
    migrate::MigrateError,
    types::chrono::{NaiveDate, NaiveDateTime, NaiveTime},
    types::time::{PrimitiveDateTime, Time},
    types::uuid::Error as UuidError,
    types::{BigDecimal, Decimal},
};
use std::io::Error as IoError;
use time::error::ComponentRange;
use tracing::{dispatcher::SetGlobalDefaultError, log::SetLoggerError};
pub trait ToErrString {
    fn to_err_string(&self) -> String;
}
impl ToErrString for String {
    fn to_err_string(&self) -> String {
        self.clone()
    }
}
impl ToErrString for i8 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for i16 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for i32 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for i64 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for u8 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for u16 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for u32 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for u64 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for f32 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for f64 {
    fn to_err_string(&self) -> String {
        self.to_string()
    }
}
impl ToErrString for Option<i8> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for Option<i16> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for Option<i32> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for Option<i64> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for Option<u8> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for Option<u16> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for Option<u32> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for Option<u64> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for Option<f32> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for Option<f64> {
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl ToErrString for SetGlobalDefaultError {
    fn to_err_string(&self) -> String {
        String::from("tracing::dispatcher::SetGlobalDefaultError")
    }
}
impl ToErrString for SetLoggerError {
    fn to_err_string(&self) -> String {
        String::from("tracing::log::SetLoggerError")
    }
}
impl ToErrString for HeaderMap {
    fn to_err_string(&self) -> String {
        format!("{self:#?}")
    }
}
impl ToErrString for SizeHint {
    fn to_err_string(&self) -> String {
        format!("{self:#?}")
    }
}
impl ToErrString for ToStrError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for AxumError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for usize {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for ComponentRange {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for UuidError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for IoError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for SqlxError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for SerdeJsonError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for Error {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for StatusCode {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for JsonDataError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for MigrateError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for JsonSyntaxError {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for JsonRejection {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for NaiveTime {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for NaiveDate {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for NaiveDateTime {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for Time {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for PrimitiveDateTime {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for Decimal {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
impl ToErrString for BigDecimal {
    fn to_err_string(&self) -> String {
        format!("{self}")
    }
}
