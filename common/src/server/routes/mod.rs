pub mod git_info;
mod health_check;
pub mod helpers;
pub mod not_found;

pub fn routes(app_state: git_info::DynArcGitInfoRouteParametersSendSync) -> axum::Router {
    axum::Router::new().merge(health_check::health_check_route()).merge(git_info::git_info_route(app_state))
}
