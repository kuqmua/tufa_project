pub(crate) type DynArcNotFoundRouteParametersSendSync =
    std::sync::Arc<dyn NotFoundRouteParameters + Send + Sync>;

pub trait NotFoundRouteParameters: git_info::GetGitCommitLink {}

#[derive(Debug, serde::Serialize)]
struct NotFoundHandle {
    message: std::string::String,
    commit: std::string::String,
    open_api_specification: &'static str,
}

//todo maybe use swagger instead
async fn not_found(
    uri: axum::http::Uri,
    axum::extract::State(app_state): axum::extract::State<DynArcNotFoundRouteParametersSendSync>,
) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        axum::Json(NotFoundHandle {
            message: format!("No route for {uri}"),
            commit: app_state.get_git_commit_link(),
            open_api_specification: constants::SLASH_SWAGGER_UI,
        }),
    )
}

pub fn not_found_route(app_state: DynArcNotFoundRouteParametersSendSync) -> axum::Router {
    axum::Router::new()
        .fallback(not_found)
        .with_state(app_state)
}
