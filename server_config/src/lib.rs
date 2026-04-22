use chrono::FixedOffset;
use config_lib::{
    GenGetterTraitsForStructFields, TryFromEnv,
    types::{SrcPlaceType, TracingLevel},
};
use optml::Optml;
use secrecy::SecretBox;
use std::net::SocketAddr;
use thiserror::Error;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, TryFromEnv, GenGetterTraitsForStructFields, Optml)]
pub struct Config {
    //todo mb auto gen .env and docker-compose environment variables. and mb write in directly into files
    pub cors_allow_origin: String,
    pub database_url: SecretBox<String>,
    pub maximum_size_of_http_body_in_bytes: usize,
    pub service_socket_address: SocketAddr,
    pub pg_pool_max_connections: u32,
    pub timezone: FixedOffset,
    pub src_place_type: SrcPlaceType,
    pub tracing_level: TracingLevel,
    pub enable_api_git_commit_check: bool,
}
#[cfg(test)]
mod tests {
    use super::Config;
    use app_state::{
        GetCorsAllowOrigin as _, GetDatabaseUrl as _, GetEnableApiGitCommitCheck as _,
        GetMaximumSizeOfHttpBodyInBytes as _, GetPgPoolMaxConnections as _,
        GetServiceSocketAddress as _, GetSrcPlaceType as _, GetTimezone as _, GetTracingLevel as _,
    };
    use config_lib::types::{SrcPlaceType, TracingLevel};
    use secrecy::{ExposeSecret as _, SecretBox};
    #[test]
    fn generated_getters_return_expected_refs_and_values() {
        let cfg = Config {
            cors_allow_origin: "*".to_owned(),
            database_url: SecretBox::new(Box::new("postgres://db".to_owned())),
            maximum_size_of_http_body_in_bytes: 16_384,
            service_socket_address: "127.0.0.1:8080".parse().expect("e7a3d5c1"),
            pg_pool_max_connections: 8,
            timezone: chrono::FixedOffset::east_opt(3i32 * 3_600i32).expect("93cbf4a2"),
            src_place_type: SrcPlaceType::Github,
            tracing_level: TracingLevel::Info,
            enable_api_git_commit_check: true,
        };
        assert_eq!(cfg.get_cors_allow_origin(), "*");
        assert_eq!(cfg.get_database_url().expose_secret(), "postgres://db");
        assert_eq!(cfg.get_maximum_size_of_http_body_in_bytes(), &16_384);
        assert_eq!(cfg.get_service_socket_address().port(), 8080);
        assert_eq!(cfg.get_pg_pool_max_connections(), &8);
        assert_eq!(cfg.get_timezone().local_minus_utc(), 3i32 * 3_600i32);
        assert_eq!(cfg.get_src_place_type(), &SrcPlaceType::Github);
        assert_eq!(cfg.get_tracing_level(), &TracingLevel::Info);
        assert!(cfg.get_enable_api_git_commit_check());
    }
}
