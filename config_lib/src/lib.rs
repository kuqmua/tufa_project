pub use try_from_env::TryFromEnv;

pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error>;
}

pub trait GetServiceSocketAddress {
    fn get_service_socket_address(&self) -> &ServiceSocketAddressWrapper;
}
#[derive(Debug, Clone, Copy)]
pub struct ServiceSocketAddressWrapper(pub std::net::SocketAddr);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkServiceSocketAddressErrorNamed {
    StdNetSocketAddr {
        std_net_socket_addr: std::net::AddrParseError,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkServiceSocketAddressErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for ServiceSocketAddressWrapper {
    type Error = TryFromStdEnvVarOkServiceSocketAddressErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match <std::net::SocketAddr as std::str::FromStr>::from_str(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdNetSocketAddr {
                    std_net_socket_addr: error,
                });
            }
        };
        Ok(Self(value))
    }
}

pub trait GetTimezone {
    fn get_timezone(&self) -> &TimezoneWrapper;
}
#[derive(Debug, Clone, Copy)]
pub struct TimezoneWrapper(pub chrono::FixedOffset);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkTimezoneErrorNamed {
    StdPrimitiveI32Parsing {
        std_primitive_i32_parsing: std::num::ParseIntError,
    },
    ChronoFixedOffset {
        chrono_fixed_offset: std::string::String,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkTimezoneErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for TimezoneWrapper {
    type Error = TryFromStdEnvVarOkTimezoneErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<std::primitive::i32>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveI32Parsing {
                    std_primitive_i32_parsing: error,
                });
            }
        };
        let value = match chrono::FixedOffset::east_opt(value) {
            Some(value) => value,
            None => {
                return Err(Self::Error::ChronoFixedOffset {
                    chrono_fixed_offset: std::string::String::from("not east"),
                });
            }
        };
        Ok(Self(value))
    }
}

pub trait GetRedisUrl {
    fn get_redis_url(&self) -> &RedisUrlWrapper;
}
#[derive(Debug)]
pub struct RedisUrlWrapper(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkRedisUrlErrorNamed {
    IsEmpty {
        is_empty: std::string::String,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkRedisUrlErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for RedisUrlWrapper {
    type Error = TryFromStdEnvVarOkRedisUrlErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: std::string::String::from("is empty"),
            });
        }
        else {
            secrecy::Secret::new(value)
        };
        Ok(Self(value))
    }
}

pub trait GetMongoUrl {
    fn get_mongo_url(&self) -> &MongoUrlWrapper;
}
#[derive(Debug)]
pub struct MongoUrlWrapper(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkMongoUrlErrorNamed {
    IsEmpty {
        is_empty: std::string::String,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkMongoUrlErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for MongoUrlWrapper {
    type Error = TryFromStdEnvVarOkMongoUrlErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: std::string::String::from("is empty"),
            });
        }
        else {
            secrecy::Secret::new(value)
        };
        Ok(Self(value))
    }
}

pub trait GetDatabaseUrl {
    fn get_database_url(&self) -> &DatabaseUrlWrapper; //postgres database url. required to exists in env
}
#[derive(Debug)]
pub struct DatabaseUrlWrapper(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkDatabaseUrlErrorNamed {
    IsEmpty {
        is_empty: std::string::String,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkDatabaseUrlErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for DatabaseUrlWrapper {
    type Error = TryFromStdEnvVarOkDatabaseUrlErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: std::string::String::from("is empty"),
            });
        }
        else {
            secrecy::Secret::new(value)
        };
        Ok(Self(value))
    }
}

pub trait GetStartingCheckLink {
    fn get_starting_check_link(&self) -> &StartingCheckLinkWrapper;
}
#[derive(Debug)]
pub struct StartingCheckLinkWrapper(pub std::string::String);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkStartingCheckLinkErrorNamed {
    IsEmpty {
        is_empty: std::string::String,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkStartingCheckLinkErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for StartingCheckLinkWrapper {
    type Error = TryFromStdEnvVarOkStartingCheckLinkErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: std::string::String::from("is empty"),
            });
        }
        else {
            value
        };
        Ok(Self(value))
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    strum_macros::EnumIter,
    enum_extension_lib::EnumExtension,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    from_str::FromStr,
)]
pub enum TracingType {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
impl std::default::Default for TracingType {
    fn default() -> Self {
        Self::Error
    }
}
impl std::fmt::Display for TracingType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_snake_case())
    }
}
pub trait GetTracingType {
    fn get_tracing_type(&self) -> &TracingTypeWrapper;
}
#[derive(Debug, Clone, Copy)]
pub struct TracingTypeWrapper(pub TracingType);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkTracingTypeErrorNamed {
    AppStateTracingTypeParsing {
        app_state_tracing_type_parsing: std::string::String,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkTracingTypeErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for TracingTypeWrapper {
    type Error = TryFromStdEnvVarOkTracingTypeErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<TracingType>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::AppStateTracingTypeParsing {
                    app_state_tracing_type_parsing: error,
                });
            }
        };
        Ok(Self(value))
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::Display,
    serde::Serialize,
    serde::Deserialize,
    from_str::FromStr,
)]
pub enum SourcePlaceType {
    Source,
    Github,
}
impl std::default::Default for SourcePlaceType {
    fn default() -> Self {
        Self::Source
    }
}
pub trait GetSourcePlaceType {
    fn get_source_place_type(&self) -> &SourcePlaceTypeWrapper;
}
#[derive(Debug, Clone, Copy)]
pub struct SourcePlaceTypeWrapper(pub SourcePlaceType);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkSourcePlaceTypeErrorNamed {
    AppStateSourcePlaceTypeParsing {
        app_state_source_place_type_parsing: std::string::String,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkSourcePlaceTypeErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for SourcePlaceTypeWrapper {
    type Error = TryFromStdEnvVarOkSourcePlaceTypeErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<SourcePlaceType>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::AppStateSourcePlaceTypeParsing {
                    app_state_source_place_type_parsing: error,
                });
            }
        };
        Ok(Self(value))
    }
}

pub trait GetEnableApiGitCommitCheck {
    fn get_enable_api_git_commit_check(&self) -> &EnableApiGitCommitCheckWrapper;
}
#[derive(Debug, Clone, Copy)]
pub struct EnableApiGitCommitCheckWrapper(pub std::primitive::bool);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed {
    StdPrimitiveBoolParsing {
        std_primitive_bool_parsing: std::str::ParseBoolError,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for EnableApiGitCommitCheckWrapper {
    type Error = TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<std::primitive::bool>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveBoolParsing {
                    std_primitive_bool_parsing: error,
                });
            }
        };
        Ok(Self(value))
    }
}

pub trait GetMaximumSizeOfHttpBodyInBytes {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &MaximumSizeOfHttpBodyInBytesWrapper;
}
#[derive(Debug, Clone, Copy)]
pub struct MaximumSizeOfHttpBodyInBytesWrapper(pub std::primitive::usize);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed {
    StdPrimitiveUsizeParsing {
        std_primitive_usize_parsing: std::num::ParseIntError,
    },
}
impl std::fmt::Display for TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}
impl TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytesWrapper {
    type Error = TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<std::primitive::usize>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveUsizeParsing {
                    std_primitive_usize_parsing: error,
                });
            }
        };
        Ok(Self(value))
    }
}

