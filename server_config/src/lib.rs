use chrono::FixedOffset;
use config_lib::{
    GenGetterTraitsForStructFields, TryFromEnv,
    types::{SourcePlaceType, TracingLevel},
};
use optml::Optml;
use secrecy::SecretBox;
use std::net::SocketAddr;
use thiserror::Error;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, TryFromEnv, GenGetterTraitsForStructFields, Optml)]
pub struct Config {
    //todo mb auto gen .env and docker-compose environment variables. and mb write in directly into files
    pub maximum_size_of_http_body_in_bytes: usize,
    pub database_url: SecretBox<String>,
    pub service_socket_address: SocketAddr,
    pub timezone: FixedOffset,
    pub source_place_type: SourcePlaceType,
    pub tracing_level: TracingLevel,
    pub enable_api_git_commit_check: bool,
}
