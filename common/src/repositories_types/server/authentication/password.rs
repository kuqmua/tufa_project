#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum GetStoredCredentialsErrorNamed {
    FetchOptional {
        #[eo_to_std_string_string]
        fetch_optional: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum ValidateCredentialsErrorNamed {
    GetStoredCredentials {
        #[eo_error_occurence]
        get_stored_credentials: GetStoredCredentialsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SpawnBlockingWithTracing {
        #[eo_to_std_string_string]
        spawn_blocking_with_tracing: tokio::task::JoinError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    VerifyPasswordHash {
        #[eo_error_occurence]
        spawn_blocking_with_tracing: VerifyPasswordHashErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnknownUsername {
        #[eo_to_std_string_string_serialize_deserialize]
        message: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum VerifyPasswordHashErrorNamed {
    ExposeSecret {
        #[eo_to_std_string_string]
        expose_secret: argon2::password_hash::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    InvalidPassword {
        #[eo_to_std_string_string]
        invalid_password: argon2::password_hash::Error, //todo - should add here or just addd message?
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum ChangePasswordErrorNamed {
    SpawnBlockingWithTracing {
        #[eo_to_std_string_string]
        spawn_blocking_with_tracing: tokio::task::JoinError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ComputePasswordHash {
        #[eo_error_occurence]
        compute_password_hash: ComputePasswordHashErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PostgresQuery {
        #[eo_to_std_string_string]
        query_error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum ComputePasswordHashErrorNamed {
    PasswordHash {
        #[eo_to_std_string_string]
        argon2_password_hash_error: argon2::password_hash::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
