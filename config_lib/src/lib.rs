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
            IsEmpty { is_empty: String },
        }
        impl TryFromStdEnvVarOk for $name {
            type Error = $er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                parse_non_empty_env_value(v, |v| v, |is_empty| Self::Error::IsEmpty { is_empty })
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
macro_rules! impl_try_from_secret_url {
    ($name:ident, $er_name:ident) => {
        #[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct $name(pub SecretBox<String>);
        #[derive(Debug, Error, Optml)]
        pub enum $er_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: String },
        }
        impl TryFromStdEnvVarOk for $name {
            type Error = $er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                parse_non_empty_env_value(
                    v,
                    |v| SecretBox::new(Box::new(v)),
                    |is_empty| Self::Error::IsEmpty { is_empty },
                )
                .map(Self)
            }
        }
    };
}
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
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct SrcPlaceType(pub types::SrcPlaceType);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkSrcPlaceTypeEr {
    #[error("{app_state_src_place_type_parsing:?}")]
    AppStateSrcPlaceTypeParsing {
        app_state_src_place_type_parsing: String,
    },
}
impl TryFromStdEnvVarOk for SrcPlaceType {
    type Error = TryFromStdEnvVarOkSrcPlaceTypeEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        parse_from_str_with_er(&v, |app_state_src_place_type_parsing| {
            Self::Error::AppStateSrcPlaceTypeParsing {
                app_state_src_place_type_parsing,
            }
        })
        .map(Self)
    }
}
impl_try_from_non_empty_string!(StartingCheckLink, TryFromStdEnvVarOkStartingCheckLinkEr);
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct Timezone(pub FixedOffset);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkTimezoneEr {
    #[error("{chrono_fixed_offset:?}")]
    ChronoFixedOffset { chrono_fixed_offset: String },
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
                chrono_fixed_offset: String::from("not east"),
            });
        };
        Ok(Self(fixed_offset))
    }
}
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct TracingLevel(pub types::TracingLevel);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkTracingLevelEr {
    #[error("{app_state_tracing_type_parsing:?}")]
    AppStateTracingLevelParsing {
        app_state_tracing_type_parsing: String,
    },
}
impl TryFromStdEnvVarOk for TracingLevel {
    type Error = TryFromStdEnvVarOkTracingLevelEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        parse_from_str_with_er(&v, |app_state_tracing_type_parsing| {
            Self::Error::AppStateTracingLevelParsing {
                app_state_tracing_type_parsing,
            }
        })
        .map(Self)
    }
}
fn parse_non_empty_env_value<T, Er>(
    v: String,
    mk_ok: impl FnOnce(String) -> T,
    mk_er: impl FnOnce(String) -> Er,
) -> Result<T, Er> {
    if v.is_empty() {
        return Err(mk_er(String::from("is empty")));
    }
    Ok(mk_ok(v))
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
        EnableApiGitCommitCheck, MaximumSizeOfHttpBodyInBytes, ServiceSocketAddress, SrcPlaceType,
        Timezone, TracingLevel, TryFromStdEnvVarOk as _,
        TryFromStdEnvVarOkEnableApiGitCommitCheckEr,
        TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr, TryFromStdEnvVarOkServiceSocketAddressEr,
        TryFromStdEnvVarOkSrcPlaceTypeEr, TryFromStdEnvVarOkStartingCheckLinkEr,
        TryFromStdEnvVarOkTimezoneEr, TryFromStdEnvVarOkTracingLevelEr, types,
    };
    #[test]
    fn src_place_type_parsing_is_case_insensitive() {
        let v = SrcPlaceType::try_from_std_env_var_ok(String::from("GITHUB"));
        assert!(matches!(v, Ok(SrcPlaceType(types::SrcPlaceType::Github))));
    }
    #[test]
    fn src_place_type_parsing_returns_error_for_unknown_value() {
        let er = SrcPlaceType::try_from_std_env_var_ok(String::from("bad"));
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkSrcPlaceTypeEr::AppStateSrcPlaceTypeParsing { .. })
        ));
    }
    #[test]
    fn tracing_level_parsing_is_case_insensitive() {
        let v = TracingLevel::try_from_std_env_var_ok(String::from("DeBuG"));
        assert!(matches!(v, Ok(TracingLevel(types::TracingLevel::Debug))));
    }
    #[test]
    fn tracing_level_parsing_returns_error_for_unknown_value() {
        let er = TracingLevel::try_from_std_env_var_ok(String::from("bad"));
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkTracingLevelEr::AppStateTracingLevelParsing { .. })
        ));
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_bool() {
        let v = EnableApiGitCommitCheck::try_from_std_env_var_ok(String::from("true"));
        assert!(matches!(v, Ok(EnableApiGitCommitCheck(true))));
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_error_for_invalid_bool() {
        let er = EnableApiGitCommitCheck::try_from_std_env_var_ok(String::from("truthy"));
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkEnableApiGitCommitCheckEr::BoolParsing { .. })
        ));
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_usize() {
        let v = MaximumSizeOfHttpBodyInBytes::try_from_std_env_var_ok(String::from("128"));
        assert!(matches!(v, Ok(MaximumSizeOfHttpBodyInBytes(128))));
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_invalid_number() {
        let er = MaximumSizeOfHttpBodyInBytes::try_from_std_env_var_ok(String::from("1k"));
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr::UsizeParsing { .. })
        ));
    }
    #[test]
    fn non_empty_string_parser_returns_error_for_empty_value() {
        let er = super::StartingCheckLink::try_from_std_env_var_ok(String::new());
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkStartingCheckLinkEr::IsEmpty { .. })
        ));
    }
    #[test]
    fn service_socket_address_parsing_returns_socket_addr() {
        let v = ServiceSocketAddress::try_from_std_env_var_ok(String::from("127.0.0.1:3000"));
        assert!(matches!(v, Ok(ServiceSocketAddress(_))));
    }
    #[test]
    fn service_socket_address_parsing_returns_error_for_invalid_addr() {
        let er = ServiceSocketAddress::try_from_std_env_var_ok(String::from("127.0.0.1"));
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkServiceSocketAddressEr::StdNetSocketAddr { .. })
        ));
    }
    #[test]
    fn timezone_parsing_returns_timezone_for_valid_offset() {
        let v = Timezone::try_from_std_env_var_ok(String::from("0"));
        assert!(matches!(v, Ok(Timezone(_))));
    }
    #[test]
    fn timezone_parsing_returns_i32_error_for_non_number() {
        let er = Timezone::try_from_std_env_var_ok(String::from("nan"));
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkTimezoneEr::I32Parsing { .. })
        ));
    }
    #[test]
    fn timezone_parsing_returns_offset_error_when_out_of_range() {
        let er = Timezone::try_from_std_env_var_ok(i32::MAX.to_string());
        assert!(matches!(
            er,
            Err(TryFromStdEnvVarOkTimezoneEr::ChronoFixedOffset { .. })
        ));
    }
}
