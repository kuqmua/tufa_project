// #[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, app_state))]
// pub async fn confirm<'a>(
//     parameters: actix_web::web::Query<
//         common::repositories_types::server::routes::Parameters,
//     >,
//     app_state: actix_web::web::Data<
//         common::repositories_types::server::routes::app_state::AppInfo<'a>,
//     >,
// ) -> actix_web::HttpResponse {
//     let id =
//         match get_subscriber_id_from_token(&app_state.postgres_pool, &parameters.subscription_token)
//             .await
//         {
//             Ok(id) => id,
//             Err(_) => return actix_web::HttpResponse::InternalServerError().finish(),
//         };
//     match id {
//         None => actix_web::HttpResponse::Unauthorized().finish(),
//         Some(subscriber_id) => {
//             if confirm_subscriber(&app_state.postgres_pool, subscriber_id)
//                 .await
//                 .is_err()
//             {
//                 return actix_web::HttpResponse::InternalServerError().finish();
//             }
//             actix_web::HttpResponse::Ok().finish()
//         }
//     }
// }

// #[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, pool))]
// pub async fn confirm_subscriber(
//     pool: &sqlx::PgPool,
//     subscriber_id: uuid::Uuid,
// ) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
//         subscriber_id,
//     )
//     .execute(pool)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//     })?;
//     Ok(())
// }

// #[tracing::instrument(name = "Get subscriber_id from token", skip(subscription_token, pool))]
// pub async fn get_subscriber_id_from_token(
//     pool: &sqlx::PgPool,
//     subscription_token: &str,
// ) -> Result<Option<uuid::Uuid>, sqlx::Error> {
//     let result = sqlx::query!(
//         r#"SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1"#,
//         subscription_token,
//     )
//     .fetch_optional(pool)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//     })?;
//     Ok(result.map(|r| r.subscriber_id))
// }
