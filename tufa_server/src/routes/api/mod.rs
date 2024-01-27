mod cats;

pub fn routes(
    app_info: tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
) -> axum::Router {
    axum::Router::new().merge(crate::routes::api::cats::routes(app_info))
}
