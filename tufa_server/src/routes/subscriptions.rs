// pub async fn subscribe<'a>(
//     form: actix_web::web::Form<tufa_common::repositories_types::tufa_server::routes::FormData>,
//     app_info: actix_web::web::Data<
//         tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo<'a>,
//     >,
//     email_client: actix_web::web::Data<
//         tufa_common::repositories_types::tufa_server::email_client::EmailClient,
//     >,
//     base_url: actix_web::web::Data<std::string::String>,
// ) -> Result<
//     actix_web::HttpResponse,
//     tufa_common::repositories_types::tufa_server::routes::SubscribeErrorNamed,
// > {
//     let new_subscriber: tufa_common::repositories_types::tufa_server::domain::NewSubscriber =
//         match form.0.try_into() {
//             Err(e) => {
//                 return Err(tufa_common::repositories_types::tufa_server::routes::subscriptions::SubscribeErrorNamed::TryIntoNewSubscriber {
//                 try_into_new_subscriber: e,
//                 code_occurence: tufa_common::code_occurence!(),
//             });
//             }
//             Ok(new_subscriber) => new_subscriber,
//         };
//     //"Failed to acquire a Postgres connection from the pool"
//     let mut transaction = match app_info.postgres_pool.begin().await {
//         Err(e) => {
//             return Err(tufa_common::repositories_types::tufa_server::routes::subscriptions::SubscribeErrorNamed::PostgresPoolBegin {
//                 postgres_pool_begin: e,
//                 code_occurence: tufa_common::code_occurence!(),
//             });
//         }
//         Ok(transaction) => transaction,
//     };
//     //"Failed to insert new subscriber in the database."
//     let subscriber_id = match insert_subscriber(&mut transaction, &new_subscriber).await {
//         Err(e) => {
//             return Err(tufa_common::repositories_types::tufa_server::routes::subscriptions::SubscribeErrorNamed::InsertSubscriber {
//                 insert_subscriber: e,
//                 code_occurence: tufa_common::code_occurence!(),
//             });
//         }
//         Ok(subscriber_id) => subscriber_id,
//     };
//     let subscription_token = tufa_common::repositories_types::tufa_server::routes::subscriptions::generate_subscription_token();
//     //"Failed to store the confirmation token for a new subscriber."
//     if let Err(e) = store_token(&mut transaction, subscriber_id, &subscription_token).await {
//         return Err(tufa_common::repositories_types::tufa_server::routes::subscriptions::SubscribeErrorNamed::StoreToken {
//             store_token: e,
//             code_occurence: tufa_common::code_occurence!(),
//         });
//     }
//     //"Failed to commit SQL transaction to store a new subscriber."
//     if let Err(e) = transaction.commit().await {
//         return Err(tufa_common::repositories_types::tufa_server::routes::subscriptions::SubscribeErrorNamed::PostgresTransactionCommit {
//             postgres_transaction_commit: e,
//             code_occurence: tufa_common::code_occurence!(),
//         });
//     }
//     //"Failed to send a confirmation email."
//     if let Err(e) = tufa_common::repositories_types::tufa_server::routes::send_confirmation_email(
//         &email_client,
//         new_subscriber,
//         &base_url,
//         &subscription_token,
//     )
//     .await
//     {
//         return Err(tufa_common::repositories_types::tufa_server::routes::subscriptions::SubscribeErrorNamed::SendConfirmationEmail {
//             send_confirmation_email: e,
//             code_occurence: tufa_common::code_occurence!(),
//         });
//     }
//     Ok(actix_web::HttpResponse::Ok().finish())
// }

// #[tracing::instrument(
//     name = "Saving new subscriber details in the database",
//     skip(new_subscriber, transaction)
// )]
// pub async fn insert_subscriber(
//     transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
//     new_subscriber: &tufa_common::repositories_types::tufa_server::domain::NewSubscriber,
// ) -> Result<uuid::Uuid, sqlx::Error> {
//     let subscriber_id = uuid::Uuid::new_v4();
//     sqlx::query!(
//         r#"
//     INSERT INTO subscriptions (id, email, name, subscribed_at, status)
//     VALUES ($1, $2, $3, $4, 'pending_confirmation')
//             "#,
//         subscriber_id,
//         new_subscriber.email.as_ref(),
//         new_subscriber.name.as_ref(),
//         chrono::Utc::now()
//     )
//     .execute(transaction)
//     .await?;
//     Ok(subscriber_id)
// }

// #[tracing::instrument(
//     name = "Store subscription token in the database",
//     skip(subscription_token, transaction)
// )]
// pub async fn store_token(
//     transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
//     subscriber_id: uuid::Uuid,
//     subscription_token: &str,
// ) -> Result<(), tufa_common::repositories_types::tufa_server::routes::StoreTokenError> {
//     sqlx::query!(
//         r#"
//     INSERT INTO subscription_tokens (subscription_token, subscriber_id)
//     VALUES ($1, $2)
//         "#,
//         subscription_token,
//         subscriber_id
//     )
//     .execute(transaction)
//     .await
//     .map_err(tufa_common::repositories_types::tufa_server::routes::StoreTokenError)?;
//     Ok(())
// }
