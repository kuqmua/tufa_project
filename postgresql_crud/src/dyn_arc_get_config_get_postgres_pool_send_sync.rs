pub trait GetConfigGetPostgresPool:
    config_lib::config_fields::GetEnableApiGitCommitCheck 
    + config_lib::config_fields::GetSourcePlaceType
    + config_lib::config_fields::GetTimezone
    + app_state::get_postgres_pool::GetPostgresPool
{
}