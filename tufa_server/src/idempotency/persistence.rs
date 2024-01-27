// pub async fn get_saved_response<'a>(
//     pool: &sqlx::PgPool,
//     idempotency_key: &tufa_common::repositories_types::tufa_server::idempotency::IdempotencyKey,
//     user_id: uuid::Uuid,
// ) -> Result<Option<actix_web::HttpResponse>, tufa_common::repositories_types::tufa_server::idempotency::persistence::GetSavedResponseErrorNamed>{
//     use tufa_common::repositories_types::tufa_server::idempotency::persistence::HeaderPairRecord;
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
//         Err(e) => {
//             return Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::GetSavedResponseErrorNamed::PostgresSelect {
//                 postgres_select: e,
//                 code_occurence: tufa_common::code_occurence!(),
//             });
//         }
//         Ok(option_record) => option_record,
//     };
//     if let Some(r) = saved_response {
//         match r.response_status_code.try_into() {
//             Err(e) => Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::GetSavedResponseErrorNamed::TryFromIntError {
//                 try_from_int_error: e,
//                 code_occurence: tufa_common::code_occurence!(),
//             }),
//             Ok(status_code_as_u16) => match actix_web::http::StatusCode::from_u16(status_code_as_u16) {
//                 Err(e) => Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::GetSavedResponseErrorNamed::InvalidStatusCode {
//                     invalid_status_code: e,
//                     code_occurence: tufa_common::code_occurence!(),
//                 }),
//                 Ok(status_code) => {
//                     let mut response = actix_web::HttpResponse::build(status_code);
//                     for tufa_common::repositories_types::tufa_server::idempotency::persistence::HeaderPairRecord { name, value } in r.response_headers {
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
//     idempotency_key: &tufa_common::repositories_types::tufa_server::idempotency::IdempotencyKey,
//     user_id: uuid::Uuid,
//     http_response: actix_web::HttpResponse,
// ) -> Result<
//     actix_web::HttpResponse,
//     tufa_common::repositories_types::tufa_server::idempotency::persistence::SaveResponseErrorNamed<
//         'a,
//     >,
// > {
//     let (response_head, body) = http_response.into_parts();
//     // `MessageBody::Error` is not `Send` + `Sync`,
//     let body = match actix_web::body::to_bytes(body).await {
//         Err(e) => {
//             return Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::SaveResponseErrorNamed::BodyToBytes {
//                 body_to_bytes: e.into(),
//                 code_occurence: tufa_common::code_occurence!(),
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
//             h.push(tufa_common::repositories_types::tufa_server::idempotency::persistence::HeaderPairRecord { name, value });
//         }
//         h
//     };
//     if let Err(e) = sqlx::query_unchecked!(
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
//         return Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::SaveResponseErrorNamed::PostgtesUpdate {
//             postgres_update: e,
//             code_occurence: tufa_common::code_occurence!(),
//         });
//     };
//     if let Err(e) = transaction.commit().await {
//         return Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::SaveResponseErrorNamed::PostgtesTransactionCommit {
//             postgres_transaction_commit: e,
//             code_occurence: tufa_common::code_occurence!(),
//         });
//     }
//     // We need `.map_into_boxed_body` to go from
//     // `actix_web::HttpResponse<Bytes>` to `actix_web::HttpResponse<BoxBody>`
//     let http_response = response_head.set_body(body).map_into_boxed_body();
//     Ok(http_response)
// }

// pub async fn try_processing<'a>(
//     pool: &sqlx::PgPool,
//     idempotency_key: &tufa_common::repositories_types::tufa_server::idempotency::IdempotencyKey,
//     user_id: uuid::Uuid,
// ) -> Result<
//     tufa_common::repositories_types::tufa_server::idempotency::NextAction,
//     tufa_common::repositories_types::tufa_server::idempotency::persistence::TryProcessingErrorNamed<
//         'a,
//     >,
// > {
//     let mut transaction = match pool.begin().await {
//         Err(e) => {
//             return Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::TryProcessingErrorNamed::PostgresPoolBegin {
//                 pool_begin_error: e,
//                 code_occurence: tufa_common::code_occurence!(),
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
//         Err(e) => {
//             return Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::TryProcessingErrorNamed::PostgresInsert {
//                 insert: e,
//                 code_occurence: tufa_common::code_occurence!(),
//             });
//         }
//         Ok(pg_query_result) => pg_query_result.rows_affected(),
//     };
//     if n_inserted_rows > 0 {
//         Ok(
//             tufa_common::repositories_types::tufa_server::idempotency::NextAction::StartProcessing(
//                 transaction,
//             ),
//         )
//     } else {
//         match get_saved_response(pool, idempotency_key, user_id).await {
//             Err(e) => {
//                 return Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::TryProcessingErrorNamed::GetSavedResponse {
//                     get_saved_response: e,
//                     code_occurence: tufa_common::code_occurence!(),
//                 });
//             },
//             Ok(option_http_response) => match option_http_response {
//                 None => Err(tufa_common::repositories_types::tufa_server::idempotency::persistence::TryProcessingErrorNamed::SavedResponseIsNone {
//                     message: "We expected a saved response, we didn't find it",
//                     code_occurence: tufa_common::code_occurence!(),
//                 }),
//                 Some(saved_response) => Ok(tufa_common::repositories_types::tufa_server::idempotency::NextAction::ReturnSavedResponse(saved_response)),
//             },
//         }
//     }
// }
