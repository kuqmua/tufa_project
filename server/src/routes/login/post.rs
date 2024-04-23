// pub async fn login<'a>(
//     form: actix_web::web::Form<
//         common::repositories_types::server::routes::login::post::FormData,
//     >,
//     app_state: actix_web::web::Data<
//         common::repositories_types::server::routes::app_state::AppInfo<'a>,
//     >,
//     session: common::repositories_types::server::session_state::TypedSession,
// ) -> Result<
//     actix_web::HttpResponse,
//     actix_web::error::InternalError<
//         common::repositories_types::server::routes::login::post::LoginErrorNamed,
//     >,
// > {
//     let credentials = common::common::postgres_credentials::PostgresCredentials {
//         username: form.0.username,
//         password: form.0.password,
//     };
//     tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
//     match crate::authentication::validate_credentials(credentials, &app_state.postgres_pool).await {
//         Ok(user_id) => {
//             tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
//             session.renew();
//             session
//                 .insert_user_id(user_id)
//                 .map_err(|e| login_redirect(common::repositories_types::server::routes::login::post::LoginErrorNamed::SessionInsert{
//                     session_insert: error,
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 }))?;
//             Ok(actix_web::HttpResponse::SeeOther()
//                 .insert_header((actix_web::http::header::LOCATION, "/admin/dashboard"))
//                 .finish())
//         }
//         Err(error) => {
//             Err(login_redirect(common::repositories_types::server::routes::login::post::LoginErrorNamed::AuthError{
//                 validate_credentials: error,
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             }))
//         }
//     }
// }

// fn login_redirect(
//     e: common::repositories_types::server::routes::login::post::LoginErrorNamed,
// ) -> actix_web::error::InternalError<
//     common::repositories_types::server::routes::login::post::LoginErrorNamed,
// > {
//     actix_web_flash_messages::FlashMessage::error(e.to_string()).send();
//     let response = actix_web::HttpResponse::SeeOther()
//         .insert_header((actix_web::http::header::LOCATION, "/login"))
//         .finish();
//     actix_web::error::InternalError::from_response(e, response)
// }
