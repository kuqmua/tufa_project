mod cats;
mod example;

pub fn routes(app_state: common::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits) -> axum::Router {
    axum::Router::new().merge(crate::routes::api::cats::routes(app_state.clone())).merge(crate::routes::api::example::routes(app_state))
}
