pub use init_from_env::TryFromEnv;

pub trait GetServiceSocketAddress {
    fn get_service_socket_address(&self) -> &std::net::SocketAddr;
}

// pub trait GetServerHost {
//     fn get_server_host(&self) -> &std::string::String;
// }

// pub use server_port::ServerPort;
// pub use server_port::ServerPortErrorNamed;
// pub use server_port::server_port_try_from_u16;
// pub trait GetServerPort {
//     fn get_server_port(&self) -> &server_port::ServerPort;
// }

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
    fn get_tracing_type(&self) -> &TracingType;
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
    fn get_source_place_type(&self) -> &SourcePlaceType;
}

pub trait GetEnableApiGitCommitCheck {
    fn get_enable_api_git_commit_check(&self) -> &std::primitive::bool;
}

pub trait GetMaximumSizeOfHttpBodyInBytes {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &std::primitive::usize;
}