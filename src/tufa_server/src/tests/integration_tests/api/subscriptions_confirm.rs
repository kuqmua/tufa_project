use crate::tests::integration_tests::api::helpers::spawn_app;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::Mock;
use wiremock::ResponseTemplate;

#[tokio::test]
async fn integration_confirmations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;
    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .expect("inside integration_confirmations_without_token_are_rejected_with_a_400 reqwest::get().await failed");
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn integration_the_link_returned_by_subscribe_returns_a_200_if_called() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.expect("inside integration_the_link_returned_by_subscribe_returns_a_200_if_called app.email_server.received_requests().await failed")[0];
    let confirmation_links = app.get_confirmation_links(email_request);
    let response = reqwest::get(confirmation_links.html).await.expect("inside integration_the_link_returned_by_subscribe_returns_a_200_if_called reqwest::get().await failed");
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn integration_clicking_on_the_confirmation_link_confirms_a_subscriber() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.expect("inside integration_clicking_on_the_confirmation_link_confirms_a_subscriber app.email_server.received_requests() failed")[0];
    let confirmation_links = app.get_confirmation_links(email_request);
    reqwest::get(confirmation_links.html)
        .await
        .expect("inside integration_clicking_on_the_confirmation_link_confirms_a_subscriber reqwest::get().await failed")
        .error_for_status()
        .expect("inside integration_clicking_on_the_confirmation_link_confirms_a_subscriber reqwest::get().await.error_for_status() failed");
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "confirmed");
}
