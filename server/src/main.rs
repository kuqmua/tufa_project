use app_state::{
    GetCorsAllowOrigin, GetDatabaseUrl, GetPgPoolMaxConnections, GetServiceSocketAddress,
};
use axum::{Router, serve};
use cmn_routes::cmn_routes;
use git_info::PROJECT_GIT_INFO;
use num_cpus::get;
use secrecy::ExposeSecret;
use server_app_state::ServerAppState;
use server_config::Config;
use server_tbl_example::TblExample;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::{net::TcpListener, runtime::Builder, signal};
use tower::ServiceBuilder;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};
fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer())
        .init();
    Builder::new_multi_thread()
        .worker_threads(get())
        .enable_all()
        .build()
        .expect("5995c954")
        .block_on(async {
            let config = Config::try_from_env().expect("d74a6e5f");
            let pg_pool = PgPoolOptions::new()
                .max_connections(*GetPgPoolMaxConnections::get_pg_pool_max_connections(
                    &config,
                ))
                .connect(ExposeSecret::expose_secret(
                    GetDatabaseUrl::get_database_url(&config),
                ))
                .await
                .expect("8b72f688");
            TblExample::prep_pg(&pg_pool).await.expect("647fa499");
            let tcp_listener =
                TcpListener::bind(GetServiceSocketAddress::get_service_socket_address(&config))
                    .await
                    .expect("3f294e7c");
            let cors_origins = GetCorsAllowOrigin::get_cors_allow_origin(&config)
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect::<Vec<_>>();
            let app_state = Arc::new(ServerAppState {
                pg_pool,
                config,
                project_git_info: &PROJECT_GIT_INFO,
            });
            let governor_conf = Arc::new(
                GovernorConfigBuilder::default()
                    .per_second(2)
                    .burst_size(10)
                    .finish()
                    .expect("b7e3a4f1"),
            );
            let api_routes = Router::new()
                .merge(cmn_routes(Arc::<ServerAppState<'_>>::clone(&app_state)))
                .merge(TblExample::routes(Arc::<ServerAppState<'_>>::clone(
                    &app_state,
                )));
            serve(
                tcp_listener,
                Router::new()
                    .nest("/api/v1", api_routes)
                    .layer(
                        ServiceBuilder::new()
                            .layer(PropagateRequestIdLayer::x_request_id())
                            .layer(TraceLayer::new_for_http())
                            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                            .layer(CorsLayer::new().allow_origin(cors_origins))
                            .layer(GovernorLayer::new(governor_conf)),
                    )
                    .into_make_service(),
            )
            .with_graceful_shutdown(async {
                signal::ctrl_c().await.expect("a3f7b1c2");
            })
            .await
            .expect("2dc4449b");
        });
}
