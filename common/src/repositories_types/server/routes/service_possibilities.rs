pub trait ServicePossibilities:
    crate::server::routes::git_info::GitInfoRouteParameters
    + crate::server::routes::not_found::NotFoundRouteParameters
    + postgresql_crud::app_info_state::GetConfigGetPostgresPool
{
}
