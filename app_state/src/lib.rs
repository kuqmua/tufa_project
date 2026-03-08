pub use config_lib::{
    GetDatabaseUrl, GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetMongoUrl,
    GetRedisUrl, GetServiceSocketAddress, GetSrcPlaceType, GetStartingCheckLink, GetTimezone,
    GetTracingLevel, types::SrcPlaceType, types::TracingLevel,
};
use sqlx::PgPool;
pub trait GetPgPool {
    fn get_pg_pool(&self) -> &PgPool;
}
