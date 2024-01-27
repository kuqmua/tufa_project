#[derive(Debug, Default, PartialEq, Eq, init_from_env::InitFromEnv)]
pub struct ConfigUnchecked {
    server_port: u16,
    hmac_secret: std::string::String,
    base_url: std::string::String,
    access_control_max_age: usize,
    access_control_allow_origin: std::string::String,

    github_name: std::string::String,
    github_token: std::string::String,

    timezone: i32, //for some reason chrono::FixedOffset::east_opt uses i32 but i16 is enough

    redis_url: std::string::String,

    mongo_url: std::string::String,

    database_url: std::string::String, //postgres_url, naming required by sqlx::query::query!

    starting_check_link: std::string::String, //todo add browser url limit check

    tracing_type: crate::server::tracing_type::TracingType,
    source_place_type: crate::common::source_place_type::SourcePlaceType,
    enable_api_git_commit_check: bool,
}

#[derive(
    Debug,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields, //todo - add 2 attributes - for reference\borrow(&) and for value(move)
)]
pub struct Config {
    server_port: crate::common::user_port::UserPort,
    socket_addr: std::string::String,
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

    tracing_type: crate::server::tracing_type::TracingType,
    source_place_type: crate::common::source_place_type::SourcePlaceType,
    enable_api_git_commit_check: bool,
}

impl std::convert::TryFrom<ConfigUnchecked> for Config {
    type Error = ConfigCheckErrorNamed;
    fn try_from(value: ConfigUnchecked) -> Result<Self, Self::Error> {
        let server_port = match crate::common::user_port::UserPort::try_from(value.server_port) {
            Ok(user_port) => user_port,
            Err(e) => {
                return Err(Self::Error::ServerPort {
                    server_port: e,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };
        let socket_addr = format!("127.0.0.1:{server_port}");
        let hmac_secret = match value.hmac_secret.is_empty() {
            true => {
                return Err(Self::Error::HmacSecret {
                    hmac_secret: value.hmac_secret,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
            false => secrecy::Secret::new(value.hmac_secret),
        };
        let base_url = match value.base_url.is_empty() {
            true => {
                return Err(Self::Error::BaseUrl {
                    base_url: value.base_url,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
            false => value.base_url,
        };
        let access_control_max_age = value.access_control_max_age;
        let access_control_allow_origin = match value.access_control_allow_origin.is_empty() {
            true => {
                return Err(Self::Error::AccessControlAllowOrigin {
                    access_control_allow_origin: value.access_control_allow_origin,
                    code_occurence: crate::code_occurence_tufa_common!(),
                }); //todo - maybe add check if its uri\url
            }
            false => value.access_control_allow_origin,
        };

        let github_name = match value.github_name.is_empty() {
            true => {
                return Err(Self::Error::GithubName {
                    github_name: value.github_name,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
            false => value.github_name,
        };
        let github_token = match value.github_token.is_empty() {
            true => {
                return Err(Self::Error::GithubToken {
                    github_token: value.github_token,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
            false => value.github_token,
        };

        let timezone = match chrono::FixedOffset::east_opt(value.timezone) {
            Some(fixed_offset) => fixed_offset,
            None => {
                return Err(Self::Error::Timezone {
                    timezone: value.timezone,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
        };

        let redis_url = match value.redis_url.is_empty() {
            true => {
                return Err(Self::Error::RedisUrl {
                    redis_url: value.redis_url,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
            false => secrecy::Secret::new(value.redis_url),
        };

        let mongo_url = match value.mongo_url.is_empty() {
            true => {
                return Err(Self::Error::MongoUrl {
                    mongo_url: value.mongo_url,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
            false => secrecy::Secret::new(value.mongo_url),
        };

        let database_url = match value.database_url.is_empty() {
            true => {
                return Err(Self::Error::DatabaseUrl {
                    database_url: value.database_url,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
            false => secrecy::Secret::new(value.database_url),
        }; //postgres_url = value.; naming required by sqlx::query::query!

        let starting_check_link = match value.starting_check_link.is_empty() {
            true => {
                return Err(Self::Error::StartingCheckLink {
                    starting_check_link: value.starting_check_link,
                    code_occurence: crate::code_occurence_tufa_common!(),
                });
            }
            false => value.starting_check_link,
        }; //todo add browser url limit check

        let tracing_type = value.tracing_type;
        let source_place_type = value.source_place_type;
        let enable_api_git_commit_check = value.enable_api_git_commit_check;
        Ok(Self {
            server_port,
            socket_addr,
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
        })
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ConfigCheckErrorNamed {
    ServerPort {
        #[eo_error_occurence]
        server_port: crate::common::user_port::UserPortTryFromU16ErrorNamed,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    HmacSecret {
        #[eo_display_with_serialize_deserialize]
        hmac_secret: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    BaseUrl {
        #[eo_display_with_serialize_deserialize]
        base_url: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //
    RequireSsl {
        #[eo_display_with_serialize_deserialize]
        require_ssl: bool,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    AccessControlAllowOrigin {
        #[eo_display_with_serialize_deserialize]
        access_control_allow_origin: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    GithubName {
        #[eo_display_with_serialize_deserialize]
        github_name: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    GithubToken {
        #[eo_display_with_serialize_deserialize]
        github_token: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    Timezone {
        #[eo_display_with_serialize_deserialize]
        timezone: i32,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    RedisUrl {
        #[eo_display_with_serialize_deserialize]
        redis_url: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    MongoUrl {
        #[eo_display_with_serialize_deserialize]
        mongo_url: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    DatabaseUrl {
        #[eo_display_with_serialize_deserialize]
        database_url: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    StartingCheckLink {
        #[eo_display_with_serialize_deserialize]
        starting_check_link: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    TracingType {
        #[eo_display_with_serialize_deserialize]
        tracing_type: crate::server::tracing_type::TracingType,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    SourcePlaceType {
        #[eo_display_with_serialize_deserialize]
        source_place_type: crate::common::source_place_type::SourcePlaceType,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

pub trait GetConfig {
    fn get_config(&self) -> &Config;
}
