pub trait GetServerPort {
    fn get_server_port(&self) -> &config_lib::user_port::UserPort;
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
    fn get_access_control_max_age(&self) -> &usize;
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
pub trait GetTracingType {
    fn get_tracing_type(&self) -> &config_lib::tracing_type::TracingType;
}
pub trait GetSourcePlaceType {
    fn get_source_place_type(&self) -> &config_lib::source_place_type::SourcePlaceType;
}
pub trait GetEnableApiGitCommitCheck {
    fn get_enable_api_git_commit_check(&self) -> &bool;
}
