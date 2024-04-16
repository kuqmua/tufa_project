pub use config_lib::GetServerHost;
pub use config_lib::ServerPortErrorNamed;
pub use config_lib::ServerPort;
pub use config_lib::GetServerPort;
pub use config_lib::GetSocketAddr;
pub use config_lib::GetHmacSecret;
pub use config_lib::GetBaseUrl;
pub use config_lib::GetAccessControlMaxAge;
pub use config_lib::GetAccessControlAllowOrigin;
pub use config_lib::GetGithubName;
pub use config_lib::GetGithubToken;
pub use config_lib::GetTimezone;
pub use config_lib::GetRedisUrl;
pub use config_lib::GetMongoUrl;
pub use config_lib::GetDatabaseUrl;
pub use config_lib::GetStartingCheckLink;
pub use config_lib::TracingType;
pub use config_lib::GetTracingType;
pub use config_lib::SourcePlaceType;
pub use config_lib::GetSourcePlaceType;
pub use config_lib::GetEnableApiGitCommitCheck;
pub use config_lib::GetMaximumSizeOfHttpBodyInBytes;

pub trait GetPostgresPool {
    fn get_postgres_pool(&self) -> &sqlx::PgPool;
}

pub trait GetServerAddress {
    fn get_server_address(&self) -> std::string::String;
}

impl<SelfGeneric> GetServerAddress for SelfGeneric
where
    Self: config_lib::GetServerHost + config_lib::GetServerPort,
{
    fn get_server_address(&self) -> std::string::String {
        format!("{}:{}", *self.get_server_host(), *self.get_server_port())
    }
}