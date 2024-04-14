//todo move and reexport traits
pub trait CombinationOfTraitsForPostgresqlCrudLogic:
    config_lib::config_fields::GetEnableApiGitCommitCheck
    + config_lib::config_fields::GetSourcePlaceType
    + config_lib::config_fields::GetTimezone
    + app_state::get_postgres_pool::GetPostgresPool
    
{
}

pub type DynArcGetConfigGetPostgresPoolSendSync =
    std::sync::Arc<dyn CombinationOfTraitsForPostgresqlCrudLogic + Send + Sync>;
