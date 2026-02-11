pub use config_lib::{
    GetDatabaseUrl, GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetMongoUrl,
    GetRedisUrl, GetServiceSocketAddress, GetSourcePlaceType, GetStartingCheckLink, GetTimezone,
    GetTracingLevel, types::SourcePlaceType, types::TracingLevel,
};

pub trait GetPostgresPool {
    fn get_postgres_pool(&self) -> &sqlx::PgPool;
}
