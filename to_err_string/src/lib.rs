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
use std::{borrow::Cow, fmt::Debug, io::Error as IoEr};
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
                static_str_to_owned($msg)
            }
        })+
    };
}
macro_rules! impl_to_err_string_as_ref_str {
    ($($ty:ty),+ $(,)?) => {
        $(impl ToErrString for $ty {
            fn to_err_string(&self) -> String {
                as_ref_str_to_owned(self)
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
    => |v| v.to_string()
);
pub trait ToErrString {
    fn to_err_string(&self) -> String;
}
impl<T> ToErrString for &T
where
    T: ToErrString + ?Sized,
{
    fn to_err_string(&self) -> String {
        (*self).to_err_string()
    }
}
impl<T> ToErrString for Option<T>
where
    T: Debug,
{
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl<T, E> ToErrString for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn to_err_string(&self) -> String {
        format!("{self:?}")
    }
}
impl_to_err_string_as_ref_str!(String, str, Cow<'_, str>);
impl_to_err_string_const!(
    SetGlobalDefaultError => "tracing::dispatcher::SetGlobalDefaultEr",
    SetLoggerError => "tracing::log::SetLoggerError",
);
fn debug_alt_to_string<T: Debug>(v: &T) -> String {
    format!("{v:#?}")
}
fn as_ref_str_to_owned<T>(v: &T) -> String
where
    T: ?Sized + AsRef<str>,
{
    v.as_ref().to_owned()
}
fn static_str_to_owned(v: &'static str) -> String {
    v.to_owned()
}
#[cfg(test)]
mod tests {
    #[allow(clippy::single_call_fn)] // shared assertion keeps ToErrString behavior checks concise and consistent
    fn assert_to_err_string(v: impl super::ToErrString, exp: &str) {
        assert_eq!(v.to_err_string(), exp);
    }
    #[test]
    fn to_err_string_for_primitives_and_options() {
        assert_to_err_string(42i32, "42");
        assert_to_err_string(Some(7u8), "Some(7)");
        assert_to_err_string(None::<u16>, "None");
        assert_to_err_string(true, "true");
        assert_to_err_string('x', "x");
        assert_to_err_string(Some(String::from("abc")), "Some(\"abc\")");
    }
    #[test]
    fn to_err_string_for_strings_and_str_refs() {
        use std::borrow::Cow;
        let owned = String::from("abc");
        let borrowed = "xyz";
        assert_to_err_string(owned, "abc");
        assert_to_err_string(borrowed, "xyz");
        assert_to_err_string(Cow::Borrowed("qwe"), "qwe");
        assert_to_err_string(Cow::<'_, str>::Owned(String::from("rty")), "rty");
    }
    #[test]
    fn to_err_string_for_result_values() {
        assert_to_err_string(Result::<u8, u16>::Ok(5), "Ok(5)");
        assert_to_err_string(Result::<u8, &'static str>::Err("er"), "Err(\"er\")");
    }
}
