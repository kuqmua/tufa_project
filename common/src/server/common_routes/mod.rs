pub mod git_info;
mod health_check;
pub mod helpers;
pub mod not_found;

type DynArcCommonRoutesParametersSendSync = std::sync::Arc<dyn CommonRoutesParameters>;
pub trait CommonRoutesParameters: git_info::GitInfoRouteParameters + not_found::NotFoundRouteParameters + Send + Sync {}
pub fn common_routes(app_state: DynArcCommonRoutesParametersSendSync) -> axum::Router {
    axum::Router::new()
    .merge(health_check::health_check_route())
    .merge(git_info::git_info_route(std::sync::Arc::<dyn CommonRoutesParameters>::clone(&app_state)))
    .merge(not_found::not_found_route(app_state))
}
