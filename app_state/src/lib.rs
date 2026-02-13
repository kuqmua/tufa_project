pub use config_lib::{
    GetDatabaseUrl, GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetMongoUrl,
    GetRedisUrl, GetServiceSocketAddress, GetSourcePlaceType, GetStartingCheckLink, GetTimezone,
    GetTracingLevel, types::SourcePlaceType, types::TracingLevel,
};

pub trait GetPgPool {
    fn get_pg_pool(&self) -> &sqlx::PgPool;
}
