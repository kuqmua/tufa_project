pub const TABLE_NAME: &std::primitive::str = "jsongeneric";
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct JsongenericOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<postgresql_crud::Value<postgresql_crud::StdPrimitiveI64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::StdPrimitiveBool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqlx_types_json_t_as_postgresql_json_b_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::SqlxTypesJson<SomethingReader>>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, PartialEq, Clone)]
pub enum JsongenericColumn {
    #[serde(rename(serialize = "std_primitive_bool_as_postgresql_bool_not_null", deserialize = "std_primitive_bool_as_postgresql_bool_not_null"))]
    StdPrimitiveBoolAsPostgresqlBoolNotNull,
    #[serde(rename(serialize = "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key", deserialize = "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key"))]
    StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
    #[serde(rename(serialize = "sqlx_types_json_t_as_postgresql_json_b_not_null", deserialize = "sqlx_types_json_t_as_postgresql_json_b_not_null"))]
    SqlxTypesJsonTAsPostgresqlJsonBNotNull { filter: std::vec::Vec<SomethingFieldToRead> },
}
impl std::fmt::Display for JsongenericColumn {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", serde_json::to_string(&self).unwrap_or_else(|e| format!("cannot serialize into json: {e:?}")))
    }
}
impl error_occurence_lib::ToStdStringString for JsongenericColumn {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for JsongenericColumn {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            JsongenericColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull,
            JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey,
            JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull {
                filter: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            },
        ]
    }
}
pub const ALLOW_METHODS: [http::Method; 4] = [http::Method::GET, http::Method::POST, http::Method::PATCH, http::Method::DELETE];
#[derive(Debug, Clone, Copy)]
pub struct JsongenericColumnReadPermission {
    std_primitive_bool_as_postgresql_bool_not_null: std::primitive::bool,
    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::primitive::bool,
    sqlx_types_json_t_as_postgresql_json_b_not_null: std::primitive::bool,
}
pub use postgresql_crud::SqlxTypesJson;
pub use postgresql_crud::StdPrimitiveBool;
pub use postgresql_crud::StdPrimitiveI64;
pub async fn create_table_if_not_exists(pool: &sqlx::Pool<sqlx::Postgres>) {
    let create_extension_if_not_exists_pg_jsonschema_query_stringified = "create extension if not exists pg_jsonschema";
    println!("{create_extension_if_not_exists_pg_jsonschema_query_stringified}");
    let _ = sqlx::query(create_extension_if_not_exists_pg_jsonschema_query_stringified).execute(pool).await.unwrap();
    let create_table_if_not_exists_query_stringified = format!
    ("CREATE TABLE IF NOT EXISTS jsongeneric (std_primitive_bool_as_postgresql_bool_not_null BOOL NOT NULL,std_primitive_i64_as_postgresql_big_serial_not_null_primary_key BIGSERIAL PRIMARY KEY,sqlx_types_json_t_as_postgresql_json_b_not_null JSONB NOT NULL, check (jsonb_matches_schema('{}', sqlx_types_json_t_as_postgresql_json_b_not_null)))",
    serde_json :: to_string(& schemars :: schema_for! (Something)).unwrap());
    println!("{create_table_if_not_exists_query_stringified}");
    let _ = sqlx::query(&create_table_if_not_exists_query_stringified).execute(pool).await.unwrap();
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElement {
    pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBool,
    pub sqlx_types_json_t_as_postgresql_json_b_not_null: SomethingToCreate,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayload(pub std::vec::Vec<CreateManyPayloadElement>);
#[derive(Debug)]
pub struct CreateManyParameters {
    pub payload: CreateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<postgresql_crud::StdPrimitiveI64>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLength {
        expected_length: std::string::String,
        got_length: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLengthAndRollback {
        expected_length: std::string::String,
        got_length: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyRouteLogicErrorNamed> for TryCreateManyRouteLogicResponseVariants {
    fn from(value: TryCreateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLength { expected_length, got_length, code_occurence } => Self::UnexpectedRowsLength { expected_length, got_length, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence } => Self::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLength {
        #[eo_to_std_string_string]
        expected_length: std::primitive::usize,
        #[eo_to_std_string_string]
        got_length: std::primitive::usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLengthAndRollback {
        #[eo_to_std_string_string]
        expected_length: std::primitive::usize,
        #[eo_to_std_string_string]
        got_length: std::primitive::usize,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2144,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    println!("kekw");
    let parameters = CreateManyParameters {
        payload: match serde_json::from_slice::<CreateManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = CreateManyPayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryCreateManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2216,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let error_0 = parameters.payload.0.len();
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        let mut values = std::string::String::default();
        for element in &parameters.payload.0 {
            let mut acc = std::string::String::default();
            match postgresql_crud::BindQuery::try_increment(&element.std_primitive_bool_as_postgresql_bool_not_null, &mut increment) {
                Ok(_) => {
                    acc.push_str(&format!("${},", increment));
                }
                Err(_) => {
                    let error = TryCreateManyRouteLogicErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 2664,
                                column: 264,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            }
            match postgresql_crud::BindQuery::try_generate_bind_increments(&element.sqlx_types_json_t_as_postgresql_json_b_not_null, &mut increment) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(error_0) => {
                    let error = TryCreateManyRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 2652,
                                column: 262,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            let _ = acc.pop();
            values.push_str(&format!("({acc}),"));
        }
        let _ = values.pop();
        format!("insert into jsongeneric (std_primitive_bool_as_postgresql_bool_not_null,sqlx_types_json_t_as_postgresql_json_b_not_null) values {values}  returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key")
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        for element in parameters.payload.0 {
            query = postgresql_crud::BindQuery::bind_value_to_query(element.std_primitive_bool_as_postgresql_bool_not_null, query);
            query = postgresql_crud::BindQuery::bind_value_to_query(element.sqlx_types_json_t_as_postgresql_json_b_not_null, query);
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2166,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2166,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 1904,
                            column: 246,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut rows = binded_query.fetch(executor.as_mut());
            let mut acc = std::vec::Vec::new();
            while let Some(value) = match {
                use postgresql_crud::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => match value {
                    Some(value) => match sqlx::Row::try_get::<std::primitive::i64, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
                        Ok(value) => Some(postgresql_crud::StdPrimitiveI64(value)),
                        Err(error_0) => {
                            drop(rows);
                            match executor.rollback().await {
                                Ok(_) => {
                                    let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2521,
                                                column: 129,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                Err(error_1) => {
                                    let error = TryCreateManyRouteLogicErrorNamed::RowAndRollback {
                                        row: error_0,
                                        rollback: error_1,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2521,
                                                column: 158,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        }
                    },
                    None => None,
                },
                Err(error_0) => {
                    drop(rows);
                    match executor.rollback().await {
                        Ok(_) => {
                            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2523,
                                        column: 125,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryCreateManyRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2523,
                                        column: 154,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            } {
                acc.push(value);
            }
            acc
        };
        {
            let error_1 = value.len();
            if error_0 != error_1 {
                match executor.rollback().await {
                    Ok(_) => {
                        let error = TryCreateManyRouteLogicErrorNamed::UnexpectedRowsLength {
                            expected_length: error_0,
                            got_length: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2718,
                                    column: 278,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_2) => {
                        let error = TryCreateManyRouteLogicErrorNamed::UnexpectedRowsLengthAndRollback {
                            expected_length: error_0,
                            got_length: error_1,
                            rollback: error_2,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2720,
                                    column: 193,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
        }
        if let Err(error_0) = executor.commit().await {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 1916,
                        column: 246,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::CREATED;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_create_many_route_logic_error_named_with_serialize_deserialize: TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many(server_location: &std::primitive::str, parameters: CreateManyParameters) -> Result<std::vec::Vec<postgresql_crud::StdPrimitiveI64>, TryCreateManyErrorNamed> {
    let payload = {
        let value = CreateManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2266,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/jsongeneric/create_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryCreateManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2318,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryCreateManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2335,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryCreateManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryCreateManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2348,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_create_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryCreateManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value.into_iter().map(|element| postgresql_crud::StdPrimitiveI64::from(element)).collect();
            return Ok(value);
        }
        TryCreateManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryCreateManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryCreateManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryCreateManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryCreateManyRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryCreateManyRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
        TryCreateManyRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryCreateManyRouteLogicResponseVariants::UnexpectedRowsLength { expected_length, got_length, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLength { expected_length, got_length, code_occurence },
        TryCreateManyRouteLogicResponseVariants::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence },
    };
    Err(TryCreateManyErrorNamed::TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_create_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2386,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CreateManyPayloadElement {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CreateManyPayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![
            postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ])
    }
}
pub async fn create_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <CreateManyPayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateOnePayload {
    pub std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBool,
    pub sqlx_types_json_t_as_postgresql_json_b_not_null: SomethingToCreate,
}
#[derive(Debug)]
pub struct CreateOneParameters {
    pub payload: CreateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateOneRouteLogicResponseVariants {
    Desirable(postgresql_crud::StdPrimitiveI64),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneRouteLogicErrorNamed> for TryCreateOneRouteLogicResponseVariants {
    fn from(value: TryCreateOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2144,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = CreateOneParameters {
        payload: match serde_json::from_slice::<CreateOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = CreateOnePayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryCreateOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2216,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        format!(
            "insert into jsongeneric (std_primitive_bool_as_postgresql_bool_not_null,sqlx_types_json_t_as_postgresql_json_b_not_null) values ({},{}) returning std_primitive_i64_as_postgresql_big_serial_not_null_primary_key",
            {
                match increment.checked_add(1) {
                    Some(incr) => {
                        increment = incr;
                        format!("${increment}")
                    }
                    None => {
                        let error = TryCreateOneRouteLogicErrorNamed::CheckedAdd {
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2920,
                                    column: 264,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            },
            match postgresql_crud::BindQuery::try_generate_bind_increments(&parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null, &mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryCreateOneRouteLogicErrorNamed::BindQuery {
                        bind_query: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 2910,
                                column: 262,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        )
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.std_primitive_bool_as_postgresql_bool_not_null, query);
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2166,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2166,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 1904,
                            column: 246,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match sqlx::Row::try_get::<std::primitive::i64, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
                    Ok(value) => postgresql_crud::StdPrimitiveI64(value),
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2530,
                                        column: 112,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryCreateOneRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2530,
                                        column: 141,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                },
                Err(error_0) => match executor.rollback().await {
                    Ok(_) => {
                        let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2532,
                                    column: 108,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_1) => {
                        let error = TryCreateOneRouteLogicErrorNamed::RowAndRollback {
                            row: error_0,
                            rollback: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2532,
                                    column: 137,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
            }
        };
        if let Err(error_0) = executor.commit().await {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 1916,
                        column: 246,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::CREATED;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_create_one_route_logic_error_named_with_serialize_deserialize: TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_one(server_location: &std::primitive::str, parameters: CreateOneParameters) -> Result<postgresql_crud::StdPrimitiveI64, TryCreateOneErrorNamed> {
    let payload = {
        let value = CreateOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryCreateOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2266,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/jsongeneric/create_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryCreateOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2318,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryCreateOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2335,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryCreateOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryCreateOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2348,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_create_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryCreateOneRouteLogicResponseVariants::Desirable(value) => {
            let value = postgresql_crud::StdPrimitiveI64::from(value);
            return Ok(value);
        }
        TryCreateOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryCreateOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryCreateOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryCreateOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryCreateOneRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryCreateOneRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryCreateOneRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
    };
    Err(TryCreateOneErrorNamed::TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_create_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2386,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for CreateOnePayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn create_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <CreateOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadManyPayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<std::vec::Vec<postgresql_crud::StdPrimitiveI64>>,
    pub std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<std::vec::Vec<postgresql_crud::WhereStdPrimitiveBool>>,
    pub sqlx_types_json_t_as_postgresql_json_b_not_null: std::option::Option<std::vec::Vec<postgresql_crud::WhereSqlxTypesJson<Something>>>,
    pub select: std::vec::Vec<JsongenericColumn>,
    pub order_by: postgresql_crud::OrderBy<JsongenericColumn>,
    pub limit: postgresql_crud::StdPrimitiveI64,
    pub offset: postgresql_crud::StdPrimitiveI64,
}
#[derive(Debug)]
pub struct ReadManyParameters {
    pub payload: ReadManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<JsongenericOptions>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        not_unique_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        not_unique_column: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        not_unique_std_primitive_bool_as_postgresql_bool_not_null: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueSqlxTypesJsonTAsPostgresqlJsonBNotNull {
        not_unique_sqlx_types_json_t_as_postgresql_json_b_not_null: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    EmptyColumnJsonReader {
        empty_column_json_reader: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumnJsonReader {
        not_unique_column_json_reader: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyRouteLogicErrorNamed> for TryReadManyRouteLogicResponseVariants {
    fn from(value: TryReadManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence } => Self::BindQuery { bind_query, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => Self::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence } => Self::NotUniqueColumn { not_unique_column, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                not_unique_std_primitive_bool_as_postgresql_bool_not_null,
                code_occurence,
            } => Self::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                not_unique_std_primitive_bool_as_postgresql_bool_not_null,
                code_occurence,
            },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueSqlxTypesJsonTAsPostgresqlJsonBNotNull {
                not_unique_sqlx_types_json_t_as_postgresql_json_b_not_null,
                code_occurence,
            } => Self::NotUniqueSqlxTypesJsonTAsPostgresqlJsonBNotNull {
                not_unique_sqlx_types_json_t_as_postgresql_json_b_not_null,
                code_occurence,
            },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::EmptyColumnJsonReader { empty_column_json_reader, code_occurence } => Self::EmptyColumnJsonReader { empty_column_json_reader, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumnJsonReader { not_unique_column_json_reader, code_occurence } => Self::NotUniqueColumnJsonReader { not_unique_column_json_reader, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: postgresql_crud::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::StdPrimitiveI64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueSqlxTypesJsonTAsPostgresqlJsonBNotNull {
        #[eo_to_std_string_string]
        not_unique_sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::SqlxTypesJson<Something>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    EmptyColumnJsonReader {
        #[eo_to_std_string_string_serialize_deserialize]
        empty_column_json_reader: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumnJsonReader {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column_json_reader: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2144,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = ReadManyParameters {
        payload: match serde_json::from_slice::<ReadManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = ReadManyPayload::from(value);
                if let Some(value) = &value.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if !acc.contains(&element) {
                            acc.push(&element);
                        } else {
                            let error_0 = *element;
                            let error = TryReadManyRouteLogicErrorNamed::NotUniquePrimaryKey {
                                not_unique_primary_key: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2444,
                                        column: 173,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                if let Some(value) = &value.std_primitive_bool_as_postgresql_bool_not_null {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if !acc.contains(&&element.value.0) {
                            acc.push(&element.value.0);
                        } else {
                            let error_0 = element.value.clone();
                            let error = TryReadManyRouteLogicErrorNamed::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                                not_unique_std_primitive_bool_as_postgresql_bool_not_null: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2481,
                                        column: 274,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                if let Some(value) = &value.sqlx_types_json_t_as_postgresql_json_b_not_null {
                    let mut acc = std::vec::Vec::new();
                    for element in value {
                        if !acc.contains(&&element.value.0) {
                            acc.push(&element.value.0);
                        } else {
                            let error_0 = element.value.clone();
                            let error = TryReadManyRouteLogicErrorNamed::NotUniqueSqlxTypesJsonTAsPostgresqlJsonBNotNull {
                                not_unique_sqlx_types_json_t_as_postgresql_json_b_not_null: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2481,
                                        column: 274,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                let mut acc = std::vec::Vec::new();
                for element in &value.select {
                    if acc.contains(&element) {
                        let error_0 = element.clone();
                        let error = TryReadManyRouteLogicErrorNamed::NotUniqueColumn {
                            not_unique_column: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 1654,
                                    column: 264,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    } else {
                        match &element {
                            JsongenericColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull => (),
                            JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => (),
                            JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull { filter } => {
                                if filter.is_empty() {
                                    let error_0 = element.clone();
                                    let error = TryReadManyRouteLogicErrorNamed::EmptyColumnJsonReader {
                                        empty_column_json_reader: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1661,
                                                column: 286,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                {
                                    let mut acc = vec![];
                                    for element_handle in filter {
                                        match acc.contains(&element_handle) {
                                            true => {
                                                let error_0 = element.clone();
                                                let error = TryReadManyRouteLogicErrorNamed::NotUniqueColumnJsonReader {
                                                    not_unique_column_json_reader: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                            line: 1663,
                                                            column: 191,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                                return response;
                                            }
                                            false => {
                                                acc.push(element_handle);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        acc.push(element);
                    }
                }
                value
            }
            Err(error_0) => {
                let error = TryReadManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2216,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = {
        format!(
            "select {} from jsongeneric {}",
            {
                let mut value = std::string::String::default();
                for element in &parameters.payload.select {
                    value.push_str(&match element {
                        JsongenericColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull => "std_primitive_bool_as_postgresql_bool_not_null".to_string(),
                        JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key".to_string(),
                        JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull { filter } => format!(
                            "{} as sqlx_types_json_t_as_postgresql_json_b_not_null",
                            postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_vec(filter, "sqlx_types_json_t_as_postgresql_json_b_not_null", "sqlx_types_json_t_as_postgresql_json_b_not_null",)
                        ),
                    });
                    value.push_str(",");
                }
                let _ = value.pop();
                value
            },
            {
                let mut increment: std::primitive::u64 = 0;
                let mut additional_parameters = std::string::String::default();
                if let Some(_) = &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
                    let prefix = match additional_parameters.is_empty() {
                        true => "where",
                        false => " and",
                    };
                    match increment.checked_add(1) {
                        Some(value) => {
                            increment = value;
                        }
                        None => {
                            let error = TryReadManyRouteLogicErrorNamed::CheckedAdd {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 3118,
                                        column: 268,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                    additional_parameters.push_str(&format!("{} std_primitive_i64_as_postgresql_big_serial_not_null_primary_key in (select unnest(${}))", prefix, increment));
                }
                if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
                    additional_parameters.push_str(&format!(
                        "{} {}",
                        match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        },
                        {
                            let mut acc = std::string::String::default();
                            for (index, element) in value.iter().enumerate() {
                                match postgresql_crud::BindQuery::try_generate_bind_increments(element, &mut increment) {
                                    Ok(value) => {
                                        let handle = format!("std_primitive_bool_as_postgresql_bool_not_null ~ {value} ");
                                        match index == 0 {
                                            true => {
                                                acc.push_str(&handle);
                                            }
                                            false => {
                                                acc.push_str(&format!("{} {handle}", element.conjunctive_operator));
                                            }
                                        }
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                            bind_query: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line: 3144,
                                                    column: 266,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            }
                            if let false = acc.is_empty() {
                                let _ = acc.pop();
                            }
                            acc
                        }
                    ));
                }
                if let Some(value) = &parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null {
                    additional_parameters.push_str(&format!(
                        "{} {}",
                        match additional_parameters.is_empty() {
                            true => "where",
                            false => " and",
                        },
                        {
                            let mut acc = std::string::String::default();
                            for (index, element) in value.iter().enumerate() {
                                match postgresql_crud::BindQuery::try_generate_bind_increments(element, &mut increment) {
                                    Ok(value) => {
                                        let handle = format!("sqlx_types_json_t_as_postgresql_json_b_not_null ~ {value} ");
                                        match index == 0 {
                                            true => {
                                                acc.push_str(&handle);
                                            }
                                            false => {
                                                acc.push_str(&format!("{} {handle}", element.conjunctive_operator));
                                            }
                                        }
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                            bind_query: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line: 3144,
                                                    column: 266,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            }
                            if let false = acc.is_empty() {
                                let _ = acc.pop();
                            }
                            acc
                        }
                    ));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = &parameters.payload.order_by;
                    let order = match &value.order {
                        Some(value) => value.to_string(),
                        None => postgresql_crud::Order::default().to_string(),
                    };
                    additional_parameters.push_str(&format!("{}order by {} {}", prefix, value.column, order));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = match postgresql_crud::BindQuery::try_generate_bind_increments(&parameters.payload.limit, &mut increment) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                bind_query: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 3193,
                                        column: 168,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    };
                    additional_parameters.push_str(&format!("{}limit {}", prefix, value));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = match postgresql_crud::BindQuery::try_generate_bind_increments(&parameters.payload.offset, &mut increment) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::BindQuery {
                                bind_query: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 3195,
                                        column: 168,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    };
                    additional_parameters.push_str(&format!("{}offset {}", prefix, value));
                }
                additional_parameters
            }
        )
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
            query = query.bind(value.into_iter().map(|element| element.into_inner().clone()).collect::<std::vec::Vec<std::primitive::i64>>());
        }
        if let Some(value) = parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
            for value in value {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
        }
        if let Some(value) = parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null {
            for value in value {
                query = postgresql_crud::BindQuery::bind_value_to_query(value, query);
            }
        }
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.limit, query);
        query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.offset, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2166,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2166,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let value = {
            let mut rows = binded_query.fetch(executor.as_mut());
            let mut acc = std::vec::Vec::new();
            while let Some(value) = match {
                use postgresql_crud::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => match value {
                    Some(value) => Some({
                        let mut std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<postgresql_crud::Value<postgresql_crud::StdPrimitiveI64>> = None;
                        let mut std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::StdPrimitiveBool>> = None;
                        let mut sqlx_types_json_t_as_postgresql_json_b_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::SqlxTypesJson<SomethingReader>>> = None;
                        for element in &parameters.payload.select {
                            match element {
                                JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => match sqlx::Row::try_get::<std::primitive::i64, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
                                    Ok(value) => {
                                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = Some(postgresql_crud::Value { value: postgresql_crud::StdPrimitiveI64(value) });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line: 1099,
                                                    column: 246,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                },
                                JsongenericColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull => match sqlx::Row::try_get::<std::primitive::bool, &std::primitive::str>(&value, "std_primitive_bool_as_postgresql_bool_not_null") {
                                    Ok(value) => {
                                        std_primitive_bool_as_postgresql_bool_not_null = Some(postgresql_crud::Value { value: postgresql_crud::StdPrimitiveBool(value) });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line: 1134,
                                                    column: 250,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                },
                                JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull { filter } => match sqlx::Row::try_get::<sqlx::types::Json<SomethingReader>, &std::primitive::str>(&value, "sqlx_types_json_t_as_postgresql_json_b_not_null") {
                                    Ok(value) => {
                                        sqlx_types_json_t_as_postgresql_json_b_not_null = Some(postgresql_crud::Value { value: postgresql_crud::SqlxTypesJson(value) });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line: 1134,
                                                    column: 250,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                },
                            }
                        }
                        JsongenericOptions {
                            std_primitive_bool_as_postgresql_bool_not_null,
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                            sqlx_types_json_t_as_postgresql_json_b_not_null,
                        }
                    }),
                    None => None,
                },
                Err(error_0) => {
                    let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3311,
                                column: 169,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            } {
                acc.push(value);
            }
            acc
        };
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::StdPrimitiveI64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
        #[eo_to_std_string_string]
        not_unique_std_primitive_bool_as_postgresql_bool_not_null: postgresql_crud::StdPrimitiveBool,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueSqlxTypesJsonTAsPostgresqlJsonBNotNull {
        #[eo_to_std_string_string]
        not_unique_sqlx_types_json_t_as_postgresql_json_b_not_null: postgresql_crud::SqlxTypesJson<Something>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_many_route_logic_error_named_with_serialize_deserialize: TryReadManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many(server_location: &std::primitive::str, parameters: ReadManyParameters) -> Result<std::vec::Vec<JsongenericOptions>, TryReadManyErrorNamed> {
    let payload = {
        if let Some(value) = &parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if !acc.contains(&element) {
                    acc.push(&element);
                } else {
                    let error_0 = *element;
                    return Err(TryReadManyErrorNamed::NotUniquePrimaryKey {
                        not_unique_primary_key: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3362,
                                column: 202,
                            }),
                        ),
                    });
                }
            }
        }
        if let Some(value) = &parameters.payload.std_primitive_bool_as_postgresql_bool_not_null {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if !acc.contains(&&element.value.0) {
                    acc.push(&element.value.0);
                } else {
                    let error_0 = element.value.clone();
                    return Err(TryReadManyErrorNamed::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
                        not_unique_std_primitive_bool_as_postgresql_bool_not_null: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3380,
                                column: 37,
                            }),
                        ),
                    });
                }
            }
        }
        if let Some(value) = &parameters.payload.sqlx_types_json_t_as_postgresql_json_b_not_null {
            let mut acc = std::vec::Vec::new();
            for element in value {
                if !acc.contains(&&element.value.0) {
                    acc.push(&element.value.0);
                } else {
                    let error_0 = element.value.clone();
                    return Err(TryReadManyErrorNamed::NotUniqueSqlxTypesJsonTAsPostgresqlJsonBNotNull {
                        not_unique_sqlx_types_json_t_as_postgresql_json_b_not_null: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3380,
                                column: 37,
                            }),
                        ),
                    });
                }
            }
        }
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.select {
            if acc.contains(&element) {
                let error_0 = element.clone();
                return Err(TryReadManyErrorNamed::NotUniqueColumn {
                    not_unique_column: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 1717,
                            column: 176,
                        }),
                    ),
                });
            } else {
                acc.push(element);
            }
        }
        let value = ReadManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryReadManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2266,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/jsongeneric/read_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryReadManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2318,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryReadManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2335,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryReadManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryReadManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2348,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_read_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryReadManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value.into_iter().map(|element| JsongenericOptions::from(element)).collect();
            return Ok(value);
        }
        TryReadManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryReadManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryReadManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryReadManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryReadManyRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
        TryReadManyRouteLogicResponseVariants::BindQuery { bind_query, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::BindQuery { bind_query, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniqueColumn { not_unique_column, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
            not_unique_std_primitive_bool_as_postgresql_bool_not_null,
            code_occurence,
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueStdPrimitiveBoolAsPostgresqlBoolNotNull {
            not_unique_std_primitive_bool_as_postgresql_bool_not_null,
            code_occurence,
        },
        TryReadManyRouteLogicResponseVariants::NotUniqueSqlxTypesJsonTAsPostgresqlJsonBNotNull {
            not_unique_sqlx_types_json_t_as_postgresql_json_b_not_null,
            code_occurence,
        } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueSqlxTypesJsonTAsPostgresqlJsonBNotNull {
            not_unique_sqlx_types_json_t_as_postgresql_json_b_not_null,
            code_occurence,
        },
        TryReadManyRouteLogicResponseVariants::EmptyColumnJsonReader { empty_column_json_reader, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::EmptyColumnJsonReader { empty_column_json_reader, code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniqueColumnJsonReader { not_unique_column_json_reader, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumnJsonReader { not_unique_column_json_reader, code_occurence },
    };
    Err(TryReadManyErrorNamed::TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_read_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2386,
                column: 223,
            }),
        ),
    })
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadOnePayload {
    pub std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdPrimitiveI64,
    pub select: std::vec::Vec<JsongenericColumn>,
}
#[derive(Debug)]
pub struct ReadOneParameters {
    pub payload: ReadOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadOneRouteLogicResponseVariants {
    Desirable(JsongenericOptions),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        not_unique_column: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    EmptyColumnJsonReader {
        empty_column_json_reader: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumnJsonReader {
        not_unique_column_json_reader: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneRouteLogicErrorNamed> for TryReadOneRouteLogicResponseVariants {
    fn from(value: TryReadOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence } => Self::NotUniqueColumn { not_unique_column, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::EmptyColumnJsonReader { empty_column_json_reader, code_occurence } => Self::EmptyColumnJsonReader { empty_column_json_reader, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumnJsonReader { not_unique_column_json_reader, code_occurence } => Self::NotUniqueColumnJsonReader { not_unique_column_json_reader, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneRouteLogicErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        #[eo_to_std_string_string]
        postgresql: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    EmptyColumnJsonReader {
        #[eo_to_std_string_string_serialize_deserialize]
        empty_column_json_reader: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumnJsonReader {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column_json_reader: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2144,
                        column: 265,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = ReadOneParameters {
        payload: match serde_json::from_slice::<ReadOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = ReadOnePayload::from(value);
                let mut acc = std::vec::Vec::new();
                for element in &value.select {
                    if acc.contains(&element) {
                        let error_0 = element.clone();
                        let error = TryReadOneRouteLogicErrorNamed::NotUniqueColumn {
                            not_unique_column: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 1654,
                                    column: 264,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    } else {
                        match &element {
                            JsongenericColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull => (),
                            JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => (),
                            JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull { filter } => {
                                if filter.is_empty() {
                                    let error_0 = element.clone();
                                    let error = TryReadOneRouteLogicErrorNamed::EmptyColumnJsonReader {
                                        empty_column_json_reader: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1661,
                                                column: 286,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                {
                                    let mut acc = vec![];
                                    for element_handle in filter {
                                        match acc.contains(&element_handle) {
                                            true => {
                                                let error_0 = element.clone();
                                                let error = TryReadOneRouteLogicErrorNamed::NotUniqueColumnJsonReader {
                                                    not_unique_column_json_reader: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                            line: 1663,
                                                            column: 191,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                                return response;
                                            }
                                            false => {
                                                acc.push(element_handle);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        acc.push(element);
                    }
                }
                value
            }
            Err(error_0) => {
                let error = TryReadOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2216,
                            column: 250,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    println!("{:#?}", parameters);
    let query_string = format!("select {} from jsongeneric where std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = $1", {
        let mut value = std::string::String::default();
        for element in &parameters.payload.select {
            value.push_str(&match element {
                JsongenericColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull => "std_primitive_bool_as_postgresql_bool_not_null".to_string(),
                JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key".to_string(),
                JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull { filter } => format!(
                    "{} as sqlx_types_json_t_as_postgresql_json_b_not_null",
                    postgresql_crud::GeneratePostgresqlQueryPartToRead::generate_postgresql_query_part_to_read_from_vec(filter, "sqlx_types_json_t_as_postgresql_json_b_not_null", "sqlx_types_json_t_as_postgresql_json_b_not_null",)
                ),
            });
            value.push_str(",");
        }
        let _ = value.pop();
        value
    },);
    println!("{}", query_string);
    let binded_query = {
        let query = sqlx::query::<sqlx::Postgres>(&query_string);
        let query = postgresql_crud::BindQuery::bind_value_to_query(parameters.payload.std_primitive_i64_as_postgresql_big_serial_not_null_primary_key, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2166,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2166,
                        column: 254,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => {
                    let mut std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: std::option::Option<postgresql_crud::Value<postgresql_crud::StdPrimitiveI64>> = None;
                    let mut std_primitive_bool_as_postgresql_bool_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::StdPrimitiveBool>> = None;
                    let mut sqlx_types_json_t_as_postgresql_json_b_not_null: std::option::Option<postgresql_crud::Value<postgresql_crud::SqlxTypesJson<SomethingReader>>> = None;
                    for element in &parameters.payload.select {
                        match element {
                            JsongenericColumn::StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey => match sqlx::Row::try_get::<std::primitive::i64, &std::primitive::str>(&value, "std_primitive_i64_as_postgresql_big_serial_not_null_primary_key") {
                                Ok(value) => {
                                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = Some(postgresql_crud::Value { value: postgresql_crud::StdPrimitiveI64(value) });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1099,
                                                column: 246,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                            JsongenericColumn::StdPrimitiveBoolAsPostgresqlBoolNotNull => match sqlx::Row::try_get::<std::primitive::bool, &std::primitive::str>(&value, "std_primitive_bool_as_postgresql_bool_not_null") {
                                Ok(value) => {
                                    std_primitive_bool_as_postgresql_bool_not_null = Some(postgresql_crud::Value { value: postgresql_crud::StdPrimitiveBool(value) });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1134,
                                                column: 250,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                            JsongenericColumn::SqlxTypesJsonTAsPostgresqlJsonBNotNull { filter } => match sqlx::Row::try_get::<sqlx::types::Json<SomethingReader>, &std::primitive::str>(&value, "sqlx_types_json_t_as_postgresql_json_b_not_null") {
                                Ok(value) => {
                                    sqlx_types_json_t_as_postgresql_json_b_not_null = Some(postgresql_crud::Value { value: postgresql_crud::SqlxTypesJson(value) });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1134,
                                                column: 250,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                        }
                    }
                    JsongenericOptions {
                        std_primitive_bool_as_postgresql_bool_not_null,
                        std_primitive_i64_as_postgresql_big_serial_not_null_primary_key,
                        sqlx_types_json_t_as_postgresql_json_b_not_null,
                    }
                }
                Err(error_0) => {
                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3578,
                                column: 169,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        };
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: http::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string_serialize_deserialize]
        response_text: std::string::String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: JsongenericColumn,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_one_route_logic_error_named_with_serialize_deserialize: TryReadOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one(server_location: &std::primitive::str, parameters: ReadOneParameters) -> Result<JsongenericOptions, TryReadOneErrorNamed> {
    let payload = {
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.select {
            if acc.contains(&element) {
                let error_0 = element.clone();
                return Err(TryReadOneErrorNamed::NotUniqueColumn {
                    not_unique_column: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 1717,
                            column: 176,
                        }),
                    ),
                });
            } else {
                acc.push(element);
            }
        }
        let value = ReadOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryReadOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2266,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/jsongeneric/read_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryReadOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2318,
                        column: 152,
                    }),
                ),
            });
        }
    };
    let error_0 = response.status();
    let error_1 = response.headers().clone();
    let error_2 = match response.text().await {
        Ok(value) => value,
        Err(error_2) => {
            return Err(TryReadOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2335,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryReadOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryReadOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2348,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_read_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryReadOneRouteLogicResponseVariants::Desirable(value) => {
            let value = JsongenericOptions::from(value);
            return Ok(value);
        }
        TryReadOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryReadOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryReadOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryReadOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryReadOneRouteLogicResponseVariants::NotUniqueColumn { not_unique_column, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence },
        TryReadOneRouteLogicResponseVariants::EmptyColumnJsonReader { empty_column_json_reader, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::EmptyColumnJsonReader { empty_column_json_reader, code_occurence },
        TryReadOneRouteLogicResponseVariants::NotUniqueColumnJsonReader { not_unique_column_json_reader, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumnJsonReader { not_unique_column_json_reader, code_occurence },
    };
    Err(TryReadOneErrorNamed::TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_read_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2386,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for ReadOnePayload {
    fn default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            select: postgresql_crud::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <ReadOnePayload as postgresql_crud::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement>::default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
