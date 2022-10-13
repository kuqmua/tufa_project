use crate::configuration::get_configuration;
use crate::configuration::DatabaseSettings;
use crate::email_client::EmailClient;
use crate::issue_delivery_worker::try_execute_task;
use crate::issue_delivery_worker::ExecutionOutcome;
use crate::startup::get_connection_pool;
use crate::startup::Application;
use crate::telemetry::get_subscriber;
use crate::telemetry::init_subscriber;
use argon2::password_hash::SaltString;
use argon2::Algorithm;
use argon2::Argon2;
use argon2::Params;
use argon2::PasswordHasher;
use argon2::Version;
use once_cell::sync::Lazy;
use sqlx::Connection;
use sqlx::Executor;
use sqlx::PgConnection;
use sqlx::PgPool;
use uuid::Uuid;
use wiremock::MockServer;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber).expect("cannot init tracing subscriber std::io::stdout");
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber).expect("cannot init tracing subscriber std::io::sink");
    };
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub email_server: MockServer,
    pub test_user: TestUser,
    pub api_client: reqwest::Client,
    pub email_client: EmailClient,
}

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

impl TestApp {
    pub async fn dispatch_all_pending_emails(&self) {
        loop {
            if let ExecutionOutcome::EmptyQueue =
                try_execute_task(&self.db_pool, &self.email_client)
                    .await
                    .expect("try_execute_task failed")
            {
                break;
            }
        }
    }
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/login", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn get_login_html(&self) -> String {
        self.api_client
            .get(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
            .text()
            .await
            .expect("inside get_login_html .text().await failed")
    }
    pub async fn get_admin_dashboard(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/admin/dashboard", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn get_admin_dashboard_html(&self) -> String {
        self.get_admin_dashboard()
            .await
            .text()
            .await
            .expect("get_admin_dashboard().await.text().await failed")
    }
    pub async fn get_change_password(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/admin/password", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn get_change_password_html(&self) -> String {
        self.get_change_password().await.text().await.expect(
            "inside get_change_password_html get_change_password().await.text().await failed",
        )
    }
    pub async fn post_logout(&self) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/admin/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn post_change_password<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/admin/password", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn get_publish_newsletter(&self) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/admin/newsletters", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub async fn get_publish_newsletter_html(&self) -> String {
        self.get_publish_newsletter().await.text().await.expect(
            "inside get_publish_newsletter_html get_publish_newsletter().await.text().await failed",
        )
    }
    pub async fn post_publish_newsletter<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/admin/newsletters", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body)
            .expect("inside get_confirmation_links serde_json::from_slice failed");
        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            assert_eq!(links.len(), 1);
            let raw_link = links[0].as_str().to_owned();
            let mut confirmation_link = reqwest::Url::parse(&raw_link)
                .expect("inside get_confirmation_links reqwest::Url::parse failed");
            assert_eq!(
                confirmation_link
                    .host_str()
                    .expect("inside get_confirmation_links confirmation_link.host_str() failed"),
                "127.0.0.1"
            );
            confirmation_link
                .set_port(Some(self.port))
                .expect("inside get_confirmation_links confirmation_link.set_port failed");
            confirmation_link
        };
        let html =
            get_link(body["HtmlBody"].as_str().expect(
                "inside get_confirmation_links get_link(body[\"HtmlBody\"].as_str() failed",
            ));
        let plain_text =
            get_link(body["TextBody"].as_str().expect(
                "inside get_confirmation_links get_link(body[\"TextBody\"].as_str() failed",
            ));
        ConfirmationLinks { html, plain_text }
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let email_server = MockServer::start().await;
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c.email_client.base_url = email_server.uri();
        c
    };
    configure_database(&configuration.database).await;
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .expect("inside spawn_app Client::builder().redirect().cookie_store().build() failed");
    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
        test_user: TestUser::generate(),
        api_client: client,
        email_client: configuration.email_client.client(),
    };
    test_app.test_user.store(&test_app.db_pool).await;
    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

pub struct TestUser {
    user_id: Uuid,
    pub username: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }
    pub async fn login(&self, app: &TestApp) {
        app.post_login(&serde_json::json!({
            "username": &self.username,
            "password": &self.password
        }))
        .await;
    }
    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            //todo- move it into config
            Params::new(15000, 2, 1, None)
                .expect("inside store Params::new(15000, 2, 1, None) failed"),
        )
        .hash_password(self.password.as_bytes(), &salt)
        .expect("inside store Argon2::new().hash_password() failed")
        .to_string();
        sqlx::query!(
            "INSERT INTO users (user_id, username, password_hash)
            VALUES ($1, $2, $3)",
            self.user_id,
            self.username,
            password_hash,
        )
        .execute(pool)
        .await
        .expect("Failed to store test user.");
    }
}

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response.headers().get("Location").expect(
            "inside assert_is_redirect_to assert_eq!(response.headers().get(\"Location\") failed"
        ),
        location
    );
}
