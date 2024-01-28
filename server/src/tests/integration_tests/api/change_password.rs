#[tokio::test]
async fn integration_you_must_be_logged_in_to_see_the_change_password_form() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let response = app.get_change_password().await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn integration_you_must_be_logged_in_to_change_your_password() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let new_password = uuid::Uuid::new_v4().to_string();
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": uuid::Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn integration_new_password_fields_must_match() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let new_password = uuid::Uuid::new_v4().to_string();
    let another_new_password = uuid::Uuid::new_v4().to_string();
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &another_new_password,
        }))
        .await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/password",
    );
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - the field values must match.</i></p>"
    ));
}

#[tokio::test]
async fn integration_current_password_must_be_valid() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let new_password = uuid::Uuid::new_v4().to_string();
    let wrong_password = uuid::Uuid::new_v4().to_string();
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/password",
    );
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>"));
}

#[tokio::test]
async fn integration_changing_password_works() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let new_password = uuid::Uuid::new_v4().to_string();
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/dashboard",
    );
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/password",
    );
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));
    let response = app.post_logout().await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(&response, "/login");
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &new_password
    });
    let response = app.post_login(&login_body).await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/dashboard",
    );
}
