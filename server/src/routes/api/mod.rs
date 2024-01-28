mod cats;

pub fn routes(
    app_info: common::repositories_types::server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
) -> axum::Router {
    axum::Router::new().merge(crate::routes::api::cats::routes(app_info))
}
