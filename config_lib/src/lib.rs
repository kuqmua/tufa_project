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
pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error>;
}
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
#[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct RedisUrl(pub SecretBox<String>);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkRedisUrlEr {
    #[error("{is_empty:?}")]
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for RedisUrl {
    type Error = TryFromStdEnvVarOkRedisUrlEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        Ok(Self(if v.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: String::from("is empty"),
            });
        } else {
            SecretBox::new(Box::new(v))
        }))
    }
}
#[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct MongoUrl(pub SecretBox<String>);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkMongoUrlEr {
    #[error("{is_empty:?}")]
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for MongoUrl {
    type Error = TryFromStdEnvVarOkMongoUrlEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        Ok(Self(if v.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: String::from("is empty"),
            });
        } else {
            SecretBox::new(Box::new(v))
        }))
    }
}
#[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct DatabaseUrl(pub SecretBox<String>);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkDatabaseUrlEr {
    #[error("{is_empty:?}")]
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for DatabaseUrl {
    type Error = TryFromStdEnvVarOkDatabaseUrlEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        Ok(Self(if v.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: String::from("is empty"),
            });
        } else {
            SecretBox::new(Box::new(v))
        }))
    }
}
#[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct StartingCheckLink(pub String);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkStartingCheckLinkEr {
    #[error("{is_empty:?}")]
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for StartingCheckLink {
    type Error = TryFromStdEnvVarOkStartingCheckLinkEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        Ok(Self(if v.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: String::from("is empty"),
            });
        } else {
            v
        }))
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
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct EnableApiGitCommitCheck(pub bool);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkEnableApiGitCommitCheckEr {
    #[error("{bool_parsing:?}")]
    BoolParsing { bool_parsing: ParseBoolError },
}
impl TryFromStdEnvVarOk for EnableApiGitCommitCheck {
    type Error = TryFromStdEnvVarOkEnableApiGitCommitCheckEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        Ok(Self(match v.parse::<bool>() {
            Ok(v0) => v0,
            Err(er) => {
                return Err(Self::Error::BoolParsing { bool_parsing: er });
            }
        }))
    }
}
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct MaximumSizeOfHttpBodyInBytes(pub usize);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr {
    #[error("{usize_parsing:?}")]
    UsizeParsing { usize_parsing: ParseIntError },
}
impl TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        Ok(Self(match v.parse::<usize>() {
            Ok(v0) => v0,
            Err(er) => {
                return Err(Self::Error::UsizeParsing { usize_parsing: er });
            }
        }))
    }
}
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct PgPoolMaxConnections(pub u32);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkPgPoolMaxConnectionsEr {
    #[error("{u32_parsing:?}")]
    U32Parsing { u32_parsing: ParseIntError },
}
impl TryFromStdEnvVarOk for PgPoolMaxConnections {
    type Error = TryFromStdEnvVarOkPgPoolMaxConnectionsEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        Ok(Self(match v.parse::<u32>() {
            Ok(v0) => v0,
            Err(er) => {
                return Err(Self::Error::U32Parsing { u32_parsing: er });
            }
        }))
    }
}
#[derive(Debug, Clone, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
pub struct CorsAllowOrigin(pub String);
#[derive(Debug, Error, Optml)]
pub enum TryFromStdEnvVarOkCorsAllowOriginEr {
    #[error("{is_empty:?}")]
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for CorsAllowOrigin {
    type Error = TryFromStdEnvVarOkCorsAllowOriginEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        Ok(Self(if v.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: String::from("is empty"),
            });
        } else {
            v
        }))
    }
}
