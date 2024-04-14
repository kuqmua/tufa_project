pub use config_lib::config_fields::GetServerPort;
pub use config_lib::config_fields::GetSocketAddr;
pub use config_lib::config_fields::GetHmacSecret;
pub use config_lib::config_fields::GetBaseUrl;
pub use config_lib::config_fields::GetAccessControlMaxAge;
pub use config_lib::config_fields::GetAccessControlAllowOrigin;
pub use config_lib::config_fields::GetGithubName;
pub use config_lib::config_fields::GetGithubToken;
pub use config_lib::config_fields::GetTimezone;
pub use config_lib::config_fields::GetRedisUrl;
pub use config_lib::config_fields::GetMongoUrl;
pub use config_lib::config_fields::GetDatabaseUrl;
pub use config_lib::config_fields::GetStartingCheckLink;
pub use config_lib::config_fields::GetTracingType;
pub use config_lib::config_fields::GetSourcePlaceType;
pub use config_lib::config_fields::GetEnableApiGitCommitCheck;
pub use config_lib::config_fields::GetMaximumSizeOfHttpBodyInBytes;

pub trait GetPostgresPool {
    fn get_postgres_pool(&self) -> &sqlx::PgPool;
}