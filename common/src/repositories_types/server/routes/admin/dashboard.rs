#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum GetUsernameErrorNamed {
    PostgresQuery {
        #[eo_display]
        get_username: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
