#[derive(Debug)] pub struct UpdateManyPayloadElement
{
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key :
    postgresql_crud::StdPrimitiveI64, pub
    std_primitive_bool_as_postgresql_bool : std :: option :: Option < Field <
    postgresql_crud::StdOptionOptionStdPrimitiveBool >> , pub
    std_primitive_i16_as_postgresql_small_int : std :: option :: Option <
    Field < postgresql_crud::StdOptionOptionStdPrimitiveI16 >> , pub
    std_primitive_i32_as_postgresql_int : std :: option :: Option < Field <
    postgresql_crud::StdOptionOptionStdPrimitiveI32 >> , pub
    sqlx_types_uuid_uuid_as_postgresql_uuid : std :: option :: Option < Field
    < postgresql_crud::StdOptionOptionSqlxTypesUuidUuid >>
} #[derive(Debug)] pub struct
UpdateManyPayload(pub std :: vec :: Vec < UpdateManyPayloadElement >);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElementWithSerializeDeserialize
{
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key :
    postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
    std_primitive_bool_as_postgresql_bool : std :: option :: Option <
    FieldWithSerializeDeserialize <
    postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
    >> , std_primitive_i16_as_postgresql_small_int : std :: option :: Option <
    FieldWithSerializeDeserialize <
    postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize >>
    , std_primitive_i32_as_postgresql_int : std :: option :: Option <
    FieldWithSerializeDeserialize <
    postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize >>
    , sqlx_types_uuid_uuid_as_postgresql_uuid : std :: option :: Option <
    FieldWithSerializeDeserialize <
    postgresql_crud::StdOptionOptionSqlxTypesUuidUuidWithSerializeDeserialize
    >>
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct
UpdateManyPayloadWithSerializeDeserialize(pub std :: vec :: Vec <
UpdateManyPayloadElementWithSerializeDeserialize >);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum
UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed
{
    SqlxTypesUuidUuidAsPostgresqlUuid
    {
        #[eo_error_occurence] sqlx_types_uuid_uuid_as_postgresql_uuid :
        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    },
} impl std :: convert :: TryFrom <
UpdateManyPayloadElementWithSerializeDeserialize > for
UpdateManyPayloadElement
{
    type Error =
    UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed;
    fn try_from(value : UpdateManyPayloadElementWithSerializeDeserialize) ->
    Result < Self, Self :: Error >
    {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
        postgresql_crud::StdPrimitiveI64 ::
        from(value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key);
        let std_primitive_bool_as_postgresql_bool = match
        value.std_primitive_bool_as_postgresql_bool
        {
            Some(value) =>
            Some(Field
            {
                value : postgresql_crud::StdOptionOptionStdPrimitiveBool ::
                from(value.value,)
            }), None => None
        }; let std_primitive_i16_as_postgresql_small_int = match
        value.std_primitive_i16_as_postgresql_small_int
        {
            Some(value) =>
            Some(Field
            {
                value : postgresql_crud::StdOptionOptionStdPrimitiveI16 ::
                from(value.value,)
            }), None => None
        }; let std_primitive_i32_as_postgresql_int = match
        value.std_primitive_i32_as_postgresql_int
        {
            Some(value) =>
            Some(Field
            {
                value : postgresql_crud::StdOptionOptionStdPrimitiveI32 ::
                from(value.value,)
            }), None => None
        }; let sqlx_types_uuid_uuid_as_postgresql_uuid = match
        value.sqlx_types_uuid_uuid_as_postgresql_uuid
        {
            Some(value) =>
            Some(Field
            {
                value : match
                postgresql_crud::StdOptionOptionSqlxTypesUuidUuid ::
                try_from(value)
                {
                    Ok(value) => value, Err(error) =>
                    {
                        return
                        Err(Self :: Error :: SqlxTypesUuidUuidAsPostgresqlUuid
                        {
                            sqlx_types_uuid_uuid_as_postgresql_uuid : error,
                            code_occurence : error_occurence_lib :: code_occurence ::
                            CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 2176, column : 13,
                            }))
                        });
                    }
                }
            }), None => None
        };
        Ok(Self
        {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            sqlx_types_uuid_uuid_as_postgresql_uuid
        })
    }
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum
UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed
{
    SqlxTypesUuidUuidAsPostgresqlUuid
    {
        #[eo_error_occurence] std_primitive_i_64 :
        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    },
} impl std :: convert :: From <
UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed
> for
UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed
{
    fn
    from(value :
    UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed)
    -> Self
    {
        match value
        {
            UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed
            :: SqlxTypesUuidUuidAsPostgresqlUuid
            { std_primitive_i_64, code_occurence, } => Self ::
            SqlxTypesUuidUuidAsPostgresqlUuid
            { std_primitive_i_64, code_occurence, },
        }
    }
} impl std :: convert :: TryFrom < UpdateManyPayloadWithSerializeDeserialize >
for UpdateManyPayload
{
    type Error =
    UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value : UpdateManyPayloadWithSerializeDeserialize) -> Result <
    Self, Self :: Error >
    {
        match
        value.0.into_iter().map(| element | UpdateManyPayloadElement ::
        try_from(element)).collect :: < Result < std :: vec :: Vec <
        UpdateManyPayloadElement > ,
        UpdateManyPayloadElementTryFromUpdateManyPayloadElementWithSerializeDeserializeErrorNamed
        >> ()
        {
            Ok(value) => Ok(Self(value)), Err(error) =>
            Err(Self :: Error :: from(error)),
        }
    }
} impl std :: convert :: From < UpdateManyPayloadElement > for
UpdateManyPayloadElementWithSerializeDeserialize
{
    fn from(value : UpdateManyPayloadElement) -> Self
    {
        let std_primitive_i64_as_postgresql_big_serial_not_null_primary_key =
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize ::
        from(value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key);
        let std_primitive_bool_as_postgresql_bool = match
        value.std_primitive_bool_as_postgresql_bool
        {
            Some(value) =>
            Some(FieldWithSerializeDeserialize
            {
                value :
                postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
                :: from(value.value,)
            }), None => None
        }; let std_primitive_i16_as_postgresql_small_int = match
        value.std_primitive_i16_as_postgresql_small_int
        {
            Some(value) =>
            Some(FieldWithSerializeDeserialize
            {
                value :
                postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize
                :: from(value.value,)
            }), None => None
        }; let std_primitive_i32_as_postgresql_int = match
        value.std_primitive_i32_as_postgresql_int
        {
            Some(value) =>
            Some(FieldWithSerializeDeserialize
            {
                value :
                postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize
                :: from(value.value,)
            }), None => None
        }; let sqlx_types_uuid_uuid_as_postgresql_uuid = match
        value.sqlx_types_uuid_uuid_as_postgresql_uuid
        {
            Some(value) =>
            Some(FieldWithSerializeDeserialize
            {
                value :
                postgresql_crud::StdOptionOptionSqlxTypesUuidUuidWithSerializeDeserialize
                :: from(value.value,)
            }), None => None
        }; Self
        {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
            sqlx_types_uuid_uuid_as_postgresql_uuid
        }
    }
} impl std :: convert :: From < UpdateManyPayload > for
UpdateManyPayloadWithSerializeDeserialize
{
    fn from(value : UpdateManyPayload) -> Self
    {
        Self(value.0.into_iter().map(| element |
        UpdateManyPayloadElementWithSerializeDeserialize ::
        from(element)).collect())
    }
} #[derive(Debug)] pub struct UpdateManyParameters
{ pub payload : UpdateManyPayload, }
#[derive(Debug, serde :: Serialize, serde :: Deserialize)] pub enum
TryUpdateManyRouteLogicResponseVariants
{
    Desirable(std :: vec :: Vec :: <
    postgresql_crud::StdPrimitiveI64WithSerializeDeserialize >), CheckBodySize
    {
        check_body_size : route_validators :: check_body_size ::
        CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Postgresql
    {
        postgresql : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, Json
    {
        json : std :: string :: String, code_occurence : error_occurence_lib
        :: code_occurence :: CodeOccurence,
    }, CheckCommit
    {
        check_commit : route_validators :: check_commit ::
        CheckCommitErrorNamedWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, QueryAndRollbackFailed
    {
        query : std :: string :: String, rollback : std :: string :: String,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, PrimaryKeyFromRowAndFailedRollback
    {
        primary_key_from_row : std :: string :: String, rollback : std ::
        string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, NonExistingPrimaryKeys
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String > , code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        non_existing_primary_keys : std :: vec :: Vec < std :: string ::
        String > , rollback : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, CommitFailed
    {
        commit_failed : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, NotUniquePrimaryKey
    {
        not_unique_primary_key : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, BindQuery
    {
        bind_query : postgresql_crud ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NoPayloadFields
    {
        no_payload_fields : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : std :: string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
    {
        update_many_payload_try_from_update_many_payload_with_serialize_deserialize
        :
        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }
} impl std :: convert :: From < TryUpdateManyRouteLogicErrorNamed > for
TryUpdateManyRouteLogicResponseVariants
{
    fn from(value : TryUpdateManyRouteLogicErrorNamed) -> Self
    {
        match value.into_serialize_deserialize_version()
        {
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            QueryAndRollbackFailed { query, rollback, code_occurence } => Self
            :: QueryAndRollbackFailed { query, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback, code_occurence } => Self ::
            PrimaryKeyFromRowAndFailedRollback
            { primary_key_from_row, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence } => Self ::
            NonExistingPrimaryKeys
            { non_existing_primary_keys, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback, code_occurence } => Self ::
            NonExistingPrimaryKeysAndFailedRollback
            { non_existing_primary_keys, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CommitFailed { commit_failed, code_occurence } => Self ::
            CommitFailed { commit_failed, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniquePrimaryKey { not_unique_primary_key, code_occurence } =>
            Self :: NotUniquePrimaryKey
            { not_unique_primary_key, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NoPayloadFields { no_payload_fields, code_occurence } => Self ::
            NoPayloadFields { no_payload_fields, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
            {
                update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
                code_occurence
            }
        }
    }
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence,)]
pub enum TryUpdateManyRouteLogicErrorNamed
{
    CheckBodySize
    {
        #[eo_error_occurence] check_body_size : route_validators ::
        check_body_size :: CheckBodySizeErrorNamed, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence
    }, Postgresql
    {
        #[eo_to_std_string_string] postgresql : sqlx :: Error, code_occurence
        : error_occurence_lib :: code_occurence :: CodeOccurence
    }, Json
    {
        #[eo_to_std_string_string] json : axum :: extract :: rejection ::
        JsonRejection, code_occurence : error_occurence_lib :: code_occurence
        :: CodeOccurence
    }, CheckCommit
    {
        #[eo_error_occurence] check_commit : route_validators :: check_commit
        :: CheckCommitErrorNamed, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence
    }, QueryAndRollbackFailed
    {
        #[eo_to_std_string_string] query : sqlx :: Error,
        #[eo_to_std_string_string] rollback : sqlx :: Error, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence
    }, PrimaryKeyFromRowAndFailedRollback
    {
        #[eo_to_std_string_string] primary_key_from_row : sqlx :: Error,
        #[eo_to_std_string_string] rollback : sqlx :: Error, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeys
    {
        #[eo_vec_to_std_string_string] non_existing_primary_keys : std :: vec
        :: Vec < postgresql_crud :: StdPrimitiveI64 > , code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence
    }, NonExistingPrimaryKeysAndFailedRollback
    {
        #[eo_vec_to_std_string_string] non_existing_primary_keys : std :: vec
        :: Vec < postgresql_crud :: StdPrimitiveI64 > ,
        #[eo_to_std_string_string] rollback : sqlx :: Error, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence
    }, CommitFailed
    {
        #[eo_to_std_string_string] commit_failed : sqlx :: Error,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }, NotUniquePrimaryKey
    {
        #[eo_to_std_string_string] not_unique_primary_key : postgresql_crud ::
        StdPrimitiveI64, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence
    }, BindQuery
    {
        #[eo_error_occurence] bind_query : postgresql_crud ::
        TryGenerateBindIncrementsErrorNamed, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence
    }, NoPayloadFields
    {
        #[eo_to_std_string_string] no_payload_fields : postgresql_crud ::
        StdPrimitiveI64, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : sqlx :: Error, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence
    }, UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
    {
        #[eo_error_occurence]
        update_many_payload_try_from_update_many_payload_with_serialize_deserialize
        :
        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }
} pub async fn
try_update_many_route_logic(app_state : axum :: extract :: State < crate ::
repositories_types :: server :: routes :: app_state ::
DynArcCombinationOfAppStateLogicTraits, > , request : axum :: extract ::
Request,) -> axum :: response :: Response
{
    let (parts, body) = request.into_parts(); let headers = parts.headers; let
    body_bytes = match route_validators :: check_body_size ::
    check_body_size(body, *
    app_state.get_maximum_size_of_http_body_in_bytes()).await
    {
        Ok(value) => value, Err(error) =>
        {
            let status_code = http_logic :: GetAxumHttpStatusCode ::
            get_axum_http_status_code(& error); let error =
            TryUpdateManyRouteLogicErrorNamed :: CheckBodySize
            {
                check_body_size : error, code_occurence : error_occurence_lib
                :: code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1719, column : 13,
                })),
            }; eprintln! ("{error}"); let mut response = axum :: response ::
            IntoResponse ::
            into_response(axum ::
            Json(TryUpdateManyRouteLogicResponseVariants :: from(error))); *
            response.status_mut() = status_code; return response;
        }
    }; if let Err(error) =
    route_validators::check_commit::check_commit(*app_state.get_enable_api_git_commit_check(),
    &headers,)
    {
        let status_code =
        postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
        let error = TryCreateManyRouteLogicErrorNamed::CheckCommit
        {
            check_commit: error, code_occurence:
            error_occurence_lib::code_occurence!(),
        }; eprintln!("{error}"); let mut response =
        axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error),));
        *response.status_mut() = status_code; return response;
    } let parameters = UpdateManyParameters
    {
        payload : match axum :: Json :: <
        UpdateManyPayloadWithSerializeDeserialize > ::
        from_bytes(& body_bytes,)
        {
            Ok(axum :: Json(value)) =>
            {
                let value = match UpdateManyPayload :: try_from(value)
                {
                    Ok(value) => value, Err(error) =>
                    {
                        let error = TryUpdateManyRouteLogicErrorNamed ::
                        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
                        {
                            update_many_payload_try_from_update_many_payload_with_serialize_deserialize
                            : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 7532, column : 13,
                            })),
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryUpdateManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: BAD_REQUEST; return response;
                    }
                };
                {
                    let mut acc = std :: vec :: Vec :: new(); for element in &
                    value.0
                    {
                        if !
                        acc.contains(&
                        element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key)
                        {
                            acc.push(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone());
                        } else
                        {
                            let error = TryUpdateManyRouteLogicErrorNamed ::
                            NotUniquePrimaryKey
                            {
                                not_unique_primary_key :
                                element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone(),
                                code_occurence : error_occurence_lib :: code_occurence ::
                                CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1900, column : 21,
                                })),
                            }; eprintln! ("{error}"); let mut response = axum ::
                            response :: IntoResponse ::
                            into_response(axum ::
                            Json(TryUpdateManyRouteLogicResponseVariants ::
                            from(error))); * response.status_mut() = axum :: http ::
                            StatusCode :: BAD_REQUEST; return response;
                        }
                    }
                } for element in & value.0
                {
                    if let (None, None, None, None) =
                    (& element.std_primitive_bool_as_postgresql_bool, &
                    element.std_primitive_i16_as_postgresql_small_int, &
                    element.std_primitive_i32_as_postgresql_int, &
                    element.sqlx_types_uuid_uuid_as_postgresql_uuid)
                    {
                        let
                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                        =
                        element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;
                        let error = TryUpdateManyRouteLogicErrorNamed ::
                        NoPayloadFields
                        {
                            no_payload_fields :
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                            code_occurence : error_occurence_lib :: code_occurence ::
                            CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1957, column : 21,
                            }))
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryUpdateManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: BAD_REQUEST; return response;
                    }
                } value
            }, Err(error) =>
            {
                let error = TryUpdateManyRouteLogicErrorNamed :: Json
                {
                    json : error, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 1808, column : 21,
                    })),
                }; eprintln! ("{error}"); let mut response = axum :: response
                :: IntoResponse ::
                into_response(axum ::
                Json(TryUpdateManyRouteLogicResponseVariants :: from(error)));
                * response.status_mut() = axum :: http :: StatusCode ::
                BAD_REQUEST; return response;
            }
        },
    }; println! ("{:#?}", parameters); let expected_updated_primary_keys =
    parameters.payload.0.iter().map(| element |
    element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone()).collect
    :: < std :: vec :: Vec < postgresql_crud::StdPrimitiveI64 > > (); let
    query_string =
    {
        let mut query = std :: string :: String :: from("update dogs set ");
        let mut increment : u64 = 0;
        {
            let mut is_std_primitive_bool_as_postgresql_bool_update_exist =
            false; for element in & parameters.payload.0
            {
                if element.std_primitive_bool_as_postgresql_bool.is_some()
                {
                    is_std_primitive_bool_as_postgresql_bool_update_exist =
                    true; break;
                }
            } if is_std_primitive_bool_as_postgresql_bool_update_exist
            {
                let mut acc = std :: string :: String ::
                from("std_primitive_bool_as_postgresql_bool = case "); for
                element in & parameters.payload.0
                {
                    if let Some(value) = &
                    element.std_primitive_bool_as_postgresql_bool
                    {
                        acc.push_str(& format!
                        ("when std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {} then {} ",
                        match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(&
                        element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                        & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1847, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }, match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(& value.value, & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1847, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }));
                    }
                }
                query.push_str(& format!
                ("{}{}", acc,
                "else std_primitive_bool_as_postgresql_bool end,"));
            }
        }
        {
            let mut is_std_primitive_i16_as_postgresql_small_int_update_exist
            = false; for element in & parameters.payload.0
            {
                if element.std_primitive_i16_as_postgresql_small_int.is_some()
                {
                    is_std_primitive_i16_as_postgresql_small_int_update_exist =
                    true; break;
                }
            } if is_std_primitive_i16_as_postgresql_small_int_update_exist
            {
                let mut acc = std :: string :: String ::
                from("std_primitive_i16_as_postgresql_small_int = case "); for
                element in & parameters.payload.0
                {
                    if let Some(value) = &
                    element.std_primitive_i16_as_postgresql_small_int
                    {
                        acc.push_str(& format!
                        ("when std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {} then {} ",
                        match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(&
                        element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                        & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1847, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }, match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(& value.value, & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1847, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }));
                    }
                }
                query.push_str(& format!
                ("{}{}", acc,
                "else std_primitive_i16_as_postgresql_small_int end,"));
            }
        }
        {
            let mut is_std_primitive_i32_as_postgresql_int_update_exist =
            false; for element in & parameters.payload.0
            {
                if element.std_primitive_i32_as_postgresql_int.is_some()
                {
                    is_std_primitive_i32_as_postgresql_int_update_exist = true;
                    break;
                }
            } if is_std_primitive_i32_as_postgresql_int_update_exist
            {
                let mut acc = std :: string :: String ::
                from("std_primitive_i32_as_postgresql_int = case "); for
                element in & parameters.payload.0
                {
                    if let Some(value) = &
                    element.std_primitive_i32_as_postgresql_int
                    {
                        acc.push_str(& format!
                        ("when std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {} then {} ",
                        match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(&
                        element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                        & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1847, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }, match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(& value.value, & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1847, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }));
                    }
                }
                query.push_str(& format!
                ("{}{}", acc,
                "else std_primitive_i32_as_postgresql_int end,"));
            }
        }
        {
            let mut is_sqlx_types_uuid_uuid_as_postgresql_uuid_update_exist =
            false; for element in & parameters.payload.0
            {
                if element.sqlx_types_uuid_uuid_as_postgresql_uuid.is_some()
                {
                    is_sqlx_types_uuid_uuid_as_postgresql_uuid_update_exist =
                    true; break;
                }
            } if is_sqlx_types_uuid_uuid_as_postgresql_uuid_update_exist
            {
                let mut acc = std :: string :: String ::
                from("sqlx_types_uuid_uuid_as_postgresql_uuid = case "); for
                element in & parameters.payload.0
                {
                    if let Some(value) = &
                    element.sqlx_types_uuid_uuid_as_postgresql_uuid
                    {
                        acc.push_str(& format!
                        ("when std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = {} then {} ",
                        match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(&
                        element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                        & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1847, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }, match postgresql_crud :: BindQuery ::
                        try_generate_bind_increments(& value.value, & mut increment)
                        {
                            Ok(value) => value, Err(error) =>
                            {
                                let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                                {
                                    bind_query : error, code_occurence : error_occurence_lib ::
                                    code_occurence :: CodeOccurence ::
                                    new(file! ().to_owned(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1847, column : 21,
                                    }))
                                }; eprintln! ("{error}"); let mut response = axum ::
                                response :: IntoResponse ::
                                into_response(axum ::
                                Json(TryUpdateManyRouteLogicResponseVariants ::
                                from(error))); * response.status_mut() = axum :: http ::
                                StatusCode :: INTERNAL_SERVER_ERROR; return response;
                            }
                        }));
                    }
                }
                query.push_str(& format!
                ("{}{}", acc,
                "else sqlx_types_uuid_uuid_as_postgresql_uuid end,"));
            }
        } let _ = query.pop();
        query.push_str(& format!
        (" where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key in ({}) returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key;",
        {
            let mut acc = std :: string :: String :: default(); for element in
            & parameters.payload.0
            {
                match postgresql_crud :: BindQuery ::
                try_generate_bind_increments(&
                element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                & mut increment)
                {
                    Ok(value) => { acc.push_str(& format! ("{value},")); },
                    Err(error) =>
                    {
                        let error = TryUpdateManyRouteLogicErrorNamed :: BindQuery
                        {
                            bind_query : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1847, column : 21,
                            }))
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryUpdateManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: INTERNAL_SERVER_ERROR; return response;
                    }
                }
            } let _ = acc.pop(); acc
        })); query
    }; println! ("{}", query_string); let binded_query =
    {
        let mut query = sqlx :: query :: < sqlx :: Postgres >
        (& query_string);
        {
            let mut is_std_primitive_bool_as_postgresql_bool_update_exist =
            false; for element in & parameters.payload.0
            {
                if element.std_primitive_bool_as_postgresql_bool.is_some()
                {
                    is_std_primitive_bool_as_postgresql_bool_update_exist =
                    true; break;
                }
            } if is_std_primitive_bool_as_postgresql_bool_update_exist
            {
                for element in & parameters.payload.0
                {
                    if let Some(value) = &
                    element.std_primitive_bool_as_postgresql_bool
                    {
                        query =
                        query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.into_inner());
                        query = query.bind(value.value.0);
                    }
                }
            }
        }
        {
            let mut is_std_primitive_i16_as_postgresql_small_int_update_exist
            = false; for element in & parameters.payload.0
            {
                if element.std_primitive_i16_as_postgresql_small_int.is_some()
                {
                    is_std_primitive_i16_as_postgresql_small_int_update_exist =
                    true; break;
                }
            } if is_std_primitive_i16_as_postgresql_small_int_update_exist
            {
                for element in & parameters.payload.0
                {
                    if let Some(value) = &
                    element.std_primitive_i16_as_postgresql_small_int
                    {
                        query =
                        query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.into_inner());
                        query = query.bind(value.value.0);
                    }
                }
            }
        }
        {
            let mut is_std_primitive_i32_as_postgresql_int_update_exist =
            false; for element in & parameters.payload.0
            {
                if element.std_primitive_i32_as_postgresql_int.is_some()
                {
                    is_std_primitive_i32_as_postgresql_int_update_exist = true;
                    break;
                }
            } if is_std_primitive_i32_as_postgresql_int_update_exist
            {
                for element in & parameters.payload.0
                {
                    if let Some(value) = &
                    element.std_primitive_i32_as_postgresql_int
                    {
                        query =
                        query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.into_inner());
                        query = query.bind(value.value.0);
                    }
                }
            }
        }
        {
            let mut is_sqlx_types_uuid_uuid_as_postgresql_uuid_update_exist =
            false; for element in & parameters.payload.0
            {
                if element.sqlx_types_uuid_uuid_as_postgresql_uuid.is_some()
                {
                    is_sqlx_types_uuid_uuid_as_postgresql_uuid_update_exist =
                    true; break;
                }
            } if is_sqlx_types_uuid_uuid_as_postgresql_uuid_update_exist
            {
                for element in & parameters.payload.0
                {
                    if let Some(value) = &
                    element.sqlx_types_uuid_uuid_as_postgresql_uuid
                    {
                        query =
                        query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.into_inner());
                        query = query.bind(value.value.0);
                    }
                }
            }
        }
        {
            for element in & parameters.payload.0
            {
                query =
                query.bind(element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.into_inner());
            }
        } query
    }; let mut pool_connection = match
    app_state.get_postgres_pool().acquire().await
    {
        Ok(value) => value, Err(error) =>
        {
            let error = TryUpdateManyRouteLogicErrorNamed :: Postgresql
            {
                postgresql : error, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1767, column : 21,
                })),
            }; eprintln! ("{error}"); ; let mut res = axum :: response ::
            IntoResponse ::
            into_response(axum ::
            Json(TryUpdateManyRouteLogicResponseVariants :: from(error))); *
            res.status_mut() = axum :: http :: StatusCode :: CREATED; return
            res;
        }
    }; let pg_connection = match sqlx :: Acquire ::
    acquire(& mut pool_connection).await
    {
        Ok(value) => value, Err(error) =>
        {
            let error = TryUpdateManyRouteLogicErrorNamed :: Postgresql
            {
                postgresql : error, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1767, column : 21,
                })),
            }; eprintln! ("{error}"); ; let mut res = axum :: response ::
            IntoResponse ::
            into_response(axum ::
            Json(TryUpdateManyRouteLogicResponseVariants :: from(error))); *
            res.status_mut() = axum :: http :: StatusCode :: CREATED; return
            res;
        }
    }; let value =
    {
        let mut postgres_transaction = match
        { use sqlx :: Acquire; pg_connection.begin() }.await
        {
            Ok(value) => value, Err(error) =>
            {
                let error = TryUpdateManyRouteLogicErrorNamed :: Postgresql
                {
                    postgresql : error, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 1767, column : 21,
                    })),
                }; eprintln! ("{error}"); let mut response = axum :: response
                :: IntoResponse ::
                into_response(axum ::
                Json(TryUpdateManyRouteLogicResponseVariants :: from(error)));
                * response.status_mut() = axum :: http :: StatusCode ::
                INTERNAL_SERVER_ERROR; return response;
            }
        }; let results_vec =
        {
            let mut results_vec = std :: vec :: Vec ::
            with_capacity(expected_updated_primary_keys.len()); let mut
            option_error : Option < sqlx :: Error > = None;
            {
                let mut rows =
                binded_query.fetch(postgres_transaction.as_mut()); while let
                (Some(Some(row)), None) =
                (match { use futures :: TryStreamExt; rows.try_next() }.await
                {
                    Ok(value) => Some(value), Err(error) =>
                    { option_error = Some(error); None }
                }, & option_error,) { results_vec.push(row); }
            } if let Some(error) = option_error
            {
                match postgres_transaction.rollback().await
                {
                    Ok(_) =>
                    {
                        let error = TryUpdateManyRouteLogicErrorNamed :: Postgresql
                        {
                            postgresql : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1767, column : 21,
                            })),
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryUpdateManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: INTERNAL_SERVER_ERROR; return response;
                    } Err(rollback_error) =>
                    {
                        let error = TryUpdateManyRouteLogicErrorNamed ::
                        QueryAndRollbackFailed
                        {
                            query : error, rollback : rollback_error, code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 873, column : 21,
                            })),
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryUpdateManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: INTERNAL_SERVER_ERROR; return response;
                    }
                }
            } results_vec
        }; let primary_key_vec =
        {
            let mut primary_key_vec = std :: vec :: Vec ::
            with_capacity(expected_updated_primary_keys.len()); for element in
            results_vec
            {
                match primary_key_try_from_sqlx_row(& element)
                {
                    Ok(primary_key) => { primary_key_vec.push(primary_key); }
                    Err(error) => match postgres_transaction.rollback().await
                    {
                        Ok(_) =>
                        {
                            let error = TryUpdateManyRouteLogicErrorNamed :: Postgresql
                            {
                                postgresql : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1767, column : 21,
                                })),
                            }; eprintln! ("{error}"); let mut response = axum ::
                            response :: IntoResponse ::
                            into_response(axum ::
                            Json(TryUpdateManyRouteLogicResponseVariants ::
                            from(error))); * response.status_mut() = axum :: http ::
                            StatusCode :: INTERNAL_SERVER_ERROR; return response;
                        } Err(rollback_error) =>
                        {
                            let error = TryUpdateManyRouteLogicErrorNamed ::
                            PrimaryKeyFromRowAndFailedRollback
                            {
                                primary_key_from_row : error, rollback : rollback_error,
                                code_occurence : error_occurence_lib :: code_occurence ::
                                CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 919, column : 21,
                                })),
                            }; eprintln! ("{error}"); let mut response = axum ::
                            response :: IntoResponse ::
                            into_response(axum ::
                            Json(TryUpdateManyRouteLogicResponseVariants ::
                            from(error))); * response.status_mut() = axum :: http ::
                            StatusCode :: INTERNAL_SERVER_ERROR; return response;
                        }
                    },
                }
            } primary_key_vec
        };
        {
            let non_existing_primary_keys =
            {
                let len = expected_updated_primary_keys.len();
                expected_updated_primary_keys.into_iter().fold(std :: vec ::
                Vec :: with_capacity(len), | mut acc, element |
                {
                    if let false = primary_key_vec.contains(& element)
                    { acc.push(element); } acc
                })
            }; if let false = non_existing_primary_keys.is_empty()
            {
                match postgres_transaction.rollback().await
                {
                    Ok(_) =>
                    {
                        let error = TryUpdateManyRouteLogicErrorNamed ::
                        NonExistingPrimaryKeys
                        {
                            non_existing_primary_keys, code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1127, column : 21,
                            })),
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryUpdateManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: BAD_REQUEST; return response;
                    } Err(error) =>
                    {
                        let error = TryUpdateManyRouteLogicErrorNamed ::
                        NonExistingPrimaryKeysAndFailedRollback
                        {
                            non_existing_primary_keys, rollback : error, code_occurence
                            : error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1170, column : 21,
                            })),
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryUpdateManyRouteLogicResponseVariants ::
                        from(error))); * response.status_mut() = axum :: http ::
                        StatusCode :: BAD_REQUEST; return response;
                    }
                }
            }
        } match postgres_transaction.commit().await
        {
            Ok(_) =>
            primary_key_vec.into_iter().map(| element |
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize ::
            from(element)).collect(), Err(error) =>
            {
                let error = TryUpdateManyRouteLogicErrorNamed :: CommitFailed
                {
                    commit_failed : error, code_occurence : error_occurence_lib
                    :: code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 1207, column : 21,
                    })),
                }; eprintln! ("{error}"); let mut response = axum :: response
                :: IntoResponse ::
                into_response(axum ::
                Json(TryUpdateManyRouteLogicResponseVariants :: from(error)));
                * response.status_mut() = axum :: http :: StatusCode ::
                INTERNAL_SERVER_ERROR; return response;
            }
        }
    }; let mut response = axum :: response :: IntoResponse ::
    into_response(axum ::
    Json(TryUpdateManyRouteLogicResponseVariants :: Desirable(value))); *
    response.status_mut() = axum :: http :: StatusCode :: OK; return response;
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyErrorNamed
{
    SerdeJsonToString
    {
        #[eo_to_std_string_string] serde_json_to_string : serde_json :: Error,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }, FailedToGetResponseText
    {
        #[eo_to_std_string_string] status_code : http :: StatusCode,
        #[eo_to_std_string_string] headers : reqwest :: header :: HeaderMap,
        #[eo_to_std_string_string] reqwest : reqwest :: Error, code_occurence
        : error_occurence_lib :: code_occurence :: CodeOccurence
    }, DeserializeResponse
    {
        #[eo_to_std_string_string] status_code : http :: StatusCode,
        #[eo_to_std_string_string] headers : reqwest :: header :: HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize] response_text : std
        :: string :: String, #[eo_to_std_string_string] serde : serde_json ::
        Error, code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }, Reqwest
    {
        #[eo_to_std_string_string] reqwest : reqwest :: Error, code_occurence
        : error_occurence_lib :: code_occurence :: CodeOccurence
    }, NotUniquePrimaryKey
    {
        #[eo_to_std_string_string] not_unique_primary_key : postgresql_crud ::
        StdPrimitiveI64, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence
    }, TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize
    {
        #[eo_to_std_string_string]
        try_update_many_route_logic_error_named_with_serialize_deserialize :
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }
} pub async fn
try_update_many(server_location : & std :: primitive :: str, parameters :
UpdateManyParameters,) -> Result < std :: vec :: Vec <
postgresql_crud::StdPrimitiveI64 > , TryUpdateManyErrorNamed >
{
    let payload =
    {
        let mut acc = std :: vec :: Vec :: new(); for element in &
        parameters.payload.0
        {
            if !
            acc.contains(&&
            element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key)
            {
                acc.push(&
                element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key);
            } else
            {
                return
                Err(TryUpdateManyErrorNamed :: NotUniquePrimaryKey
                {
                    not_unique_primary_key :
                    element.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key.clone(),
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 1900, column : 21,
                    })),
                });
            }
        } let value = UpdateManyPayloadWithSerializeDeserialize ::
        from(parameters.payload); match serde_json :: to_string(& value)
        {
            Ok(value) => value, Err(error) =>
            {
                return
                Err(TryUpdateManyErrorNamed :: SerdeJsonToString
                {
                    serde_json_to_string : error, code_occurence :
                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 1534, column : 21,
                    })),
                });
            }
        }
    }; let url = format! ("{}/dogs/update_many", server_location,); let future
    = reqwest :: Client ::
    new().patch(&
    url).header(& postgresql_crud :: CommitSnakeCase.to_string(), git_info ::
    PROJECT_GIT_INFO.commit,).header(reqwest :: header :: CONTENT_TYPE,
    "application/json").body(payload).send(); let response = match
    future.await
    {
        Ok(value) => value, Err(error) =>
        {
            return
            Err(TryUpdateManyErrorNamed :: Reqwest
            {
                reqwest : error, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1693, column : 21,
                })),
            });
        }
    }; let status_code = response.status(); let headers =
    response.headers().clone(); let response_text = match
    response.text().await
    {
        Ok(value) => value, Err(error) =>
        {
            return
            Err(TryUpdateManyErrorNamed :: FailedToGetResponseText
            {
                status_code, headers, reqwest : error, code_occurence :
                error_occurence_lib :: code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1590, column : 21,
                }))
            });
        }
    }; let expected_response = match serde_json :: from_str :: <
    TryUpdateManyRouteLogicResponseVariants > (& response_text)
    {
        Ok(value) => value, Err(error) =>
        {
            return
            Err(TryUpdateManyErrorNamed :: DeserializeResponse
            {
                status_code, headers, response_text, serde : error,
                code_occurence : error_occurence_lib :: code_occurence ::
                CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1653, column : 21,
                })),
            });
        }
    }; let try_update_many_route_logic_error_named_with_serialize_deserialize
    = match expected_response
    {
        TryUpdateManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value =
            value.into_iter().map(| element | postgresql_crud::StdPrimitiveI64
            :: from(element)).collect(); return Ok(value);
        }, TryUpdateManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        Postgresql { postgresql, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: Json
        { json, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryUpdateManyRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: QueryAndRollbackFailed
        { query, rollback, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        QueryAndRollbackFailed { query, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants ::
        PrimaryKeyFromRowAndFailedRollback
        { primary_key_from_row, rollback, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        PrimaryKeyFromRowAndFailedRollback
        { primary_key_from_row, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: NonExistingPrimaryKeys
        { non_existing_primary_keys, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
        TryUpdateManyRouteLogicResponseVariants ::
        NonExistingPrimaryKeysAndFailedRollback
        { non_existing_primary_keys, rollback, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NonExistingPrimaryKeysAndFailedRollback
        { non_existing_primary_keys, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: CommitFailed
        { commit_failed, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CommitFailed { commit_failed, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: NotUniquePrimaryKey
        { not_unique_primary_key, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: BindQuery
        { bind_query, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize :: BindQuery
        { bind_query, code_occurence },
        TryUpdateManyRouteLogicResponseVariants :: NoPayloadFields
        { no_payload_fields, code_occurence } =>
        TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NoPayloadFields { no_payload_fields, code_occurence },
        TryUpdateManyRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }, TryUpdateManyRouteLogicResponseVariants ::
        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
        {
            update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
            code_occurence
        } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        UpdateManyPayloadTryFromUpdateManyPayloadWithSerializeDeserialize
        {
            update_many_payload_try_from_update_many_payload_with_serialize_deserialize,
            code_occurence
        }
    };
    Err(TryUpdateManyErrorNamed ::
    TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize
    {
        try_update_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence ::
        new(file! ().to_owned(), line! (), column! (),
        Some(error_occurence_lib :: code_occurence :: MacroOccurence
        {
            file : std :: string :: String ::
            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line
            : 7034, column : 13,
        })),
    })
}