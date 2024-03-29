async fn create_unconfirmed_subscriber(
    app: &crate::tests::integration_tests::api::helpers::TestApp,
) -> crate::tests::integration_tests::api::helpers::ConfirmationLinks {
    let name: std::string::String = fake::Fake::fake(&fake::faker::name::en::Name());
    let email: std::string::String = fake::Fake::fake(&fake::faker::internet::en::SafeEmail());
    let body = serde_urlencoded::to_string(&serde_json::json!({
        "name": name,
        "email": email
    }))
    .expect("inside create_unconfirmed_subscriber serde_urlencoded::to_string failed");
    let _mock_guard = wiremock::Mock::given(wiremock::matchers::path("/email"))
        .and(wiremock::matchers::method("POST"))
        .respond_with(wiremock::ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;
    app.post_subscriptions(body)
        .await
        .error_for_status()
        .expect("inside create_unconfirmed_subscriber app.post_subscriptions().await().error_for_status() failed");
    let email_request = &app
        .email_server
        .received_requests()
        .await
        .expect("inside create_unconfirmed_subscriber app.email_server.received_requests().await failed")
        .pop()
        .expect("inside create_unconfirmed_subscriber app.email_server.received_requests().await.pop() failed");
    app.get_confirmation_links(email_request)
}

async fn create_confirmed_subscriber(app: &crate::tests::integration_tests::api::helpers::TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await.html;
    reqwest::get(confirmation_link)
        .await
        .expect("inside create_confirmed_subscriber reqwest::get().await failed")
        .error_for_status()
        .expect(
            "inside create_confirmed_subscriber reqwest::get().await.error_for_status() failed",
        );
}

#[tokio::test]
async fn integration_newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    create_unconfirmed_subscriber(&app).await;
    app.test_user.login(&app).await;
    wiremock::Mock::given(wiremock::matchers::any())
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/newsletters",
    );
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains(
        "<p><i>The newsletter issue has been accepted - \
        emails will go out shortly.</i></p>"
    ));
    app.dispatch_all_pending_emails().await;
}

#[tokio::test]
async fn integration_newsletters_are_delivered_to_confirmed_subscribers() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;
    wiremock::Mock::given(wiremock::matchers::path("/email"))
        .and(wiremock::matchers::method("POST"))
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/newsletters",
    );
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains(
        "<p><i>The newsletter issue has been accepted - \
        emails will go out shortly.</i></p>"
    ));
    app.dispatch_all_pending_emails().await;
}

#[tokio::test]
async fn integration_you_must_be_logged_in_to_see_the_newsletter_form() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let response = app.get_publish_newsletter().await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn integration_you_must_be_logged_in_to_publish_a_newsletter() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn integration_newsletter_creation_is_idempotent() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;
    wiremock::Mock::given(wiremock::matchers::path("/email"))
        .and(wiremock::matchers::method("POST"))
        .respond_with(wiremock::ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/newsletters",
    );
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains(
        "<p><i>The newsletter issue has been accepted - \
        emails will go out shortly.</i></p>"
    ));
    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/newsletters",
    );
    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains(
        "<p><i>The newsletter issue has been accepted - \
        emails will go out shortly.</i></p>"
    ));
    app.dispatch_all_pending_emails().await;
}

#[tokio::test]
async fn integration_concurrent_form_submission_is_handled_gracefully() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;
    wiremock::Mock::given(wiremock::matchers::path("/email"))
        .and(wiremock::matchers::method("POST"))
        .respond_with(
            wiremock::ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(2)),
        )
        .expect(1)
        .mount(&app.email_server)
        .await;
    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response1 = app.post_publish_newsletter(&newsletter_request_body);
    let response2 = app.post_publish_newsletter(&newsletter_request_body);
    let (response1, response2) = tokio::join!(response1, response2);
    assert_eq!(response1.status(), response2.status());
    assert_eq!(
        response1.text().await.expect("inside integration_concurrent_form_submission_is_handled_gracefully response1.text().await failed"),
        response2.text().await.expect("inside integration_concurrent_form_submission_is_handled_gracefully response2.text().await failed")
    );
    app.dispatch_all_pending_emails().await;
}
