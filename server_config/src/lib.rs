use chrono::FixedOffset;
use config_lib::types::{SourcePlaceType, TracingLevel};
use secrecy::SecretBox;
use std::net::SocketAddr;
use thiserror::Error;
#[derive(Debug, config_lib::TryFromEnv, config_lib::GenGetterTraitsForStructFields)]
pub struct Config {
    //todo maybe auto gen .env and docker-compose environment variables. and maybe write in directly into files
    pub database_url: SecretBox<String>,
    pub enable_api_git_commit_check: bool,
    pub maximum_size_of_http_body_in_bytes: usize,
    pub service_socket_address: SocketAddr,
    pub source_place_type: SourcePlaceType,
    pub timezone: FixedOffset,
    pub tracing_level: TracingLevel,
}
