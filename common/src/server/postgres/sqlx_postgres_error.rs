#[derive(
    Debug,
    thiserror::Error,
    error_occurence::ErrorOccurence,
    // type_variants_from_reqwest_response::TypeVariantsFromReqwestResponseFromChecker,
)]
// #[type_variants_from_reqwest_response::type_variants_from_reqwest_response_from_checker_paths(
//     crate::repositories_types::server::routes::api::cats::TryCreateMany,
//     crate::repositories_types::server::routes::api::cats::TryCreateOne,
//     crate::repositories_types::server::routes::api::cats::TryReadMany,
//     crate::repositories_types::server::routes::api::cats::TryReadOne,
//     crate::repositories_types::server::routes::api::cats::TryUpdateMany,
//     crate::repositories_types::server::routes::api::cats::TryUpdateOne,
//     crate::repositories_types::server::routes::api::cats::TryDeleteMany,
//     crate::repositories_types::server::routes::api::cats::TryDeleteOne
// )]
pub enum SqlxPostgresErrorErrorNamed {
    // #[tvfrr_500_internal_server_error]
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_404_not_found]
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_400_bad_request]
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_400_bad_request]
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_408_request_timeout]
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    // #[tvfrr_500_internal_server_error]
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    //#[non_exhaustive] case
    // #[tvfrr_500_internal_server_error]
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl std::convert::From<sqlx::Error> for SqlxPostgresErrorErrorNamed {
    fn from(e: sqlx::Error) -> SqlxPostgresErrorErrorNamed {
        // todo https://github.com/cschaible/actix-web-security-samples/blob/46bb7aa62ada7cb176d8765e2f60b497392b1840/oauth-resource-server/backend/src/error/mod.rs#L46
        // todo https://www.postgresql.org/docs/current/errcodes-appendix.html
        match e {
            sqlx::Error::Configuration(value) => {
                SqlxPostgresErrorErrorNamed::Configuration {
                    configuration: value.to_string(),
                    code_occurence: crate::code_occurence_common!(),
                }
            }
            sqlx::Error::Database(database) => {
                SqlxPostgresErrorErrorNamed::Database {
                    database: database.message().to_string(),
                    code_occurence: crate::code_occurence_common!(),
                }
            }
            sqlx::Error::Io(io) => SqlxPostgresErrorErrorNamed::Io {
                io,
                code_occurence: crate::code_occurence_common!(),
            },
            sqlx::Error::Tls(value) => SqlxPostgresErrorErrorNamed::Tls {
                tls: value.to_string(),
                code_occurence: crate::code_occurence_common!(),
            },
            sqlx::Error::Protocol(string) => SqlxPostgresErrorErrorNamed::Protocol {
                protocol: string,
                code_occurence: crate::code_occurence_common!(),
            },
            sqlx::Error::RowNotFound => SqlxPostgresErrorErrorNamed::RowNotFound {
                row_not_found: std::string::String::from("row_not_found"),
                code_occurence: crate::code_occurence_common!(),
            },
            sqlx::Error::TypeNotFound { type_name } => SqlxPostgresErrorErrorNamed::TypeNotFound {
                type_not_found: type_name,
                code_occurence: crate::code_occurence_common!(),
            },
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                SqlxPostgresErrorErrorNamed::ColumnIndexOutOfBounds {
                    column_index_out_of_bounds: index,
                    len,
                    code_occurence: crate::code_occurence_common!(),
                }
            }
            sqlx::Error::ColumnNotFound(column_not_found) => {
                SqlxPostgresErrorErrorNamed::ColumnNotFound {
                    column_not_found,
                    code_occurence: crate::code_occurence_common!(),
                }
            }
            sqlx::Error::ColumnDecode { index, source } => {
                SqlxPostgresErrorErrorNamed::ColumnDecode {
                    column_decode_index: index,
                    source_handle: source.to_string(),
                    code_occurence: crate::code_occurence_common!(),
                }
            }
            sqlx::Error::Decode(value) => SqlxPostgresErrorErrorNamed::Decode {
                decode: value.to_string(),
                code_occurence: crate::code_occurence_common!(),
            },
            sqlx::Error::PoolTimedOut => SqlxPostgresErrorErrorNamed::PoolTimedOut {
                pool_timed_out: std::string::String::from("pool timed out"),
                code_occurence: crate::code_occurence_common!(),
            },
            sqlx::Error::PoolClosed => SqlxPostgresErrorErrorNamed::PoolClosed {
                pool_closed: std::string::String::from("pool closed"),
                code_occurence: crate::code_occurence_common!(),
            },
            sqlx::Error::WorkerCrashed => SqlxPostgresErrorErrorNamed::WorkerCrashed {
                worker_crashed: std::string::String::from("worker crashed"),
                code_occurence: crate::code_occurence_common!(),
            },
            sqlx::Error::Migrate(migrate_error) => SqlxPostgresErrorErrorNamed::Migrate {
                migrate: *migrate_error,
                code_occurence: crate::code_occurence_common!(),
            },
            _ => SqlxPostgresErrorErrorNamed::UnexpectedCase {
                unexpected_case: std::string::String::from("unexpected_case"),
                code_occurence: crate::code_occurence_common!(),
            },
        }
    }
}

