#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, getset::Getters)]
pub struct ServerPort {
    #[getset(get = "pub")]
    port: std::primitive::u16,
}
impl std::fmt::Display for ServerPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.port)
    }
}
impl std::convert::TryFrom<std::primitive::u16> for ServerPort {
    type Error = std::primitive::u16;
    fn try_from(value: std::primitive::u16) -> Result<Self, Self::Error> {
        if value < 1024 {
            Err(value)
        } else if value < 49152 {
            Ok(Self { port: value })
        } else {
            Err(value)
        }
    }
}
// macro_rules! user_port_try_from_u16_with_possible_runtime_panic{
//     ($possible_port:expr) => {
//         if $possible_port < 1024 {
//             panic!("failed to user_port_try_from_u16!(), reason: system port range 0-1023");
//         }
//         else if $possible_port < 49152 {
//             ServerPort {
//                 port: $possible_port
//             }
//         }
//         else {
//             panic!("failed to user_port_try_from_u16!(), reason: ephemeral port range 49152-65535");
//         }
//     }
// }
pub trait GetServerPort {
    fn get_server_port(&self) -> &ServerPort;
}

pub trait GetSocketAddr {
    fn get_socket_addr(&self) -> &std::string::String;
}

pub trait GetHmacSecret {
    fn get_hmac_secret(&self) -> &secrecy::Secret<std::string::String>;
}

pub trait GetBaseUrl {
    fn get_base_url(&self) -> &std::string::String;
}

pub trait GetAccessControlMaxAge {
    fn get_access_control_max_age(&self) -> &std::primitive::usize;
}

pub trait GetAccessControlAllowOrigin {
    fn get_access_control_allow_origin(&self) -> &std::string::String;
}

pub trait GetGithubName {
    fn get_github_name(&self) -> &std::string::String;
}

pub trait GetGithubToken {
    fn get_github_token(&self) -> &std::string::String;
}

pub trait GetTimezone {
    fn get_timezone(&self) -> &chrono::FixedOffset;
}

pub trait GetRedisUrl {
    fn get_redis_url(&self) -> &secrecy::Secret<std::string::String>;
}

pub trait GetMongoUrl {
    fn get_mongo_url(&self) -> &secrecy::Secret<std::string::String>;
}

pub trait GetDatabaseUrl {
    fn get_database_url(&self) -> &secrecy::Secret<std::string::String>; //postgres database url. required to exists in env
}

pub trait GetStartingCheckLink {
    fn get_starting_check_link(&self) -> &std::string::String;
}

#[derive(
    Debug,
    Clone,
    strum_macros::EnumIter,
    enum_extension::EnumExtension,
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_snake_case())
    }
}
pub trait GetTracingType {
    fn get_tracing_type(&self) -> &TracingType;
}

#[derive(
    Debug,
    Clone,
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
    fn get_source_place_type(&self) -> &SourcePlaceType;
}

pub trait GetEnableApiGitCommitCheck {
    fn get_enable_api_git_commit_check(&self) -> &std::primitive::bool;
}

pub trait GetMaximumSizeOfHttpBodyInBytes {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &std::primitive::usize;
}