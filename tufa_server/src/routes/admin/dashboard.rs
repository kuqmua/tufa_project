// pub async fn admin_dashboard<'a>(
//     session: tufa_common::repositories_types::tufa_server::session_state::TypedSession,
//     app_info: actix_web::web::Data<
//         tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo<'a>,
//     >,
// ) -> Result<actix_web::HttpResponse, actix_web::Error> {
//     let username = if let Some(user_id) = session
//         .get_user_id()
//         .map_err(tufa_common::repositories_types::tufa_server::utils::status_codes::e500)?
//     {
//         get_username(user_id, &app_info.postgres_pool)
//             .await
//             .map_err(tufa_common::repositories_types::tufa_server::utils::status_codes::e500)?
//     } else {
//         return Ok(actix_web::HttpResponse::SeeOther()
//             .insert_header((actix_web::http::header::LOCATION, "/login"))
//             .finish());
//     };
//     Ok(actix_web::HttpResponse::Ok()
//         .content_type(actix_web::http::header::ContentType::html())
//         .body(format!(
//             r#"<!DOCTYPE html>
// <html lang="en">
// <head>
//     <meta http-equiv="content-type" content="text/html; charset=utf-8">
//     <title>Admin dashboard</title>
// </head>
// <body>
//     <p>Welcome {username}!</p>
//     <p>Available actions:</p>
//     <ol>
//         <li><a href="/admin/password">Change password</a></li>
//         <li>
//             <form name="logoutForm" action="/admin/logout" method="post">
//                 <input type="submit" value="Logout">
//             </form>
//         </li>
//     </ol>
// </body>
// </html>"#,
//         )))
// }

// pub async fn get_username<'a>(
//     user_id: uuid::Uuid,
//     pool: &sqlx::PgPool,
// ) -> Result<
//     std::string::String,
//     tufa_common::repositories_types::tufa_server::routes::admin::dashboard::GetUsernameErrorNamed<
//         'a,
//     >,
// > {
//     match sqlx::query!(
//         r#"
//         SELECT username
//         FROM users
//         WHERE user_id = $1
//         "#,
//         user_id,
//     )
//     .fetch_one(pool)
//     .await {
//         Ok(row) => Ok(row.username),
//         Err(e) => Err(
//             tufa_common::repositories_types::tufa_server::routes::admin::dashboard::GetUsernameErrorNamed::PostgresQuery {
//                 get_username: e,
//                 code_occurence: tufa_common::code_occurence!()
//             }
//         ),
//     }
// }
