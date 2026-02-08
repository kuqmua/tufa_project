#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy)]
# [postgresql_crud :: generate_postgresql_table_config { { "create_many_content_write_into_generate_postgresql_table_create_many" : "False" , "create_one_content_write_into_generate_postgresql_table_create_one" : "False" , "read_many_content_write_into_generate_postgresql_table_read_many" : "False" , "read_one_content_write_into_generate_postgresql_table_read_one" : "False" , "update_many_content_write_into_generate_postgresql_table_update_many" : "False" , "update_one_content_write_into_generate_postgresql_table_update_one" : "False" , "delete_many_content_write_into_generate_postgresql_table_delete_many" : "False" , "delete_one_content_write_into_generate_postgresql_table_delete_one" : "False" , "tests_content_write_into_generate_postgresql_table_tests" : "False" , "common_content_write_into_generate_postgresql_table_common" : "False" , "whole_content_write_into_generate_postgresql_table" : "False" } }]
# [postgresql_crud :: create_many_additional_error_variants { enum CreateManyAdditionalErrorVariants { } }]
# [postgresql_crud :: create_one_additional_error_variants { enum CreateOneAdditionalErrorVariants { } }]
# [postgresql_crud :: read_many_additional_error_variants { enum ReadManyAdditionalErrorVariants { } }]
# [postgresql_crud :: read_one_additional_error_variants { enum ReadOneAdditionalErrorVariants { } }]
# [postgresql_crud :: update_many_additional_error_variants { enum UpdateManyAdditionalErrorVariants { } }]
# [postgresql_crud :: update_one_additional_error_variants { enum UpdateOneAdditionalErrorVariants { } }]
# [postgresql_crud :: delete_many_additional_error_variants { enum DeleteManyAdditionalErrorVariants { } }]
# [postgresql_crud :: delete_one_additional_error_variants { enum DeleteOneAdditionalErrorVariants { } }]
# [postgresql_crud :: common_additional_error_variants { enum CommonAdditionalErrorVariants { CheckCommit { # [eo_error_occurence] check_commit : postgresql_crud :: check_commit :: ErrorNamed , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , } }]
# [postgresql_crud :: create_many_additional_logic { }]
# [postgresql_crud :: create_one_additional_logic { }]
# [postgresql_crud :: read_many_additional_logic { }]
# [postgresql_crud :: read_one_additional_logic { }]
# [postgresql_crud :: update_many_additional_logic { }]
# [postgresql_crud :: update_one_additional_logic { }]
# [postgresql_crud :: delete_many_additional_logic { }]
# [postgresql_crud :: delete_one_additional_logic { }]
# [postgresql_crud :: common_additional_logic { }]
pub struct TableExample {
    pub primary_key_column:
        postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,
    pub column_0: postgresql_crud::StdPrimitiveI16AsNotNullInt2,
    pub column_1: postgresql_crud::OptionStdPrimitiveI16AsNullableInt2,
    pub column_2: postgresql_crud::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2,
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl TableExample {
    #[must_use]
    pub const fn table_name() -> &'static str {
        "table_example"
    }
    const fn primary_key() -> &'static str {
        "primary_key_column"
    }
    pub async fn prepare_extensions(
        pool: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<(), TableExamplePreparePostgresqlErrorNamed> {
        if let Err(error) = sqlx::query("create extension if not exists pg_jsonschema")
            .execute(pool)
            .await
        {
            return Err(
                TableExamplePreparePostgresqlErrorNamed::CreateExtensionIfNotExistsPgJsonschema {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                },
            );
        }
        if let Err(error) = sqlx::query("create extension if not exists \"uuid-ossp\"")
            .execute(pool)
            .await
        {
            return Err(
                TableExamplePreparePostgresqlErrorNamed::CreateExtensionIfNotExistsUuidOssp {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                },
            );
        }
        Ok(())
    }
    pub async fn prepare_postgresql_table(
        pool: &sqlx::Pool<sqlx::Postgres>,
        table: &str,
    ) -> Result<(), TableExamplePreparePostgresqlErrorNamed> {
        if let Err (error) = sqlx :: query (& format ! ("create table if not exists {table} ({},{},{},{})" , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: create_table_column_query_part (& "primary_key_column" , true) , < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: create_table_column_query_part (& "column_0" , false) , < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: create_table_column_query_part (& "column_1" , false) , < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: create_table_column_query_part (& "column_2" , false))) . execute (pool) . await { return Err (TableExamplePreparePostgresqlErrorNamed :: PreparePostgresql { error , code_occurence : error_occurence_lib :: code_occurence ! () }) ; }
        Ok(())
    }
    pub async fn prepare_postgresql(
        pool: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<(), TableExamplePreparePostgresqlErrorNamed> {
        Self::prepare_extensions(pool).await?;
        Self::prepare_postgresql_table(pool, "table_example").await?;
        Ok(())
    }
    #[must_use]
    pub const fn allow_methods() -> [http::Method; 4] {
        [
            http::Method::GET,
            http::Method::POST,
            http::Method::PATCH,
            http::Method::DELETE,
        ]
    }
    fn generate_select_query_part(
        select: &postgresql_crud::NotEmptyUniqueVec<TableExampleSelect>,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc_37c883c3 = String::default();
        for element_78d2ec39 in select.to_vec() {
            acc_37c883c3 . push_str (& match element_78d2ec39 { TableExampleSelect :: PrimaryKeyColumn (column) => match < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: select_query_part (column , "primary_key_column") { Ok (value_820e1163) => value_820e1163 , Err (error) => { return Err (error) ; } } , TableExampleSelect :: Column0 (column) => match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_query_part (column , "column_0") { Ok (value_820e1163) => value_820e1163 , Err (error) => { return Err (error) ; } } , TableExampleSelect :: Column1 (column) => match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: select_query_part (column , "column_1") { Ok (value_820e1163) => value_820e1163 , Err (error) => { return Err (error) ; } } , TableExampleSelect :: Column2 (column) => match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_query_part (column , "column_2") { Ok (value_820e1163) => value_820e1163 , Err (error) => { return Err (error) ; } } }) ;
            acc_37c883c3.push(',');
        }
        let _: Option<char> = acc_37c883c3.pop();
        Ok(acc_37c883c3)
    }
    #[allow(clippy::single_call_fn)]
    async fn create_many_handle(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
        table: &str,
    ) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (value) if value == http :: header :: HeaderValue :: from_static ("application/json"))
        {
            let error =
                TableExampleCreateManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2932,
                            column: 25,
                        }),
                    ),
                };
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TableExampleCreateManyResponseVariants::from_handle(error),
            ));
            *response.status_mut() = http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(
            body,
            *app_state.get_maximum_size_of_http_body_in_bytes(),
        )
        .await
        {
            Ok(value_cfac9140) => value_cfac9140,
            Err(error_0) => {
                let error = TableExampleCreateManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2940,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleCreateManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleCreateManyParameters {
            payload: match serde_json::from_slice::<TableExampleCreateManyPayload>(&body_bytes) {
                Ok(value_9e6fcd2d) => value_9e6fcd2d,
                Err(error_0) => {
                    let error = TableExampleCreateManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3065,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleCreateManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_create_many_query_string(
            table,
            "primary_key_column,column_0,column_1,column_2",
            &{
                let mut increment: u64 = 0;
                let mut acc_8a58994e = String::default();
                for element_1651705d in &parameters.payload.0 {
                    match element_1651705d.create_query_part(&mut increment) {
                        Ok(value_f4fdd10d) => {
                            if {
                                use std::fmt::Write as _;
                                write!(acc_8a58994e, "({value_f4fdd10d}),")
                            }
                            .is_err()
                            {
                                let error_0 =
                                    postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                                        code_occurence: error_occurence_lib::code_occurence!(),
                                    };
                                let error = TableExampleCreateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3365 , column : 21 , })) } ;
                                let mut response =
                                    axum::response::IntoResponse::into_response(axum::Json(
                                        TableExampleCreateManyResponseVariants::from_handle(error),
                                    ));
                                *response.status_mut() = http::StatusCode::BAD_REQUEST;
                                return response;
                            }
                        }
                        Err(error_0) => {
                            let error = TableExampleCreateManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence:
                                    error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from(
                                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                            ),
                                            line: 3419,
                                            column: 200,
                                        }),
                                    ),
                            };
                            let mut response =
                                axum::response::IntoResponse::into_response(axum::Json(
                                    TableExampleCreateManyResponseVariants::from_handle(error),
                                ));
                            *response.status_mut() = http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                let _: Option<char> = acc_8a58994e.pop();
                acc_8a58994e
            },
            &{
                let mut acc_a35168d8 = String::new();
                match < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: select_only_ids_query_part ("primary_key_column") { Ok (value_aa341baf) => { acc_a35168d8 . push_str (& value_aa341baf) ; } , Err (error_0) => { let error = TableExampleCreateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3336 , column : 191 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } }
                match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_ids_query_part ("column_0") { Ok (value_aa341baf) => { acc_a35168d8 . push_str (& value_aa341baf) ; } , Err (error_0) => { let error = TableExampleCreateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3336 , column : 191 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } }
                match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: select_only_ids_query_part ("column_1") { Ok (value_aa341baf) => { acc_a35168d8 . push_str (& value_aa341baf) ; } , Err (error_0) => { let error = TableExampleCreateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3336 , column : 191 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } }
                match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_ids_query_part ("column_2") { Ok (value_aa341baf) => { acc_a35168d8 . push_str (& value_aa341baf) ; } , Err (error_0) => { let error = TableExampleCreateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3336 , column : 191 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } }
                let _: Option<char> = acc_a35168d8.pop();
                acc_a35168d8
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            for element_7f862135 in parameters.payload.0 {
                match element_7f862135.create_query_bind(query) {
                    Ok(value_011a3eb4) => {
                        query = value_011a3eb4;
                    }
                    Err(error_0) => {
                        let error = TableExampleCreateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from(
                                        "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                    ),
                                    line: 3445,
                                    column: 252,
                                }),
                            ),
                        };
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TableExampleCreateManyResponseVariants::from_handle(error),
                        ));
                        *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value_4535ee48) => value_4535ee48,
            Err(error_0) => {
                let error = TableExampleCreateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleCreateManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value_61ae8f84) => value_61ae8f84,
            Err(error_0) => {
                let error = TableExampleCreateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleCreateManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                Ok(value_1aaca28f) => value_1aaca28f,
                Err(error_0) => {
                    let error = TableExampleCreateManyErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 2629,
                                column: 25,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleCreateManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc_d16ac269 = Vec::new();
                while let Some(value_d9cc2c36) = match postgresql_crud::TryStreamExt::try_next(
                    &mut rows,
                )
                .await
                {
                    Ok(value_19f3d6e1) => match value_19f3d6e1 {
                        Some(value_b27d7d79) => Some(
                            match <TableExampleReadOnlyIds as sqlx::FromRow<
                                '_,
                                sqlx::postgres::PgRow,
                            >>::from_row(&value_b27d7d79)
                            {
                                Ok(value_33759463) => value_33759463,
                                Err(error_0) => {
                                    drop(rows);
                                    {
                                        if let Err(error_1) = executor.rollback().await {
                                            let error = TableExampleCreateManyErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3250 , column : 204 , })) } ;
                                            let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateManyResponseVariants :: from_handle (error))) ;
                                            *response.status_mut() =
                                                http::StatusCode::INTERNAL_SERVER_ERROR;
                                            return response;
                                        }
                                        let error = TableExampleCreateManyErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3250 , column : 175 , })) } ;
                                        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateManyResponseVariants :: from_handle (error))) ;
                                        *response.status_mut() =
                                            http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            },
                        ),
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleCreateManyErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3261 , column : 166 , })) } ;
                                let mut response =
                                    axum::response::IntoResponse::into_response(axum::Json(
                                        TableExampleCreateManyResponseVariants::from_handle(error),
                                    ));
                                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleCreateManyErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence:
                                    error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from(
                                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                            ),
                                            line: 3261,
                                            column: 137,
                                        }),
                                    ),
                            };
                            let mut response =
                                axum::response::IntoResponse::into_response(axum::Json(
                                    TableExampleCreateManyResponseVariants::from_handle(error),
                                ));
                            *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                } {
                    acc_d16ac269.push(value_d9cc2c36);
                }
                acc_d16ac269
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleCreateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2647,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleCreateManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TableExampleCreateManyResponseVariants::Desirable(value),
        ));
        *response.status_mut() = http::StatusCode::CREATED;
        response
    }
    #[allow(clippy::single_call_fn)]
    pub async fn create_many(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
    ) -> axum::response::Response {
        Self::create_many_handle(app_state, request, Self::table_name()).await
    }
    #[allow(clippy::single_call_fn)]
    async fn try_create_many_handle(
        endpoint_location: &str,
        parameters: TableExampleCreateManyParameters,
        table: &str,
    ) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryCreateManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value_1772a83e) => value_1772a83e,
                Err(error_0) => {
                    return Err(TableExampleTryCreateManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3090,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/create_many");
        let future = reqwest::Client::new()
            .post(&url)
            .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send();
        let response = match future.await {
            Ok(value_180559e9) => value_180559e9,
            Err(error_0) => {
                return Err(TableExampleTryCreateManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3128,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value_6a62b2b9) => value_6a62b2b9,
            Err(error_2) => {
                return Err(
                    TableExampleTryCreateManyErrorNamed::FailedToGetResponseText {
                        status_code: error_0,
                        headers: error_1,
                        reqwest: error_2,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3145,
                                column: 192,
                            }),
                        ),
                    },
                );
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleCreateManyResponseVariants>(
            &error_2,
        ) {
            Ok(value_563d2a75) => value_563d2a75,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3157,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let create_many_error_named_with_serialize_deserialize = match expected_response { TableExampleCreateManyResponseVariants :: Desirable (value) => { return Ok (value) ; } , TableExampleCreateManyResponseVariants :: CheckBodySize { check_body_size , code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } , TableExampleCreateManyResponseVariants :: Postgresql { postgresql , code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } , TableExampleCreateManyResponseVariants :: SerdeJson { serde_json , code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } , TableExampleCreateManyResponseVariants :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleCreateManyResponseVariants :: CheckCommit { check_commit , code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } , TableExampleCreateManyResponseVariants :: QueryPart { error , code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } , TableExampleCreateManyResponseVariants :: RowAndRollback { row , rollback , code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } , TableExampleCreateManyResponseVariants :: TryBind { try_bind , code_occurence } => TableExampleCreateManyErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } } ;
        Err (TableExampleTryCreateManyErrorNamed :: TableExampleCreateManyErrorNamedWithSerializeDeserialize { create_many_error_named_with_serialize_deserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3194 , column : 175 , })) , })
    }
    pub async fn try_create_many(
        endpoint_location: &str,
        parameters: TableExampleCreateManyParameters,
    ) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryCreateManyErrorNamed> {
        Self::try_create_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    #[must_use]
    pub fn create_many_payload_example() -> axum::response::Response {
        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (< TableExampleCreateManyPayload as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element ())) ;
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    async fn create_one_handle(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
        table: &str,
    ) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (value) if value == http :: header :: HeaderValue :: from_static ("application/json"))
        {
            let error = TableExampleCreateOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from(
                            "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                        ),
                        line: 2932,
                        column: 25,
                    }),
                ),
            };
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TableExampleCreateOneResponseVariants::from_handle(error),
            ));
            *response.status_mut() = http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(
            body,
            *app_state.get_maximum_size_of_http_body_in_bytes(),
        )
        .await
        {
            Ok(value_cfac9140) => value_cfac9140,
            Err(error_0) => {
                let error = TableExampleCreateOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2940,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleCreateOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleCreateOneParameters {
            payload: match serde_json::from_slice::<TableExampleCreate>(&body_bytes) {
                Ok(value_9e6fcd2d) => value_9e6fcd2d,
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3065,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleCreateOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_create_one_query_string(
            table,
            "primary_key_column,column_0,column_1,column_2",
            &match parameters.payload.create_query_part(&mut 0) {
                Ok(value_3267d57d) => value_3267d57d,
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3541,
                                column: 200,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleCreateOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &{
                let mut acc_a35168d8 = String::new();
                match < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: select_only_ids_query_part ("primary_key_column") { Ok (value_aa341baf) => { acc_a35168d8 . push_str (& value_aa341baf) ; } , Err (error_0) => { let error = TableExampleCreateOneErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3336 , column : 191 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } }
                match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_ids_query_part ("column_0") { Ok (value_aa341baf) => { acc_a35168d8 . push_str (& value_aa341baf) ; } , Err (error_0) => { let error = TableExampleCreateOneErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3336 , column : 191 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } }
                match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: select_only_ids_query_part ("column_1") { Ok (value_aa341baf) => { acc_a35168d8 . push_str (& value_aa341baf) ; } , Err (error_0) => { let error = TableExampleCreateOneErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3336 , column : 191 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } }
                match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_ids_query_part ("column_2") { Ok (value_aa341baf) => { acc_a35168d8 . push_str (& value_aa341baf) ; } , Err (error_0) => { let error = TableExampleCreateOneErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3336 , column : 191 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleCreateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } }
                let _: Option<char> = acc_a35168d8.pop();
                acc_a35168d8
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match parameters.payload.create_query_bind(query) {
                Ok(value_06f852cd) => {
                    query = value_06f852cd;
                }
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3559,
                                column: 252,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleCreateOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value_4535ee48) => value_4535ee48,
            Err(error_0) => {
                let error = TableExampleCreateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleCreateOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value_61ae8f84) => value_61ae8f84,
            Err(error_0) => {
                let error = TableExampleCreateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleCreateOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                Ok(value_1aaca28f) => value_1aaca28f,
                Err(error_0) => {
                    let error = TableExampleCreateOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 2629,
                                column: 25,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleCreateOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value_b27d7d79) => {
                        match <TableExampleReadOnlyIds as sqlx::FromRow<
                            '_,
                            sqlx::postgres::PgRow,
                        >>::from_row(&value_b27d7d79)
                        {
                            Ok(value_33759463) => value_33759463,
                            Err(error_0) => {
                                if let Err(error_1) = executor.rollback().await {
                                    let error = TableExampleCreateOneErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3583 , column : 191 , })) } ;
                                    let mut response =
                                        axum::response::IntoResponse::into_response(axum::Json(
                                            TableExampleCreateOneResponseVariants::from_handle(
                                                error,
                                            ),
                                        ));
                                    *response.status_mut() =
                                        http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                let error = TableExampleCreateOneErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3583 , column : 162 , })) } ;
                                let mut response =
                                    axum::response::IntoResponse::into_response(axum::Json(
                                        TableExampleCreateOneResponseVariants::from_handle(error),
                                    ));
                                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                    Err(error_0) => {
                        if let Err(error_1) = executor.rollback().await {
                            let error = TableExampleCreateOneErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence:
                                    error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from(
                                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                            ),
                                            line: 3586,
                                            column: 161,
                                        }),
                                    ),
                            };
                            let mut response =
                                axum::response::IntoResponse::into_response(axum::Json(
                                    TableExampleCreateOneResponseVariants::from_handle(error),
                                ));
                            *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        let error = TableExampleCreateOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from(
                                        "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                    ),
                                    line: 3586,
                                    column: 132,
                                }),
                            ),
                        };
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TableExampleCreateOneResponseVariants::from_handle(error),
                        ));
                        *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2647,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleCreateOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TableExampleCreateOneResponseVariants::Desirable(value),
        ));
        *response.status_mut() = http::StatusCode::CREATED;
        response
    }
    #[allow(clippy::single_call_fn)]
    pub async fn create_one(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
    ) -> axum::response::Response {
        Self::create_one_handle(app_state, request, Self::table_name()).await
    }
    #[allow(clippy::single_call_fn)]
    async fn try_create_one_handle(
        endpoint_location: &str,
        parameters: TableExampleCreateOneParameters,
        table: &str,
    ) -> Result<TableExampleReadOnlyIds, TableExampleTryCreateOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value_1772a83e) => value_1772a83e,
                Err(error_0) => {
                    return Err(TableExampleTryCreateOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3090,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/create_one");
        let future = reqwest::Client::new()
            .post(&url)
            .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send();
        let response = match future.await {
            Ok(value_180559e9) => value_180559e9,
            Err(error_0) => {
                return Err(TableExampleTryCreateOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3128,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value_6a62b2b9) => value_6a62b2b9,
            Err(error_2) => {
                return Err(
                    TableExampleTryCreateOneErrorNamed::FailedToGetResponseText {
                        status_code: error_0,
                        headers: error_1,
                        reqwest: error_2,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3145,
                                column: 192,
                            }),
                        ),
                    },
                );
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleCreateOneResponseVariants>(
            &error_2,
        ) {
            Ok(value_563d2a75) => value_563d2a75,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3157,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let create_one_error_named_with_serialize_deserialize = match expected_response { TableExampleCreateOneResponseVariants :: Desirable (value) => { return Ok (value) ; } , TableExampleCreateOneResponseVariants :: CheckBodySize { check_body_size , code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } , TableExampleCreateOneResponseVariants :: Postgresql { postgresql , code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } , TableExampleCreateOneResponseVariants :: SerdeJson { serde_json , code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } , TableExampleCreateOneResponseVariants :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleCreateOneResponseVariants :: CheckCommit { check_commit , code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } , TableExampleCreateOneResponseVariants :: RowAndRollback { row , rollback , code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } , TableExampleCreateOneResponseVariants :: QueryPart { error , code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } , TableExampleCreateOneResponseVariants :: TryBind { try_bind , code_occurence } => TableExampleCreateOneErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } } ;
        Err (TableExampleTryCreateOneErrorNamed :: TableExampleCreateOneErrorNamedWithSerializeDeserialize { create_one_error_named_with_serialize_deserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3194 , column : 175 , })) , })
    }
    pub async fn try_create_one(
        endpoint_location: &str,
        parameters: TableExampleCreateOneParameters,
    ) -> Result<TableExampleReadOnlyIds, TableExampleTryCreateOneErrorNamed> {
        Self::try_create_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    #[must_use]
    pub fn create_one_payload_example() -> axum::response::Response {
        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (< TableExampleCreate as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element ())) ;
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    async fn read_many_handle(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
        table: &str,
    ) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (value) if value == http :: header :: HeaderValue :: from_static ("application/json"))
        {
            let error = TableExampleReadManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from(
                            "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                        ),
                        line: 2932,
                        column: 25,
                    }),
                ),
            };
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TableExampleReadManyResponseVariants::from_handle(error),
            ));
            *response.status_mut() = http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(
            body,
            *app_state.get_maximum_size_of_http_body_in_bytes(),
        )
        .await
        {
            Ok(value_cfac9140) => value_cfac9140,
            Err(error_0) => {
                let error = TableExampleReadManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2940,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleReadManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleReadManyParameters {
            payload: match serde_json::from_slice::<TableExampleReadManyPayload>(&body_bytes) {
                Ok(value_9e6fcd2d) => value_9e6fcd2d,
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3065,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleReadManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_read_many_query_string(
            table,
            &match Self::generate_select_query_part(&parameters.payload.select) {
                Ok(value_357219fb) => value_357219fb,
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 1144,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleReadManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &{
                let mut increment: u64 = 0;
                let mut additional_parameters =
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                        &parameters.payload.where_many,
                        &mut increment,
                        &"",
                        false,
                    ) {
                        Ok(value_d1627695) => value_d1627695,
                        Err(error_0) => {
                            let error = TableExampleReadManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence:
                                    error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from(
                                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                            ),
                                            line: 1542,
                                            column: 21,
                                        }),
                                    ),
                            };
                            let mut response =
                                axum::response::IntoResponse::into_response(axum::Json(
                                    TableExampleReadManyResponseVariants::from_handle(error),
                                ));
                            *response.status_mut() = http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    };
                let prefix = if additional_parameters.is_empty() {
                    ""
                } else {
                    " "
                };
                if {
                    use std::fmt::Write as _;
                    write!(
                        additional_parameters,
                        "{}order by {} {}",
                        prefix,
                        &match &parameters.payload.order_by.column {
                            TableExampleSelect::PrimaryKeyColumn(_) => "primary_key_column",
                            TableExampleSelect::Column0(_) => "column_0",
                            TableExampleSelect::Column1(_) => "column_1",
                            TableExampleSelect::Column2(_) => "column_2",
                        },
                        parameters.payload.order_by.order.as_ref().map_or_else(
                            || postgresql_crud::Order::default().to_snake_case_stringified(),
                            postgresql_crud::Order::to_snake_case_stringified
                        )
                    )
                }
                .is_err()
                {
                    let error_0 = postgresql_crud::QueryPartErrorNamed::WriteIntoBuffer {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    };
                    let error = TableExampleReadManyErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3365,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleReadManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
                if { use std :: fmt :: Write as _ ; write ! (additional_parameters , "{prefix}{}" , match postgresql_crud :: PostgresqlTypeWhereFilter :: query_part (& parameters . payload . pagination , & mut increment , & "" , bool :: default ()) { Ok (value_742be6cf) => value_742be6cf , Err (error_0) => { let error = TableExampleReadManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3695 , column : 200 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleReadManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } , }) } . is_err () { let error_0 = postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () } ; let error = TableExampleReadManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3365 , column : 21 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleReadManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; }
                additional_parameters
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(
                parameters.payload.where_many,
                query,
            ) {
                Ok(value_03a58371) => {
                    query = value_03a58371;
                }
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 1575,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleReadManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(
                parameters.payload.pagination,
                query,
            ) {
                Ok(value_9f7e487b) => {
                    query = value_9f7e487b;
                }
                Err(error_0) => {
                    let error = TableExampleReadManyErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3760,
                                column: 252,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleReadManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value_4535ee48) => value_4535ee48,
            Err(error_0) => {
                let error = TableExampleReadManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleReadManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value_61ae8f84) => value_61ae8f84,
            Err(error_0) => {
                let error = TableExampleReadManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleReadManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            {
                let mut rows = binded_query.fetch(executor_acquire.as_mut());
                let mut acc_d16ac269 = Vec::new();
                while let Some (value_d9cc2c36) = match postgresql_crud :: TryStreamExt :: try_next (& mut rows) . await { Ok (value_19f3d6e1) => match value_19f3d6e1 { Some (value_b27d7d79) => Some (match TableExampleRead :: try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_table_example_select (& value_b27d7d79 , & parameters . payload . select) { Ok (value_90535a1d) => value_90535a1d , Err (error_0) => { let error = TableExampleReadManyErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 1610 , column : 21 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleReadManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }) , None => None , } , Err (error_0) => { let error = TableExampleReadManyErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3785 , column : 169 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleReadManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } } { acc_d16ac269 . push (value_d9cc2c36) ; }
                acc_d16ac269
            }
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TableExampleReadManyResponseVariants::Desirable(value),
        ));
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    pub async fn read_many(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
    ) -> axum::response::Response {
        Self::read_many_handle(app_state, request, Self::table_name()).await
    }
    #[allow(clippy::single_call_fn)]
    async fn try_read_many_handle(
        endpoint_location: &str,
        parameters: TableExampleReadManyParameters,
        table: &str,
    ) -> Result<Vec<TableExampleRead>, TableExampleTryReadManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value_1772a83e) => value_1772a83e,
                Err(error_0) => {
                    return Err(TableExampleTryReadManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3090,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/read_many");
        let future = reqwest::Client::new()
            .post(&url)
            .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send();
        let response = match future.await {
            Ok(value_180559e9) => value_180559e9,
            Err(error_0) => {
                return Err(TableExampleTryReadManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3128,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value_6a62b2b9) => value_6a62b2b9,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3145,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleReadManyResponseVariants>(
            &error_2,
        ) {
            Ok(value_563d2a75) => value_563d2a75,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3157,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let read_many_error_named_with_serialize_deserialize = match expected_response { TableExampleReadManyResponseVariants :: Desirable (value) => { return Ok (value . into_iter () . fold (Vec :: new () , | mut acc_4adf5a80 , element_6a197212 | { acc_4adf5a80 . push (element_6a197212) ; acc_4adf5a80 })) ; } , TableExampleReadManyResponseVariants :: CheckBodySize { check_body_size , code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } , TableExampleReadManyResponseVariants :: Postgresql { postgresql , code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } , TableExampleReadManyResponseVariants :: SerdeJson { serde_json , code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } , TableExampleReadManyResponseVariants :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleReadManyResponseVariants :: CheckCommit { check_commit , code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } , TableExampleReadManyResponseVariants :: NotUniqueField { not_unique_field , code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize :: NotUniqueField { not_unique_field , code_occurence } , TableExampleReadManyResponseVariants :: QueryPart { error , code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } , TableExampleReadManyResponseVariants :: TryBind { try_bind , code_occurence } => TableExampleReadManyErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } } ;
        Err (TableExampleTryReadManyErrorNamed :: TableExampleReadManyErrorNamedWithSerializeDeserialize { read_many_error_named_with_serialize_deserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3194 , column : 175 , })) , })
    }
    pub async fn try_read_many(
        endpoint_location: &str,
        parameters: TableExampleReadManyParameters,
    ) -> Result<Vec<TableExampleRead>, TableExampleTryReadManyErrorNamed> {
        Self::try_read_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    #[must_use]
    pub fn read_many_payload_example() -> axum::response::Response {
        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (< TableExampleReadManyPayload as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element ())) ;
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    async fn read_one_handle(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
        table: &str,
    ) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (value) if value == http :: header :: HeaderValue :: from_static ("application/json"))
        {
            let error = TableExampleReadOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from(
                            "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                        ),
                        line: 2932,
                        column: 25,
                    }),
                ),
            };
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TableExampleReadOneResponseVariants::from_handle(error),
            ));
            *response.status_mut() = http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(
            body,
            *app_state.get_maximum_size_of_http_body_in_bytes(),
        )
        .await
        {
            Ok(value_cfac9140) => value_cfac9140,
            Err(error_0) => {
                let error = TableExampleReadOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2940,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleReadOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleReadOneParameters {
            payload: match serde_json::from_slice::<TableExampleReadOnePayload>(&body_bytes) {
                Ok(value_9e6fcd2d) => value_9e6fcd2d,
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3065,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleReadOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_read_one_query_string(
            table,
            &match Self::generate_select_query_part(&parameters.payload.select) {
                Ok(value_357219fb) => value_357219fb,
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 1144,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleReadOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            &match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                &parameters.payload.primary_key_column,
                &mut 0,
                &Self::primary_key(),
                false,
            ) {
                Ok(value_be9e7b7d) => value_be9e7b7d,
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3895,
                                column: 200,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleReadOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(
                parameters.payload.primary_key_column,
                query,
            ) {
                Ok(value_80ee6983) => {
                    query = value_80ee6983;
                }
                Err(error_0) => {
                    let error = TableExampleReadOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3914,
                                column: 256,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleReadOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value_4535ee48) => value_4535ee48,
            Err(error_0) => {
                let error = TableExampleReadOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleReadOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value_61ae8f84) => value_61ae8f84,
            Err(error_0) => {
                let error = TableExampleReadOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleReadOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query . fetch_one (executor_acquire . as_mut ()) . await { Ok (value_b27d7d79) => { match TableExampleRead :: try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_table_example_select (& value_b27d7d79 , & parameters . payload . select) { Ok (value_90535a1d) => value_90535a1d , Err (error_0) => { let error = TableExampleReadOneErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 1610 , column : 21 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleReadOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } } } , Err (error_0) => { let error = TableExampleReadOneErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3935 , column : 165 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleReadOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TableExampleReadOneResponseVariants::Desirable(value),
        ));
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    pub async fn read_one(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
    ) -> axum::response::Response {
        Self::read_one_handle(app_state, request, Self::table_name()).await
    }
    #[allow(clippy::single_call_fn)]
    async fn try_read_one_handle(
        endpoint_location: &str,
        parameters: TableExampleReadOneParameters,
        table: &str,
    ) -> Result<TableExampleRead, TableExampleTryReadOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value_1772a83e) => value_1772a83e,
                Err(error_0) => {
                    return Err(TableExampleTryReadOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3090,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/read_one");
        let future = reqwest::Client::new()
            .post(&url)
            .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send();
        let response = match future.await {
            Ok(value_180559e9) => value_180559e9,
            Err(error_0) => {
                return Err(TableExampleTryReadOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3128,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value_6a62b2b9) => value_6a62b2b9,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3145,
                            column: 192,
                        }),
                    ),
                });
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleReadOneResponseVariants>(
            &error_2,
        ) {
            Ok(value_563d2a75) => value_563d2a75,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3157,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let read_one_error_named_with_serialize_deserialize = match expected_response { TableExampleReadOneResponseVariants :: Desirable (value) => { return Ok (value) ; } , TableExampleReadOneResponseVariants :: CheckBodySize { check_body_size , code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } , TableExampleReadOneResponseVariants :: Postgresql { postgresql , code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } , TableExampleReadOneResponseVariants :: SerdeJson { serde_json , code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } , TableExampleReadOneResponseVariants :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleReadOneResponseVariants :: CheckCommit { check_commit , code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } , TableExampleReadOneResponseVariants :: NotUniqueField { not_unique_field , code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize :: NotUniqueField { not_unique_field , code_occurence } , TableExampleReadOneResponseVariants :: QueryPart { error , code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } , TableExampleReadOneResponseVariants :: TryBind { try_bind , code_occurence } => TableExampleReadOneErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } } ;
        Err (TableExampleTryReadOneErrorNamed :: TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3194 , column : 175 , })) , })
    }
    pub async fn try_read_one(
        endpoint_location: &str,
        parameters: TableExampleReadOneParameters,
    ) -> Result<TableExampleRead, TableExampleTryReadOneErrorNamed> {
        Self::try_read_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    #[must_use]
    pub fn read_one_payload_example() -> axum::response::Response {
        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (< TableExampleReadOnePayload as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element ())) ;
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    async fn update_many_handle(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
        table: &str,
    ) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (value) if value == http :: header :: HeaderValue :: from_static ("application/json"))
        {
            let error =
                TableExampleUpdateManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2932,
                            column: 25,
                        }),
                    ),
                };
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TableExampleUpdateManyResponseVariants::from_handle(error),
            ));
            *response.status_mut() = http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(
            body,
            *app_state.get_maximum_size_of_http_body_in_bytes(),
        )
        .await
        {
            Ok(value_cfac9140) => value_cfac9140,
            Err(error_0) => {
                let error = TableExampleUpdateManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2940,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleUpdateManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleUpdateManyParameters {
            payload: match serde_json::from_slice::<TableExampleUpdateManyPayload>(&body_bytes) {
                Ok(value_9e6fcd2d) => value_9e6fcd2d,
                Err(error_0) => {
                    let error = TableExampleUpdateManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3065,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleUpdateManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let update_for_query_vec = parameters
            .payload
            .0
            .into_iter()
            .map(TableExampleUpdateForQuery::from_handle)
            .collect::<Vec<TableExampleUpdateForQuery>>();
        let query_string = {
            let mut increment: u64 = 0;
            let elements = {
                let mut acc_b86a253a = String::default();
                {
                    let mut is_column_0_update_exist = false;
                    for element_a72f3eac in &update_for_query_vec {
                        if element_a72f3eac.column_0.is_some() {
                            is_column_0_update_exist = true;
                            break;
                        }
                    }
                    if is_column_0_update_exist {
                        acc_b86a253a . push_str (& postgresql_crud :: generate_column_equals_case_acc_else_column_end_comma_update_many_query_part ("column_0" , & { let mut acc_8ad06c8c = String :: default () ; for element_defbc401 in & update_for_query_vec { if let Some (value_3ea04126) = & element_defbc401 . column_0 { acc_8ad06c8c . push_str (& postgresql_crud :: generate_when_column_id_then_value_update_many_query_part (Self :: primary_key () , & match element_defbc401 . update_query_part_primary_key (& mut increment) { Ok (value_00890100) => value_00890100 , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4176 , column : 200 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } } , & match TableExampleUpdateForQuery :: update_query_part_column_0 (value_3ea04126 , & mut increment) { Ok (value_8797585c) => value_8797585c , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4176 , column : 200 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } } ,)) ; } } acc_8ad06c8c })) ;
                    }
                }
                {
                    let mut is_column_1_update_exist = false;
                    for element_a72f3eac in &update_for_query_vec {
                        if element_a72f3eac.column_1.is_some() {
                            is_column_1_update_exist = true;
                            break;
                        }
                    }
                    if is_column_1_update_exist {
                        acc_b86a253a . push_str (& postgresql_crud :: generate_column_equals_case_acc_else_column_end_comma_update_many_query_part ("column_1" , & { let mut acc_8ad06c8c = String :: default () ; for element_defbc401 in & update_for_query_vec { if let Some (value_3ea04126) = & element_defbc401 . column_1 { acc_8ad06c8c . push_str (& postgresql_crud :: generate_when_column_id_then_value_update_many_query_part (Self :: primary_key () , & match element_defbc401 . update_query_part_primary_key (& mut increment) { Ok (value_00890100) => value_00890100 , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4176 , column : 200 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } } , & match TableExampleUpdateForQuery :: update_query_part_column_1 (value_3ea04126 , & mut increment) { Ok (value_8797585c) => value_8797585c , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4176 , column : 200 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } } ,)) ; } } acc_8ad06c8c })) ;
                    }
                }
                {
                    let mut is_column_2_update_exist = false;
                    for element_a72f3eac in &update_for_query_vec {
                        if element_a72f3eac.column_2.is_some() {
                            is_column_2_update_exist = true;
                            break;
                        }
                    }
                    if is_column_2_update_exist {
                        acc_b86a253a . push_str (& postgresql_crud :: generate_column_equals_case_acc_else_column_end_comma_update_many_query_part ("column_2" , & { let mut acc_8ad06c8c = String :: default () ; for element_defbc401 in & update_for_query_vec { if let Some (value_3ea04126) = & element_defbc401 . column_2 { acc_8ad06c8c . push_str (& postgresql_crud :: generate_when_column_id_then_value_update_many_query_part (Self :: primary_key () , & match element_defbc401 . update_query_part_primary_key (& mut increment) { Ok (value_00890100) => value_00890100 , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4176 , column : 200 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } } , & match TableExampleUpdateForQuery :: update_query_part_column_2 (value_3ea04126 , & mut increment) { Ok (value_8797585c) => value_8797585c , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4176 , column : 200 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } } ,)) ; } } acc_8ad06c8c })) ;
                    }
                }
                let _: Option<char> = acc_b86a253a.pop();
                acc_b86a253a
            };
            let primary_keys = {
                let mut acc_a95eb175 = String::default();
                for element_9b2b56f8 in &update_for_query_vec {
                    if { use std :: fmt :: Write as _ ; write ! (acc_a95eb175 , "{}," , match element_9b2b56f8 . update_query_part_primary_key (& mut increment) { Ok (value_f269a3b2) => value_f269a3b2 , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 2305 , column : 21 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } }) } . is_err () { let error_0 = postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () } ; let error = TableExampleUpdateManyErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3365 , column : 21 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; }
                }
                let _: Option<char> = acc_a95eb175.pop();
                acc_a95eb175
            };
            let return_columns = {
                let mut acc_fd44b0aa = String::new();
                for element_bcf0dde4 in &update_for_query_vec {
                    match element_bcf0dde4.select_only_updated_ids_query_part(&mut increment) {
                        Ok(value_4f536654) => {
                            acc_fd44b0aa.push_str(&value_4f536654);
                        }
                        Err(error_0) => {
                            let error = TableExampleUpdateManyErrorNamed::QueryPart {
                                error: error_0,
                                code_occurence:
                                    error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from(
                                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                            ),
                                            line: 4176,
                                            column: 200,
                                        }),
                                    ),
                            };
                            let mut response =
                                axum::response::IntoResponse::into_response(axum::Json(
                                    TableExampleUpdateManyResponseVariants::from_handle(error),
                                ));
                            *response.status_mut() = http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                acc_fd44b0aa
            };
            postgresql_crud::generate_update_many_query_string(
                table,
                &elements,
                Self::primary_key(),
                &primary_keys,
                &return_columns,
            )
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            for element_4b24f8f0 in &update_for_query_vec {
                if let Some(value_2edaa480) = &element_4b24f8f0.column_0 {
                    if let Err(error_696908ba) = query.try_bind(element_4b24f8f0.primary_key_column)
                    {
                        let error_0 = error_696908ba.to_string();
                        let error = TableExampleUpdateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from(
                                        "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                    ),
                                    line: 4299,
                                    column: 252,
                                }),
                            ),
                        };
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TableExampleUpdateManyResponseVariants::from_handle(error),
                        ));
                        *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: update_query_bind (value_2edaa480 . value . clone () , query ,) { Ok (value_600e67dc) => { query = value_600e67dc ; } , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4299 , column : 252 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
                }
            }
            for element_4b24f8f0 in &update_for_query_vec {
                if let Some(value_2edaa480) = &element_4b24f8f0.column_1 {
                    if let Err(error_696908ba) = query.try_bind(element_4b24f8f0.primary_key_column)
                    {
                        let error_0 = error_696908ba.to_string();
                        let error = TableExampleUpdateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from(
                                        "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                    ),
                                    line: 4299,
                                    column: 252,
                                }),
                            ),
                        };
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TableExampleUpdateManyResponseVariants::from_handle(error),
                        ));
                        *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: update_query_bind (value_2edaa480 . value . clone () , query ,) { Ok (value_600e67dc) => { query = value_600e67dc ; } , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4299 , column : 252 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
                }
            }
            for element_4b24f8f0 in &update_for_query_vec {
                if let Some(value_2edaa480) = &element_4b24f8f0.column_2 {
                    if let Err(error_696908ba) = query.try_bind(element_4b24f8f0.primary_key_column)
                    {
                        let error_0 = error_696908ba.to_string();
                        let error = TableExampleUpdateManyErrorNamed::TryBind {
                            try_bind: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from(
                                        "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                    ),
                                    line: 4299,
                                    column: 252,
                                }),
                            ),
                        };
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TableExampleUpdateManyResponseVariants::from_handle(error),
                        ));
                        *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: update_query_bind (value_2edaa480 . value . clone () , query ,) { Ok (value_600e67dc) => { query = value_600e67dc ; } , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4299 , column : 252 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
                }
            }
            for element_323f7dfc in &update_for_query_vec {
                match < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: update_query_bind (element_323f7dfc . primary_key_column , query ,) { Ok (value_c40a4522) => { query = value_c40a4522 ; } , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4299 , column : 252 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
            }
            for element_a1660ed1 in &update_for_query_vec {
                if let Some(value_47030ac2) = &element_a1660ed1.column_0 {
                    match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_bind (& value_47030ac2 . value , query) { Ok (value_c5b79b95) => { query = value_c5b79b95 ; } , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4299 , column : 252 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
                }
            }
            for element_a1660ed1 in &update_for_query_vec {
                if let Some(value_47030ac2) = &element_a1660ed1.column_1 {
                    match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_bind (& value_47030ac2 . value , query) { Ok (value_c5b79b95) => { query = value_c5b79b95 ; } , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4299 , column : 252 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
                }
            }
            for element_a1660ed1 in &update_for_query_vec {
                if let Some(value_47030ac2) = &element_a1660ed1.column_2 {
                    match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_bind (& value_47030ac2 . value , query) { Ok (value_c5b79b95) => { query = value_c5b79b95 ; } , Err (error_0) => { let error = TableExampleUpdateManyErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4299 , column : 252 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value_4535ee48) => value_4535ee48,
            Err(error_0) => {
                let error = TableExampleUpdateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleUpdateManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value_61ae8f84) => value_61ae8f84,
            Err(error_0) => {
                let error = TableExampleUpdateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleUpdateManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                Ok(value_1aaca28f) => value_1aaca28f,
                Err(error_0) => {
                    let error = TableExampleUpdateManyErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 2629,
                                column: 25,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleUpdateManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc_d16ac269 = Vec::new();
                while let Some(value_d9cc2c36) = match postgresql_crud::TryStreamExt::try_next(
                    &mut rows,
                )
                .await
                {
                    Ok(value_19f3d6e1) => match value_19f3d6e1 {
                        Some(value_b27d7d79) => Some(
                            match <TableExampleReadOnlyIds as sqlx::FromRow<
                                '_,
                                sqlx::postgres::PgRow,
                            >>::from_row(&value_b27d7d79)
                            {
                                Ok(value_33759463) => value_33759463,
                                Err(error_0) => {
                                    drop(rows);
                                    {
                                        if let Err(error_1) = executor.rollback().await {
                                            let error = TableExampleUpdateManyErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3250 , column : 204 , })) } ;
                                            let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ;
                                            *response.status_mut() =
                                                http::StatusCode::INTERNAL_SERVER_ERROR;
                                            return response;
                                        }
                                        let error = TableExampleUpdateManyErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3250 , column : 175 , })) } ;
                                        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateManyResponseVariants :: from_handle (error))) ;
                                        *response.status_mut() =
                                            http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            },
                        ),
                        None => None,
                    },
                    Err(error_0) => {
                        drop(rows);
                        {
                            if let Err(error_1) = executor.rollback().await {
                                let error = TableExampleUpdateManyErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3261 , column : 166 , })) } ;
                                let mut response =
                                    axum::response::IntoResponse::into_response(axum::Json(
                                        TableExampleUpdateManyResponseVariants::from_handle(error),
                                    ));
                                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                            let error = TableExampleUpdateManyErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence:
                                    error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from(
                                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                            ),
                                            line: 3261,
                                            column: 137,
                                        }),
                                    ),
                            };
                            let mut response =
                                axum::response::IntoResponse::into_response(axum::Json(
                                    TableExampleUpdateManyResponseVariants::from_handle(error),
                                ));
                            *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    }
                } {
                    acc_d16ac269.push(value_d9cc2c36);
                }
                acc_d16ac269
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleUpdateManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2647,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleUpdateManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TableExampleUpdateManyResponseVariants::Desirable(value),
        ));
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    pub async fn update_many(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
    ) -> axum::response::Response {
        Self::update_many_handle(app_state, request, Self::table_name()).await
    }
    #[allow(clippy::single_call_fn)]
    async fn try_update_many_handle(
        endpoint_location: &str,
        parameters: TableExampleUpdateManyParameters,
        table: &str,
    ) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryUpdateManyErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value_1772a83e) => value_1772a83e,
                Err(error_0) => {
                    return Err(TableExampleTryUpdateManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3090,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/update_many");
        let future = reqwest::Client::new()
            .patch(&url)
            .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send();
        let response = match future.await {
            Ok(value_180559e9) => value_180559e9,
            Err(error_0) => {
                return Err(TableExampleTryUpdateManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3128,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value_6a62b2b9) => value_6a62b2b9,
            Err(error_2) => {
                return Err(
                    TableExampleTryUpdateManyErrorNamed::FailedToGetResponseText {
                        status_code: error_0,
                        headers: error_1,
                        reqwest: error_2,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3145,
                                column: 192,
                            }),
                        ),
                    },
                );
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleUpdateManyResponseVariants>(
            &error_2,
        ) {
            Ok(value_563d2a75) => value_563d2a75,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3157,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let update_many_error_named_with_serialize_deserialize = match expected_response { TableExampleUpdateManyResponseVariants :: Desirable (value) => { return Ok (value) ; } , TableExampleUpdateManyResponseVariants :: CheckBodySize { check_body_size , code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } , TableExampleUpdateManyResponseVariants :: Postgresql { postgresql , code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } , TableExampleUpdateManyResponseVariants :: SerdeJson { serde_json , code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } , TableExampleUpdateManyResponseVariants :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleUpdateManyResponseVariants :: CheckCommit { check_commit , code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } , TableExampleUpdateManyResponseVariants :: RowAndRollback { row , rollback , code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } , TableExampleUpdateManyResponseVariants :: QueryPart { error , code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } , TableExampleUpdateManyResponseVariants :: TryBind { try_bind , code_occurence } => TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } } ;
        Err (TableExampleTryUpdateManyErrorNamed :: TableExampleUpdateManyErrorNamedWithSerializeDeserialize { update_many_error_named_with_serialize_deserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3194 , column : 175 , })) , })
    }
    pub async fn try_update_many(
        endpoint_location: &str,
        parameters: TableExampleUpdateManyParameters,
    ) -> Result<Vec<TableExampleReadOnlyIds>, TableExampleTryUpdateManyErrorNamed> {
        Self::try_update_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    #[must_use]
    pub fn update_many_payload_example() -> axum::response::Response {
        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (< TableExampleUpdateManyPayload as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element ())) ;
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    async fn update_one_handle(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
        table: &str,
    ) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (value) if value == http :: header :: HeaderValue :: from_static ("application/json"))
        {
            let error = TableExampleUpdateOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from(
                            "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                        ),
                        line: 2932,
                        column: 25,
                    }),
                ),
            };
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TableExampleUpdateOneResponseVariants::from_handle(error),
            ));
            *response.status_mut() = http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(
            body,
            *app_state.get_maximum_size_of_http_body_in_bytes(),
        )
        .await
        {
            Ok(value_cfac9140) => value_cfac9140,
            Err(error_0) => {
                let error = TableExampleUpdateOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2940,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleUpdateOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleUpdateOneParameters {
            payload: match serde_json::from_slice::<TableExampleUpdate>(&body_bytes) {
                Ok(value_9e6fcd2d) => value_9e6fcd2d,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3065,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleUpdateOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let update_for_query = TableExampleUpdateForQuery::from_handle(parameters.payload);
        let query_string = {
            let mut increment: u64 = 0;
            let columns = {
                let mut acc_683e37b8 = String::default();
                if let Some(value_2d144436) = &update_for_query.column_0 {
                    acc_683e37b8 . push_str (& postgresql_crud :: generate_column_queals_value_comma_update_one_query_part ("column_0" , & match TableExampleUpdateForQuery :: update_query_part_column_0 (value_2d144436 , & mut increment) { Ok (value_1ec12051) => value_1ec12051 , Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4469 , column : 212 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } })) ;
                }
                if let Some(value_2d144436) = &update_for_query.column_1 {
                    acc_683e37b8 . push_str (& postgresql_crud :: generate_column_queals_value_comma_update_one_query_part ("column_1" , & match TableExampleUpdateForQuery :: update_query_part_column_1 (value_2d144436 , & mut increment) { Ok (value_1ec12051) => value_1ec12051 , Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4469 , column : 212 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } })) ;
                }
                if let Some(value_2d144436) = &update_for_query.column_2 {
                    acc_683e37b8 . push_str (& postgresql_crud :: generate_column_queals_value_comma_update_one_query_part ("column_2" , & match TableExampleUpdateForQuery :: update_query_part_column_2 (value_2d144436 , & mut increment) { Ok (value_1ec12051) => value_1ec12051 , Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: QueryPart { error : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4469 , column : 212 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: BAD_REQUEST ; return response ; } })) ;
                }
                let _: Option<char> = acc_683e37b8.pop();
                acc_683e37b8
            };
            let primary_key_query_part = match update_for_query
                .update_query_part_primary_key(&mut increment)
            {
                Ok(value_f269a3b2) => value_f269a3b2,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 2305,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleUpdateOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            };
            let return_columns = match update_for_query
                .select_only_updated_ids_query_part(&mut increment)
            {
                Ok(value_7f0d86a1) => value_7f0d86a1,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::QueryPart {
                        error: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 4496,
                                column: 200,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleUpdateOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            };
            postgresql_crud::generate_update_one_query_string(
                table,
                &columns,
                Self::primary_key(),
                &primary_key_query_part,
                &return_columns,
            )
        };
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            if let Some(value_ed87c152) = &update_for_query.column_0 {
                match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: update_query_bind (value_ed87c152 . value . clone () , query) { Ok (value_c3c1b857) => { query = value_c3c1b857 ; } Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4524 , column : 198 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
            }
            if let Some(value_ed87c152) = &update_for_query.column_1 {
                match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: update_query_bind (value_ed87c152 . value . clone () , query) { Ok (value_c3c1b857) => { query = value_c3c1b857 ; } Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4524 , column : 198 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
            }
            if let Some(value_ed87c152) = &update_for_query.column_2 {
                match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: update_query_bind (value_ed87c152 . value . clone () , query) { Ok (value_c3c1b857) => { query = value_c3c1b857 ; } Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4524 , column : 198 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
            }
            match < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: update_query_bind (update_for_query . primary_key_column , query ,) { Ok (value_d64bac39) => { query = value_d64bac39 ; } , Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4524 , column : 198 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
            if let Some(value_b2902425) = &update_for_query.column_0 {
                match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_bind (& value_b2902425 . value , query) { Ok (value_cc6145f8) => { query = value_cc6145f8 ; } , Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4524 , column : 198 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
            }
            if let Some(value_b2902425) = &update_for_query.column_1 {
                match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_bind (& value_b2902425 . value , query) { Ok (value_cc6145f8) => { query = value_cc6145f8 ; } , Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4524 , column : 198 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
            }
            if let Some(value_b2902425) = &update_for_query.column_2 {
                match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_bind (& value_b2902425 . value , query) { Ok (value_cc6145f8) => { query = value_cc6145f8 ; } , Err (error_0) => { let error = TableExampleUpdateOneErrorNamed :: TryBind { try_bind : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4524 , column : 198 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleUpdateOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value_4535ee48) => value_4535ee48,
            Err(error_0) => {
                let error = TableExampleUpdateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleUpdateOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value_61ae8f84) => value_61ae8f84,
            Err(error_0) => {
                let error = TableExampleUpdateOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleUpdateOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                Ok(value_1aaca28f) => value_1aaca28f,
                Err(error_0) => {
                    let error = TableExampleUpdateOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 2629,
                                column: 25,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleUpdateOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query.fetch_one(executor.as_mut()).await {
                    Ok(value_b27d7d79) => {
                        match <TableExampleReadOnlyIds as sqlx::FromRow<
                            '_,
                            sqlx::postgres::PgRow,
                        >>::from_row(&value_b27d7d79)
                        {
                            Ok(value_33759463) => value_33759463,
                            Err(error_0) => {
                                if let Err(error_1) = executor.rollback().await {
                                    let error = TableExampleUpdateOneErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4602 , column : 165 , })) } ;
                                    let mut response =
                                        axum::response::IntoResponse::into_response(axum::Json(
                                            TableExampleUpdateOneResponseVariants::from_handle(
                                                error,
                                            ),
                                        ));
                                    *response.status_mut() =
                                        http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                let error = TableExampleUpdateOneErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 4602 , column : 136 , })) } ;
                                let mut response =
                                    axum::response::IntoResponse::into_response(axum::Json(
                                        TableExampleUpdateOneResponseVariants::from_handle(error),
                                    ));
                                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                                return response;
                            }
                        }
                    }
                    Err(error_0) => {
                        if let Err(error_1) = executor.rollback().await {
                            let error = TableExampleUpdateOneErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence:
                                    error_occurence_lib::code_occurence::CodeOccurence::new(
                                        file!().to_owned(),
                                        line!(),
                                        column!(),
                                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                                            file: String::from(
                                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                            ),
                                            line: 4604,
                                            column: 161,
                                        }),
                                    ),
                            };
                            let mut response =
                                axum::response::IntoResponse::into_response(axum::Json(
                                    TableExampleUpdateOneResponseVariants::from_handle(error),
                                ));
                            *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        let error = TableExampleUpdateOneErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from(
                                        "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                    ),
                                    line: 4604,
                                    column: 132,
                                }),
                            ),
                        };
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TableExampleUpdateOneResponseVariants::from_handle(error),
                        ));
                        *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2647,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleUpdateOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TableExampleUpdateOneResponseVariants::Desirable(value),
        ));
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    pub async fn update_one(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
    ) -> axum::response::Response {
        Self::update_one_handle(app_state, request, Self::table_name()).await
    }
    #[allow(clippy::single_call_fn)]
    async fn try_update_one_handle(
        endpoint_location: &str,
        parameters: TableExampleUpdateOneParameters,
        table: &str,
    ) -> Result<TableExampleReadOnlyIds, TableExampleTryUpdateOneErrorNamed> {
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value_1772a83e) => value_1772a83e,
                Err(error_0) => {
                    return Err(TableExampleTryUpdateOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3090,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/update_one");
        let future = reqwest::Client::new()
            .patch(&url)
            .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send();
        let response = match future.await {
            Ok(value_180559e9) => value_180559e9,
            Err(error_0) => {
                return Err(TableExampleTryUpdateOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3128,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value_6a62b2b9) => value_6a62b2b9,
            Err(error_2) => {
                return Err(
                    TableExampleTryUpdateOneErrorNamed::FailedToGetResponseText {
                        status_code: error_0,
                        headers: error_1,
                        reqwest: error_2,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3145,
                                column: 192,
                            }),
                        ),
                    },
                );
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleUpdateOneResponseVariants>(
            &error_2,
        ) {
            Ok(value_563d2a75) => value_563d2a75,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3157,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let update_one_error_named_with_serialize_deserialize = match expected_response { TableExampleUpdateOneResponseVariants :: Desirable (value) => { return Ok (value) ; } , TableExampleUpdateOneResponseVariants :: CheckBodySize { check_body_size , code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } , TableExampleUpdateOneResponseVariants :: Postgresql { postgresql , code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } , TableExampleUpdateOneResponseVariants :: SerdeJson { serde_json , code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } , TableExampleUpdateOneResponseVariants :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleUpdateOneResponseVariants :: CheckCommit { check_commit , code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } , TableExampleUpdateOneResponseVariants :: RowAndRollback { row , rollback , code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } , TableExampleUpdateOneResponseVariants :: QueryPart { error , code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } , TableExampleUpdateOneResponseVariants :: TryBind { try_bind , code_occurence } => TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } } ;
        Err (TableExampleTryUpdateOneErrorNamed :: TableExampleUpdateOneErrorNamedWithSerializeDeserialize { update_one_error_named_with_serialize_deserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3194 , column : 175 , })) , })
    }
    pub async fn try_update_one(
        endpoint_location: &str,
        parameters: TableExampleUpdateOneParameters,
    ) -> Result<TableExampleReadOnlyIds, TableExampleTryUpdateOneErrorNamed> {
        Self::try_update_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    #[must_use]
    pub fn update_one_payload_example() -> axum::response::Response {
        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (< TableExampleUpdate as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element ())) ;
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    async fn delete_many_handle(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
        table: &str,
    ) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (value) if value == http :: header :: HeaderValue :: from_static ("application/json"))
        {
            let error =
                TableExampleDeleteManyErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2932,
                            column: 25,
                        }),
                    ),
                };
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TableExampleDeleteManyResponseVariants::from_handle(error),
            ));
            *response.status_mut() = http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(
            body,
            *app_state.get_maximum_size_of_http_body_in_bytes(),
        )
        .await
        {
            Ok(value_cfac9140) => value_cfac9140,
            Err(error_0) => {
                let error = TableExampleDeleteManyErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2940,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleDeleteManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleDeleteManyParameters {
            payload: match serde_json::from_slice::<TableExampleDeleteManyPayload>(&body_bytes) {
                Ok(value_9e6fcd2d) => value_9e6fcd2d,
                Err(error_0) => {
                    let error = TableExampleDeleteManyErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3065,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleDeleteManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string = postgresql_crud::generate_delete_many_query_string(
            table,
            &{
                let mut increment: u64 = 0;
                match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                    &parameters.payload.where_many,
                    &mut increment,
                    &"",
                    false,
                ) {
                    Ok(value_d1627695) => value_d1627695,
                    Err(error_0) => {
                        let error = TableExampleDeleteManyErrorNamed::QueryPart {
                            error: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: String::from(
                                        "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                    ),
                                    line: 1542,
                                    column: 21,
                                }),
                            ),
                        };
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(
                            TableExampleDeleteManyResponseVariants::from_handle(error),
                        ));
                        *response.status_mut() = http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            },
            Self::primary_key(),
        );
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(
                parameters.payload.where_many,
                query,
            ) {
                Ok(value_03a58371) => {
                    query = value_03a58371;
                }
                Err(error_0) => {
                    let error = TableExampleDeleteManyErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 1575,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleDeleteManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value_4535ee48) => value_4535ee48,
            Err(error_0) => {
                let error = TableExampleDeleteManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleDeleteManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value_61ae8f84) => value_61ae8f84,
            Err(error_0) => {
                let error = TableExampleDeleteManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleDeleteManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                Ok(value_1aaca28f) => value_1aaca28f,
                Err(error_0) => {
                    let error = TableExampleDeleteManyErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 2629,
                                column: 25,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleDeleteManyResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                let mut rows = binded_query.fetch(executor.as_mut());
                let mut acc_d16ac269 = Vec::new();
                while let Some (value_d9cc2c36) = match postgresql_crud :: TryStreamExt :: try_next (& mut rows) . await { Ok (value_19f3d6e1) => match value_19f3d6e1 { Some (value_b27d7d79) => match sqlx :: Row :: try_get :: < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read , & str > (& value_b27d7d79 , Self :: primary_key ()) { Ok (value_69ecb6a9) => Some (value_69ecb6a9) , Err (error_0) => { drop (rows) ; { if let Err (error_1) = executor . rollback () . await { let error = TableExampleDeleteManyErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3258 , column : 174 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleDeleteManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } let error = TableExampleDeleteManyErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3258 , column : 145 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleDeleteManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } } } , None => None , } , Err (error_0) => { drop (rows) ; { if let Err (error_1) = executor . rollback () . await { let error = TableExampleDeleteManyErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3261 , column : 166 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleDeleteManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } let error = TableExampleDeleteManyErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3261 , column : 137 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleDeleteManyResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } } } { acc_d16ac269 . push (value_d9cc2c36) ; }
                acc_d16ac269
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleDeleteManyErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2647,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleDeleteManyResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TableExampleDeleteManyResponseVariants::Desirable(value),
        ));
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    pub async fn delete_many(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
    ) -> axum::response::Response {
        Self::delete_many_handle(app_state, request, Self::table_name()).await
    }
    #[allow(clippy::single_call_fn)]    async fn try_delete_many_handle (endpoint_location : & str , parameters : TableExampleDeleteManyParameters , table : & str ,) -> Result < Vec < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read > , TableExampleTryDeleteManyErrorNamed >{
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value_1772a83e) => value_1772a83e,
                Err(error_0) => {
                    return Err(TableExampleTryDeleteManyErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3090,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/delete_many");
        let future = reqwest::Client::new()
            .delete(&url)
            .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send();
        let response = match future.await {
            Ok(value_180559e9) => value_180559e9,
            Err(error_0) => {
                return Err(TableExampleTryDeleteManyErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3128,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value_6a62b2b9) => value_6a62b2b9,
            Err(error_2) => {
                return Err(
                    TableExampleTryDeleteManyErrorNamed::FailedToGetResponseText {
                        status_code: error_0,
                        headers: error_1,
                        reqwest: error_2,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3145,
                                column: 192,
                            }),
                        ),
                    },
                );
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleDeleteManyResponseVariants>(
            &error_2,
        ) {
            Ok(value_563d2a75) => value_563d2a75,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3157,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let delete_many_error_named_with_serialize_deserialize = match expected_response { TableExampleDeleteManyResponseVariants :: Desirable (value) => { return Ok (value) ; } , TableExampleDeleteManyResponseVariants :: CheckBodySize { check_body_size , code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } , TableExampleDeleteManyResponseVariants :: Postgresql { postgresql , code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } , TableExampleDeleteManyResponseVariants :: SerdeJson { serde_json , code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } , TableExampleDeleteManyResponseVariants :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleDeleteManyResponseVariants :: CheckCommit { check_commit , code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } , TableExampleDeleteManyResponseVariants :: RowAndRollback { row , rollback , code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } , TableExampleDeleteManyResponseVariants :: QueryPart { error , code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } , TableExampleDeleteManyResponseVariants :: TryBind { try_bind , code_occurence } => TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } } ;
        Err (TableExampleTryDeleteManyErrorNamed :: TableExampleDeleteManyErrorNamedWithSerializeDeserialize { delete_many_error_named_with_serialize_deserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3194 , column : 175 , })) , })
    }    pub async fn try_delete_many (endpoint_location : & str , parameters : TableExampleDeleteManyParameters) -> Result < Vec < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read > , TableExampleTryDeleteManyErrorNamed >{
        Self::try_delete_many_handle(endpoint_location, parameters, Self::table_name()).await
    }
    #[must_use]
    pub fn delete_many_payload_example() -> axum::response::Response {
        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (< TableExampleDeleteManyPayload as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element ())) ;
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    async fn delete_one_handle(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
        table: &str,
    ) -> axum::response::Response {
        let (parts, body) = request.into_parts();
        let headers = parts.headers;
        if !matches ! (headers . get (http :: header :: CONTENT_TYPE) , Some (value) if value == http :: header :: HeaderValue :: from_static ("application/json"))
        {
            let error = TableExampleDeleteOneErrorNamed::HeaderContentTypeApplicationJsonNotFound {
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: String::from(
                            "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                        ),
                        line: 2932,
                        column: 25,
                    }),
                ),
            };
            let mut response = axum::response::IntoResponse::into_response(axum::Json(
                TableExampleDeleteOneResponseVariants::from_handle(error),
            ));
            *response.status_mut() = http::StatusCode::BAD_REQUEST;
            return response;
        }
        let body_bytes = match postgresql_crud::check_body_size::check_body_size(
            body,
            *app_state.get_maximum_size_of_http_body_in_bytes(),
        )
        .await
        {
            Ok(value_cfac9140) => value_cfac9140,
            Err(error_0) => {
                let error = TableExampleDeleteOneErrorNamed::CheckBodySize {
                    check_body_size: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2940,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleDeleteOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::BAD_REQUEST;
                return response;
            }
        };
        let parameters = TableExampleDeleteOneParameters {
            payload: match serde_json::from_slice::<TableExampleDeleteOnePayload>(&body_bytes) {
                Ok(value_9e6fcd2d) => value_9e6fcd2d,
                Err(error_0) => {
                    let error = TableExampleDeleteOneErrorNamed::SerdeJson {
                        serde_json: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3065,
                                column: 21,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleDeleteOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
        };
        let query_string =
            postgresql_crud::generate_delete_one_query_string(table, Self::primary_key());
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(
                parameters.payload.primary_key_column,
                query,
            ) {
                Ok(value_3099ea0f) => {
                    query = value_3099ea0f;
                }
                Err(error_0) => {
                    let error = TableExampleDeleteOneErrorNamed::TryBind {
                        try_bind: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 4803,
                                column: 198,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleDeleteOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value_4535ee48) => value_4535ee48,
            Err(error_0) => {
                let error = TableExampleDeleteOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleDeleteOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let executor_acquire = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value_61ae8f84) => value_61ae8f84,
            Err(error_0) => {
                let error = TableExampleDeleteOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2996,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleDeleteOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            let mut executor = match sqlx::Acquire::begin(executor_acquire).await {
                Ok(value_1aaca28f) => value_1aaca28f,
                Err(error_0) => {
                    let error = TableExampleDeleteOneErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 2629,
                                column: 25,
                            }),
                        ),
                    };
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(
                        TableExampleDeleteOneResponseVariants::from_handle(error),
                    ));
                    *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            };
            let value = {
                match binded_query . fetch_one (executor . as_mut ()) . await { Ok (value_b27d7d79) => { match sqlx :: Row :: try_get :: < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read , & str > (& value_b27d7d79 , Self :: primary_key ()) { Ok (value_69ecb6a9) => value_69ecb6a9 , Err (error_0) => { { if let Err (error_1) = executor . rollback () . await { let error = TableExampleDeleteOneErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3279 , column : 25 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleDeleteOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } let error = TableExampleDeleteOneErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3276 , column : 25 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleDeleteOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } } } } , Err (error_0) => { { if let Err (error_1) = executor . rollback () . await { let error = TableExampleDeleteOneErrorNamed :: RowAndRollback { row : error_0 , rollback : error_1 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3289 , column : 21 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleDeleteOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } let error = TableExampleDeleteOneErrorNamed :: Postgresql { postgresql : error_0 , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3286 , column : 21 , })) } ; let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (TableExampleDeleteOneResponseVariants :: from_handle (error))) ; * response . status_mut () = http :: StatusCode :: INTERNAL_SERVER_ERROR ; return response ; } } }
            };
            if let Err(error_0) = executor.commit().await {
                let error = TableExampleDeleteOneErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 2647,
                            column: 25,
                        }),
                    ),
                };
                let mut response = axum::response::IntoResponse::into_response(axum::Json(
                    TableExampleDeleteOneResponseVariants::from_handle(error),
                ));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
            value
        };
        let mut response = axum::response::IntoResponse::into_response(axum::Json(
            TableExampleDeleteOneResponseVariants::Desirable(value),
        ));
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    pub async fn delete_one(
        app_state: axum::extract::State<
            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        >,
        request: axum::extract::Request,
    ) -> axum::response::Response {
        Self::delete_one_handle(app_state, request, Self::table_name()).await
    }
    #[allow(clippy::single_call_fn)]    async fn try_delete_one_handle (endpoint_location : & str , parameters : TableExampleDeleteOneParameters , table : & str ,) -> Result < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read , TableExampleTryDeleteOneErrorNamed >{
        let payload = {
            match serde_json::to_string(&parameters.payload) {
                Ok(value_1772a83e) => value_1772a83e,
                Err(error_0) => {
                    return Err(TableExampleTryDeleteOneErrorNamed::SerdeJsonToString {
                        serde_json_to_string: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3090,
                                column: 178,
                            }),
                        ),
                    });
                }
            }
        };
        let url = format!("{endpoint_location}/{table}/delete_one");
        let future = reqwest::Client::new()
            .delete(&url)
            .header(&"commit".to_owned(), git_info::PROJECT_GIT_INFO.commit)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send();
        let response = match future.await {
            Ok(value_180559e9) => value_180559e9,
            Err(error_0) => {
                return Err(TableExampleTryDeleteOneErrorNamed::Reqwest {
                    reqwest: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3128,
                            column: 152,
                        }),
                    ),
                });
            }
        };
        let error_0 = response.status();
        let error_1 = response.headers().clone();
        let error_2 = match response.text().await {
            Ok(value_6a62b2b9) => value_6a62b2b9,
            Err(error_2) => {
                return Err(
                    TableExampleTryDeleteOneErrorNamed::FailedToGetResponseText {
                        status_code: error_0,
                        headers: error_1,
                        reqwest: error_2,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: String::from(
                                    "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                                ),
                                line: 3145,
                                column: 192,
                            }),
                        ),
                    },
                );
            }
        };
        let expected_response = match serde_json::from_str::<TableExampleDeleteOneResponseVariants>(
            &error_2,
        ) {
            Ok(value_563d2a75) => value_563d2a75,
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
                            file: String::from(
                                "postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs",
                            ),
                            line: 3157,
                            column: 178,
                        }),
                    ),
                });
            }
        };
        let delete_one_error_named_with_serialize_deserialize = match expected_response { TableExampleDeleteOneResponseVariants :: Desirable (value) => { return Ok (value) ; } , TableExampleDeleteOneResponseVariants :: CheckBodySize { check_body_size , code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } , TableExampleDeleteOneResponseVariants :: Postgresql { postgresql , code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } , TableExampleDeleteOneResponseVariants :: SerdeJson { serde_json , code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } , TableExampleDeleteOneResponseVariants :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleDeleteOneResponseVariants :: CheckCommit { check_commit , code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } , TableExampleDeleteOneResponseVariants :: RowAndRollback { row , rollback , code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } , TableExampleDeleteOneResponseVariants :: TryBind { try_bind , code_occurence } => TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } } ;
        Err (TableExampleTryDeleteOneErrorNamed :: TableExampleDeleteOneErrorNamedWithSerializeDeserialize { delete_one_error_named_with_serialize_deserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence :: new (file ! () . to_owned () , line ! () , column ! () , Some (error_occurence_lib :: code_occurence :: MacroOccurence { file : String :: from ("postgresql_crud/postgresql_table/generate_postgresql_table_source/src/lib.rs") , line : 3194 , column : 175 , })) , })
    }    pub async fn try_delete_one (endpoint_location : & str , parameters : TableExampleDeleteOneParameters) -> Result < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read , TableExampleTryDeleteOneErrorNamed >{
        Self::try_delete_one_handle(endpoint_location, parameters, Self::table_name()).await
    }
    #[must_use]
    pub fn delete_one_payload_example() -> axum::response::Response {
        let mut response = axum :: response :: IntoResponse :: into_response (axum :: Json (< TableExampleDeleteOnePayload as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element ())) ;
        *response.status_mut() = http::StatusCode::OK;
        response
    }
    #[allow(clippy::single_call_fn)]
    fn routes_handle(
        app_state: std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
        table: &str,
    ) -> axum::Router {
        axum::Router::new().nest(
            &format!("/{table}"),
            axum::Router::new()
                .route(
                    "/create_many",
                    axum::routing::post({
                        let table_owned = table.to_owned();
                        async move |app_state_99328dfe: axum::extract::State<
                            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
                        >,
                                    request: axum::extract::Request| {
                            Self::create_many_handle(app_state_99328dfe, request, &table_owned)
                                .await
                        }
                    }),
                )
                .route(
                    "/create_many_payload_example",
                    axum::routing::get(async move || Self::create_many_payload_example()),
                )
                .route(
                    "/create_one",
                    axum::routing::post({
                        let table_owned = table.to_owned();
                        async move |app_state_99328dfe: axum::extract::State<
                            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
                        >,
                                    request: axum::extract::Request| {
                            Self::create_one_handle(app_state_99328dfe, request, &table_owned).await
                        }
                    }),
                )
                .route(
                    "/create_one_payload_example",
                    axum::routing::get(async move || Self::create_one_payload_example()),
                )
                .route(
                    "/read_many",
                    axum::routing::post({
                        let table_owned = table.to_owned();
                        async move |app_state_99328dfe: axum::extract::State<
                            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
                        >,
                                    request: axum::extract::Request| {
                            Self::read_many_handle(app_state_99328dfe, request, &table_owned).await
                        }
                    }),
                )
                .route(
                    "/read_many_payload_example",
                    axum::routing::get(async move || Self::read_many_payload_example()),
                )
                .route(
                    "/read_one",
                    axum::routing::post({
                        let table_owned = table.to_owned();
                        async move |app_state_99328dfe: axum::extract::State<
                            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
                        >,
                                    request: axum::extract::Request| {
                            Self::read_one_handle(app_state_99328dfe, request, &table_owned).await
                        }
                    }),
                )
                .route(
                    "/read_one_payload_example",
                    axum::routing::get(async move || Self::read_one_payload_example()),
                )
                .route(
                    "/update_many",
                    axum::routing::patch({
                        let table_owned = table.to_owned();
                        async move |app_state_99328dfe: axum::extract::State<
                            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
                        >,
                                    request: axum::extract::Request| {
                            Self::update_many_handle(app_state_99328dfe, request, &table_owned)
                                .await
                        }
                    }),
                )
                .route(
                    "/update_many_payload_example",
                    axum::routing::get(async move || Self::update_many_payload_example()),
                )
                .route(
                    "/update_one",
                    axum::routing::patch({
                        let table_owned = table.to_owned();
                        async move |app_state_99328dfe: axum::extract::State<
                            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
                        >,
                                    request: axum::extract::Request| {
                            Self::update_one_handle(app_state_99328dfe, request, &table_owned).await
                        }
                    }),
                )
                .route(
                    "/update_one_payload_example",
                    axum::routing::get(async move || Self::update_one_payload_example()),
                )
                .route(
                    "/delete_many",
                    axum::routing::delete({
                        let table_owned = table.to_owned();
                        async move |app_state_99328dfe: axum::extract::State<
                            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
                        >,
                                    request: axum::extract::Request| {
                            Self::delete_many_handle(app_state_99328dfe, request, &table_owned)
                                .await
                        }
                    }),
                )
                .route(
                    "/delete_many_payload_example",
                    axum::routing::get(async move || Self::delete_many_payload_example()),
                )
                .route(
                    "/delete_one",
                    axum::routing::delete({
                        let table_owned = table.to_owned();
                        async move |app_state_99328dfe: axum::extract::State<
                            std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
                        >,
                                    request: axum::extract::Request| {
                            Self::delete_one_handle(app_state_99328dfe, request, &table_owned).await
                        }
                    }),
                )
                .route(
                    "/delete_one_payload_example",
                    axum::routing::get(async move || Self::delete_one_payload_example()),
                )
                .with_state(app_state),
        )
    }
    pub fn routes(
        app_state: std::sync::Arc<dyn postgresql_crud::CombinationOfAppStateLogicTraits>,
    ) -> axum::Router {
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
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleCreate { pub column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create , pub column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create , pub column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create }
#[allow(clippy::arbitrary_source_item_ordering)]
impl TableExampleCreate {
    fn create_query_part(
        &self,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc_a097110b = String::default();
        match < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: create_query_part (& < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , increment) { Ok (value_c3f0b59a) => { if { use std :: fmt :: Write as _ ; write ! (acc_a097110b , "{value_c3f0b59a},") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error_0) => { return Err (error_0) ; } }
        match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: create_query_part (& self . column_0 , increment) { Ok (value_c3f0b59a) => { if { use std :: fmt :: Write as _ ; write ! (acc_a097110b , "{value_c3f0b59a},") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error_0) => { return Err (error_0) ; } }
        match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: create_query_part (& self . column_1 , increment) { Ok (value_c3f0b59a) => { if { use std :: fmt :: Write as _ ; write ! (acc_a097110b , "{value_c3f0b59a},") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error_0) => { return Err (error_0) ; } }
        match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: create_query_part (& self . column_2 , increment) { Ok (value_c3f0b59a) => { if { use std :: fmt :: Write as _ ; write ! (acc_a097110b , "{value_c3f0b59a},") } . is_err () { return Err (postgresql_crud :: QueryPartErrorNamed :: WriteIntoBuffer { code_occurence : error_occurence_lib :: code_occurence ! () }) ; } } Err (error_0) => { return Err (error_0) ; } }
        let _: Option<char> = acc_a097110b.pop();
        Ok(acc_a097110b)
    }
    fn create_query_bind(
        self,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
        match < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: create_query_bind (< < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , query) { Ok (value_3c55d2e1) => { query = value_3c55d2e1 ; } , Err (error_0) => { return Err (error_0) ; } }
        match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: create_query_bind (self . column_0 , query) { Ok (value_3c55d2e1) => { query = value_3c55d2e1 ; } , Err (error_0) => { return Err (error_0) ; } }
        match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: create_query_bind (self . column_1 , query) { Ok (value_3c55d2e1) => { query = value_3c55d2e1 ; } , Err (error_0) => { return Err (error_0) ; } }
        match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: create_query_bind (self . column_2 , query) { Ok (value_3c55d2e1) => { query = value_3c55d2e1 ; } , Err (error_0) => { return Err (error_0) ; } }
        Ok(query)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleCreate
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { column_0 : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleWhereMany { primary_key_column : Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where > > , column_0 : Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > > , column_1 : Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Where > > , column_2 : Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > > }
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleWhereManyTryNewErrorNamed {
    NoFieldsProvided {
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleWhereMany {
    pub fn try_new(
        primary_key_column : Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where > >,
        column_0 : Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > >,
        column_1 : Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Where > >,
        column_2 : Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > >,
    ) -> Result<Self, TableExampleWhereManyTryNewErrorNamed> {
        if matches!(
            (&primary_key_column, &column_0, &column_1, &column_2),
            (None, None, None, None)
        ) {
            return Err(TableExampleWhereManyTryNewErrorNamed::NoFieldsProvided {
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        Ok(Self {
            primary_key_column,
            column_0,
            column_1,
            column_2,
        })
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
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
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        2u64 => Ok(__Field::__field2),
                        3u64 => Ok(__Field::__field3),
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
                        "column_1" => Ok(__Field::__field2),
                        "column_2" => Ok(__Field::__field3),
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
                        b"column_1" => Ok(__Field::__field2),
                        b"column_2" => Ok(__Field::__field3),
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
                marker: _serde::__private228::PhantomData<TableExampleWhereMany>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableExampleWhereMany;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct TableExampleWhereMany",
                    )
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some (__field0_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct TableExampleWhereMany with 4 elements")) ; } ;
                    let Some (__field1_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct TableExampleWhereMany with 4 elements")) ; } ;
                    let Some (__field2_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Where > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct TableExampleWhereMany with 4 elements")) ; } ;
                    let Some (__field3_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct TableExampleWhereMany with 4 elements")) ; } ;
                    match TableExampleWhereMany::try_new(
                        __field0_handle,
                        __field1_handle,
                        __field2_handle,
                        __field3_handle,
                    ) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0 : Option < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where > > > = None ;
                    let mut __field1 : Option < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > > > = None ;
                    let mut __field2 : Option < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Where > > > = None ;
                    let mut __field3 : Option < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > > > = None ;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "primary_key_column",
                                    ));
                                }
                                __field0 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "column_0",
                                    ));
                                }
                                __field1 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field2 => {
                                if Option::is_some(&__field2) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "column_1",
                                    ));
                                }
                                __field2 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Where > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field3 => {
                                if Option::is_some(&__field3) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "column_2",
                                    ));
                                }
                                __field3 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Where > > > (& mut __map) ? ,) ;
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let __field0_handle = match __field0 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("primary_key_column")?,
                    };
                    let __field1_handle = match __field1 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("column_0")?,
                    };
                    let __field2_handle = match __field2 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("column_1")?,
                    };
                    let __field3_handle = match __field3 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("column_2")?,
                    };
                    match TableExampleWhereMany::try_new(
                        __field0_handle,
                        __field1_handle,
                        __field2_handle,
                        __field3_handle,
                    ) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["primary_key_column", "column_0", "column_1", "column_2"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "TableExampleWhereMany",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleWhereMany
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { primary_key_column : Some (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()) , column_0 : Some (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()) , column_1 : Some (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()) , column_2 : Some (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()) , }
    }
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct StdOptionOptionTableExampleWhereMany(pub Option<TableExampleWhereMany>);
#[allow(clippy::arbitrary_source_item_ordering)]
impl<'lifetime> postgresql_crud::PostgresqlTypeWhereFilter<'lifetime>
    for StdOptionOptionTableExampleWhereMany
{
    fn query_part(
        &self,
        increment: &mut u64,
        _: &dyn std::fmt::Display,
        _: bool,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        Ok(match &self.0 {
            Some(value) => {
                let mut additional_parameters = String::from("where");
                let mut is_first_push_to_additional_parameters_already_happend = false;
                if let Some(value_da0f0616) = &value.primary_key_column {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                        value_da0f0616,
                        increment,
                        &"primary_key_column",
                        is_first_push_to_additional_parameters_already_happend,
                    ) {
                        Ok(value_9e3f8fdd) => {
                            additional_parameters.push_str(&value_9e3f8fdd);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            return Err(error_0);
                        }
                    }
                }
                if let Some(value_da0f0616) = &value.column_0 {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                        value_da0f0616,
                        increment,
                        &"column_0",
                        is_first_push_to_additional_parameters_already_happend,
                    ) {
                        Ok(value_9e3f8fdd) => {
                            additional_parameters.push_str(&value_9e3f8fdd);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            return Err(error_0);
                        }
                    }
                }
                if let Some(value_da0f0616) = &value.column_1 {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                        value_da0f0616,
                        increment,
                        &"column_1",
                        is_first_push_to_additional_parameters_already_happend,
                    ) {
                        Ok(value_9e3f8fdd) => {
                            additional_parameters.push_str(&value_9e3f8fdd);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            return Err(error_0);
                        }
                    }
                }
                if let Some(value_da0f0616) = &value.column_2 {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(
                        value_da0f0616,
                        increment,
                        &"column_2",
                        is_first_push_to_additional_parameters_already_happend,
                    ) {
                        Ok(value_9e3f8fdd) => {
                            additional_parameters.push_str(&value_9e3f8fdd);
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
    fn query_bind(
        self,
        mut query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
    {
        if let Some(value_27176ffb) = self.0 {
            if let Some(value_b12d6fe0) = value_27176ffb.primary_key_column {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value_b12d6fe0, query)
                {
                    Ok(value_edaee3b2) => {
                        query = value_edaee3b2;
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                }
            }
            if let Some(value_b12d6fe0) = value_27176ffb.column_0 {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value_b12d6fe0, query)
                {
                    Ok(value_edaee3b2) => {
                        query = value_edaee3b2;
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                }
            }
            if let Some(value_b12d6fe0) = value_27176ffb.column_1 {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value_b12d6fe0, query)
                {
                    Ok(value_edaee3b2) => {
                        query = value_edaee3b2;
                    }
                    Err(error_0) => {
                        return Err(error_0);
                    }
                }
            }
            if let Some(value_b12d6fe0) = value_27176ffb.column_2 {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value_b12d6fe0, query)
                {
                    Ok(value_edaee3b2) => {
                        query = value_edaee3b2;
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
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for StdOptionOptionTableExampleWhereMany
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self (Some (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()))
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleSelect {
    # [serde (rename (serialize = "primary_key_column" , deserialize = "primary_key_column"))] PrimaryKeyColumn (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Select) , # [serde (rename (serialize = "column_0" , deserialize = "column_0"))] Column0 (< postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Select) , # [serde (rename (serialize = "column_1" , deserialize = "column_1"))] Column1 (< postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Select) , # [serde (rename (serialize = "column_2" , deserialize = "column_2"))] Column2 (< postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Select) }
impl std::fmt::Display for TableExampleSelect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).unwrap_or_else(|element_2636212f| format!(
                "cannot serialize into json: {element_2636212f:?}"
            ))
        )
    }
}
impl error_occurence_lib::ToStdStringString for TableExampleSelect {
    fn to_std_string_string(&self) -> String {
        format!("{self}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleSelect
{
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
    -> Vec<Self> {
        vec ! [Self :: PrimaryKeyColumn (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()) , Self :: Column0 (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()) , Self :: Column1 (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()) , Self :: Column2 (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())]
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct TableExampleRead { # [serde (skip_serializing_if = "Option::is_none")] pub primary_key_column : Option < postgresql_crud :: Value < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read > > , # [serde (skip_serializing_if = "Option::is_none")] pub column_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Read > > , # [serde (skip_serializing_if = "Option::is_none")] pub column_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Read > > , # [serde (skip_serializing_if = "Option::is_none")] pub column_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Read > > }
impl TableExampleRead {
    fn try_from_sqlx_postgres_pg_row_with_not_empty_unique_vec_table_example_select(
        value: &sqlx::postgres::PgRow,
        select: &postgresql_crud::NotEmptyUniqueVec<TableExampleSelect>,
    ) -> Result<Self, sqlx::Error> {
        let mut primary_key_column : Option < postgresql_crud :: Value < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read > > = None ;
        let mut column_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Read > > = None ;
        let mut column_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Read > > = None ;
        let mut column_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Read > > = None ;
        for element_dca9f0b7 in select.to_vec() {
            match element_dca9f0b7 { TableExampleSelect :: PrimaryKeyColumn (_) => match sqlx :: Row :: try_get :: < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read , & str > (value , "primary_key_column") { Ok (value_dccdf117) => { primary_key_column = Some (postgresql_crud :: Value { value : value_dccdf117 }) ; } , Err (error_0) => { return Err (error_0) ; } } , TableExampleSelect :: Column0 (_) => match sqlx :: Row :: try_get :: < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Read , & str > (value , "column_0") { Ok (value_09b0fc09) => { column_0 = Some (postgresql_crud :: Value { value : value_09b0fc09 }) ; } , Err (error_0) => { return Err (error_0) ; } } , TableExampleSelect :: Column1 (_) => match sqlx :: Row :: try_get :: < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Read , & str > (value , "column_1") { Ok (value_09b0fc09) => { column_1 = Some (postgresql_crud :: Value { value : value_09b0fc09 }) ; } , Err (error_0) => { return Err (error_0) ; } } , TableExampleSelect :: Column2 (_) => match sqlx :: Row :: try_get :: < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Read , & str > (value , "column_2") { Ok (value_09b0fc09) => { column_2 = Some (postgresql_crud :: Value { value : value_09b0fc09 }) ; } , Err (error_0) => { return Err (error_0) ; } } }
        }
        Ok(Self {
            primary_key_column,
            column_0,
            column_1,
            column_2,
        })
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct TableExampleReadOnlyIds { pub primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: ReadOnlyIds , pub column_0 : Option < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: ReadOnlyIds > , pub column_1 : Option < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: ReadOnlyIds > , pub column_2 : Option < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: ReadOnlyIds > }
impl < 'lifetime , R : :: sqlx :: Row < Database = sqlx :: Postgres >> :: sqlx :: FromRow < 'lifetime , R > for TableExampleReadOnlyIds where & 'lifetime :: std :: primitive :: str : :: sqlx :: ColumnIndex < R > , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: ReadOnlyIds : :: sqlx :: decode :: Decode < 'lifetime , R :: Database > , < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: ReadOnlyIds : :: sqlx :: decode :: Decode < 'lifetime , R :: Database > , < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: ReadOnlyIds : :: sqlx :: decode :: Decode < 'lifetime , R :: Database > , < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: ReadOnlyIds : :: sqlx :: decode :: Decode < 'lifetime , R :: Database > { fn from_row (__row : & 'lifetime R) -> :: sqlx :: Result < Self > { let primary_key_column = match sqlx :: Row :: try_get :: < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: ReadOnlyIds , & str > (__row , "primary_key_column") { Ok (value_283179dd) => value_283179dd , Err (error_0) => { return Err (error_0) ; } } ; let column_0 = sqlx :: Row :: try_get :: < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: ReadOnlyIds , & str > (__row , "column_0") . ok () ; let column_1 = sqlx :: Row :: try_get :: < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: ReadOnlyIds , & str > (__row , "column_1") . ok () ; let column_2 = sqlx :: Row :: try_get :: < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: ReadOnlyIds , & str > (__row , "column_2") . ok () ; Ok (Self { primary_key_column , column_0 , column_1 , column_2 }) } }
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleUpdate { primary_key_column : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate , column_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > > , column_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Update > > , column_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > > }
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateTryNewErrorNamed {
    NoFieldsProvided {
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleUpdate {
    pub fn try_new(
        primary_key_column : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate,
        column_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > >,
        column_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Update > >,
        column_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > >,
    ) -> Result<Self, TableExampleUpdateTryNewErrorNamed> {
        if matches!((&column_0, &column_1, &column_2), (None, None, None)) {
            return Err(TableExampleUpdateTryNewErrorNamed::NoFieldsProvided {
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        Ok(Self {
            primary_key_column,
            column_0,
            column_1,
            column_2,
        })
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
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
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        2u64 => Ok(__Field::__field2),
                        3u64 => Ok(__Field::__field3),
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
                        "column_1" => Ok(__Field::__field2),
                        "column_2" => Ok(__Field::__field3),
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
                        b"column_1" => Ok(__Field::__field2),
                        b"column_2" => Ok(__Field::__field3),
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
                marker: _serde::__private228::PhantomData<TableExampleUpdate>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableExampleUpdate;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "struct TableExampleUpdate",
                    )
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some (__field0_handle) = serde :: de :: SeqAccess :: next_element :: < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct TableExampleUpdate with 4 elements")) ; } ;
                    let Some (__field1_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct TableExampleUpdate with 4 elements")) ; } ;
                    let Some (__field2_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Update > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct TableExampleUpdate with 4 elements")) ; } ;
                    let Some (__field3_handle) = serde :: de :: SeqAccess :: next_element :: < Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > > > (& mut __seq) ? else { return Err (serde :: de :: Error :: invalid_length (0usize , & "struct TableExampleUpdate with 4 elements")) ; } ;
                    match TableExampleUpdate::try_new(
                        __field0_handle,
                        __field1_handle,
                        __field2_handle,
                        __field3_handle,
                    ) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0 : Option < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate > = None ;
                    let mut __field1 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > > > = None ;
                    let mut __field2 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Update > > > = None ;
                    let mut __field3 : Option < Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > > > = None ;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "primary_key_column",
                                    ));
                                }
                                __field0 = Some (serde :: de :: MapAccess :: next_value :: < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate > (& mut __map) ? ,) ;
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "column_0",
                                    ));
                                }
                                __field1 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field2 => {
                                if Option::is_some(&__field2) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "column_1",
                                    ));
                                }
                                __field2 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Update > > > (& mut __map) ? ,) ;
                            }
                            __Field::__field3 => {
                                if Option::is_some(&__field3) {
                                    return Err(<__A::Error as serde::de::Error>::duplicate_field(
                                        "column_2",
                                    ));
                                }
                                __field3 = Some (serde :: de :: MapAccess :: next_value :: < Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Update > > > (& mut __map) ? ,) ;
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let __field0_handle = match __field0 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("primary_key_column")?,
                    };
                    let __field1_handle = match __field1 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("column_0")?,
                    };
                    let __field2_handle = match __field2 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("column_1")?,
                    };
                    let __field3_handle = match __field3 {
                        Some(value_4f8faf03) => value_4f8faf03,
                        None => serde::__private228::de::missing_field("column_2")?,
                    };
                    match TableExampleUpdate::try_new(
                        __field0_handle,
                        __field1_handle,
                        __field2_handle,
                        __field3_handle,
                    ) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["primary_key_column", "column_0", "column_1", "column_2"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "TableExampleUpdate",
                FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleUpdate
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { primary_key_column : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_0 : Some (postgresql_crud :: Value { value : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }) , column_1 : Some (postgresql_crud :: Value { value : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }) , column_2 : Some (postgresql_crud :: Value { value : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }) }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleUpdateForQuery { primary_key_column : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdateForQuery , column_0 : Option < postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: UpdateForQuery > > , column_1 : Option < postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: UpdateForQuery > > , column_2 : Option < postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: UpdateForQuery > > }
#[allow(clippy::arbitrary_source_item_ordering)]
impl TableExampleUpdateForQuery {
    fn update_query_part_primary_key(
        &self,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: update_query_part (& self . primary_key_column , "" , TableExample :: primary_key () , "" , increment ,) { Ok (value_snake_case) => Ok (value_snake_case) , Err (error_0) => Err (error_0) }
    }
    fn update_query_part_column_0(
        value : & postgresql_crud :: Value < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: UpdateForQuery >,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: update_query_part (& value . value , "column_0" , "column_0" , "" , increment) { Ok (value_f75dfd93) => Ok (value_f75dfd93) , Err (error_0) => Err (error_0) , }
    }
    fn update_query_part_column_1(
        value : & postgresql_crud :: Value < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: UpdateForQuery >,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: update_query_part (& value . value , "column_1" , "column_1" , "" , increment) { Ok (value_f75dfd93) => Ok (value_f75dfd93) , Err (error_0) => Err (error_0) , }
    }
    fn update_query_part_column_2(
        value : & postgresql_crud :: Value < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: UpdateForQuery >,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: update_query_part (& value . value , "column_2" , "column_2" , "" , increment) { Ok (value_f75dfd93) => Ok (value_f75dfd93) , Err (error_0) => Err (error_0) , }
    }
    fn select_only_updated_ids_query_part(
        &self,
        increment: &mut u64,
    ) -> Result<String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc_88c91f52 = String::new();
        acc_88c91f52 . push_str (& match < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_part (& self . primary_key_column , "primary_key_column" , increment ,) { Ok (value) => value , Err (error) => { return Err (error) ; } }) ;
        if let Some(value_90f79b11) = &self.column_0 {
            acc_88c91f52 . push_str (& match < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_part (& value_90f79b11 . value , "column_0" , increment ,) { Ok (value_47a6f597) => value_47a6f597 , Err (error) => { return Err (error) ; } }) ;
        }
        if let Some(value_90f79b11) = &self.column_1 {
            acc_88c91f52 . push_str (& match < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_part (& value_90f79b11 . value , "column_1" , increment ,) { Ok (value_47a6f597) => value_47a6f597 , Err (error) => { return Err (error) ; } }) ;
        }
        if let Some(value_90f79b11) = &self.column_2 {
            acc_88c91f52 . push_str (& match < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: select_only_updated_ids_query_part (& value_90f79b11 . value , "column_2" , increment ,) { Ok (value_47a6f597) => value_47a6f597 , Err (error) => { return Err (error) ; } }) ;
        }
        let _: Option<char> = acc_88c91f52.pop();
        Ok(acc_88c91f52)
    }
    fn from_handle(value: TableExampleUpdate) -> Self {
        Self { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: UpdateForQuery :: from (value . primary_key_column) , column_0 : value . column_0 . map (| value_0e64c53a | postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: UpdateForQuery :: from (value_0e64c53a . value) }) , column_1 : value . column_1 . map (| value_0e64c53a | postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: UpdateForQuery :: from (value_0e64c53a . value) }) , column_2 : value . column_2 . map (| value_0e64c53a | postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: UpdateForQuery :: from (value_0e64c53a . value) }) }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleCreateManyPayload(pub Vec<TableExampleCreate>);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleCreateManyPayload
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self (vec ! [postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()])
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub struct TableExampleCreateManyParameters {
    pub payload: TableExampleCreateManyPayload,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleCreateManyResponseVariants {
    Desirable(Vec<TableExampleReadOnlyIds>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: String,
        rollback: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleCreateManyResponseVariants {
    fn from_handle(value: TableExampleCreateManyErrorNamed) -> Self {
        match value . into_serialize_deserialize_version () { TableExampleCreateManyErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } => Self :: CheckBodySize { check_body_size , code_occurence } , TableExampleCreateManyErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } => Self :: Postgresql { postgresql , code_occurence } , TableExampleCreateManyErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } => Self :: SerdeJson { serde_json , code_occurence } , TableExampleCreateManyErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleCreateManyErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } => Self :: CheckCommit { check_commit , code_occurence } , TableExampleCreateManyErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } => Self :: QueryPart { error , code_occurence } , TableExampleCreateManyErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } => Self :: RowAndRollback { row , rollback , code_occurence } , TableExampleCreateManyErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } => Self :: TryBind { try_bind , code_occurence } }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleCreateManyErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::ErrorNamed,
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
        check_commit: postgresql_crud::check_commit::ErrorNamed,
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
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryCreateManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
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
        create_many_error_named_with_serialize_deserialize:
            TableExampleCreateManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub struct TableExampleCreateOneParameters {
    pub payload: TableExampleCreate,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleCreateOneResponseVariants {
    Desirable(TableExampleReadOnlyIds),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: String,
        rollback: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleCreateOneResponseVariants {
    fn from_handle(value: TableExampleCreateOneErrorNamed) -> Self {
        match value . into_serialize_deserialize_version () { TableExampleCreateOneErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } => Self :: CheckBodySize { check_body_size , code_occurence } , TableExampleCreateOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } => Self :: Postgresql { postgresql , code_occurence } , TableExampleCreateOneErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } => Self :: SerdeJson { serde_json , code_occurence } , TableExampleCreateOneErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleCreateOneErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } => Self :: CheckCommit { check_commit , code_occurence } , TableExampleCreateOneErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } => Self :: RowAndRollback { row , rollback , code_occurence } , TableExampleCreateOneErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } => Self :: QueryPart { error , code_occurence } , TableExampleCreateOneErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } => Self :: TryBind { try_bind , code_occurence } }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleCreateOneErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::ErrorNamed,
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
        check_commit: postgresql_crud::check_commit::ErrorNamed,
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
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryCreateOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
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
        create_one_error_named_with_serialize_deserialize:
            TableExampleCreateOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleReadManyPayload {
    pub where_many: StdOptionOptionTableExampleWhereMany,
    pub select: postgresql_crud::NotEmptyUniqueVec<TableExampleSelect>,
    pub order_by: postgresql_crud::OrderBy<TableExampleSelect>,
    pub pagination: postgresql_crud::PaginationStartsWithZero,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleReadManyPayload
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { where_many : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () , select : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () , order_by : postgresql_crud :: OrderBy { column : TableExampleSelect :: PrimaryKeyColumn (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()) , order : Some (postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()) , } , pagination : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () , }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub struct TableExampleReadManyParameters {
    pub payload: TableExampleReadManyPayload,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleReadManyResponseVariants {
    Desirable(Vec<TableExampleRead>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleReadManyResponseVariants {
    fn from_handle(value: TableExampleReadManyErrorNamed) -> Self {
        match value . into_serialize_deserialize_version () { TableExampleReadManyErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } => Self :: CheckBodySize { check_body_size , code_occurence } , TableExampleReadManyErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } => Self :: Postgresql { postgresql , code_occurence } , TableExampleReadManyErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } => Self :: SerdeJson { serde_json , code_occurence } , TableExampleReadManyErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleReadManyErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } => Self :: CheckCommit { check_commit , code_occurence } , TableExampleReadManyErrorNamedWithSerializeDeserialize :: NotUniqueField { not_unique_field , code_occurence } => Self :: NotUniqueField { not_unique_field , code_occurence } , TableExampleReadManyErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } => Self :: QueryPart { error , code_occurence } , TableExampleReadManyErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } => Self :: TryBind { try_bind , code_occurence } }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleReadManyErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::ErrorNamed,
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
        check_commit: postgresql_crud::check_commit::ErrorNamed,
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
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryReadManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
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
        read_many_error_named_with_serialize_deserialize:
            TableExampleReadManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleReadOnePayload {
    pub primary_key_column:
        postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead,
    pub select: postgresql_crud::NotEmptyUniqueVec<TableExampleSelect>,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleReadOnePayload
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { primary_key_column : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () , select : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub struct TableExampleReadOneParameters {
    pub payload: TableExampleReadOnePayload,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleReadOneResponseVariants {
    Desirable(TableExampleRead),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueField {
        not_unique_field: TableExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleReadOneResponseVariants {
    fn from_handle(value: TableExampleReadOneErrorNamed) -> Self {
        match value . into_serialize_deserialize_version () { TableExampleReadOneErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } => Self :: CheckBodySize { check_body_size , code_occurence } , TableExampleReadOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } => Self :: Postgresql { postgresql , code_occurence } , TableExampleReadOneErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } => Self :: SerdeJson { serde_json , code_occurence } , TableExampleReadOneErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleReadOneErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } => Self :: CheckCommit { check_commit , code_occurence } , TableExampleReadOneErrorNamedWithSerializeDeserialize :: NotUniqueField { not_unique_field , code_occurence } => Self :: NotUniqueField { not_unique_field , code_occurence } , TableExampleReadOneErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } => Self :: QueryPart { error , code_occurence } , TableExampleReadOneErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } => Self :: TryBind { try_bind , code_occurence } }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleReadOneErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::ErrorNamed,
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
        check_commit: postgresql_crud::check_commit::ErrorNamed,
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
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryReadOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
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
        read_one_error_named_with_serialize_deserialize:
            TableExampleReadOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, utoipa :: ToSchema)]
pub struct TableExampleUpdateManyPayload(Vec<TableExampleUpdate>);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateManyPayloadTryNewErrorNamed {
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key:
            postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlUpdate,
        #[eo_to_std_string_string]
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleUpdateManyPayload {
    pub fn try_new(
        value: Vec<TableExampleUpdate>,
    ) -> Result<Self, TableExampleUpdateManyPayloadTryNewErrorNamed> {
        let mut acc_6bf275fc = Vec::new();
        for element_35facc3a in &value {
            if acc_6bf275fc.contains(&&element_35facc3a.primary_key_column) {
                return Err(
                    TableExampleUpdateManyPayloadTryNewErrorNamed::NotUniquePrimaryKey {
                        not_unique_primary_key: element_35facc3a.primary_key_column,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    },
                );
            }
            acc_6bf275fc.push(&element_35facc3a.primary_key_column);
        }
        Ok(Self(value))
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[allow(
        unused_extern_crates,
        clippy::useless_attribute,
        clippy::arbitrary_source_item_ordering
    )]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TableExampleUpdateManyPayload {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private228::PhantomData<TableExampleUpdateManyPayload>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableExampleUpdateManyPayload;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "tuple struct TableExampleUpdateManyPayload",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<TableExampleUpdate> =
                        <Vec<TableExampleUpdate> as _serde::Deserialize>::deserialize(__e)?;
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
                    let Some(__field0) =
                        _serde::de::SeqAccess::next_element::<Vec<TableExampleUpdate>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct TableExampleUpdateManyPayload with 1 element",
                        ));
                    };
                    match TableExampleUpdateManyPayload::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "TableExampleUpdateManyPayload",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleUpdateManyPayload
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self (vec ! [postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ()])
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub struct TableExampleUpdateManyParameters {
    pub payload: TableExampleUpdateManyPayload,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleUpdateManyResponseVariants {
    Desirable(Vec<TableExampleReadOnlyIds>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: String,
        rollback: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleUpdateManyResponseVariants {
    fn from_handle(value: TableExampleUpdateManyErrorNamed) -> Self {
        match value . into_serialize_deserialize_version () { TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } => Self :: CheckBodySize { check_body_size , code_occurence } , TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } => Self :: Postgresql { postgresql , code_occurence } , TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } => Self :: SerdeJson { serde_json , code_occurence } , TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } => Self :: CheckCommit { check_commit , code_occurence } , TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } => Self :: RowAndRollback { row , rollback , code_occurence } , TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } => Self :: QueryPart { error , code_occurence } , TableExampleUpdateManyErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } => Self :: TryBind { try_bind , code_occurence } }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateManyErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::ErrorNamed,
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
        check_commit: postgresql_crud::check_commit::ErrorNamed,
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
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryUpdateManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
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
        update_many_error_named_with_serialize_deserialize:
            TableExampleUpdateManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub struct TableExampleUpdateOneParameters {
    pub payload: TableExampleUpdate,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleUpdateOneResponseVariants {
    Desirable(TableExampleReadOnlyIds),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    HeaderContentTypeApplicationJsonNotFound {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::ErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: String,
        rollback: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    QueryPart {
        error: postgresql_crud::QueryPartErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryBind {
        try_bind: String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl TableExampleUpdateOneResponseVariants {
    fn from_handle(value: TableExampleUpdateOneErrorNamed) -> Self {
        match value . into_serialize_deserialize_version () { TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } => Self :: CheckBodySize { check_body_size , code_occurence } , TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } => Self :: Postgresql { postgresql , code_occurence } , TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } => Self :: SerdeJson { serde_json , code_occurence } , TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } => Self :: CheckCommit { check_commit , code_occurence } , TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } => Self :: RowAndRollback { row , rollback , code_occurence } , TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } => Self :: QueryPart { error , code_occurence } , TableExampleUpdateOneErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } => Self :: TryBind { try_bind , code_occurence } }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleUpdateOneErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::ErrorNamed,
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
        check_commit: postgresql_crud::check_commit::ErrorNamed,
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
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryUpdateOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
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
        update_one_error_named_with_serialize_deserialize:
            TableExampleUpdateOneErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleDeleteManyPayload {
    pub where_many: StdOptionOptionTableExampleWhereMany,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleDeleteManyPayload
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { where_many : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub struct TableExampleDeleteManyParameters {
    pub payload: TableExampleDeleteManyPayload,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleDeleteManyResponseVariants {
    Desirable (Vec < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >) , CheckBodySize { check_body_size : postgresql_crud :: check_body_size :: ErrorNamedWithSerializeDeserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , Postgresql { postgresql : String , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , SerdeJson { serde_json : String , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , HeaderContentTypeApplicationJsonNotFound { code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , CheckCommit { check_commit : postgresql_crud :: check_commit :: ErrorNamedWithSerializeDeserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , RowAndRollback { row : String , rollback : String , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , QueryPart { error : postgresql_crud :: QueryPartErrorNamedWithSerializeDeserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , TryBind { try_bind : String , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } }
impl TableExampleDeleteManyResponseVariants {
    fn from_handle(value: TableExampleDeleteManyErrorNamed) -> Self {
        match value . into_serialize_deserialize_version () { TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } => Self :: CheckBodySize { check_body_size , code_occurence } , TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } => Self :: Postgresql { postgresql , code_occurence } , TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } => Self :: SerdeJson { serde_json , code_occurence } , TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } => Self :: CheckCommit { check_commit , code_occurence } , TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } => Self :: RowAndRollback { row , rollback , code_occurence } , TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: QueryPart { error , code_occurence } => Self :: QueryPart { error , code_occurence } , TableExampleDeleteManyErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } => Self :: TryBind { try_bind , code_occurence } }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleDeleteManyErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::ErrorNamed,
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
        check_commit: postgresql_crud::check_commit::ErrorNamed,
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
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryDeleteManyErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
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
        delete_many_error_named_with_serialize_deserialize:
            TableExampleDeleteManyErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct TableExampleDeleteOnePayload {
    pub primary_key_column:
        postgresql_crud::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlRead,
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement
    for TableExampleDeleteOnePayload
{
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self { primary_key_column : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy)]
pub struct TableExampleDeleteOneParameters {
    pub payload: TableExampleDeleteOnePayload,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TableExampleDeleteOneResponseVariants {
    Desirable (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read) , CheckBodySize { check_body_size : postgresql_crud :: check_body_size :: ErrorNamedWithSerializeDeserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , Postgresql { postgresql : String , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , SerdeJson { serde_json : String , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , HeaderContentTypeApplicationJsonNotFound { code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , CheckCommit { check_commit : postgresql_crud :: check_commit :: ErrorNamedWithSerializeDeserialize , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , RowAndRollback { row : String , rollback : String , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } , TryBind { try_bind : String , code_occurence : error_occurence_lib :: code_occurence :: CodeOccurence , } }
impl TableExampleDeleteOneResponseVariants {
    fn from_handle(value: TableExampleDeleteOneErrorNamed) -> Self {
        match value . into_serialize_deserialize_version () { TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: CheckBodySize { check_body_size , code_occurence } => Self :: CheckBodySize { check_body_size , code_occurence } , TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , code_occurence } => Self :: Postgresql { postgresql , code_occurence } , TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: SerdeJson { serde_json , code_occurence } => Self :: SerdeJson { serde_json , code_occurence } , TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: HeaderContentTypeApplicationJsonNotFound { code_occurence } => Self :: HeaderContentTypeApplicationJsonNotFound { code_occurence } , TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: CheckCommit { check_commit , code_occurence } => Self :: CheckCommit { check_commit , code_occurence } , TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: RowAndRollback { row , rollback , code_occurence } => Self :: RowAndRollback { row , rollback , code_occurence } , TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: TryBind { try_bind , code_occurence } => Self :: TryBind { try_bind , code_occurence } }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleDeleteOneErrorNamed {
    CheckBodySize {
        #[eo_error_occurence]
        check_body_size: postgresql_crud::check_body_size::ErrorNamed,
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
        check_commit: postgresql_crud::check_commit::ErrorNamed,
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
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TableExampleTryDeleteOneErrorNamed {
    SerdeJsonToString {
        #[eo_to_std_string_string]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
        #[eo_to_std_string_string]
        headers: reqwest::header::HeaderMap,
        #[eo_to_std_string_string]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_to_std_string_string]
        status_code: reqwest::StatusCode,
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
        delete_one_error_named_with_serialize_deserialize:
            TableExampleDeleteOneErrorNamedWithSerializeDeserialize,
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
        fn generate_ident_where_many_pripery_key_others_none(
            option_postgresql_type_where : Option < postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where >>,
        ) -> TableExampleWhereMany {
            TableExampleWhereMany::try_new(option_postgresql_type_where, None, None, None)
                .expect("5fb2b219-8bd7-4edd-9722-b475826707f5")
        }        fn generate_postgresql_type_where_try_new_primary_key (logical_operator : postgresql_crud :: LogicalOperator , vec : Vec < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere >) -> postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where >{
            postgresql_crud::PostgresqlTypeWhere::try_new(logical_operator, vec)
                .expect("fd20ad6d-fb4c-4da7-96b5-43fce0cdb94c")
        }        fn generate_postgresql_type_where_try_new_or_primary_keys (vec_read_only_ids : & [TableExampleReadOnlyIds]) -> postgresql_crud :: PostgresqlTypeWhere < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where >{
            generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec_read_only_ids . iter () . map (| element_9530b728 | postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_9530b728 . primary_key_column) ,)) , })) . collect ())
        }
        async fn generate_try_read_many_order_by_primary_key_with_big_pagination(
            endpoint_location: &str,
            current_ident_where_many: TableExampleWhereMany,
            select: postgresql_crud::NotEmptyUniqueVec<TableExampleSelect>,
            table: &str,
        ) -> Result<Vec<TableExampleRead>, TableExampleTryReadManyErrorNamed> {
            TableExample :: try_read_many_handle (endpoint_location , TableExampleReadManyParameters { payload : TableExampleReadManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (current_ident_where_many)) , select , order_by : postgresql_crud :: OrderBy { column : TableExampleSelect :: PrimaryKeyColumn (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Select :: default ()) , order : Some (postgresql_crud :: Order :: Asc) } , pagination : postgresql_crud :: PaginationStartsWithZero :: try_new (10000 , 0) . expect ("b0cdf0cb-1e31-4a7e-9e53-d2ff71efb983") , } } , table) . await
        }
        async fn generate_ident_try_read_one_handle_primary_key(
            url: &str,
            primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read,
            select: postgresql_crud::NotEmptyUniqueVec<TableExampleSelect>,
            table: &str,
        ) -> Result<TableExampleRead, TableExampleTryReadOneErrorNamed> {
            TableExample::try_read_one_handle(
                url,
                TableExampleReadOneParameters {
                    payload: TableExampleReadOnePayload {
                        primary_key_column,
                        select,
                    },
                },
                table,
            )
            .await
        }
        async fn generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
            url: &str,
            primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read,
            select: postgresql_crud::NotEmptyUniqueVec<TableExampleSelect>,
            table: &str,
        ) {
            if let Err(error) = generate_ident_try_read_one_handle_primary_key(
                url,
                primary_key_column,
                select,
                table,
            )
            .await
            {
                if let TableExampleTryReadOneErrorNamed :: TableExampleReadOneErrorNamedWithSerializeDeserialize { read_one_error_named_with_serialize_deserialize , .. } = error { if let TableExampleReadOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , .. } = read_one_error_named_with_serialize_deserialize { assert ! (postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row () , "58b9a6a4-cf9b-49f3-a20f-7007deea40fd") ; } else { panic ! ("0ad0117b-a2e0-4629-99d0-71935cd93d15") ; } } else { panic ! ("c6695392-4b5f-4482-86aa-b2f19c33a746") }
            } else {
                panic!("67e43b7a-d3ec-4a3b-a3f1-8c11499fd090")
            }
        }
        fn ident_create_default() -> TableExampleCreate {
            TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () }
        }
        async fn generate_read_only_ids_from_try_create_one(
            url: &str,
            payload: TableExampleCreate,
            table: &str,
        ) -> TableExampleReadOnlyIds {
            TableExample::try_create_one_handle(
                url,
                TableExampleCreateOneParameters { payload },
                table,
            )
            .await
            .expect("32e30b87-b46a-4f39-aeb0-39694fc52d30")
        }
        async fn generate_read_only_ids_from_try_create_one_default(
            url: &str,
            table: &str,
        ) -> TableExampleReadOnlyIds {
            generate_read_only_ids_from_try_create_one(url, ident_create_default(), table).await
        }        async fn generate_try_delete_one_handle (url : & str , primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read , table : & str ,) -> Result < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read , TableExampleTryDeleteOneErrorNamed >{
            TableExample::try_delete_one_handle(
                url,
                TableExampleDeleteOneParameters {
                    payload: TableExampleDeleteOnePayload { primary_key_column },
                },
                table,
            )
            .await
        }
        fn no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row() -> &'static str {
            "no rows returned by a query that expected to return at least one row"
        }
        fn generate_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create(
            read_only_ids_from_try_create_many: Vec<TableExampleReadOnlyIds>,
            ident_vec_create: Vec<TableExampleCreate>,
        ) -> Vec<TableExampleRead> {
            let mut acc_1debe8fb = Vec::new();
            assert_eq!(
                read_only_ids_from_try_create_many.len(),
                ident_vec_create.len(),
                "88fb286c-a440-441f-9e36-83454d0c9f75"
            );
            for (read_only_ids, create) in read_only_ids_from_try_create_many
                .into_iter()
                .zip(ident_vec_create.into_iter())
            {
                acc_1debe8fb . push (TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids . column_0 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , create . column_0) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids . column_1 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , create . column_1) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids . column_2 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , create . column_2) }) ;
            }
            acc_1debe8fb.sort_by(|first, second| {
                if let (Some(first_handle), Some(second_handle)) =
                    (&first.primary_key_column, &second.primary_key_column)
                {
                    first_handle.value.cmp(&second_handle.value)
                } else {
                    panic!("d760ffa3-e9df-4628-92cd-d52c1ae1f91a");
                }
            });
            acc_1debe8fb
        }
        async fn generate_read_only_ids_current_elements(
            url: &str,
            current_table: &str,
            select_default_all_with_max_page_size: postgresql_crud::NotEmptyUniqueVec<
                TableExampleSelect,
            >,
            read_only_ids_to_two_dimensional_vec_read_inner_acc: Vec<TableExampleCreate>,
        ) -> Vec<TableExampleReadOnlyIds> {
            let read_only_ids_current_elements =
                futures::StreamExt::collect::<Vec<Vec<TableExampleReadOnlyIds>>>(
                    futures::StreamExt::buffer_unordered(
                        futures::stream::iter(
                            read_only_ids_to_two_dimensional_vec_read_inner_acc
                                .chunks(25)
                                .map(Vec::from)
                                .map(|element_8e425cb1| {
                                    futures::FutureExt::boxed(async move {
                                        TableExample::try_create_many_handle(
                                            url,
                                            TableExampleCreateManyParameters {
                                                payload: TableExampleCreateManyPayload(
                                                    element_8e425cb1,
                                                ),
                                            },
                                            current_table,
                                        )
                                        .await
                                        .expect("38a24e7a-5b0e-4237-b686-3f03ab332efd")
                                    })
                                }),
                        ),
                        5,
                    ),
                )
                .await
                .into_iter()
                .flatten()
                .collect::<Vec<TableExampleReadOnlyIds>>();
            assert_eq ! (itertools :: Itertools :: sorted_by (read_only_ids_current_elements . iter () . map (| element_f108da5a | { TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& element_f108da5a . primary_key_column) , column_0 : element_f108da5a . column_0 . as_ref () . map_or_else (|| Some (postgresql_crud :: Value { value : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }) , < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element) , column_1 : element_f108da5a . column_1 . as_ref () . map_or_else (|| Some (postgresql_crud :: Value { value : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }) , < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element) , column_2 : element_f108da5a . column_2 . as_ref () . map_or_else (|| Some (postgresql_crud :: Value { value : postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element () }) , < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element) } }) , | first , second | match (& first . primary_key_column , & second . primary_key_column) { (Some (first_handle) , Some (second_handle)) => first_handle . value . cmp (& second_handle . value) , _ => panic ! ("0f1d45ed-b6e3-4fbd-bd41-bcbe61739f83") , }) . collect :: < Vec < TableExampleRead >> () , itertools :: Itertools :: sorted_by (generate_try_read_many_order_by_primary_key_with_big_pagination (url , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , read_only_ids_current_elements . iter () . map (| element_43ab7fb5 | postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_43ab7fb5 . primary_key_column))) })) . collect ()))) , select_default_all_with_max_page_size . clone () , current_table) . await . expect ("097d5e7d-41c6-41f4-8847-720647f2d6ea") . into_iter () , | first , second | match (& first . primary_key_column , & second . primary_key_column) { (Some (first_handle) , Some (second_handle)) => first_handle . value . cmp (& second_handle . value) , _ => panic ! ("51e477ea-0a01-46f0-89fb-967bb8e4e131") , }) . collect :: < Vec < TableExampleRead >> () , "50198a7f-e65c-4e4e-8d7f-9881cfd42453");
            read_only_ids_current_elements
        }
        tracing_subscriber::fmt::init();
        tokio :: runtime :: Builder :: new_multi_thread () . worker_threads (num_cpus :: get ()) . enable_all () . build () . expect ("38823c21-1879-449c-9b60-ce7293709959") . block_on (async { let config = server_config :: Config { service_socket_address : < config_lib :: ServiceSocketAddress as config_lib :: TryFromStdEnvVarOk > :: try_from_std_env_var_ok ("127.0.0.1:8080" . to_owned ()) . expect ("b5b3915a-0e18-4815-a614-6b0e9a00d73f") . 0 , database_url : < config_lib :: DatabaseUrl as config_lib :: TryFromStdEnvVarOk > :: try_from_std_env_var_ok ("postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=10" . to_owned ()) . expect ("f9c20f05-3cdf-46ae-b6d3-5943c627f0df") . 0 , timezone : < config_lib :: Timezone as config_lib :: TryFromStdEnvVarOk > :: try_from_std_env_var_ok ("10800" . to_owned ()) . expect ("d00d8998-52f9-45c1-a4b0-c93bc95a313e") . 0 , tracing_level : < config_lib :: TracingLevel as config_lib :: TryFromStdEnvVarOk > :: try_from_std_env_var_ok ("error" . to_owned ()) . expect ("957178c9-4d92-4110-b524-9dc21d147a7c") . 0 , source_place_type : < config_lib :: SourcePlaceType as config_lib :: TryFromStdEnvVarOk > :: try_from_std_env_var_ok ("source" . to_owned ()) . expect ("bec0950e-e9de-42f3-b3a2-67d9d98ae8a6") . 0 , enable_api_git_commit_check : < config_lib :: EnableApiGitCommitCheck as config_lib :: TryFromStdEnvVarOk > :: try_from_std_env_var_ok ("true" . to_owned ()) . expect ("31f02640-d62b-41ca-837d-d61b707d4baf") . 0 , maximum_size_of_http_body_in_bytes : < config_lib :: MaximumSizeOfHttpBodyInBytes as config_lib :: TryFromStdEnvVarOk > :: try_from_std_env_var_ok ("1048576000" . to_owned ()) . expect ("93b2f818-18be-4bb6-8a02-53c6e55ded2d") . 0 , } ; let postgres_pool = sqlx :: postgres :: PgPoolOptions :: new () . max_connections (50) . connect (secrecy :: ExposeSecret :: expose_secret (app_state :: GetDatabaseUrl :: get_database_url (& config))) . await . expect ("e3044bb9-7b76-4c0c-bc5f-eb34da05a103") ; let url = format ! ("http://{}" , app_state :: GetServiceSocketAddress :: get_service_socket_address (& config)) ; let table = "table_example" ; let add_table_postfix = | postfix : & str | { let value = format ! ("{table}_{postfix}") ; assert ! (value . len () <= 63 , "77f9bfb7-f7d8-4ba0-96d0-712d4246ecae") ; value } ; let table_initialization = add_table_postfix ("initialization") ; let table_create_many = add_table_postfix ("create_many") ; let table_create_one = add_table_postfix ("create_one") ; let table_test_read_many_by_non_existent_primary_keys = add_table_postfix ("test_read_many_by_non_existent_primary_keys") ; let table_test_read_many_by_equal_to_created_primary_keys = add_table_postfix ("test_read_many_by_equal_to_created_primary_keys") ; let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0 = add_table_postfix ("8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0") ; let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_1 = add_table_postfix ("8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_1") ; let table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_2 = add_table_postfix ("8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_2") ; let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0 = add_table_postfix ("eb24448c_fa63_4259_bb05_3215802a78f6_column_0") ; let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_1 = add_table_postfix ("eb24448c_fa63_4259_bb05_3215802a78f6_column_1") ; let table_eb24448c_fa63_4259_bb05_3215802a78f6_column_2 = add_table_postfix ("eb24448c_fa63_4259_bb05_3215802a78f6_column_2") ; let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0 = add_table_postfix ("9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0") ; let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_1 = add_table_postfix ("9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_1") ; let table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_2 = add_table_postfix ("9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_2") ; let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0 = add_table_postfix ("72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0") ; let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_1 = add_table_postfix ("72940b0e_cd26_493f_9ec1_2d999d9a4401_column_1") ; let table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_2 = add_table_postfix ("72940b0e_cd26_493f_9ec1_2d999d9a4401_column_2") ; let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0 = add_table_postfix ("5a52af33_a590_403b_808e_961df6d7e7aa_column_0") ; let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_1 = add_table_postfix ("5a52af33_a590_403b_808e_961df6d7e7aa_column_1") ; let table_5a52af33_a590_403b_808e_961df6d7e7aa_column_2 = add_table_postfix ("5a52af33_a590_403b_808e_961df6d7e7aa_column_2") ; let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0 = add_table_postfix ("1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0") ; let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_1 = add_table_postfix ("1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_1") ; let table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_2 = add_table_postfix ("1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_2") ; let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0 = add_table_postfix ("581c947f_9b0f_452f_8e52_524088bbb2e7_column_0") ; let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_1 = add_table_postfix ("581c947f_9b0f_452f_8e52_524088bbb2e7_column_1") ; let table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_2 = add_table_postfix ("581c947f_9b0f_452f_8e52_524088bbb2e7_column_2") ; let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0 = add_table_postfix ("de556c26_9297_4adb_9483_22d474cf1e7d_column_0") ; let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_1 = add_table_postfix ("de556c26_9297_4adb_9483_22d474cf1e7d_column_1") ; let table_de556c26_9297_4adb_9483_22d474cf1e7d_column_2 = add_table_postfix ("de556c26_9297_4adb_9483_22d474cf1e7d_column_2") ; let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0 = add_table_postfix ("35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0") ; let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_1 = add_table_postfix ("35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_1") ; let table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_2 = add_table_postfix ("35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_2") ; let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0 = add_table_postfix ("1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0") ; let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_1 = add_table_postfix ("1ce53b67_1e94_413e_83cf_c6d7094289a8_column_1") ; let table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_2 = add_table_postfix ("1ce53b67_1e94_413e_83cf_c6d7094289a8_column_2") ; let table_6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_0 = add_table_postfix ("6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_0") ; let table_6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_1 = add_table_postfix ("6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_1") ; let table_6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_2 = add_table_postfix ("6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_2") ; let table_35a01678_f7e2_482d_9803_c3b5a23d36ad_column_0 = add_table_postfix ("35a01678_f7e2_482d_9803_c3b5a23d36ad_column_0") ; let table_35a01678_f7e2_482d_9803_c3b5a23d36ad_column_1 = add_table_postfix ("35a01678_f7e2_482d_9803_c3b5a23d36ad_column_1") ; let table_35a01678_f7e2_482d_9803_c3b5a23d36ad_column_2 = add_table_postfix ("35a01678_f7e2_482d_9803_c3b5a23d36ad_column_2") ; let table_33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_0 = add_table_postfix ("33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_0") ; let table_33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_1 = add_table_postfix ("33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_1") ; let table_33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_2 = add_table_postfix ("33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_2") ; let table_a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_0 = add_table_postfix ("a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_0") ; let table_a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_1 = add_table_postfix ("a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_1") ; let table_a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_2 = add_table_postfix ("a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_2") ; let table_427ac837_383b_4af1_b956_3e64a78e1449_column_0 = add_table_postfix ("427ac837_383b_4af1_b956_3e64a78e1449_column_0") ; let table_427ac837_383b_4af1_b956_3e64a78e1449_column_1 = add_table_postfix ("427ac837_383b_4af1_b956_3e64a78e1449_column_1") ; let table_427ac837_383b_4af1_b956_3e64a78e1449_column_2 = add_table_postfix ("427ac837_383b_4af1_b956_3e64a78e1449_column_2") ; let table_fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_0 = add_table_postfix ("fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_0") ; let table_fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_1 = add_table_postfix ("fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_1") ; let table_fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_2 = add_table_postfix ("fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_2") ; let table_b4504737_4463_4e47_bb30_9512275c66b1_column_0 = add_table_postfix ("b4504737_4463_4e47_bb30_9512275c66b1_column_0") ; let table_b4504737_4463_4e47_bb30_9512275c66b1_column_1 = add_table_postfix ("b4504737_4463_4e47_bb30_9512275c66b1_column_1") ; let table_b4504737_4463_4e47_bb30_9512275c66b1_column_2 = add_table_postfix ("b4504737_4463_4e47_bb30_9512275c66b1_column_2") ; let table_read_one = add_table_postfix ("read_one") ; let table_update_many = add_table_postfix ("update_many") ; let table_update_one = add_table_postfix ("update_one") ; let table_delete_many = add_table_postfix ("delete_many") ; let table_delete_one = add_table_postfix ("delete_one") ; let table_names = [& table_initialization , & table_create_many , & table_create_one , & table_test_read_many_by_non_existent_primary_keys , & table_test_read_many_by_equal_to_created_primary_keys , & table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0 , & table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_1 , & table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_2 , & table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0 , & table_eb24448c_fa63_4259_bb05_3215802a78f6_column_1 , & table_eb24448c_fa63_4259_bb05_3215802a78f6_column_2 , & table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0 , & table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_1 , & table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_2 , & table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0 , & table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_1 , & table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_2 , & table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0 , & table_5a52af33_a590_403b_808e_961df6d7e7aa_column_1 , & table_5a52af33_a590_403b_808e_961df6d7e7aa_column_2 , & table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0 , & table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_1 , & table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_2 , & table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0 , & table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_1 , & table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_2 , & table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0 , & table_de556c26_9297_4adb_9483_22d474cf1e7d_column_1 , & table_de556c26_9297_4adb_9483_22d474cf1e7d_column_2 , & table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0 , & table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_1 , & table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_2 , & table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0 , & table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_1 , & table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_2 , & table_6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_0 , & table_6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_1 , & table_6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_2 , & table_35a01678_f7e2_482d_9803_c3b5a23d36ad_column_0 , & table_35a01678_f7e2_482d_9803_c3b5a23d36ad_column_1 , & table_35a01678_f7e2_482d_9803_c3b5a23d36ad_column_2 , & table_33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_0 , & table_33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_1 , & table_33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_2 , & table_a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_0 , & table_a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_1 , & table_a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_2 , & table_427ac837_383b_4af1_b956_3e64a78e1449_column_0 , & table_427ac837_383b_4af1_b956_3e64a78e1449_column_1 , & table_427ac837_383b_4af1_b956_3e64a78e1449_column_2 , & table_fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_0 , & table_fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_1 , & table_fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_2 , & table_b4504737_4463_4e47_bb30_9512275c66b1_column_0 , & table_b4504737_4463_4e47_bb30_9512275c66b1_column_1 , & table_b4504737_4463_4e47_bb30_9512275c66b1_column_2 , & table_read_one , & table_update_many , & table_update_one , & table_delete_many , & table_delete_one ,] ; let drop_all_test_tables = async || { let _unused = futures :: future :: try_join_all (table_names . iter () . map (| table_name | { let postgres_pool_3b948340 = & postgres_pool ; async move { sqlx :: query (& format ! ("drop table if exists {table_name}")) . execute (postgres_pool_3b948340) . await } })) . await . expect ("b9c1eb2e-4ead-449b-abb8-0a160cf68efd") ; } ; drop_all_test_tables () . await ; TableExample :: prepare_extensions (& postgres_pool) . await . expect ("0633ff48-ebc4-460f-a282-d750511f5d78") ; for element_dac43b91 in table_names { TableExample :: prepare_postgresql_table (& postgres_pool , element_dac43b91 ,) . await . expect ("c7952247-dc94-441b-9aef-368b8fdc593c") ; } let postgres_pool_for_tokio_spawn_sync_move = postgres_pool . clone () ; let table_names_cloned = table_names . iter () . map (| element_26b304d1 | (* element_26b304d1) . to_owned ()) . collect :: < Vec < String >> () ; let (started_tx , started_rx) = tokio :: sync :: oneshot :: channel () ; let _unused = tokio :: spawn (async move { let tcp_listener = tokio :: net :: TcpListener :: bind (app_state :: GetServiceSocketAddress :: get_service_socket_address (& config)) . await . expect ("663ae29e-bc00-4ea1-a7e9-4dddceb5b53a") ; let app_state = std :: sync :: Arc :: new (server_app_state :: ServerAppState { postgres_pool : postgres_pool_for_tokio_spawn_sync_move . clone () , config , project_git_info : & git_info :: PROJECT_GIT_INFO , }) ; started_tx . send (()) . expect ("431a6f8d-3fbb-4eb2-86f6-1b9cfd57e32e") ; axum :: serve (tcp_listener , { let mut router = axum :: Router :: new () . merge (TableExample :: routes (std :: sync :: Arc :: < server_app_state :: ServerAppState < '_ >> :: clone (& app_state))) ; for element_ef09f2b0 in table_names_cloned { router = router . merge (TableExample :: routes_handle (std :: sync :: Arc :: < server_app_state :: ServerAppState < '_ >> :: clone (& app_state) , & element_ef09f2b0)) ; } router . into_make_service () } ,) . await . expect ("71c1bc30-2f27-4fb4-8545-bc1bf21bc1ea") ; }) ; started_rx . await . expect ("87003141-43a4-4975-8ddf-273148add50f") ; let select_primary_key = postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [TableExampleSelect :: PrimaryKeyColumn (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Select :: default () ,)]) . expect ("0776170e-4dd6-4c14-a412-ce10b0c746f1") ; let ident_create_default = ident_create_default () ; let select_default_all_with_max_page_size = postgresql_crud :: NotEmptyUniqueVec :: try_new (vec ! [TableExampleSelect :: PrimaryKeyColumn (< < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Select as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize > :: default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size ()) , TableExampleSelect :: Column0 (< < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Select as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize > :: default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size ()) , TableExampleSelect :: Column1 (< < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Select as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize > :: default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size ()) , TableExampleSelect :: Column2 (< < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Select as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElementWithMaxPageSize > :: default_but_option_is_always_some_and_vec_always_contains_one_element_with_max_page_size ())]) . expect ("5e82ac66-c7d7-4e1c-8800-b2bb75b0d0cc") ; let common_read_only_ids_returned_from_create_one = { let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one_default (& url , & table_initialization) . await ; let primary_key_read = < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) ; assert_eq ! (TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : primary_key_read . clone () }) , column_0 : None , column_1 : None , column_2 : None } , generate_ident_try_read_one_handle_primary_key (& url , primary_key_read . clone () , select_primary_key . clone () , & table_initialization) . await . expect ("36b95e96-8f97-4088-86e3-c521adf8b199") , "3d9f2ec0-e374-48d2-a36b-486f5598b0b4") ; assert_eq ! (generate_try_delete_one_handle (& url , primary_key_read . clone () , & table_initialization ,) . await . expect ("4d96d385-1ff8-4cc4-a8af-b2c8c6118ad4") , primary_key_read . clone () , "26e2058b-4bc1-42da-8f35-0ab993904de5") ; generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url , primary_key_read , select_default_all_with_max_page_size . clone () , & table_initialization ,) . await ; read_only_ids_from_try_create_one } ; let column_0_read_only_ids_to_two_dimensional_vec_read_inner_acc = { let mut acc_458cda9e = Vec :: new () ; if let Some (value_a5f7e6cd) = & common_read_only_ids_returned_from_create_one . column_0 { for element_b3522b7d in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (value_a5f7e6cd) { for _ in element_b3522b7d { acc_458cda9e . push (ident_create_default . clone ()) ; } } } acc_458cda9e } ; let column_1_read_only_ids_to_two_dimensional_vec_read_inner_acc = { let mut acc_458cda9e = Vec :: new () ; if let Some (value_a5f7e6cd) = & common_read_only_ids_returned_from_create_one . column_1 { for element_b3522b7d in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (value_a5f7e6cd) { for _ in element_b3522b7d { acc_458cda9e . push (ident_create_default . clone ()) ; } } } acc_458cda9e } ; let column_2_read_only_ids_to_two_dimensional_vec_read_inner_acc = { let mut acc_458cda9e = Vec :: new () ; if let Some (value_a5f7e6cd) = & common_read_only_ids_returned_from_create_one . column_2 { for element_b3522b7d in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (value_a5f7e6cd) { for _ in element_b3522b7d { acc_458cda9e . push (ident_create_default . clone ()) ; } } } acc_458cda9e } ; futures :: StreamExt :: for_each_concurrent (futures :: stream :: iter ({ let mut acc_9189f86e : Vec < futures :: future :: BoxFuture < 'static , () >> = Vec :: new () ; for element_fce0969c in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) . chunks (10) . map (Vec :: from) { let table_create_many_cloned = table_create_many . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_vec_create = { let mut acc_92d248f7 = Vec :: new () ; for element_03a4f4ee in element_fce0969c { acc_92d248f7 . push (TableExampleCreate { column_0 : element_03a4f4ee , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () }) ; } acc_92d248f7 } ; let read_only_ids_from_try_create_many = TableExample :: try_create_many_handle (& url_cloned , TableExampleCreateManyParameters { payload : TableExampleCreateManyPayload (ident_vec_create . clone ()) } , & table_create_many_cloned . clone ()) . await . expect ("5eecedc4-bb02-454a-acd9-0af758f30b2e") ; assert_eq ! (generate_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create (read_only_ids_from_try_create_many . clone () , ident_vec_create . clone ()) , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , { let mut acc_1381c719 = Vec :: new () ; for element_bf356906 in & read_only_ids_from_try_create_many { acc_1381c719 . push (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_bf356906 . primary_key_column))) , })) ; } acc_1381c719 }))) , select_default_all_with_max_page_size_cloned . clone () , & table_create_many_cloned) . await . expect ("bdb72341-016f-4d85-8ce8-abe7e97666ca") , "d19bbbf6-f64c-4151-8b5b-998a93e13af5") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_or_primary_keys (& read_only_ids_from_try_create_many)) , column_0 : None , column_1 : None , column_2 : None })) } } , & table_create_many_cloned) . await . expect ("716e470e-d738-4642-adfc-df1f9b945d27") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , itertools :: Itertools :: sorted (read_only_ids_from_try_create_many . into_iter () . map (| element_80a93892 | { < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_80a93892 . primary_key_column) })) . collect :: < Vec < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () , "f58f5572-4286-4a74-8006-0507339910d4") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , { let mut acc_87ea12c9 = Vec :: new () ; for element_a37bca54 in & read_only_ids_from_try_delete_many { acc_87ea12c9 . push (postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (element_a37bca54 . clone ())) , })) ; } acc_87ea12c9 }))) , select_default_all_with_max_page_size_cloned . clone () , & table_create_many_cloned) . await . expect ("24ab86d6-15c9-47f1-a43f-c5fac4b38188") . is_empty () , "4e88679a-0d23-418f-8767-4e9b7531429c") ; })) ; } for element_fce0969c in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) . chunks (10) . map (Vec :: from) { let table_create_many_cloned = table_create_many . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_vec_create = { let mut acc_92d248f7 = Vec :: new () ; for element_03a4f4ee in element_fce0969c { acc_92d248f7 . push (TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element_03a4f4ee , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () }) ; } acc_92d248f7 } ; let read_only_ids_from_try_create_many = TableExample :: try_create_many_handle (& url_cloned , TableExampleCreateManyParameters { payload : TableExampleCreateManyPayload (ident_vec_create . clone ()) } , & table_create_many_cloned . clone ()) . await . expect ("5eecedc4-bb02-454a-acd9-0af758f30b2e") ; assert_eq ! (generate_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create (read_only_ids_from_try_create_many . clone () , ident_vec_create . clone ()) , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , { let mut acc_1381c719 = Vec :: new () ; for element_bf356906 in & read_only_ids_from_try_create_many { acc_1381c719 . push (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_bf356906 . primary_key_column))) , })) ; } acc_1381c719 }))) , select_default_all_with_max_page_size_cloned . clone () , & table_create_many_cloned) . await . expect ("bdb72341-016f-4d85-8ce8-abe7e97666ca") , "d19bbbf6-f64c-4151-8b5b-998a93e13af5") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_or_primary_keys (& read_only_ids_from_try_create_many)) , column_0 : None , column_1 : None , column_2 : None })) } } , & table_create_many_cloned) . await . expect ("716e470e-d738-4642-adfc-df1f9b945d27") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , itertools :: Itertools :: sorted (read_only_ids_from_try_create_many . into_iter () . map (| element_80a93892 | { < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_80a93892 . primary_key_column) })) . collect :: < Vec < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () , "f58f5572-4286-4a74-8006-0507339910d4") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , { let mut acc_87ea12c9 = Vec :: new () ; for element_a37bca54 in & read_only_ids_from_try_delete_many { acc_87ea12c9 . push (postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (element_a37bca54 . clone ())) , })) ; } acc_87ea12c9 }))) , select_default_all_with_max_page_size_cloned . clone () , & table_create_many_cloned) . await . expect ("24ab86d6-15c9-47f1-a43f-c5fac4b38188") . is_empty () , "4e88679a-0d23-418f-8767-4e9b7531429c") ; })) ; } for element_fce0969c in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) . chunks (10) . map (Vec :: from) { let table_create_many_cloned = table_create_many . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_vec_create = { let mut acc_92d248f7 = Vec :: new () ; for element_03a4f4ee in element_fce0969c { acc_92d248f7 . push (TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element_03a4f4ee }) ; } acc_92d248f7 } ; let read_only_ids_from_try_create_many = TableExample :: try_create_many_handle (& url_cloned , TableExampleCreateManyParameters { payload : TableExampleCreateManyPayload (ident_vec_create . clone ()) } , & table_create_many_cloned . clone ()) . await . expect ("5eecedc4-bb02-454a-acd9-0af758f30b2e") ; assert_eq ! (generate_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create (read_only_ids_from_try_create_many . clone () , ident_vec_create . clone ()) , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , { let mut acc_1381c719 = Vec :: new () ; for element_bf356906 in & read_only_ids_from_try_create_many { acc_1381c719 . push (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_bf356906 . primary_key_column))) , })) ; } acc_1381c719 }))) , select_default_all_with_max_page_size_cloned . clone () , & table_create_many_cloned) . await . expect ("bdb72341-016f-4d85-8ce8-abe7e97666ca") , "d19bbbf6-f64c-4151-8b5b-998a93e13af5") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_or_primary_keys (& read_only_ids_from_try_create_many)) , column_0 : None , column_1 : None , column_2 : None })) } } , & table_create_many_cloned) . await . expect ("716e470e-d738-4642-adfc-df1f9b945d27") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , itertools :: Itertools :: sorted (read_only_ids_from_try_create_many . into_iter () . map (| element_80a93892 | { < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_80a93892 . primary_key_column) })) . collect :: < Vec < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () , "f58f5572-4286-4a74-8006-0507339910d4") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , { let mut acc_87ea12c9 = Vec :: new () ; for element_a37bca54 in & read_only_ids_from_try_delete_many { acc_87ea12c9 . push (postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (element_a37bca54 . clone ())) , })) ; } acc_87ea12c9 }))) , select_default_all_with_max_page_size_cloned . clone () , & table_create_many_cloned) . await . expect ("24ab86d6-15c9-47f1-a43f-c5fac4b38188") . is_empty () , "4e88679a-0d23-418f-8767-4e9b7531429c") ; })) ; } for element_7632d698 in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let table_create_one_cloned = table_create_one . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element_7632d698 , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & table_create_one_cloned) . await ; assert_eq ! (TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) }) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_from_try_create_one . column_0 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_from_try_create_one . column_1 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_from_try_create_one . column_2 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2) } , generate_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned . clone () , & table_create_one_cloned) . await . expect ("f8e1cb88-93ef-440d-9888-49fef60182d6") , "5f2adbed-f716-440e-a990-4f1c258808b1") ; assert_eq ! (generate_try_delete_one_handle (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , & table_create_one_cloned) . await . expect ("20d5a40a-8467-481c-9715-f9b8fef63fbd") , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , "4f563faf-1d9b-4ef3-8636-f93fde8ef235") ; generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned , & table_create_one_cloned ,) . await ; })) ; } for element_7632d698 in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let table_create_one_cloned = table_create_one . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element_7632d698 , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & table_create_one_cloned) . await ; assert_eq ! (TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) }) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_from_try_create_one . column_0 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_from_try_create_one . column_1 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_from_try_create_one . column_2 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2) } , generate_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned . clone () , & table_create_one_cloned) . await . expect ("f8e1cb88-93ef-440d-9888-49fef60182d6") , "5f2adbed-f716-440e-a990-4f1c258808b1") ; assert_eq ! (generate_try_delete_one_handle (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , & table_create_one_cloned) . await . expect ("20d5a40a-8467-481c-9715-f9b8fef63fbd") , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , "4f563faf-1d9b-4ef3-8636-f93fde8ef235") ; generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned , & table_create_one_cloned ,) . await ; })) ; } for element_7632d698 in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let table_create_one_cloned = table_create_one . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element_7632d698 } ; let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & table_create_one_cloned) . await ; assert_eq ! (TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) }) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_from_try_create_one . column_0 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_from_try_create_one . column_1 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_from_try_create_one . column_2 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2) } , generate_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned . clone () , & table_create_one_cloned) . await . expect ("f8e1cb88-93ef-440d-9888-49fef60182d6") , "5f2adbed-f716-440e-a990-4f1c258808b1") ; assert_eq ! (generate_try_delete_one_handle (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , & table_create_one_cloned) . await . expect ("20d5a40a-8467-481c-9715-f9b8fef63fbd") , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , "4f563faf-1d9b-4ef3-8636-f93fde8ef235") ; generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned , & table_create_one_cloned ,) . await ; })) ; } for element_30614c66 in [1 , 2] { let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; let current_table = table_test_read_many_by_non_existent_primary_keys . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one_default (& url_cloned , & current_table) . await ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , std :: iter :: repeat_with (|| < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (uuid :: Uuid :: new_v4 ()) })) . take (element_30614c66) . collect :: < Vec < _ >> ()))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("e661c49b-2288-4548-8783-35495e193976") . is_empty () , "06df4025-e2d1-4128-b819-c06613c6ae3f") ; let _ : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read = generate_try_delete_one_handle (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , & current_table) . await . expect ("93b4bf61-3a98-42ea-ab66-015c5d211622") ; generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned , & current_table ,) . await ; })) ; } for element_a636d084 in [1 , 2] { let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; let current_table = table_test_read_many_by_equal_to_created_primary_keys . clone () ; let ident_create_default_cloned = ident_create_default . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one_default (& url_cloned , & current_table) . await ; let ident_vec_create = std :: iter :: repeat_n (ident_create_default_cloned . clone () , element_a636d084) . collect :: < Vec < TableExampleCreate >> () ; let read_only_ids_from_try_create_many = TableExample :: try_create_many_handle (& url_cloned , TableExampleCreateManyParameters { payload : TableExampleCreateManyPayload (ident_vec_create . clone ()) } , & current_table) . await . expect ("d775179f-f7b1-41d3-9c83-4ca8bd1abeec") ; assert_eq ! (generate_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create (read_only_ids_from_try_create_many . clone () , ident_vec_create . clone ()) , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , read_only_ids_from_try_create_many . iter () . map (| element_adf2b4c4 | { postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_adf2b4c4 . primary_key_column))) , } ,) }) . collect ()))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("b8efe770-153c-4e4a-ab0e-6484a8dc5343") , "error 3b2cf1f5-2c4e-4908-ba66-f4af84fe0893") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_or_primary_keys (& read_only_ids_from_try_create_many)) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("d5c23a9d-eb02-44e4-8654-e2a3d7752f51") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , itertools :: Itertools :: sorted (read_only_ids_from_try_create_many . into_iter () . map (| element_fdc88812 | { < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (element_fdc88812 . primary_key_column) }) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () , "ebbbea6e-c402-4637-9bab-02678c11926c") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , read_only_ids_from_try_delete_many . iter () . map (| element_1e9c87ce | postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (element_1e9c87ce . clone ())) , } ,)) . collect ()))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1f079962-06af-4d21-a837-c88b0e7db265") . is_empty () , "d79c0af3-5e2e-4891-a7ff-d1007b573e77") ; let _ : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read = generate_try_delete_one_handle (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , & current_table) . await . expect ("93b4bf61-3a98-42ea-ab66-015c5d211622") ; generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned , & current_table ,) . await ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("11c3740b-7c3c-4dd5-b468-71bfa2f10892") , ident_create . column_0 . clone ())]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("11c3740b-7c3c-4dd5-b468-71bfa2f10892") , ident_create . column_1 . clone ())]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_8e427ad7_5231_4f1e_8579_2e1aaa5da988_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("11c3740b-7c3c-4dd5-b468-71bfa2f10892") , ident_create . column_2 . clone ())]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("11c3740b-7c3c-4dd5-b468-71bfa2f10892") , ident_create . column_0 . clone ()))) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("11c3740b-7c3c-4dd5-b468-71bfa2f10892") , ident_create . column_1 . clone ()))) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_eb24448c_fa63_4259_bb05_3215802a78f6_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: new (postgresql_crud :: LogicalOperator :: And , < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_vec_where_equal_using_fields (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("11c3740b-7c3c-4dd5-b468-71bfa2f10892") , ident_create . column_2 . clone ())))) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_d5cd3c70) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("65cef584-1427-482f-9c42-574630badedf") , ident_create . column_0 . clone ()) { for element_48a3d976 in value_d5cd3c70 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_48a3d976]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_d5cd3c70) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("65cef584-1427-482f-9c42-574630badedf") , ident_create . column_1 . clone ()) { for element_48a3d976 in value_d5cd3c70 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_48a3d976]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_9ac6d79a_2673_4c07_be4a_01c5c20ff1ab_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_d5cd3c70) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("65cef584-1427-482f-9c42-574630badedf") , ident_create . column_2 . clone ()) { for element_48a3d976 in value_d5cd3c70 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_48a3d976]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_b02d763d) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: create_into_postgresql_type_option_vec_where_dimension_one_equal (ident_create . column_0 . clone ()) { for element_39d1fb5d in value_b02d763d . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_39d1fb5d]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_b02d763d) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: create_into_postgresql_type_option_vec_where_dimension_one_equal (ident_create . column_1 . clone ()) { for element_39d1fb5d in value_b02d763d . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_39d1fb5d]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . unwrap_or (Vec :: new ()) { let current_table = table_72940b0e_cd26_493f_9ec1_2d999d9a4401_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_b02d763d) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: create_into_postgresql_type_option_vec_where_dimension_one_equal (ident_create . column_2 . clone ()) { for element_39d1fb5d in value_b02d763d . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_39d1fb5d]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: postgresql_type_option_vec_where_greater_than_test () . map_or_else (Vec :: new , Into :: into) { let current_table = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element . create , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_60baba1f) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than (element . variant , read_only_ids_returned_from_create_one . column_0 . clone () . expect ("c8d34556-5a81-4c63-8e26-c79021eb876c") , element . greater_than ,) { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [value_60baba1f]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: postgresql_type_option_vec_where_greater_than_test () . map_or_else (Vec :: new , Into :: into) { let current_table = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element . create , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_60baba1f) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than (element . variant , read_only_ids_returned_from_create_one . column_1 . clone () . expect ("c8d34556-5a81-4c63-8e26-c79021eb876c") , element . greater_than ,) { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [value_60baba1f]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: postgresql_type_option_vec_where_greater_than_test () . map_or_else (Vec :: new , Into :: into) { let current_table = table_5a52af33_a590_403b_808e_961df6d7e7aa_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element . create } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_60baba1f) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than (element . variant , read_only_ids_returned_from_create_one . column_2 . clone () . expect ("c8d34556-5a81-4c63-8e26-c79021eb876c") , element . greater_than ,) { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [value_60baba1f]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_0 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_1 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_1f388ef8_dc28_489d_bed9_ca4e7f640dd5_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_2 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_0 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_1 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_581c947f_9b0f_452f_8e52_524088bbb2e7_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_2 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_0 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_1 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_de556c26_9297_4adb_9483_22d474cf1e7d_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_2 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_0 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_1 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_bb67b871) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("2ed000a5-cf70-4df1-903a-c1f6d224e926") , ident_create . column_2 . clone ()) { for element_3efa0bb4 in value_bb67b871 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_3efa0bb4]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_f825e068) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (ident_create . column_0 . clone ()) { for element_c09ef321 in value_f825e068 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_c09ef321]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_f825e068) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (ident_create . column_1 . clone ()) { for element_c09ef321 in value_f825e068 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_c09ef321]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_1ce53b67_1e94_413e_83cf_c6d7094289a8_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_f825e068) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_equal (ident_create . column_2 . clone ()) { for element_c09ef321 in value_f825e068 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_c09ef321]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_cd4aa374) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (ident_create . column_0 . clone ()) { for element_527b546b in value_cd4aa374 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_527b546b]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_cd4aa374) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (ident_create . column_1 . clone ()) { for element_527b546b in value_cd4aa374 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_527b546b]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_cd4aa374) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: create_into_postgresql_json_type_option_vec_where_length_greater_than (ident_create . column_2 . clone ()) { for element_527b546b in value_cd4aa374 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , vec ! [element_527b546b]) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_35a01678_f7e2_482d_9803_c3b5a23d36ad_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_0 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_35a01678_f7e2_482d_9803_c3b5a23d36ad_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_1 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_35a01678_f7e2_482d_9803_c3b5a23d36ad_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_2 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_0 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_1 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_33a3706a_ef28_4c80_88e0_b8e7fb720de2_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_2 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_0 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_1 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_a3e2165c_e030_4b31_ab3d_dcd29f27f90b_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_2 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_427ac837_383b_4af1_b956_3e64a78e1449_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_0 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_427ac837_383b_4af1_b956_3e64a78e1449_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_1 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_427ac837_383b_4af1_b956_3e64a78e1449_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_2 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_0 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_1 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_fe3267a0_f49a_42ce_8e51_2a10e5360eb8_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_greater_than (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_2 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_0 . clone ()]) { let current_table = table_b4504737_4463_4e47_bb30_9512275c66b1_column_0 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : element , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_0 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_1 . clone ()]) { let current_table = table_b4504737_4463_4e47_bb30_9512275c66b1_column_1 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : element , column_2 : < < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_1 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,) , None) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } for element in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: option_vec_create () . filter (| element_bba28182 | ! element_bba28182 . is_empty ()) . unwrap_or_else (|| vec ! [ident_create_default . column_2 . clone ()]) { let current_table = table_b4504737_4463_4e47_bb30_9512275c66b1_column_2 . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let ident_create = TableExampleCreate { column_0 : < < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_1 : < < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlType > :: Create as postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement > :: default_but_option_is_always_some_and_vec_always_contains_one_element () , column_2 : element } ; let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one (& url_cloned , ident_create . clone () , & current_table) . await ; if let Some (value_0b85c066) = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_element_regular_expression (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("df01c8ac-63e3-42f7-aae4-018c7958c00d") , ident_create . column_2 . clone ()) { for element_feacc53b in value_0b85c066 . into_vec () { assert_eq ! (vec ! [TableExampleRead { primary_key_column : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_returned_from_create_one . primary_key_column) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_0 . clone ()) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_1 . clone ()) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . clone () . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create . column_2 . clone ()) }] , generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , TableExampleWhereMany :: try_new (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: And , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_where_equal (read_only_ids_returned_from_create_one . primary_key_column , postgresql_crud :: DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement :: default_but_option_is_always_some_and_vec_always_contains_one_element ())])) , None , None , Some (postgresql_crud :: PostgresqlTypeWhere :: try_new (postgresql_crud :: LogicalOperator :: And , match element_feacc53b { postgresql_crud :: SingleOrMultiple :: Single (single) => std :: iter :: once (single) . collect () , postgresql_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () . into_iter () . collect () , }) . expect ("6b0491b2-1555-4f1c-81f7-5b22d7d353fb") ,)) . expect ("83c2d430-9ca6-4131-8ca0-8ce8ecf6af1a") , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("c3e316c0-e6da-4790-a97b-4ff09fe87a0f") , "ee8d232d-98f2-4449-ad30-0e36ca2e7094") ; } } let read_only_ids_from_try_delete_many = itertools :: Itertools :: sorted (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })])) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("338bcf89-0c3d-49d7-ac51-b73af98a32b0") . into_iter ()) . collect :: < Vec << postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () ; assert_eq ! (read_only_ids_from_try_delete_many , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column)] , "9fc29fa5-caba-403d-99da-ca9107d0c2e9") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column))) })]))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("1817b67a-c6c5-4fea-8ca7-23581c1888a3") . is_empty () , "38187925-c136-41de-940d-eba75efc3a39") ; })) ; } acc_9189f86e . push ({ let table_read_one_cloned = table_read_one . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; futures :: FutureExt :: boxed (async move { generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read :: new (uuid :: Uuid :: new_v4 ()) , select_default_all_with_max_page_size_cloned . clone () , & table_read_one_cloned ,) . await ; }) }) ; for (index_7f181188 , read_only_ids_current_element) in generate_read_only_ids_current_elements (& url , & table_update_many , select_default_all_with_max_page_size . clone () , column_0_read_only_ids_to_two_dimensional_vec_read_inner_acc . clone ()) . await . into_iter () . enumerate () { let table_update_many_cloned = table_update_many . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let previous_read = itertools :: Itertools :: sorted_by (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column))) , })]))) , select_default_all_with_max_page_size_cloned . clone () , & table_update_many_cloned) . await . expect ("540ec737-dea7-4d50-a42a-45ea1f81d6c1") . into_iter () , | first , second | { match (& first . primary_key_column , & second . primary_key_column) { (Some (first_handle) , Some (second_handle)) => first_handle . value . cmp (& second_handle . value) , _ => panic ! ("99ba9dc3-ca32-4462-b9b4-b1202265beee") , } }) ; let update = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped ({ let mut index_e0c50b3e : usize = 0 ; let mut option_test_case = None ; for element_76abae3a in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& read_only_ids_current_element . column_0 . clone () . expect ("af7d979d-0d43-47e9-bbf6-07282cae7eff")) { let mut should_break = false ; for element_72f5ad12 in element_76abae3a { if index_e0c50b3e == index_7f181188 { option_test_case = Some (element_72f5ad12) ; should_break = true ; break ; } index_e0c50b3e = index_e0c50b3e . checked_add (1) . expect ("0968dda6-4c3a-42b6-8aa1-83058a928dde") ; } if should_break { break ; } } option_test_case . expect ("769983ba-3992-4298-97ab-9a3721c32359") }) ; assert_eq ! (vec ! [TableExampleReadOnlyIds { primary_key_column : read_only_ids_current_element . primary_key_column , column_0 : Some (< postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: update_to_read_only_ids (& update)) , column_1 : None , column_2 : None }] , TableExample :: try_update_many_handle (& url_cloned , TableExampleUpdateManyParameters { payload : TableExampleUpdateManyPayload :: try_new (vec ! [TableExampleUpdate :: try_new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Update :: from (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_update (read_only_ids_current_element . primary_key_column)) , Some (postgresql_crud :: Value { value : update . clone () }) , None , None) . expect ("42dc87b3-2ac4-4e66-8287-91aeb13f0ee8")]) . expect ("69e1bd8a-fe78-4301-85ca-f4f3958d7493") } , & table_update_many_cloned) . await . expect ("d2de0bd6-1b01-4ef2-b074-a60878241b52") , "34bfb3c7-7a53-479e-9d4f-0856003573e1") ; assert_eq ! ({ previous_read . into_iter () . map (| element_a6bc6b2f | TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) }) , column_0 : Some (postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: previous_read_merged_with_option_update_into_read (< postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_current_element . column_0 . clone () . expect ("96213542-59eb-4767-b120-d9ba575087c8")) . expect ("bf0d6f55-7457-4ec1-8b79-50efad297ccb") . value , Some (update . clone ())) }) , column_1 : element_a6bc6b2f . column_1 , column_2 : element_a6bc6b2f . column_2 }) . collect :: < Vec < TableExampleRead >> () } , itertools :: Itertools :: sorted_by (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column))) , })]))) , select_default_all_with_max_page_size_cloned , & table_update_many_cloned) . await . expect ("25c561e2-6b39-4982-8fe7-4473d12b3271") . into_iter () , | first , second | match (& first . primary_key_column , & second . primary_key_column) { (Some (first_handle) , Some (second_handle)) => first_handle . value . cmp (& second_handle . value) , _ => panic ! ("3c827ad6-30bb-49db-8f49-8c903a236040") , }) . collect :: < Vec < TableExampleRead >> () , "ae2a2da5-3697-4fd7-9ad2-4a535618fbc3") ; })) ; } for (index_7f181188 , read_only_ids_current_element) in generate_read_only_ids_current_elements (& url , & table_update_many , select_default_all_with_max_page_size . clone () , column_1_read_only_ids_to_two_dimensional_vec_read_inner_acc . clone ()) . await . into_iter () . enumerate () { let table_update_many_cloned = table_update_many . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let previous_read = itertools :: Itertools :: sorted_by (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column))) , })]))) , select_default_all_with_max_page_size_cloned . clone () , & table_update_many_cloned) . await . expect ("540ec737-dea7-4d50-a42a-45ea1f81d6c1") . into_iter () , | first , second | { match (& first . primary_key_column , & second . primary_key_column) { (Some (first_handle) , Some (second_handle)) => first_handle . value . cmp (& second_handle . value) , _ => panic ! ("99ba9dc3-ca32-4462-b9b4-b1202265beee") , } }) ; let update = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped ({ let mut index_e0c50b3e : usize = 0 ; let mut option_test_case = None ; for element_76abae3a in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& read_only_ids_current_element . column_1 . clone () . expect ("af7d979d-0d43-47e9-bbf6-07282cae7eff")) { let mut should_break = false ; for element_72f5ad12 in element_76abae3a { if index_e0c50b3e == index_7f181188 { option_test_case = Some (element_72f5ad12) ; should_break = true ; break ; } index_e0c50b3e = index_e0c50b3e . checked_add (1) . expect ("0968dda6-4c3a-42b6-8aa1-83058a928dde") ; } if should_break { break ; } } option_test_case . expect ("769983ba-3992-4298-97ab-9a3721c32359") }) ; assert_eq ! (vec ! [TableExampleReadOnlyIds { primary_key_column : read_only_ids_current_element . primary_key_column , column_0 : None , column_1 : Some (< postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: update_to_read_only_ids (& update)) , column_2 : None }] , TableExample :: try_update_many_handle (& url_cloned , TableExampleUpdateManyParameters { payload : TableExampleUpdateManyPayload :: try_new (vec ! [TableExampleUpdate :: try_new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Update :: from (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_update (read_only_ids_current_element . primary_key_column)) , None , Some (postgresql_crud :: Value { value : update . clone () }) , None) . expect ("42dc87b3-2ac4-4e66-8287-91aeb13f0ee8")]) . expect ("69e1bd8a-fe78-4301-85ca-f4f3958d7493") } , & table_update_many_cloned) . await . expect ("d2de0bd6-1b01-4ef2-b074-a60878241b52") , "34bfb3c7-7a53-479e-9d4f-0856003573e1") ; assert_eq ! ({ previous_read . into_iter () . map (| element_a6bc6b2f | TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) }) , column_0 : element_a6bc6b2f . column_0 , column_1 : Some (postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: previous_read_merged_with_option_update_into_read (< postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_current_element . column_1 . clone () . expect ("96213542-59eb-4767-b120-d9ba575087c8")) . expect ("bf0d6f55-7457-4ec1-8b79-50efad297ccb") . value , Some (update . clone ())) }) , column_2 : element_a6bc6b2f . column_2 }) . collect :: < Vec < TableExampleRead >> () } , itertools :: Itertools :: sorted_by (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column))) , })]))) , select_default_all_with_max_page_size_cloned , & table_update_many_cloned) . await . expect ("25c561e2-6b39-4982-8fe7-4473d12b3271") . into_iter () , | first , second | match (& first . primary_key_column , & second . primary_key_column) { (Some (first_handle) , Some (second_handle)) => first_handle . value . cmp (& second_handle . value) , _ => panic ! ("3c827ad6-30bb-49db-8f49-8c903a236040") , }) . collect :: < Vec < TableExampleRead >> () , "ae2a2da5-3697-4fd7-9ad2-4a535618fbc3") ; })) ; } for (index_7f181188 , read_only_ids_current_element) in generate_read_only_ids_current_elements (& url , & table_update_many , select_default_all_with_max_page_size . clone () , column_2_read_only_ids_to_two_dimensional_vec_read_inner_acc . clone ()) . await . into_iter () . enumerate () { let table_update_many_cloned = table_update_many . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let previous_read = itertools :: Itertools :: sorted_by (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column))) , })]))) , select_default_all_with_max_page_size_cloned . clone () , & table_update_many_cloned) . await . expect ("540ec737-dea7-4d50-a42a-45ea1f81d6c1") . into_iter () , | first , second | { match (& first . primary_key_column , & second . primary_key_column) { (Some (first_handle) , Some (second_handle)) => first_handle . value . cmp (& second_handle . value) , _ => panic ! ("99ba9dc3-ca32-4462-b9b4-b1202265beee") , } }) ; let update = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped ({ let mut index_e0c50b3e : usize = 0 ; let mut option_test_case = None ; for element_76abae3a in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& read_only_ids_current_element . column_2 . clone () . expect ("af7d979d-0d43-47e9-bbf6-07282cae7eff")) { let mut should_break = false ; for element_72f5ad12 in element_76abae3a { if index_e0c50b3e == index_7f181188 { option_test_case = Some (element_72f5ad12) ; should_break = true ; break ; } index_e0c50b3e = index_e0c50b3e . checked_add (1) . expect ("0968dda6-4c3a-42b6-8aa1-83058a928dde") ; } if should_break { break ; } } option_test_case . expect ("769983ba-3992-4298-97ab-9a3721c32359") }) ; assert_eq ! (vec ! [TableExampleReadOnlyIds { primary_key_column : read_only_ids_current_element . primary_key_column , column_0 : None , column_1 : None , column_2 : Some (< postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: update_to_read_only_ids (& update)) }] , TableExample :: try_update_many_handle (& url_cloned , TableExampleUpdateManyParameters { payload : TableExampleUpdateManyPayload :: try_new (vec ! [TableExampleUpdate :: try_new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Update :: from (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_update (read_only_ids_current_element . primary_key_column)) , None , None , Some (postgresql_crud :: Value { value : update . clone () })) . expect ("42dc87b3-2ac4-4e66-8287-91aeb13f0ee8")]) . expect ("69e1bd8a-fe78-4301-85ca-f4f3958d7493") } , & table_update_many_cloned) . await . expect ("d2de0bd6-1b01-4ef2-b074-a60878241b52") , "34bfb3c7-7a53-479e-9d4f-0856003573e1") ; assert_eq ! ({ previous_read . into_iter () . map (| element_a6bc6b2f | TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) }) , column_0 : element_a6bc6b2f . column_0 , column_1 : element_a6bc6b2f . column_1 , column_2 : Some (postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: previous_read_merged_with_option_update_into_read (< postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_current_element . column_2 . clone () . expect ("96213542-59eb-4767-b120-d9ba575087c8")) . expect ("bf0d6f55-7457-4ec1-8b79-50efad297ccb") . value , Some (update . clone ())) }) }) . collect :: < Vec < TableExampleRead >> () } , itertools :: Itertools :: sorted_by (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , vec ! [postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlWhere :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: into_inner (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column))) , })]))) , select_default_all_with_max_page_size_cloned , & table_update_many_cloned) . await . expect ("25c561e2-6b39-4982-8fe7-4473d12b3271") . into_iter () , | first , second | match (& first . primary_key_column , & second . primary_key_column) { (Some (first_handle) , Some (second_handle)) => first_handle . value . cmp (& second_handle . value) , _ => panic ! ("3c827ad6-30bb-49db-8f49-8c903a236040") , }) . collect :: < Vec < TableExampleRead >> () , "ae2a2da5-3697-4fd7-9ad2-4a535618fbc3") ; })) ; } for (index_26824592 , read_only_ids_current_element) in generate_read_only_ids_current_elements (& url , & table_update_one , select_default_all_with_max_page_size . clone () , column_0_read_only_ids_to_two_dimensional_vec_read_inner_acc) . await . into_iter () . enumerate () { let table_update_one_cloned = table_update_one . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let previous_read = generate_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) , select_default_all_with_max_page_size_cloned . clone () , & table_update_one_cloned) . await . expect ("e6998b47-c593-449e-89fd-3888de9f84a6") ; let update = < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped ({ let mut index_e0d2f9db : usize = 0 ; let mut option_test_case = None ; for element_3a9a65ee in < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& read_only_ids_current_element . column_0 . clone () . expect ("c4d98a71-f30f-410e-b410-a75f4672f2f7")) { let mut should_break = false ; for element_bb734c11 in element_3a9a65ee { if index_e0d2f9db == index_26824592 { option_test_case = Some (element_bb734c11) ; should_break = true ; break ; } index_e0d2f9db = index_e0d2f9db . checked_add (1) . expect ("326274d1-199d-4c43-89b3-c61c8ecdfd77") ; } if should_break { break ; } } option_test_case . expect ("bd79056e-bd30-4eda-b913-2afffaf1bfc3") }) ; assert_eq ! (TableExampleReadOnlyIds { primary_key_column : read_only_ids_current_element . primary_key_column , column_0 : Some (< postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: update_to_read_only_ids (& update)) , column_1 : None , column_2 : None } , TableExample :: try_update_one_handle (& url_cloned , TableExampleUpdateOneParameters { payload : TableExampleUpdate :: try_new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Update :: from (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_update (read_only_ids_current_element . primary_key_column)) , Some (postgresql_crud :: Value { value : update . clone () }) , None , None) . expect ("0e5d65a5-12c8-4c48-a24c-0f1fe376ada2") } , & table_update_one_cloned) . await . expect ("4d755542-e3e3-4c68-a3a2-beb654cf5b80") , "564de31c-3664-4c62-85fc-e03793372f8f") ; assert_eq ! (TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) }) , column_0 : Some (postgresql_crud :: Value { value : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: previous_read_merged_with_option_update_into_read (< postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_current_element . column_0 . clone () . expect ("4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")) . expect ("c7685b19-9bca-47bc-a3a5-8fc543b174a5") . value , Some (update . clone ())) }) , column_1 : previous_read . column_1 , column_2 : previous_read . column_2 } , generate_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) , select_default_all_with_max_page_size_cloned , & table_update_one_cloned) . await . expect ("75894c76-88f5-406e-ab46-c27b1c7d4212") , "d5dec823-b1f9-49b2-9c24-bf788f08cd8c") ; })) ; } for (index_26824592 , read_only_ids_current_element) in generate_read_only_ids_current_elements (& url , & table_update_one , select_default_all_with_max_page_size . clone () , column_1_read_only_ids_to_two_dimensional_vec_read_inner_acc) . await . into_iter () . enumerate () { let table_update_one_cloned = table_update_one . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let previous_read = generate_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) , select_default_all_with_max_page_size_cloned . clone () , & table_update_one_cloned) . await . expect ("e6998b47-c593-449e-89fd-3888de9f84a6") ; let update = < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped ({ let mut index_e0d2f9db : usize = 0 ; let mut option_test_case = None ; for element_3a9a65ee in < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& read_only_ids_current_element . column_1 . clone () . expect ("c4d98a71-f30f-410e-b410-a75f4672f2f7")) { let mut should_break = false ; for element_bb734c11 in element_3a9a65ee { if index_e0d2f9db == index_26824592 { option_test_case = Some (element_bb734c11) ; should_break = true ; break ; } index_e0d2f9db = index_e0d2f9db . checked_add (1) . expect ("326274d1-199d-4c43-89b3-c61c8ecdfd77") ; } if should_break { break ; } } option_test_case . expect ("bd79056e-bd30-4eda-b913-2afffaf1bfc3") }) ; assert_eq ! (TableExampleReadOnlyIds { primary_key_column : read_only_ids_current_element . primary_key_column , column_0 : None , column_1 : Some (< postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: update_to_read_only_ids (& update)) , column_2 : None } , TableExample :: try_update_one_handle (& url_cloned , TableExampleUpdateOneParameters { payload : TableExampleUpdate :: try_new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Update :: from (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_update (read_only_ids_current_element . primary_key_column)) , None , Some (postgresql_crud :: Value { value : update . clone () }) , None) . expect ("0e5d65a5-12c8-4c48-a24c-0f1fe376ada2") } , & table_update_one_cloned) . await . expect ("4d755542-e3e3-4c68-a3a2-beb654cf5b80") , "564de31c-3664-4c62-85fc-e03793372f8f") ; assert_eq ! (TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) }) , column_0 : previous_read . column_0 , column_1 : Some (postgresql_crud :: Value { value : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: previous_read_merged_with_option_update_into_read (< postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_current_element . column_1 . clone () . expect ("4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")) . expect ("c7685b19-9bca-47bc-a3a5-8fc543b174a5") . value , Some (update . clone ())) }) , column_2 : previous_read . column_2 } , generate_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) , select_default_all_with_max_page_size_cloned , & table_update_one_cloned) . await . expect ("75894c76-88f5-406e-ab46-c27b1c7d4212") , "d5dec823-b1f9-49b2-9c24-bf788f08cd8c") ; })) ; } for (index_26824592 , read_only_ids_current_element) in generate_read_only_ids_current_elements (& url , & table_update_one , select_default_all_with_max_page_size . clone () , column_2_read_only_ids_to_two_dimensional_vec_read_inner_acc) . await . into_iter () . enumerate () { let table_update_one_cloned = table_update_one . clone () ; let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let previous_read = generate_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) , select_default_all_with_max_page_size_cloned . clone () , & table_update_one_cloned) . await . expect ("e6998b47-c593-449e-89fd-3888de9f84a6") ; let update = < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_inner_into_update_with_new_or_try_new_unwraped ({ let mut index_e0d2f9db : usize = 0 ; let mut option_test_case = None ; for element_3a9a65ee in < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_two_dimensional_vec_read_inner (& read_only_ids_current_element . column_2 . clone () . expect ("c4d98a71-f30f-410e-b410-a75f4672f2f7")) { let mut should_break = false ; for element_bb734c11 in element_3a9a65ee { if index_e0d2f9db == index_26824592 { option_test_case = Some (element_bb734c11) ; should_break = true ; break ; } index_e0d2f9db = index_e0d2f9db . checked_add (1) . expect ("326274d1-199d-4c43-89b3-c61c8ecdfd77") ; } if should_break { break ; } } option_test_case . expect ("bd79056e-bd30-4eda-b913-2afffaf1bfc3") }) ; assert_eq ! (TableExampleReadOnlyIds { primary_key_column : read_only_ids_current_element . primary_key_column , column_0 : None , column_1 : None , column_2 : Some (< postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: update_to_read_only_ids (& update)) } , TableExample :: try_update_one_handle (& url_cloned , TableExampleUpdateOneParameters { payload : TableExampleUpdate :: try_new (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Update :: from (< postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_update (read_only_ids_current_element . primary_key_column)) , None , None , Some (postgresql_crud :: Value { value : update . clone () })) . expect ("0e5d65a5-12c8-4c48-a24c-0f1fe376ada2") } , & table_update_one_cloned) . await . expect ("4d755542-e3e3-4c68-a3a2-beb654cf5b80") , "564de31c-3664-4c62-85fc-e03793372f8f") ; assert_eq ! (TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) }) , column_0 : previous_read . column_0 , column_1 : previous_read . column_1 , column_2 : Some (postgresql_crud :: Value { value : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: previous_read_merged_with_option_update_into_read (< postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& read_only_ids_current_element . column_2 . clone () . expect ("4f19d0d2-a23f-4b77-a2bc-c7b04db7a129")) . expect ("c7685b19-9bca-47bc-a3a5-8fc543b174a5") . value , Some (update . clone ())) }) } , generate_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_current_element . primary_key_column) , select_default_all_with_max_page_size_cloned , & table_update_one_cloned) . await . expect ("75894c76-88f5-406e-ab46-c27b1c7d4212") , "d5dec823-b1f9-49b2-9c24-bf788f08cd8c") ; })) ; } for element_39819198 in [1 , 2] { let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; let current_table = table_test_read_many_by_equal_to_created_primary_keys . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one_default (& url_cloned , & current_table) . await ; assert ! (TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , std :: iter :: repeat_with (|| < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresqlTableTypeDeclaration :: new (uuid :: Uuid :: new_v4 ()) })) . take (element_39819198) . collect ())) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("0d5dec47-8b2e-4f02-909b-3a58b65bc6a5") . is_empty () , "51d14103-5122-4d96-a45c-4dd958ab3adc") ; let _ : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read = generate_try_delete_one_handle (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , & current_table) . await . expect ("93b4bf61-3a98-42ea-ab66-015c5d211622") ; generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned , & current_table ,) . await ; })) ; } ; for element_56409d32 in [1 , 2] { let url_cloned = url . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; let current_table = table_test_read_many_by_equal_to_created_primary_keys . clone () ; let ident_create_default_cloned = ident_create_default . clone () ; acc_9189f86e . push (futures :: FutureExt :: boxed (async move { let read_only_ids_from_try_create_one = generate_read_only_ids_from_try_create_one_default (& url_cloned , & current_table) . await ; let read_only_ids_from_try_create_many = TableExample :: try_create_many_handle (& url_cloned , TableExampleCreateManyParameters { payload : TableExampleCreateManyPayload (std :: iter :: repeat_n (ident_create_default_cloned , element_56409d32) . collect ()) } , & current_table) . await . expect ("b8695890-65fb-469b-a6f9-be481d648eb9") ; let read_only_ids_from_try_delete_many = TableExample :: try_delete_many_handle (& url_cloned , TableExampleDeleteManyParameters { payload : TableExampleDeleteManyPayload { where_many : StdOptionOptionTableExampleWhereMany (Some (TableExampleWhereMany { primary_key_column : Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , read_only_ids_from_try_create_many . iter () . map (| element_3bb88958 | < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_table_type_declaration (element_3bb88958 . primary_key_column) , })) . collect ())) , column_0 : None , column_1 : None , column_2 : None })) , } , } , & current_table) . await . expect ("b80b91b8-7de1-4ea2-97cf-1987a5f7cc57") ; assert_eq ! (read_only_ids_from_try_delete_many , { read_only_ids_from_try_create_many . iter () . map (| element_ba0f6b1c | < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_to_option_value_read_default_but_option_is_always_some_and_vec_always_contains_one_element (& element_ba0f6b1c . primary_key_column) . expect ("3ee5ee86-05dc-4dc8-9262-8ffa1855d5e4") . value) . collect :: < Vec < < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read >> () } , "db5e88a6-c75b-421b-acfb-56931b97ba3b") ; assert ! (generate_try_read_many_order_by_primary_key_with_big_pagination (& url_cloned , generate_ident_where_many_pripery_key_others_none (Some (generate_postgresql_type_where_try_new_primary_key (postgresql_crud :: LogicalOperator :: Or , read_only_ids_from_try_delete_many . into_iter () . map (| element_adcc8db3 | < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Where :: Equal (postgresql_crud :: PostgresqlTypeWhereEqual { logical_operator : postgresql_crud :: LogicalOperator :: Or , value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_into_table_type_declaration (element_adcc8db3) , })) . collect ()))) , select_default_all_with_max_page_size_cloned . clone () , & current_table) . await . expect ("bcb79917-ee81-416e-82a3-f43a823266a3") . is_empty () , "77f038b0-6f39-4b5b-a402-a1b6142acd0d") ; let _ : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read = generate_try_delete_one_handle (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , & current_table) . await . expect ("93b4bf61-3a98-42ea-ab66-015c5d211622") ; generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url_cloned , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_from_try_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned , & current_table ,) . await ; })) ; } ; acc_9189f86e . push ({ let table_delete_one_cloned = table_delete_one . clone () ; let select_default_all_with_max_page_size_cloned = select_default_all_with_max_page_size . clone () ; futures :: FutureExt :: boxed (async move { if let Err (error) = generate_try_delete_one_handle (& url , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlType > :: Read :: new (uuid :: Uuid :: new_v4 ()) , & table_delete_one_cloned) . await { if let TableExampleTryDeleteOneErrorNamed :: TableExampleDeleteOneErrorNamedWithSerializeDeserialize { delete_one_error_named_with_serialize_deserialize , .. } = error { if let TableExampleDeleteOneErrorNamedWithSerializeDeserialize :: Postgresql { postgresql , .. } = delete_one_error_named_with_serialize_deserialize { assert ! (postgresql == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row () , "c9261bb8-d391-4c4b-9707-3a2c4278ad90") ; } else { panic ! ("e63b27a3-f3e3-4f19-998a-88ce798b08cc") ; } } else { panic ! ("47a8e0d9-1f95-4fa7-91dc-a94955195204") } } else { panic ! ("9be62f9f-31d9-493c-bb0f-b83b6ecb0026") } let read_only_ids_returned_from_create_one = generate_read_only_ids_from_try_create_one_default (& url , & table_delete_one_cloned) . await ; assert_eq ! (TableExampleRead { primary_key_column : Some (postgresql_crud :: Value { value : < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column) }) , column_0 : < postgresql_crud :: StdPrimitiveI16AsNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_0 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create_default . column_0) , column_1 : < postgresql_crud :: OptionStdPrimitiveI16AsNullableInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_1 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create_default . column_1) , column_2 : < postgresql_crud :: VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2 as postgresql_crud :: PostgresqlTypeTestCases > :: read_only_ids_merged_with_create_into_option_value_read (read_only_ids_returned_from_create_one . column_2 . expect ("f967434c-f45a-47f9-a289-36ef99d80e33") , ident_create_default . column_2) } , generate_ident_try_read_one_handle_primary_key (& url , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned . clone () , & table_delete_one_cloned) . await . expect ("c8c44c89-aeb0-43d3-ae72-02b7a5979f5a") , "86ef08ae-4356-4417-9490-1d13eb2af71f") ; assert_eq ! (generate_try_delete_one_handle (& url , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column) , & table_delete_one_cloned) . await . expect ("7e1d1a70-8f93-43b9-9cfe-37fc240ca7ba") , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column) , "99f81971-dc80-46db-b466-4f309b215a8c") ; generate_check_no_rows_returned_from_ident_try_read_one_handle_primary_key (& url , < postgresql_crud :: SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql as postgresql_crud :: PostgresqlTypePrimaryKey > :: read_only_ids_into_read (read_only_ids_returned_from_create_one . primary_key_column) , select_default_all_with_max_page_size_cloned . clone () , & table_delete_one_cloned ,) . await ; }) }) ; acc_9189f86e }) , 100 , async | fut | { fut . await ; } ,) . await ; drop_all_test_tables () . await ; }) ;
    }
}
