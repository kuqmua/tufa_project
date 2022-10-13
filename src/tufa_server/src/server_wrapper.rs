use crate::configuration::ApplicationSettings;
use crate::configuration::DatabaseSettings;
use crate::configuration::EmailClientSettings;
use crate::configuration::Settings;
use crate::issue_delivery_worker::run_worker_until_stopped;
use crate::lazy_static::config::CONFIG;
use crate::startup::Application;
use crate::startup::ApplicationBuildErrorEnum;
use secrecy::Secret;
use std::fmt::Debug;
use std::fmt::Display;
use tokio::task::JoinError;
use tufa_common::helpers::redis::get_redis_url::get_redis_url;

#[actix_web::main] // or #[tokio::main]
pub async fn server_wrapper() -> Result<(), Box<ApplicationBuildErrorEnum>> {
    let configuration = Settings {
        database: DatabaseSettings {
            host: CONFIG.postgres_ip.clone(),
            port: CONFIG.postgres_port,
            username: CONFIG.postgres_login.clone(),
            password: Secret::new(CONFIG.postgres_password.clone()),
            database_name: CONFIG.postgres_db.clone(),
            require_ssl: CONFIG.require_ssl,
        },
        application: ApplicationSettings {
            port: CONFIG.server_port,
            host: CONFIG.server_ip.clone(),
            base_url: CONFIG.base_url.clone(),
            hmac_secret: Secret::new(CONFIG.hmac_secret.clone()),
        },
        email_client: EmailClientSettings {
            base_url: CONFIG.base_url.clone(),
            sender_email: "test@gmail.com".to_string(),
            authorization_token: Secret::new("my-secret-token".to_string()),
            timeout_milliseconds: 10000,
        },
        redis_uri: Secret::new(get_redis_url(&CONFIG.redis_ip, CONFIG.redis_port)),
    };
    let application = match Application::build(configuration.clone()).await {
        Ok(app) => app,
        Err(e) => return Err(e),
    };
    let application_task = tokio::spawn(application.run_until_stopped()).await;
    //remove this coz too much spam
    // match application_task {
    //     Ok(_) => todo!(),
    //     Err(_) => todo!(),
    // }
    // let worker_task = tokio::spawn(run_worker_until_stopped(configuration));
    // tokio::select! {
    //     o = application_task => report_exit("API", o),
    //     o = worker_task => report_exit("Background worker", o),
    // };
    Ok(())
}

// fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
//     match outcome {
//         Ok(Ok(())) => {
//             tracing::info!("{} has exited", task_name)
//         }
//         Ok(Err(e)) => {
//             tracing::error!(
//                 error.cause_chain = ?e,
//                 error.message = %e,
//                 "{} failed",
//                 task_name
//             )
//         }
//         Err(e) => {
//             tracing::error!(
//                 error.cause_chain = ?e,
//                 error.message = %e,
//                 "{}' task failed to complete",
//                 task_name
//             )
//         }
//     }
// }
