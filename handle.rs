impl TableExample {
    pub const fn table_name() -> &'static str {
        "table_example"
    }
    const fn primary_key() -> &'static str {
        "primary_key_column"
    }
    pub async fn prepare_extensions(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), TableExamplePreparePostgresqlErrorNamed> {
        if let Err(error) = sqlx::query("create extension if not exists pg_jsonschema").execute(pool).await {
            return Err(TableExamplePreparePostgresqlErrorNamed::CreateExtensionIfNotExistsPgJsonschema { error, code_occurence: error_occurence_lib::code_occurence!() });
        }
        if let Err(error) = sqlx::query("create extension if not exists \"uuid-ossp\"").execute(pool).await {
            return Err(TableExamplePreparePostgresqlErrorNamed::CreateExtensionIfNotExistsUuidOssp { error, code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(())
    }
    pub async fn prepare_postgresql_table(pool: &sqlx::Pool<sqlx::Postgres>, table: &str) -> Result<(), TableExamplePreparePostgresqlErrorNamed> {
        if let Err(error) = sqlx::query(&format!(
            "create table if not exists {table} ({},{},{})",
            <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::create_table_column_query_part(&"primary_key_column", true),
            <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::create_table_column_query_part(&"column_0", false),
            <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::create_table_column_query_part(&"column_142", false)
        ))
        .execute(pool)
        .await
        {
            return Err(TableExamplePreparePostgresqlErrorNamed::PreparePostgresql { error, code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(())
    }
    pub async fn prepare_postgresql(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), TableExamplePreparePostgresqlErrorNamed> {
        Self::prepare_extensions(pool).await?;
        Self::prepare_postgresql_table(pool, "table_example").await?;
        Ok(())
    }
    pub const fn allow_methods() -> [http::Method; 4] {
        [http::Method::GET, http::Method::POST, http::Method::PATCH, http::Method::DELETE]
    }
    fn generate_select_query_part(select: &postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut value = String::default();
        for element in select.to_vec() {
            value.push_str(&match element {
                TableExampleSelect::PrimaryKeyColumn(value) => match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_query_part(value, "primary_key_column") {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(error);
                    }
                },
                TableExampleSelect::Column0(value) => match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_query_part(value, "column_0") {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(error);
                    }
                },
                TableExampleSelect::Column142(value) => match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::select_query_part(value, "column_142") {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(error);
                    }
                },
            });
            value.push(',');
        }
        let _: Option<char> = value.pop();
        Ok(value)
    }
    async fn create_many_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleCreateManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleCreateManyParameters {
            payload: match serde_json::from_slice::<TableExampleCreateManyPayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_create_many_query_string(
            table,
            "primary_key_column,column_0,column_142",
            &{
                let mut increment: u64 = 0;
                let mut acc = String::default();
                for element in &parameters.payload.0 {
                    match element.create_query_part(&mut increment) {
                        Ok(value) => {
                            use std::fmt::Write as _;
                            if write!(acc, "({value}),").is_err() {
                                let error_0 = postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() };
                                let error = TableExampleCreateManyErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2484,
                                            column: 241,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                        Err(error_0) => {
                            let error = TableExampleCreateManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2523,
                                        column: 254,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            },
            &{
                let mut acc = String::new();
                match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_ids_query_part("primary_key_column") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_0") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_142") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            for element in parameters.payload.0 {
                match element.create_query_bind(query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2548,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateManyErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = Vec::new();
                while let Some(value) = match postgresql_crud::TryStreamExt::try_next(&mut rows).await {
                    Ok(value) => match value {
                        Some(value) => match TableExampleReadOnlyIds::try_from(value) {
                            Ok(value) => Some(value),
                            Err(error_0) => {
                                drop(rows);
                                {
                                    if let Err(error_1) = executor.rollback().await {
                                        let error = TableExampleCreateManyErrorNamed::RowAndRollback {
                                            row: error_0,
                                            rollback: error_1,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                    line: 2579,
                                                    column: 259,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                    let error = TableExampleCreateManyErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                line: 2579,
                                                column: 230,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        },
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleCreateManyErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2589,
                                            column: 178,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleCreateManyErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2589,
                                        column: 149,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleCreateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::CREATED;
        response
    }
    pub async fn create_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::create_many_handle(app_state, request, Self::table_name()).await
    }
    async fn try_create_many_handle(endpoint_location: &str, parameters: TableExampleCreateManyParameters, table: &str) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryCreateManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryCreateManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/create_many");
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryCreateManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
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
                return Err(TableExampleTryCreateManyErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleCreateManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryCreateManyErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let create_many_error_named_with_serialize_deserialize = match expected_response {
            TableExampleCreateManyResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleCreateManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleCreateManyResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleCreateManyResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleCreateManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleCreateManyResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleCreateManyResponseVariants::QueryPart { error, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleCreateManyResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleCreateManyResponseVariants::TryBind { try_bind, code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryCreateManyErrorNamed::TableExampleCreateManyErrorNamedWithSerializeDeserialize {
            create_many_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_create_many(endpoint_location: &str, parameters: TableExampleCreateManyParameters) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryCreateManyErrorNamed> {
        Self::try_create_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn create_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleCreateManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn create_one_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleCreateOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleCreateOneParameters {
            payload: match serde_json::from_slice::<TableExampleCreate>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_create_one_query_string(
            table,
            "primary_key_column,column_0,column_142",
            &match parameters.payload.create_query_part(&mut 0) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2640,
                                column: 254,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &{
                let mut acc = String::new();
                match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_ids_query_part("primary_key_column") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateOneErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_0") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateOneErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::select_only_ids_query_part("column_142") {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateOneErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2461,
                                    column: 245,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match parameters.payload.create_query_bind(query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2657,
                                column: 252,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleCreateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match TableExampleReadOnlyIds::try_from(value) {
                        Ok(value) => value,
                        Err(error_0) => {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleCreateOneErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2689,
                                            column: 225,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleCreateOneErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2689,
                                        column: 196,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                    Err(error_0) => {
                        if let Err(error_1) = executor.rollback().await {
                            let error = TableExampleCreateOneErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2699,
                                        column: 161,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        let error = TableExampleCreateOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2699,
                                    column: 132,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleCreateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleCreateOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::CREATED;
        response
    }
    pub async fn create_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::create_one_handle(app_state, request, Self::table_name()).await
    }
    async fn try_create_one_handle(endpoint_location: &str, parameters: TableExampleCreateOneParameters, table: &str) -> Result<TableExampleReadOnlyIds, TableExampleTryCreateOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryCreateOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/create_one");
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryCreateOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
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
                return Err(TableExampleTryCreateOneErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleCreateOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryCreateOneErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let create_one_error_named_with_serialize_deserialize = match expected_response {
            TableExampleCreateOneResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleCreateOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleCreateOneResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleCreateOneResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleCreateOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleCreateOneResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleCreateOneResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleCreateOneResponseVariants::QueryPart { error, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleCreateOneResponseVariants::TryBind { try_bind, code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryCreateOneErrorNamed::TableExampleCreateOneErrorNamedWithSerializeDeserialize {
            create_one_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_create_one(endpoint_location: &str, parameters: TableExampleCreateOneParameters) -> Result<TableExampleReadOnlyIds, TableExampleTryCreateOneErrorNamed> {
        Self::try_create_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn create_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleCreate as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn read_many_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleReadManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleReadManyParameters {
            payload: match serde_json::from_slice::<TableExampleReadManyPayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_read_many_query_string(
            table,
            &match Self::generate_select_query_part(&parameters.payload.select) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 859,
                                column: 241,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &{
                let mut increment: u64 = 0;
                let mut additional_parameters = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.where_many, &mut increment, &"", false) {
                    Ok(value) => value,
                    Err(error_0) => {
                        let error = TableExampleReadManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 1159,
                                    column: 274,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                };
                {
                    let prefix = if additional_parameters.is_empty() { "" } else { " " };
                    let value = &parameters.payload.order_by;
                    let order = match &value.order {
                        Some(value) => value.to_snake_case_stringified(),
                        None => postgresql_crud::Order::default().to_snake_case_stringified(),
                    };
                    {
                        {
                            use std::fmt::Write as _;
                            if write!(
                                additional_parameters,
                                "{}order by {} {}",
                                prefix,
                                &match &value.column {
                                    TableExampleSelect::PrimaryKeyColumn(_) => "primary_key_column",
                                    TableExampleSelect::Column0(_) => "column_0",
                                    TableExampleSelect::Column142(_) => "column_142",
                                },
                                order,
                            )
                            .is_err()
                            {
                                let error_0 = postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() };
                                let error = TableExampleReadManyErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2484,
                                            column: 241,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                    }
                };
                {
                    let prefix = if additional_parameters.is_empty() { "" } else { " " };
                    let value = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.pagination, &mut increment, &"", bool::default()) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TableExampleReadManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2778,
                                        column: 254,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    };
                    {
                        {
                            use std::fmt::Write as _;
                            if write!(additional_parameters, "{prefix}{value}").is_err() {
                                let error_0 = postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() };
                                let error = TableExampleReadManyErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2484,
                                            column: 241,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                    }
                };
                additional_parameters
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.where_many, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1182,
                                column: 239,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.pagination, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2835,
                                column: 252,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = Vec::new();
                while let Some(value) = match postgresql_crud::TryStreamExt::try_next(&mut rows).await {
                    Ok(value) => match value {
                        Some(value) => Some(match TableExampleRead::try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_table_example_select(value, &parameters.payload.select) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = TableExampleReadManyErrorNamed::Postgresql {
                                    postgresql: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 1203,
                                            column: 271,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }),
                        None => None,
                    },
                    Err(error_0) => {
                        let error = TableExampleReadManyErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2859,
                                    column: 169,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                } {
                    acc.push(value);
                }
                acc
            }
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn read_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::read_many_handle(app_state, request, Self::table_name()).await
    }
    async fn try_read_many_handle(endpoint_location: &str, parameters: TableExampleReadManyParameters, table: &str) -> Result<Vec<TableExampleRead>, TableExampleTryReadManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryReadManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/read_many");
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryReadManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
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
                return Err(TableExampleTryReadManyErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleReadManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryReadManyErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let read_many_error_named_with_serialize_deserialize = match expected_response {
            TableExampleReadManyResponseVariants::Desirable(value) => {
                return Ok(value.into_iter().fold(Vec::new(), |mut acc, element| {
                    acc.push(element);
                    acc
                }));
            }
            TableExampleReadManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleReadManyResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleReadManyResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleReadManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleReadManyResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleReadManyResponseVariants::QueryPart { error, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleReadManyResponseVariants::NotUniqueField { not_unique_field, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence },
            TableExampleReadManyResponseVariants::TryBind { try_bind, code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryReadManyErrorNamed::TableExampleReadManyErrorNamedWithSerializeDeserialize {
            read_many_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_read_many(endpoint_location: &str, parameters: TableExampleReadManyParameters) -> Result<Vec<TableExampleRead>, TableExampleTryReadManyErrorNamed> {
        Self::try_read_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn read_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleReadManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn read_one_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleReadOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleReadOneParameters {
            payload: match serde_json::from_slice::<TableExampleReadOnePayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_read_one_query_string(
            table,
            &match Self::generate_select_query_part(&parameters.payload.select) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 859,
                                column: 241,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.primary_key_column, &mut 0, &Self::primary_key(), false) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2945,
                                column: 254,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.primary_key_column, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2964,
                                column: 256,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleReadOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match TableExampleRead::try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_table_example_select(value, &parameters.payload.select) {
                    Ok(value) => value,
                    Err(error_0) => {
                        let error = TableExampleReadOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 1203,
                                    column: 271,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2984,
                                column: 165,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleReadOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn read_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::read_one_handle(app_state, request, Self::table_name()).await
    }
    async fn try_read_one_handle(endpoint_location: &str, parameters: TableExampleReadOneParameters, table: &str) -> Result<TableExampleRead, TableExampleTryReadOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryReadOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/read_one");
        let future = reqwest::Client::new().post(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryReadOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
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
                return Err(TableExampleTryReadOneErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleReadOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryReadOneErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let read_one_error_named_with_serialize_deserialize = match expected_response {
            TableExampleReadOneResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleReadOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleReadOneResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleReadOneResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleReadOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleReadOneResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleReadOneResponseVariants::NotUniqueField { not_unique_field, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence },
            TableExampleReadOneResponseVariants::QueryPart { error, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleReadOneResponseVariants::TryBind { try_bind, code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize {
            read_one_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_read_one(endpoint_location: &str, parameters: TableExampleReadOneParameters) -> Result<TableExampleRead, TableExampleTryReadOneErrorNamed> {
        Self::try_read_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn read_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleReadOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn update_many_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleUpdateManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleUpdateManyParameters {
            payload: match serde_json::from_slice::<TableExampleUpdateManyPayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let update_for_query_vec = parameters.payload.0.into_iter().map(|element| TableExampleUpdateForQuery::from(element)).collect::<Vec<TableExampleUpdateForQuery>>();
        let query_string = {
            let mut increment: u64 = 0;
            let elements = {
                let mut acc = String::default();
                {
                    let mut is_column_0_update_exist = false;
                    for element in &update_for_query_vec {
                        if element.column_0.is_some() {
                            is_column_0_update_exist = true;
                            break;
                        }
                    }
                    if is_column_0_update_exist {
                        acc.push_str(&postgresql_crud::generate_column_equals_case_acc_else_column_end_comma_update_many_query_part("column_0", &{
                            let mut acc = String::default();
                            for element in &update_for_query_vec {
                                if let Some(value) = &element.column_0 {
                                    acc.push_str(&postgresql_crud::generate_when_column_id_then_value_update_many_query_part(
                                        Self::primary_key(),
                                        &match element.update_query_part_primary_key(&mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                                    error: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                            line: 3181,
                                                            column: 254,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                        &match TableExampleUpdateForQuery::update_query_part_column_0(value, &mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                                    error: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                            line: 3181,
                                                            column: 254,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                    ));
                                }
                            }
                            acc
                        }));
                    }
                }
                {
                    let mut is_column_142_update_exist = false;
                    for element in &update_for_query_vec {
                        if element.column_142.is_some() {
                            is_column_142_update_exist = true;
                            break;
                        }
                    }
                    if is_column_142_update_exist {
                        acc.push_str(&postgresql_crud::generate_column_equals_case_acc_else_column_end_comma_update_many_query_part("column_142", &{
                            let mut acc = String::default();
                            for element in &update_for_query_vec {
                                if let Some(value) = &element.column_142 {
                                    acc.push_str(&postgresql_crud::generate_when_column_id_then_value_update_many_query_part(
                                        Self::primary_key(),
                                        &match element.update_query_part_primary_key(&mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                                    error: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                            line: 3181,
                                                            column: 254,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                        &match TableExampleUpdateForQuery::update_query_part_column_142(value, &mut increment) {
                                            Ok(value) => value,
                                            Err(error_0) => {
                                                let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                                    error: error_0,
                                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                        file!().to_owned(),
                                                        line!(),
                                                        column!(),
                                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                            line: 3181,
                                                            column: 254,
                                                        }),
                                                    ),
                                                };
                                                eprintln!("{error}");
                                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                                return response;
                                            }
                                        },
                                    ));
                                }
                            }
                            acc
                        }));
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            };
            let primary_keys = {
                let mut acc = String::default();
                for element in &update_for_query_vec {
                    {
                        use std::fmt::Write as _;
                        if write!(
                            acc,
                            "{},",
                            match element.update_query_part_primary_key(&mut increment) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                        error: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                line: 1722,
                                                column: 241,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                    return response;
                                }
                            }
                        )
                        .is_err()
                        {
                            let error_0 = postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() };
                            let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2484,
                                        column: 241,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                let _: Option<char> = acc.pop();
                acc
            };
            let return_columns = {
                let mut acc = String::new();
                for element in &update_for_query_vec {
                    match element.select_only_updated_ids_query_part(&mut increment) {
                        Ok(value) => {
                            acc.push_str(&value);
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3181,
                                        column: 254,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                acc
            };
            postgresql_crud::generate_update_many_query_string(table, &elements, Self::primary_key(), &primary_keys, &return_columns)
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_0 {
                    if let Err(error_0) = query.try_bind(element.primary_key_column.clone()) {
                        let error_0 = error_0.to_string();
                        let error = TableExampleUpdateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3276,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::TryBind {
                                try_bind: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3276,
                                        column: 252,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_142 {
                    if let Err(error_0) = query.try_bind(element.primary_key_column.clone()) {
                        let error_0 = error_0.to_string();
                        let error = TableExampleUpdateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3276,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::TryBind {
                                try_bind: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3276,
                                        column: 252,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            for element in &update_for_query_vec {
                match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_bind(element.primary_key_column.clone(), query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3276,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_0 {
                    match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_bind(&value.value, query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::TryBind {
                                try_bind: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3276,
                                        column: 252,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            for element in &update_for_query_vec {
                if let Some(value) = &element.column_142 {
                    match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_bind(&value.value, query) {
                        Ok(value) => {
                            query = value;
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::TryBind {
                                try_bind: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3276,
                                        column: 252,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateManyErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = Vec::new();
                while let Some(value) = match postgresql_crud::TryStreamExt::try_next(&mut rows).await {
                    Ok(value) => match value {
                        Some(value) => match TableExampleReadOnlyIds::try_from(value) {
                            Ok(value) => Some(value),
                            Err(error_0) => {
                                drop(rows);
                                {
                                    if let Err(error_1) = executor.rollback().await {
                                        let error = TableExampleUpdateManyErrorNamed::RowAndRollback {
                                            row: error_0,
                                            rollback: error_1,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                    line: 3375,
                                                    column: 259,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                    let error = TableExampleUpdateManyErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                line: 3375,
                                                column: 230,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        },
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleUpdateManyErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 3385,
                                            column: 178,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleUpdateManyErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3385,
                                        column: 149,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleUpdateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn update_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::update_many_handle(app_state, request, Self::table_name()).await
    }
    async fn try_update_many_handle(endpoint_location: &str, parameters: TableExampleUpdateManyParameters, table: &str) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryUpdateManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryUpdateManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/update_many");
        let future = reqwest::Client::new().patch(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryUpdateManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
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
                return Err(TableExampleTryUpdateManyErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleUpdateManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryUpdateManyErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let update_many_error_named_with_serialize_deserialize = match expected_response {
            TableExampleUpdateManyResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleUpdateManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleUpdateManyResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleUpdateManyResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleUpdateManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleUpdateManyResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleUpdateManyResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleUpdateManyResponseVariants::QueryPart { error, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleUpdateManyResponseVariants::TryBind { try_bind, code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryUpdateManyErrorNamed::TableExampleUpdateManyErrorNamedWithSerializeDeserialize {
            update_many_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_update_many(endpoint_location: &str, parameters: TableExampleUpdateManyParameters) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryUpdateManyErrorNamed> {
        Self::try_update_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn update_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleUpdateManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn update_one_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleUpdateOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleUpdateOneParameters {
            payload: match serde_json::from_slice::<TableExampleUpdate>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let update_for_query = TableExampleUpdateForQuery::from(parameters.payload);
        let query_string = {
            let mut increment: u64 = 0;
            let columns = {
                let mut acc = String::default();
                if let Some(value) = &update_for_query.column_0 {
                    acc.push_str(&postgresql_crud::generate_column_queals_value_comma_update_one_query_part(
                        "column_0",
                        &match TableExampleUpdateForQuery::update_query_part_column_0(value, &mut increment) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = TableExampleUpdateOneErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 3446,
                                            column: 258,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        },
                    ));
                }
                if let Some(value) = &update_for_query.column_142 {
                    acc.push_str(&postgresql_crud::generate_column_queals_value_comma_update_one_query_part(
                        "column_142",
                        &match TableExampleUpdateForQuery::update_query_part_column_142(value, &mut increment) {
                            Ok(value) => value,
                            Err(error_0) => {
                                let error = TableExampleUpdateOneErrorNamed::QueryPart {
                                    error: error_0,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 3446,
                                            column: 258,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        },
                    ));
                }
                let _: Option<char> = acc.pop();
                acc
            };
            let primary_key_query_part = match update_for_query.update_query_part_primary_key(&mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1722,
                                column: 241,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            };
            let return_columns = match update_for_query.select_only_updated_ids_query_part(&mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 3466,
                                column: 254,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            };
            postgresql_crud::generate_update_one_query_string(table, &columns, Self::primary_key(), &primary_key_query_part, &return_columns)
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value) = &update_for_query.column_0 {
                match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateOneErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3496,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            if let Some(value) = &update_for_query.column_142 {
                match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateOneErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3496,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_bind(update_for_query.primary_key_column, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 3496,
                                column: 252,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            if let Some(value) = &update_for_query.column_0 {
                match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_bind(&value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateOneErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3496,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            if let Some(value) = &update_for_query.column_142 {
                match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_bind(&value.value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        let error = TableExampleUpdateOneErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3496,
                                    column: 252,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleUpdateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match TableExampleReadOnlyIds::try_from(value) {
                        Ok(value) => value,
                        Err(error_0) => {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleUpdateOneErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 3568,
                                            column: 225,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleUpdateOneErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3568,
                                        column: 196,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                    Err(error_0) => {
                        if let Err(error_1) = executor.rollback().await {
                            let error = TableExampleUpdateOneErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 3576,
                                        column: 161,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        let error = TableExampleUpdateOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 3576,
                                    column: 132,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleUpdateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleUpdateOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn update_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::update_one_handle(app_state, request, Self::table_name()).await
    }
    async fn try_update_one_handle(endpoint_location: &str, parameters: TableExampleUpdateOneParameters, table: &str) -> Result<TableExampleReadOnlyIds, TableExampleTryUpdateOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryUpdateOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/update_one");
        let future = reqwest::Client::new().patch(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryUpdateOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
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
                return Err(TableExampleTryUpdateOneErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleUpdateOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryUpdateOneErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let update_one_error_named_with_serialize_deserialize = match expected_response {
            TableExampleUpdateOneResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleUpdateOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleUpdateOneResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleUpdateOneResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleUpdateOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleUpdateOneResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleUpdateOneResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleUpdateOneResponseVariants::QueryPart { error, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleUpdateOneResponseVariants::TryBind { try_bind, code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryUpdateOneErrorNamed::TableExampleUpdateOneErrorNamedWithSerializeDeserialize {
            update_one_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_update_one(endpoint_location: &str, parameters: TableExampleUpdateOneParameters) -> Result<TableExampleReadOnlyIds, TableExampleTryUpdateOneErrorNamed> {
        Self::try_update_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn update_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleUpdate as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn delete_many_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleDeleteManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleDeleteManyParameters {
            payload: match serde_json::from_slice::<TableExampleDeleteManyPayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleDeleteManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_delete_many_query_string(
            table,
            &{
                let mut increment: u64 = 0;
                match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.where_many, &mut increment, &"", false) {
                    Ok(value) => value,
                    Err(error_0) => {
                        let error = TableExampleDeleteManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 1159,
                                    column: 274,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            },
            Self::primary_key(),
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.where_many, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleDeleteManyErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1182,
                                column: 239,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleDeleteManyErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc = Vec::new();
                while let Some(value) = match postgresql_crud::TryStreamExt::try_next(&mut rows).await {
                    Ok(value) => match value {
                        Some(value) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, &str>(&value, Self::primary_key()) {
                            Ok(value) => Some(value),
                            Err(error_0) => {
                                drop(rows);
                                {
                                    if let Err(error_1) = executor.rollback().await {
                                        let error = TableExampleDeleteManyErrorNamed::RowAndRollback {
                                            row: error_0,
                                            rollback: error_1,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                    line: 2407,
                                                    column: 166,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                    let error = TableExampleDeleteManyErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                                line: 2407,
                                                column: 137,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            }
                        },
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleDeleteManyErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2409,
                                            column: 162,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleDeleteManyErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2409,
                                        column: 133,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                } {
                    acc.push(value);
                }
                acc
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleDeleteManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteManyResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn delete_many(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::delete_many_handle(app_state, request, Self::table_name()).await
    }
    async fn try_delete_many_handle(endpoint_location: &str, parameters: TableExampleDeleteManyParameters, table: &str) -> Result<Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>, TableExampleTryDeleteManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryDeleteManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/delete_many");
        let future = reqwest::Client::new().delete(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryDeleteManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
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
                return Err(TableExampleTryDeleteManyErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleDeleteManyResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryDeleteManyErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let delete_many_error_named_with_serialize_deserialize = match expected_response {
            TableExampleDeleteManyResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleDeleteManyResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleDeleteManyResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleDeleteManyResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleDeleteManyResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleDeleteManyResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleDeleteManyResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleDeleteManyResponseVariants::QueryPart { error, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence },
            TableExampleDeleteManyResponseVariants::TryBind { try_bind, code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryDeleteManyErrorNamed::TableExampleDeleteManyErrorNamedWithSerializeDeserialize {
            delete_many_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_delete_many(endpoint_location: &str, parameters: TableExampleDeleteManyParameters) -> Result<Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>, TableExampleTryDeleteManyErrorNamed> {
        Self::try_delete_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn delete_many_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleDeleteManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    async fn delete_one_handle(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request, table: &str) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches!
        (headers.get(axum :: http :: header :: CONTENT_TYPE), Some(value) if
        value == axum :: http :: header :: HeaderValue ::
        from_static("application/json"))
        {
            let error = TableExampleDeleteOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                        line: 2149,
                        column: 330,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2150,
                            column: 268,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleDeleteOneParameters {
            payload: match serde_json::from_slice::<TableExampleDeleteOnePayload>(&body_bytes) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleDeleteOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2231,
                                column: 249,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_delete_one_query_string(table, Self::primary_key());
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.primary_key_column, query) {
                Ok(value) => {
                    query = value;
                }
                Err(error_0) => {
                    let error = TableExampleDeleteOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 3717,
                                column: 252,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TableExampleDeleteOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2176,
                            column: 257,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor).await {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TableExampleDeleteOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 1945,
                                column: 245,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, &str>(&value, Self::primary_key()) {
                        Ok(value) => value,
                        Err(error_0) => {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleDeleteOneErrorNamed::RowAndRollback {
                                    row: error_0,
                                    rollback: error_1,
                                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                            line: 2423,
                                            column: 149,
                                        }),
                                    ),
                                };
                                eprintln!("{error}");
                                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleDeleteOneErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2423,
                                        column: 120,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                    Err(error_0) => {
                        if let Err(error_1) = executor.rollback().await {
                            let error = TableExampleDeleteOneErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                        line: 2425,
                                        column: 145,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        let error = TableExampleDeleteOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                    line: 2425,
                                    column: 116,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleDeleteOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 1956,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(TableExampleDeleteOneResponseVariants::Desirable(value)));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    pub async fn delete_one(app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request) -> axum::response::Response {
        Self::delete_one_handle(app_state, request, Self::table_name()).await
    }
    async fn try_delete_one_handle(endpoint_location: &str, parameters: TableExampleDeleteOneParameters, table: &str) -> Result<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, TableExampleTryDeleteOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value) => value,
                Err(error_0) => {
                    return Err(TableExampleTryDeleteOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                                line: 2256,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/delete_one");
        let future = reqwest::Client::new().delete(&url).header(&"commit".to_string(), git_info::PROJECT_GIT_INFO.commit).header(reqwest::header::CONTENT_TYPE, "application/json").body(payload).send();
        let response = match future.await {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TableExampleTryDeleteOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2294,
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
                return Err(TableExampleTryDeleteOneErrorNamed::FailedToGetResponseText {
                    status_code: error_0,
                    headers: error_1,
                    reqwest: error_2,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2311,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleDeleteOneResponseVariants>(&error_2) {
            Ok(value) => value,
            Err(error_3) => {
                return Err(TableExampleTryDeleteOneErrorNamed::DeserializeResponse {
                    status_code: error_0,
                    headers: error_1,
                    response_text: error_2,
                    serde: error_3,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                            line: 2323,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let delete_one_error_named_with_serialize_deserialize = match expected_response {
            TableExampleDeleteOneResponseVariants::Desirable(value) => {
                return Ok(value);
            }
            TableExampleDeleteOneResponseVariants::CheckBodySize { check_body_size, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
            TableExampleDeleteOneResponseVariants::Postgresql { postgresql, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
            TableExampleDeleteOneResponseVariants::SerdeJson { serde_json, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
            TableExampleDeleteOneResponseVariants::HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleDeleteOneResponseVariants::CheckCommit { check_commit, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
            TableExampleDeleteOneResponseVariants::RowAndRollback { row, rollback, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
            TableExampleDeleteOneResponseVariants::TryBind { try_bind, code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence },
        };
        Err(TableExampleTryDeleteOneErrorNamed::TableExampleDeleteOneErrorNamedWithSerializeDeserialize {
            delete_one_error_named_with_serialize_deserialize,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(error_occurence_lib::code_occurence::MacroOccurence {
                    file: String::from("postgresql_crud/postgresql_table/generate_postgresql_table/src/lib.rs"),
                    line: 2360,
                    column: 223,
                }),
            ),
        })
    }
    pub async fn try_delete_one(endpoint_location: &str, parameters: TableExampleDeleteOneParameters) -> Result<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, TableExampleTryDeleteOneErrorNamed> {
        Self::try_delete_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    pub async fn delete_one_payload_example() -> axum::response::Response {
        let mut response = axum::response::IntoResponse::into_response(axum::Json(<TableExampleDeleteOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
        *response.status_mut() = axum::http::StatusCode::OK;
        response
    }
    fn routes_handle(app_state: std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>, table: &str) -> axum::Router {
        axum::Router::new().nest(
            &format!("/{table}"),
            axum::Router::new()
                .route(
                    "/create_many",
                    axum::routing::post({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::create_many_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/create_many_payload_example", axum::routing::get(Self::create_many_payload_example))
                .route(
                    "/create_one",
                    axum::routing::post({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::create_one_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/create_one_payload_example", axum::routing::get(Self::create_one_payload_example))
                .route(
                    "/read_many",
                    axum::routing::post({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::read_many_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/read_many_payload_example", axum::routing::get(Self::read_many_payload_example))
                .route(
                    "/read_one",
                    axum::routing::post({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::read_one_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/read_one_payload_example", axum::routing::get(Self::read_one_payload_example))
                .route(
                    "/update_many",
                    axum::routing::patch({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::update_many_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/update_many_payload_example", axum::routing::get(Self::update_many_payload_example))
                .route(
                    "/update_one",
                    axum::routing::patch({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::update_one_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/update_one_payload_example", axum::routing::get(Self::update_one_payload_example))
                .route(
                    "/delete_many",
                    axum::routing::delete({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::delete_many_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/delete_many_payload_example", axum::routing::get(Self::delete_many_payload_example))
                .route(
                    "/delete_one",
                    axum::routing::delete({
                        let table = table.to_string();
                        move |app_state: axum::extract::State<std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>>, request: axum::extract::Request| {
                            let table = table.clone();
                            async move { Self::delete_one_handle(app_state, request, &table).await }
                        }
                    }),
                )
                .route("/delete_one_payload_example", axum::routing::get(Self::delete_one_payload_example))
                .with_state(app_state),
        )
    }
    pub fn routes(app_state: std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>) -> axum::Router {
        Self::routes_handle(app_state, Self::table_name())
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExamplePreparePostgresqlErrorNamed {
    CreateExtensionIfNotExistsPgJsonschema {
        #[eo_to_std_string_string]
        error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CreateExtensionIfNotExistsUuidOssp {
        #[eo_to_std_string_string]
        error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PreparePostgresql {
        #[eo_to_std_string_string]
        error: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa ::
ToSchema,
)]
pub struct TableExampleCreate {
    pub column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create,
    pub column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create,
}
impl TableExampleCreate {
    fn create_query_part(&self, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = String::default();
        match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_part(&<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(), increment) {
            Ok(value) => {
                use std::fmt::Write as _;
                if write!(acc, "{value},").is_err() {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::create_query_part(&self.column_0, increment) {
            Ok(value) => {
                use std::fmt::Write as _;
                if write!(acc, "{value},").is_err() {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::create_query_part(&self.column_142, increment) {
            Ok(value) => {
                use std::fmt::Write as _;
                if write!(acc, "{value},").is_err() {
                    return Err(postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        let _: Option<char> = acc.pop();
        Ok(acc)
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_bind(<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(), query) {
            Ok(value) => {
                query = value;
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::create_query_bind(self.column_0, query) {
            Ok(value) => {
                query = value;
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::create_query_bind(self.column_142, query) {
            Ok(value) => {
                query = value;
            }
            Err(error_0) => {
                return Err(error_0);
            }
        }
        Ok(query)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_0: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            column_142: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleWhereMany {
    primary_key_column: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>,
    column_0: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>,
    column_142: Option<postgresql_crud::PostgresqlTypeWhere<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Where>>,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleWhereManyTryNewErrorNamed {
    NoFieldsProvided {
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleWhereMany {
    pub fn try_new(
        primary_key_column: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>,
        column_0: Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>,
        column_142: Option<postgresql_crud::PostgresqlTypeWhere<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Where>>,
    ) -> Result<Self, TableExampleWhereManyTryNewErrorNamed> {
        if let (None, None, None) = (&primary_key_column, &column_0, &column_142) {
            return Err(TableExampleWhereManyTryNewErrorNamed::NoFieldsProvided { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { primary_key_column, column_0, column_142 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TableExampleWhereMany {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        2u64 => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "primary_key_column" => Ok(__Field::__field0),
                        "column_0" => Ok(__Field::__field1),
                        "column_142" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"primary_key_column" => Ok(__Field::__field0),
                        b"column_0" => Ok(__Field::__field1),
                        b"column_142" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TableExampleWhereMany>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableExampleWhereMany;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct TableExampleWhereMany")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleWhereMany with 3 elements"));
                    };
                    let Some(__field1) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleWhereMany with 3 elements"));
                    };
                    let Some(__field2) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::PostgresqlTypeWhere<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Where>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleWhereMany with 3 elements"));
                    };
                    match TableExampleWhereMany::try_new(__field0, __field1, __field2) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: Option<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>> = None;
                    let mut __field1: Option<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>> = None;
                    let mut __field2: Option<Option<postgresql_crud::PostgresqlTypeWhere<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Where>>> = None;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("primary_key_column"));
                                }
                                __field0 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where>>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("column_0"));
                                }
                                __field1 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Where>>>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if Option::is_some(&__field2) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("column_142"));
                                }
                                __field2 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::PostgresqlTypeWhere<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Where>>>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        Some(__field0) => __field0,
                        None => serde::__private::de::missing_field("primary_key_column")?,
                    };
                    let __field1 = match __field1 {
                        Some(__field1) => __field1,
                        None => serde::__private::de::missing_field("column_0")?,
                    };
                    let __field2 = match __field2 {
                        Some(__field2) => __field2,
                        None => serde::__private::de::missing_field("column_142")?,
                    };
                    match TableExampleWhereMany::try_new(__field0, __field1, __field2) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["primary_key_column", "column_0", "column_142"];
            _serde::Deserializer::deserialize_struct(__deserializer, "TableExampleWhereMany", FIELDS, __Visitor { marker: _serde::__private::PhantomData::<Self>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleWhereMany {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            primary_key_column: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            column_0: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            column_142: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        }
    }
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    utoipa ::
ToSchema,
)]
pub struct StdOptionOptionTableExampleWhereMany(pub Option<TableExampleWhereMany>);
impl<'lifetime> postgresql_crud::PostgresqlTypeWhereFilter<'lifetime> for StdOptionOptionTableExampleWhereMany {
    fn query_part(&self, increment: &mut u64, _: &dyn std::fmt::Display, _: bool) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(match &self.0 {
            Some(value) => {
                let mut additional_parameters = String::from("where");
                let mut is_first_push_to_additional_parameters_already_happend = false;
                if let Some(value) = &value.primary_key_column {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &"primary_key_column", is_first_push_to_additional_parameters_already_happend) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            return Err(error_0);
                        }
                    }
                }
                if let Some(value) = &value.column_0 {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &"column_0", is_first_push_to_additional_parameters_already_happend) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            return Err(error_0);
                        }
                    }
                }
                if let Some(value) = &value.column_142 {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &"column_142", is_first_push_to_additional_parameters_already_happend) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                        }
                        Err(error_0) => {
                            return Err(error_0);
                        }
                    }
                }
                additional_parameters
            }
            None => String::default(),
        })
    }
    fn query_bind(self, mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        if let Some(value) = self.0 {
            if let Some(value) = value.primary_key_column {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                }
            }
            if let Some(value) = value.column_0 {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                }
            }
            if let Some(value) = value.column_142 {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query) {
                    Ok(value) => {
                        query = value;
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                }
            }
        }
        Ok(query)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdOptionOptionTableExampleWhereMany {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, PartialEq, Clone)]
pub enum TableExampleSelect {
    #[serde(rename(serialize = "primary_key_column", deserialize = "primary_key_column"))]
    PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select),
    #[serde(rename(serialize = "column_0", deserialize = "column_0"))]
    Column0(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Select),
    #[serde(rename(serialize = "column_142", deserialize = "column_142"))]
    Column142(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Select),
}
impl std::fmt::Display for TableExampleSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_else(|element| format!("cannot serialize into json: {element:?}")))
    }
}
impl error_occurence_lib::ToStdStringString for TableExampleSelect {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleSelect {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Vec<Self> {
        vec![
            Self::PrimaryKeyColumn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Column0(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Column142(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct TableExampleRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key_column: Option<postgresql_crud::Value<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_142: Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Read>>,
}
impl TableExampleRead {
    fn try_from_sqlx_postgres_pg_row_with_not_empty_unique_enum_vec_table_example_select(value: sqlx::postgres::PgRow, select: &postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>) -> Result<Self, sqlx::Error> {
        let mut primary_key_column: Option<postgresql_crud::Value<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>> = None;
        let mut column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Read>> = None;
        let mut column_142: Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Read>> = None;
        for element in select.to_vec() {
            match element {
                TableExampleSelect::PrimaryKeyColumn(_) => match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read, &str>(&value, "primary_key_column") {
                    Ok(value) => {
                        primary_key_column = Some(postgresql_crud::Value { value });
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                },
                TableExampleSelect::Column0(_) => match sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Read, &str>(&value, "column_0") {
                    Ok(value) => {
                        column_0 = Some(postgresql_crud::Value { value });
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                },
                TableExampleSelect::Column142(_) => match sqlx::Row::try_get::<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Read, &str>(&value, "column_142") {
                    Ok(value) => {
                        column_142 = Some(postgresql_crud::Value { value });
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                },
            }
        }
        Ok(Self { primary_key_column, column_0, column_142 })
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct TableExampleReadOnlyIds {
    pub primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::ReadOnlyIds,
    pub column_0: Option<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::ReadOnlyIds>,
    pub column_142: Option<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::ReadOnlyIds>,
}
impl TryFrom<sqlx::postgres::PgRow> for TableExampleReadOnlyIds {
    type Error = sqlx::Error;
    fn try_from(value: sqlx::postgres::PgRow) -> Result<Self, Self::Error> {
        let primary_key_column = match sqlx::Row::try_get::<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::ReadOnlyIds, &str>(&value, "primary_key_column") {
            Ok(value) => value,
            Err(error_0) => {
                return Err(error_0);
            }
        };
        let column_0 = sqlx::Row::try_get::<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::ReadOnlyIds, &str>(&value, "column_0").ok();
        let column_142 = sqlx::Row::try_get::<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::ReadOnlyIds, &str>(&value, "column_142").ok();
        Ok(Self { primary_key_column, column_0, column_142 })
    }
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleUpdate {
    primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate,
    column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>,
    column_142: Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Update>>,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateTryNewErrorNamed {
    NoFieldsProvided {
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleUpdate {
    pub fn try_new(primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate, column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>, column_142: Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Update>>) -> Result<Self, TableExampleUpdateTryNewErrorNamed> {
        if let (None, None) = (&column_0, &column_142) {
            return Err(TableExampleUpdateTryNewErrorNamed::NoFieldsProvided { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { primary_key_column, column_0, column_142 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TableExampleUpdate {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        2u64 => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "primary_key_column" => Ok(__Field::__field0),
                        "column_0" => Ok(__Field::__field1),
                        "column_142" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"primary_key_column" => Ok(__Field::__field0),
                        b"column_0" => Ok(__Field::__field1),
                        b"column_142" => Ok(__Field::__field2),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TableExampleUpdate>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableExampleUpdate;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct TableExampleUpdate")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = serde::de::SeqAccess::next_element::<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleUpdate with 3 elements"));
                    };
                    let Some(__field1) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleUpdate with 3 elements"));
                    };
                    let Some(__field2) = serde::de::SeqAccess::next_element::<Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Update>>>(&mut __seq)? else {
                        return Err(serde::de::Error::invalid_length(0usize, &"struct TableExampleUpdate with 3 elements"));
                    };
                    match TableExampleUpdate::try_new(__field0, __field1, __field2) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: Option<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate> = None;
                    let mut __field1: Option<Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>> = None;
                    let mut __field2: Option<Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Update>>> = None;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("primary_key_column"));
                                }
                                __field0 = Some(serde::de::MapAccess::next_value::<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("column_0"));
                                }
                                __field1 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Update>>>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if Option::is_some(&__field2) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field("column_142"));
                                }
                                __field2 = Some(serde::de::MapAccess::next_value::<Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Update>>>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        Some(__field0) => __field0,
                        None => serde::__private::de::missing_field("primary_key_column")?,
                    };
                    let __field1 = match __field1 {
                        Some(__field1) => __field1,
                        None => serde::__private::de::missing_field("column_0")?,
                    };
                    let __field2 = match __field2 {
                        Some(__field2) => __field2,
                        None => serde::__private::de::missing_field("column_142")?,
                    };
                    match TableExampleUpdate::try_new(__field0, __field1, __field2) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["primary_key_column", "column_0", "column_142"];
            _serde::Deserializer::deserialize_struct(__deserializer, "TableExampleUpdate", FIELDS, __Visitor { marker: _serde::__private::PhantomData::<Self>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            primary_key_column: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            column_0: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            column_142: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleUpdateForQuery {
    primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdateForQuery,
    column_0: Option<postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::UpdateForQuery>>,
    column_142: Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::UpdateForQuery>>,
}
impl TableExampleUpdateForQuery {
    fn update_query_part_primary_key(&self, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_part(&self.primary_key_column, "", TableExample::primary_key(), "", increment) {
            Ok(value_snake_case) => Ok(value_snake_case),
            Err(error_0) => Err(error_0),
        }
    }
    fn update_query_part_column_0(value: &postgresql_crud::Value<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::UpdateForQuery>, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::update_query_part(&value.value, "column_0", "column_0", "", increment) {
            Ok(value) => Ok(value),
            Err(error_0) => Err(error_0),
        }
    }
    fn update_query_part_column_142(value: &postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::UpdateForQuery>, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_part(&value.value, "column_142", "column_142", "", increment) {
            Ok(value) => Ok(value),
            Err(error_0) => Err(error_0),
        }
    }
    fn select_only_updated_ids_query_part(&self, increment: &mut u64) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = String::new();
        acc.push_str(&match <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_part(&self.primary_key_column, "primary_key_column", increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        });
        if let Some(value) = &self.column_0 {
            acc.push_str(&match <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_part(&value.value, "column_0", increment) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            });
        }
        if let Some(value) = &self.column_142 {
            acc.push_str(&match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::select_only_updated_ids_query_part(&value.value, "column_142", increment) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            });
        }
        let _: Option<char> = acc.pop();
        Ok(acc)
    }
}
impl From<TableExampleUpdate> for TableExampleUpdateForQuery {
    fn from(value: TableExampleUpdate) -> Self {
        Self {
            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::UpdateForQuery::from(value.primary_key_column),
            column_0: match value.column_0 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::UpdateForQuery::from(value.value),
                }),
                None => None,
            },
            column_142: match value.column_142 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::UpdateForQuery::from(value.value),
                }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleCreateManyPayload(pub Vec<TableExampleCreate>);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleCreateManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
#[derive(Debug)]
pub struct TableExampleCreateManyParameters {
    pub payload: TableExampleCreateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleCreateManyResponseVariants {
    Desirable(Vec<TableExampleReadOnlyIds>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleCreateManyErrorNamed> for TableExampleCreateManyResponseVariants {
    fn from(value: TableExampleCreateManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleCreateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleCreateManyErrorNamed {
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
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryCreateManyErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleCreateManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        create_many_error_named_with_serialize_deserialize: TableExampleCreateManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug)]
pub struct TableExampleCreateOneParameters {
    pub payload: TableExampleCreate,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleCreateOneResponseVariants {
    Desirable(TableExampleReadOnlyIds),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleCreateOneErrorNamed> for TableExampleCreateOneResponseVariants {
    fn from(value: TableExampleCreateOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleCreateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleCreateOneErrorNamed {
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
    HeaderContentTypeApplicationJsonNotFound {
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
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryCreateOneErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleCreateOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        create_one_error_named_with_serialize_deserialize: TableExampleCreateOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleReadManyPayload {
    pub where_many: StdOptionOptionTableExampleWhereMany,
    pub select: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>,
    pub order_by: postgresql_crud::OrderBy<TableExampleSelect>,
    pub pagination: postgresql_crud::PaginationStartsWithZero,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleReadManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            where_many: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            select: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            order_by: postgresql_crud::OrderBy {
                column: TableExampleSelect::PrimaryKeyColumn(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                order: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            },
            pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug)]
pub struct TableExampleReadManyParameters {
    pub payload: TableExampleReadManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleReadManyResponseVariants {
    Desirable(Vec<TableExampleRead>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    NotUniqueField { not_unique_field: TableExampleSelect, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleReadManyErrorNamed> for TableExampleReadManyResponseVariants {
    fn from(value: TableExampleReadManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleReadManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence } => Self::NotUniqueField { not_unique_field, code_occurence },
            TableExampleReadManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleReadManyErrorNamed {
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
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryReadManyErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleReadManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        read_many_error_named_with_serialize_deserialize: TableExampleReadManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleReadOnePayload {
    pub primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead,
    pub select: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleReadOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            primary_key_column: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            select: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug)]
pub struct TableExampleReadOneParameters {
    pub payload: TableExampleReadOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleReadOneResponseVariants {
    Desirable(TableExampleRead),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    NotUniqueField { not_unique_field: TableExampleSelect, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleReadOneErrorNamed> for TableExampleReadOneResponseVariants {
    fn from(value: TableExampleReadOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleReadOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::NotUniqueField { not_unique_field, code_occurence } => Self::NotUniqueField { not_unique_field, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleReadOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleReadOneErrorNamed {
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
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryReadOneErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleReadOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        read_one_error_named_with_serialize_deserialize: TableExampleReadOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleUpdateManyPayload(Vec<TableExampleUpdate>);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateManyPayloadTryNewErrorNamed {
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate,
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleUpdateManyPayload {
    pub fn try_new(value: Vec<TableExampleUpdate>) -> Result<Self, TableExampleUpdateManyPayloadTryNewErrorNamed> {
        let mut acc = Vec::new();
        for element in &value {
            if acc.contains(&&element.primary_key_column) {
                return Err(TableExampleUpdateManyPayloadTryNewErrorNamed::NotUniquePrimaryKey {
                    not_unique_primary_key: element.primary_key_column.clone(),
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            acc.push(&element.primary_key_column);
        }
        Ok(Self(value))
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TableExampleUpdateManyPayload {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TableExampleUpdateManyPayload>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableExampleUpdateManyPayload;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct TableExampleUpdateManyPayload")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<TableExampleUpdate> = <Vec<TableExampleUpdate> as _serde::Deserialize>::deserialize(__e)?;
                    match TableExampleUpdateManyPayload::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<Vec<TableExampleUpdate>>(&mut __seq)? {
                        Some(__value) => __value,
                        None => {
                            return Err(_serde::de::Error::invalid_length(0usize, &"tuple struct TableExampleUpdateManyPayload with 1 element"));
                        }
                    };
                    match TableExampleUpdateManyPayload::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(__deserializer, "TableExampleUpdateManyPayload", __Visitor { marker: _serde::__private::PhantomData::<Self>, lifetime: _serde::__private::PhantomData })
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleUpdateManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
#[derive(Debug)]
pub struct TableExampleUpdateManyParameters {
    pub payload: TableExampleUpdateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleUpdateManyResponseVariants {
    Desirable(Vec<TableExampleReadOnlyIds>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleUpdateManyErrorNamed> for TableExampleUpdateManyResponseVariants {
    fn from(value: TableExampleUpdateManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateManyErrorNamed {
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
    HeaderContentTypeApplicationJsonNotFound {
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
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryUpdateManyErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleUpdateManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        update_many_error_named_with_serialize_deserialize: TableExampleUpdateManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug)]
pub struct TableExampleUpdateOneParameters {
    pub payload: TableExampleUpdate,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleUpdateOneResponseVariants {
    Desirable(TableExampleReadOnlyIds),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleUpdateOneErrorNamed> for TableExampleUpdateOneResponseVariants {
    fn from(value: TableExampleUpdateOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateOneErrorNamed {
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
    HeaderContentTypeApplicationJsonNotFound {
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
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryUpdateOneErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleUpdateOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        update_one_error_named_with_serialize_deserialize: TableExampleUpdateOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleDeleteManyPayload {
    pub where_many: StdOptionOptionTableExampleWhereMany,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleDeleteManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            where_many: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug)]
pub struct TableExampleDeleteManyParameters {
    pub payload: TableExampleDeleteManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleDeleteManyResponseVariants {
    Desirable(Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    QueryPart { error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleDeleteManyErrorNamed> for TableExampleDeleteManyResponseVariants {
    fn from(value: TableExampleDeleteManyErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::QueryPart { error, code_occurence } => Self::QueryPart { error, code_occurence },
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleDeleteManyErrorNamed {
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
    HeaderContentTypeApplicationJsonNotFound {
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
    QueryPart {
        #[eo_error_occurence]
        error: postgresql_crud::QueryPartErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryDeleteManyErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleDeleteManyErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        delete_many_error_named_with_serialize_deserialize: TableExampleDeleteManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleDeleteOnePayload {
    pub primary_key_column: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for TableExampleDeleteOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            primary_key_column: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug)]
pub struct TableExampleDeleteOneParameters {
    pub payload: TableExampleDeleteOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleDeleteOneResponseVariants {
    Desirable(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read),
    CheckBodySize { check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    Postgresql { postgresql: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    SerdeJson { serde_json: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    HeaderContentTypeApplicationJsonNotFound { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    CheckCommit { check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    RowAndRollback { row: String, rollback: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
    TryBind { try_bind: String, code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl From<TableExampleDeleteOneErrorNamed> for TableExampleDeleteOneResponseVariants {
    fn from(value: TableExampleDeleteOneErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self::HeaderContentTypeApplicationJsonNotFound { code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize::TryBind { try_bind, code_occurence } => Self::TryBind { try_bind, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleDeleteOneErrorNamed {
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
    HeaderContentTypeApplicationJsonNotFound {
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
    TryBind {
        #[eo_to_std_string_string_serialize_deserialize]
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryDeleteOneErrorNamed {
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
        response_text: String,
        #[eo_to_std_string_string]
        serde: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TableExampleDeleteOneErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        delete_one_error_named_with_serialize_deserialize: TableExampleDeleteOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[cfg(test)]
mod table_example_tests {
    use super::*;
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<TableExample>(), 0);
    }
    #[test]
    fn crud() {
        std::thread::Builder::new()
            .stack_size(16 * 1024 * 1024)
            .spawn(|| {
                tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("error 38823c21-1879-449c-9b60-ce7293709959").block_on(async {
                    tracing_subscriber::fmt::init();
                    let no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row = "no rows returned by a query that expected to return at least one row";
                    static CONFIG: std::sync::OnceLock<server_config::Config> = std::sync::OnceLock::new();
                    let config = CONFIG.get_or_init(|| server_config::Config {
                        service_socket_address: <config_lib::ServiceSocketAddress as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("127.0.0.1:8080".to_string()).expect("error b5b3915a-0e18-4815-a614-6b0e9a00d73f").0,
                        database_url: <config_lib::DatabaseUrl as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=10".to_string()).expect("error f9c20f05-3cdf-46ae-b6d3-5943c627f0df").0,
                        timezone: <config_lib::Timezone as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("10800".to_string()).expect("error d00d8998-52f9-45c1-a4b0-c93bc95a313e").0,
                        tracing_level: <config_lib::TracingLevel as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("error".to_string()).expect("error 957178c9-4d92-4110-b524-9dc21d147a7c").0,
                        source_place_type: <config_lib::SourcePlaceType as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("source".to_string()).expect("error bec0950e-e9de-42f3-b3a2-67d9d98ae8a6").0,
                        enable_api_git_commit_check: <config_lib::EnableApiGitCommitCheck as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("true".to_string()).expect("error 31f02640-d62b-41ca-837d-d61b707d4baf").0,
                        maximum_size_of_http_body_in_bytes: <config_lib::MaximumSizeOfHttpBodyInBytes as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok("1048576000".to_string()).expect("error 93b2f818-18be-4bb6-8a02-53c6e55ded2d").0,
                    });
                    let postgres_pool = sqlx::postgres::PgPoolOptions::new().max_connections(50).connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&config))).await.expect("error e3044bb9-7b76-4c0c-bc5f-eb34da05a103");
                    let url = format!("http://{}", app_state::GetServiceSocketAddress::get_service_socket_address(&config));
                    let table = "table_example";
                    let table_create_many = format!("{table}_create_many");
                    let table_create_one = format!("{table}_create_one");
                    let table_test_read_many_by_non_existent_primary_keys = format!("{table}_test_read_many_by_non_existent_primary_keys");
                    let table_test_read_many_by_equal_to_created_primary_keys = format!("{table}_test_read_many_by_equal_to_created_primary_keys");
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0 = format!("{table}_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0");
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142 = format!("{table}_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142");
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0 = format!("{table}_eb24448c_fa63_4259_bb05_3215802a78f6_column_0");
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_142 = format!("{table}_eb24448c_fa63_4259_bb05_3215802a78f6_column_142");
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0 = format!("{table}_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0");
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142 = format!("{table}_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142");
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0 = format!("{table}_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0");
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142 = format!("{table}_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142");
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0 = format!("{table}_5a52af33_a590_403b_808e_961df6d7e7aa_column_0");
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_142 = format!("{table}_5a52af33_a590_403b_808e_961df6d7e7aa_column_142");
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0 = format!("{table}_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0");
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142 = format!("{table}_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142");
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0 = format!("{table}_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0");
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142 = format!("{table}_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142");
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0 = format!("{table}_de556c26_9297_4adb_9483_22d474cf1e7d_column_0");
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_142 = format!("{table}_de556c26_9297_4adb_9483_22d474cf1e7d_column_142");
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0 = format!("{table}_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0");
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142 = format!("{table}_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142");
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0 = format!("{table}_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0");
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142 = format!("{table}_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142");
                    let table_read_one = format!("{table}_read_one");
                    let table_update_many = format!("{table}_update_many");
                    let table_update_one = format!("{table}_update_one");
                    let table_delete_many = format!("{table}_delete_many");
                    let table_delete_one = format!("{table}_delete_one");
                    let table_create_many_cloned = table_create_many.clone();
                    let table_create_one_cloned = table_create_one.clone();
                    let table_test_read_many_by_non_existent_primary_keys_cloned = table_test_read_many_by_non_existent_primary_keys.clone();
                    let table_test_read_many_by_equal_to_created_primary_keys_cloned = table_test_read_many_by_equal_to_created_primary_keys.clone();
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0.clone();
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142_cloned = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_142_cloned = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_142.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142_cloned = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142_cloned = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_142_cloned = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_142.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142_cloned = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142_cloned = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_142_cloned = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_142.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142_cloned = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142_cloned = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142.clone();
                    let table_read_one_cloned = table_read_one.clone();
                    let table_update_many_cloned = table_update_many.clone();
                    let table_update_one_cloned = table_update_one.clone();
                    let table_delete_many_cloned = table_delete_many.clone();
                    let table_delete_one_cloned = table_delete_one.clone();
                    let table_create_many_cloned2 = table_create_many.clone();
                    let table_create_one_cloned2 = table_create_one.clone();
                    let table_test_read_many_by_non_existent_primary_keys_cloned2 = table_test_read_many_by_non_existent_primary_keys.clone();
                    let table_test_read_many_by_equal_to_created_primary_keys_cloned2 = table_test_read_many_by_equal_to_created_primary_keys.clone();
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned2 = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0.clone();
                    let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142_cloned2 = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned2 = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0.clone();
                    let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_142_cloned2 = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_142.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned2 = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0.clone();
                    let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142_cloned2 = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned2 = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0.clone();
                    let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142_cloned2 = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned2 = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0.clone();
                    let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_142_cloned2 = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_142.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned2 = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0.clone();
                    let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142_cloned2 = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned2 = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0.clone();
                    let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142_cloned2 = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned2 = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0.clone();
                    let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_142_cloned2 = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_142.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned2 = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0.clone();
                    let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142_cloned2 = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned2 = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0.clone();
                    let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142_cloned2 = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142.clone();
                    let table_read_one_cloned2 = table_read_one.clone();
                    let table_update_many_cloned2 = table_update_many.clone();
                    let table_update_one_cloned2 = table_update_one.clone();
                    let table_delete_one_cloned2 = table_delete_one.clone();
                    let drop_all_test_tables = async || {
                        async fn drop_table_if_exists(postgres_pool: &sqlx::Pool<sqlx::Postgres>, table: &str) {
                            let query = format!("drop table if exists {table}");
                            let _unused = sqlx::query(&query).execute(postgres_pool).await.expect("error 1b11bf1b-9180-419f-bae7-b1ab93cd9c57");
                        }
                        drop_table_if_exists(&postgres_pool, table).await;
                        drop_table_if_exists(&postgres_pool, &table_create_many).await;
                        drop_table_if_exists(&postgres_pool, &table_create_one).await;
                        drop_table_if_exists(&postgres_pool, &table_test_read_many_by_non_existent_primary_keys_cloned2).await;
                        drop_table_if_exists(&postgres_pool, &table_test_read_many_by_equal_to_created_primary_keys_cloned2).await;
                        drop_table_if_exists(&postgres_pool, &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0).await;
                        drop_table_if_exists(&postgres_pool, &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142).await;
                        drop_table_if_exists(&postgres_pool, &table_read_one).await;
                        drop_table_if_exists(&postgres_pool, &table_update_many).await;
                        drop_table_if_exists(&postgres_pool, &table_update_one).await;
                        drop_table_if_exists(&postgres_pool, &table_delete_many).await;
                        drop_table_if_exists(&postgres_pool, &table_delete_one).await;
                    };
                    drop_all_test_tables().await;
                    let postgres_pool_for_tokio_spawn_sync_move = postgres_pool.clone();
                    let (started_tx, started_rx) = tokio::sync::oneshot::channel();
                    let _unused = tokio::spawn(async move {
                        TableExample::prepare_extensions(&postgres_pool_for_tokio_spawn_sync_move).await.expect("error 0633ff48-ebc4-460f-a282-d750511f5d78");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, table).await.expect("error 0c29cf7d-1af7-459c-b0c6-69855ca98bef");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_create_many_cloned).await.expect("error 141d990c-91e5-4518-8978-7660fcf88784");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_create_one_cloned).await.expect("error cdd3b111-5e8b-4201-896e-bd38dc8b4d7c");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_test_read_many_by_non_existent_primary_keys_cloned).await.expect("error 56a27d70-0393-4759-9d02-f9eb1e623f5f");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_test_read_many_by_equal_to_created_primary_keys_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142_cloned).await.expect("error 4bd22656-8d17-427f-820f-2dd0ea2eac86");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_read_one_cloned).await.expect("error 425e8574-6cdd-43b5-9b7b-75efce9b750d");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_update_many_cloned).await.expect("error ab50eb74-29ab-49b3-bdd4-ff6c6c6b700a");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_update_one_cloned).await.expect("error de8885ae-34f5-430b-a3b4-bf91c999b2c8");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_delete_many_cloned).await.expect("error 2bb3d5ec-1069-470c-a758-60afe3bd5224");
                        TableExample::prepare_postgresql_table(&postgres_pool_for_tokio_spawn_sync_move, &table_delete_one_cloned).await.expect("error e5cc2f6f-65a2-472d-8a1e-56e23fbc165a");
                        let app_state = std::sync::Arc::new(server_app_state::ServerAppState {
                            postgres_pool: postgres_pool_for_tokio_spawn_sync_move.clone(),
                            config,
                            project_git_info: &git_info::PROJECT_GIT_INFO,
                        });
                        let tcp_listener = tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&config)).await.expect("error 663ae29e-bc00-4ea1-a7e9-4dddceb5b53a");
                        if let Err(error) = started_tx.send(()) {
                            panic!("error aa3b8154-1fe2-4d3f-a164-26f9d21245cd {error:#?}");
                        }
                        axum::serve(
                            tcp_listener,
                            axum::Router::new()
                                .merge(TableExample::routes(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state)))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_create_many_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_create_one_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_test_read_many_by_non_existent_primary_keys_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_test_read_many_by_equal_to_created_primary_keys_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_eb24448c_fa63_4259_bb05_3215802a78f6_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_5a52af33_a590_403b_808e_961df6d7e7aa_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_de556c26_9297_4adb_9483_22d474cf1e7d_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_read_one_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_update_many_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_update_one_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_delete_many_cloned))
                                .merge(TableExample::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &table_delete_one_cloned))
                                .into_make_service(),
                        )
                        .await
                        .unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
                    });
                    started_rx.await.expect("error 87003141-43a4-4975-8ddf-273148add50f");
                    let select_primary_key = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default())]).expect("error 0776170e-4dd6-4c14-a412-ce10b0c746f1");
                    let ident_create_default = TableExampleCreate {
                        column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                        column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                    };
                    let select_default_all_with_max_page_size = postgresql_crud::NotEmptyUniqueEnumVec::try_new(vec![
                        TableExampleSelect::PrimaryKeyColumn(<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize>::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size()),
                        TableExampleSelect::Column0(<<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize>::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size()),
                        TableExampleSelect::Column142(<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize>::default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size()),
                    ])
                    .expect("error 8f42ee4f-00d9-4b67-8ead-adddf5bcdf94");
                    let common_read_only_ids_returned_from_create_one = TableExample::try_create_one(&url, TableExampleCreateOneParameters { payload: ident_create_default.clone() }).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                    let some_value_read_only_ids_returned_from_create_one = Some(postgresql_crud::Value {
                        value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                    });
                    assert_eq!(
                        TableExampleRead {
                            primary_key_column: some_value_read_only_ids_returned_from_create_one.clone(),
                            column_0: None,
                            column_142: None
                        },
                        TableExample::try_read_one(
                            &url,
                            TableExampleReadOneParameters {
                                payload: TableExampleReadOnePayload {
                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                    select: select_primary_key.clone(),
                                },
                            },
                        )
                        .await
                        .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                        "error 3d9f2ec0-e374-48d2-a36b-486f5598b0b4"
                    );
                    let read_only_ids_from_try_delete_one = TableExample::try_delete_one(
                        &url,
                        TableExampleDeleteOneParameters {
                            payload: TableExampleDeleteOnePayload {
                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                            },
                        },
                    )
                    .await
                    .expect("error 006b18e9-c965-45ee-afc0-a4f6b850ed06");
                    assert_eq!(read_only_ids_from_try_delete_one, <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()), "error 26e2058b-4bc1-42da-8f35-0ab993904de5");
                    if let Err(error) = TableExample::try_read_one(
                        &url,
                        TableExampleReadOneParameters {
                            payload: TableExampleReadOnePayload {
                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(common_read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                select: select_default_all_with_max_page_size.clone(),
                            },
                        },
                    )
                    .await
                    {
                        if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                            if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 58b9a6a4-cf9b-49f3-a20f-7007deea40fd");
                            } else {
                                panic!("error 0ad0117b-a2e0-4629-99d0-71935cd93d15");
                            }
                        } else {
                            panic!("error c6695392-4b5f-4482-86aa-b2f19c33a746")
                        }
                    } else {
                        panic!("error 67e43b7a-d3ec-4a3b-a3f1-8c11499fd090")
                    }
                    futures::StreamExt::for_each_concurrent(
                        futures::stream::iter({
                            let mut acc: Vec<futures::future::BoxFuture<'static, ()>> = vec![];
                            {
                                let current_table = table_create_many_cloned2.clone();
                                {
                                    for chunk in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]).chunks(10).map(Vec::from) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_vec_create = {
                                                let mut acc = vec![];
                                                for element in chunk {
                                                    acc.push(TableExampleCreate {
                                                        column_0: element,
                                                        column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                    });
                                                }
                                                acc
                                            };
                                            let read_only_ids_from_try_create_many = TableExample::try_create_many_handle(&url_cloned, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(ident_vec_create.clone()) }, &current_table.clone()).await.expect("error 5eecedc4-bb02-454a-acd9-0af758f30b2e");
                                            assert_eq!(
                                                {
                                                    let mut acc = vec![];
                                                    assert_eq!(read_only_ids_from_try_create_many.len(), ident_vec_create.len(), "error 39572295-b6a4-49d7-a65a-16f8bcf44ede");
                                                    for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_vec_create.into_iter()) {
                                                        acc.push(TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids.primary_key_column),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_0.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), create.column_0),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_142.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), create.column_142),
                                                        });
                                                    }
                                                    acc.sort_by(|first, second| {
                                                        if let (Some(first), Some(second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                            first.value.cmp(&second.value)
                                                        } else {
                                                            panic!("error 4428083a-53be-4184-a5b7-94ae2de21d40");
                                                        }
                                                    });
                                                    acc
                                                },
                                                TableExample::try_read_many_handle(
                                                    &url_cloned,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                            let mut acc = vec![];
                                                                            for element in &read_only_ids_from_try_create_many {
                                                                                acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        element.primary_key_column.clone(),
                                                                                    ))),
                                                                                }));
                                                                            }
                                                                            acc
                                                                        })
                                                                        .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
                                                                    ),
                                                                    None,
                                                                    None
                                                                )
                                                                .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
                                                            )),
                                                            select: select_default_all_with_max_page_size_cloned.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc),
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                                        }
                                                    },
                                                    &current_table
                                                )
                                                .await
                                                .expect("error cb64ceaf-74a4-4501-b614-0c9d9e4e0598"),
                                                "error d19bbbf6-f64c-4151-8b5b-998a93e13af5"
                                            );
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        let mut acc = vec![];
                                                                        for element in &read_only_ids_from_try_create_many {
                                                                            acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                    element.primary_key_column.clone(),
                                                                                ))),
                                                                            }));
                                                                        }
                                                                        acc
                                                                    })
                                                                    .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 716e470e-d738-4642-adfc-df1f9b945d27");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(
                                                read_only_ids_from_try_delete_many,
                                                {
                                                    let mut acc = read_only_ids_from_try_create_many
                                                        .into_iter()
                                                        .map(|element| <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(element.primary_key_column))
                                                        .collect::<Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>();
                                                    acc.sort();
                                                    acc
                                                },
                                                "error f58f5572-4286-4a74-8006-0507339910d4"
                                            );
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        let mut acc = vec![];
                                                                        for element in &read_only_ids_from_try_delete_many {
                                                                            acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.clone())),
                                                                            }));
                                                                        }
                                                                        acc
                                                                    })
                                                                    .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned,
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 4e88679a-0d23-418f-8767-4e9b7531429c"),
                                                Err(error) => {
                                                    panic!("error 24ab86d6-15c9-47f1-a43f-c5fac4b38188 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    for chunk in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]).chunks(10).map(Vec::from) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_vec_create = {
                                                let mut acc = vec![];
                                                for element in chunk {
                                                    acc.push(TableExampleCreate {
                                                        column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                        column_142: element,
                                                    });
                                                }
                                                acc
                                            };
                                            let read_only_ids_from_try_create_many = TableExample::try_create_many_handle(&url_cloned, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(ident_vec_create.clone()) }, &current_table.clone()).await.expect("error 5eecedc4-bb02-454a-acd9-0af758f30b2e");
                                            assert_eq!(
                                                {
                                                    let mut acc = vec![];
                                                    assert_eq!(read_only_ids_from_try_create_many.len(), ident_vec_create.len(), "error 39572295-b6a4-49d7-a65a-16f8bcf44ede");
                                                    for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_vec_create.into_iter()) {
                                                        acc.push(TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids.primary_key_column),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_0.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), create.column_0),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_142.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), create.column_142),
                                                        });
                                                    }
                                                    acc.sort_by(|first, second| {
                                                        if let (Some(first), Some(second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                            first.value.cmp(&second.value)
                                                        } else {
                                                            panic!("error 4428083a-53be-4184-a5b7-94ae2de21d40");
                                                        }
                                                    });
                                                    acc
                                                },
                                                TableExample::try_read_many_handle(
                                                    &url_cloned,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                            let mut acc = vec![];
                                                                            for element in &read_only_ids_from_try_create_many {
                                                                                acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        element.primary_key_column.clone(),
                                                                                    ))),
                                                                                }));
                                                                            }
                                                                            acc
                                                                        })
                                                                        .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
                                                                    ),
                                                                    None,
                                                                    None
                                                                )
                                                                .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
                                                            )),
                                                            select: select_default_all_with_max_page_size_cloned.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc),
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                                        }
                                                    },
                                                    &current_table
                                                )
                                                .await
                                                .expect("error cb64ceaf-74a4-4501-b614-0c9d9e4e0598"),
                                                "error d19bbbf6-f64c-4151-8b5b-998a93e13af5"
                                            );
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        let mut acc = vec![];
                                                                        for element in &read_only_ids_from_try_create_many {
                                                                            acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                    element.primary_key_column.clone(),
                                                                                ))),
                                                                            }));
                                                                        }
                                                                        acc
                                                                    })
                                                                    .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 716e470e-d738-4642-adfc-df1f9b945d27");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(
                                                read_only_ids_from_try_delete_many,
                                                {
                                                    let mut acc = read_only_ids_from_try_create_many
                                                        .into_iter()
                                                        .map(|element| <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(element.primary_key_column))
                                                        .collect::<Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>();
                                                    acc.sort();
                                                    acc
                                                },
                                                "error f58f5572-4286-4a74-8006-0507339910d4"
                                            );
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        let mut acc = vec![];
                                                                        for element in &read_only_ids_from_try_delete_many {
                                                                            acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.clone())),
                                                                            }));
                                                                        }
                                                                        acc
                                                                    })
                                                                    .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned,
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 4e88679a-0d23-418f-8767-4e9b7531429c"),
                                                Err(error) => {
                                                    panic!("error 24ab86d6-15c9-47f1-a43f-c5fac4b38188 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                            };
                            {
                                let current_table = table_create_one_cloned2.clone();
                                {
                                    for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_from_try_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                                            assert_eq!(
                                                TableExampleRead {
                                                    primary_key_column: Some(postgresql_crud::Value {
                                                        value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone())
                                                    }),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_from_try_create_one.column_0.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0),
                                                    column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_from_try_create_one.column_142.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142)
                                                },
                                                TableExample::try_read_one_handle(
                                                    &url_cloned,
                                                    TableExampleReadOneParameters {
                                                        payload: TableExampleReadOnePayload {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone()),
                                                            select: select_default_all_with_max_page_size_cloned.clone()
                                                        }
                                                    },
                                                    &current_table
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                                                "error 5f2adbed-f716-440e-a990-4f1c258808b1"
                                            );
                                            let read_only_ids_from_try_delete_one = TableExample::try_delete_one_handle(
                                                &url_cloned,
                                                TableExampleDeleteOneParameters {
                                                    payload: TableExampleDeleteOnePayload {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone()),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            .expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                                            assert_eq!(read_only_ids_from_try_delete_one, <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone()), "error 4f563faf-1d9b-4ef3-8636-f93fde8ef235");
                                            if let Err(error) = TableExample::try_read_one_handle(
                                                &url_cloned,
                                                TableExampleReadOneParameters {
                                                    payload: TableExampleReadOnePayload {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone()),
                                                        select: select_default_all_with_max_page_size_cloned,
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                                    if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                        assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error d7152378-3a59-4050-8710-87b7000c8e3d");
                                                    } else {
                                                        panic!("error e1ac93a5-59e6-477e-a99d-c02e99497421");
                                                    }
                                                } else {
                                                    panic!("error bcd3f9bf-d6b7-4594-b078-8fe9c34bcf18")
                                                }
                                            } else {
                                                panic!("error 893263c9-7c62-4551-9225-74153c6e1c57")
                                            }
                                        }));
                                    }
                                };
                                {
                                    for element in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_from_try_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                                            assert_eq!(
                                                TableExampleRead {
                                                    primary_key_column: Some(postgresql_crud::Value {
                                                        value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone())
                                                    }),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_from_try_create_one.column_0.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0),
                                                    column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_from_try_create_one.column_142.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142)
                                                },
                                                TableExample::try_read_one_handle(
                                                    &url_cloned,
                                                    TableExampleReadOneParameters {
                                                        payload: TableExampleReadOnePayload {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone()),
                                                            select: select_default_all_with_max_page_size_cloned.clone()
                                                        }
                                                    },
                                                    &current_table
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                                                "error 5f2adbed-f716-440e-a990-4f1c258808b1"
                                            );
                                            let read_only_ids_from_try_delete_one = TableExample::try_delete_one_handle(
                                                &url_cloned,
                                                TableExampleDeleteOneParameters {
                                                    payload: TableExampleDeleteOnePayload {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone()),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            .expect("error 32e30b87-b46a-4f39-aeb0-39694fc52d30");
                                            assert_eq!(read_only_ids_from_try_delete_one, <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone()), "error 4f563faf-1d9b-4ef3-8636-f93fde8ef235");
                                            if let Err(error) = TableExample::try_read_one_handle(
                                                &url_cloned,
                                                TableExampleReadOneParameters {
                                                    payload: TableExampleReadOnePayload {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one.primary_key_column.clone()),
                                                        select: select_default_all_with_max_page_size_cloned,
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                                    if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                        assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error d7152378-3a59-4050-8710-87b7000c8e3d");
                                                    } else {
                                                        panic!("error e1ac93a5-59e6-477e-a99d-c02e99497421");
                                                    }
                                                } else {
                                                    panic!("error bcd3f9bf-d6b7-4594-b078-8fe9c34bcf18")
                                                }
                                            } else {
                                                panic!("error 893263c9-7c62-4551-9225-74153c6e1c57")
                                            }
                                        }));
                                    }
                                };
                            };
                            {
                                {
                                    let current_table = table_test_read_many_by_non_existent_primary_keys_cloned2.clone();
                                    async fn generate_test_read_many_by_non_existent_primary_keys(length: usize, url: &str, select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>, current_table: &str, ident_create_default: TableExampleCreate, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str) {
                                        let read_only_ids_from_try_create_one_default = TableExample::try_create_one_handle(url, TableExampleCreateOneParameters { payload: ident_create_default.clone() }, current_table).await.expect("error 71632985-ec25-4928-aa9e-1e224a7478c1");
                                        match TableExample::try_read_many_handle(
                                            url,
                                            TableExampleReadManyParameters {
                                                payload: TableExampleReadManyPayload {
                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                        TableExampleWhereMany::try_new(
                                                            Some(
                                                                postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                    let mut acc = vec![];
                                                                    for _ in 1..=length {
                                                                        acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(uuid::Uuid::new_v4()),
                                                                        }));
                                                                    }
                                                                    acc
                                                                })
                                                                .expect("error 6de1e731-a28a-4f74-8a73-0f8f8ec34a43"),
                                                            ),
                                                            None,
                                                            None,
                                                        )
                                                        .expect("error 5dfe67ec-9d91-4bf6-a4fb-f71e7826c15c"),
                                                    )),
                                                    select: select_default_all_with_max_page_size.clone(),
                                                    order_by: postgresql_crud::OrderBy {
                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                        order: Some(postgresql_crud::Order::Asc),
                                                    },
                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            Ok(value) => assert!(value.is_empty(), "error 06df4025-e2d1-4128-b819-c06613c6ae3f"),
                                            Err(error) => {
                                                panic!("error e661c49b-2288-4548-8783-35495e193976: {error:#?}");
                                            }
                                        }
                                        let _: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read = TableExample::try_delete_one_handle(
                                            url,
                                            TableExampleDeleteOneParameters {
                                                payload: TableExampleDeleteOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        .expect("error cc3958f0-1a4a-4440-97c7-ca63611405c5");
                                        if let Err(error) = TableExample::try_read_one_handle(
                                            url,
                                            TableExampleReadOneParameters {
                                                payload: TableExampleReadOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                    select: select_default_all_with_max_page_size.clone(),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                                if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                    assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 99bd4d82-4976-4e1e-8022-543b01221a91");
                                                } else {
                                                    panic!("error 5a86690f-80e2-4dbc-9853-1826f94748bd");
                                                }
                                            } else {
                                                panic!("error d90d6d02-33f7-4886-99df-dee76b83400f")
                                            }
                                        } else {
                                            panic!("error 62e65598-2c1f-4912-b3a2-dccd6e1714a1")
                                        }
                                    }
                                    let lengths = vec![1, 2];
                                    for element in lengths {
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        let current_table = current_table.clone();
                                        let ident_create_default_cloned = ident_create_default.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            generate_test_read_many_by_non_existent_primary_keys(element, &url_cloned, select_default_all_with_max_page_size_cloned, &current_table, ident_create_default_cloned, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row).await;
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_test_read_many_by_equal_to_created_primary_keys_cloned2.clone();
                                    async fn generate_test_read_many_by_equal_to_created_primary_keys(length: usize, url: &str, select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>, current_table: &str, ident_create_default: TableExampleCreate, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str) {
                                        let read_only_ids_from_try_create_one_default = TableExample::try_create_one_handle(url, TableExampleCreateOneParameters { payload: ident_create_default.clone() }, current_table).await.expect("error 71632985-ec25-4928-aa9e-1e224a7478c1");
                                        let ident_vec_create = {
                                            let mut acc = vec![];
                                            for _ in 1..=length {
                                                acc.push(ident_create_default.clone());
                                            }
                                            acc
                                        };
                                        let read_only_ids_from_try_create_many = TableExample::try_create_many_handle(url, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(ident_vec_create.clone()) }, current_table).await.expect("error d775179f-f7b1-41d3-9c83-4ca8bd1abeec");
                                        assert_eq!(
                                            {
                                                let mut acc = vec![];
                                                assert_eq!(read_only_ids_from_try_create_many.len(), ident_vec_create.len(), "error 52c9d1ea-1593-4b32-97d1-0ed4a529a74a");
                                                for (read_only_ids, create) in read_only_ids_from_try_create_many.clone().into_iter().zip(ident_vec_create.into_iter()) {
                                                    acc.push(TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids.primary_key_column),
                                                        column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_0.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), create.column_0),
                                                        column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids.column_142.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), create.column_142),
                                                    });
                                                }
                                                acc.sort_by(|first, second| {
                                                    if let (Some(first), Some(second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                        first.value.cmp(&second.value)
                                                    } else {
                                                        panic!("error 0faa6fb3-a7c0-44ca-9b51-13f6ca2fc543");
                                                    }
                                                });
                                                acc
                                            },
                                            TableExample::try_read_many_handle(
                                                url,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        let mut acc = vec![];
                                                                        for element in &read_only_ids_from_try_create_many {
                                                                            acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                    element.primary_key_column.clone(),
                                                                                ))),
                                                                            }));
                                                                        }
                                                                        acc
                                                                    })
                                                                    .expect("error 32ad1f00-4ba1-4da9-8927-b8e067e3dcd9"),
                                                                ),
                                                                None,
                                                                None
                                                            )
                                                            .expect("error e3309f9a-cb37-4c3d-931f-9457b43594bd"),
                                                        )),
                                                        select: select_default_all_with_max_page_size.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc)
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 0bb172c7-3344-4d31-bba5-6ce9e8f28746"),
                                                    }
                                                },
                                                current_table
                                            )
                                            .await
                                            .expect("error 0c45413e-45c7-493c-a105-3ba88661d360"),
                                            "error 3b2cf1f5-2c4e-4908-ba66-f4af84fe0893"
                                        );
                                        let read_only_ids_from_try_delete_many = {
                                            let mut acc = TableExample::try_delete_many_handle(
                                                url,
                                                TableExampleDeleteManyParameters {
                                                    payload: TableExampleDeleteManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                            primary_key_column: Some(
                                                                postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                    let mut acc = vec![];
                                                                    for element in &read_only_ids_from_try_create_many {
                                                                        acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                element.primary_key_column.clone(),
                                                                            ))),
                                                                        }));
                                                                    }
                                                                    acc
                                                                })
                                                                .expect("error dbfe049c-4142-469f-907c-4ecc5dd132dc"),
                                                            ),
                                                            column_0: None,
                                                            column_142: None,
                                                        })),
                                                    },
                                                },
                                                current_table,
                                            )
                                            .await
                                            .expect("error d5c23a9d-eb02-44e4-8654-e2a3d7752f51");
                                            acc.sort();
                                            acc
                                        };
                                        assert_eq!(
                                            read_only_ids_from_try_delete_many,
                                            {
                                                let mut acc = read_only_ids_from_try_create_many
                                                    .into_iter()
                                                    .map(|element| <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(element.primary_key_column))
                                                    .collect::<Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>();
                                                acc.sort();
                                                acc
                                            },
                                            "error ebbbea6e-c402-4637-9bab-02678c11926c"
                                        );
                                        match TableExample::try_read_many_handle(
                                            url,
                                            TableExampleReadManyParameters {
                                                payload: TableExampleReadManyPayload {
                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                        TableExampleWhereMany::try_new(
                                                            Some(
                                                                postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                    let mut acc = vec![];
                                                                    for element in &read_only_ids_from_try_delete_many {
                                                                        acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(element.clone())),
                                                                        }));
                                                                    }
                                                                    acc
                                                                })
                                                                .expect("error d9a27180-ef8f-48c3-86b1-fef0a49d3d13"),
                                                            ),
                                                            None,
                                                            None,
                                                        )
                                                        .expect("error 466716e1-9746-4dfc-bfe2-ba689d3178d6"),
                                                    )),
                                                    select: select_default_all_with_max_page_size.clone(),
                                                    order_by: postgresql_crud::OrderBy {
                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                        order: Some(postgresql_crud::Order::Asc),
                                                    },
                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error bd3be33e-f145-445b-8d02-4c42c8ab4a0c"),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            Ok(value) => assert!(value == Vec::new(), "error d79c0af3-5e2e-4891-a7ff-d1007b573e77"),
                                            Err(error) => {
                                                panic!("error 1f079962-06af-4d21-a837-c88b0e7db265 {error:#?}");
                                            }
                                        }
                                        let _: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read = TableExample::try_delete_one_handle(
                                            url,
                                            TableExampleDeleteOneParameters {
                                                payload: TableExampleDeleteOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        .expect("error cc3958f0-1a4a-4440-97c7-ca63611405c5");
                                        if let Err(error) = TableExample::try_read_one_handle(
                                            url,
                                            TableExampleReadOneParameters {
                                                payload: TableExampleReadOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                    select: select_default_all_with_max_page_size.clone(),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                                if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                    assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 99bd4d82-4976-4e1e-8022-543b01221a91");
                                                } else {
                                                    panic!("error 5a86690f-80e2-4dbc-9853-1826f94748bd");
                                                }
                                            } else {
                                                panic!("error d90d6d02-33f7-4886-99df-dee76b83400f")
                                            }
                                        } else {
                                            panic!("error 62e65598-2c1f-4912-b3a2-dccd6e1714a1")
                                        }
                                    }
                                    let lengths = vec![1, 2];
                                    for element in lengths {
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        let current_table = current_table.clone();
                                        let ident_create_default_cloned = ident_create_default.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            generate_test_read_many_by_equal_to_created_primary_keys(element, &url_cloned, select_default_all_with_max_page_size_cloned, &current_table, ident_create_default_cloned, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row).await;
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0_cloned2.clone();
                                    for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            assert_eq!(
                                                vec![TableExampleRead {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                    column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                }],
                                                TableExample::try_read_many_handle(
                                                    &url_cloned,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                            postgresql_crud::LogicalOperator::And,
                                                                            vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                            )]
                                                                        )
                                                                        .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                    ),
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                            postgresql_crud::LogicalOperator::And,
                                                                            vec![<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(read_only_ids_returned_from_create_one.column_0.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), ident_create.column_0.clone())]
                                                                        )
                                                                        .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                    ),
                                                                    None
                                                                )
                                                                .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                            )),
                                                            select: select_default_all_with_max_page_size_cloned.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc)
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                        }
                                                    },
                                                    &current_table
                                                )
                                                .await
                                                .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                            );
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_142_cloned2.clone();
                                    for element in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            assert_eq!(
                                                vec![TableExampleRead {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                    column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                }],
                                                TableExample::try_read_many_handle(
                                                    &url_cloned,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                            postgresql_crud::LogicalOperator::And,
                                                                            vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                            )]
                                                                        )
                                                                        .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                    ),
                                                                    None,
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                            postgresql_crud::LogicalOperator::And,
                                                                            vec![<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(read_only_ids_returned_from_create_one.column_142.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), ident_create.column_142.clone())]
                                                                        )
                                                                        .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                    )
                                                                )
                                                                .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                            )),
                                                            select: select_default_all_with_max_page_size_cloned.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc)
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                        }
                                                    },
                                                    &current_table
                                                )
                                                .await
                                                .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                            );
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0_cloned2.clone();
                                    for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            assert_eq!(
                                                vec![TableExampleRead {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                    column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                }],
                                                TableExample::try_read_many_handle(
                                                    &url_cloned,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                            postgresql_crud::LogicalOperator::And,
                                                                            vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                            )]
                                                                        )
                                                                        .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                    ),
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                            postgresql_crud::LogicalOperator::And,
                                                                            <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_equal_using_fields(read_only_ids_returned_from_create_one.column_0.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), ident_create.column_0.clone())
                                                                        )
                                                                        .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                    ),
                                                                    None
                                                                )
                                                                .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                            )),
                                                            select: select_default_all_with_max_page_size_cloned.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc)
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                        }
                                                    },
                                                    &current_table
                                                )
                                                .await
                                                .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                            );
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_142_cloned2.clone();
                                    for element in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            assert_eq!(
                                                vec![TableExampleRead {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                    column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                    column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                }],
                                                TableExample::try_read_many_handle(
                                                    &url_cloned,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                            postgresql_crud::LogicalOperator::And,
                                                                            vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                            )]
                                                                        )
                                                                        .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                    ),
                                                                    None,
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                            postgresql_crud::LogicalOperator::And,
                                                                            <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_vec_where_equal_using_fields(read_only_ids_returned_from_create_one.column_142.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), ident_create.column_142.clone())
                                                                        )
                                                                        .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                    )
                                                                )
                                                                .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                            )),
                                                            select: select_default_all_with_max_page_size_cloned.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc)
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                        }
                                                    },
                                                    &current_table
                                                )
                                                .await
                                                .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                            );
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0_cloned2.clone();
                                    for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field(read_only_ids_returned_from_create_one.column_0.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), ident_create.column_0.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),),
                                                                            None
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_142_cloned2.clone();
                                    for element in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field(read_only_ids_returned_from_create_one.column_142.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), ident_create.column_142.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            None,
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),)
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0_cloned2.clone();
                                    for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::create_into_postgresql_type_option_vec_where_dimension_one_equal(ident_create.column_0.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),),
                                                                            None
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_142_cloned2.clone();
                                    for element in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::create_into_postgresql_type_option_vec_where_dimension_one_equal(ident_create.column_142.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            None,
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),)
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0_cloned2.clone();
                                    for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::postgresql_type_option_vec_where_greater_than_test().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element.create,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than(element.variant, read_only_ids_returned_from_create_one.column_0.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), element.greater_than) {
                                                assert_eq!(
                                                    vec![TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                        column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                        column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                    }],
                                                    TableExample::try_read_many_handle(
                                                        &url_cloned,
                                                        TableExampleReadManyParameters {
                                                            payload: TableExampleReadManyPayload {
                                                                where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                    TableExampleWhereMany::try_new(
                                                                        Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::And,
                                                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                    read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                    postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                )]
                                                                            )
                                                                            .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                        ),
                                                                        Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![value]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),),
                                                                        None
                                                                    )
                                                                    .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                )),
                                                                select: select_default_all_with_max_page_size_cloned.clone(),
                                                                order_by: postgresql_crud::OrderBy {
                                                                    column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                    order: Some(postgresql_crud::Order::Asc)
                                                                },
                                                                pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                            }
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                    "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                );
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_142_cloned2.clone();
                                    for element in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::postgresql_type_option_vec_where_greater_than_test().unwrap_or(vec![]) {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element.create,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than(element.variant, read_only_ids_returned_from_create_one.column_142.clone().expect("error 2f7cdf57-72f7-4a1d-a1a1-8a7cbc5b90db"), element.greater_than) {
                                                assert_eq!(
                                                    vec![TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                        column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                        column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                    }],
                                                    TableExample::try_read_many_handle(
                                                        &url_cloned,
                                                        TableExampleReadManyParameters {
                                                            payload: TableExampleReadManyPayload {
                                                                where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                    TableExampleWhereMany::try_new(
                                                                        Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::And,
                                                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                    read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                    postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                )]
                                                                            )
                                                                            .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                        ),
                                                                        None,
                                                                        Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![value]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),)
                                                                    )
                                                                    .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                )),
                                                                select: select_default_all_with_max_page_size_cloned.clone(),
                                                                order_by: postgresql_crud::OrderBy {
                                                                    column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                    order: Some(postgresql_crud::Order::Asc)
                                                                },
                                                                pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                            }
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                    "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                );
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0_cloned2.clone();
                                    for element in {
                                        let mut acc = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_0.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(read_only_ids_returned_from_create_one.column_0.clone().expect("error 2ed000a5-cf70-4df1-903a-c1f6d224e926"), ident_create.column_0.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),),
                                                                            None
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_142_cloned2.clone();
                                    for element in {
                                        let mut acc = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_142.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal(read_only_ids_returned_from_create_one.column_142.clone().expect("error 2ed000a5-cf70-4df1-903a-c1f6d224e926"), ident_create.column_142.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            None,
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),)
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0_cloned2.clone();
                                    for element in {
                                        let mut acc = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_0.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(read_only_ids_returned_from_create_one.column_0.clone().expect("error 2ed000a5-cf70-4df1-903a-c1f6d224e926"), ident_create.column_0.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),),
                                                                            None
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_142_cloned2.clone();
                                    for element in {
                                        let mut acc = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_142.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal(read_only_ids_returned_from_create_one.column_142.clone().expect("error 2ed000a5-cf70-4df1-903a-c1f6d224e926"), ident_create.column_142.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            None,
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),)
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0_cloned2.clone();
                                    for element in {
                                        let mut acc = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_0.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(read_only_ids_returned_from_create_one.column_0.clone().expect("error 2ed000a5-cf70-4df1-903a-c1f6d224e926"), ident_create.column_0.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),),
                                                                            None
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_142_cloned2.clone();
                                    for element in {
                                        let mut acc = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_142.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal(read_only_ids_returned_from_create_one.column_142.clone().expect("error 2ed000a5-cf70-4df1-903a-c1f6d224e926"), ident_create.column_142.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            None,
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),)
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0_cloned2.clone();
                                    for element in {
                                        let mut acc = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_0.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(read_only_ids_returned_from_create_one.column_0.clone().expect("error 2ed000a5-cf70-4df1-903a-c1f6d224e926"), ident_create.column_0.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),),
                                                                            None
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_142_cloned2.clone();
                                    for element in {
                                        let mut acc = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_142.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal(read_only_ids_returned_from_create_one.column_142.clone().expect("error 2ed000a5-cf70-4df1-903a-c1f6d224e926"), ident_create.column_142.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            None,
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),)
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0_cloned2.clone();
                                    for element in {
                                        let mut acc = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_0.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: element,
                                                column_142: <<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::create_into_postgresql_json_type_option_vec_where_length_equal(ident_create.column_0.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),),
                                                                            None
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                                {
                                    let current_table = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_142_cloned2.clone();
                                    for element in {
                                        let mut acc = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::option_vec_create().unwrap_or(vec![]);
                                        if acc.is_empty() {
                                            acc.push(ident_create_default.column_142.clone());
                                        }
                                        acc
                                    } {
                                        let current_table = current_table.clone();
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            let ident_create = TableExampleCreate {
                                                column_0: <<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlType>::Create as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                column_142: element,
                                            };
                                            let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create.clone() }, &current_table).await.expect("error d6f20011-a88d-44f6-af7f-b2b8eca4c649");
                                            if let Some(value) = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::create_into_postgresql_json_type_option_vec_where_length_equal(ident_create.column_142.clone()) {
                                                for element in value {
                                                    assert_eq!(
                                                        vec![TableExampleRead {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_0.clone()),
                                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.clone().expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create.column_142.clone())
                                                        }],
                                                        TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                        TableExampleWhereMany::try_new(
                                                                            Some(
                                                                                postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                    postgresql_crud::LogicalOperator::And,
                                                                                    vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                                                                        read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                                        postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                                                                                    )]
                                                                                )
                                                                                .expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),
                                                                            ),
                                                                            None,
                                                                            Some(postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::And, vec![element]).expect("error c10cf3d9-f531-442a-99f0-f36c80fee4b1"),)
                                                                        )
                                                                        .expect("error 80a91f82-aeda-4bea-9577-5297b8b3bfb9"),
                                                                    )),
                                                                    select: select_default_all_with_max_page_size_cloned.clone(),
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc)
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error cc5e6038-77f0-499d-bbd6-cc3b45ebe2b6"),
                                                                }
                                                            },
                                                            &current_table
                                                        )
                                                        .await
                                                        .expect("error 91dd4f87-4f0f-4f5a-a844-4161d78dbf4a"),
                                                        "error ee8d232d-98f2-4449-ad30-0e36ca2e7094"
                                                    );
                                                }
                                            }
                                            let read_only_ids_from_try_delete_many = {
                                                let mut acc = TableExample::try_delete_many_handle(
                                                    &url_cloned,
                                                    TableExampleDeleteManyParameters {
                                                        payload: TableExampleDeleteManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                primary_key_column: Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })]
                                                                    })
                                                                    .expect("error 05846791-39e6-4f62-beb0-94f508ed1dad"),
                                                                ),
                                                                column_0: None,
                                                                column_142: None,
                                                            })),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 338bcf89-0c3d-49d7-ac51-b73af98a32b0");
                                                acc.sort();
                                                acc
                                            };
                                            assert_eq!(read_only_ids_from_try_delete_many, vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())], "error 9fc29fa5-caba-403d-99da-ca9107d0c2e9");
                                            match TableExample::try_read_many_handle(
                                                &url_cloned,
                                                TableExampleReadManyParameters {
                                                    payload: TableExampleReadManyPayload {
                                                        where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                            TableExampleWhereMany::try_new(
                                                                Some(
                                                                    postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                        postgresql_crud::LogicalOperator::Or,
                                                                        vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                read_only_ids_returned_from_create_one.primary_key_column.clone(),
                                                                            ))),
                                                                        })],
                                                                    )
                                                                    .expect("error 7f048712-b850-4320-8696-2e4f791f9be6"),
                                                                ),
                                                                None,
                                                                None,
                                                            )
                                                            .expect("error 6770e94a-3716-47b1-ac71-e4d0053e4e4e"),
                                                        )),
                                                        select: select_default_all_with_max_page_size_cloned.clone(),
                                                        order_by: postgresql_crud::OrderBy {
                                                            column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                            order: Some(postgresql_crud::Order::Asc),
                                                        },
                                                        pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 34df6654-6ac6-4734-ba3c-583b8ba4005b"),
                                                    },
                                                },
                                                &current_table,
                                            )
                                            .await
                                            {
                                                Ok(value) => assert!(value == Vec::new(), "error 38187925-c136-41de-940d-eba75efc3a39"),
                                                Err(error) => {
                                                    panic!("error 1817b67a-c6c5-4fea-8ca7-23581c1888a3 {error:#?}");
                                                }
                                            }
                                        }));
                                    }
                                };
                            };
                            {
                                let current_table = table_read_one_cloned2.clone();
                                let url_cloned = url.clone();
                                let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                acc.push(futures::FutureExt::boxed(async move {
                                    if let Err(error) = TableExample::try_read_one_handle(
                                        &url_cloned,
                                        TableExampleReadOneParameters {
                                            payload: TableExampleReadOnePayload {
                                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read::new(uuid::Uuid::new_v4()),
                                                select: select_default_all_with_max_page_size_cloned.clone(),
                                            },
                                        },
                                        &current_table,
                                    )
                                    .await
                                    {
                                        if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                            if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 10010cca-57ec-4620-8ddf-4a3227999b06");
                                            } else {
                                                panic!("error c77029fe-1f95-4df5-a5fb-ef663d7bc08d");
                                            }
                                        } else {
                                            panic!("error 8031870d-aea7-44ef-a91b-1b1ea068e5dd")
                                        }
                                    } else {
                                        panic!("error 9153abfc-f12f-45dd-8d64-52147577f8dd")
                                    }
                                }));
                            };
                            {
                                let current_table = table_update_many_cloned2.clone();
                                {
                                    let read_only_ids_to_two_dimensional_vec_read_inner_acc = {
                                        let mut acc = vec![];
                                        if let Some(value) = &common_read_only_ids_returned_from_create_one.column_0 {
                                            for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(value) {
                                                for _ in element {
                                                    acc.push(ident_create_default.clone());
                                                }
                                            }
                                        }
                                        acc
                                    };
                                    if read_only_ids_to_two_dimensional_vec_read_inner_acc.is_empty() {
                                        println!("PostgresqlTypeTestCases read_only_ids_to_two_dimensional_vec_read_inner is empty for column_0");
                                    } else {
                                        let current_table = current_table.clone();
                                        let read_only_ids_current_elements = {
                                            futures::StreamExt::collect::<Vec<Vec<TableExampleReadOnlyIds>>>(futures::StreamExt::buffer_unordered(
                                                futures::stream::iter(read_only_ids_to_two_dimensional_vec_read_inner_acc.chunks(25).map(Vec::from).map(|element| {
                                                    let current_table = current_table.clone();
                                                    let url_cloned = url.clone();
                                                    futures::FutureExt::boxed(async move { TableExample::try_create_many_handle(&url_cloned, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(element) }, &current_table).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
                                                })),
                                                5,
                                            ))
                                            .await
                                            .into_iter()
                                            .flatten()
                                            .collect::<Vec<TableExampleReadOnlyIds>>()
                                        };
                                        assert_eq!(
                                            {
                                                let mut acc = vec![];
                                                for element in &read_only_ids_current_elements {
                                                    acc.push(TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
                                                        column_0: match &element.column_0 {
                                                            Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                        column_142: match &element.column_142 {
                                                            Some(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                    });
                                                }
                                                acc.sort_by(|first, second| {
                                                    if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                        value_first.value.cmp(&value_second.value)
                                                    } else {
                                                        panic!("must not be what");
                                                    }
                                                });
                                                acc
                                            },
                                            {
                                                let mut acc = TableExample::try_read_many_handle(
                                                    &url,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                            let mut acc = vec![];
                                                                            for element in &read_only_ids_current_elements {
                                                                                acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        element.primary_key_column.clone(),
                                                                                    ))),
                                                                                }));
                                                                            }
                                                                            acc
                                                                        })
                                                                        .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                    ),
                                                                    None,
                                                                    None,
                                                                )
                                                                .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                            )),
                                                            select: select_default_all_with_max_page_size.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc),
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                acc.sort_by(|first, second| if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) { value_first.value.cmp(&value_second.value) } else { panic!("must not be what") });
                                                acc
                                            },
                                            "error 50198a7f-e65c-4e4e-8d7f-9881cfd42453"
                                        );
                                        for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                                            let current_table = table_update_many_cloned2.clone();
                                            let url_cloned = url.clone();
                                            let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                            acc.push(futures::FutureExt::boxed(async move {
                                                let previous_read = {
                                                    let mut acc = TableExample::try_read_many_handle(
                                                        &url_cloned,
                                                        TableExampleReadManyParameters {
                                                            payload: TableExampleReadManyPayload {
                                                                where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                    TableExampleWhereMany::try_new(
                                                                        Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::Or,
                                                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        read_only_ids_current_element.primary_key_column.clone(),
                                                                                    ))),
                                                                                })],
                                                                            )
                                                                            .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                        ),
                                                                        None,
                                                                        None,
                                                                    )
                                                                    .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                                )),
                                                                select: select_default_all_with_max_page_size_cloned.clone(),
                                                                order_by: postgresql_crud::OrderBy {
                                                                    column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                    order: Some(postgresql_crud::Order::Asc),
                                                                },
                                                                pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                            },
                                                        },
                                                        &current_table,
                                                    )
                                                    .await
                                                    .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                    acc.sort_by(|first, second| {
                                                        if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                            value_first.value.cmp(&value_second.value)
                                                        } else {
                                                            panic!("must not be what");
                                                        }
                                                    });
                                                    acc
                                                };
                                                let update = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_inner_into_update_with_new_or_try_new_unwraped({
                                                    let mut local_increment: usize = 0;
                                                    let mut option_test_case = None;
                                                    for element_0 in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&read_only_ids_current_element.column_0.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
                                                        let mut should_break = false;
                                                        for element_1 in element_0 {
                                                            if local_increment == increment {
                                                                option_test_case = Some(element_1);
                                                                should_break = true;
                                                                break;
                                                            }
                                                            local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                                        }
                                                        if should_break {
                                                            break;
                                                        }
                                                    }
                                                    option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                                });
                                                assert_eq!(
                                                    vec![TableExampleReadOnlyIds {
                                                        primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
                                                        column_0: Some(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)),
                                                        column_142: None
                                                    }],
                                                    TableExample::try_update_many_handle(
                                                        &url_cloned,
                                                        TableExampleUpdateManyParameters {
                                                            payload: TableExampleUpdateManyPayload::try_new(vec![
                                                                TableExampleUpdate::try_new(
                                                                    <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_update(read_only_ids_current_element.primary_key_column.clone())),
                                                                    Some(postgresql_crud::Value { value: update.clone() }),
                                                                    None
                                                                )
                                                                .expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")
                                                            ])
                                                            .expect("error 69e1bd8a-fe78-4301-85ca-f4f3958d7493")
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"),
                                                    "error 34bfb3c7-7a53-479e-9d4f-0856003573e1"
                                                );
                                                assert_eq!(
                                                    {
                                                        let mut acc = vec![];
                                                        for element in previous_read {
                                                            acc.push(TableExampleRead {
                                                                primary_key_column: Some(postgresql_crud::Value {
                                                                    value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone()),
                                                                }),
                                                                column_0: Some(postgresql_crud::Value {
                                                                    value: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::previous_read_merged_with_option_update_into_read(
                                                                        <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_0.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129"))
                                                                            .expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5")
                                                                            .value,
                                                                        Some(update.clone()),
                                                                    ),
                                                                }),
                                                                column_142: element.column_142,
                                                            });
                                                        }
                                                        acc
                                                    },
                                                    {
                                                        let mut acc = TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                        primary_key_column: Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::Or,
                                                                                vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        read_only_ids_current_element.primary_key_column.clone(),
                                                                                    ))),
                                                                                })],
                                                                            )
                                                                            .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
                                                                        ),
                                                                        column_0: None,
                                                                        column_142: None,
                                                                    })),
                                                                    select: select_default_all_with_max_page_size_cloned,
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc),
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                                },
                                                            },
                                                            &current_table,
                                                        )
                                                        .await
                                                        .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                        acc.sort_by(|first, second| {
                                                            if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                                value_first.value.cmp(&value_second.value)
                                                            } else {
                                                                panic!("must not be what");
                                                            }
                                                        });
                                                        acc
                                                    },
                                                    "error ae2a2da5-3697-4fd7-9ad2-4a535618fbc3"
                                                );
                                            }));
                                        }
                                    }
                                };
                                {
                                    let read_only_ids_to_two_dimensional_vec_read_inner_acc = {
                                        let mut acc = vec![];
                                        if let Some(value) = &common_read_only_ids_returned_from_create_one.column_142 {
                                            for element in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(value) {
                                                for _ in element {
                                                    acc.push(ident_create_default.clone());
                                                }
                                            }
                                        }
                                        acc
                                    };
                                    if read_only_ids_to_two_dimensional_vec_read_inner_acc.is_empty() {
                                        println!("PostgresqlTypeTestCases read_only_ids_to_two_dimensional_vec_read_inner is empty for column_142");
                                    } else {
                                        let current_table = current_table.clone();
                                        let read_only_ids_current_elements = {
                                            futures::StreamExt::collect::<Vec<Vec<TableExampleReadOnlyIds>>>(futures::StreamExt::buffer_unordered(
                                                futures::stream::iter(read_only_ids_to_two_dimensional_vec_read_inner_acc.chunks(25).map(Vec::from).map(|element| {
                                                    let current_table = current_table.clone();
                                                    let url_cloned = url.clone();
                                                    futures::FutureExt::boxed(async move { TableExample::try_create_many_handle(&url_cloned, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(element) }, &current_table).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
                                                })),
                                                5,
                                            ))
                                            .await
                                            .into_iter()
                                            .flatten()
                                            .collect::<Vec<TableExampleReadOnlyIds>>()
                                        };
                                        assert_eq!(
                                            {
                                                let mut acc = vec![];
                                                for element in &read_only_ids_current_elements {
                                                    acc.push(TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
                                                        column_0: match &element.column_0 {
                                                            Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                        column_142: match &element.column_142 {
                                                            Some(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                    });
                                                }
                                                acc.sort_by(|first, second| {
                                                    if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                        value_first.value.cmp(&value_second.value)
                                                    } else {
                                                        panic!("must not be what");
                                                    }
                                                });
                                                acc
                                            },
                                            {
                                                let mut acc = TableExample::try_read_many_handle(
                                                    &url,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                            let mut acc = vec![];
                                                                            for element in &read_only_ids_current_elements {
                                                                                acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        element.primary_key_column.clone(),
                                                                                    ))),
                                                                                }));
                                                                            }
                                                                            acc
                                                                        })
                                                                        .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                    ),
                                                                    None,
                                                                    None,
                                                                )
                                                                .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                            )),
                                                            select: select_default_all_with_max_page_size.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc),
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                acc.sort_by(|first, second| if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) { value_first.value.cmp(&value_second.value) } else { panic!("must not be what") });
                                                acc
                                            },
                                            "error 50198a7f-e65c-4e4e-8d7f-9881cfd42453"
                                        );
                                        for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                                            let current_table = table_update_many_cloned2.clone();
                                            let url_cloned = url.clone();
                                            let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                            acc.push(futures::FutureExt::boxed(async move {
                                                let previous_read = {
                                                    let mut acc = TableExample::try_read_many_handle(
                                                        &url_cloned,
                                                        TableExampleReadManyParameters {
                                                            payload: TableExampleReadManyPayload {
                                                                where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                    TableExampleWhereMany::try_new(
                                                                        Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::Or,
                                                                                vec![<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        read_only_ids_current_element.primary_key_column.clone(),
                                                                                    ))),
                                                                                })],
                                                                            )
                                                                            .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                        ),
                                                                        None,
                                                                        None,
                                                                    )
                                                                    .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                                )),
                                                                select: select_default_all_with_max_page_size_cloned.clone(),
                                                                order_by: postgresql_crud::OrderBy {
                                                                    column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                    order: Some(postgresql_crud::Order::Asc),
                                                                },
                                                                pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                            },
                                                        },
                                                        &current_table,
                                                    )
                                                    .await
                                                    .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                    acc.sort_by(|first, second| {
                                                        if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                            value_first.value.cmp(&value_second.value)
                                                        } else {
                                                            panic!("must not be what");
                                                        }
                                                    });
                                                    acc
                                                };
                                                let update = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_inner_into_update_with_new_or_try_new_unwraped({
                                                    let mut local_increment: usize = 0;
                                                    let mut option_test_case = None;
                                                    for element_0 in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&read_only_ids_current_element.column_142.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
                                                        let mut should_break = false;
                                                        for element_1 in element_0 {
                                                            if local_increment == increment {
                                                                option_test_case = Some(element_1);
                                                                should_break = true;
                                                                break;
                                                            }
                                                            local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                                        }
                                                        if should_break {
                                                            break;
                                                        }
                                                    }
                                                    option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                                });
                                                assert_eq!(
                                                    vec![TableExampleReadOnlyIds {
                                                        primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
                                                        column_0: None,
                                                        column_142: Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update))
                                                    }],
                                                    TableExample::try_update_many_handle(
                                                        &url_cloned,
                                                        TableExampleUpdateManyParameters {
                                                            payload: TableExampleUpdateManyPayload::try_new(vec![
                                                                TableExampleUpdate::try_new(
                                                                    <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_update(read_only_ids_current_element.primary_key_column.clone())),
                                                                    None,
                                                                    Some(postgresql_crud::Value { value: update.clone() })
                                                                )
                                                                .expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")
                                                            ])
                                                            .expect("error 69e1bd8a-fe78-4301-85ca-f4f3958d7493")
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"),
                                                    "error 34bfb3c7-7a53-479e-9d4f-0856003573e1"
                                                );
                                                assert_eq!(
                                                    {
                                                        let mut acc = vec![];
                                                        for element in previous_read {
                                                            acc.push(TableExampleRead {
                                                                primary_key_column: Some(postgresql_crud::Value {
                                                                    value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone()),
                                                                }),
                                                                column_0: element.column_0,
                                                                column_142: Some(postgresql_crud::Value {
                                                                    value: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::previous_read_merged_with_option_update_into_read(
                                                                        <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_142.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129"))
                                                                            .expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5")
                                                                            .value,
                                                                        Some(update.clone()),
                                                                    ),
                                                                }),
                                                            });
                                                        }
                                                        acc
                                                    },
                                                    {
                                                        let mut acc = TableExample::try_read_many_handle(
                                                            &url_cloned,
                                                            TableExampleReadManyParameters {
                                                                payload: TableExampleReadManyPayload {
                                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                                        primary_key_column: Some(
                                                                            postgresql_crud::PostgresqlTypeWhere::try_new(
                                                                                postgresql_crud::LogicalOperator::Or,
                                                                                vec![postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        read_only_ids_current_element.primary_key_column.clone(),
                                                                                    ))),
                                                                                })],
                                                                            )
                                                                            .expect("error 5f1e5f9d-d189-4368-807e-a84348967610"),
                                                                        ),
                                                                        column_0: None,
                                                                        column_142: None,
                                                                    })),
                                                                    select: select_default_all_with_max_page_size_cloned,
                                                                    order_by: postgresql_crud::OrderBy {
                                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                        order: Some(postgresql_crud::Order::Asc),
                                                                    },
                                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                                },
                                                            },
                                                            &current_table,
                                                        )
                                                        .await
                                                        .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                        acc.sort_by(|first, second| {
                                                            if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                                value_first.value.cmp(&value_second.value)
                                                            } else {
                                                                panic!("must not be what");
                                                            }
                                                        });
                                                        acc
                                                    },
                                                    "error ae2a2da5-3697-4fd7-9ad2-4a535618fbc3"
                                                );
                                            }));
                                        }
                                    }
                                };
                            };
                            {
                                {
                                    let read_only_ids_to_two_dimensional_vec_read_inner_acc = {
                                        let mut acc = vec![];
                                        if let Some(value) = &common_read_only_ids_returned_from_create_one.column_0 {
                                            for element in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(value) {
                                                for _ in element {
                                                    acc.push(ident_create_default.clone());
                                                }
                                            }
                                        }
                                        acc
                                    };
                                    if read_only_ids_to_two_dimensional_vec_read_inner_acc.is_empty() {
                                        println!("PostgresqlTypeTestCases read_only_ids_to_two_dimensional_vec_read_inner is empty for column_0");
                                    } else {
                                        let current_table = table_update_one_cloned2.clone();
                                        let read_only_ids_current_elements = {
                                            futures::StreamExt::collect::<Vec<Vec<TableExampleReadOnlyIds>>>(futures::StreamExt::buffer_unordered(
                                                futures::stream::iter(read_only_ids_to_two_dimensional_vec_read_inner_acc.chunks(25).map(Vec::from).map(|element| {
                                                    let current_table = current_table.clone();
                                                    let url_cloned = url.clone();
                                                    futures::FutureExt::boxed(async move { TableExample::try_create_many_handle(&url_cloned, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(element) }, &current_table).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
                                                })),
                                                5,
                                            ))
                                            .await
                                            .into_iter()
                                            .flatten()
                                            .collect::<Vec<TableExampleReadOnlyIds>>()
                                        };
                                        assert_eq!(
                                            {
                                                let mut acc = vec![];
                                                for element in &read_only_ids_current_elements {
                                                    acc.push(TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
                                                        column_0: match &element.column_0 {
                                                            Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                        column_142: match &element.column_142 {
                                                            Some(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                    });
                                                }
                                                acc.sort_by(|first, second| {
                                                    if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                        value_first.value.cmp(&value_second.value)
                                                    } else {
                                                        panic!("must not be what");
                                                    }
                                                });
                                                acc
                                            },
                                            {
                                                let mut acc = TableExample::try_read_many_handle(
                                                    &url,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                            let mut acc = vec![];
                                                                            for element in &read_only_ids_current_elements {
                                                                                acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        element.primary_key_column.clone(),
                                                                                    ))),
                                                                                }));
                                                                            }
                                                                            acc
                                                                        })
                                                                        .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                    ),
                                                                    None,
                                                                    None,
                                                                )
                                                                .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                            )),
                                                            select: select_default_all_with_max_page_size.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc),
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                acc.sort_by(|first, second| if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) { value_first.value.cmp(&value_second.value) } else { panic!("must not be what") });
                                                acc
                                            },
                                            "error db146190-0496-42a7-93d6-8405eb641954"
                                        );
                                        for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                                            let current_table = table_update_one_cloned2.clone();
                                            let url_cloned = url.clone();
                                            let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                            acc.push(futures::FutureExt::boxed(async move {
                                                let previous_read = TableExample::try_read_one_handle(
                                                    &url_cloned,
                                                    TableExampleReadOneParameters {
                                                        payload: TableExampleReadOnePayload {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone()),
                                                            select: select_default_all_with_max_page_size_cloned.clone(),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                let update = <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_inner_into_update_with_new_or_try_new_unwraped({
                                                    let mut local_increment: usize = 0;
                                                    let mut option_test_case = None;
                                                    for element_0 in <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&read_only_ids_current_element.column_0.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
                                                        let mut should_break = false;
                                                        for element_1 in element_0 {
                                                            if local_increment == increment {
                                                                option_test_case = Some(element_1);
                                                                should_break = true;
                                                                break;
                                                            }
                                                            local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                                        }
                                                        if should_break {
                                                            break;
                                                        }
                                                    }
                                                    option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                                });
                                                assert_eq!(
                                                    TableExampleReadOnlyIds {
                                                        primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
                                                        column_0: Some(<postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update)),
                                                        column_142: None
                                                    },
                                                    TableExample::try_update_one_handle(
                                                        &url_cloned,
                                                        TableExampleUpdateOneParameters {
                                                            payload: TableExampleUpdate::try_new(
                                                                <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_update(read_only_ids_current_element.primary_key_column.clone())),
                                                                Some(postgresql_crud::Value { value: update.clone() }),
                                                                None
                                                            )
                                                            .expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"),
                                                    "error 564de31c-3664-4c62-85fc-e03793372f8f"
                                                );
                                                assert_eq!(
                                                    TableExampleRead {
                                                        primary_key_column: Some(postgresql_crud::Value {
                                                            value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone())
                                                        }),
                                                        column_0: Some(postgresql_crud::Value {
                                                            value: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::previous_read_merged_with_option_update_into_read(
                                                                <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_0.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129"))
                                                                    .expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5")
                                                                    .value,
                                                                Some(update.clone())
                                                            )
                                                        }),
                                                        column_142: previous_read.column_142
                                                    },
                                                    TableExample::try_read_one_handle(
                                                        &url_cloned,
                                                        TableExampleReadOneParameters {
                                                            payload: TableExampleReadOnePayload {
                                                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone()),
                                                                select: select_default_all_with_max_page_size_cloned
                                                            }
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                                                    "error d5dec823-b1f9-49b2-9c24-bf788f08cd8c"
                                                );
                                            }));
                                        }
                                    }
                                };
                                {
                                    let read_only_ids_to_two_dimensional_vec_read_inner_acc = {
                                        let mut acc = vec![];
                                        if let Some(value) = &common_read_only_ids_returned_from_create_one.column_142 {
                                            for element in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(value) {
                                                for _ in element {
                                                    acc.push(ident_create_default.clone());
                                                }
                                            }
                                        }
                                        acc
                                    };
                                    if read_only_ids_to_two_dimensional_vec_read_inner_acc.is_empty() {
                                        println!("PostgresqlTypeTestCases read_only_ids_to_two_dimensional_vec_read_inner is empty for column_142");
                                    } else {
                                        let current_table = table_update_one_cloned2.clone();
                                        let read_only_ids_current_elements = {
                                            futures::StreamExt::collect::<Vec<Vec<TableExampleReadOnlyIds>>>(futures::StreamExt::buffer_unordered(
                                                futures::stream::iter(read_only_ids_to_two_dimensional_vec_read_inner_acc.chunks(25).map(Vec::from).map(|element| {
                                                    let current_table = current_table.clone();
                                                    let url_cloned = url.clone();
                                                    futures::FutureExt::boxed(async move { TableExample::try_create_many_handle(&url_cloned, TableExampleCreateManyParameters { payload: TableExampleCreateManyPayload(element) }, &current_table).await.expect("error 0aedfa07-149b-4028-a131-a64ccdda6b98") })
                                                })),
                                                5,
                                            ))
                                            .await
                                            .into_iter()
                                            .flatten()
                                            .collect::<Vec<TableExampleReadOnlyIds>>()
                                        };
                                        assert_eq!(
                                            {
                                                let mut acc = vec![];
                                                for element in &read_only_ids_current_elements {
                                                    acc.push(TableExampleRead {
                                                        primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column),
                                                        column_0: match &element.column_0 {
                                                            Some(value) => <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                        column_142: match &element.column_142 {
                                                            Some(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(value),
                                                            None => Some(postgresql_crud::Value {
                                                                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                                                            }),
                                                        },
                                                    });
                                                }
                                                acc.sort_by(|first, second| {
                                                    if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) {
                                                        value_first.value.cmp(&value_second.value)
                                                    } else {
                                                        panic!("must not be what");
                                                    }
                                                });
                                                acc
                                            },
                                            {
                                                let mut acc = TableExample::try_read_many_handle(
                                                    &url,
                                                    TableExampleReadManyParameters {
                                                        payload: TableExampleReadManyPayload {
                                                            where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                                TableExampleWhereMany::try_new(
                                                                    Some(
                                                                        postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                            let mut acc = vec![];
                                                                            for element in &read_only_ids_current_elements {
                                                                                acc.push(postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                                    logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                                    value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::into_inner(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(
                                                                                        element.primary_key_column.clone(),
                                                                                    ))),
                                                                                }));
                                                                            }
                                                                            acc
                                                                        })
                                                                        .expect("error f4202d10-5444-4717-8af0-9358ee044c20"),
                                                                    ),
                                                                    None,
                                                                    None,
                                                                )
                                                                .expect("error e594dd1f-4b25-4ac0-9674-82076f8feafb"),
                                                            )),
                                                            select: select_default_all_with_max_page_size.clone(),
                                                            order_by: postgresql_crud::OrderBy {
                                                                column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                                order: Some(postgresql_crud::Order::Asc),
                                                            },
                                                            pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error 8070b103-ef91-4188-b788-b14439b6235a"),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                acc.sort_by(|first, second| if let (Some(value_first), Some(value_second)) = (&first.primary_key_column, &second.primary_key_column) { value_first.value.cmp(&value_second.value) } else { panic!("must not be what") });
                                                acc
                                            },
                                            "error db146190-0496-42a7-93d6-8405eb641954"
                                        );
                                        for (increment, read_only_ids_current_element) in read_only_ids_current_elements.into_iter().enumerate() {
                                            let current_table = table_update_one_cloned2.clone();
                                            let url_cloned = url.clone();
                                            let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                            acc.push(futures::FutureExt::boxed(async move {
                                                let previous_read = TableExample::try_read_one_handle(
                                                    &url_cloned,
                                                    TableExampleReadOneParameters {
                                                        payload: TableExampleReadOnePayload {
                                                            primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone()),
                                                            select: select_default_all_with_max_page_size_cloned.clone(),
                                                        },
                                                    },
                                                    &current_table,
                                                )
                                                .await
                                                .expect("error 35141faa-387c-4302-aa7a-c529966f974b");
                                                let update = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_inner_into_update_with_new_or_try_new_unwraped({
                                                    let mut local_increment: usize = 0;
                                                    let mut option_test_case = None;
                                                    for element_0 in <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_two_dimensional_vec_read_inner(&read_only_ids_current_element.column_142.clone().expect("error c4d98a71-f30f-410e-b410-a75f4672f2f7")) {
                                                        let mut should_break = false;
                                                        for element_1 in element_0 {
                                                            if local_increment == increment {
                                                                option_test_case = Some(element_1);
                                                                should_break = true;
                                                                break;
                                                            }
                                                            local_increment = local_increment.checked_add(1).expect("error 326274d1-199d-4c43-89b3-c61c8ecdfd77");
                                                        }
                                                        if should_break {
                                                            break;
                                                        }
                                                    }
                                                    option_test_case.expect("error bd79056e-bd30-4eda-b913-2afffaf1bfc3")
                                                });
                                                assert_eq!(
                                                    TableExampleReadOnlyIds {
                                                        primary_key_column: read_only_ids_current_element.primary_key_column.clone(),
                                                        column_0: None,
                                                        column_142: Some(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::update_to_read_only_ids(&update))
                                                    },
                                                    TableExample::try_update_one_handle(
                                                        &url_cloned,
                                                        TableExampleUpdateOneParameters {
                                                            payload: TableExampleUpdate::try_new(
                                                                <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Update::from(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_update(read_only_ids_current_element.primary_key_column.clone())),
                                                                None,
                                                                Some(postgresql_crud::Value { value: update.clone() })
                                                            )
                                                            .expect("error 0e5d65a5-12c8-4c48-a24c-0f1fe376ada2")
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error d2de0bd6-1b01-4ef2-b074-a60878241b52"),
                                                    "error 564de31c-3664-4c62-85fc-e03793372f8f"
                                                );
                                                assert_eq!(
                                                    TableExampleRead {
                                                        primary_key_column: Some(postgresql_crud::Value {
                                                            value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone())
                                                        }),
                                                        column_0: previous_read.column_0,
                                                        column_142: Some(postgresql_crud::Value {
                                                            value: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::previous_read_merged_with_option_update_into_read(
                                                                <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&read_only_ids_current_element.column_142.clone().expect("error 4f19d0d2-a23f-4b77-a2bc-c7b04db7a129"))
                                                                    .expect("error c7685b19-9bca-47bc-a3a5-8fc543b174a5")
                                                                    .value,
                                                                Some(update.clone())
                                                            )
                                                        })
                                                    },
                                                    TableExample::try_read_one_handle(
                                                        &url_cloned,
                                                        TableExampleReadOneParameters {
                                                            payload: TableExampleReadOnePayload {
                                                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_current_element.primary_key_column.clone()),
                                                                select: select_default_all_with_max_page_size_cloned
                                                            }
                                                        },
                                                        &current_table
                                                    )
                                                    .await
                                                    .expect("error 35141faa-387c-4302-aa7a-c529966f974b"),
                                                    "error d5dec823-b1f9-49b2-9c24-bf788f08cd8c"
                                                );
                                            }));
                                        }
                                    }
                                };
                            };
                            {
                                {
                                    let current_table = table_test_read_many_by_equal_to_created_primary_keys_cloned2.clone();
                                    async fn generate_test_delete_many_by_non_existent_primary_keys(length: usize, url: &str, select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>, current_table: &str, ident_create_default: TableExampleCreate, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str) {
                                        let read_only_ids_from_try_create_one_default = TableExample::try_create_one_handle(url, TableExampleCreateOneParameters { payload: ident_create_default.clone() }, current_table).await.expect("error 71632985-ec25-4928-aa9e-1e224a7478c1");
                                        match TableExample::try_delete_many_handle(
                                            url,
                                            TableExampleDeleteManyParameters {
                                                payload: TableExampleDeleteManyPayload {
                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                        primary_key_column: Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                let mut acc = vec![];
                                                                for _ in 1..=length {
                                                                    acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                        value: postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration::new(uuid::Uuid::new_v4()),
                                                                    }));
                                                                }
                                                                acc
                                                            })
                                                            .expect("error 7affcad2-0410-4eab-b766-eaf23b133540"),
                                                        ),
                                                        column_0: None,
                                                        column_142: None,
                                                    })),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            Ok(value) => assert!(value.is_empty(), "error 51d14103-5122-4d96-a45c-4dd958ab3adc"),
                                            Err(error) => panic!("error 0d5dec47-8b2e-4f02-909b-3a58b65bc6a5 {error:#?}"),
                                        }
                                        let _: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read = TableExample::try_delete_one_handle(
                                            url,
                                            TableExampleDeleteOneParameters {
                                                payload: TableExampleDeleteOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        .expect("error cc3958f0-1a4a-4440-97c7-ca63611405c5");
                                        if let Err(error) = TableExample::try_read_one_handle(
                                            url,
                                            TableExampleReadOneParameters {
                                                payload: TableExampleReadOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                    select: select_default_all_with_max_page_size.clone(),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                                if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                    assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 99bd4d82-4976-4e1e-8022-543b01221a91");
                                                } else {
                                                    panic!("error 5a86690f-80e2-4dbc-9853-1826f94748bd");
                                                }
                                            } else {
                                                panic!("error d90d6d02-33f7-4886-99df-dee76b83400f")
                                            }
                                        } else {
                                            panic!("error 62e65598-2c1f-4912-b3a2-dccd6e1714a1")
                                        }
                                    }
                                    let lengths = vec![1, 2];
                                    for element in lengths {
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        let current_table = current_table.clone();
                                        let ident_create_default_cloned = ident_create_default.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            generate_test_delete_many_by_non_existent_primary_keys(element, &url_cloned, select_default_all_with_max_page_size_cloned, &current_table, ident_create_default_cloned, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row).await;
                                        }));
                                    }
                                }
                                {
                                    let current_table = table_test_read_many_by_equal_to_created_primary_keys_cloned2.clone();
                                    async fn generate_test_delete_many_by_primary_keys(length: usize, url: &str, select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueEnumVec<TableExampleSelect>, current_table: &str, ident_create_default: TableExampleCreate, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row: &str) {
                                        let read_only_ids_from_try_create_one_default = TableExample::try_create_one_handle(url, TableExampleCreateOneParameters { payload: ident_create_default.clone() }, current_table).await.expect("error 71632985-ec25-4928-aa9e-1e224a7478c1");
                                        let read_only_ids_from_try_create_many = TableExample::try_create_many_handle(
                                            url,
                                            TableExampleCreateManyParameters {
                                                payload: TableExampleCreateManyPayload({
                                                    let mut acc = vec![];
                                                    for _ in 1..=length {
                                                        acc.push(ident_create_default.clone());
                                                    }
                                                    acc
                                                }),
                                            },
                                            current_table,
                                        )
                                        .await
                                        .expect("error b8695890-65fb-469b-a6f9-be481d648eb9");
                                        let read_only_ids_from_try_delete_many = TableExample::try_delete_many_handle(
                                            url,
                                            TableExampleDeleteManyParameters {
                                                payload: TableExampleDeleteManyPayload {
                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(TableExampleWhereMany {
                                                        primary_key_column: Some(
                                                            postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                let mut acc = vec![];
                                                                for element in &read_only_ids_from_try_create_many {
                                                                    acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                        logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                        value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_table_type_declaration(element.primary_key_column.clone()),
                                                                    }));
                                                                }
                                                                acc
                                                            })
                                                            .expect("error 059792c8-a025-45bb-9895-3be8dbbdb6a5"),
                                                        ),
                                                        column_0: None,
                                                        column_142: None,
                                                    })),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        .expect("error b80b91b8-7de1-4ea2-97cf-1987a5f7cc57");
                                        assert_eq!(
                                            read_only_ids_from_try_delete_many,
                                            {
                                                read_only_ids_from_try_create_many
                                                    .iter()
                                                    .map(|element| {
                                                        <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element(&element.primary_key_column)
                                                            .expect("error 3ee5ee86-05dc-4dc8-9262-8ffa1855d5e4")
                                                            .value
                                                    })
                                                    .collect::<Vec<<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>()
                                            },
                                            "error db5e88a6-c75b-421b-acfb-56931b97ba3b"
                                        );
                                        match TableExample::try_read_many_handle(
                                            url,
                                            TableExampleReadManyParameters {
                                                payload: TableExampleReadManyPayload {
                                                    where_many: StdOptionOptionTableExampleWhereMany(Some(
                                                        TableExampleWhereMany::try_new(
                                                            Some(
                                                                postgresql_crud::PostgresqlTypeWhere::try_new(postgresql_crud::LogicalOperator::Or, {
                                                                    let mut acc = vec![];
                                                                    for element in read_only_ids_from_try_delete_many {
                                                                        acc.push(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Where::Equal(postgresql_crud::PostgresqlTypeWhereEqual {
                                                                            logical_operator: postgresql_crud::LogicalOperator::Or,
                                                                            value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_into_table_type_declaration(element),
                                                                        }));
                                                                    }
                                                                    acc
                                                                })
                                                                .expect("error b5d89c37-b41e-49c8-bc50-2872b456b37d"),
                                                            ),
                                                            None,
                                                            None,
                                                        )
                                                        .expect("error 3d716223-4ad8-40fc-99a2-d0de3ea5ca5c"),
                                                    )),
                                                    select: select_default_all_with_max_page_size.clone(),
                                                    order_by: postgresql_crud::OrderBy {
                                                        column: TableExampleSelect::PrimaryKeyColumn(<postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Select::default()),
                                                        order: Some(postgresql_crud::Order::Asc),
                                                    },
                                                    pagination: postgresql_crud::PaginationStartsWithZero::try_new(10000, 0).expect("error e5b2edbc-f2b2-48a0-82b9-02720f721eae"),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            Ok(value) => assert!(value.is_empty(), "error 77f038b0-6f39-4b5b-a402-a1b6142acd0d"),
                                            Err(error) => panic!("error bcb79917-ee81-416e-82a3-f43a823266a3 {error:#?}"),
                                        }
                                        let _: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read = TableExample::try_delete_one_handle(
                                            url,
                                            TableExampleDeleteOneParameters {
                                                payload: TableExampleDeleteOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        .expect("error cc3958f0-1a4a-4440-97c7-ca63611405c5");
                                        if let Err(error) = TableExample::try_read_one_handle(
                                            url,
                                            TableExampleReadOneParameters {
                                                payload: TableExampleReadOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_from_try_create_one_default.primary_key_column.clone()),
                                                    select: select_default_all_with_max_page_size.clone(),
                                                },
                                            },
                                            current_table,
                                        )
                                        .await
                                        {
                                            if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                                if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                    assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 99bd4d82-4976-4e1e-8022-543b01221a91");
                                                } else {
                                                    panic!("error 5a86690f-80e2-4dbc-9853-1826f94748bd");
                                                }
                                            } else {
                                                panic!("error d90d6d02-33f7-4886-99df-dee76b83400f")
                                            }
                                        } else {
                                            panic!("error 62e65598-2c1f-4912-b3a2-dccd6e1714a1")
                                        }
                                    }
                                    let lengths = vec![1, 2];
                                    for element in lengths {
                                        let url_cloned = url.clone();
                                        let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                        let current_table = current_table.clone();
                                        let ident_create_default_cloned = ident_create_default.clone();
                                        acc.push(futures::FutureExt::boxed(async move {
                                            generate_test_delete_many_by_primary_keys(element, &url_cloned, select_default_all_with_max_page_size_cloned, &current_table, ident_create_default_cloned, no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row).await;
                                        }));
                                    }
                                }
                            };
                            {
                                let current_table = table_delete_one_cloned2.clone();
                                let ident_create_default_cloned = ident_create_default.clone();
                                let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size.clone();
                                let url_cloned = url.clone();
                                acc.push(futures::FutureExt::boxed(async move {
                                    if let Err(error) = TableExample::try_delete_one_handle(
                                        &url_cloned,
                                        TableExampleDeleteOneParameters {
                                            payload: TableExampleDeleteOnePayload {
                                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlType>::Read::new(uuid::Uuid::new_v4()),
                                            },
                                        },
                                        &current_table,
                                    )
                                    .await
                                    {
                                        if let TableExampleTryDeleteOneErrorNamed::TableExampleDeleteOneErrorNamedWithSerializeDeserialize { delete_one_error_named_with_serialize_deserialize, .. } = error {
                                            if let TableExampleDeleteOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = delete_one_error_named_with_serialize_deserialize {
                                                assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error c9261bb8-d391-4c4b-9707-3a2c4278ad90");
                                            } else {
                                                panic!("error e63b27a3-f3e3-4f19-998a-88ce798b08cc");
                                            }
                                        } else {
                                            panic!("error 47a8e0d9-1f95-4fa7-91dc-a94955195204")
                                        }
                                    } else {
                                        panic!("error 9be62f9f-31d9-493c-bb0f-b83b6ecb0026")
                                    }
                                    let read_only_ids_returned_from_create_one = TableExample::try_create_one_handle(&url_cloned, TableExampleCreateOneParameters { payload: ident_create_default_cloned.clone() }, &current_table).await.expect("error 8be80909-0e8d-42f9-a5c8-fa08244cb592");
                                    assert_eq!(
                                        TableExampleRead {
                                            primary_key_column: Some(postgresql_crud::Value {
                                                value: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone())
                                            }),
                                            column_0: <postgresql_crud::StdPrimitiveI16AsNotNullInt2 as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_0.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create_default.column_0),
                                            column_142: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(read_only_ids_returned_from_create_one.column_142.expect("error 88038e29-adc7-4e1c-ae5b-609c18831a1b"), ident_create_default.column_142)
                                        },
                                        TableExample::try_read_one_handle(
                                            &url_cloned,
                                            TableExampleReadOneParameters {
                                                payload: TableExampleReadOnePayload {
                                                    primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                    select: select_default_all_with_max_page_size_cloned.clone()
                                                }
                                            },
                                            &current_table
                                        )
                                        .await
                                        .expect("error c8c44c89-aeb0-43d3-ae72-02b7a5979f5a"),
                                        "error 86ef08ae-4356-4417-9490-1d13eb2af71f"
                                    );
                                    let read_only_ids_from_try_delete_one = TableExample::try_delete_one_handle(
                                        &url_cloned,
                                        TableExampleDeleteOneParameters {
                                            payload: TableExampleDeleteOnePayload {
                                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                            },
                                        },
                                        &current_table,
                                    )
                                    .await
                                    .expect("error acab86b7-b199-4732-b8ea-76c00a12abb2");
                                    assert_eq!(read_only_ids_from_try_delete_one, <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()), "error 99f81971-dc80-46db-b466-4f309b215a8c");
                                    if let Err(error) = TableExample::try_read_one_handle(
                                        &url_cloned,
                                        TableExampleReadOneParameters {
                                            payload: TableExampleReadOnePayload {
                                                primary_key_column: <postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::read_only_ids_into_read(read_only_ids_returned_from_create_one.primary_key_column.clone()),
                                                select: select_default_all_with_max_page_size_cloned.clone(),
                                            },
                                        },
                                        &current_table,
                                    )
                                    .await
                                    {
                                        if let TableExampleTryReadOneErrorNamed::TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize, .. } = error {
                                            if let TableExampleReadOneErrorNamedWithSerializeDeserialize::Postgresql { postgresql, .. } = read_one_error_named_with_serialize_deserialize {
                                                assert!(postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row, "error 2c10a1e0-ee7f-4710-9329-5e6ba04a880c");
                                            } else {
                                                panic!("error f6bb9f64-4453-4eb2-9972-7266270b3972");
                                            }
                                        } else {
                                            panic!("error e494cdfd-1f76-4694-a727-9365e1fdf3c6")
                                        }
                                    } else {
                                        panic!("error baa0a952-679c-4485-8360-279eef9982b4")
                                    }
                                }));
                            };
                            acc
                        }),
                        100,
                        async |fut| {
                            fut.await;
                        },
                    )
                    .await;
                    drop_all_test_tables().await;
                });
            })
            .expect("error 4d329978-f5af-424e-8757-e8a32dbeb5a1")
            .join()
            .unwrap_or_else(|error| {
                panic!("error b2f21a5f-d9ce-435c-809f-bd40741c8795 {error:#?}");
            });
    }
}
