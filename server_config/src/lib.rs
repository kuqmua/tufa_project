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
