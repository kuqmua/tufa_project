pub use config_lib::{
    GetDatabaseUrl, GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetMongoUrl,
    GetRedisUrl, GetServiceSocketAddress, GetSourcePlaceType, GetStartingCheckLink, GetTimezone,
    GetTracingLevel, types::SourcePlaceType, types::TracingLevel,
};
use sqlx::PgPool;

pub trait GetPgPool {
    fn get_pg_pool(&self) -> &PgPool;
}
