pub static CONFIG: std::sync::OnceLock<
    tufa_common::repositories_types::tufa_server::config::config_struct::Config,
> = std::sync::OnceLock::new();
//todo - maybe Arc<RwLock<Store>> ?
