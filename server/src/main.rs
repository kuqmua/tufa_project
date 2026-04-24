use app_state::{
    GetCorsAllowOrigin, GetDatabaseUrl, GetPgPoolMaxConnections, GetServiceSocketAddress,
};
use axum::http::HeaderValue;
use axum::{Router, serve};
use cmn_routes::cmn_routes;
use git_info::PROJECT_GIT_INFO;
use num_cpus::get;
use secrecy::ExposeSecret;
use server_app_state::ServerAppState;
use server_config::Config;
use server_tbl_example::TblExample;
use sqlx::postgres::PgPoolOptions;
#[cfg(test)]
use std::str::Split;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    runtime::{Builder, Runtime},
    signal,
};
use tower::ServiceBuilder;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};
const TRACING_DFLT_FILTER: &str = "info";
const CORS_ALLOW_ORIGIN_SPLIT_CH: char = ',';
type SharedAppState = Arc<ServerAppState<'static>>;
#[cfg(test)]
type SplitByChar<'split_lt> = Split<'split_lt, char>;
#[allow(clippy::single_call_fn)] // route wiring is reused by startup flow and isolated from layer setup
fn mk_api_routes(app_state: &SharedAppState) -> Router {
    Router::new()
        .merge(cmn_routes(clone_shared_app_state(app_state)))
        .merge(TblExample::routes(clone_shared_app_state(app_state)))
}
#[allow(clippy::single_call_fn)] // keeps SharedAppState cloning typed in one place for route wiring reuse
fn clone_shared_app_state(app_state: &SharedAppState) -> SharedAppState {
    Arc::clone(app_state)
}
#[allow(clippy::single_call_fn)] // keeps state creation shape reusable and type-stable in one place
fn mk_app_state(config: Config, pg_pool: sqlx::PgPool) -> SharedAppState {
    Arc::new(ServerAppState {
        pg_pool,
        config,
        project_git_info: &PROJECT_GIT_INFO,
    })
}
#[allow(clippy::single_call_fn)] // generic parser keeps separator handling reusable for non-header values and future config fields
fn parse_separated_values<T>(
    v: &str,
    split_ch: char,
    parse_value: impl FnMut(&str) -> Option<T>,
) -> Vec<T> {
    v.split(split_ch).filter_map(parse_value).collect()
}
#[allow(clippy::single_call_fn)] // extracted so per-value parse behavior can be reused and tested directly
fn parse_cors_allow_origin_value(value: &str) -> Option<HeaderValue> {
    value.trim().parse::<HeaderValue>().ok()
}
#[allow(clippy::single_call_fn)] // reusable comma-separated header parser keeps split+parse behavior in one place
fn parse_comma_separated_header_values(v: &str) -> Vec<HeaderValue> {
    parse_separated_values(v, CORS_ALLOW_ORIGIN_SPLIT_CH, parse_cors_allow_origin_value)
}
#[allow(clippy::single_call_fn)] // extracted for reuse in main setup and tests
fn parse_cors_allow_origin(v: &str) -> Vec<HeaderValue> {
    parse_comma_separated_header_values(v)
}
#[allow(clippy::single_call_fn)] // generic splitter is test-only and keeps separator behavior assertions deterministic
#[cfg(test)]
fn split_by_char(v: &str, split_ch: char) -> SplitByChar<'_> {
    v.split(split_ch)
}
#[allow(clippy::single_call_fn)] // tracing initialization is split out so runtime bootstrap stays focused
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(TRACING_DFLT_FILTER)),
        )
        .with(fmt::layer())
        .init();
}
#[allow(clippy::single_call_fn)] // runtime builder is shared by main and can be reused by startup tests
fn mk_runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(get())
        .enable_all()
        .build()
        .expect("5995c954")
}
#[allow(clippy::single_call_fn)] // isolated pool builder keeps startup flow linear and reuses config getters in one place
async fn mk_pg_pool(config: &Config) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(*GetPgPoolMaxConnections::get_pg_pool_max_connections(
            config,
        ))
        .connect(ExposeSecret::expose_secret(
            GetDatabaseUrl::get_database_url(config),
        ))
        .await
        .expect("8b72f688")
}
#[allow(clippy::single_call_fn)] // startup flow is grouped for separation from process/bootstrap concerns
async fn run_server() {
    let config = Config::try_from_env().expect("d74a6e5f");
    let pg_pool = mk_pg_pool(&config).await;
    TblExample::prep_pg(&pg_pool).await.expect("647fa499");
    let tcp_listener =
        TcpListener::bind(GetServiceSocketAddress::get_service_socket_address(&config))
            .await
            .expect("3f294e7c");
    let cors_origins = parse_cors_allow_origin(GetCorsAllowOrigin::get_cors_allow_origin(&config));
    let app_state = mk_app_state(config, pg_pool);
    let api_routes = mk_api_routes(&app_state);
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(10)
            .finish()
            .expect("b7e3a4f1"),
    );
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
}
fn main() {
    init_tracing();
    mk_runtime().block_on(run_server());
}
#[cfg(test)]
mod tests {
    use super::{
        CORS_ALLOW_ORIGIN_SPLIT_CH, TRACING_DFLT_FILTER, parse_comma_separated_header_values,
        parse_cors_allow_origin, parse_cors_allow_origin_value, parse_separated_values,
        split_by_char,
    };
    use axum::http::HeaderValue;
    #[allow(clippy::single_call_fn)] // shared fixture keeps two-origin header expectations reusable across parser tests
    fn mk_two_origin_headers() -> Vec<HeaderValue> {
        vec![
            HeaderValue::from_static("https://a.example"),
            HeaderValue::from_static("https://b.example"),
        ]
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps valid CORS header vector checks reusable across parser entry points
    fn assert_two_origin_headers(v: &[HeaderValue]) {
        assert_eq!(v, mk_two_origin_headers());
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps split behavior checks concise across separator tests
    fn assert_split_by_char_parts(input: &str, split_ch: char, exp: &[&str]) {
        let parts = split_by_char(input, split_ch).collect::<Vec<_>>();
        assert_eq!(parts, exp);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps numeric parser checks consistent across separator helpers
    fn assert_parsed_u8_values(input: &str, split_ch: char, exp: &[u8]) {
        let parsed = parse_separated_values(input, split_ch, |part| part.parse::<u8>().ok());
        assert_eq!(parsed, exp);
    }
    #[test]
    fn parse_cors_allow_origin_keeps_valid_values() {
        let v = parse_cors_allow_origin("https://a.example, https://b.example");
        assert_two_origin_headers(&v);
    }
    #[test]
    fn parse_cors_allow_origin_skips_invalid_values() {
        let v = parse_cors_allow_origin("https://ok.example,bad\nvalue");
        assert_eq!(v, vec![HeaderValue::from_static("https://ok.example")]);
    }
    #[test]
    fn parse_cors_allow_origin_keeps_empty_item_behavior() {
        let v = parse_cors_allow_origin("");
        assert_eq!(v, vec![HeaderValue::from_static("")]);
    }
    #[test]
    fn parse_cors_allow_origin_value_trims_and_parses_valid_header() {
        assert_eq!(
            parse_cors_allow_origin_value(" https://a.example "),
            Some(HeaderValue::from_static("https://a.example"))
        );
    }
    #[test]
    fn parse_cors_allow_origin_value_returns_none_for_invalid_header() {
        assert!(parse_cors_allow_origin_value("bad\nvalue").is_none());
    }
    #[test]
    fn split_by_char_preserves_empty_segments_for_cors_separator() {
        assert_split_by_char_parts("a,,b,", CORS_ALLOW_ORIGIN_SPLIT_CH, &["a", "", "b", ""]);
    }
    #[test]
    fn parse_comma_separated_header_values_keeps_only_valid_values() {
        let parsed =
            parse_comma_separated_header_values("https://a.example,bad\nvalue, https://b.example");
        assert_two_origin_headers(&parsed);
    }
    #[test]
    fn parse_comma_separated_values_supports_non_header_parser() {
        assert_parsed_u8_values("1,2,nope,3", CORS_ALLOW_ORIGIN_SPLIT_CH, &[1, 2, 3]);
    }
    #[test]
    fn split_by_char_supports_custom_separator() {
        assert_split_by_char_parts("a;b;;", ';', &["a", "b", "", ""]);
    }
    #[test]
    fn parse_separated_values_supports_custom_separator() {
        assert_parsed_u8_values("10;20;bad;30", ';', &[10, 20, 30]);
    }
    #[test]
    fn cors_allow_origin_split_char_is_stable() {
        assert_eq!(CORS_ALLOW_ORIGIN_SPLIT_CH, ',');
    }
    #[test]
    fn tracing_default_filter_is_stable() {
        assert_eq!(TRACING_DFLT_FILTER, "info");
    }
}
