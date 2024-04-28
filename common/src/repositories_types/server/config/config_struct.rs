
pub use config_lib::TryFromEnv;
pub use generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFieldsHandle;
#[derive(Debug, 
    // config_lib::TryFromEnv,
    // generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFieldsHandle
)]
pub struct Config {
    //todo maybe auto generate .env and docker-compose environment variables. and maybe write in directly into files
    // service_socket_address: config_lib::ServiceSocketAddressWrapper,
    service_socket_address: std::net::SocketAddr,
    
    // timezone: config_lib::TimezoneWrapper, //for some reason chrono::FixedOffset::east_opt uses i32 but i16 is enough
    timezone: chrono::FixedOffset,

    // redis_url: config_lib::RedisUrlWrapper,
    redis_url: secrecy::Secret<std::string::String>,
    
    // mongo_url: config_lib::MongoUrlWrapper,
    mongo_url: secrecy::Secret<std::string::String>,
    
    // database_url: config_lib::DatabaseUrlWrapper, //postgres_url, naming required by sqlx::query::query!
    database_url: secrecy::Secret<std::string::String>,
    
    // starting_check_link: config_lib::StartingCheckLinkWrapper, //todo add browser url limit check
    starting_check_link: std::string::String,
    
    // tracing_type: config_lib::TracingTypeWrapper,
    tracing_type: config_lib::TracingType,
    // source_place_type: config_lib::SourcePlaceTypeWrapper,
    source_place_type: config_lib::SourcePlaceType,
    // enable_api_git_commit_check: config_lib::EnableApiGitCommitCheckWrapper,
    enable_api_git_commit_check: std::primitive::bool,
    // maximum_size_of_http_body_in_bytes: config_lib::MaximumSizeOfHttpBodyInBytesWrapper,
    maximum_size_of_http_body_in_bytes: std::primitive::usize,
}

