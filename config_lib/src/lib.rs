pub mod str_from_enum_macros;
pub mod types;
use chrono::FixedOffset;
pub use gen_getter_traits_for_struct_fields::GenGetterTraitsForStructFields;
use optml::Optml;
use secrecy::SecretBox;
use std::{
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
                ensure_non_empty_env_value(v, |is_empty| Self::Error::IsEmpty { is_empty })
                    .map(Self)
            }
        }
    };
}
macro_rules! impl_try_from_parse {
    ($name:ident, $er_name:ident, $inner:ty, $er_vrt:ident, $er_field:ident, $er_ty:ty, $($derive:ident),*) => {
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
                parse_from_str_with_er(&v, |$er_field| Self::Error::$er_vrt { $er_field })
                    .map(Self)
            }
        }
    };
}
macro_rules! impl_try_from_parse_string_er {
    ($name:ident, $er_name:ident, $inner:ty, $er_vrt:ident, $er_field:ident) => {
        #[derive(
            Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml,
        )]
        pub struct $name(pub $inner);
        #[derive(Debug, Error, Optml)]
        pub enum $er_name {
            #[error("{:?}", .$er_field)]
            $er_vrt { $er_field: String },
        }
        impl TryFromStdEnvVarOk for $name {
            type Error = $er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                parse_from_str_with_er(&v, |$er_field| Self::Error::$er_vrt { $er_field }).map(Self)
            }
        }
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
                ensure_non_empty_env_value(v, |is_empty| Self::Error::IsEmpty { is_empty })
                    .map(|v| SecretBox::new(Box::new(v)))
                    .map(Self)
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
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct ServiceSocketAddress(pub SocketAddr);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkServiceSocketAddressEr {
    #[error("{std_net_socket_addr:?}")]
    StdNetSocketAddr { std_net_socket_addr: AddrParseError },
}
impl TryFromStdEnvVarOk for ServiceSocketAddress {
    type Error = TryFromStdEnvVarOkServiceSocketAddressEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        parse_from_str_with_er(&v, |std_net_socket_addr| Self::Error::StdNetSocketAddr {
            std_net_socket_addr,
        })
        .map(Self)
    }
}
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
        let Some(fixed_offset) = FixedOffset::east_opt(i32_v) else {
            return Err(Self::Error::ChronoFixedOffset {
                chrono_fixed_offset: TIMEZONE_NOT_EAST_MSG,
            });
        };
        Ok(Self(fixed_offset))
    }
}
impl_try_from_parse_string_er!(
    TracingLevel,
    TryFromStdEnvVarOkTracingLevelEr,
    types::TracingLevel,
    AppStateTracingLevelParsing,
    app_state_tracing_type_parsing
);
fn ensure_non_empty_env_value<Er>(
    v: String,
    mk_er: impl FnOnce(&'static str) -> Er,
) -> Result<String, Er> {
    if v.is_empty() {
        return Err(mk_er(ENV_VALUE_IS_EMPTY_MSG));
    }
    Ok(v)
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
#[cfg(test)]
mod tests {
    use super::{
        CorsAllowOrigin, DatabaseUrl, EnableApiGitCommitCheck, MaximumSizeOfHttpBodyInBytes,
        PgPoolMaxConnections, ServiceSocketAddress, SrcPlaceType, StartingCheckLink, Timezone,
        TracingLevel, TryFromStdEnvVarOk as _, TryFromStdEnvVarOkCorsAllowOriginEr,
        TryFromStdEnvVarOkDatabaseUrlEr, TryFromStdEnvVarOkEnableApiGitCommitCheckEr,
        TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr, TryFromStdEnvVarOkPgPoolMaxConnectionsEr,
        TryFromStdEnvVarOkServiceSocketAddressEr, TryFromStdEnvVarOkSrcPlaceTypeEr,
        TryFromStdEnvVarOkStartingCheckLinkEr, TryFromStdEnvVarOkTimezoneEr,
        TryFromStdEnvVarOkTracingLevelEr, types,
    };
    fn parse_env<T>(v: &str) -> Result<T, T::Error>
    where
        T: super::TryFromStdEnvVarOk,
    {
        T::try_from_std_env_var_ok(v.to_owned())
    }
    #[test]
    fn cors_allow_origin_parsing_returns_value() {
        let v = parse_env::<CorsAllowOrigin>("*");
        assert!(matches!(v, Ok(CorsAllowOrigin(_))));
    }
    #[test]
    fn cors_allow_origin_parsing_returns_error_for_empty_string() {
        let er = CorsAllowOrigin::try_from_std_env_var_ok(String::new());
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkCorsAllowOriginEr::IsEmpty { .. })
        ));
    }
    #[test]
    fn database_url_parsing_returns_value_for_non_empty_input() {
        let v = parse_env::<DatabaseUrl>("postgres://db");
        assert!(matches!(v, Ok(DatabaseUrl(_))));
    }
    #[test]
    fn database_url_parsing_returns_error_for_empty_string() {
        let er = DatabaseUrl::try_from_std_env_var_ok(String::new());
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkDatabaseUrlEr::IsEmpty { .. })
        ));
    }
    #[test]
    fn src_place_type_parsing_is_case_insensitive() {
        let v = parse_env::<SrcPlaceType>("GITHUB");
        assert!(matches!(v, Ok(SrcPlaceType(types::SrcPlaceType::Github))));
    }
    #[test]
    fn src_place_type_parsing_returns_error_for_unknown_value() {
        let er = parse_env::<SrcPlaceType>("bad");
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkSrcPlaceTypeEr::AppStateSrcPlaceTypeParsing { .. })
        ));
    }
    #[test]
    fn tracing_level_parsing_is_case_insensitive() {
        let v = parse_env::<TracingLevel>("DeBuG");
        assert!(matches!(v, Ok(TracingLevel(types::TracingLevel::Debug))));
    }
    #[test]
    fn tracing_level_parsing_returns_error_for_unknown_value() {
        let er = parse_env::<TracingLevel>("bad");
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkTracingLevelEr::AppStateTracingLevelParsing { .. })
        ));
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_bool() {
        let v = parse_env::<EnableApiGitCommitCheck>("true");
        assert!(matches!(v, Ok(EnableApiGitCommitCheck(true))));
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_error_for_invalid_bool() {
        let er = parse_env::<EnableApiGitCommitCheck>("truthy");
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkEnableApiGitCommitCheckEr::BoolParsing { .. })
        ));
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_usize() {
        let v = parse_env::<MaximumSizeOfHttpBodyInBytes>("128");
        assert!(matches!(v, Ok(MaximumSizeOfHttpBodyInBytes(128))));
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_invalid_number() {
        let er = parse_env::<MaximumSizeOfHttpBodyInBytes>("1k");
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr::UsizeParsing { .. })
        ));
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_u32() {
        let v = parse_env::<PgPoolMaxConnections>("10");
        assert!(matches!(v, Ok(PgPoolMaxConnections(10))));
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_error_for_invalid_number() {
        let er = parse_env::<PgPoolMaxConnections>("bad");
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkPgPoolMaxConnectionsEr::U32Parsing { .. })
        ));
    }
    #[test]
    fn non_empty_string_parser_returns_error_for_empty_value() {
        let er = parse_env::<StartingCheckLink>("");
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkStartingCheckLinkEr::IsEmpty { .. })
        ));
    }
    #[test]
    fn service_socket_address_parsing_returns_socket_addr() {
        let v = parse_env::<ServiceSocketAddress>("127.0.0.1:3000");
        assert!(matches!(v, Ok(ServiceSocketAddress(_))));
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
        let v = parse_env::<Timezone>("0");
        assert!(matches!(v, Ok(Timezone(_))));
    }
    #[test]
    fn timezone_parsing_returns_i32_error_for_non_number() {
        let er = parse_env::<Timezone>("nan");
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkTimezoneEr::I32Parsing { .. })
        ));
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
}
