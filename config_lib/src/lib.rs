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
                if v.is_empty() {
                    return Err(Self::Error::IsEmpty {
                        is_empty: String::from("is empty"),
                    });
                }
                Ok(Self(v))
            }
        }
    };
}
macro_rules! impl_try_from_parse {
    ($name:ident, $er_name:ident, $inner:ty, $er_vrt:ident, $er_field:ident, $er_ty:ty, $($derive:ident),*) => {
        #[derive(Debug, $($derive,)* gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct $name(pub $inner);
        #[derive(Debug, Optml)]
        pub enum $er_name {
            $er_vrt { $er_field: $er_ty },
        }
        impl std::fmt::Display for $er_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::$er_vrt { $er_field } => write!(f, "{:?}", $er_field),
                }
            }
        }
        impl std::error::Error for $er_name {}
        impl TryFromStdEnvVarOk for $name {
            type Error = $er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                match v.parse::<$inner>() {
                    Ok(v0) => Ok(Self(v0)),
                    Err(er) => Err(Self::Error::$er_vrt { $er_field: er }),
                }
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
                if v.is_empty() {
                    return Err(Self::Error::IsEmpty {
                        is_empty: String::from("is empty"),
                    });
                }
                Ok(Self(SecretBox::new(Box::new(v))))
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
        <SocketAddr as FromStr>::from_str(&v)
            .map(Self)
            .map_err(|er| Self::Error::StdNetSocketAddr {
                std_net_socket_addr: er,
            })
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
        Ok(Self(match v.parse::<types::SrcPlaceType>() {
            Ok(v0) => v0,
            Err(er) => {
                return Err(Self::Error::AppStateSrcPlaceTypeParsing {
                    app_state_src_place_type_parsing: er,
                });
            }
        }))
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
        let Some(fixed_offset) = FixedOffset::east_opt(match v.parse::<i32>() {
            Ok(v0) => v0,
            Err(er) => {
                return Err(Self::Error::I32Parsing { i32_parsing: er });
            }
        }) else {
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
        Ok(Self(match v.parse::<types::TracingLevel>() {
            Ok(v0) => v0,
            Err(er) => {
                return Err(Self::Error::AppStateTracingLevelParsing {
                    app_state_tracing_type_parsing: er,
                });
            }
        }))
    }
}
