#[tokio::test]
async fn integration_subscribe_returns_a_200_for_valid_form_data() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    wiremock::Mock::given(wiremock::matchers::path("/email"))
        .and(wiremock::matchers::method("POST"))
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    let response = app.post_subscriptions(body.into()).await;
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn integration_subscribe_persists_the_new_subscriber() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    wiremock::Mock::given(wiremock::matchers::path("/email"))
        .and(wiremock::matchers::method("POST"))
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    app.post_subscriptions(body.into()).await;
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn integration_subscribe_returns_a_400_when_data_is_missing() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn integration_subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];
    for (body, description) in test_cases {
        let response = app.post_subscriptions(body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}

#[tokio::test]
async fn integration_subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    wiremock::Mock::given(wiremock::matchers::path("/email"))
        .and(wiremock::matchers::method("POST"))
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.expect("inside integration_subscribe_sends_a_confirmation_email_for_valid_data app.email_server.received_requests() failed")[0];
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).expect("inside integration_subscribe_sends_a_confirmation_email_for_valid_data serde_json::from_slice failed");
    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    let html_link = get_link(body["HtmlBody"].as_str().expect("inside integration_subscribe_sends_a_confirmation_email_for_valid_data get_link(body[\"HtmlBody\"].as_str() failed"));
    let text_link = get_link(body["TextBody"].as_str().expect("inside integration_subscribe_sends_a_confirmation_email_for_valid_data get_link(body[\"TextBody\"].as_str() failed"));
    assert_eq!(html_link, text_link);
}

#[tokio::test]
async fn integration_subscribe_sends_a_confirmation_email_with_a_link() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    wiremock::Mock::given(wiremock::matchers::path("/email"))
        .and(wiremock::matchers::method("POST"))
        .respond_with(wiremock::ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.expect("inside  integration_subscribe_sends_a_confirmation_email_with_a_link app.email_server.received_requests().await failed")[0];
    let confirmation_links = app.get_confirmation_links(email_request);
    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
async fn integration_subscribe_fails_if_there_is_a_fatal_database_error() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    sqlx::query!("ALTER TABLE subscriptions DROP COLUMN email;",)
        .execute(&app.db_pool)
        .await
        .expect("inside integration_subscribe_fails_if_there_is_a_fatal_database_error sqlx::query!().execute().await failed");
    let response = app.post_subscriptions(body.into()).await;
    assert_eq!(response.status().as_u16(), 500);
}
