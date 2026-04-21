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
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    io::Error as IoEr,
};
use time::error::ComponentRange;
use tracing::{dispatcher::SetGlobalDefaultError, log::SetLoggerError};
macro_rules! impl_to_err_string_with {
    ($($ty:ty),+ => |$value:ident| $body:expr) => {
        $(impl ToErrString for $ty {
            fn to_err_string(&self) -> String {
                let $value = self;
                $body
            }
        })+
    };
}
macro_rules! impl_to_err_string_const {
    ($($ty:ty => $msg:expr),+ $(,)?) => {
        $(impl ToErrString for $ty {
            fn to_err_string(&self) -> String {
                String::from($msg)
            }
        })+
    };
}
macro_rules! impl_to_err_string_str_ref {
    ($($ty:ty),+ $(,)?) => {
        $(impl ToErrString for $ty {
            fn to_err_string(&self) -> String {
                str_ref_to_owned(self.as_ref())
            }
        })+
    };
}
impl_to_err_string_with!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, bool, char => |v| v.to_string());
impl_to_err_string_with!(HeaderMap, SizeHint => |v| debug_alt_to_string(v));
impl_to_err_string_with!(
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
    => |v| display_to_string(v)
);
pub trait ToErrString {
    fn to_err_string(&self) -> String;
}
impl<T> ToErrString for Option<T>
where
    T: Debug,
{
    fn to_err_string(&self) -> String {
        debug_to_string(self)
    }
}
impl<T, E> ToErrString for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn to_err_string(&self) -> String {
        debug_to_string(self)
    }
}
impl_to_err_string_str_ref!(String, str, &str, Cow<'_, str>);
impl_to_err_string_const!(
    SetGlobalDefaultError => "tracing::dispatcher::SetGlobalDefaultEr",
    SetLoggerError => "tracing::log::SetLoggerError",
);
fn display_to_string<T: Display>(v: &T) -> String {
    v.to_string()
}
fn debug_to_string<T: Debug>(v: &T) -> String {
    format!("{v:?}")
}
fn debug_alt_to_string<T: Debug>(v: &T) -> String {
    format!("{v:#?}")
}
fn str_ref_to_owned(v: &str) -> String {
    v.to_owned()
}
#[cfg(test)]
mod tests {
    use super::ToErrString as _;
    #[test]
    fn to_err_string_for_primitives_and_options() {
        assert_eq!(42i32.to_err_string(), "42");
        assert_eq!(Some(7u8).to_err_string(), "Some(7)");
        assert_eq!(None::<u16>.to_err_string(), "None");
        assert_eq!(true.to_err_string(), "true");
        assert_eq!('x'.to_err_string(), "x");
        assert_eq!(Some(String::from("abc")).to_err_string(), "Some(\"abc\")");
    }
    #[test]
    fn to_err_string_for_strings_and_str_refs() {
        use std::borrow::Cow;
        let owned = String::from("abc");
        assert_eq!(owned.to_err_string(), "abc");
        assert_eq!("xyz".to_err_string(), "xyz");
        assert_eq!(Cow::Borrowed("qwe").to_err_string(), "qwe");
        assert_eq!(
            Cow::<'_, str>::Owned(String::from("rty")).to_err_string(),
            "rty"
        );
    }
    #[test]
    fn to_err_string_for_result_values() {
        assert_eq!(Result::<u8, u16>::Ok(5).to_err_string(), "Ok(5)");
        assert_eq!(
            Result::<u8, &'static str>::Err("er").to_err_string(),
            "Err(\"er\")"
        );
    }
}