////////////
#[derive(Debug, thiserror :: Error)]
pub enum ConfigTryFromEnvErrorNamed {
    Dotenv {
        dotenv: dotenv::Error,
    },
    StdEnvVarError {
        std_env_var_error: std::env::VarError,
        env_var_name: std::string::String,
    },
    ServiceSocketAddress {
        service_socket_address: config_lib::TryFromStdEnvVarOkServiceSocketAddressErrorNamed,
    },
    Timezone {
        timezone: config_lib::TryFromStdEnvVarOkTimezoneErrorNamed,
    },
    RedisUrl {
        redis_url: config_lib::TryFromStdEnvVarOkRedisUrlErrorNamed,
    },
    MongoUrl {
        mongo_url: config_lib::TryFromStdEnvVarOkMongoUrlErrorNamed,
    },
    DatabaseUrl {
        database_url: config_lib::TryFromStdEnvVarOkDatabaseUrlErrorNamed,
    },
    StartingCheckLink {
        starting_check_link: config_lib::TryFromStdEnvVarOkStartingCheckLinkErrorNamed,
    },
    TracingType {
        tracing_type: config_lib::TryFromStdEnvVarOkTracingTypeErrorNamed,
    },
    SourcePlaceType {
        source_place_type: config_lib::TryFromStdEnvVarOkSourcePlaceTypeErrorNamed,
    },
    EnableApiGitCommitCheck {
        enable_api_git_commit_check:
            config_lib::TryFromStdEnvVarOkEnableApiGitCommitCheckErrorNamed,
    },
    MaximumSizeOfHttpBodyInBytes {
        maximum_size_of_http_body_in_bytes:
            config_lib::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesErrorNamed,
    },
}
impl std::fmt::Display for ConfigTryFromEnvErrorNamed {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dotenv { dotenv } => write!(formatter, "{}", dotenv),
            Self::StdEnvVarError {
                std_env_var_error,
                env_var_name,
            } => write!(formatter, "{} {}", std_env_var_error, env_var_name),
            Self::ServiceSocketAddress {
                service_socket_address,
            } => write!(formatter, "{}", service_socket_address),
            Self::Timezone { timezone } => write!(formatter, "{}", timezone),
            Self::RedisUrl { redis_url } => write!(formatter, "{}", redis_url),
            Self::MongoUrl { mongo_url } => write!(formatter, "{}", mongo_url),
            Self::DatabaseUrl { database_url } => write!(formatter, "{}", database_url),
            Self::StartingCheckLink {
                starting_check_link,
            } => write!(formatter, "{}", starting_check_link),
            Self::TracingType { tracing_type } => write!(formatter, "{}", tracing_type),
            Self::SourcePlaceType { source_place_type } => {
                write!(formatter, "{}", source_place_type)
            }
            Self::EnableApiGitCommitCheck {
                enable_api_git_commit_check,
            } => write!(formatter, "{}", enable_api_git_commit_check),
            Self::MaximumSizeOfHttpBodyInBytes {
                maximum_size_of_http_body_in_bytes,
            } => write!(formatter, "{}", maximum_size_of_http_body_in_bytes),
        }
    }
}
impl Config {
    pub fn try_from_env() -> Result<Self, ConfigTryFromEnvErrorNamed> {
        if let Err(error) = dotenv::dotenv() {
            return Err(ConfigTryFromEnvErrorNamed::Dotenv { dotenv: error });
        }
        // let service_socket_address: config_lib::ServiceSocketAddressWrapper = {
        //     let env_var_name = std::string::String::from("SERVICE_SOCKET_ADDRESS");
        //     match std::env::var(&env_var_name) {
        //         Err(error) => {
        //             return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
        //                 std_env_var_error: error,
        //                 env_var_name,
        //             });
        //         }
        //         Ok(value) => match config_lib::TryFromStdEnvVarOk::try_from_std_env_var_ok(value) {
        //             Err(error) => {
        //                 return Err(ConfigTryFromEnvErrorNamed::ServiceSocketAddress {
        //                     service_socket_address: error,
        //                 });
        //             }
        //             Ok(value) => value,
        //         },
        //     }
        // };
        // let timezone: config_lib::TimezoneWrapper = {
        //     let env_var_name = std::string::String::from("TIMEZONE");
        //     match std::env::var(&env_var_name) {
        //         Err(error) => {
        //             return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
        //                 std_env_var_error: error,
        //                 env_var_name,
        //             });
        //         }
        //         Ok(value) => match config_lib::TryFromStdEnvVarOk::try_from_std_env_var_ok(value) {
        //             Err(error) => {
        //                 return Err(ConfigTryFromEnvErrorNamed::Timezone { timezone: error });
        //             }
        //             Ok(value) => value,
        //         },
        //     }
        // };
        // let redis_url: config_lib::RedisUrlWrapper = {
        //     let env_var_name = std::string::String::from("REDIS_URL");
        //     match std::env::var(&env_var_name) {
        //         Err(error) => {
        //             return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
        //                 std_env_var_error: error,
        //                 env_var_name,
        //             });
        //         }
        //         Ok(value) => match config_lib::TryFromStdEnvVarOk::try_from_std_env_var_ok(value) {
        //             Err(error) => {
        //                 return Err(ConfigTryFromEnvErrorNamed::RedisUrl { redis_url: error });
        //             }
        //             Ok(value) => value,
        //         },
        //     }
        // };
        // let mongo_url: config_lib::MongoUrlWrapper = {
        //     let env_var_name = std::string::String::from("MONGO_URL");
        //     match std::env::var(&env_var_name) {
        //         Err(error) => {
        //             return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
        //                 std_env_var_error: error,
        //                 env_var_name,
        //             });
        //         }
        //         Ok(value) => match config_lib::TryFromStdEnvVarOk::try_from_std_env_var_ok(value) {
        //             Err(error) => {
        //                 return Err(ConfigTryFromEnvErrorNamed::MongoUrl { mongo_url: error });
        //             }
        //             Ok(value) => value,
        //         },
        //     }
        // };
        // let database_url: config_lib::DatabaseUrlWrapper = {
        //     let env_var_name = std::string::String::from("DATABASE_URL");
        //     match std::env::var(&env_var_name) {
        //         Err(error) => {
        //             return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
        //                 std_env_var_error: error,
        //                 env_var_name,
        //             });
        //         }
        //         Ok(value) => match config_lib::TryFromStdEnvVarOk::try_from_std_env_var_ok(value) {
        //             Err(error) => {
        //                 return Err(ConfigTryFromEnvErrorNamed::DatabaseUrl {
        //                     database_url: error,
        //                 });
        //             }
        //             Ok(value) => value,
        //         },
        //     }
        // };
        // let starting_check_link: config_lib::StartingCheckLinkWrapper = {
        //     let env_var_name = std::string::String::from("STARTING_CHECK_LINK");
        //     match std::env::var(&env_var_name) {
        //         Err(error) => {
        //             return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
        //                 std_env_var_error: error,
        //                 env_var_name,
        //             });
        //         }
        //         Ok(value) => match config_lib::TryFromStdEnvVarOk::try_from_std_env_var_ok(value) {
        //             Err(error) => {
        //                 return Err(ConfigTryFromEnvErrorNamed::StartingCheckLink {
        //                     starting_check_link: error,
        //                 });
        //             }
        //             Ok(value) => value,
        //         },
        //     }
        // };
        // let tracing_type: config_lib::TracingTypeWrapper = {
        //     let env_var_name = std::string::String::from("TRACING_TYPE");
        //     match std::env::var(&env_var_name) {
        //         Err(error) => {
        //             return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
        //                 std_env_var_error: error,
        //                 env_var_name,
        //             });
        //         }
        //         Ok(value) => match config_lib::TryFromStdEnvVarOk::try_from_std_env_var_ok(value) {
        //             Err(error) => {
        //                 return Err(ConfigTryFromEnvErrorNamed::TracingType {
        //                     tracing_type: error,
        //                 });
        //             }
        //             Ok(value) => value,
        //         },
        //     }
        // };
        // let source_place_type: config_lib::SourcePlaceTypeWrapper = {
        //     let env_var_name = std::string::String::from("SOURCE_PLACE_TYPE");
        //     match std::env::var(&env_var_name) {
        //         Err(error) => {
        //             return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
        //                 std_env_var_error: error,
        //                 env_var_name,
        //             });
        //         }
        //         Ok(value) => match config_lib::TryFromStdEnvVarOk::try_from_std_env_var_ok(value) {
        //             Err(error) => {
        //                 return Err(ConfigTryFromEnvErrorNamed::SourcePlaceType {
        //                     source_place_type: error,
        //                 });
        //             }
        //             Ok(value) => value,
        //         },
        //     }
        // };
        // let enable_api_git_commit_check: config_lib::EnableApiGitCommitCheckWrapper = {
        //     let env_var_name = std::string::String::from("ENABLE_API_GIT_COMMIT_CHECK");
        //     match std::env::var(&env_var_name) {
        //         Err(error) => {
        //             return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
        //                 std_env_var_error: error,
        //                 env_var_name,
        //             });
        //         }
        //         Ok(value) => match config_lib::TryFromStdEnvVarOk::try_from_std_env_var_ok(value) {
        //             Err(error) => {
        //                 return Err(ConfigTryFromEnvErrorNamed::EnableApiGitCommitCheck {
        //                     enable_api_git_commit_check: error,
        //                 });
        //             }
        //             Ok(value) => value,
        //         },
        //     }
        // };
        let maximum_size_of_http_body_in_bytes = {
            let env_var_name = std::string::String::from("MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES");
            match std::env::var(&env_var_name) {
                Err(error) => {
                    return Err(ConfigTryFromEnvErrorNamed::StdEnvVarError {
                        std_env_var_error: error,
                        env_var_name,
                    });
                }
                Ok(value) => match <config_lib::MaximumSizeOfHttpBodyInBytesWrapper as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(value) {
                    Err(error) => {
                        return Err(ConfigTryFromEnvErrorNamed::MaximumSizeOfHttpBodyInBytes {
                            maximum_size_of_http_body_in_bytes: error,
                        });
                    }
                    Ok(value) => value.0,
                },
            }
        };
        todo!()
        // Ok(Self {
        //     service_socket_address,
        //     timezone,
        //     redis_url,
        //     mongo_url,
        //     database_url,
        //     starting_check_link,
        //     tracing_type,
        //     source_place_type,
        //     enable_api_git_commit_check,
        //     maximum_size_of_http_body_in_bytes,
        // })
    }
}
impl app_state::GetServiceSocketAddress for Config {
    fn get_service_socket_address(&self) -> &std::net::SocketAddr {
        &self.service_socket_address
    }
}
impl app_state::GetServiceSocketAddress for &Config {
    fn get_service_socket_address(&self) -> &std::net::SocketAddr {
        &self.service_socket_address
    }
}
impl app_state::GetTimezone for Config {
    fn get_timezone(&self) -> &chrono::FixedOffset {
        &self.timezone
    }
}
impl app_state::GetTimezone for &Config {
    fn get_timezone(&self) -> &chrono::FixedOffset {
        &self.timezone
    }
}
impl app_state::GetRedisUrl for Config {
    fn get_redis_url(&self) -> &secrecy::Secret<std::string::String> {
        &self.redis_url
    }
}
impl app_state::GetRedisUrl for &Config {
    fn get_redis_url(&self) -> &secrecy::Secret<std::string::String> {
        &self.redis_url
    }
}
impl app_state::GetMongoUrl for Config {
    fn get_mongo_url(&self) -> &secrecy::Secret<std::string::String> {
        &self.mongo_url
    }
}
impl app_state::GetMongoUrl for &Config {
    fn get_mongo_url(&self) -> &secrecy::Secret<std::string::String> {
        &self.mongo_url
    }
}
impl app_state::GetDatabaseUrl for Config {
    fn get_database_url(&self) -> &secrecy::Secret<std::string::String> {
        &self.database_url
    }
}
impl app_state::GetDatabaseUrl for &Config {
    fn get_database_url(&self) -> &secrecy::Secret<std::string::String> {
        &self.database_url
    }
}
impl app_state::GetStartingCheckLink for Config {
    fn get_starting_check_link(&self) -> &std::string::String {
        &self.starting_check_link
    }
}
impl app_state::GetStartingCheckLink for &Config {
    fn get_starting_check_link(&self) -> &std::string::String {
        &self.starting_check_link
    }
}
impl app_state::GetTracingType for Config {
    fn get_tracing_type(&self) -> &config_lib::TracingType {
        &self.tracing_type
    }
}
impl app_state::GetTracingType for &Config {
    fn get_tracing_type(&self) -> &config_lib::TracingType {
        &self.tracing_type
    }
}
impl app_state::GetSourcePlaceType for Config {
    fn get_source_place_type(&self) -> &config_lib::SourcePlaceType {
        &self.source_place_type
    }
}
impl app_state::GetSourcePlaceType for &Config {
    fn get_source_place_type(&self) -> &config_lib::SourcePlaceType {
        &self.source_place_type
    }
}
impl app_state::GetEnableApiGitCommitCheck for Config {
    fn get_enable_api_git_commit_check(&self) -> &std::primitive::bool {
        &self.enable_api_git_commit_check
    }
}
impl app_state::GetEnableApiGitCommitCheck for &Config {
    fn get_enable_api_git_commit_check(&self) -> &std::primitive::bool {
        &self.enable_api_git_commit_check
    }
}
impl app_state::GetMaximumSizeOfHttpBodyInBytes for Config {
    fn get_maximum_size_of_http_body_in_bytes(
        &self,
    ) -> &std::primitive::usize {
        &self.maximum_size_of_http_body_in_bytes
    }
}
impl app_state::GetMaximumSizeOfHttpBodyInBytes for &Config {
    fn get_maximum_size_of_http_body_in_bytes(
        &self,
    ) -> &std::primitive::usize {
        &self.maximum_size_of_http_body_in_bytes
    }
}
