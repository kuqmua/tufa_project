pub use generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields;
pub use try_from_env::TryFromEnv;
pub mod types;

pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct ServiceSocketAddress(pub std::net::SocketAddr);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkServiceSocketAddressErrorNamed {
    StdNetSocketAddr { std_net_socket_addr: std::net::AddrParseError },
}
impl TryFromStdEnvVarOk for ServiceSocketAddress {
    type Error = TryFromStdEnvVarOkServiceSocketAddressErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match <std::net::SocketAddr as std::str::FromStr>::from_str(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdNetSocketAddr { std_net_socket_addr: error });
            }
        };
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct Timezone(pub chrono::FixedOffset);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkTimezoneErrorNamed {
    StdPrimitiveI32Parsing { std_primitive_i32_parsing: std::num::ParseIntError },
    ChronoFixedOffset { chrono_fixed_offset: std::string::String },
}
impl TryFromStdEnvVarOk for Timezone {
    type Error = TryFromStdEnvVarOkTimezoneErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<std::primitive::i32>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveI32Parsing { std_primitive_i32_parsing: error });
            }
        };
        let Some(value) = chrono::FixedOffset::east_opt(value) else {
            return Err(Self::Error::ChronoFixedOffset { chrono_fixed_offset: std::string::String::from("not east") });
        };
        Ok(Self(value))
    }
}

#[derive(Debug, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct RedisUrl(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkRedisUrlErrorNamed {
    IsEmpty { is_empty: std::string::String },
}
impl TryFromStdEnvVarOk for RedisUrl {
    type Error = TryFromStdEnvVarOkRedisUrlErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty { is_empty: std::string::String::from("is empty") });
        } else {
            secrecy::Secret::new(value)
        };
        Ok(Self(value))
    }
}

#[derive(Debug, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct MongoUrl(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkMongoUrlErrorNamed {
    IsEmpty { is_empty: std::string::String },
}
impl TryFromStdEnvVarOk for MongoUrl {
    type Error = TryFromStdEnvVarOkMongoUrlErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty { is_empty: std::string::String::from("is empty") });
        } else {
            secrecy::Secret::new(value)
        };
        Ok(Self(value))
    }
}

#[derive(Debug, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct DatabaseUrl(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkDatabaseUrlErrorNamed {
    IsEmpty { is_empty: std::string::String },
}
impl TryFromStdEnvVarOk for DatabaseUrl {
    type Error = TryFromStdEnvVarOkDatabaseUrlErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty { is_empty: std::string::String::from("is empty") });
        } else {
            secrecy::Secret::new(value)
        };
        Ok(Self(value))
    }
}

#[derive(Debug, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct StartingCheckLink(pub std::string::String);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkStartingCheckLinkErrorNamed {
    IsEmpty { is_empty: std::string::String },
}
impl TryFromStdEnvVarOk for StartingCheckLink {
    type Error = TryFromStdEnvVarOkStartingCheckLinkErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty { is_empty: std::string::String::from("is empty") });
        } else {
            value
        };
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct TracingLevel(pub crate::types::TracingLevel);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkTracingLevelErrorNamed {
    AppStateTracingLevelParsing { app_state_tracing_type_parsing: std::string::String },
}
impl TryFromStdEnvVarOk for TracingLevel {
    type Error = TryFromStdEnvVarOkTracingLevelErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<crate::types::TracingLevel>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::AppStateTracingLevelParsing { app_state_tracing_type_parsing: error });
            }
        };
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct SourcePlaceType(pub crate::types::SourcePlaceType);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkSourcePlaceTypeErrorNamed {
    AppStateSourcePlaceTypeParsing { app_state_source_place_type_parsing: std::string::String },
}
impl TryFromStdEnvVarOk for SourcePlaceType {
    type Error = TryFromStdEnvVarOkSourcePlaceTypeErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<crate::types::SourcePlaceType>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::AppStateSourcePlaceTypeParsing { app_state_source_place_type_parsing: error });
            }
        };
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct EnableApiGitCommitCheck(pub std::primitive::bool);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed {
    StdPrimitiveBoolParsing { std_primitive_bool_parsing: std::str::ParseBoolError },
}
impl TryFromStdEnvVarOk for EnableApiGitCommitCheck {
    type Error = TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<std::primitive::bool>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveBoolParsing { std_primitive_bool_parsing: error });
            }
        };
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, generate_getter_traits_for_struct_fields::GenerateGetterTrait)]
pub struct MaximumSizeOfHttpBodyInBytes(pub std::primitive::usize);
#[derive(Debug, thiserror::Error, impl_display_as_debug::ImplDisplayAsDebug)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed {
    StdPrimitiveUsizeParsing { std_primitive_usize_parsing: std::num::ParseIntError },
}
impl TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<std::primitive::usize>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveUsizeParsing { std_primitive_usize_parsing: error });
            }
        };
        Ok(Self(value))
    }
}
