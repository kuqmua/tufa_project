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

#[derive(Debug, 
    config_lib::TryFromEnv,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFieldsHandle
)]
struct ConfigHandle {
    //todo maybe auto generate .env and docker-compose environment variables. and maybe write in directly into files
    service_socket_address: config_lib::ServiceSocketAddressWrapper,
    
    timezone: config_lib::TimezoneWrapper, //for some reason chrono::FixedOffset::east_opt uses i32 but i16 is enough
    
    redis_url: config_lib::RedisUrlWrapper,
    
    mongo_url: config_lib::MongoUrlWrapper,
    
    database_url: config_lib::DatabaseUrlWrapper, //postgres_url, naming required by sqlx::query::query!
    
    starting_check_link: config_lib::StartingCheckLinkWrapper, //todo add browser url limit check
    
    tracing_type: config_lib::TracingTypeWrapper,
    source_place_type: config_lib::SourcePlaceTypeWrapper,
    enable_api_git_commit_check: config_lib::EnableApiGitCommitCheckWrapper,
    maximum_size_of_http_body_in_bytes: config_lib::MaximumSizeOfHttpBodyInBytesWrapper,
}

// impl Config {
//     pub fn try_from_env() -> Result<Self, ConfigHandleTryFromEnvErrorNamed> {
//         let config_handle = match ConfigHandle::try_from_env() {
//             Ok(value) => value,
//             Err(error) => {
//                 return Err(error);
//             }
//         };
//         Ok(Self {
//             service_socket_address: config_handle.service_socket_address.0,
        
//             timezone: config_handle.timezone.0,
        
//             redis_url: config_handle.redis_url.0,
        
//             mongo_url: config_handle.mongo_url.0,
        
//             database_url: config_handle.database_url.0, //postgres_url, naming required by sqlx::query::query!
        
//             starting_check_link: config_handle.starting_check_link.0, //todo add browser url limit check
        
//             tracing_type: config_handle.tracing_type.0,
//             source_place_type: config_handle.source_place_type.0,
//             enable_api_git_commit_check: config_handle.enable_api_git_commit_check.0,
//             maximum_size_of_http_body_in_bytes: config_handle.maximum_size_of_http_body_in_bytes.0,
//         })
//     }
// }
////////////////////

