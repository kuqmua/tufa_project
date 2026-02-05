//todo generate openapi spec
type DynArcCommonRoutesParametersSendSync = std::sync::Arc<dyn CommonRoutesParameters>;
pub trait CommonRoutesParameters: git_info::GetGitCommitLink + Send + Sync {}
pub fn common_routes(app_state_b9fc2d94: DynArcCommonRoutesParametersSendSync) -> axum::Router {
    axum::Router::new()
        .route(
            "/health_check",
            axum::routing::get(async || axum::http::StatusCode::OK),
        )
        .route(
            "/git_info",
            axum::routing::get(
                async |axum::extract::State(app_state_76fb2013): axum::extract::State<
                    DynArcCommonRoutesParametersSendSync,
                >| {
                    #[derive(Debug, serde::Serialize)]
                    struct GitInfo {
                        commit: String,
                    }
                    (
                        axum::http::StatusCode::OK,
                        axum::Json(GitInfo {
                            commit: app_state_76fb2013.get_git_commit_link(),
                        }),
                    )
                },
            ),
        )
        .with_state(std::sync::Arc::<dyn CommonRoutesParameters>::clone(
            &app_state_b9fc2d94,
        ))
        .fallback(
            async |uri: axum::http::Uri,
                   axum::extract::State(app_state_19103bd5): axum::extract::State<
                DynArcCommonRoutesParametersSendSync,
            >| {
                #[derive(Debug, serde::Serialize)]
                struct NotFoundHandle {
                    commit: String,
                    message: String,
                    open_api_specification: &'static str,
                }
                (
                    axum::http::StatusCode::NOT_FOUND,
                    axum::Json(NotFoundHandle {
                        message: format!("No route for {uri}"),
                        commit: app_state_19103bd5.get_git_commit_link(),
                        open_api_specification: constants::SLASH_SWAGGER_UI,
                    }),
                )
            },
        )
        .with_state(app_state_b9fc2d94)
}
