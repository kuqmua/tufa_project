#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum TryBuildServer {
    // NewRedisSessionStore {
    //     #[eo_to_std_string_string_serialize_deserialize]
    //     new_redis_session_store: std::string::String,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    HttpServerListen {
        #[eo_to_std_string_string]
        http_server_listen: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
