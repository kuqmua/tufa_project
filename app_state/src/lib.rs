// config_lib::config_fields::GetEnableApiGitCommitCheck
pub trait GetPostgresPool {
    fn get_postgres_pool(&self) -> &sqlx::PgPool;
}