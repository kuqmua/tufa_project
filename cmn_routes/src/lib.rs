//todo gen openapi spec
use app_state::GetPgPool;
use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, Uri},
    routing::get,
};
use git_info::GetGitCommitLink;
use optml::Optml;
use serde::Serialize;
use std::sync::Arc;
const SLASH_SWAGGER_UI: &str = "/swagger-ui";
type DynArcCmnRoutesPrmsSendSync = Arc<dyn CmnRoutesPrms>;
pub trait CmnRoutesPrms: GetGitCommitLink + GetPgPool + Send + Sync {}
pub fn cmn_routes(app_state_b9fc2d94: DynArcCmnRoutesPrmsSendSync) -> Router {
    Router::new()
        .route(
            "/health_check",
            get(
                async |State(app_state_hc): State<DynArcCmnRoutesPrmsSendSync>| match sqlx::query(
                    "SELECT 1",
                )
                .execute(app_state_hc.get_pg_pool())
                .await
                {
                    Ok(_) => StatusCode::OK,
                    Err(_) => StatusCode::SERVICE_UNAVAILABLE,
                },
            ),
        )
        .route(
            "/git_info",
            get(
                async |State(app_state_76fb2013): State<DynArcCmnRoutesPrmsSendSync>| {
                    #[derive(Debug, Serialize, Optml)]
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
        .with_state(Arc::<dyn CmnRoutesPrms>::clone(&app_state_b9fc2d94))
        .fallback(
            async |uri: Uri, State(app_state_19103bd5): State<DynArcCmnRoutesPrmsSendSync>| {
                #[derive(Debug, Serialize, Optml)]
                struct NotFoundH {
                    commit: String,
                    msg: String,
                    open_api_specification: &'static str,
                }
                (
                    StatusCode::NOT_FOUND,
                    Json(NotFoundH {
                        msg: format!("No route for {uri}"),
                        commit: app_state_19103bd5.get_git_commit_link(),
                        open_api_specification: SLASH_SWAGGER_UI,
                    }),
                )
            },
        )
        .with_state(app_state_b9fc2d94)
}
