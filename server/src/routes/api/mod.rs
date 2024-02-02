mod cats;

pub fn routes(
    app_info: postgresql_crud::app_info_state::DynArcGetConfigGetPostgresPoolSendSync,
) -> axum::Router {
    axum::Router::new().merge(crate::routes::api::cats::routes(app_info))
}
