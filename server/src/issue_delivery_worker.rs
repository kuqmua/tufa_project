// pub async fn run_worker_until_stopped(
//     config: &'static (
//         impl common::traits::get_email_client::GetEmailClient
//         + common::traits::get_postgres_connection_pool::GetPostgresConnectionPool
//         + common::common::config::config_fields::GetSourcePlaceType
//         + common::common::config::config_fields::GetTimezone
//         + std::marker::Send
//         + std::marker::Sync
//     )
// ) {// -> Result<(), Error>
//     async move {
//         loop {
//             match try_execute_task(config).await {
//                 Ok(common::repositories_types::server::issue_delivery_worker::ExecutionOutcome::EmptyQueue) => {
//                     println!("task queue is empty");
//                     tokio::time::sleep(std::time::Duration::from_secs(10)).await;
//                 }
//                 Ok(common::repositories_types::server::issue_delivery_worker::ExecutionOutcome::TaskCompleted) => {}
//                 Err(error) => {
//                     use common::common::error_logs_logic::error_log::ErrorLog;
//                     e.error_log(config);
//                     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//                 }
//             }
//         }
//     }.await;
// }

// pub async fn try_execute_task<'a>(
//     config: &'static (
//         impl common::traits::get_email_client::GetEmailClient
//         + common::traits::get_postgres_connection_pool::GetPostgresConnectionPool
//     )
// ) -> Result<common::repositories_types::server::issue_delivery_worker::ExecutionOutcome, common::repositories_types::server::issue_delivery_worker::TryExecuteTaskErrorNamed> {
//     let pool = &config.get_postgres_connection_pool();
//     let email_client = &config.get_email_client();
//     let task = match dequeue_task(pool).await {
//         Err(error) => {
//             return Err(common::repositories_types::server::issue_delivery_worker::TryExecuteTaskErrorNamed::DequeueTask {
//                 dequeue_task: error,
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             });
//         },
//         Ok(option_task) => option_task,
//     };
//     match task {
//         None => Ok(common::repositories_types::server::issue_delivery_worker::ExecutionOutcome::EmptyQueue),
//         Some(task) => {
//             let (transaction, issue_id, email) = task;
//             tracing::Span::current()
//                 .record("newsletter_issue_id", &tracing::field::display(issue_id))
//                 .record("subscriber_email", &tracing::field::display(&email));
//             match common::repositories_types::server::domain::SubscriberEmail::try_from(email.clone()) {
//                 Ok(email) => {
//                     let issue = match get_issue(pool, issue_id).await {
//                         Err(error) => {
//                             return Err(common::repositories_types::server::issue_delivery_worker::TryExecuteTaskErrorNamed::GetIssue {
//                                 get_issue: error,
//                                 code_occurence: error_occurence_lib::code_occurence!(),
//                             });
//                         },
//                         Ok(newletter_issue) => newletter_issue,
//                     };
//                     if let Err(error) = email_client
//                         .send_email(
//                             &email,
//                             &issue.title,
//                             &issue.html_content,
//                             &issue.text_content,
//                         )
//                         .await
//                     {
//                         tracing::error!(
//                             error.cause_chain = ?e,
//                             error.message = %e,
//                             "Failed to deliver issue to a confirmed subscriber. \
//                                 Skipping.",
//                         );
//                     }
//                 }
//                 Err(error) => {
//                     tracing::error!(
//                         error.cause_chain = ?e,
//                         error.message = %e,
//                         "Skipping a confirmed subscriber. \
//                             Their stored contact details are invalid",
//                     );
//                 }
//             }
//             if let Err(error) = delete_task(transaction, issue_id, &email).await {
//                 return Err(common::repositories_types::server::issue_delivery_worker::TryExecuteTaskErrorNamed::DeleteTask {
//                     delete_task: error,
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 });
//             }
//             Ok(common::repositories_types::server::issue_delivery_worker::ExecutionOutcome::TaskCompleted)
//         }
//     }
// }

// async fn dequeue_task<'a>(
//     pool: &sqlx::PgPool,
// ) -> Result<Option<(sqlx::Transaction<'static, sqlx::Postgres>, uuid::Uuid, std::string::String)>, common::repositories_types::server::issue_delivery_worker::DequeueTaskErrorNamed> {
//     match pool.begin().await {
//         Err(error) => Err(common::repositories_types::server::issue_delivery_worker::DequeueTaskErrorNamed::PostgresPoolBegin {
//             postgres_pool_begin: error,
//             code_occurence: error_occurence_lib::code_occurence!(),
//         }),
//         Ok(mut transaction) => match sqlx::query!(
//             r#"
//             SELECT newsletter_issue_id, subscriber_email
//             FROM issue_delivery_queue
//             FOR UPDATE
//             SKIP LOCKED
//             LIMIT 1
//             "#,
//         )
//         .fetch_optional(&mut transaction)
//         .await {
//             Err(error) => Err(common::repositories_types::server::issue_delivery_worker::DequeueTaskErrorNamed::PostgresSelect {
//                 postgres_select: error,
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             }),
//             Ok(option_record) => match option_record {
//                 Some(record) => Ok(Some((
//                     transaction,
//                     record.newsletter_issue_id,
//                     record.subscriber_email,
//                 ))),
//                 None => Ok(None),
//             }
//         },
//     }
//     Ok(None)
// }

// #[tracing::instrument(skip_all)]
// async fn delete_task<'a>(
//     mut transaction: sqlx::Transaction<'static, sqlx::Postgres>,
//     issue_id: uuid::Uuid,
//     email: &str,
// ) -> Result<(), common::repositories_types::server::issue_delivery_worker::DeleteTaskErrorNamed> {
//     if let Err(error) = sqlx::query!(
//         r#"
//         DELETE FROM issue_delivery_queue
//         WHERE
//             newsletter_issue_id = $1 AND
//             subscriber_email = $2
//         "#,
//         issue_id,
//         email
//     )
//     .execute(&mut transaction)
//     .await {
//         return Err(common::repositories_types::server::issue_delivery_worker::DeleteTaskErrorNamed::PostgresDeleteTask {
//             postgres_delete_task: error,
//             code_occurence: error_occurence_lib::code_occurence!(),
//         });
//     }
//     if let Err(error) = transaction.commit().await {
//         return Err(common::repositories_types::server::issue_delivery_worker::DeleteTaskErrorNamed::PostgresTransactionCommit {
//             postgres_transaction_commit: error,
//             code_occurence: error_occurence_lib::code_occurence!(),
//         });
//     }
//     Ok(())
// }

// #[tracing::instrument(skip_all)]
// async fn get_issue<'a>(pool: &sqlx::PgPool, issue_id: uuid::Uuid) -> Result<common::repositories_types::server::issue_delivery_worker::NewsletterIssue, common::repositories_types::server::issue_delivery_worker::GetIssueErrorNamed> {
//     match sqlx::query_as!(
//         common::repositories_types::server::issue_delivery_worker::NewsletterIssue,
//         r#"
//         SELECT title, text_content, html_content
//         FROM newsletter_issues
//         WHERE
//             newsletter_issue_id = $1
//         "#,
//         issue_id
//     )
//     .fetch_one(pool)
//     .await {
//         Err(error) => Err(common::repositories_types::server::issue_delivery_worker::GetIssueErrorNamed::PostgresSelectNewsletterIssues {
//             postgres_select_newsletter_issues: error,
//             code_occurence: error_occurence_lib::code_occurence!(),
//         }),
//         Ok(issue) => Ok(issue)
//     }
// }
