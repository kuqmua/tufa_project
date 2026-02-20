//todo gen openapi spec
use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, Uri},
    routing::get,
};
use constants::SLASH_SWAGGER_UI;
use git_info::GetGitCommitLink;
use serde::Serialize;
use std::sync::Arc;
type DynArcCommonRoutesParamsSendSync = Arc<dyn CommonRoutesParams>;
pub trait CommonRoutesParams: GetGitCommitLink + Send + Sync {}
pub fn common_routes(app_state_b9fc2d94: DynArcCommonRoutesParamsSendSync) -> Router {
    Router::new()
        .route("/health_check", get(async || StatusCode::OK))
        .route(
            "/git_info",
            get(
                async |State(app_state_76fb2013): State<DynArcCommonRoutesParamsSendSync>| {
                    #[derive(Debug, Serialize)]
                    struct GitInfo {
                        commit: String,
                    }
                    (
                        StatusCode::OK,
                        Json(GitInfo {
                            commit: app_state_76fb2013.get_git_commit_link(),
                        }),
                    )
                },
            ),
        )
        .with_state(Arc::<dyn CommonRoutesParams>::clone(&app_state_b9fc2d94))
        .fallback(
            async |uri: Uri, State(app_state_19103bd5): State<DynArcCommonRoutesParamsSendSync>| {
                #[derive(Debug, Serialize)]
                struct NotFoundHandle {
                    commit: String,
                    message: String,
                    open_api_specification: &'static str,
                }
                (
                    StatusCode::NOT_FOUND,
                    Json(NotFoundHandle {
                        message: format!("No route for {uri}"),
                        commit: app_state_19103bd5.get_git_commit_link(),
                        open_api_specification: SLASH_SWAGGER_UI,
                    }),
                )
            },
        )
        .with_state(app_state_b9fc2d94)
}
