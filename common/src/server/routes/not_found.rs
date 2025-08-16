pub(crate) type DynArcNotFoundRouteParametersSendSync = std::sync::Arc<dyn NotFoundRouteParameters + Send + Sync>;

pub trait NotFoundRouteParameters: git_info::GetGitCommitLink {}

#[derive(Debug, serde::Serialize)]
struct NotFoundHandle {
    message: std::string::String,
    commit: std::string::String,
    open_api_specification: &'static str,
}

//todo maybe use swagger instead
pub fn not_found_route(app_state: DynArcNotFoundRouteParametersSendSync) -> axum::Router {
    axum::Router::new().fallback(|uri: axum::http::Uri, axum::extract::State(app_state): axum::extract::State<DynArcNotFoundRouteParametersSendSync>| async move { (axum::http::StatusCode::NOT_FOUND, axum::Json(NotFoundHandle { message: format!("No route for {uri}"), commit: app_state.get_git_commit_link(), open_api_specification: constants::SLASH_SWAGGER_UI })) }).with_state(app_state)
}
