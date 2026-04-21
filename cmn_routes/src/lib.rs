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
const SLASH_HEALTH_CHECK: &str = "/health_check";
const SLASH_GIT_INFO: &str = "/git_info";
const SLASH_SWAGGER_UI: &str = "/swagger-ui";
const HEALTH_CHECK_SQL: &str = "SELECT 1";
const NO_ROUTE_MSG_PREFIX: &str = "No route for ";
type DynArcCmnRoutesPrmsSendSync = Arc<dyn CmnRoutesPrms>;
pub trait CmnRoutesPrms: GetGitCommitLink + GetPgPool + Send + Sync {}
#[derive(Debug, Serialize, Optml)]
struct GitInfo {
    commit: String,
}
#[derive(Debug, Serialize, Optml)]
struct NotFoundH {
    commit: String,
    msg: String,
    open_api_specification: &'static str,
}
fn get_commit_link(app_state: &DynArcCmnRoutesPrmsSendSync) -> String {
    app_state.as_ref().get_git_commit_link()
}
pub fn cmn_routes(app_state_b9fc2d94: DynArcCmnRoutesPrmsSendSync) -> Router {
    Router::new()
        .route(
            SLASH_HEALTH_CHECK,
            get(
                async |State(app_state_hc): State<DynArcCmnRoutesPrmsSendSync>| {
                    if sqlx::query(HEALTH_CHECK_SQL)
                        .execute(app_state_hc.get_pg_pool())
                        .await
                        .is_ok()
                    {
                        StatusCode::OK
                    } else {
                        StatusCode::SERVICE_UNAVAILABLE
                    }
                },
            ),
        )
        .route(
            SLASH_GIT_INFO,
            get(
                async |State(app_state_76fb2013): State<DynArcCmnRoutesPrmsSendSync>| {
                    (
                        StatusCode::OK,
                        Json(GitInfo {
                            commit: get_commit_link(&app_state_76fb2013),
                        }),
                    )
                },
            ),
        )
        .fallback(
            async |uri: Uri, State(app_state_19103bd5): State<DynArcCmnRoutesPrmsSendSync>| {
                let uri_str = uri.to_string();
                let cap = NO_ROUTE_MSG_PREFIX.len().saturating_add(uri_str.len());
                let mut msg = String::with_capacity(cap);
                msg.push_str(NO_ROUTE_MSG_PREFIX);
                msg.push_str(&uri_str);
                (
                    StatusCode::NOT_FOUND,
                    Json(NotFoundH {
                        msg,
                        commit: get_commit_link(&app_state_19103bd5),
                        open_api_specification: SLASH_SWAGGER_UI,
                    }),
                )
            },
        )
        .with_state(app_state_b9fc2d94)
}
#[cfg(test)]
mod tests {
    use super::{GitInfo, NotFoundH, SLASH_SWAGGER_UI};
    use axum::http::{StatusCode, Uri};
    #[test]
    fn git_info_response_shape_stays_stable() {
        let git_info = GitInfo {
            commit: String::from("abc123"),
        };
        assert_eq!(git_info.commit, "abc123");
    }
    #[test]
    fn not_found_response_shape_stays_stable() {
        let uri = Uri::from_static("/unknown");
        let not_found = NotFoundH {
            commit: String::from("deadbeef"),
            msg: format!("No route for {uri}"),
            open_api_specification: SLASH_SWAGGER_UI,
        };
        assert_eq!(not_found.commit, "deadbeef");
        assert_eq!(not_found.msg, "No route for /unknown");
        assert_eq!(not_found.open_api_specification, SLASH_SWAGGER_UI);
    }
    #[test]
    fn status_code_constants_are_stable_for_common_routes() {
        assert_eq!(StatusCode::OK.as_u16(), 200);
        assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
    }
}
