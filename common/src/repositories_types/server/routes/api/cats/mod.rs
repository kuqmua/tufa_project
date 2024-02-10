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

////////////////////
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElement {
    pub name: std::string::String,
    pub color: std::string::String,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayload(pub std::vec::Vec<CreateManyPayloadElement>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElementWithSerializeDeserialize {
    pub name: std::string::String,
    pub color: std::string::String,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct CreateManyPayloadWithSerializeDeserialize(
    std::vec::Vec<CreateManyPayloadElementWithSerializeDeserialize>,
);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum CreateManyPayloadElementTryFromCreateManyPayloadElementWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<CreateManyPayloadElementWithSerializeDeserialize>
    for CreateManyPayloadElement
{
    type Error =
        CreateManyPayloadElementTryFromCreateManyPayloadElementWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: CreateManyPayloadElementWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let name = value.name;
        let color = value.color;
        Ok(Self { name, color })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<CreateManyPayloadWithSerializeDeserialize> for CreateManyPayload {
    type Error = CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: CreateManyPayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let mut elements = std::vec::Vec::with_capacity(value.0.len());
        for element in value.0 {
            match CreateManyPayloadElement::try_from(element) {
                Ok(value) => {
                    elements.push(value);
                }
                Err(e) => todo!(),
            }
        }
        Ok(Self(elements))
    }
}
impl std::convert::From<CreateManyPayloadElement>
    for CreateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: CreateManyPayloadElement) -> Self {
        let name = value.name;
        let color = value.color;
        Self { name, color }
    }
}
impl std::convert::From<CreateManyPayload> for CreateManyPayloadWithSerializeDeserialize {
    fn from(value: CreateManyPayload) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| CreateManyPayloadElementWithSerializeDeserialize::from(element))
                .collect::<std::vec::Vec<CreateManyPayloadElementWithSerializeDeserialize>>(),
        )
    }
}
#[derive(Debug)]
pub struct CreateManyParameters {
    pub payload: CreateManyPayload,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed {
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(
        crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
    ),
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryCreateMany {
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
    BindQuery {
        #[eo_error_occurence]
        bind_query: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        create_many_payload_try_from_create_many_payload_with_serialize_deserialize:
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamed,
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
pub enum TryCreateManyResponseVariants {
    Desirable(std :: vec :: Vec :: < crate :: server :: postgres ::
    uuid_wrapper :: PossibleUuidWrapper >), Configuration
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
    }, BindQuery
    {
        bind_query :
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
    {
        create_many_payload_try_from_create_many_payload_with_serialize_deserialize
        :
        CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
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
impl std::convert::From<TryCreateMany> for TryCreateManyResponseVariants {
    fn from(value: TryCreateMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Io { io, code_occurence }
            => Self :: Io { io, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryCreateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryCreateManyWithSerializeDeserialize ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            }, TryCreateManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryCreateManyWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryCreateManyResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryCreateManyResponseVariants) -> Self {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Configuration { configuration : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Database { database : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            TypeNotFound { type_not_found : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            WorkerCrashed { worker_crashed : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Migrate { migrate : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            JsonDataError { json_data_error : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => axum :: http
            :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr201Created {
    Desirable(std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>),
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr201Created>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr201Created) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr408RequestTimeout>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr400BadRequest {
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
    }, CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
    {
        create_many_payload_try_from_create_many_payload_with_serialize_deserialize
        :
        CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
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
impl std::convert::From<TryCreateManyResponseVariantsTvfrr400BadRequest>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryCreateManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            }, TryCreateManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr404NotFound>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr404NotFound::RowNotFound {
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
pub enum TryCreateManyResponseVariantsTvfrr500InternalServerError {
    Configuration
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
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize<>, len : usize<>, code_occurence :
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
    }, BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, BindQuery
    {
        bind_query :
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std::string::String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr500InternalServerError>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
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
impl TryFrom<TryCreateManyResponseVariants>
    for std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>
{
    type Error = TryCreateManyWithSerializeDeserialize;
    fn try_from(value: TryCreateManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(i) => Ok(i),
            TryCreateManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryCreateManyResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryCreateManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryCreateManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryCreateManyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryCreateManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryCreateManyResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryCreateManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryCreateManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryCreateManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryCreateManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryCreateManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryCreateManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryCreateManyResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryCreateManyResponseVariants :: UnexpectedCase
            { unexpected_case, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryCreateManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryCreateManyResponseVariants ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            }), TryCreateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryCreateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryCreateManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateManyWithSerializeDeserialize,
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
async fn tvfrr_extraction_logic_try_create_many<'a>(
    future: impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>,
    TryCreateManyRequestError,
> {
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return
            Err(TryCreateManyRequestError :: Reqwest
            {
                reqwest : e, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(crate :: global_variables :: compile_time ::
                project_git_info :: PROJECT_GIT_INFO.commit.to_string(), file!
                ().to_string(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                    line : 880, column : 13,
                })),
            }) ;
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return
            Err(TryCreateManyRequestError :: FailedToGetResponseText
            {
                reqwest : e, status_code, headers, code_occurence :
                error_occurence_lib :: code_occurence :: CodeOccurence ::
                new(crate :: global_variables :: compile_time ::
                project_git_info :: PROJECT_GIT_INFO.commit.to_string(), file!
                ().to_string(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                    line : 886, column : 13,
                }))
            }) ;
        }
    };
    let variants = if status_code == http::StatusCode::CREATED {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr201Created>(&response_text) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                return
                Err(TryCreateManyRequestError :: DeserializeResponse
                {
                    serde : e, status_code, headers, response_text,
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line : 793, column : 17,
                    }))
                }) ;
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr400BadRequest>(
            &response_text,
        ) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                return
                Err(TryCreateManyRequestError :: DeserializeResponse
                {
                    serde : e, status_code, headers, response_text,
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line : 825, column : 17,
                    }))
                }) ;
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr404NotFound>(&response_text)
        {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                return
                Err(TryCreateManyRequestError :: DeserializeResponse
                {
                    serde : e, status_code, headers, response_text,
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line : 825, column : 17,
                    }))
                }) ;
            }
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr500InternalServerError>(
            &response_text,
        ) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                return
                Err(TryCreateManyRequestError :: DeserializeResponse
                {
                    serde : e, status_code, headers, response_text,
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line : 825, column : 17,
                    }))
                }) ;
            }
        }
    } else {
        return
        Err(TryCreateManyRequestError :: UnexpectedStatusCode
        {
            status_code, headers, response_text_result : crate :: common ::
            api_request_unexpected_error :: ResponseTextResult ::
            ResponseText(response_text), code_occurence : error_occurence_lib
            :: code_occurence :: CodeOccurence ::
            new(crate :: global_variables :: compile_time :: project_git_info
            :: PROJECT_GIT_INFO.commit.to_string(), file! ().to_string(),
            line! (), column! (),
            Some(error_occurence_lib :: code_occurence :: MacroOccurence
            {
                file : std :: string :: String ::
                from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                line : 819, column : 17,
            }))
        }) ;
    };
    match std :: vec :: Vec :: < crate :: server :: postgres ::
    uuid_wrapper :: PossibleUuidWrapper > :: try_from(variants)
    {
        Ok(value) => Ok(value), Err(e) =>
        Err(TryCreateManyRequestError :: ExpectedType
        {
            expected_type : e, code_occurence : error_occurence_lib ::
            code_occurence :: CodeOccurence ::
            new(crate :: global_variables :: compile_time :: project_git_info
            :: PROJECT_GIT_INFO.commit.to_string(), file! ().to_string(),
            line! (), column! (),
            Some(error_occurence_lib :: code_occurence :: MacroOccurence
            {
                file : std :: string :: String ::
                from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                line : 892, column : 13,
            }))
        }),
    }
}
pub enum TryCreateManyStatusCodesChecker {
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
    BindQueryTvfrr500InternalServerError,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeTvfrr400BadRequest,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryCreateManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryCreateManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            } TryCreateManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
    RequestError {
        #[eo_error_occurence]
        request_error: TryCreateManyRequestError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_vec_error_occurence]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client:
            std::vec::Vec<
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed,
            >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many<'a>(
    server_location: &str,
    parameters: CreateManyParameters,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
    TryCreateManyErrorNamed,
