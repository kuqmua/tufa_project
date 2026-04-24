pub mod str_from_enum_macros;
pub mod types;
use chrono::FixedOffset;
pub use gen_getter_traits_for_struct_fields::GenGetterTraitsForStructFields;
use optml::Optml;
use secrecy::SecretBox;
use std::{
    env,
    net::{AddrParseError, SocketAddr},
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};
use thiserror::Error;
pub use try_from_env::TryFromEnv;
macro_rules! impl_try_from_non_empty_string {
    ($name:ident, $er_name:ident) => {
        #[derive(Debug, Clone, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct $name(pub String);
        #[derive(Debug, Error, Optml)]
        pub enum $er_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for $name {
            type Error = $er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                try_map_non_empty_env_value(v, |is_empty| Self::Error::IsEmpty { is_empty }, Self)
            }
        }
    };
}
macro_rules! impl_try_from_parse_mapped {
    ($name:ident, $er_name:ident, $inner:ty, $er_vrt:ident, $er_field:ident, $er_ty:ty, $map_parse_er:expr, $($derive:ident),*) => {
        #[derive(Debug, $($derive,)* gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct $name(pub $inner);
        #[derive(Debug, Error, Optml)]
        pub enum $er_name {
            #[error("{:?}", .$er_field)]
            $er_vrt { $er_field: $er_ty },
        }
        impl TryFromStdEnvVarOk for $name {
            type Error = $er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                parse_from_str_with_er(&v, $map_parse_er).map(Self)
            }
        }
    };
}
macro_rules! impl_try_from_parse {
    ($name:ident, $er_name:ident, $inner:ty, $er_vrt:ident, $er_field:ident, $er_ty:ty, $($derive:ident),*) => {
        impl_try_from_parse_mapped!(
            $name,
            $er_name,
            $inner,
            $er_vrt,
            $er_field,
            $er_ty,
            |$er_field| Self::Error::$er_vrt { $er_field },
            $($derive),*
        );
    };
}
macro_rules! impl_try_from_parse_string_er {
    ($name:ident, $er_name:ident, $inner:ty, $er_vrt:ident, $er_field:ident) => {
        impl_try_from_parse_mapped!(
            $name,
            $er_name,
            $inner,
            $er_vrt,
            $er_field,
            String,
            |$er_field| Self::Error::$er_vrt { $er_field },
            Clone,
            Copy
        );
    };
}
macro_rules! impl_try_from_secret_url {
    ($name:ident, $er_name:ident) => {
        #[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct $name(pub SecretBox<String>);
        #[derive(Debug, Error, Optml)]
        pub enum $er_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for $name {
            type Error = $er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                try_map_non_empty_env_value(
                    v,
                    |is_empty| Self::Error::IsEmpty { is_empty },
                    |v| Self(SecretBox::new(Box::new(v))),
                )
            }
        }
    };
}
const ENV_VALUE_IS_EMPTY_MSG: &str = "is empty";
const TIMEZONE_NOT_EAST_MSG: &str = "not east";
pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error>;
}
impl_try_from_non_empty_string!(CorsAllowOrigin, TryFromStdEnvVarOkCorsAllowOriginEr);
impl_try_from_secret_url!(DatabaseUrl, TryFromStdEnvVarOkDatabaseUrlEr);
impl_try_from_parse!(
    EnableApiGitCommitCheck,
    TryFromStdEnvVarOkEnableApiGitCommitCheckEr,
    bool,
    BoolParsing,
    bool_parsing,
    ParseBoolError,
    Clone,
    Copy
);
impl_try_from_parse!(
    MaximumSizeOfHttpBodyInBytes,
    TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr,
    usize,
    UsizeParsing,
    usize_parsing,
    ParseIntError,
    Clone,
    Copy
);
impl_try_from_secret_url!(MongoUrl, TryFromStdEnvVarOkMongoUrlEr);
impl_try_from_parse!(
    PgPoolMaxConnections,
    TryFromStdEnvVarOkPgPoolMaxConnectionsEr,
    u32,
    U32Parsing,
    u32_parsing,
    ParseIntError,
    Clone,
    Copy
);
impl_try_from_secret_url!(RedisUrl, TryFromStdEnvVarOkRedisUrlEr);
impl_try_from_parse!(
    ServiceSocketAddress,
    TryFromStdEnvVarOkServiceSocketAddressEr,
    SocketAddr,
    StdNetSocketAddr,
    std_net_socket_addr,
    AddrParseError,
    Clone,
    Copy
);
impl_try_from_parse_string_er!(
    SrcPlaceType,
    TryFromStdEnvVarOkSrcPlaceTypeEr,
    types::SrcPlaceType,
    AppStateSrcPlaceTypeParsing,
    app_state_src_place_type_parsing
);
impl_try_from_non_empty_string!(StartingCheckLink, TryFromStdEnvVarOkStartingCheckLinkEr);
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct Timezone(pub FixedOffset);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkTimezoneEr {
    #[error("{chrono_fixed_offset:?}")]
    ChronoFixedOffset { chrono_fixed_offset: &'static str },
    #[error("{i32_parsing:?}")]
    I32Parsing { i32_parsing: ParseIntError },
}
impl TryFromStdEnvVarOk for Timezone {
    type Error = TryFromStdEnvVarOkTimezoneEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        let i32_v =
            parse_from_str_with_er(&v, |i32_parsing| Self::Error::I32Parsing { i32_parsing })?;
        parse_east_fixed_offset(i32_v)
            .map_err(|chrono_fixed_offset| Self::Error::ChronoFixedOffset {
                chrono_fixed_offset,
            })
            .map(Self)
    }
}
impl_try_from_parse_string_er!(
    TracingLevel,
    TryFromStdEnvVarOkTracingLevelEr,
    types::TracingLevel,
    AppStateTracingLevelParsing,
    app_state_tracing_type_parsing
);
#[allow(clippy::single_call_fn)] // shared helper centralizes env var read + parse + error mapping for TryFromEnv derive output
pub fn parse_required_env_var<T, ParseEr, Er, MapEnvVarEr, Parse, MapParseEr>(
    env_var_name: &'static str,
    map_env_var_er: MapEnvVarEr,
    parse: Parse,
    map_parse_er: MapParseEr,
) -> Result<T, Er>
where
    MapEnvVarEr: FnOnce(env::VarError, String) -> Er,
    Parse: FnOnce(String) -> Result<T, ParseEr>,
    MapParseEr: FnOnce(ParseEr) -> Er,
{
    let v = env::var(env_var_name)
        .map_err(|std_env_var_er| map_env_var_er(std_env_var_er, env_var_name.to_owned()))?;
    parse(v).map_err(map_parse_er)
}
fn try_map_non_empty_env_value<T, Er>(
    v: String,
    mk_er: impl FnOnce(&'static str) -> Er,
    map_ok: impl FnOnce(String) -> T,
) -> Result<T, Er> {
    if v.is_empty() {
        return Err(mk_er(ENV_VALUE_IS_EMPTY_MSG));
    }
    Ok(map_ok(v))
}
fn parse_from_str_with_er<T, ParseEr, Er>(
    v: &str,
    mk_er: impl FnOnce(ParseEr) -> Er,
) -> Result<T, Er>
where
    T: FromStr<Err = ParseEr>,
{
    v.parse::<T>().map_err(mk_er)
}
#[allow(clippy::single_call_fn)] // extracted timezone conversion keeps conversion + message mapping reusable and directly testable
fn parse_east_fixed_offset(v: i32) -> Result<FixedOffset, &'static str> {
    FixedOffset::east_opt(v).ok_or(TIMEZONE_NOT_EAST_MSG)
}
#[cfg(test)]
mod tests {
    use super::{
        CorsAllowOrigin, DatabaseUrl, EnableApiGitCommitCheck, MaximumSizeOfHttpBodyInBytes,
        MongoUrl, PgPoolMaxConnections, RedisUrl, ServiceSocketAddress, SrcPlaceType,
        StartingCheckLink, Timezone, TracingLevel, TryFromStdEnvVarOkCorsAllowOriginEr,
        TryFromStdEnvVarOkDatabaseUrlEr, TryFromStdEnvVarOkEnableApiGitCommitCheckEr,
        TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr, TryFromStdEnvVarOkMongoUrlEr,
        TryFromStdEnvVarOkPgPoolMaxConnectionsEr, TryFromStdEnvVarOkRedisUrlEr,
        TryFromStdEnvVarOkServiceSocketAddressEr, TryFromStdEnvVarOkSrcPlaceTypeEr,
        TryFromStdEnvVarOkStartingCheckLinkEr, TryFromStdEnvVarOkTimezoneEr,
        TryFromStdEnvVarOkTracingLevelEr, types,
    };
    macro_rules! assert_parse_ok_matches {
        ($type0:ty, $value:expr, $pattern:pat) => {
            assert!(matches!(parse_env::<$type0>($value), Ok($pattern)));
        };
    }
    macro_rules! assert_parse_err_matches {
        ($type0:ty, $value:expr, $pattern:pat) => {
            assert!(matches!(parse_env::<$type0>($value), Err($pattern)));
        };
    }
    macro_rules! assert_empty_parse_err_matches {
        ($type0:ty, $pattern:pat) => {
            assert!(matches!(parse_env::<$type0>(""), Err($pattern)));
        };
    }
    #[derive(Debug, PartialEq, Eq)]
    enum ParseRequiredEnvVarTestEr {
        EnvVar { env_var_name: String },
        Parse { parse: &'static str },
    }
    fn parse_env<T>(v: &str) -> Result<T, T::Error>
    where
        T: super::TryFromStdEnvVarOk,
    {
        T::try_from_std_env_var_ok(v.to_owned())
    }
    #[test]
    fn cors_allow_origin_parsing_returns_value() {
        assert_parse_ok_matches!(CorsAllowOrigin, "*", CorsAllowOrigin(_));
    }
    #[test]
    fn cors_allow_origin_parsing_returns_error_for_empty_string() {
        assert_empty_parse_err_matches!(
            CorsAllowOrigin,
            TryFromStdEnvVarOkCorsAllowOriginEr::IsEmpty { .. }
        );
    }
    #[test]
    fn database_url_parsing_returns_value_for_non_empty_input() {
        assert_parse_ok_matches!(DatabaseUrl, "postgres://db", DatabaseUrl(_));
    }
    #[test]
    fn database_url_parsing_returns_error_for_empty_string() {
        assert_empty_parse_err_matches!(
            DatabaseUrl,
            TryFromStdEnvVarOkDatabaseUrlEr::IsEmpty { .. }
        );
    }
    #[test]
    fn mongo_url_parsing_returns_value_for_non_empty_input() {
        assert_parse_ok_matches!(MongoUrl, "mongodb://db", MongoUrl(_));
    }
    #[test]
    fn mongo_url_parsing_returns_error_for_empty_string() {
        assert_empty_parse_err_matches!(MongoUrl, TryFromStdEnvVarOkMongoUrlEr::IsEmpty { .. });
    }
    #[test]
    fn redis_url_parsing_returns_value_for_non_empty_input() {
        assert_parse_ok_matches!(RedisUrl, "redis://db", RedisUrl(_));
    }
    #[test]
    fn redis_url_parsing_returns_error_for_empty_string() {
        assert_empty_parse_err_matches!(RedisUrl, TryFromStdEnvVarOkRedisUrlEr::IsEmpty { .. });
    }
    #[test]
    fn src_place_type_parsing_is_case_insensitive() {
        assert_parse_ok_matches!(
            SrcPlaceType,
            "GITHUB",
            SrcPlaceType(types::SrcPlaceType::Github)
        );
    }
    #[test]
    fn src_place_type_parsing_returns_error_for_unknown_value() {
        assert_parse_err_matches!(
            SrcPlaceType,
            "bad",
            TryFromStdEnvVarOkSrcPlaceTypeEr::AppStateSrcPlaceTypeParsing { .. }
        );
    }
    #[test]
    fn tracing_level_parsing_is_case_insensitive() {
        assert_parse_ok_matches!(
            TracingLevel,
            "DeBuG",
            TracingLevel(types::TracingLevel::Debug)
        );
    }
    #[test]
    fn tracing_level_parsing_returns_error_for_unknown_value() {
        assert_parse_err_matches!(
            TracingLevel,
            "bad",
            TryFromStdEnvVarOkTracingLevelEr::AppStateTracingLevelParsing { .. }
        );
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_bool() {
        assert_parse_ok_matches!(
            EnableApiGitCommitCheck,
            "true",
            EnableApiGitCommitCheck(true)
        );
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_error_for_invalid_bool() {
        assert_parse_err_matches!(
            EnableApiGitCommitCheck,
            "truthy",
            TryFromStdEnvVarOkEnableApiGitCommitCheckEr::BoolParsing { .. }
        );
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_usize() {
        assert_parse_ok_matches!(
            MaximumSizeOfHttpBodyInBytes,
            "128",
            MaximumSizeOfHttpBodyInBytes(128)
        );
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_invalid_number() {
        assert_parse_err_matches!(
            MaximumSizeOfHttpBodyInBytes,
            "1k",
            TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr::UsizeParsing { .. }
        );
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_u32() {
        assert_parse_ok_matches!(PgPoolMaxConnections, "10", PgPoolMaxConnections(10));
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_error_for_invalid_number() {
        assert_parse_err_matches!(
            PgPoolMaxConnections,
            "bad",
            TryFromStdEnvVarOkPgPoolMaxConnectionsEr::U32Parsing { .. }
        );
    }
    #[test]
    fn non_empty_string_parser_returns_error_for_empty_value() {
        assert_empty_parse_err_matches!(
            StartingCheckLink,
            TryFromStdEnvVarOkStartingCheckLinkEr::IsEmpty { .. }
        );
    }
    #[test]
    fn non_empty_string_parser_returns_value_for_non_empty_value() {
        assert_parse_ok_matches!(
            StartingCheckLink,
            "https://example.com",
            StartingCheckLink(_)
        );
    }
    #[test]
    fn service_socket_address_parsing_returns_socket_addr() {
        assert_parse_ok_matches!(
            ServiceSocketAddress,
            "127.0.0.1:3000",
            ServiceSocketAddress(_)
        );
    }
    #[test]
    fn service_socket_address_parsing_returns_error_for_invalid_addr() {
        let er = parse_env::<ServiceSocketAddress>("127.0.0.1");
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkServiceSocketAddressEr::StdNetSocketAddr { .. })
        ));
    }
    #[test]
    fn timezone_parsing_returns_timezone_for_valid_offset() {
        assert_parse_ok_matches!(Timezone, "0", Timezone(_));
    }
    #[test]
    fn timezone_parsing_returns_i32_error_for_non_number() {
        assert_parse_err_matches!(
            Timezone,
            "nan",
            TryFromStdEnvVarOkTimezoneEr::I32Parsing { .. }
        );
    }
    #[test]
    fn parse_east_fixed_offset_returns_offset_for_valid_seconds() {
        let parsed = super::parse_east_fixed_offset(3i32 * 3_600i32);
        assert!(matches!(parsed, Ok(v) if v.local_minus_utc() == 3i32 * 3_600i32));
    }
    #[test]
    fn parse_east_fixed_offset_returns_error_for_out_of_range_seconds() {
        let parsed = super::parse_east_fixed_offset(i32::MAX);
        assert_eq!(parsed, Err(super::TIMEZONE_NOT_EAST_MSG));
    }
    #[test]
    fn timezone_parsing_returns_offset_error_when_out_of_range() {
        let out_of_range = i32::MAX.to_string();
        let er = parse_env::<Timezone>(&out_of_range);
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkTimezoneEr::ChronoFixedOffset { .. })
        ));
    }
    #[test]
    fn parse_required_env_var_parses_value_when_env_var_exists() {
        let parsed = super::parse_required_env_var(
            "PATH",
            |_std_env_var_er, env_var_name| ParseRequiredEnvVarTestEr::EnvVar { env_var_name },
            |v| Ok::<_, &'static str>(v.len()),
            |parse| ParseRequiredEnvVarTestEr::Parse { parse },
        );
        assert!(matches!(parsed, Ok(v) if v > 0));
    }
    #[test]
    fn parse_required_env_var_maps_missing_env_var_error() {
        let parsed = super::parse_required_env_var(
            "CONFIG_LIB_TEST_ENV_VAR_4E8A7F21",
            |_std_env_var_er, env_var_name| ParseRequiredEnvVarTestEr::EnvVar { env_var_name },
            Ok::<_, &'static str>,
            |parse| ParseRequiredEnvVarTestEr::Parse { parse },
        );
        assert_eq!(
            parsed,
            Err(ParseRequiredEnvVarTestEr::EnvVar {
                env_var_name: "CONFIG_LIB_TEST_ENV_VAR_4E8A7F21".to_owned()
            })
        );
    }
    #[test]
    fn parse_required_env_var_maps_parse_error() {
        let parsed = super::parse_required_env_var(
            "PATH",
            |_std_env_var_er, env_var_name| ParseRequiredEnvVarTestEr::EnvVar { env_var_name },
            |_v| Err::<(), _>("parse failed"),
            |parse| ParseRequiredEnvVarTestEr::Parse { parse },
        );
        assert_eq!(
            parsed,
            Err(ParseRequiredEnvVarTestEr::Parse {
                parse: "parse failed"
            })
        );
    }
}
