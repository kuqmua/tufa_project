#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryBuildServer {
    // NewRedisSessionStore {
    //     #[eo_display_with_serialize_deserialize]
    //     new_redis_session_store: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    HttpServerListen {
        #[eo_display]
        http_server_listen: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
