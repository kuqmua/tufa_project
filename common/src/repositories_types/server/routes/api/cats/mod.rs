//todo openapi
//todo test if create\update\delete empty array

#[derive(
    Debug,
    postgresql_crud::GeneratePostgresqlCrud,
)]
#[postgresql_crud::create_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::create_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::additional_http_status_codes_error_variants{
    #[path(crate::server::extractors::commit_extractor::)]
    enum CommitExtractorCheckErrorNamed {
        #[tvfrr_400_bad_request]
        CommitExtractorNotEqual {
            #[eo_display_with_serialize_deserialize]
            commit_not_equal: std::string::String,
            #[eo_display_with_serialize_deserialize]
            commit_to_use: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        CommitExtractorToStrConversion {
            #[eo_display]
            commit_to_str_conversion: http::header::ToStrError,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        NoCommitExtractorHeader {
            #[eo_display_with_serialize_deserialize]
            no_commit_header: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
    // ;
    // enum SomethingErrorNamed {
    //     #[tvfrr_400_bad_request]
    //     SomethingVariant {
    //         #[eo_display_with_serialize_deserialize]
    //         something_field: std::string::String,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //     },
    // }
}]
pub struct Dog {
    #[generate_postgresql_crud_primary_key]
    pub id: sqlx::types::Uuid, //todo make it UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    #[generate_postgresql_crud_varchar]
    pub name: std::string::String,
    #[generate_postgresql_crud_varchar]
    pub color: std::string::String,
}


///////////////////////////////
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryDeleteOne {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize:
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteOneResponseVariants {
    Desirable(crate :: server :: postgres :: uuid_wrapper ::
    PossibleUuidWrapper), Configuration
    {
        configuration : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Database
    {
        database : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Io
    {
        io : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Tls
    {
        tls : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Protocol
    {
        protocol : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, RowNotFound
    {
        row_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize<>, len : usize<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std::string::String<>, source_handle :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Decode
    {
        decode : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PoolClosed
    {
        pool_closed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Migrate
    {
        migrate : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
    {
        delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize
        :
        DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorNotEqual
    {
        commit_not_equal : std::string::String<>, commit_to_use :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorToStrConversion
    {
        commit_to_str_conversion : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoCommitExtractorHeader
    {
        no_commit_header : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryDeleteOne> for TryDeleteOneResponseVariants {
    fn from(value: TryDeleteOne) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryDeleteOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: Io { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: RowNotFound { row_not_found, code_occurence } => Self ::
            RowNotFound { row_not_found, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryDeleteOneWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryDeleteOneWithSerializeDeserialize ::
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
                code_occurence
            }, TryDeleteOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryDeleteOneWithSerializeDeserialize :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryDeleteOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryDeleteOneWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryDeleteOneResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryDeleteOneResponseVariants) -> Self {
        match value
        {
            TryDeleteOneResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: Database
            { database : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryDeleteOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            OK, TryDeleteOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            ColumnNotFound { column_not_found : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: OK, TryDeleteOneResponseVariants
            :: Decode { decode : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } => axum :: http :: StatusCode
            :: OK, TryDeleteOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK, TryDeleteOneResponseVariants ::
            MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: OK,
            TryDeleteOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: OK, TryDeleteOneResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: OK
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr200Ok {
    Desirable(crate::server::postgres::uuid_wrapper::PossibleUuidWrapper),
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr200Ok> for TryDeleteOneResponseVariants {
    fn from(value: TryDeleteOneResponseVariantsTvfrr200Ok) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr200Ok::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr500InternalServerError {
    Configuration {
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        io: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        column_index_out_of_bounds: usize,
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        column_decode_index: std::string::String,
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        migrate: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr500InternalServerError>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryDeleteOneResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
    {
        delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize
        :
        DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorNotEqual
    {
        commit_not_equal : std::string::String<>, commit_to_use :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorToStrConversion
    {
        commit_to_str_conversion : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoCommitExtractorHeader
    {
        no_commit_header : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr400BadRequest>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryDeleteOneResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
                code_occurence
            }, TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryDeleteOneResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr404NotFound>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryDeleteOneResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneResponseVariantsTvfrr408RequestTimeout>
    for TryDeleteOneResponseVariants
{
    fn from(value: TryDeleteOneResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryDeleteOneResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
// async fn try_from_response_try_delete_one(
//     response: reqwest::Response,
// ) -> Result<
//     TryDeleteOneResponseVariants,
//     crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
// > {
//     let status_code = response.status();
//     let headers = response.headers().clone();
//     match response.text().await
//     {
//         Ok(response_text) =>
//         {
//             if status_code == http :: StatusCode :: OK
//             {
//                 match serde_json :: from_str :: <
//                 TryDeleteOneResponseVariantsTvfrr200Ok > (& response_text)
//                 {
//                     Ok(value) =>
//                     Ok(TryDeleteOneResponseVariants :: from(value)), Err(e) =>
//                     Err(crate :: common :: api_request_unexpected_error ::
//                     ApiRequestUnexpectedError :: DeserializeBody
//                     { serde : e, status_code, headers, response_text }),
//                 }
//             } else if status_code == http :: StatusCode ::
//             INTERNAL_SERVER_ERROR
//             {
//                 match serde_json :: from_str :: <
//                 TryDeleteOneResponseVariantsTvfrr500InternalServerError >
//                 (& response_text)
//                 {
//                     Ok(value) =>
//                     Ok(TryDeleteOneResponseVariants :: from(value)), Err(e) =>
//                     Err(crate :: common :: api_request_unexpected_error ::
//                     ApiRequestUnexpectedError :: DeserializeBody
//                     { serde : e, status_code, headers, response_text }),
//                 }
//             } else if status_code == http :: StatusCode :: BAD_REQUEST
//             {
//                 match serde_json :: from_str :: <
//                 TryDeleteOneResponseVariantsTvfrr400BadRequest >
//                 (& response_text)
//                 {
//                     Ok(value) =>
//                     Ok(TryDeleteOneResponseVariants :: from(value)), Err(e) =>
//                     Err(crate :: common :: api_request_unexpected_error ::
//                     ApiRequestUnexpectedError :: DeserializeBody
//                     { serde : e, status_code, headers, response_text }),
//                 }
//             } else if status_code == http :: StatusCode :: NOT_FOUND
//             {
//                 match serde_json :: from_str :: <
//                 TryDeleteOneResponseVariantsTvfrr404NotFound >
//                 (& response_text)
//                 {
//                     Ok(value) =>
//                     Ok(TryDeleteOneResponseVariants :: from(value)), Err(e) =>
//                     Err(crate :: common :: api_request_unexpected_error ::
//                     ApiRequestUnexpectedError :: DeserializeBody
//                     { serde : e, status_code, headers, response_text }),
//                 }
//             } else
//             {
//                 Err(crate :: common :: api_request_unexpected_error ::
//                 ApiRequestUnexpectedError :: StatusCode
//                 {
//                     status_code, headers, response_text_result : crate :: common
//                     :: api_request_unexpected_error :: ResponseTextResult ::
//                     ResponseText(response_text)
//                 })
//             }
//         }, 
//         Err(e) => Err(crate :: common :: api_request_unexpected_error ::
//         ApiRequestUnexpectedError :: FailedToGetResponseText
//         { reqwest : e, status_code, headers, }),
//     }
// }
impl TryFrom<TryDeleteOneResponseVariants>
    for crate::server::postgres::uuid_wrapper::PossibleUuidWrapper
{
    type Error = TryDeleteOneWithSerializeDeserialize;
    fn try_from(value: TryDeleteOneResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryDeleteOneResponseVariants :: Desirable(i) => Ok(i),
            TryDeleteOneResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryDeleteOneResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Database
            { database, code_occurence }), TryDeleteOneResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Io
            { io, code_occurence }), TryDeleteOneResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryDeleteOneResponseVariants :: Protocol
            { protocol, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryDeleteOneResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryDeleteOneResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryDeleteOneResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryDeleteOneResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryDeleteOneResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryDeleteOneResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryDeleteOneWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryDeleteOneResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryDeleteOneResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryDeleteOneResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryDeleteOneResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryDeleteOneResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }), TryDeleteOneResponseVariants
            :: JsonSyntaxError { json_syntax_error, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryDeleteOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryDeleteOneResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }), TryDeleteOneResponseVariants
            :: UnexpectedCase { unexpected_case, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }), TryDeleteOneResponseVariants
            :: DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize,
                code_occurence
            }), TryDeleteOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryDeleteOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryDeleteOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryDeleteOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryDeleteOneWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteOneRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryDeleteOneWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
async fn tvfrr_extraction_logic_try_delete_one<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper, TryDeleteOneRequestError> {
    match future.await
    {
        Ok(response) => {
            //
            let match_value = {
            // try_from_response_try_delete_one(response).await
            //
            let status_code = response.status();
            let headers = response.headers().clone();
            match response.text().await {
                Ok(response_text) => {
                    if status_code == http :: StatusCode :: OK {
                        match serde_json :: from_str :: <
                        TryDeleteOneResponseVariantsTvfrr200Ok > (& response_text)
                        {
                            Ok(value) => {
                                Ok(TryDeleteOneResponseVariants :: from(value))
                                as 
                                Result<
                                    TryDeleteOneResponseVariants,
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                                >
                                //
                                // match crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::try_from(variants)
                                // {
                                //     Ok(value) => Ok(value), 
                                //     Err(e) => Err(TryDeleteOneRequestError :: ExpectedType
                                //     {
                                //         expected_type : e, code_occurence : error_occurence_lib ::
                                //         code_occurence :: CodeOccurence ::
                                //         new(crate :: global_variables :: compile_time ::
                                //         project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                                //         file! ().to_string(), line! (), column! (),
                                //         Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                //         {
                                //             file : std :: string :: String ::
                                //             from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                                //             line : 882, column : 17,
                                //         })),
                                //     }),
                                // }
                                //
                            }, 
                            Err(e) => Err(
                                crate :: common :: api_request_unexpected_error ::ApiRequestUnexpectedError :: DeserializeBody{ serde : e, status_code, headers, response_text }
                            )
                            
                                                            as 
                                Result<
                                    TryDeleteOneResponseVariants,
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                                >
                            ,
                        }
                    } else if status_code == http :: StatusCode ::INTERNAL_SERVER_ERROR
                    {
                        match serde_json :: from_str :: <
                        TryDeleteOneResponseVariantsTvfrr500InternalServerError >
                        (& response_text)
                        {
                            Ok(value) => 
                                Ok(TryDeleteOneResponseVariants :: from(value))
                                                            as 
                                Result<
                                    TryDeleteOneResponseVariants,
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                                >    
                            , 
                            Err(e) => 
                            
                                Err(crate :: common :: api_request_unexpected_error ::ApiRequestUnexpectedError :: DeserializeBody{ serde : e, status_code, headers, response_text })
                                
                                                            as 
                                Result<
                                    TryDeleteOneResponseVariants,
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                                >    
                            ,
                        }
                    } else if status_code == http :: StatusCode :: BAD_REQUEST
                    {
                        match serde_json :: from_str :: <
                        TryDeleteOneResponseVariantsTvfrr400BadRequest >
                        (& response_text)
                        {
                            Ok(value) => 
                                Ok(TryDeleteOneResponseVariants :: from(value))
                                                                as 
                                Result<
                                    TryDeleteOneResponseVariants,
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                                >
                                , 
                            Err(e) => 
                                Err(crate :: common :: api_request_unexpected_error ::ApiRequestUnexpectedError :: DeserializeBody{ serde : e, status_code, headers, response_text })
                                                                as 
                                Result<
                                    TryDeleteOneResponseVariants,
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                                >
                                ,
                        }
                    } else if status_code == http :: StatusCode :: NOT_FOUND
                    {
                        match serde_json :: from_str :: <
                        TryDeleteOneResponseVariantsTvfrr404NotFound >
                        (& response_text)
                        {
                            Ok(value) =>
                            Ok(TryDeleteOneResponseVariants :: from(value))
                                                            as 
                                Result<
                                    TryDeleteOneResponseVariants,
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                                >
                            , Err(e) =>
                            Err(crate :: common :: api_request_unexpected_error ::
                            ApiRequestUnexpectedError :: DeserializeBody
                            { serde : e, status_code, headers, response_text })
                                                            as 
                                Result<
                                    TryDeleteOneResponseVariants,
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                                >
                            ,
                        }
                    } else
                    {
                        Err(crate :: common :: api_request_unexpected_error ::
                        ApiRequestUnexpectedError :: StatusCode
                        {
                            status_code, headers, response_text_result : crate :: common
                            :: api_request_unexpected_error :: ResponseTextResult ::
                            ResponseText(response_text)
                        })
                                                        as 
                                Result<
                                    TryDeleteOneResponseVariants,
                                    crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                                >
                    }
                }, 
                Err(e) => {
                    Err(crate::common :: api_request_unexpected_error ::ApiRequestUnexpectedError :: FailedToGetResponseText{ reqwest : e, status_code, headers, })
                    //
                    as 
                    Result<
                        TryDeleteOneResponseVariants,
                        crate::common::api_request_unexpected_error::ApiRequestUnexpectedError,
                    >
                    //
                },
            }
            //
        };
        match match_value {
            Ok(variants) => match crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::try_from(variants)
            {
                Ok(value) => Ok(value), 
                Err(e) => Err(TryDeleteOneRequestError :: ExpectedType
                {
                    expected_type : e, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line : 882, column : 17,
                    })),
                }),
            }, 
            Err(e) => match e {
                crate::common::api_request_unexpected_error::ApiRequestUnexpectedError::StatusCode{ status_code, headers, response_text_result, } => Err(TryDeleteOneRequestError :: UnexpectedStatusCode
                {
                    status_code, headers, response_text_result, code_occurence :
                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line : 898, column : 13,
                    }))
                }), 
                
                crate :: common :: api_request_unexpected_error ::
                ApiRequestUnexpectedError :: FailedToGetResponseText
                { reqwest, status_code, headers } =>
                Err(TryDeleteOneRequestError :: FailedToGetResponseText
                {
                    reqwest, status_code, headers, code_occurence :
                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line : 904, column : 13,
                    }))
                }), 
                
                crate :: common :: api_request_unexpected_error ::ApiRequestUnexpectedError :: DeserializeBody
                { serde, status_code, headers, response_text, } =>
                Err(TryDeleteOneRequestError :: DeserializeResponse
                {
                    serde, status_code, headers, response_text, code_occurence :
                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line : 910, column : 13,
                    }))
                }),
            },
        }
            //
        }, 
        Err(e) => Err(TryDeleteOneRequestError :: Reqwest {
            reqwest : e, 
            code_occurence : 
            error_occurence_lib ::
            code_occurence :: CodeOccurence ::new(
                crate :: global_variables :: compile_time :: project_git_info:: PROJECT_GIT_INFO.commit.to_string(), 
                file! ().to_string(),
                line! (), 
                column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                    line : 916, column : 13,
                })),
        }),
    }
}
pub enum TryDeleteOneStatusCodesChecker {
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
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserializeTvfrr400BadRequest,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryDeleteOneResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryDeleteOneResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            } TryDeleteOneResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants ::
            DeleteOnePayloadTryFromDeleteOnePayloadWithSerializeDeserialize
            {
                delete_one_payload_try_from_delete_one_payload_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }, TryDeleteOneResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: OK ; res
            }
        }
    }
}
