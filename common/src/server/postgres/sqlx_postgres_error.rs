#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurence,
    type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
#[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
    crate::repositories_types::server::routes::api::cats::TryCreateMany
    ,
    // crate::repositories_types::server::routes::api::cats::TryCreateOne
    // ,
    crate::repositories_types::server::routes::api::cats::TryReadMany
    ,
    crate::repositories_types::server::routes::api::cats::TryReadOne
    // ,
    // crate::repositories_types::server::routes::api::cats::TryUpdateMany
    // ,
    // crate::repositories_types::server::routes::api::cats::TryUpdateOne
    // ,
    // crate::repositories_types::server::routes::api::cats::TryDeleteMany
    // ,
    // crate::repositories_types::server::routes::api::cats::TryDeleteOne
)]
pub enum SqlxPostgresErrorNamed {
    #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl std::convert::From<sqlx::Error> for SqlxPostgresErrorNamed {
    fn from(e: sqlx::Error) -> Self {
        // todo https://github.com/cschaible/actix-web-security-samples/blob/46bb7aa62ada7cb176d8765e2f60b497392b1840/oauth-resource-server/backend/src/error/mod.rs#L46
        // todo https://www.postgresql.org/docs/current/errcodes-appendix.html
        match e {
            sqlx::Error::Configuration(value) => Self::Configuration {
                configuration: value.to_string(),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::Database(database) => Self::Database {
                database: database.message().to_string(),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::Io(io) => Self::Io {
                io,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::Tls(value) => Self::Tls {
                tls: value.to_string(),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::Protocol(string) => Self::Protocol {
                protocol: string,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::RowNotFound => Self::RowNotFound {
                row_not_found: std::string::String::from("row_not_found"),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::TypeNotFound { type_name } => Self::TypeNotFound {
                type_not_found: type_name,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: index,
                len,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::ColumnNotFound(column_not_found) => Self::ColumnNotFound {
                column_not_found,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::ColumnDecode { index, source } => Self::ColumnDecode {
                column_decode_index: index,
                source_handle: source.to_string(),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::Decode(value) => Self::Decode {
                decode: value.to_string(),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::PoolTimedOut => Self::PoolTimedOut {
                pool_timed_out: std::string::String::from("pool timed out"),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::PoolClosed => Self::PoolClosed {
                pool_closed: std::string::String::from("pool closed"),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::WorkerCrashed => Self::WorkerCrashed {
                worker_crashed: std::string::String::from("worker crashed"),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            sqlx::Error::Migrate(migrate_error) => Self::Migrate {
                migrate: *migrate_error,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
            _ => Self::UnexpectedCase {
                unexpected_case: std::string::String::from("unexpected_case"),
                code_occurence: error_occurence_lib::code_occurence!(),
            },
        }
    }
}

//////////////////////////////////
