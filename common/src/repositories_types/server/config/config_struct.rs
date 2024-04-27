#[derive(
    Debug,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields, //todo - add 2 attributes - for reference\borrow(&) and for value(move)
)]
pub struct Config {
    service_socket_address: std::net::SocketAddr,

    timezone: chrono::FixedOffset,

    redis_url: secrecy::Secret<std::string::String>,

    mongo_url: secrecy::Secret<std::string::String>,

    database_url: secrecy::Secret<std::string::String>, //postgres_url, naming required by sqlx::query::query!

    starting_check_link: std::string::String, //todo add browser url limit check

    tracing_type: app_state::TracingType,
    source_place_type: app_state::SourcePlaceType,
    enable_api_git_commit_check: std::primitive::bool,
    maximum_size_of_http_body_in_bytes: std::primitive::usize,
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum ConfigCheckErrorNamed {
    ServiceSocketAddress {
        #[eo_display]
        server_port: std::net::AddrParseError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    //
    RequireSsl {
        #[eo_display_with_serialize_deserialize]
        require_ssl: bool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Timezone {
        #[eo_display_with_serialize_deserialize]
        timezone: i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RedisUrl {
        #[eo_display_with_serialize_deserialize]
        redis_url: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MongoUrl {
        #[eo_display_with_serialize_deserialize]
        mongo_url: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DatabaseUrl {
        #[eo_display_with_serialize_deserialize]
        database_url: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    StartingCheckLink {
        #[eo_display_with_serialize_deserialize]
        starting_check_link: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TracingType {
        #[eo_display_with_serialize_deserialize]
        tracing_type: app_state::TracingType,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SourcePlaceType {
        #[eo_display_with_serialize_deserialize]
        source_place_type: app_state::SourcePlaceType,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl Config {
    pub fn try_from_env() -> Result<Self, ConfigCheckErrorNamed> {
        #[derive(Debug, Default, PartialEq, Eq, config_lib::InitFromEnv)]
        struct ConfigUnchecked {
            //todo maybe auto generate .env and docker-compose environment variables. and maybe write in directly into files
            service_socket_address: std::string::String,
        
            timezone: std::primitive::i32, //for some reason chrono::FixedOffset::east_opt uses i32 but i16 is enough
        
            redis_url: std::string::String,
        
            mongo_url: std::string::String,
        
            database_url: std::string::String, //postgres_url, naming required by sqlx::query::query!
        
            starting_check_link: std::string::String, //todo add browser url limit check
        
            tracing_type: app_state::TracingType,
            source_place_type: app_state::SourcePlaceType,
            enable_api_git_commit_check: std::primitive::bool,
            maximum_size_of_http_body_in_bytes: std::primitive::usize,
        }
        ///////////////

        ////////////////
        let value = ConfigUnchecked::new().unwrap_or_else(|error| panic!("failed to ConfigUnchecked::new(), reason: {error:#?}"));
        // Self::try_from(value).unwrap_or_else(|error| panic!("failed to Config try_from ConfigUnchecked, reason: {error}"))
        let service_socket_address =  match <std::net::SocketAddr as std::str::FromStr>::from_str(&value.service_socket_address) {
            Ok(value) => value,
            Err(error) => {
                return Err(ConfigCheckErrorNamed::ServiceSocketAddress {
                    server_port: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        let timezone = match chrono::FixedOffset::east_opt(value.timezone) {
            Some(fixed_offset) => fixed_offset,
            None => {
                return Err(ConfigCheckErrorNamed::Timezone {
                    timezone: value.timezone,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        let redis_url = if value.redis_url.is_empty() {
            return Err(ConfigCheckErrorNamed::RedisUrl {
                redis_url: value.redis_url,
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            secrecy::Secret::new(value.redis_url)
        };
        let mongo_url = if value.mongo_url.is_empty() {
            return Err(ConfigCheckErrorNamed::MongoUrl {
                mongo_url: value.mongo_url,
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            secrecy::Secret::new(value.mongo_url)
        };
        let database_url = if value.database_url.is_empty() {
            return Err(ConfigCheckErrorNamed::DatabaseUrl {
                database_url: value.database_url,
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            secrecy::Secret::new(value.database_url)
        }; //postgres_url = value.; naming required by sqlx::query::query!
        let starting_check_link = if value.starting_check_link.is_empty() {
            return Err(ConfigCheckErrorNamed::StartingCheckLink {
                starting_check_link: value.starting_check_link,
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            value.starting_check_link
        }; //todo add browser url limit check
        let tracing_type = value.tracing_type;
        let source_place_type = value.source_place_type;
        let enable_api_git_commit_check = value.enable_api_git_commit_check;
        let maximum_size_of_http_body_in_bytes = value.maximum_size_of_http_body_in_bytes;
        Ok(Self {
            service_socket_address,
        
            timezone,
        
            redis_url,
        
            mongo_url,
        
            database_url, //postgres_url, naming required by sqlx::query::query!
        
            starting_check_link, //todo add browser url limit check
        
            tracing_type,
            source_place_type,
            enable_api_git_commit_check,
            maximum_size_of_http_body_in_bytes,
        })
    }
}

pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error>;
}


#[derive(Debug, Clone, Copy)]
pub struct ServiceSocketAddress(pub std::net::SocketAddr);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkServiceSocketAddressErrorNamed {
    StdNetSocketAddr {
        #[eo_display]
        std_net_socket_addr: std::net::AddrParseError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for ServiceSocketAddress {
    type Error = TryFromStdEnvVarOkServiceSocketAddressErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match <std::net::SocketAddr as std::str::FromStr>::from_str(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdNetSocketAddr {
                    std_net_socket_addr: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Timezone(pub chrono::FixedOffset);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkTimezoneErrorNamed {
    StdPrimitiveI32Parsing {
        #[eo_display]
        std_primitive_i32_parsing: std::num::ParseIntError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ChronoFixedOffset {
        #[eo_display_with_serialize_deserialize]
        chrono_fixed_offset: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for Timezone {
    type Error = TryFromStdEnvVarOkTimezoneErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<std::primitive::i32>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveI32Parsing {
                    std_primitive_i32_parsing: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        let value = match chrono::FixedOffset::east_opt(value) {
            Some(value) => value,
            None => {
                return Err(Self::Error::ChronoFixedOffset {
                    chrono_fixed_offset: std::string::String::from("not east"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        Ok(Self(value))
    }
}
#[derive(Debug)]
pub struct RedisUrl(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkRedisUrlErrorNamed {
    IsEmpty {
        #[eo_display_with_serialize_deserialize]
        is_empty: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for RedisUrl {
    type Error = TryFromStdEnvVarOkRedisUrlErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: std::string::String::from("is empty"),
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            secrecy::Secret::new(value)
        };
        Ok(Self(value))
    }
}
#[derive(Debug)]
pub struct MongoUrl(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkMongoUrlErrorNamed {
    IsEmpty {
        #[eo_display_with_serialize_deserialize]
        is_empty: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for MongoUrl {
    type Error = TryFromStdEnvVarOkMongoUrlErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: std::string::String::from("is empty"),
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            secrecy::Secret::new(value)
        };
        Ok(Self(value))
    }
}
#[derive(Debug)]
pub struct DatabaseUrl(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkDatabaseUrlErrorNamed {
    IsEmpty {
        #[eo_display_with_serialize_deserialize]
        is_empty: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for DatabaseUrl {
    type Error = TryFromStdEnvVarOkDatabaseUrlErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: std::string::String::from("is empty"),
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            secrecy::Secret::new(value)
        };
        Ok(Self(value))
    }
}
#[derive(Debug)]
pub struct StartingCheckLink(pub std::string::String);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkStartingCheckLinkErrorNamed {
    IsEmpty {
        #[eo_display_with_serialize_deserialize]
        is_empty: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for StartingCheckLink {
    type Error = TryFromStdEnvVarOkStartingCheckLinkErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = if value.is_empty() {
            return Err(Self::Error::IsEmpty {
                is_empty: std::string::String::from("is empty"),
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            value
        };
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TracingType(pub app_state::TracingType);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkTracingTypeErrorNamed {
    AppStateTracingTypeParsing {
        #[eo_display_with_serialize_deserialize]
        app_state_tracing_type_parsing: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for TracingType {
    type Error = TryFromStdEnvVarOkTracingTypeErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<app_state::TracingType>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::AppStateTracingTypeParsing {
                    app_state_tracing_type_parsing: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SourcePlaceType(pub app_state::SourcePlaceType);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkSourcePlaceTypeErrorNamed {
    AppStateSourcePlaceTypeParsing {
        #[eo_display_with_serialize_deserialize]
        app_state_source_place_type_parsing: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for SourcePlaceType {
    type Error = TryFromStdEnvVarOkSourcePlaceTypeErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<app_state::SourcePlaceType>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::AppStateSourcePlaceTypeParsing {
                    app_state_source_place_type_parsing: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EnableApiGitCommitCheck(pub std::primitive::bool);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed {
    StdPrimitiveBoolParsing {
        #[eo_display]
        std_primitive_bool_parsing: std::str::ParseBoolError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for EnableApiGitCommitCheck {
    type Error = TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<std::primitive::bool>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveBoolParsing {
                    std_primitive_bool_parsing: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MaximumSizeOfHttpBodyInBytes(pub std::primitive::usize);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed {
    StdPrimitiveUsizeParsing {
        #[eo_display]
        std_primitive_usize_parsing: std::num::ParseIntError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        let value = match value.parse::<std::primitive::usize>() {
            Ok(value) => value,
            Err(error) => {
                return Err(Self::Error::StdPrimitiveUsizeParsing {
                    std_primitive_usize_parsing: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
        };
        Ok(Self(value))
    }
}

/////////////
        #[derive(Debug, config_lib::TryFromEnv)]
        struct ConfigUnchecked {
            //todo maybe auto generate .env and docker-compose environment variables. and maybe write in directly into files
            service_socket_address: ServiceSocketAddress,
        
            timezone: Timezone, //for some reason chrono::FixedOffset::east_opt uses i32 but i16 is enough
        
            redis_url: RedisUrl,
        
            mongo_url: MongoUrl,
        
            database_url: DatabaseUrl, //postgres_url, naming required by sqlx::query::query!
        
            starting_check_link: StartingCheckLink, //todo add browser url limit check
        
            tracing_type: TracingType,
            source_place_type: SourcePlaceType,
            enable_api_git_commit_check: EnableApiGitCommitCheck,
            maximum_size_of_http_body_in_bytes: MaximumSizeOfHttpBodyInBytes,
        }