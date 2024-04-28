
#[derive(
    Debug, 
    config_lib::TryFromEnv,
    config_lib::GenerateGetterTraitsForStructFields
)]
pub struct Config {
    //todo maybe auto generate .env and docker-compose environment variables. and maybe write in directly into files
    service_socket_address: std::net::SocketAddr,
    
    timezone: chrono::FixedOffset,

    redis_url: secrecy::Secret<std::string::String>,
    
    mongo_url: secrecy::Secret<std::string::String>,
    
    database_url: secrecy::Secret<std::string::String>,
    
    starting_check_link: std::string::String,
    
    tracing_level: config_lib::types::TracingLevel,
    source_place_type: config_lib::types::SourcePlaceType,
    enable_api_git_commit_check: std::primitive::bool,
    maximum_size_of_http_body_in_bytes: std::primitive::usize,
}