// pub async fn change_password<'a>(
//     form: actix_web::web::Form<
//         common::common::change_password_form_data::ChangePasswordFormData,
//     >,
//     app_state: actix_web::web::Data<
//         common::repositories_types::server::routes::app_state::AppInfo<'a>,
//     >,
//     user_id: actix_web::web::ReqData<
//         common::repositories_types::server::authentication::UserId,
//     >,
// ) -> Result<actix_web::HttpResponse, actix_web::Error> {
//     let user_id = user_id.into_inner();
//     if secrecy::ExposeSecret::expose_secret(&form.new_password) != secrecy::ExposeSecret::expose_secret(form.new_password_check) {
//         actix_web_flash_messages::FlashMessage::error(
//             "You entered two different new passwords - the field values must match.",
//         )
//         .send();
//         return Ok(
//             common::repositories_types::server::utils::status_codes::see_other(
//                 "/admin/password",
//             ),
//         );
//     }
//     let username = crate::routes::dashboard::get_username(*user_id, &app_state.postgres_pool)
//         .await
//         .map_err(common::repositories_types::server::utils::status_codes::e500)?;
//     let credentials = common::common::postgres_credentials::PostgresCredentials {
//         username,
//         password: form.0.current_password,
//     };
//     if let Err(error) =
//         crate::authentication::validate_credentials(credentials, &app_state.postgres_pool).await
//     {
//         //todo - add to body deserialized version?
//         return match &e {
//             common::repositories_types::server::authentication::password::ValidateCredentialsErrorNamed::GetStoredCredentials {
//                 get_stored_credentials,
//                 ..
//             } => match get_stored_credentials {
//                 common::repositories_types::server::authentication::password::GetStoredCredentialsErrorNamed::FetchOptional {..} => Err(common::repositories_types::server::utils::status_codes::e500(e)),
//             },
//             common::repositories_types::server::authentication::password::ValidateCredentialsErrorNamed::SpawnBlockingWithTracing {..} => Err(common::repositories_types::server::utils::status_codes::e500(e)),
//             common::repositories_types::server::authentication::password::ValidateCredentialsErrorNamed::VerifyPasswordHash {
//                 spawn_blocking_with_tracing,
//                 ..
//             } => match spawn_blocking_with_tracing {
//                 common::repositories_types::server::authentication::password::VerifyPasswordHashErrorNamed::ExposeSecret {..} => Err(common::repositories_types::server::utils::status_codes::e500(e)),
//                 common::repositories_types::server::authentication::password::VerifyPasswordHashErrorNamed::InvalidPassword {..} => {
//                     actix_web_flash_messages::FlashMessage::error("The current password is incorrect.").send();
//                     Ok(common::repositories_types::server::utils::status_codes::see_other("/admin/password"))
//                 },
//             },
//             common::repositories_types::server::authentication::password::ValidateCredentialsErrorNamed::UnknownUsername {..} => {
//                 actix_web_flash_messages::FlashMessage::error("The current password is incorrect.").send();
//                 Ok(common::repositories_types::server::utils::status_codes::see_other("/admin/password"))
//             }
//         };
//     }
//     crate::authentication::change_password(*user_id, form.0.new_password, &app_state.postgres_pool)
//         .await
//         .map_err(common::repositories_types::server::utils::status_codes::e500)?;
//     actix_web_flash_messages::FlashMessage::error("Your password has been changed.").send();
//     Ok(
//         common::repositories_types::server::utils::status_codes::see_other(
//             "/admin/password",
//         ),
//     )
// }
