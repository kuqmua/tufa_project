mod cats;

pub fn routes(
    app_state: postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
) -> axum::Router {
    axum::Router::new().merge(crate::routes::api::cats::routes(app_state))
}
