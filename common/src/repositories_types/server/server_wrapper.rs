#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum ServerWrapperErrorNamed {
    // TryGetRedisSessionStorage {
    //     #[eo_to_std_string_string]
    //     try_get_redis_session_storage: crate::common::config::try_get_redis_session_storage::TryGetRedisSessionStorageError,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    BuildServer {
        #[eo_error_occurence]
        build_server: crate::repositories_types::server::try_build_server::TryBuildServer,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
