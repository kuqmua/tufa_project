type DynArcNotFoundRouteParametersSendSync = std::sync::Arc<dyn NotFoundRouteParameters>;
pub trait NotFoundRouteParameters: git_info::GetGitCommitLink + Send + Sync {}
#[derive(Debug, serde::Serialize)]
struct NotFoundHandle {
    message: String,
    commit: String,
    open_api_specification: &'static str,
}

//todo maybe use swagger instead
pub fn not_found_route(app_state: DynArcNotFoundRouteParametersSendSync) -> axum::Router {
    axum::Router::new()
        .fallback(async |uri: axum::http::Uri, axum::extract::State(app_state): axum::extract::State<DynArcNotFoundRouteParametersSendSync>| {
            (
                axum::http::StatusCode::NOT_FOUND,
                axum::Json(NotFoundHandle {
                    message: format!("No route for {uri}"),
                    commit: app_state.get_git_commit_link(),
                    open_api_specification: constants::SLASH_SWAGGER_UI,
                }),
            )
        })
        .with_state(app_state)
}
