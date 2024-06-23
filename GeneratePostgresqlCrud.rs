#[derive(Debug)] pub struct ReadManyPayload
{
    pub sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key : std ::
    option :: Option < std :: vec :: Vec < postgresql_crud::SqlxTypesUuidUuid
    >> , pub std_primitive_bool_as_postgresql_bool : std :: option :: Option <
    std :: vec :: Vec < postgresql_crud::WhereStdOptionOptionStdPrimitiveBool
    >> , pub std_primitive_i16_as_postgresql_small_int : std :: option ::
    Option < std :: vec :: Vec <
    postgresql_crud::WhereStdOptionOptionStdPrimitiveI16 >> , pub
    std_primitive_i32_as_postgresql_int : std :: option :: Option < std :: vec
    :: Vec < postgresql_crud::WhereStdOptionOptionStdPrimitiveI32 >> , pub
    select : std :: vec :: Vec < DogColumn > , pub order_by : postgresql_crud
    :: OrderBy < DogColumn > , pub limit : postgresql_crud::StdPrimitiveI64,
    pub offset : postgresql_crud::StdPrimitiveI64,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadManyPayloadWithSerializeDeserialize
{
    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key : std ::
    option :: Option < std :: vec :: Vec <
    postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize >> ,
    std_primitive_bool_as_postgresql_bool : std :: option :: Option < std ::
    vec :: Vec <
    postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
    >> , std_primitive_i16_as_postgresql_small_int : std :: option :: Option <
    std :: vec :: Vec <
    postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize
    >> , std_primitive_i32_as_postgresql_int : std :: option :: Option < std
    :: vec :: Vec <
    postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize
    >> , select : std :: vec :: Vec < DogColumn > , order_by : postgresql_crud
    :: OrderBy < DogColumn > , limit :
    postgresql_crud::StdPrimitiveI64WithSerializeDeserialize, offset :
    postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum
ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserializeErrorNamed
{
    SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey
    {
        #[eo_error_occurence] sqlx_types_uuid_uuid :
        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NotUniqueColumn
    {
        #[eo_to_std_string_string_serialize_deserialize] not_unique_column :
        DogColumn, code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }, NotUniquePrimaryKey
    {
        #[eo_to_std_string_string] not_unique_primary_key : postgresql_crud ::
        SqlxTypesUuidUuid, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence
    },
} impl std :: convert :: TryFrom < ReadManyPayloadWithSerializeDeserialize >
for ReadManyPayload
{
    type Error =
    ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value : ReadManyPayloadWithSerializeDeserialize) -> Result <
    Self, Self :: Error >
    {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key =
        match
        value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
        {
            Some(value) =>
            {
                let mut acc = std :: vec :: Vec :: new(); let mut
                not_unique_primary_key_option = None; for element in value
                {
                    match postgresql_crud::SqlxTypesUuidUuid ::
                    try_from(element)
                    {
                        Ok(value) =>
                        {
                            if acc.contains(& value)
                            { not_unique_primary_key_option = Some(value); break; } else
                            { acc.push(value); }
                        }, Err(error) =>
                        {
                            return
                            Err(Self :: Error ::
                            SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey
                            {
                                sqlx_types_uuid_uuid : error, code_occurence :
                                error_occurence_lib :: code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 556, column : 13,
                                }))
                            });
                        }
                    }
                } if let Some(not_unique_primary_key) =
                not_unique_primary_key_option
                {
                    return
                    Err(Self :: Error :: NotUniquePrimaryKey
                    {
                        not_unique_primary_key, code_occurence : error_occurence_lib
                        :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1284, column : 21,
                        })),
                    });
                } Some(acc)
            }, None => None
        }; let std_primitive_bool_as_postgresql_bool = match
        value.std_primitive_bool_as_postgresql_bool
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBool ::
            from(element)).collect()), None => None,
        }; let std_primitive_i16_as_postgresql_small_int = match
        value.std_primitive_i16_as_postgresql_small_int
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16 ::
            from(element)).collect()), None => None,
        }; let std_primitive_i32_as_postgresql_int = match
        value.std_primitive_i32_as_postgresql_int
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32 ::
            from(element)).collect()), None => None,
        }; let select =
        {
            let mut vec = std :: vec :: Vec :: with_capacity(4); for element
            in value.select
            {
                if vec.contains(& element)
                {
                    return
                    Err(Self :: Error :: NotUniqueColumn
                    {
                        not_unique_column : element, code_occurence :
                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1858, column : 21,
                        }))
                    });
                } else { vec.push(element); }
            } vec
        }; let order_by = value.order_by; let limit =
        postgresql_crud::StdPrimitiveI64 :: from(value.limit); let offset =
        postgresql_crud::StdPrimitiveI64 :: from(value.offset);
        Ok(Self
        {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key,
            select, order_by, limit, offset,
        })
    }
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum
ReadManyPayloadWithSerializeDeserializeTryFromReadManyPayloadErrorNamed
{
    NotUniqueColumn
    {
        #[eo_to_std_string_string_serialize_deserialize] not_unique_column :
        DogColumn, code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }
} impl std :: convert :: TryFrom < ReadManyPayload > for
ReadManyPayloadWithSerializeDeserialize
{
    type Error =
    ReadManyPayloadWithSerializeDeserializeTryFromReadManyPayloadErrorNamed;
    fn try_from(value : ReadManyPayload) -> Result < Self, Self :: Error >
    {
        let sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key =
        match
        value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize ::
            from(element)).collect :: < std :: vec :: Vec <
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize >>
            (),), None => None,
        }; let std_primitive_bool_as_postgresql_bool = match
        value.std_primitive_bool_as_postgresql_bool
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
            :: from(element)).collect()), None => None
        }; let std_primitive_i16_as_postgresql_small_int = match
        value.std_primitive_i16_as_postgresql_small_int
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI16WithSerializeDeserialize
            :: from(element)).collect()), None => None
        }; let std_primitive_i32_as_postgresql_int = match
        value.std_primitive_i32_as_postgresql_int
        {
            Some(value) =>
            Some(value.into_iter().map(| element |
            postgresql_crud::WhereStdOptionOptionStdPrimitiveI32WithSerializeDeserialize
            :: from(element)).collect()), None => None
        }; let select =
        {
            let mut vec = std :: vec :: Vec :: with_capacity(4); for element
            in value.select
            {
                if vec.contains(& element)
                {
                    return
                    Err(Self :: Error :: NotUniqueColumn
                    {
                        not_unique_column : element, code_occurence :
                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1858, column : 21,
                        }))
                    });
                } else { vec.push(element); }
            } vec
        }; let order_by = value.order_by; let limit =
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize ::
        from(value.limit); let offset =
        postgresql_crud::StdPrimitiveI64WithSerializeDeserialize ::
        from(value.offset);
        Ok(Self
        {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key,
            select, order_by, limit, offset,
        })
    }
} #[derive(Debug)] pub struct ReadManyParameters
{ pub payload : ReadManyPayload, }
#[derive(Debug, serde :: Serialize, serde :: Deserialize)] pub enum
TryReadManyRouteLogicResponseVariants
{
    Desirable(std :: vec :: Vec :: < DogOptions >), CheckBodySize
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
    }, CheckedAdd
    {
        checked_add : std :: string :: String, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence,
    }, BindQuery
    {
        bind_query : postgresql_crud ::
        TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }, NotUniquePrimaryKeyWithSerializeDeserialize
    {
        not_unique_primary_key_with_serialize_deserialize : std :: string ::
        String, code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : std :: string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
    {
        read_many_payload_try_from_read_many_payload_with_serialize_deserialize
        :
        ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }
} impl std :: convert :: From < TryReadManyRouteLogicErrorNamed > for
TryReadManyRouteLogicResponseVariants
{
    fn from(value : TryReadManyRouteLogicErrorNamed) -> Self
    {
        match value.into_serialize_deserialize_version()
        {
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckedAdd { checked_add, code_occurence } => Self :: CheckedAdd
            { checked_add, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            NotUniquePrimaryKeyWithSerializeDeserialize
            {
                not_unique_primary_key_with_serialize_deserialize,
                code_occurence
            } => Self :: NotUniquePrimaryKeyWithSerializeDeserialize
            {
                not_unique_primary_key_with_serialize_deserialize,
                code_occurence
            }, TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
            ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
            {
                read_many_payload_try_from_read_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
            {
                read_many_payload_try_from_read_many_payload_with_serialize_deserialize,
                code_occurence
            }
        }
    }
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence,)]
pub enum TryReadManyRouteLogicErrorNamed
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
    }, CheckedAdd
    {
        #[eo_to_std_string_string_serialize_deserialize] checked_add : std ::
        string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence
    }, BindQuery
    {
        #[eo_error_occurence] bind_query : postgresql_crud ::
        TryGenerateBindIncrementsErrorNamed, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence
    }, NotUniquePrimaryKeyWithSerializeDeserialize
    {
        #[eo_to_std_string_string]
        not_unique_primary_key_with_serialize_deserialize : postgresql_crud ::
        SqlxTypesUuidUuidWithSerializeDeserialize, code_occurence :
        error_occurence_lib :: code_occurence :: CodeOccurence
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : sqlx :: Error, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence
    }, ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
    {
        #[eo_error_occurence]
        read_many_payload_try_from_read_many_payload_with_serialize_deserialize
        :
        ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }
} pub async fn
try_read_many_route_logic(app_state : axum :: extract :: State < crate ::
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
            TryReadManyRouteLogicErrorNamed :: CheckBodySize
            {
                check_body_size : error, code_occurence : error_occurence_lib
                :: code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 2091, column : 13,
                })),
            }; eprintln! ("{error}"); let mut response = axum :: response ::
            IntoResponse ::
            into_response(axum ::
            Json(TryReadManyRouteLogicResponseVariants :: from(error))); *
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
    } let parameters = ReadManyParameters
    {
        payload : match axum :: Json :: <
        ReadManyPayloadWithSerializeDeserialize > :: from_bytes(& body_bytes,)
        {
            Ok(axum :: Json(value)) =>
            {
                if let
                Some(sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key)
                = &
                value.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
                {
                    let mut acc = std :: vec :: Vec :: new(); for
                    not_unique_primary_key in
                    sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
                    {
                        if ! acc.contains(& not_unique_primary_key)
                        { acc.push(& not_unique_primary_key); } else
                        {
                            let error = TryReadManyRouteLogicErrorNamed ::
                            NotUniquePrimaryKeyWithSerializeDeserialize
                            {
                                not_unique_primary_key_with_serialize_deserialize,
                                code_occurence : error_occurence_lib :: code_occurence ::
                                CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 1331, column : 21,
                                })),
                            }; eprintln! ("{error}"); let mut response = axum ::
                            response :: IntoResponse ::
                            into_response(axum ::
                            Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                            * response.status_mut() = axum :: http :: StatusCode ::
                            INTERNAL_SERVER_ERROR; return response;
                        }
                    }
                } match ReadManyPayload :: try_from(value)
                {
                    Ok(value) => value, Err(error) =>
                    {
                        let error = TryReadManyRouteLogicErrorNamed ::
                        ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
                        {
                            read_many_payload_try_from_read_many_payload_with_serialize_deserialize
                            : error, code_occurence : error_occurence_lib ::
                            code_occurence :: CodeOccurence ::
                            new(file! ().to_owned(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 7421, column : 13,
                            })),
                        }; eprintln! ("{error}"); let mut response = axum ::
                        response :: IntoResponse ::
                        into_response(axum ::
                        Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                        * response.status_mut() = axum :: http :: StatusCode ::
                        BAD_REQUEST; return response;
                    }
                }
            }, Err(error) =>
            {
                let error = TryReadManyRouteLogicErrorNamed :: Json
                {
                    json : error, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 2180, column : 21,
                    })),
                }; eprintln! ("{error}"); let mut response = axum :: response
                :: IntoResponse ::
                into_response(axum ::
                Json(TryReadManyRouteLogicResponseVariants :: from(error))); *
                response.status_mut() = axum :: http :: StatusCode ::
                BAD_REQUEST; return response;
            }
        },
    }; println! ("{:#?}", parameters); let query_string = format!
    ("select {} from dogs {}",
    generate_query_vec_column(& parameters.payload.select),
    {
        let mut increment : u64 = 0; let mut additional_parameters = std ::
        string :: String :: default(); if let Some(value) = &
        parameters.payload.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
        {
            let prefix = match additional_parameters.is_empty()
            { true => "where", false => " and", }; match
            increment.checked_add(1)
            {
                Some(value) => { increment = value; }, None =>
                {
                    let error = TryReadManyRouteLogicErrorNamed :: CheckedAdd
                    {
                        checked_add : std :: string :: String ::
                        from("checked_add is None"), code_occurence :
                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1031, column : 21,
                        })),
                    }; eprintln! ("{error}"); let mut response = axum ::
                    response :: IntoResponse ::
                    into_response(axum ::
                    Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                    * response.status_mut() = axum :: http :: StatusCode ::
                    BAD_REQUEST; return response;
                },
            }
            additional_parameters.push_str(& format!
            ("{} sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key in (select unnest(${}))",
            prefix, increment));
        } if let Some(value) = &
        parameters.payload.std_primitive_bool_as_postgresql_bool
        {
            let prefix = match additional_parameters.is_empty()
            { true => "where", false => " and", }; let bind_increments =
            {
                let mut bind_increments = std :: string :: String ::
                default(); for (index, element) in value.iter().enumerate()
                {
                    match postgresql_crud :: BindQuery ::
                    try_generate_bind_increments(element, & mut increment)
                    {
                        Ok(value) =>
                        {
                            let handle = format!
                            ("std_primitive_bool_as_postgresql_bool ~ {value} "); match
                            index == 0
                            {
                                true => { bind_increments.push_str(& handle); }, false =>
                                {
                                    bind_increments.push_str(& format!
                                    ("{} {handle}", element.conjuctive_operator));
                                },
                            }
                        }, Err(error) =>
                        {
                            let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 2219, column : 21,
                                }))
                            }; eprintln! ("{error}"); let mut response = axum ::
                            response :: IntoResponse ::
                            into_response(axum ::
                            Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                            * response.status_mut() = axum :: http :: StatusCode ::
                            INTERNAL_SERVER_ERROR; return response;
                        },
                    }
                } if let false = bind_increments.is_empty()
                { let _ = bind_increments.pop(); } bind_increments
            };
            additional_parameters.push_str(& format!
            ("{prefix} {bind_increments}"));
        } if let Some(value) = &
        parameters.payload.std_primitive_i16_as_postgresql_small_int
        {
            let prefix = match additional_parameters.is_empty()
            { true => "where", false => " and", }; let bind_increments =
            {
                let mut bind_increments = std :: string :: String ::
                default(); for (index, element) in value.iter().enumerate()
                {
                    match postgresql_crud :: BindQuery ::
                    try_generate_bind_increments(element, & mut increment)
                    {
                        Ok(value) =>
                        {
                            let handle = format!
                            ("std_primitive_i16_as_postgresql_small_int ~ {value} ");
                            match index == 0
                            {
                                true => { bind_increments.push_str(& handle); }, false =>
                                {
                                    bind_increments.push_str(& format!
                                    ("{} {handle}", element.conjuctive_operator));
                                },
                            }
                        }, Err(error) =>
                        {
                            let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 2219, column : 21,
                                }))
                            }; eprintln! ("{error}"); let mut response = axum ::
                            response :: IntoResponse ::
                            into_response(axum ::
                            Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                            * response.status_mut() = axum :: http :: StatusCode ::
                            INTERNAL_SERVER_ERROR; return response;
                        },
                    }
                } if let false = bind_increments.is_empty()
                { let _ = bind_increments.pop(); } bind_increments
            };
            additional_parameters.push_str(& format!
            ("{prefix} {bind_increments}"));
        } if let Some(value) = &
        parameters.payload.std_primitive_i32_as_postgresql_int
        {
            let prefix = match additional_parameters.is_empty()
            { true => "where", false => " and", }; let bind_increments =
            {
                let mut bind_increments = std :: string :: String ::
                default(); for (index, element) in value.iter().enumerate()
                {
                    match postgresql_crud :: BindQuery ::
                    try_generate_bind_increments(element, & mut increment)
                    {
                        Ok(value) =>
                        {
                            let handle = format!
                            ("std_primitive_i32_as_postgresql_int ~ {value} "); match
                            index == 0
                            {
                                true => { bind_increments.push_str(& handle); }, false =>
                                {
                                    bind_increments.push_str(& format!
                                    ("{} {handle}", element.conjuctive_operator));
                                },
                            }
                        }, Err(error) =>
                        {
                            let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                            {
                                bind_query : error, code_occurence : error_occurence_lib ::
                                code_occurence :: CodeOccurence ::
                                new(file! ().to_owned(), line! (), column! (),
                                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                {
                                    file : std :: string :: String ::
                                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line : 2219, column : 21,
                                }))
                            }; eprintln! ("{error}"); let mut response = axum ::
                            response :: IntoResponse ::
                            into_response(axum ::
                            Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                            * response.status_mut() = axum :: http :: StatusCode ::
                            INTERNAL_SERVER_ERROR; return response;
                        },
                    }
                } if let false = bind_increments.is_empty()
                { let _ = bind_increments.pop(); } bind_increments
            };
            additional_parameters.push_str(& format!
            ("{prefix} {bind_increments}"));
        }
        {
            let prefix = match additional_parameters.is_empty()
            { true => "", false => " ", }; let value = &
            parameters.payload.order_by; let order_stringified = match &
            value.order
            {
                Some(order) => order.to_string(), None => postgresql_crud ::
                Order :: default().to_string(),
            };
            additional_parameters.push_str(& format!
            ("{}order_by {} {}", prefix, value.column, order_stringified));
        }
        {
            let prefix = match additional_parameters.is_empty()
            { true => "", false => " ", }; let value = match postgresql_crud
            :: BindQuery ::
            try_generate_bind_increments(& parameters.payload.limit, & mut
            increment)
            {
                Ok(value) => value, Err(error) =>
                {
                    let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                    {
                        bind_query : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2219, column : 21,
                        }))
                    }; eprintln! ("{error}"); let mut response = axum ::
                    response :: IntoResponse ::
                    into_response(axum ::
                    Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                    * response.status_mut() = axum :: http :: StatusCode ::
                    INTERNAL_SERVER_ERROR; return response;
                },
            };
            additional_parameters.push_str(& format!
            ("{}limit {}", prefix, value));
        }
        {
            let prefix = match additional_parameters.is_empty()
            { true => "", false => " ", }; let value = match postgresql_crud
            :: BindQuery ::
            try_generate_bind_increments(& parameters.payload.offset, & mut
            increment)
            {
                Ok(value) => value, Err(error) =>
                {
                    let error = TryReadManyRouteLogicErrorNamed :: BindQuery
                    {
                        bind_query : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2219, column : 21,
                        }))
                    }; eprintln! ("{error}"); let mut response = axum ::
                    response :: IntoResponse ::
                    into_response(axum ::
                    Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                    * response.status_mut() = axum :: http :: StatusCode ::
                    INTERNAL_SERVER_ERROR; return response;
                },
            };
            additional_parameters.push_str(& format!
            ("{}offset {}", prefix, value));
        } additional_parameters
    }); println! ("{}", query_string); let binded_query =
    {
        let mut query = sqlx :: query :: < sqlx :: Postgres >
        (& query_string); if let Some(value) =
        parameters.payload.sqlx_types_uuid_uuid_as_postgresql_uuid_not_null_primary_key
        {
            query =
            query.bind(value.into_iter().map(| element |
            element.into_inner().clone()).collect :: < std :: vec :: Vec <
            sqlx::types::uuid::Uuid >> ());
        } if let Some(values) =
        parameters.payload.std_primitive_bool_as_postgresql_bool
        {
            for value in values
            {
                query = postgresql_crud :: BindQuery ::
                bind_value_to_query(value, query,);
            }
        } if let Some(values) =
        parameters.payload.std_primitive_i16_as_postgresql_small_int
        {
            for value in values
            {
                query = postgresql_crud :: BindQuery ::
                bind_value_to_query(value, query,);
            }
        } if let Some(values) =
        parameters.payload.std_primitive_i32_as_postgresql_int
        {
            for value in values
            {
                query = postgresql_crud :: BindQuery ::
                bind_value_to_query(value, query,);
            }
        } query = postgresql_crud :: BindQuery ::
        bind_value_to_query(parameters.payload.limit, query,); query =
        postgresql_crud :: BindQuery ::
        bind_value_to_query(parameters.payload.offset, query,); query
    }; let mut pool_connection = match
    app_state.get_postgres_pool().acquire().await
    {
        Ok(value) => value, Err(error) =>
        {
            let error = TryReadManyRouteLogicErrorNamed :: Postgresql
            {
                postgresql : error, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 2139, column : 21,
                })),
            }; eprintln! ("{error}"); ; let mut res = axum :: response ::
            IntoResponse ::
            into_response(axum ::
            Json(TryReadManyRouteLogicResponseVariants :: from(error))); *
            res.status_mut() = axum :: http :: StatusCode :: CREATED; return
            res;
        }
    }; let pg_connection = match sqlx :: Acquire ::
    acquire(& mut pool_connection).await
    {
        Ok(value) => value, Err(error) =>
        {
            let error = TryReadManyRouteLogicErrorNamed :: Postgresql
            {
                postgresql : error, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 2139, column : 21,
                })),
            }; eprintln! ("{error}"); ; let mut res = axum :: response ::
            IntoResponse ::
            into_response(axum ::
            Json(TryReadManyRouteLogicResponseVariants :: from(error))); *
            res.status_mut() = axum :: http :: StatusCode :: CREATED; return
            res;
        }
    }; let value =
    {
        let mut rows = binded_query.fetch(pg_connection.as_mut()); let mut
        vec_values = std :: vec :: Vec :: new(); let wrapper_vec_column =
        WrapperVecColumn(parameters.payload.select); while let Some(row) =
        {
            match { use futures :: TryStreamExt; rows.try_next() }.await
            {
                Ok(value) => value, Err(error) =>
                {
                    let error = TryReadManyRouteLogicErrorNamed :: Postgresql
                    {
                        postgresql : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2139, column : 21,
                        })),
                    }; eprintln! ("{error}"); let mut response = axum ::
                    response :: IntoResponse ::
                    into_response(axum ::
                    Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                    * response.status_mut() = axum :: http :: StatusCode ::
                    INTERNAL_SERVER_ERROR; return response;
                }
            }
        }
        {
            match wrapper_vec_column.options_try_from_sqlx_row(& row)
            {
                Ok(value) => { vec_values.push(value); } Err(error) =>
                {
                    let error = TryReadManyRouteLogicErrorNamed :: Postgresql
                    {
                        postgresql : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2139, column : 21,
                        })),
                    }; eprintln! ("{error}"); let mut response = axum ::
                    response :: IntoResponse ::
                    into_response(axum ::
                    Json(TryReadManyRouteLogicResponseVariants :: from(error)));
                    * response.status_mut() = axum :: http :: StatusCode ::
                    INTERNAL_SERVER_ERROR; return response;
                }
            }
        } vec_values
    }; let mut response = axum :: response :: IntoResponse ::
    into_response(axum ::
    Json(TryReadManyRouteLogicResponseVariants :: Desirable(value))); *
    response.status_mut() = axum :: http :: StatusCode :: OK; return response;
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyErrorNamed
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
    }, ReadManyPayloadWithSerializeDeserializeTryFromReadManyPayload
    {
        #[eo_error_occurence]
        read_many_payload_with_serialize_deserialize_try_from_read_many_payload
        :
        ReadManyPayloadWithSerializeDeserializeTryFromReadManyPayloadErrorNamed,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }, TryReadManyRouteLogicErrorNamedWithSerializeDeserialize
    {
        #[eo_to_std_string_string]
        try_read_many_route_logic_error_named_with_serialize_deserialize :
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }
} pub async fn
try_read_many(server_location : & std :: primitive :: str, parameters :
ReadManyParameters,) -> Result < std :: vec :: Vec :: < DogOptions > ,
TryReadManyErrorNamed >
{
    let payload =
    {
        let value = match ReadManyPayloadWithSerializeDeserialize ::
        try_from(parameters.payload)
        {
            Ok(value) => value, Err(error) =>
            {
                return
                Err(TryReadManyErrorNamed ::
                ReadManyPayloadWithSerializeDeserializeTryFromReadManyPayload
                {
                    read_many_payload_with_serialize_deserialize_try_from_read_many_payload
                    : error, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 2389, column : 17,
                    }))
                });
            }
        }; match serde_json :: to_string(& value)
        {
            Ok(value) => value, Err(error) =>
            {
                return
                Err(TryReadManyErrorNamed :: SerdeJsonToString
                {
                    serde_json_to_string : error, code_occurence :
                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 1906, column : 21,
                    })),
                });
            }
        }
    }; let url = format! ("{}/dogs/read_many", server_location,); let future =
    reqwest :: Client ::
    new().post(&
    url).header(& postgresql_crud :: CommitSnakeCase.to_string(), git_info ::
    PROJECT_GIT_INFO.commit,).header(reqwest :: header :: CONTENT_TYPE,
    "application/json").body(payload).send(); let response = match
    future.await
    {
        Ok(value) => value, Err(error) =>
        {
            return
            Err(TryReadManyErrorNamed :: Reqwest
            {
                reqwest : error, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 2065, column : 21,
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
            Err(TryReadManyErrorNamed :: FailedToGetResponseText
            {
                status_code, headers, reqwest : error, code_occurence :
                error_occurence_lib :: code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1962, column : 21,
                }))
            });
        }
    }; let expected_response = match serde_json :: from_str :: <
    TryReadManyRouteLogicResponseVariants > (& response_text)
    {
        Ok(value) => value, Err(error) =>
        {
            return
            Err(TryReadManyErrorNamed :: DeserializeResponse
            {
                status_code, headers, response_text, serde : error,
                code_occurence : error_occurence_lib :: code_occurence ::
                CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 2025, column : 21,
                })),
            });
        }
    }; let try_read_many_route_logic_error_named_with_serialize_deserialize =
    match expected_response
    {
        TryReadManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            let value =
            value.into_iter().fold(std :: vec :: Vec :: new(), | mut acc,
            element | { acc.push(element); acc }); return Ok(value);
        }, TryReadManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryReadManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: Postgresql
        { postgresql, code_occurence }, TryReadManyRouteLogicResponseVariants
        :: Json { json, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryReadManyRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: CheckCommit
        { check_commit, code_occurence },
        TryReadManyRouteLogicResponseVariants :: CheckedAdd
        { checked_add, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: CheckedAdd
        { checked_add, code_occurence }, TryReadManyRouteLogicResponseVariants
        :: BindQuery { bind_query, code_occurence } =>
        TryReadManyRouteLogicErrorNamedWithSerializeDeserialize :: BindQuery
        { bind_query, code_occurence }, TryReadManyRouteLogicResponseVariants
        :: NotUniquePrimaryKeyWithSerializeDeserialize
        { not_unique_primary_key_with_serialize_deserialize, code_occurence }
        => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        NotUniquePrimaryKeyWithSerializeDeserialize
        { not_unique_primary_key_with_serialize_deserialize, code_occurence },
        TryReadManyRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }, TryReadManyRouteLogicResponseVariants ::
        ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
        {
            read_many_payload_try_from_read_many_payload_with_serialize_deserialize,
            code_occurence
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize ::
        ReadManyPayloadTryFromReadManyPayloadWithSerializeDeserialize
        {
            read_many_payload_try_from_read_many_payload_with_serialize_deserialize,
            code_occurence
        }
    };
    Err(TryReadManyErrorNamed ::
    TryReadManyRouteLogicErrorNamedWithSerializeDeserialize
    {
        try_read_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence ::
        new(file! ().to_owned(), line! (), column! (),
        Some(error_occurence_lib :: code_occurence :: MacroOccurence
        {
            file : std :: string :: String ::
            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line
            : 6923, column : 13,
        })),
    })
}