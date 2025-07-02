//todo how to handle sql injection ?
//todo - maybe check max length for field here instead of put it in postgres and recieve error ? color VARCHAR (255) NOT NULL
//todo - add limit everywhere possible
//todo header Retry-After logic
//todo - its the case if all columns except id are not null. for nullable columns must be different logic(post or put)

pub(crate) fn routes(app_state: common::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits) -> axum::Router {
    axum::Router::new().nest(
        &format!(
            "/{}",
            // "todo",
            common::repositories_types::server::routes::api::example::Example::table_name()
        ),
        axum::Router::new().merge(crud(app_state)),
    )
}

// async fn get_root() {}

fn crud(app_state: common::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits) -> axum::Router {
    axum::Router::new()
        //todo - remove it its just a mock route
        // .route(
        //     "/",
        //     axum::routing::get(get_root),
        // )
        //todo generate axum::Router and make it pub instead of create_many -like router handlers
        .route("/create_many", axum::routing::post(common::repositories_types::server::routes::api::example::Example::try_create_many_route_logic))
        .route("/create_many_payload_example", axum::routing::get(common::repositories_types::server::routes::api::example::Example::create_many_payload_example_route_logic))
        .route("/create_one", axum::routing::post(common::repositories_types::server::routes::api::example::Example::try_create_one_route_logic))
        .route("/create_one_payload_example", axum::routing::get(common::repositories_types::server::routes::api::example::Example::create_one_payload_example_route_logic))
        .route("/read_many", axum::routing::post(common::repositories_types::server::routes::api::example::Example::try_read_many_route_logic))
        .route("/read_many_payload_example", axum::routing::get(common::repositories_types::server::routes::api::example::Example::read_many_payload_example_route_logic))
        .route("/read_one", axum::routing::post(common::repositories_types::server::routes::api::example::Example::try_read_one_route_logic))
        .route("/read_one_payload_example", axum::routing::get(common::repositories_types::server::routes::api::example::Example::read_one_payload_example_route_logic))
        .route("/update_many", axum::routing::patch(common::repositories_types::server::routes::api::example::Example::try_update_many_route_logic))
        .route("/update_many_payload_example", axum::routing::get(common::repositories_types::server::routes::api::example::Example::update_many_payload_example_route_logic))
        .route("/update_one", axum::routing::patch(common::repositories_types::server::routes::api::example::Example::try_update_one_route_logic))
        .route("/update_one_payload_example", axum::routing::get(common::repositories_types::server::routes::api::example::Example::update_one_payload_example_route_logic))
        .route("/delete_many", axum::routing::delete(common::repositories_types::server::routes::api::example::Example::try_delete_many_route_logic))
        .route("/delete_many_payload_example", axum::routing::get(common::repositories_types::server::routes::api::example::Example::delete_many_payload_example_route_logic))
        .route("/delete_one", axum::routing::delete(common::repositories_types::server::routes::api::example::Example::try_delete_one_route_logic))
        .route("/delete_one_payload_example", axum::routing::get(common::repositories_types::server::routes::api::example::Example::delete_one_payload_example_route_logic))
        //
        // .layer(tower_http::cors::CorsLayer::new().allow_methods(
        //     common::repositories_types::server::routes::api::example::ALLOW_METHODS,
        // ))
        // .route_layer(axum::middleware::from_fn_with_state(
        //     app_state.clone() as common::server::middleware::commit_checker::CommitCheckerAppState,
        //     common::server::middleware::commit_checker::commit_checker,
        // ))
        .with_state(app_state)
}
