pub use generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields;
pub use try_from_env::TryFromEnv;
pub mod types;

pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct ServiceSocketAddress(pub std::net::SocketAddr);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkServiceSocketAddressErrorNamed {
    StdNetSocketAddr { std_net_socket_addr: std::net::AddrParseError },
}
impl TryFromStdEnvVarOk for ServiceSocketAddress {
    type Error = TryFromStdEnvVarOkServiceSocketAddressErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        <std::net::SocketAddr as std::str::FromStr>::from_str(&value)
            .map(Self)
            .map_err(|error| Self::Error::StdNetSocketAddr {
                std_net_socket_addr: error,
            })
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct Timezone(pub chrono::FixedOffset);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkTimezoneErrorNamed {
    StdPrimitiveI32Parsing { std_primitive_i32_parsing: std::num::ParseIntError },
    ChronoFixedOffset { chrono_fixed_offset: String },
}
impl TryFromStdEnvVarOk for Timezone {
    type Error = TryFromStdEnvVarOkTimezoneErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        let Some(fixed_offset) = chrono::FixedOffset::east_opt(
            match value.parse::<i32>() {
                Ok(value_i32) => value_i32,
                Err(error) => {
                    return Err(Self::Error::StdPrimitiveI32Parsing { std_primitive_i32_parsing: error });
                }
            }
        ) else {
            return Err(Self::Error::ChronoFixedOffset { chrono_fixed_offset: String::from("not east") });
        };
        Ok(Self(fixed_offset))
    }
}

#[derive(Debug, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct RedisUrl(pub secrecy::Secret<String>);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkRedisUrlErrorNamed {
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for RedisUrl {
    type Error = TryFromStdEnvVarOkRedisUrlErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(if value.is_empty() {
            return Err(Self::Error::IsEmpty { is_empty: String::from("is empty") });
        } else {
            secrecy::Secret::new(value)
        }))
    }
}

#[derive(Debug, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct MongoUrl(pub secrecy::Secret<String>);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkMongoUrlErrorNamed {
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for MongoUrl {
    type Error = TryFromStdEnvVarOkMongoUrlErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(if value.is_empty() {
            return Err(Self::Error::IsEmpty { is_empty: String::from("is empty") });
        } else {
            secrecy::Secret::new(value)
        }))
    }
}

#[derive(Debug, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct DatabaseUrl(pub secrecy::Secret<String>);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkDatabaseUrlErrorNamed {
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for DatabaseUrl {
    type Error = TryFromStdEnvVarOkDatabaseUrlErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(if value.is_empty() {
            return Err(Self::Error::IsEmpty { is_empty: String::from("is empty") });
        } else {
            secrecy::Secret::new(value)
        }))
    }
}

#[derive(Debug, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct StartingCheckLink(pub String);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkStartingCheckLinkErrorNamed {
    IsEmpty { is_empty: String },
}
impl TryFromStdEnvVarOk for StartingCheckLink {
    type Error = TryFromStdEnvVarOkStartingCheckLinkErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(if value.is_empty() {
            return Err(Self::Error::IsEmpty { is_empty: String::from("is empty") });
        } else {
            value
        }))
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct TracingLevel(pub types::TracingLevel);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkTracingLevelErrorNamed {
    AppStateTracingLevelParsing { app_state_tracing_type_parsing: String },
}
impl TryFromStdEnvVarOk for TracingLevel {
    type Error = TryFromStdEnvVarOkTracingLevelErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(match value.parse::<types::TracingLevel>() {
            Ok(handle) => handle,
            Err(error) => {
                return Err(Self::Error::AppStateTracingLevelParsing { app_state_tracing_type_parsing: error });
            }
        }))
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct SourcePlaceType(pub types::SourcePlaceType);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkSourcePlaceTypeErrorNamed {
    AppStateSourcePlaceTypeParsing { app_state_source_place_type_parsing: String },
}
impl TryFromStdEnvVarOk for SourcePlaceType {
    type Error = TryFromStdEnvVarOkSourcePlaceTypeErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(match value.parse::<types::SourcePlaceType>() {
            Ok(handle) => handle,
            Err(error) => {
                return Err(Self::Error::AppStateSourcePlaceTypeParsing { app_state_source_place_type_parsing: error });
            }
        }))
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct EnableApiGitCommitCheck(pub bool);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed {
    StdPrimitiveBoolParsing { std_primitive_bool_parsing: std::str::ParseBoolError },
}
impl TryFromStdEnvVarOk for EnableApiGitCommitCheck {
    type Error = TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(match value.parse::<bool>() {
            Ok(handle) => handle,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveBoolParsing { std_primitive_bool_parsing: error });
            }
        }))
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct MaximumSizeOfHttpBodyInBytes(pub usize);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed {
    StdPrimitiveUsizeParsing { std_primitive_usize_parsing: std::num::ParseIntError },
}
impl TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed;
    fn try_from_std_env_var_ok(value: String) -> Result<Self, Self::Error> {
        Ok(Self(match value.parse::<usize>() {
            Ok(handle) => handle,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveUsizeParsing { std_primitive_usize_parsing: error });
            }
        }))
    }
}
