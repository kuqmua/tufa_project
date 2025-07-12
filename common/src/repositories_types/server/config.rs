#[derive(Debug, config_lib::TryFromEnv, config_lib::GenerateGetterTraitsForStructFields)]
pub struct Config {
    //todo maybe auto generate .env and docker-compose environment variables. and maybe write in directly into files
    pub service_socket_address: std::net::SocketAddr,
    pub timezone: chrono::FixedOffset,
    pub redis_url: secrecy::Secret<std::string::String>,
    pub mongo_url: secrecy::Secret<std::string::String>,
    pub database_url: secrecy::Secret<std::string::String>,
    pub starting_check_link: std::string::String,
    pub tracing_level: config_lib::types::TracingLevel,
    pub source_place_type: config_lib::types::SourcePlaceType,
    pub enable_api_git_commit_check: std::primitive::bool,
    pub maximum_size_of_http_body_in_bytes: std::primitive::usize,
}
