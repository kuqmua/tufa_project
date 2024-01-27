// pub async fn run_worker_until_stopped(
//     config: &'static (
//         impl tufa_common::traits::get_email_client::GetEmailClient
//         + tufa_common::traits::get_postgres_connection_pool::GetPostgresConnectionPool
//         + tufa_common::common::config::config_fields::GetSourcePlaceType
//         + tufa_common::common::config::config_fields::GetTimezone
//         + std::marker::Send
//         + std::marker::Sync
//     )
// ) {// -> Result<(), Error>
//     async move {
//         loop {
//             match try_execute_task(config).await {
//                 Ok(tufa_common::repositories_types::tufa_server::issue_delivery_worker::ExecutionOutcome::EmptyQueue) => {
//                     println!("task queue is empty");
//                     tokio::time::sleep(std::time::Duration::from_secs(10)).await;
//                 }
//                 Ok(tufa_common::repositories_types::tufa_server::issue_delivery_worker::ExecutionOutcome::TaskCompleted) => {}
//                 Err(e) => {
//                     use tufa_common::common::error_logs_logic::error_log::ErrorLog;
//                     e.error_log(config);
//                     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//                 }
//             }
//         }
//     }.await;
// }

// pub async fn try_execute_task<'a>(
//     config: &'static (
//         impl tufa_common::traits::get_email_client::GetEmailClient
//         + tufa_common::traits::get_postgres_connection_pool::GetPostgresConnectionPool
//     )
// ) -> Result<tufa_common::repositories_types::tufa_server::issue_delivery_worker::ExecutionOutcome, tufa_common::repositories_types::tufa_server::issue_delivery_worker::TryExecuteTaskErrorNamed> {
//     let pool = &config.get_postgres_connection_pool();
//     let email_client = &config.get_email_client();
//     let task = match dequeue_task(pool).await {
//         Err(e) => {
//             return Err(tufa_common::repositories_types::tufa_server::issue_delivery_worker::TryExecuteTaskErrorNamed::DequeueTask {
//                 dequeue_task: e,
//                 code_occurence: tufa_common::code_occurence!(),
//             });
//         },
//         Ok(option_task) => option_task,
//     };
//     match task {
//         None => Ok(tufa_common::repositories_types::tufa_server::issue_delivery_worker::ExecutionOutcome::EmptyQueue),
//         Some(task) => {
//             let (transaction, issue_id, email) = task;
//             tracing::Span::current()
//                 .record("newsletter_issue_id", &tracing::field::display(issue_id))
//                 .record("subscriber_email", &tracing::field::display(&email));
//             match tufa_common::repositories_types::tufa_server::domain::SubscriberEmail::try_from(email.clone()) {
//                 Ok(email) => {
//                     let issue = match get_issue(pool, issue_id).await {
//                         Err(e) => {
//                             return Err(tufa_common::repositories_types::tufa_server::issue_delivery_worker::TryExecuteTaskErrorNamed::GetIssue {
//                                 get_issue: e,
//                                 code_occurence: tufa_common::code_occurence!(),
//                             });
//                         },
//                         Ok(newletter_issue) => newletter_issue,
//                     };
//                     if let Err(e) = email_client
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
//                 Err(e) => {
//                     tracing::error!(
//                         error.cause_chain = ?e,
//                         error.message = %e,
//                         "Skipping a confirmed subscriber. \
//                             Their stored contact details are invalid",
//                     );
//                 }
//             }
//             if let Err(e) = delete_task(transaction, issue_id, &email).await {
//                 return Err(tufa_common::repositories_types::tufa_server::issue_delivery_worker::TryExecuteTaskErrorNamed::DeleteTask {
//                     delete_task: e,
//                     code_occurence: tufa_common::code_occurence!(),
//                 });
//             }
//             Ok(tufa_common::repositories_types::tufa_server::issue_delivery_worker::ExecutionOutcome::TaskCompleted)
//         }
//     }
// }

// async fn dequeue_task<'a>(
//     pool: &sqlx::PgPool,
// ) -> Result<Option<(sqlx::Transaction<'static, sqlx::Postgres>, uuid::Uuid, std::string::String)>, tufa_common::repositories_types::tufa_server::issue_delivery_worker::DequeueTaskErrorNamed> {
//     match pool.begin().await {
//         Err(e) => Err(tufa_common::repositories_types::tufa_server::issue_delivery_worker::DequeueTaskErrorNamed::PostgresPoolBegin {
//             postgres_pool_begin: e,
//             code_occurence: tufa_common::code_occurence!(),
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
//             Err(e) => Err(tufa_common::repositories_types::tufa_server::issue_delivery_worker::DequeueTaskErrorNamed::PostgresSelect {
//                 postgres_select: e,
//                 code_occurence: tufa_common::code_occurence!(),
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
// ) -> Result<(), tufa_common::repositories_types::tufa_server::issue_delivery_worker::DeleteTaskErrorNamed> {
//     if let Err(e) = sqlx::query!(
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
//         return Err(tufa_common::repositories_types::tufa_server::issue_delivery_worker::DeleteTaskErrorNamed::PostgresDeleteTask {
//             postgres_delete_task: e,
//             code_occurence: tufa_common::code_occurence!(),
//         });
//     }
//     if let Err(e) = transaction.commit().await {
//         return Err(tufa_common::repositories_types::tufa_server::issue_delivery_worker::DeleteTaskErrorNamed::PostgresTransactionCommit {
//             postgres_transaction_commit: e,
//             code_occurence: tufa_common::code_occurence!(),
//         });
//     }
//     Ok(())
// }

// #[tracing::instrument(skip_all)]
// async fn get_issue<'a>(pool: &sqlx::PgPool, issue_id: uuid::Uuid) -> Result<tufa_common::repositories_types::tufa_server::issue_delivery_worker::NewsletterIssue, tufa_common::repositories_types::tufa_server::issue_delivery_worker::GetIssueErrorNamed> {
//     match sqlx::query_as!(
//         tufa_common::repositories_types::tufa_server::issue_delivery_worker::NewsletterIssue,
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
//         Err(e) => Err(tufa_common::repositories_types::tufa_server::issue_delivery_worker::GetIssueErrorNamed::PostgresSelectNewsletterIssues {
//             postgres_select_newsletter_issues: e,
//             code_occurence: tufa_common::code_occurence!(),
//         }),
//         Ok(issue) => Ok(issue)
//     }
// }
