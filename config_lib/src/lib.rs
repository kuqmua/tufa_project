pub mod types;
use chrono::FixedOffset;
use secrecy::SecretBox;
use std::{
    net::{AddrParseError, SocketAddr},
    num::ParseIntError,
    str::{FromStr, ParseBoolError},
};

pub use gen_getter_traits_for_struct_fields::GenGetterTraitsForStructFields;
pub use try_from_env::TryFromEnv;

pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error>;
}
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct ServiceSocketAddress(pub SocketAddr);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkServiceSocketAddressErrorNamed {
    StdNetSocketAddr { std_net_socket_addr: AddrParseError },
}
impl TryFromStdEnvVarOk for ServiceSocketAddress {
    type Error = TryFromStdEnvVarOkServiceSocketAddressErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        <SocketAddr as FromStr>::from_str(&value)
            .map(Self)
            .map_err(|error| Self::Error::StdNetSocketAddr {
                std_net_socket_addr: error,
            })
    }
}
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct Timezone(pub FixedOffset);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkTimezoneErrorNamed {
    ChronoFixedOffset {
        chrono_fixed_offset: String,
    },
    StdPrimitiveI32Parsing {
        std_primitive_i32_parsing: ParseIntError,
    },
}
impl TryFromStdEnvVarOk for Timezone {
    type Error = TryFromStdEnvVarOkTimezoneErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        let Some(fixed_offset) = FixedOffset::east_opt(match value.parse::<i32>() {
            Ok(value_i32) => value_i32,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveI32Parsing {
                    std_primitive_i32_parsing: error,
                });
            }
        }) else {
            return Err(Self::Error::ChronoFixedOffset {
                chrono_fixed_offset: String::from("not east"),
            });
        };
        Ok(Self(fixed_offset))
    }
}
#[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct RedisUrl(pub SecretBox<String>);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkRedisUrlErrorNamed {
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for RedisUrl {
    type Error = TryFromStdEnvVarOkRedisUrlErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: String::from("is empty"),
            });
        } else {
            SecretBox::new(Box::new(value))
        }))
    }
}
#[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct MongoUrl(pub SecretBox<String>);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkMongoUrlErrorNamed {
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for MongoUrl {
    type Error = TryFromStdEnvVarOkMongoUrlErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: String::from("is empty"),
            });
        } else {
            SecretBox::new(Box::new(value))
        }))
    }
}
#[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct DatabaseUrl(pub SecretBox<String>);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkDatabaseUrlErrorNamed {
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for DatabaseUrl {
    type Error = TryFromStdEnvVarOkDatabaseUrlErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: String::from("is empty"),
            });
        } else {
            SecretBox::new(Box::new(value))
        }))
    }
}
#[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct StartingCheckLink(pub String);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkStartingCheckLinkErrorNamed {
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for StartingCheckLink {
    type Error = TryFromStdEnvVarOkStartingCheckLinkErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: String::from("is empty"),
            });
        } else {
            value
        }))
    }
}

#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct TracingLevel(pub types::TracingLevel);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkTracingLevelErrorNamed {
    AppStateTracingLevelParsing {
        app_state_tracing_type_parsing: String,
    },
}
impl TryFromStdEnvVarOk for TracingLevel {
    type Error = TryFromStdEnvVarOkTracingLevelErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(match value.parse::<types::TracingLevel>() {
            Ok(handle) => handle,
            Err(error) => {
                return Err(Self::Error::AppStateTracingLevelParsing {
                    app_state_tracing_type_parsing: error,
                });
            }
        }))
    }
}

#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct SourcePlaceType(pub types::SourcePlaceType);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkSourcePlaceTypeErrorNamed {
    AppStateSourcePlaceTypeParsing {
        app_state_source_place_type_parsing: String,
    },
}
impl TryFromStdEnvVarOk for SourcePlaceType {
    type Error = TryFromStdEnvVarOkSourcePlaceTypeErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(match value.parse::<types::SourcePlaceType>() {
            Ok(handle) => handle,
            Err(error) => {
                return Err(Self::Error::AppStateSourcePlaceTypeParsing {
                    app_state_source_place_type_parsing: error,
                });
            }
        }))
    }
}

#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct EnableApiGitCommitCheck(pub bool);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed {
    StdPrimitiveBoolParsing {
        std_primitive_bool_parsing: ParseBoolError,
    },
}
impl TryFromStdEnvVarOk for EnableApiGitCommitCheck {
    type Error = TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(match value.parse::<bool>() {
            Ok(handle) => handle,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveBoolParsing {
                    std_primitive_bool_parsing: error,
                });
            }
        }))
    }
}

#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait)]
pub struct MaximumSizeOfHttpBodyInBytes(pub usize);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed {
    StdPrimitiveUsizeParsing {
        std_primitive_usize_parsing: ParseIntError,
    },
}
impl TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(match value.parse::<usize>() {
            Ok(handle) => handle,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveUsizeParsing {
                    std_primitive_usize_parsing: error,
                });
            }
        }))
    }
}
