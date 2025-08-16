pub(crate) type DynArcGitInfoRouteParametersSendSync =
    std::sync::Arc<dyn GitInfoRouteParameters + Send + Sync>;

pub trait GitInfoRouteParameters: git_info::GetGitCommitLink {}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct GitInfo {
    #[schema(
        example = "https://github.com/kuqmua/tufa_project/tree/f7de4c3c218579600cd0737d5a6b6c8362cbd88b"
    )]
    commit: std::string::String,
}

#[utoipa::path(
    get,
    path = "/git_info",
    responses(
        (status = 200, description = "source code repository git information", body = [GitInfo])
    )
)]
pub async fn git_info(
    axum::extract::State(app_state): axum::extract::State<DynArcGitInfoRouteParametersSendSync>,
) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::OK,
        axum::Json(GitInfo {
            commit: app_state.get_git_commit_link(),
        }),
    )
}

pub(crate) fn git_info_route(app_state: DynArcGitInfoRouteParametersSendSync) -> axum::Router {
    axum::Router::new()
        .route("/git_info", axum::routing::get(git_info))
        .with_state(app_state)
}
