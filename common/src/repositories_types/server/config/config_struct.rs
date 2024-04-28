#[derive(Debug, 
    config_lib::TryFromEnv,
    generate_getter_traits_for_struct_fields::GenerateGetterTraitsForStructFieldsHandle
)]
pub struct Config {
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

