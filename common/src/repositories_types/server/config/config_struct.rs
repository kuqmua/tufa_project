
#[derive(
    Debug, 
    config_lib::TryFromEnv,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFields
)]
pub struct Config {
    //todo maybe auto generate .env and docker-compose environment variables. and maybe write in directly into files
    service_socket_address: std::net::SocketAddr,
    
    timezone: chrono::FixedOffset,

    redis_url: secrecy::Secret<std::string::String>,
    
    mongo_url: secrecy::Secret<std::string::String>,
    
    database_url: secrecy::Secret<std::string::String>,
    
    starting_check_link: std::string::String,
    
    tracing_type: config_lib::TracingTypeEnum,
    source_place_type: config_lib::SourcePlaceType,
    enable_api_git_commit_check: std::primitive::bool,
    maximum_size_of_http_body_in_bytes: std::primitive::usize,
}