//todo how to handle sql injection ?
//todo - maybe check max length for field here instead of put it in postgres and recieve error ? color VARCHAR (255) NOT NULL
//todo - add limit everywhere possible
//todo header Retry-After logic
//todo - its the case if all columns except id are not null. for nullable columns must be different logic(post or put)

pub fn routes(
    app_info: tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
) -> axum::Router {
    axum::Router::new().nest(
        &format!(
            "/{}",
            tufa_common::repositories_types::tufa_server::routes::api::cats::TABLE_NAME
        ),
        axum::Router::new().merge(crud(app_info)),
    )
}

// async fn get_root() {}

fn crud(
    app_info: tufa_common::repositories_types::tufa_server::routes::api::cats::DynArcGetConfigGetPostgresPoolSendSync,
) -> axum::Router {
    axum::Router::new()
        //todo - remove it its just a mock route
        // .route(
        //     "/",
        //     axum::routing::get(get_root),
        // )

        //todo generate axum::Router and make it pub instead of create_many -like router handlers
        .route(
            "/create_many",
            axum::routing::post(tufa_common::repositories_types::tufa_server::routes::api::cats::create_many),
        )
        .route(
            "/create_one",
            axum::routing::post(tufa_common::repositories_types::tufa_server::routes::api::cats::create_one),
        )
        .route(
            "/read_many",
            axum::routing::post(tufa_common::repositories_types::tufa_server::routes::api::cats::read_many),
        )
        .route(
            "/read_one",
            axum::routing::post(tufa_common::repositories_types::tufa_server::routes::api::cats::read_one),
        )
        .route(
            "/update_many",
            axum::routing::patch(tufa_common::repositories_types::tufa_server::routes::api::cats::update_many),
        )
        .route(
            "/update_one",
            axum::routing::patch(tufa_common::repositories_types::tufa_server::routes::api::cats::update_one),
        )
        .route(
            "/delete_many",
            axum::routing::delete(tufa_common::repositories_types::tufa_server::routes::api::cats::delete_many),
        )
        .route(
            "/delete_one",
            axum::routing::delete(tufa_common::repositories_types::tufa_server::routes::api::cats::delete_one),
        )
        // .layer(tower_http::cors::CorsLayer::new().allow_methods(
        //     tufa_common::repositories_types::tufa_server::routes::api::cats::ALLOW_METHODS,
        // ))
        .route_layer(axum::middleware::from_fn_with_state(
            app_info.clone(),
            tufa_common::server::middleware::project_commit_checker::project_commit_checker,
        ))
        .with_state(app_info)
}
