// pub async fn get_saved_response<'a>(
//     pool: &sqlx::PgPool,
//     idempotency_key: &common::repositories_types::server::idempotency::IdempotencyKey,
//     user_id: uuid::Uuid,
// ) -> Result<Option<actix_web::HttpResponse>, common::repositories_types::server::idempotency::persistence::GetSavedResponseErrorNamed>{
//     use common::repositories_types::server::idempotency::persistence::HeaderPairRecord;
//     //todo - sqlx::query! is a macro to check db on compile time. DATABASE_URL must be set in env variables. its not for lib. change it later
//     let saved_response = match sqlx::query!(
//         r#"
//         SELECT
//             response_status_code as "response_status_code!",
//             response_headers as "response_headers!: Vec<HeaderPairRecord>",
//             response_body as "response_body!"
//         FROM idempotency
//         WHERE
//           user_id = $1 AND
//           idempotency_key = $2
//         "#,
//         user_id,
//         idempotency_key.as_ref()
//     )
//     .fetch_optional(pool)
//     .await
//     {
//         Err(error) => {
//             return Err(common::repositories_types::server::idempotency::persistence::GetSavedResponseErrorNamed::PostgresSelect {
//                 postgres_select: error,
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             });
//         }
//         Ok(option_record) => option_record,
//     };
//     if let Some(r) = saved_response {
//         match r.response_status_code.try_into() {
//             Err(error) => Err(common::repositories_types::server::idempotency::persistence::GetSavedResponseErrorNamed::TryFromIntError {
//                 try_from_int_error: error,
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             }),
//             Ok(status_code_as_u16) => match actix_web::http::StatusCode::from_u16(status_code_as_u16) {
//                 Err(error) => Err(common::repositories_types::server::idempotency::persistence::GetSavedResponseErrorNamed::InvalidStatusCode {
//                     invalid_status_code: error,
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 }),
//                 Ok(status_code) => {
//                     let mut response = actix_web::HttpResponse::build(status_code);
//                     for common::repositories_types::server::idempotency::persistence::HeaderPairRecord { name, value } in r.response_headers {
//                         response.append_header((name, value));
//                     }
//                     Ok(Some(response.body(r.response_body)))
//                 },
//             },
//         }
//     } else {
//         Ok(None)
//     }
// }

// pub async fn save_response<'a>(
//     mut transaction: sqlx::Transaction<'static, sqlx::Postgres>,
//     idempotency_key: &common::repositories_types::server::idempotency::IdempotencyKey,
//     user_id: uuid::Uuid,
//     http_response: actix_web::HttpResponse,
// ) -> Result<
//     actix_web::HttpResponse,
//     common::repositories_types::server::idempotency::persistence::SaveResponseErrorNamed<
//         'a,
//     >,
// > {
//     let (response_head, body) = http_response.into_parts();
//     // `MessageBody::Error` is not `Send` + `Sync`,
//     let body = match actix_web::body::to_bytes(body).await {
//         Err(error) => {
//             return Err(common::repositories_types::server::idempotency::persistence::SaveResponseErrorNamed::BodyToBytes {
//                 body_to_bytes: e.into(),
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             });
//         }
//         Ok(bytes) => bytes,
//     };
//     let status_code = response_head.status().as_u16() as i16;
//     let headers = {
//         let mut h = Vec::with_capacity(response_head.headers().len());
//         for (name, value) in response_head.headers().iter() {
//             let name = name.as_str().to_owned();
//             let value = value.as_bytes().to_owned();
//             h.push(common::repositories_types::server::idempotency::persistence::HeaderPairRecord { name, value });
//         }
//         h
//     };
//     if let Err(error) = sqlx::query_unchecked!(
//         r#"
//         UPDATE idempotency
//         SET
//             response_status_code = $3,
//             response_headers = $4,
//             response_body = $5
//         WHERE
//             user_id = $1 AND
//             idempotency_key = $2
//         "#,
//         user_id,
//         idempotency_key.as_ref(),
//         status_code,
//         headers,
//         body.as_ref()
//     )
//     .execute(&mut transaction)
//     .await
//     {
//         return Err(common::repositories_types::server::idempotency::persistence::SaveResponseErrorNamed::PostgtesUpdate {
//             postgres_update: error,
//             code_occurence: error_occurence_lib::code_occurence!(),
//         });
//     };
//     if let Err(error) = transaction.commit().await {
//         return Err(common::repositories_types::server::idempotency::persistence::SaveResponseErrorNamed::PostgtesTransactionCommit {
//             postgres_transaction_commit: error,
//             code_occurence: error_occurence_lib::code_occurence!(),
//         });
//     }
//     // We need `.map_into_boxed_body` to go from
//     // `actix_web::HttpResponse<Bytes>` to `actix_web::HttpResponse<BoxBody>`
//     let http_response = response_head.set_body(body).map_into_boxed_body();
//     Ok(http_response)
// }

// pub async fn try_processing<'a>(
//     pool: &sqlx::PgPool,
//     idempotency_key: &common::repositories_types::server::idempotency::IdempotencyKey,
//     user_id: uuid::Uuid,
// ) -> Result<
//     common::repositories_types::server::idempotency::NextAction,
//     common::repositories_types::server::idempotency::persistence::TryProcessingErrorNamed<
//         'a,
//     >,
// > {
//     let mut transaction = match pool.begin().await {
//         Err(error) => {
//             return Err(common::repositories_types::server::idempotency::persistence::TryProcessingErrorNamed::PostgresPoolBegin {
//                 pool_begin_error: error,
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             });
//         }
//         Ok(transaction) => transaction,
//     };
//     let n_inserted_rows = match sqlx::query!(
//         r#"
//         INSERT INTO idempotency (
//             user_id,
//             idempotency_key,
//             created_at
//         )
//         VALUES ($1, $2, now())
//         ON CONFLICT DO NOTHING
//         "#,
//         user_id,
//         idempotency_key.as_ref()
//     )
//     .execute(&mut transaction)
//     .await
//     {
//         Err(error) => {
//             return Err(common::repositories_types::server::idempotency::persistence::TryProcessingErrorNamed::PostgresInsert {
//                 insert: error,
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             });
//         }
//         Ok(pg_query_result) => pg_query_result.rows_affected(),
//     };
//     if n_inserted_rows > 0 {
//         Ok(
//             common::repositories_types::server::idempotency::NextAction::StartProcessing(
//                 transaction,
//             ),
//         )
//     } else {
//         match get_saved_response(pool, idempotency_key, user_id).await {
//             Err(error) => {
//                 return Err(common::repositories_types::server::idempotency::persistence::TryProcessingErrorNamed::GetSavedResponse {
//                     get_saved_response: error,
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 });
//             },
//             Ok(option_http_response) => match option_http_response {
//                 None => Err(common::repositories_types::server::idempotency::persistence::TryProcessingErrorNamed::SavedResponseIsNone {
//                     message: "We expected a saved response, we didn't find it",
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 }),
//                 Some(saved_response) => Ok(common::repositories_types::server::idempotency::NextAction::ReturnSavedResponse(saved_response)),
//             },
//         }
//     }
// }
