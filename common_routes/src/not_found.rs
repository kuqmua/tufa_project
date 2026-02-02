type DynArcRouteParametersSendSync = std::sync::Arc<dyn RouteParameters>;
pub trait RouteParameters: git_info::GetGitCommitLink + Send + Sync {}
#[derive(Debug, serde::Serialize)]
struct NotFoundHandle {
    message: String,
    commit: String,
    open_api_specification: &'static str,
}
//todo maybe use swagger instead
pub fn route(app_state: DynArcRouteParametersSendSync) -> axum::Router {
    axum::Router::new()
        .fallback(
            async |uri: axum::http::Uri,
                   axum::extract::State(state_value): axum::extract::State<
                DynArcRouteParametersSendSync,
            >| {
                (
                    axum::http::StatusCode::NOT_FOUND,
                    axum::Json(NotFoundHandle {
                        message: format!("No route for {uri}"),
                        commit: state_value.get_git_commit_link(),
                        open_api_specification: constants::SLASH_SWAGGER_UI,
                    }),
                )
            },
        )
        .with_state(app_state)
}
