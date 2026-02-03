pub mod git_info;
pub mod not_found;

type DynArcCommonRoutesParametersSendSync = std::sync::Arc<dyn CommonRoutesParameters>;
pub trait CommonRoutesParameters:
    git_info::RouteParameters + not_found::RouteParameters + Send + Sync
{
}
pub fn common_routes(app_state: DynArcCommonRoutesParametersSendSync) -> axum::Router {
    axum::Router::new()
        .merge(axum::Router::new().route(
            "/health_check",
            axum::routing::get(async || {
                println!("health_check");
                axum::http::StatusCode::OK
            }),
        ))
        .merge(git_info::git_info_route(std::sync::Arc::<
            dyn CommonRoutesParameters,
        >::clone(&app_state)))
        .merge(not_found::route(app_state))
}
