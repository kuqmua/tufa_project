mod cats;

pub fn routes(
    app_state: postgresql_crud::DynArcCombinationOfTraitsForPostgresqlCrudLogicSendSync,
) -> axum::Router {
    axum::Router::new().merge(crate::routes::api::cats::routes(app_state))
}
