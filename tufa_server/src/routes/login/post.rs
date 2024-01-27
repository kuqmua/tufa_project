// pub async fn login<'a>(
//     form: actix_web::web::Form<
//         tufa_common::repositories_types::tufa_server::routes::login::post::FormData,
//     >,
//     app_info: actix_web::web::Data<
//         tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo<'a>,
//     >,
//     session: tufa_common::repositories_types::tufa_server::session_state::TypedSession,
// ) -> Result<
//     actix_web::HttpResponse,
//     actix_web::error::InternalError<
//         tufa_common::repositories_types::tufa_server::routes::login::post::LoginErrorNamed,
//     >,
// > {
//     let credentials = tufa_common::common::postgres_credentials::PostgresCredentials {
//         username: form.0.username,
//         password: form.0.password,
//     };
//     tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
//     match crate::authentication::validate_credentials(credentials, &app_info.postgres_pool).await {
//         Ok(user_id) => {
//             tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
//             session.renew();
//             session
//                 .insert_user_id(user_id)
//                 .map_err(|e| login_redirect(tufa_common::repositories_types::tufa_server::routes::login::post::LoginErrorNamed::SessionInsert{
//                     session_insert: e,
//                     code_occurence: tufa_common::code_occurence!(),
//                 }))?;
//             Ok(actix_web::HttpResponse::SeeOther()
//                 .insert_header((actix_web::http::header::LOCATION, "/admin/dashboard"))
//                 .finish())
//         }
//         Err(e) => {
//             Err(login_redirect(tufa_common::repositories_types::tufa_server::routes::login::post::LoginErrorNamed::AuthError{
//                 validate_credentials: e,
//                 code_occurence: tufa_common::code_occurence!(),
//             }))
//         }
//     }
// }

// fn login_redirect(
//     e: tufa_common::repositories_types::tufa_server::routes::login::post::LoginErrorNamed,
// ) -> actix_web::error::InternalError<
//     tufa_common::repositories_types::tufa_server::routes::login::post::LoginErrorNamed,
// > {
//     actix_web_flash_messages::FlashMessage::error(e.to_string()).send();
//     let response = actix_web::HttpResponse::SeeOther()
//         .insert_header((actix_web::http::header::LOCATION, "/login"))
//         .finish();
//     actix_web::error::InternalError::from_response(e, response)
// }
