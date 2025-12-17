#[derive(Debug, config_lib::TryFromEnv, config_lib::GenerateGetterTraitsForStructFields)]
pub struct Config {
    //todo maybe auto generate .env and docker-compose environment variables. and maybe write in directly into files
    service_socket_address: std::net::SocketAddr,
    database_url: secrecy::Secret<String>,
    timezone: chrono::FixedOffset,
    tracing_level: config_lib::types::TracingLevel,
    source_place_type: config_lib::types::SourcePlaceType,
    enable_api_git_commit_check: bool,
    maximum_size_of_http_body_in_bytes: usize,
}

type ConfigFieldsTypes = (
    std::net::SocketAddr,
    secrecy::Secret<String>,
    chrono::FixedOffset,
    config_lib::types::TracingLevel,
    config_lib::types::SourcePlaceType,
    bool,
    usize,
);

impl From<ConfigFieldsTypes> for Config {
    fn from(value: ConfigFieldsTypes) -> Self {
        Self {
            service_socket_address: value.0,
            database_url: value.1,
            timezone: value.2,
            tracing_level: value.3,
            source_place_type: value.4,
            enable_api_git_commit_check: value.5,
            maximum_size_of_http_body_in_bytes: value.6,
        }
    }
}