//////////////////////////////////

impl std::convert::From<SqlxPostgresErrorErrorNamed>
    for crate::repositories_types::server::routes::api::cats::TryCreateManyResponseVariants
{
    fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
        match val.into_serialize_deserialize_version() {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamed>
    for crate::repositories_types::server::routes::api::cats::TryCreateOneResponseVariants
{
    fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
        match val.into_serialize_deserialize_version() {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamed>
    for crate::repositories_types::server::routes::api::cats::TryReadManyResponseVariants
{
    fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
        match val.into_serialize_deserialize_version() {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamed>
    for crate::repositories_types::server::routes::api::cats::TryReadOneResponseVariants
{
    fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
        match val.into_serialize_deserialize_version() {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamed>
    for crate::repositories_types::server::routes::api::cats::TryUpdateManyResponseVariants
{
    fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
        match val.into_serialize_deserialize_version() {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamed>
    for crate::repositories_types::server::routes::api::cats::TryUpdateOneResponseVariants
{
    fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
        match val.into_serialize_deserialize_version() {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamed>
    for crate::repositories_types::server::routes::api::cats::TryDeleteManyResponseVariants
{
    fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
        match val.into_serialize_deserialize_version() {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamed>
    for crate::repositories_types::server::routes::api::cats::TryDeleteOneResponseVariants
{
    fn from(val: SqlxPostgresErrorErrorNamed) -> Self {
        match val.into_serialize_deserialize_version() {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Configuration {
                configuration,
                code_occurence,
            } => Self::Configuration {
                configuration,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Database {
                database,
                code_occurence,
            } => Self::Database {
                database,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Io { io, code_occurence } => {
                Self::Io { io, code_occurence }
            }
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Tls {
                tls,
                code_occurence,
            } => Self::Tls {
                tls,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Protocol {
                protocol,
                code_occurence,
            } => Self::Protocol {
                protocol,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::TypeNotFound {
                type_not_found,
                code_occurence,
            } => Self::TypeNotFound {
                type_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            } => Self::ColumnIndexOutOfBounds {
                column_index_out_of_bounds,
                len,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnNotFound {
                column_not_found,
                code_occurence,
            } => Self::ColumnNotFound {
                column_not_found,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            } => Self::ColumnDecode {
                column_decode_index,
                source_handle,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Decode {
                decode,
                code_occurence,
            } => Self::Decode {
                decode,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::PoolClosed {
                pool_closed,
                code_occurence,
            } => Self::PoolClosed {
                pool_closed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::WorkerCrashed {
                worker_crashed,
                code_occurence,
            } => Self::WorkerCrashed {
                worker_crashed,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::Migrate {
                migrate,
                code_occurence,
            } => Self::Migrate {
                migrate,
                code_occurence,
            },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize::UnexpectedCase {
                unexpected_case,
                code_occurence,
            } => Self::UnexpectedCase {
                unexpected_case,
                code_occurence,
            },
        }
    }
}
impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for
crate::repositories_types::server::routes::api::cats::TryCreateManyWithSerializeDeserialize
{
    fn from(val : SqlxPostgresErrorErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence }
        }
    }
}
impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for
crate::repositories_types::server::routes::api::cats::TryCreateOneWithSerializeDeserialize
{
    fn from(val : SqlxPostgresErrorErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence }
        }
    }
}
impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for
crate::repositories_types::server::routes::api::cats::TryReadManyWithSerializeDeserialize
{
    fn from(val : SqlxPostgresErrorErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence }
        }
    }
}
impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for
crate::repositories_types::server::routes::api::cats::TryReadOneWithSerializeDeserialize
{
    fn from(val : SqlxPostgresErrorErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence }
        }
    }
}
impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for
crate::repositories_types::server::routes::api::cats::TryUpdateManyWithSerializeDeserialize
{
    fn from(val : SqlxPostgresErrorErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence }
        }
    }
}
impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for
crate::repositories_types::server::routes::api::cats::TryUpdateOneWithSerializeDeserialize
{
    fn from(val : SqlxPostgresErrorErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence }
        }
    }
}
impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for
crate::repositories_types::server::routes::api::cats::TryDeleteManyWithSerializeDeserialize
{
    fn from(val : SqlxPostgresErrorErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence }
        }
    }
}
impl std :: convert :: From < SqlxPostgresErrorErrorNamed > for
crate::repositories_types::server::routes::api::cats::TryDeleteOneWithSerializeDeserialize
{
    fn from(val : SqlxPostgresErrorErrorNamed) -> Self
    {
        match val.into_serialize_deserialize_version()
        {
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            TypeNotFound { type_not_found, code_occurence } => Self ::
            TypeNotFound { type_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnNotFound { column_not_found, code_occurence } => Self ::
            ColumnNotFound { column_not_found, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            SqlxPostgresErrorErrorNamedWithSerializeDeserialize ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence }
        }
    }
}
impl From<&SqlxPostgresErrorErrorNamed> for http::StatusCode {
    fn from(val: &SqlxPostgresErrorErrorNamed) -> Self {
        match &val {
            SqlxPostgresErrorErrorNamed::Configuration {
                configuration: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::Database {
                database: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::Io {
                io: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::Tls {
                tls: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::Protocol {
                protocol: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::RowNotFound {
                row_not_found: _,
                code_occurence: _,
            } => http::StatusCode::NOT_FOUND,
            SqlxPostgresErrorErrorNamed::TypeNotFound {
                type_not_found: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            SqlxPostgresErrorErrorNamed::ColumnIndexOutOfBounds {
                column_index_out_of_bounds: _,
                len: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::ColumnNotFound {
                column_not_found: _,
                code_occurence: _,
            } => http::StatusCode::BAD_REQUEST,
            SqlxPostgresErrorErrorNamed::ColumnDecode {
                column_decode_index: _,
                source_handle: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::Decode {
                decode: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::PoolTimedOut {
                pool_timed_out: _,
                code_occurence: _,
            } => http::StatusCode::REQUEST_TIMEOUT,
            SqlxPostgresErrorErrorNamed::PoolClosed {
                pool_closed: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::WorkerCrashed {
                worker_crashed: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::Migrate {
                migrate: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
            SqlxPostgresErrorErrorNamed::UnexpectedCase {
                unexpected_case: _,
                code_occurence: _,
            } => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
#[allow(clippy::enum_variant_names, dead_code)]
enum SqlxPostgresErrorErrorNamedStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
}
impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
    for crate::repositories_types::server::routes::api::cats::TryCreateManyStatusCodesChecker
{
    fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ConfigurationTvfrr500InternalServerError => Self ::
            ConfigurationTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DatabaseTvfrr500InternalServerError => Self ::
            DatabaseTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            IoTvfrr500InternalServerError => Self ::
            IoTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TlsTvfrr500InternalServerError => Self ::
            TlsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ProtocolTvfrr500InternalServerError => Self ::
            ProtocolTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            RowNotFoundTvfrr404NotFound => Self ::
            RowNotFoundTvfrr404NotFound,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TypeNotFoundTvfrr400BadRequest => Self ::
            TypeNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError => Self ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnNotFoundTvfrr400BadRequest => Self ::
            ColumnNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnDecodeTvfrr500InternalServerError => Self ::
            ColumnDecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DecodeTvfrr500InternalServerError => Self ::
            DecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolTimedOutTvfrr408RequestTimeout => Self ::
            PoolTimedOutTvfrr408RequestTimeout,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolClosedTvfrr500InternalServerError => Self ::
            PoolClosedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            WorkerCrashedTvfrr500InternalServerError => Self ::
            WorkerCrashedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            MigrateTvfrr500InternalServerError => Self ::
            MigrateTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            UnexpectedCaseTvfrr500InternalServerError => Self ::
            UnexpectedCaseTvfrr500InternalServerError
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
    for crate::repositories_types::server::routes::api::cats::TryCreateOneStatusCodesChecker
{
    fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ConfigurationTvfrr500InternalServerError => Self ::
            ConfigurationTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DatabaseTvfrr500InternalServerError => Self ::
            DatabaseTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            IoTvfrr500InternalServerError => Self ::
            IoTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TlsTvfrr500InternalServerError => Self ::
            TlsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ProtocolTvfrr500InternalServerError => Self ::
            ProtocolTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            RowNotFoundTvfrr404NotFound => Self ::
            RowNotFoundTvfrr404NotFound,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TypeNotFoundTvfrr400BadRequest => Self ::
            TypeNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError => Self ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnNotFoundTvfrr400BadRequest => Self ::
            ColumnNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnDecodeTvfrr500InternalServerError => Self ::
            ColumnDecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DecodeTvfrr500InternalServerError => Self ::
            DecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolTimedOutTvfrr408RequestTimeout => Self ::
            PoolTimedOutTvfrr408RequestTimeout,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolClosedTvfrr500InternalServerError => Self ::
            PoolClosedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            WorkerCrashedTvfrr500InternalServerError => Self ::
            WorkerCrashedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            MigrateTvfrr500InternalServerError => Self ::
            MigrateTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            UnexpectedCaseTvfrr500InternalServerError => Self ::
            UnexpectedCaseTvfrr500InternalServerError
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
    for crate::repositories_types::server::routes::api::cats::TryReadManyStatusCodesChecker
{
    fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ConfigurationTvfrr500InternalServerError => Self ::
            ConfigurationTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DatabaseTvfrr500InternalServerError => Self ::
            DatabaseTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            IoTvfrr500InternalServerError => Self ::
            IoTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TlsTvfrr500InternalServerError => Self ::
            TlsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ProtocolTvfrr500InternalServerError => Self ::
            ProtocolTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            RowNotFoundTvfrr404NotFound => Self ::
            RowNotFoundTvfrr404NotFound,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TypeNotFoundTvfrr400BadRequest => Self ::
            TypeNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError => Self ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnNotFoundTvfrr400BadRequest => Self ::
            ColumnNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnDecodeTvfrr500InternalServerError => Self ::
            ColumnDecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DecodeTvfrr500InternalServerError => Self ::
            DecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolTimedOutTvfrr408RequestTimeout => Self ::
            PoolTimedOutTvfrr408RequestTimeout,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolClosedTvfrr500InternalServerError => Self ::
            PoolClosedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            WorkerCrashedTvfrr500InternalServerError => Self ::
            WorkerCrashedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            MigrateTvfrr500InternalServerError => Self ::
            MigrateTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            UnexpectedCaseTvfrr500InternalServerError => Self ::
            UnexpectedCaseTvfrr500InternalServerError
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
    for crate::repositories_types::server::routes::api::cats::TryReadOneStatusCodesChecker
{
    fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ConfigurationTvfrr500InternalServerError => Self ::
            ConfigurationTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DatabaseTvfrr500InternalServerError => Self ::
            DatabaseTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            IoTvfrr500InternalServerError => Self ::
            IoTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TlsTvfrr500InternalServerError => Self ::
            TlsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ProtocolTvfrr500InternalServerError => Self ::
            ProtocolTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            RowNotFoundTvfrr404NotFound => Self ::
            RowNotFoundTvfrr404NotFound,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TypeNotFoundTvfrr400BadRequest => Self ::
            TypeNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError => Self ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnNotFoundTvfrr400BadRequest => Self ::
            ColumnNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnDecodeTvfrr500InternalServerError => Self ::
            ColumnDecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DecodeTvfrr500InternalServerError => Self ::
            DecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolTimedOutTvfrr408RequestTimeout => Self ::
            PoolTimedOutTvfrr408RequestTimeout,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolClosedTvfrr500InternalServerError => Self ::
            PoolClosedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            WorkerCrashedTvfrr500InternalServerError => Self ::
            WorkerCrashedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            MigrateTvfrr500InternalServerError => Self ::
            MigrateTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            UnexpectedCaseTvfrr500InternalServerError => Self ::
            UnexpectedCaseTvfrr500InternalServerError
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
    for crate::repositories_types::server::routes::api::cats::TryUpdateManyStatusCodesChecker
{
    fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ConfigurationTvfrr500InternalServerError => Self ::
            ConfigurationTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DatabaseTvfrr500InternalServerError => Self ::
            DatabaseTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            IoTvfrr500InternalServerError => Self ::
            IoTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TlsTvfrr500InternalServerError => Self ::
            TlsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ProtocolTvfrr500InternalServerError => Self ::
            ProtocolTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            RowNotFoundTvfrr404NotFound => Self ::
            RowNotFoundTvfrr404NotFound,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TypeNotFoundTvfrr400BadRequest => Self ::
            TypeNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError => Self ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnNotFoundTvfrr400BadRequest => Self ::
            ColumnNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnDecodeTvfrr500InternalServerError => Self ::
            ColumnDecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DecodeTvfrr500InternalServerError => Self ::
            DecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolTimedOutTvfrr408RequestTimeout => Self ::
            PoolTimedOutTvfrr408RequestTimeout,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolClosedTvfrr500InternalServerError => Self ::
            PoolClosedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            WorkerCrashedTvfrr500InternalServerError => Self ::
            WorkerCrashedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            MigrateTvfrr500InternalServerError => Self ::
            MigrateTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            UnexpectedCaseTvfrr500InternalServerError => Self ::
            UnexpectedCaseTvfrr500InternalServerError
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
    for crate::repositories_types::server::routes::api::cats::TryUpdateOneStatusCodesChecker
{
    fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ConfigurationTvfrr500InternalServerError => Self ::
            ConfigurationTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DatabaseTvfrr500InternalServerError => Self ::
            DatabaseTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            IoTvfrr500InternalServerError => Self ::
            IoTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TlsTvfrr500InternalServerError => Self ::
            TlsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ProtocolTvfrr500InternalServerError => Self ::
            ProtocolTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            RowNotFoundTvfrr404NotFound => Self ::
            RowNotFoundTvfrr404NotFound,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TypeNotFoundTvfrr400BadRequest => Self ::
            TypeNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError => Self ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnNotFoundTvfrr400BadRequest => Self ::
            ColumnNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnDecodeTvfrr500InternalServerError => Self ::
            ColumnDecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DecodeTvfrr500InternalServerError => Self ::
            DecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolTimedOutTvfrr408RequestTimeout => Self ::
            PoolTimedOutTvfrr408RequestTimeout,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolClosedTvfrr500InternalServerError => Self ::
            PoolClosedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            WorkerCrashedTvfrr500InternalServerError => Self ::
            WorkerCrashedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            MigrateTvfrr500InternalServerError => Self ::
            MigrateTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            UnexpectedCaseTvfrr500InternalServerError => Self ::
            UnexpectedCaseTvfrr500InternalServerError
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
    for crate::repositories_types::server::routes::api::cats::TryDeleteManyStatusCodesChecker
{
    fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ConfigurationTvfrr500InternalServerError => Self ::
            ConfigurationTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DatabaseTvfrr500InternalServerError => Self ::
            DatabaseTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            IoTvfrr500InternalServerError => Self ::
            IoTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TlsTvfrr500InternalServerError => Self ::
            TlsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ProtocolTvfrr500InternalServerError => Self ::
            ProtocolTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            RowNotFoundTvfrr404NotFound => Self ::
            RowNotFoundTvfrr404NotFound,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TypeNotFoundTvfrr400BadRequest => Self ::
            TypeNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError => Self ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnNotFoundTvfrr400BadRequest => Self ::
            ColumnNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnDecodeTvfrr500InternalServerError => Self ::
            ColumnDecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DecodeTvfrr500InternalServerError => Self ::
            DecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolTimedOutTvfrr408RequestTimeout => Self ::
            PoolTimedOutTvfrr408RequestTimeout,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolClosedTvfrr500InternalServerError => Self ::
            PoolClosedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            WorkerCrashedTvfrr500InternalServerError => Self ::
            WorkerCrashedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            MigrateTvfrr500InternalServerError => Self ::
            MigrateTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            UnexpectedCaseTvfrr500InternalServerError => Self ::
            UnexpectedCaseTvfrr500InternalServerError
        }
    }
}
impl std::convert::From<SqlxPostgresErrorErrorNamedStatusCodesChecker>
    for crate::repositories_types::server::routes::api::cats::TryDeleteOneStatusCodesChecker
{
    fn from(val: SqlxPostgresErrorErrorNamedStatusCodesChecker) -> Self {
        match val
        {
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ConfigurationTvfrr500InternalServerError => Self ::
            ConfigurationTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DatabaseTvfrr500InternalServerError => Self ::
            DatabaseTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            IoTvfrr500InternalServerError => Self ::
            IoTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TlsTvfrr500InternalServerError => Self ::
            TlsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ProtocolTvfrr500InternalServerError => Self ::
            ProtocolTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            RowNotFoundTvfrr404NotFound => Self ::
            RowNotFoundTvfrr404NotFound,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            TypeNotFoundTvfrr400BadRequest => Self ::
            TypeNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError => Self ::
            ColumnIndexOutOfBoundsTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnNotFoundTvfrr400BadRequest => Self ::
            ColumnNotFoundTvfrr400BadRequest,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            ColumnDecodeTvfrr500InternalServerError => Self ::
            ColumnDecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            DecodeTvfrr500InternalServerError => Self ::
            DecodeTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolTimedOutTvfrr408RequestTimeout => Self ::
            PoolTimedOutTvfrr408RequestTimeout,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            PoolClosedTvfrr500InternalServerError => Self ::
            PoolClosedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            WorkerCrashedTvfrr500InternalServerError => Self ::
            WorkerCrashedTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            MigrateTvfrr500InternalServerError => Self ::
            MigrateTvfrr500InternalServerError,
            SqlxPostgresErrorErrorNamedStatusCodesChecker ::
            UnexpectedCaseTvfrr500InternalServerError => Self ::
            UnexpectedCaseTvfrr500InternalServerError
        }
    }
}
