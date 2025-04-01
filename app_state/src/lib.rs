pub use config_lib::GetDatabaseUrl;
pub use config_lib::GetEnableApiGitCommitCheck;
pub use config_lib::GetMaximumSizeOfHttpBodyInBytes;
pub use config_lib::GetMongoUrl;
pub use config_lib::GetRedisUrl;
pub use config_lib::GetServiceSocketAddress;
pub use config_lib::GetSourcePlaceType;
pub use config_lib::GetStartingCheckLink;
pub use config_lib::GetTimezone;
pub use config_lib::GetTracingLevel;
pub use config_lib::types::SourcePlaceType;
pub use config_lib::types::TracingLevel;

pub trait GetPostgresPool {
    fn get_postgres_pool(&self) -> &sqlx::PgPool;
}
