pub static CONFIG: std::sync::OnceLock<
    common::repositories_types::server::config::config_struct::Config,
> = std::sync::OnceLock::new();
//todo - maybe Arc<RwLock<Store>> ?
