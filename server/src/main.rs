use app_state::{GetDatabaseUrl, GetServiceSocketAddress};
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
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::fmt::init;
fn main() {
    init();
    Builder::new_multi_thread()
        .worker_threads(get())
        .enable_all()
        .build()
        .expect("5995c954")
        .block_on(async {
            let config = Config::try_from_env().expect("d74a6e5f");
            let pg_pool = PgPoolOptions::new()
                .max_connections(50)
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
            let app_state = Arc::new(ServerAppState {
                pg_pool,
                config,
                project_git_info: &PROJECT_GIT_INFO,
            });
            serve(
                tcp_listener,
                Router::new()
                    .merge(cmn_routes(Arc::<ServerAppState<'_>>::clone(&app_state)))
                    .merge(TblExample::routes(Arc::<ServerAppState<'_>>::clone(
                        &app_state,
                    )))
                    .layer(TraceLayer::new_for_http())
                    .layer(
                        CorsLayer::new()
                            .allow_origin(["http://127.0.0.1".parse().expect("2a0b7c30")]),
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
