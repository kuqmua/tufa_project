//todo generate openapi spec
use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, Uri},
    routing::get,
};
use std::sync::Arc;
type DynArcCommonRoutesParametersSendSync = Arc<dyn CommonRoutesParameters>;
pub trait CommonRoutesParameters: git_info::GetGitCommitLink + Send + Sync {}
pub fn common_routes(app_state_b9fc2d94: DynArcCommonRoutesParametersSendSync) -> Router {
    Router::new()
        .route(
            "/health_check",
            get(async || StatusCode::OK),
        )
        .route(
            "/git_info",
            get(
                async |State(app_state_76fb2013): State<
                    DynArcCommonRoutesParametersSendSync,
                >| {
                    #[derive(Debug, serde::Serialize)]
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
        .with_state(Arc::<dyn CommonRoutesParameters>::clone(
            &app_state_b9fc2d94,
        ))
        .fallback(
            async |uri: Uri,
                   State(app_state_19103bd5): State<
                DynArcCommonRoutesParametersSendSync,
            >| {
                #[derive(Debug, serde::Serialize)]
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
                        open_api_specification: constants::SLASH_SWAGGER_UI,
                    }),
                )
            },
        )
        .with_state(app_state_b9fc2d94)
}
