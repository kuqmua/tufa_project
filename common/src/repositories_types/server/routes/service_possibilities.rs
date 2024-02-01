pub trait ServicePossibilities:
    crate::server::routes::git_info::GitInfoRouteParameters
    + crate::server::routes::not_found::NotFoundRouteParameters
    + postgresql_crud::dyn_arc_get_config_get_postgres_pool_send_sync::GetConfigGetPostgresPool
{
}
