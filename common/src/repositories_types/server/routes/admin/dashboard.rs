#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum GetUsernameErrorNamed {
    PostgresQuery {
        #[eo_to_std_string_string]
        get_username: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
