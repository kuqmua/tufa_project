#[derive(Debug)] pub struct CreateManyPayloadElement
{
    pub std_primitive_bool_as_postgresql_bool :
    postgresql_crud::StdOptionOptionStdPrimitiveBool, pub
    std_primitive_i16_as_postgresql_small_int :
    postgresql_crud::StdOptionOptionStdPrimitiveI16, pub
    std_primitive_i32_as_postgresql_int :
    postgresql_crud::StdOptionOptionStdPrimitiveI32, pub
    sqlx_types_uuid_uuid_as_postgresql_uuid :
    postgresql_crud::StdOptionOptionSqlxTypesUuidUuid
} #[derive(Debug)] pub struct
CreateManyPayload(pub std :: vec :: Vec < CreateManyPayloadElement >);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElementWithSerializeDeserialize
{
    pub std_primitive_bool_as_postgresql_bool :
    postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize,
    pub std_primitive_i16_as_postgresql_small_int :
    postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize,
    pub std_primitive_i32_as_postgresql_int :
    postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize,
    pub sqlx_types_uuid_uuid_as_postgresql_uuid :
    postgresql_crud::StdOptionOptionSqlxTypesUuidUuidWithSerializeDeserialize
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct
CreateManyPayloadWithSerializeDeserialize(pub std :: vec :: Vec <
CreateManyPayloadElementWithSerializeDeserialize >);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum
CreateManyPayloadElementTryFromCreateManyPayloadElementWithSerializeDeserializeErrorNamed
{
    SqlxTypesUuidUuidAsPostgresqlUuid
    {
        #[eo_error_occurence] std_primitive_i_64 :
        postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserializeErrorNamed,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    },
} impl std :: convert :: TryFrom <
CreateManyPayloadElementWithSerializeDeserialize > for
CreateManyPayloadElement
{
    type Error =
    CreateManyPayloadElementTryFromCreateManyPayloadElementWithSerializeDeserializeErrorNamed;
    fn try_from(value : CreateManyPayloadElementWithSerializeDeserialize) ->
    Result < Self, Self :: Error >
    {
        let std_primitive_bool_as_postgresql_bool =
        postgresql_crud::StdOptionOptionStdPrimitiveBool ::
        from(value.std_primitive_bool_as_postgresql_bool); let
        std_primitive_i16_as_postgresql_small_int =
        postgresql_crud::StdOptionOptionStdPrimitiveI16 ::
        from(value.std_primitive_i16_as_postgresql_small_int); let
        std_primitive_i32_as_postgresql_int =
        postgresql_crud::StdOptionOptionStdPrimitiveI32 ::
        from(value.std_primitive_i32_as_postgresql_int); let
        sqlx_types_uuid_uuid_as_postgresql_uuid = match
        postgresql_crud::StdOptionOptionSqlxTypesUuidUuid ::
        try_from(value.sqlx_types_uuid_uuid_as_postgresql_uuid)
        {
            Ok(value) => value, Err(error) =>
            {
                return
                Err(Self :: Error :: SqlxTypesUuidUuidAsPostgresqlUuid
                {
                    std_primitive_i_64 : error, code_occurence :
                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 8995, column : 9,
                    }))
                });
            }
        };
        Ok(Self
        {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            sqlx_types_uuid_uuid_as_postgresql_uuid
        })
    }
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum
CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamed
{
    StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
    {
        #[eo_error_occurence] std_primitive_i_64 :
        CreateManyPayloadElementTryFromCreateManyPayloadElementWithSerializeDeserializeErrorNamed,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    },
} impl std :: convert :: TryFrom < CreateManyPayloadWithSerializeDeserialize >
for CreateManyPayload
{
    type Error =
    CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value : CreateManyPayloadWithSerializeDeserialize) -> Result <
    Self, Self :: Error >
    {
        let mut elements = std :: vec :: Vec :: with_capacity(value.0.len());
        for element in value.0
        {
            match CreateManyPayloadElement :: try_from(element)
            {
                Ok(value) => { elements.push(value); }, Err(error) =>
                {
                    return
                    Err(Self :: Error ::
                    StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey
                    {
                        std_primitive_i_64 : error, code_occurence :
                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2090, column : 21,
                        }))
                    });
                }
            }
        } Ok(Self(elements))
    }
} impl std :: convert :: From < CreateManyPayloadElement > for
CreateManyPayloadElementWithSerializeDeserialize
{
    fn from(value : CreateManyPayloadElement) -> Self
    {
        let std_primitive_bool_as_postgresql_bool =
        postgresql_crud::StdOptionOptionStdPrimitiveBoolWithSerializeDeserialize
        :: from(value.std_primitive_bool_as_postgresql_bool); let
        std_primitive_i16_as_postgresql_small_int =
        postgresql_crud::StdOptionOptionStdPrimitiveI16WithSerializeDeserialize
        :: from(value.std_primitive_i16_as_postgresql_small_int); let
        std_primitive_i32_as_postgresql_int =
        postgresql_crud::StdOptionOptionStdPrimitiveI32WithSerializeDeserialize
        :: from(value.std_primitive_i32_as_postgresql_int); let
        sqlx_types_uuid_uuid_as_postgresql_uuid =
        postgresql_crud::StdOptionOptionSqlxTypesUuidUuidWithSerializeDeserialize
        :: from(value.sqlx_types_uuid_uuid_as_postgresql_uuid); Self
        {
            std_primitive_bool_as_postgresql_bool,
            std_primitive_i16_as_postgresql_small_int,
            std_primitive_i32_as_postgresql_int,
            sqlx_types_uuid_uuid_as_postgresql_uuid
        }
    }
} impl std :: convert :: From < CreateManyPayload > for
CreateManyPayloadWithSerializeDeserialize
{
    fn from(value : CreateManyPayload) -> Self
    {
        Self(value.0.into_iter().map(| element |
        CreateManyPayloadElementWithSerializeDeserialize ::
        from(element)).collect :: < std :: vec :: Vec <
        CreateManyPayloadElementWithSerializeDeserialize >> ())
    }
} #[derive(Debug)] pub struct CreateManyParameters
{ pub payload : CreateManyPayload, }
#[derive(Debug, serde :: Serialize, serde :: Deserialize)] pub enum
TryCreateManyRouteLogicResponseVariants
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
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : std :: string :: String, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence,
    }, CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
    {
        create_many_payload_try_from_create_many_payload_with_serialize_deserialize
        :
        CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence,
    }
} impl std :: convert :: From < TryCreateManyRouteLogicErrorNamed > for
TryCreateManyRouteLogicResponseVariants
{
    fn from(value : TryCreateManyRouteLogicErrorNamed) -> Self
    {
        match value.into_serialize_deserialize_version()
        {
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckBodySize { check_body_size, code_occurence } => Self ::
            CheckBodySize { check_body_size, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            Postgresql { postgresql, code_occurence } => Self :: Postgresql
            { postgresql, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
            { json, code_occurence } => Self :: Json { json, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CheckCommit { check_commit, code_occurence } => Self ::
            CheckCommit { check_commit, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            } => Self ::
            OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
            {
                operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
                code_occurence
            }, TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            }
        }
    }
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence,)]
pub enum TryCreateManyRouteLogicErrorNamed
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
    },
    OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
    {
        #[eo_to_std_string_string]
        operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server
        : sqlx :: Error, code_occurence : error_occurence_lib ::
        code_occurence :: CodeOccurence
    }, CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
    {
        #[eo_error_occurence]
        create_many_payload_try_from_create_many_payload_with_serialize_deserialize
        :
        CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }
} pub async fn
try_create_many_route_logic(app_state : axum :: extract :: State < crate ::
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
            TryCreateManyRouteLogicErrorNamed :: CheckBodySize
            {
                check_body_size : error, code_occurence : error_occurence_lib
                :: code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1880, column : 13,
                })),
            }; eprintln! ("{error}"); let mut response = axum :: response ::
            IntoResponse ::
            into_response(axum ::
            Json(TryCreateManyRouteLogicResponseVariants :: from(error))); *
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
    } println!("kekw"); let parameters = CreateManyParameters
    {
        payload : match axum :: Json :: <
        CreateManyPayloadWithSerializeDeserialize > ::
        from_bytes(& body_bytes,)
        {
            Ok(axum :: Json(value)) => match CreateManyPayload ::
            try_from(value)
            {
                Ok(value) => value, Err(error) =>
                {
                    let error = TryCreateManyRouteLogicErrorNamed ::
                    CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
                    {
                        create_many_payload_try_from_create_many_payload_with_serialize_deserialize
                        : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 8075, column : 13,
                        })),
                    }; eprintln! ("{error}"); let mut response = axum ::
                    response :: IntoResponse ::
                    into_response(axum ::
                    Json(TryCreateManyRouteLogicResponseVariants ::
                    from(error))); * response.status_mut() = axum :: http ::
                    StatusCode :: BAD_REQUEST; return response;
                }
            }, Err(error) =>
            {
                let error = TryCreateManyRouteLogicErrorNamed :: Json
                {
                    json : error, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(file! ().to_owned(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 1965, column : 21,
                    })),
                }; eprintln! ("{error}"); let mut response = axum :: response
                :: IntoResponse ::
                into_response(axum ::
                Json(TryCreateManyRouteLogicResponseVariants :: from(error)));
                * response.status_mut() = axum :: http :: StatusCode ::
                BAD_REQUEST; return response;
            }
        },
    }; println! ("{:#?}", parameters); let query_string =
    "insert into dogs (std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int, sqlx_types_uuid_uuid_as_postgresql_uuid) select std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int, sqlx_types_uuid_uuid_as_postgresql_uuid from unnest($1, $2, $3, $4) as a(std_primitive_bool_as_postgresql_bool, std_primitive_i16_as_postgresql_small_int, std_primitive_i32_as_postgresql_int, sqlx_types_uuid_uuid_as_postgresql_uuid) returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key";
    println! ("{}", query_string); let binded_query =
    {
        let mut query = sqlx :: query :: < sqlx :: Postgres >
        (& query_string); let current_vec_len = parameters.payload.0.len();
        let
        (std_primitive_bool_as_postgresql_bool_vec,
        std_primitive_i16_as_postgresql_small_int_vec,
        std_primitive_i32_as_postgresql_int_vec,
        sqlx_types_uuid_uuid_as_postgresql_uuid_vec) =
        parameters.payload.0.into_iter().fold((std :: vec :: Vec ::
        with_capacity(current_vec_len), std :: vec :: Vec ::
        with_capacity(current_vec_len), std :: vec :: Vec ::
        with_capacity(current_vec_len), std :: vec :: Vec ::
        with_capacity(current_vec_len)), | mut acc, element |
        {
            acc.0.push(element.std_primitive_bool_as_postgresql_bool);
            acc.1.push(element.std_primitive_i16_as_postgresql_small_int);
            acc.2.push(element.std_primitive_i32_as_postgresql_int);
            acc.3.push(element.sqlx_types_uuid_uuid_as_postgresql_uuid); acc
        }); query =
        query.bind(postgresql_crud::StdOptionOptionStdPrimitiveBool ::
        into_inner_type_vec(std_primitive_bool_as_postgresql_bool_vec)); query
        =
        query.bind(postgresql_crud::StdOptionOptionStdPrimitiveI16 ::
        into_inner_type_vec(std_primitive_i16_as_postgresql_small_int_vec));
        query =
        query.bind(postgresql_crud::StdOptionOptionStdPrimitiveI32 ::
        into_inner_type_vec(std_primitive_i32_as_postgresql_int_vec)); query =
        query.bind(postgresql_crud::StdOptionOptionSqlxTypesUuidUuid ::
        into_inner_type_vec(sqlx_types_uuid_uuid_as_postgresql_uuid_vec));
        query
    }; let mut pool_connection = match
    app_state.get_postgres_pool().acquire().await
    {
        Ok(value) => value, Err(error) =>
        {
            let error = TryCreateManyRouteLogicErrorNamed :: Postgresql
            {
                postgresql : error, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1928, column : 21,
                })),
            }; eprintln! ("{error}"); ; let mut res = axum :: response ::
            IntoResponse ::
            into_response(axum ::
            Json(TryCreateManyRouteLogicResponseVariants :: from(error))); *
            res.status_mut() = axum :: http :: StatusCode :: CREATED; return
            res;
        }
    }; let pg_connection = match sqlx :: Acquire ::
    acquire(& mut pool_connection).await
    {
        Ok(value) => value, Err(error) =>
        {
            let error = TryCreateManyRouteLogicErrorNamed :: Postgresql
            {
                postgresql : error, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1928, column : 21,
                })),
            }; eprintln! ("{error}"); ; let mut res = axum :: response ::
            IntoResponse ::
            into_response(axum ::
            Json(TryCreateManyRouteLogicResponseVariants :: from(error))); *
            res.status_mut() = axum :: http :: StatusCode :: CREATED; return
            res;
        }
    }; let value =
    {
        let mut rows = binded_query.fetch(pg_connection.as_mut()); let mut
        vec_values = std :: vec :: Vec :: new(); while let Some(value) =
        {
            match { use futures :: TryStreamExt; rows.try_next() }.await
            {
                Ok(value) => value, Err(error) =>
                {
                    let error = TryCreateManyRouteLogicErrorNamed :: Postgresql
                    {
                        postgresql : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1928, column : 21,
                        })),
                    }; eprintln! ("{error}"); let mut response = axum ::
                    response :: IntoResponse ::
                    into_response(axum ::
                    Json(TryCreateManyRouteLogicResponseVariants ::
                    from(error))); * response.status_mut() = axum :: http ::
                    StatusCode :: INTERNAL_SERVER_ERROR; return response;
                }
            }
        }
        {
            match sqlx :: Row :: try_get :: < std::primitive::i64, & std ::
            primitive :: str >
            (& value,
            "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key")
            {
                Ok(value) =>
                {
                    vec_values.push(postgresql_crud::StdPrimitiveI64WithSerializeDeserialize
                    :: from(postgresql_crud::StdPrimitiveI64(value)),);
                } Err(error) =>
                {
                    let error = TryCreateManyRouteLogicErrorNamed :: Postgresql
                    {
                        postgresql : error, code_occurence : error_occurence_lib ::
                        code_occurence :: CodeOccurence ::
                        new(file! ().to_owned(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1928, column : 21,
                        })),
                    }; eprintln! ("{error}"); let mut response = axum ::
                    response :: IntoResponse ::
                    into_response(axum ::
                    Json(TryCreateManyRouteLogicResponseVariants ::
                    from(error))); * response.status_mut() = axum :: http ::
                    StatusCode :: INTERNAL_SERVER_ERROR; return response;
                }
            }
        } vec_values
    }; let mut response = axum :: response :: IntoResponse ::
    into_response(axum ::
    Json(TryCreateManyRouteLogicResponseVariants :: Desirable(value))); *
    response.status_mut() = axum :: http :: StatusCode :: CREATED; return
    response;
} #[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed
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
    }, TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize
    {
        #[eo_to_std_string_string]
        try_create_many_route_logic_error_named_with_serialize_deserialize :
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence
    }
} pub async fn
try_create_many(server_location : & std :: primitive :: str, parameters :
CreateManyParameters,) -> Result < std :: vec :: Vec <
postgresql_crud::StdPrimitiveI64 > , TryCreateManyErrorNamed >
{
    let payload = match serde_json ::
    to_string(& CreateManyPayloadWithSerializeDeserialize ::
    from(parameters.payload,))
    {
        Ok(value) => value, Err(error) =>
        {
            return
            Err(TryCreateManyErrorNamed :: SerdeJsonToString
            {
                serde_json_to_string : error, code_occurence :
                error_occurence_lib :: code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1695, column : 21,
                })),
            });
        }
    }; let url = format! ("{}/dogs/create_many", server_location,); let future
    = reqwest :: Client ::
    new().post(&
    url).header(& postgresql_crud :: CommitSnakeCase.to_string(), git_info ::
    PROJECT_GIT_INFO.commit,).header(reqwest :: header :: CONTENT_TYPE,
    "application/json").body(payload).send(); let response = match
    future.await
    {
        Ok(value) => value, Err(error) =>
        {
            return
            Err(TryCreateManyErrorNamed :: Reqwest
            {
                reqwest : error, code_occurence : error_occurence_lib ::
                code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1854, column : 21,
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
            Err(TryCreateManyErrorNamed :: FailedToGetResponseText
            {
                status_code, headers, reqwest : error, code_occurence :
                error_occurence_lib :: code_occurence :: CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1751, column : 21,
                }))
            });
        }
    }; let expected_response = match serde_json :: from_str :: <
    TryCreateManyRouteLogicResponseVariants > (& response_text)
    {
        Ok(value) => value, Err(error) =>
        {
            return
            Err(TryCreateManyErrorNamed :: DeserializeResponse
            {
                status_code, headers, response_text, serde : error,
                code_occurence : error_occurence_lib :: code_occurence ::
                CodeOccurence ::
                new(file! ().to_owned(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 1814, column : 21,
                })),
            });
        }
    }; let try_create_many_route_logic_error_named_with_serialize_deserialize
    = match expected_response
    {
        TryCreateManyRouteLogicResponseVariants :: Desirable(value) =>
        {
            return
            Ok(value.into_iter().map(| element |
            postgresql_crud::StdPrimitiveI64 :: from(element)).collect());
        }, TryCreateManyRouteLogicResponseVariants :: CheckBodySize
        { check_body_size, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckBodySize { check_body_size, code_occurence },
        TryCreateManyRouteLogicResponseVariants :: Postgresql
        { postgresql, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        Postgresql { postgresql, code_occurence },
        TryCreateManyRouteLogicResponseVariants :: Json
        { json, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize :: Json
        { json, code_occurence }, TryCreateManyRouteLogicResponseVariants ::
        CheckCommit { check_commit, code_occurence } =>
        TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CheckCommit { check_commit, code_occurence },
        TryCreateManyRouteLogicResponseVariants ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        OperationDoneButPrimaryKeyInnerTypeTryFromPrimaryKeyInnerTypeWithSerializeDeserializeFailedInServer
        {
            operation_done_but_primary_key_inner_type_try_from_primary_key_inner_type_with_serialize_deserialize_failed_in_server,
            code_occurence
        }, TryCreateManyRouteLogicResponseVariants ::
        CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
        {
            create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
            code_occurence
        } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize ::
        CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
        {
            create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
            code_occurence
        }
    };
    Err(TryCreateManyErrorNamed ::
    TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize
    {
        try_create_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence : error_occurence_lib :: code_occurence ::
        CodeOccurence ::
        new(file! ().to_owned(), line! (), column! (),
        Some(error_occurence_lib :: code_occurence :: MacroOccurence
        {
            file : std :: string :: String ::
            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"), line
            : 7582, column : 13,
        })),
    })
}