> {
    let payload = match serde_json::to_string(&CreateManyPayloadWithSerializeDeserialize::from(parameters.payload)) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                        .commit
                        .to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1720,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/create_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(
            postgresql_crud::COMMIT,
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            let request_error = TryCreateManyRequestError::Reqwest {
                reqwest: e, 
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(), 
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line: 880,
                        column: 13,
                    })
                ),
            };
            return Err(TryCreateManyErrorNamed::RequestError {
                request_error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                        .commit
                        .to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 1698,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            let request_error = TryCreateManyRequestError::FailedToGetResponseText {
                reqwest: e, 
                status_code, 
                headers, 
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(), 
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line: 886,
                        column: 13,
                    })
                )
            };
            return Err(TryCreateManyErrorNamed::RequestError {
                request_error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                        .commit
                        .to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 1698,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants = if status_code == http::StatusCode::CREATED {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr201Created>(&response_text) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                let request_error = TryCreateManyRequestError::DeserializeResponse {
                    serde: e, 
                    status_code, 
                    headers, 
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate :: global_variables :: compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                            line: 793, 
                            column: 17,
                        })
                    )
                };
                return Err(TryCreateManyErrorNamed::RequestError {
                    request_error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                            .commit
                            .to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 1698,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::BAD_REQUEST {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr400BadRequest>(&response_text) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                let request_error = TryCreateManyRequestError::DeserializeResponse {
                    serde: e, 
                    status_code, 
                    headers, 
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                        file!().to_string(),
                        line!(), 
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                            line: 825, 
                            column: 17,
                        })
                    )
                };
                return Err(TryCreateManyErrorNamed::RequestError {
                    request_error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                            .commit
                            .to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 1698,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::NOT_FOUND {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr404NotFound>(&response_text) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                let request_error = TryCreateManyRequestError::DeserializeResponse {
                    serde: e, 
                    status_code, 
                    headers, 
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                            line: 825, 
                            column: 17,
                        })
                    )
                };
                return Err(TryCreateManyErrorNamed::RequestError {
                    request_error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                            .commit
                            .to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 1698,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else if status_code == http::StatusCode::INTERNAL_SERVER_ERROR {
        match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr500InternalServerError>(&response_text) {
            Ok(value) => TryCreateManyResponseVariants::from(value),
            Err(e) => {
                let request_error = TryCreateManyRequestError::DeserializeResponse {
                    serde: e, 
                    status_code, 
                    headers, 
                    response_text,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(),
                        file!().to_string(), 
                        line!(), 
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                            line: 825, 
                            column: 17,
                        })
                    )
                };
                return Err(TryCreateManyErrorNamed::RequestError {
                    request_error,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                            .commit
                            .to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 1698,
                            column: 13,
                        }),
                    ),
                });
            }
        }
    } else {
        let request_error = TryCreateManyRequestError::UnexpectedStatusCode {
            status_code, 
            headers, 
            response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(response_text), 
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(), file! ().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                    line: 819,
                    column: 17,
                })
            )
        };
        return Err(TryCreateManyErrorNamed::RequestError {
            request_error,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                    .commit
                    .to_string(),
                file!().to_string(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line: 1698,
                    column: 13,
                }),
            ),
        });
    };
    match std::vec::Vec::<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>::try_from(variants) {
        Ok(value) => {
            let mut vec_values = std::vec::Vec::with_capacity(value.len());
            let mut vec_errors = std::vec::Vec::with_capacity(value.len());
            for element in value {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(element) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        vec_errors.push(OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed
                        ::
                        OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(e))
                        ;
                    }
                }
            }
            if let false = vec_errors.is_empty() {
                return
                Err(TryCreateManyErrorNamed ::
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
                {
                    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client
                    : vec_errors, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 2099, column : 13,
                    }))
                }) ;
            }
            Ok(vec_values)
        }, 
        Err(e) => {
            let request_error = TryCreateManyRequestError::ExpectedType {
                expected_type: e, 
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit.to_string(), 
                    file!().to_string(),
                    line!(), 
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/type_variants_from_request_response_generator.rs"),
                        line: 892, 
                        column: 13,
                    })
                )
            };
            return Err(TryCreateManyErrorNamed::RequestError {
                request_error,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                        .commit
                        .to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 1698,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(post, path = "/dogs/create_many", operation_id = "/dogs/create_many", tag
= "dogs",
request_body(content = CreateManyPayload, description =
"dogs create_many payload", content_type = "application/json"),
responses((status = 201, description = "created", body =
TryCreateManyResponseVariantsTvfrr201Created, content_type =
"application/json"),
(status = 500, description = "internal server error", body =
TryCreateManyResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryCreateManyResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryCreateManyResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryCreateManyResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn create_many(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<CreateManyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = CreateManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                CreateManyPayloadWithSerializeDeserialize,
                TryCreateManyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => match CreateManyPayload::try_from(value) {
                    Ok(value) => value,
                    Err(e) => {
                        let error = TryCreateMany ::
                    CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
                    {
                        create_many_payload_try_from_create_many_payload_with_serialize_deserialize
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(crate :: global_variables :: compile_time ::
                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                        file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2872, column : 17,
                        })),
                    } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(
                            &error,
                            app_state.as_ref(),
                        );
                        return TryCreateManyResponseVariants::from(error);
                    }
                },
                Err(err) => {
                    return err;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let query_string = {
            "insert into dogs (name, color) select name, color from unnest($1, $2) as a(name, color) returning id"
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let current_vec_len = parameters.payload.0.len();
            let (name_vec, color_vec) = parameters.payload.0.into_iter().fold(
                (
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                ),
                |mut acc, element| {
                    acc.0.push(element.name);
                    acc.1.push(element.color);
                    acc
                },
            );
            query = query.bind(name_vec);
            query = query.bind(color_vec);
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let error = TryCreateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&error, app_state.as_ref());
                return TryCreateManyResponseVariants::from(error);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let error = TryCreateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&error, app_state.as_ref());
                return TryCreateManyResponseVariants::from(error);
            }
        };
        let mut rows = binded_query.fetch(pg_connection.as_mut());
        let mut vec_values = std::vec::Vec::new();
        while let Some(row) = {
            match {
                use futures::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => value,
                Err(e) => {
                    let error = TryCreateMany::from(e);
                    error_occurence_lib::error_log::ErrorLog::error_log(&error, app_state.as_ref());
                    return TryCreateManyResponseVariants::from(error);
                }
            }
        } {
            match {
                use sqlx::Row;
                row.try_get::<sqlx::types::Uuid, &str>("id")
            } {
                Ok(value) => {
                    vec_values.push(
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value),
                    );
                }
                Err(e) => {
                    let error = TryCreateMany ::
                    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                    {
                        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(crate :: global_variables :: compile_time ::
                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                        file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2078, column : 13,
                        })),
                    } ;
                    error_occurence_lib::error_log::ErrorLog::error_log(&error, app_state.as_ref());
                    return TryCreateManyResponseVariants::from(error);
                }
            }
        }
        TryCreateManyResponseVariants::Desirable(vec_values)
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryCreateMany
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
