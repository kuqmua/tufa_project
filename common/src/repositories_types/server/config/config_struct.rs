#[derive(
    Debug,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields, //todo - add 2 attributes - for reference\borrow(&) and for value(move)
)]
pub struct Config {
    service_socket_address: std::net::SocketAddr,
    hmac_secret: secrecy::Secret<std::string::String>,
    base_url: std::string::String,
    access_control_max_age: usize,
    access_control_allow_origin: std::string::String,

    github_name: std::string::String,
    github_token: std::string::String,

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
    // ConfigUnchecked {
    //     #[eo_display]
    //     config_unchecked_error: ConfigUncheckedError,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    ServiceSocketAddress {
        #[eo_display]
        server_port: std::net::AddrParseError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HmacSecret {
        #[eo_display_with_serialize_deserialize]
        hmac_secret: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BaseUrl {
        #[eo_display_with_serialize_deserialize]
        base_url: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    //
    RequireSsl {
        #[eo_display_with_serialize_deserialize]
        require_ssl: bool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    AccessControlAllowOrigin {
        #[eo_display_with_serialize_deserialize]
        access_control_allow_origin: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    GithubName {
        #[eo_display_with_serialize_deserialize]
        github_name: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    GithubToken {
        #[eo_display_with_serialize_deserialize]
        github_token: std::string::String,
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
            hmac_secret: std::string::String,
            base_url: std::string::String,
            access_control_max_age: std::primitive::usize,
            access_control_allow_origin: std::string::String,
        
            github_name: std::string::String,
            github_token: std::string::String,
        
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
        let hmac_secret = if value.hmac_secret.is_empty() {
            return Err(ConfigCheckErrorNamed::HmacSecret {
                hmac_secret: value.hmac_secret,
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            secrecy::Secret::new(value.hmac_secret)
        };
        let base_url = if value.base_url.is_empty() {
            return Err(ConfigCheckErrorNamed::BaseUrl {
                base_url: value.base_url,
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            value.base_url
        };
        let access_control_max_age = value.access_control_max_age;
        let access_control_allow_origin = if value.access_control_allow_origin.is_empty() {
            return Err(ConfigCheckErrorNamed::AccessControlAllowOrigin {
                access_control_allow_origin: value.access_control_allow_origin,
                code_occurence: error_occurence_lib::code_occurence!(),
            }); //todo - maybe add check if its uri\url
        }
        else {
            value.access_control_allow_origin
        };
        
        let github_name = if value.github_name.is_empty() {
            return Err(ConfigCheckErrorNamed::GithubName {
                github_name: value.github_name,
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            value.github_name
        };
        let github_token = if value.github_token.is_empty() {
            return Err(ConfigCheckErrorNamed::GithubToken {
                github_token: value.github_token,
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        else {
            value.github_token
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
            hmac_secret,
            base_url,
            access_control_max_age,
            access_control_allow_origin,
        
            github_name,
            github_token,
        
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

pub trait FromStdEnvVarOkHandle: Sized {
    fn from_std_env_var_ok_handle(value: std::string::String) -> Self;
}

pub trait TryFromStdEnvVarOkHandle: Sized {
    type Error;
    fn try_from_std_env_var_ok_handle(value: std::string::String) -> Result<Self, Self::Error>;
}

pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct ServiceSocketAddress(pub std::net::SocketAddr);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkServiceSocketAddressErrorNamed {
    ServiceSocketAddress {
        #[eo_display]
        service_socket_address: std::net::AddrParseError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for ServiceSocketAddress {
    type Error = TryFromStdEnvVarOkServiceSocketAddressErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        match <std::net::SocketAddr as std::str::FromStr>::from_str(&value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(Self::Error::ServiceSocketAddress {
                service_socket_address: error,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
#[derive(Debug)]
pub struct HmacSecret(pub secrecy::Secret<std::string::String>);
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryFromStdEnvVarOkHmacSecretErrorNamed {
    HmacSecret {
        #[eo_display_with_serialize_deserialize]
        hmac_secret: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TryFromStdEnvVarOk for secrecy::Secret<std::string::String> {
    type Error = TryFromStdEnvVarOkHmacSecretErrorNamed;
    fn try_from_std_env_var_ok(value: std::string::String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(Self::Error::HmacSecret {
                hmac_secret: value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
        else {
            Ok(Self::new(value))
        }
    }
}
#[derive(Debug)]
pub struct BaseUrl(pub std::string::String);
#[derive(Debug, Clone, Copy)]
pub struct AccessControlMaxAge(pub std::primitive::usize);
#[derive(Debug)]
pub struct AccessControlAllowOrigin(pub std::string::String);
#[derive(Debug)]
pub struct GithubName(pub std::string::String);
#[derive(Debug)]
pub struct GithubToken(pub std::string::String);
#[derive(Debug, Clone, Copy)]
pub struct Timezone(pub chrono::FixedOffset);
#[derive(Debug)]
pub struct RedisUrl(pub secrecy::Secret<std::string::String>);
#[derive(Debug)]
pub struct MongoUrl(pub secrecy::Secret<std::string::String>);
#[derive(Debug)]
pub struct DatabaseUrl(pub secrecy::Secret<std::string::String>);
#[derive(Debug)]
pub struct StartingCheckLink(pub std::string::String);
#[derive(Debug, Clone, Copy)]
pub struct TracingType(pub app_state::TracingType);
#[derive(Debug, Clone, Copy)]
pub struct SourcePlaceType(pub app_state::SourcePlaceType);
#[derive(Debug, Clone, Copy)]
pub struct EnableApiGitCommitCheck(pub std::primitive::bool);
#[derive(Debug, Clone, Copy)]
pub struct MaximumSizeOfHttpBodyInBytes(pub std::primitive::usize);