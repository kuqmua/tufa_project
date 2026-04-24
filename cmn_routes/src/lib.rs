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
use std::{borrow::Cow, sync::Arc};
const SLASH_HEALTH_CHECK: &str = "/health_check";
const SLASH_GIT_INFO: &str = "/git_info";
const SLASH_SWAGGER_UI: &str = "/swagger-ui";
const HEALTH_CHECK_SQL: &str = "SELECT 1";
const NO_ROUTE_MSG_PREFIX: &str = "No route for ";
const HEALTH_CHECK_OK_STATUS: StatusCode = StatusCode::OK;
const HEALTH_CHECK_ER_STATUS: StatusCode = StatusCode::SERVICE_UNAVAILABLE;
type DynArcCmnRoutesPrms = Arc<dyn CmnRoutesPrms>;
type CmnState = State<DynArcCmnRoutesPrms>;
type JsonRes<T> = (StatusCode, Json<T>);
#[derive(Debug, Serialize, Optml)]
struct GitInfo {
    commit: Cow<'static, str>,
}
#[derive(Debug, Serialize, Optml)]
struct NotFoundH {
    commit: Cow<'static, str>,
    msg: String,
    open_api_specification: &'static str,
}
pub trait CmnRoutesPrms: GetGitCommitLink + GetPgPool + Send + Sync {}
#[allow(clippy::single_call_fn)] // keeps commit-link extraction shape shared between handlers and tests
const fn mk_git_info_payload(commit: Cow<'static, str>) -> GitInfo {
    GitInfo { commit }
}
#[allow(clippy::single_call_fn)] // single source for no-route text reused by payload builder and tests
fn mk_no_route_msg(uri: &Uri) -> String {
    mk_no_route_msg_for_suffix(get_uri_suffix(uri))
}
#[allow(clippy::single_call_fn)] // isolated for reuse in tests and payload builder when suffix is precomputed
fn mk_no_route_msg_for_suffix(uri_suffix: &str) -> String {
    let cap = no_route_msg_capacity(uri_suffix);
    let mut msg = String::with_capacity(cap);
    msg.push_str(NO_ROUTE_MSG_PREFIX);
    msg.push_str(uri_suffix);
    msg
}
#[allow(clippy::single_call_fn)] // isolated for reuse in tests and message builder
const fn no_route_msg_capacity(uri_suffix: &str) -> usize {
    NO_ROUTE_MSG_PREFIX.len().saturating_add(uri_suffix.len())
}
#[allow(clippy::single_call_fn)] // keeps route text construction consistent for path-only and path+query URIs
fn get_uri_suffix(uri: &Uri) -> &str {
    uri.path_and_query()
        .map_or_else(|| uri.path(), |v| v.as_str())
}
#[allow(clippy::single_call_fn)] // keeps fallback payload assembly in one place
fn mk_not_found_payload(uri: &Uri, commit: Cow<'static, str>) -> NotFoundH {
    mk_not_found_payload_with_msg(mk_no_route_msg(uri), commit)
}
#[allow(clippy::single_call_fn)] // shared suffix-based assembly is reusable by handlers that already have a path suffix
#[cfg(test)]
fn mk_not_found_payload_for_suffix(uri_suffix: &str, commit: Cow<'static, str>) -> NotFoundH {
    mk_not_found_payload_with_msg(mk_no_route_msg_for_suffix(uri_suffix), commit)
}
#[allow(clippy::single_call_fn)] // shared payload constructor keeps not-found response shape centralized
const fn mk_not_found_payload_with_msg(msg: String, commit: Cow<'static, str>) -> NotFoundH {
    NotFoundH {
        commit,
        msg,
        open_api_specification: SLASH_SWAGGER_UI,
    }
}
#[allow(clippy::single_call_fn)] // shared helper keeps commit-based status+json responses consistent across handlers
fn mk_commit_json_res<S, T>(
    commit_src: &S,
    status: StatusCode,
    map: impl FnOnce(Cow<'static, str>) -> T,
) -> JsonRes<T>
where
    S: ?Sized + GetGitCommitLink,
{
    mk_json_res(status, map(commit_src.get_git_commit_link_cow()))
}
#[allow(clippy::single_call_fn)] // keeps status+json tuple construction consistent across handlers
const fn mk_json_res<T>(status: StatusCode, payload: T) -> JsonRes<T> {
    (status, Json(payload))
}
#[allow(clippy::single_call_fn)] // shared mapping keeps health-check status behavior centralized
const fn map_health_check_status(is_ok: bool) -> StatusCode {
    if is_ok {
        HEALTH_CHECK_OK_STATUS
    } else {
        HEALTH_CHECK_ER_STATUS
    }
}
#[allow(clippy::single_call_fn)] // named handler is clearer than inline closure for route wiring
async fn health_check(State(app_state_hc): CmnState) -> StatusCode {
    map_health_check_status(
        sqlx::query(HEALTH_CHECK_SQL)
            .execute(app_state_hc.get_pg_pool())
            .await
            .is_ok(),
    )
}
#[allow(clippy::single_call_fn)] // named handler is clearer than inline closure for route wiring
async fn git_info(State(app_state_76fb2013): CmnState) -> JsonRes<GitInfo> {
    mk_commit_json_res(
        app_state_76fb2013.as_ref(),
        StatusCode::OK,
        mk_git_info_payload,
    )
}
#[allow(clippy::single_call_fn)] // named handler isolates fallback behavior for maintenance
async fn not_found(uri: Uri, State(app_state_19103bd5): CmnState) -> JsonRes<NotFoundH> {
    mk_commit_json_res(
        app_state_19103bd5.as_ref(),
        StatusCode::NOT_FOUND,
        |commit| mk_not_found_payload(&uri, commit),
    )
}
pub fn cmn_routes(app_state_b9fc2d94: DynArcCmnRoutesPrms) -> Router {
    Router::new()
        .route(SLASH_HEALTH_CHECK, get(health_check))
        .route(SLASH_GIT_INFO, get(git_info))
        .fallback(not_found)
        .with_state(app_state_b9fc2d94)
}
#[cfg(test)]
mod tests {
    use super::{
        HEALTH_CHECK_ER_STATUS, HEALTH_CHECK_OK_STATUS, NO_ROUTE_MSG_PREFIX, SLASH_SWAGGER_UI,
        get_uri_suffix, map_health_check_status, mk_commit_json_res, mk_git_info_payload,
        mk_json_res, mk_no_route_msg, mk_no_route_msg_for_suffix, mk_not_found_payload,
        mk_not_found_payload_for_suffix,
    };
    use crate::CmnRoutesPrms;
    use app_state::GetPgPool;
    use axum::http::{StatusCode, Uri};
    use git_info::{GetGitCommitId, git_commit_link};
    use sqlx::PgPool;
    use std::{borrow::Cow, sync::Arc};
    const TEST_COMMIT: &str = "abc123";
    #[derive(Debug)]
    struct TestState {
        commit: &'static str,
    }
    impl GetGitCommitId for TestState {
        fn get_git_commit_id(&self) -> String {
            self.commit.to_owned()
        }
        fn get_git_commit_id_ref(&self) -> Option<&str> {
            Some(self.commit)
        }
    }
    impl GetPgPool for TestState {
        fn get_pg_pool(&self) -> &PgPool {
            panic!("38f80f5f")
        }
    }
    impl CmnRoutesPrms for TestState {}
    fn test_state() -> Arc<dyn CmnRoutesPrms> {
        Arc::new(TestState {
            commit: TEST_COMMIT,
        })
    }
    fn test_commit_link() -> String {
        git_commit_link(TEST_COMMIT)
    }
    #[allow(clippy::single_call_fn)] // shared owned->Cow conversion keeps commit-link payload setup consistent across tests
    fn test_commit_link_cow() -> Cow<'static, str> {
        Cow::Owned(test_commit_link())
    }
    const fn b_cow(v: &'static str) -> Cow<'static, str> {
        Cow::Borrowed(v)
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps git-info payload checks concise and consistent
    fn assert_git_info_commit(payload: &super::GitInfo, exp_commit: &str) {
        assert_eq!(payload.commit, exp_commit);
    }
    #[allow(clippy::single_call_fn)] // shared assertion centralizes not-found payload checks used across direct and state-based tests
    fn assert_not_found_payload(payload: &super::NotFoundH, exp_uri_suffix: &str) {
        assert_no_route_msg(&payload.msg, exp_uri_suffix);
        assert_eq!(payload.open_api_specification, SLASH_SWAGGER_UI);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps not-found commit and payload checks coupled across tests
    fn assert_not_found_payload_with_commit(
        payload: &super::NotFoundH,
        exp_commit: &str,
        exp_uri_suffix: &str,
    ) {
        assert_eq!(payload.commit, exp_commit);
        assert_not_found_payload(payload, exp_uri_suffix);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps no-route message checks consistent across uri and suffix-based tests
    fn assert_no_route_msg(actual: &str, uri_suffix: &str) {
        assert_eq!(actual, mk_no_route_msg_for_suffix(uri_suffix));
    }
    #[test]
    fn git_info_response_shape_stays_stable() {
        let git_info = mk_git_info_payload(b_cow("abc123"));
        assert_git_info_commit(&git_info, "abc123");
    }
    #[test]
    fn not_found_response_shape_stays_stable() {
        let uri = Uri::from_static("/unknown");
        let not_found = mk_not_found_payload(&uri, b_cow("deadbeef"));
        assert_not_found_payload_with_commit(&not_found, "deadbeef", "/unknown");
    }
    #[test]
    fn no_route_msg_includes_uri() {
        let uri = Uri::from_static("/missing/path");
        assert_no_route_msg(&mk_no_route_msg(&uri), "/missing/path");
    }
    #[test]
    fn no_route_msg_for_suffix_uses_prefix_once() {
        assert_no_route_msg(
            &mk_no_route_msg_for_suffix("/missing/path"),
            "/missing/path",
        );
    }
    #[test]
    fn get_uri_suffix_prefers_path_and_query_when_query_exists() {
        let uri = Uri::from_static("/missing/path?limit=10");
        assert_eq!(get_uri_suffix(&uri), "/missing/path?limit=10");
    }
    #[test]
    fn no_route_msg_keeps_query_parameters() {
        let uri = Uri::from_static("/missing/path?limit=10");
        assert_no_route_msg(&mk_no_route_msg(&uri), "/missing/path?limit=10");
    }
    #[test]
    fn status_code_constants_are_stable_for_common_routes() {
        assert_eq!(StatusCode::OK.as_u16(), 200);
        assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
    }
    #[test]
    fn git_info_response_contains_commit_link() {
        let exp_commit = test_commit_link();
        let payload = mk_git_info_payload(test_commit_link_cow());
        assert_git_info_commit(&payload, &exp_commit);
    }
    #[test]
    fn git_info_payload_from_state_contains_commit_link() {
        let payload = mk_git_info_payload(test_state().get_git_commit_link_cow());
        assert_git_info_commit(&payload, test_commit_link().as_str());
    }
    #[test]
    fn not_found_response_uses_uri_and_swagger_path() {
        let uri = Uri::from_static("/missing");
        let commit_link = test_commit_link();
        let payload = mk_not_found_payload(&uri, test_commit_link_cow());
        assert_not_found_payload_with_commit(&payload, &commit_link, "/missing");
    }
    #[test]
    fn not_found_payload_from_state_uses_uri_and_swagger_path() {
        let uri = Uri::from_static("/missing");
        let payload = mk_not_found_payload(&uri, test_state().get_git_commit_link_cow());
        assert_not_found_payload_with_commit(&payload, &test_commit_link(), "/missing");
    }
    #[test]
    fn not_found_payload_for_suffix_uses_given_suffix_and_swagger_path() {
        let commit_link = test_commit_link();
        let payload = mk_not_found_payload_for_suffix("/missing", test_commit_link_cow());
        assert_not_found_payload_with_commit(&payload, &commit_link, "/missing");
    }
    #[test]
    fn no_route_prefix_stays_stable() {
        assert_eq!(NO_ROUTE_MSG_PREFIX, "No route for ");
    }
    #[test]
    fn no_route_msg_capacity_is_exact_for_uri_suffix() {
        assert_eq!(
            super::no_route_msg_capacity("/abc?x=1"),
            "No route for /abc?x=1".len()
        );
    }
    #[test]
    fn map_health_check_status_returns_ok_for_success() {
        assert_eq!(map_health_check_status(true), HEALTH_CHECK_OK_STATUS);
    }
    #[test]
    fn map_health_check_status_returns_unavailable_for_error() {
        assert_eq!(map_health_check_status(false), HEALTH_CHECK_ER_STATUS);
    }
    #[test]
    fn mk_state_payload_uses_state_trait_object() {
        assert_eq!(test_state().get_git_commit_link_cow(), test_commit_link());
    }
    #[test]
    fn mk_json_res_wraps_payload_with_status() {
        let (status, payload) =
            mk_json_res(StatusCode::CREATED, mk_git_info_payload(b_cow(TEST_COMMIT)));
        assert_eq!(status, StatusCode::CREATED);
        assert_git_info_commit(&payload.0, TEST_COMMIT);
    }
    #[test]
    fn mk_state_payload_passes_commit_link_to_mapper() {
        let actual = format!("v={}", test_state().get_git_commit_link_cow());
        assert_eq!(actual, format!("v={}", test_commit_link()));
    }
    #[test]
    fn mk_commit_json_res_combines_status_and_commit_payload() {
        let (status, payload) =
            mk_commit_json_res(test_state().as_ref(), StatusCode::OK, mk_git_info_payload);
        assert_eq!(status, StatusCode::OK);
        assert_git_info_commit(&payload.0, test_commit_link().as_str());
    }
}
