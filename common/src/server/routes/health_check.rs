pub(crate) fn health_check_route() -> axum::Router {
    axum::Router::new().route("/health_check", axum::routing::get(|| async {
        println!("health_check");
        axum::http::StatusCode::OK
    }))
}
