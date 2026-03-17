use axum::{
    Error as AxumEr,
    extract::rejection::{JsonDataError, JsonRejection, JsonSyntaxError},
};
use http::header::ToStrError;
use http_body::SizeHint;
use reqwest::{Error, StatusCode, header::HeaderMap};
use serde_json::Error as SerdeJsonEr;
use sqlx::{
    Error as SqlxEr,
    migrate::MigrateError,
    types::chrono::{NaiveDate, NaiveDateTime, NaiveTime},
    types::time::{PrimitiveDateTime, Time},
    types::uuid::Error as UuidEr,
    types::{BigDecimal, Decimal},
};
use std::io::Error as IoEr;
use time::error::ComponentRange;
use tracing::{dispatcher::SetGlobalDefaultError, log::SetLoggerError};
macro_rules! impl_to_err_string_to_string {
    ($($ty:ty),+) => {
        $(impl ToErrString for $ty {
            fn to_err_string(&self) -> String {
                self.to_string()
            }
        })+
    };
}
macro_rules! impl_to_err_string_debug {
    ($($ty:ty),+) => {
        $(impl ToErrString for $ty {
            fn to_err_string(&self) -> String {
                format!("{self:?}")
            }
        })+
    };
}
macro_rules! impl_to_err_string_display {
    ($($ty:ty),+) => {
        $(impl ToErrString for $ty {
            fn to_err_string(&self) -> String {
                format!("{self}")
            }
        })+
    };
}
macro_rules! impl_to_err_string_debug_pretty {
    ($($ty:ty),+) => {
        $(impl ToErrString for $ty {
            fn to_err_string(&self) -> String {
                format!("{self:#?}")
            }
        })+
    };
}
impl_to_err_string_to_string!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
impl_to_err_string_debug!(
    Option<i8>,
    Option<i16>,
    Option<i32>,
    Option<i64>,
    Option<u8>,
    Option<u16>,
    Option<u32>,
    Option<u64>,
    Option<f32>,
    Option<f64>
);
impl_to_err_string_debug_pretty!(HeaderMap, SizeHint);
impl_to_err_string_display!(
    ToStrError,
    AxumEr,
    usize,
    ComponentRange,
    UuidEr,
    IoEr,
    SqlxEr,
    SerdeJsonEr,
    Error,
    StatusCode,
    JsonDataError,
    MigrateError,
    JsonSyntaxError,
    JsonRejection,
    NaiveTime,
    NaiveDate,
    NaiveDateTime,
    Time,
    PrimitiveDateTime,
    Decimal,
    BigDecimal
);
pub trait ToErrString {
    fn to_err_string(&self) -> String;
}
impl ToErrString for String {
    fn to_err_string(&self) -> String {
        self.clone()
    }
}
impl ToErrString for SetGlobalDefaultError {
    fn to_err_string(&self) -> String {
        String::from("tracing::dispatcher::SetGlobalDefaultEr")
    }
}
impl ToErrString for SetLoggerError {
    fn to_err_string(&self) -> String {
        String::from("tracing::log::SetLoggerError")
    }
}
