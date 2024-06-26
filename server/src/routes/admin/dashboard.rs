// pub async fn admin_dashboard<'a>(
//     session: common::repositories_types::server::session_state::TypedSession,
//     app_state: actix_web::web::Data<
//         common::repositories_types::server::routes::app_state::AppInfo<'a>,
//     >,
// ) -> Result<actix_web::HttpResponse, actix_web::Error> {
//     let username = if let Some(user_id) = session
//         .get_user_id()
//         .map_err(common::repositories_types::server::utils::status_codes::e500)?
//     {
//         get_username(user_id, &app_state.postgres_pool)
//             .await
//             .map_err(common::repositories_types::server::utils::status_codes::e500)?
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
//     common::repositories_types::server::routes::admin::dashboard::GetUsernameErrorNamed<
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
//         Err(error) => Err(
//             common::repositories_types::server::routes::admin::dashboard::GetUsernameErrorNamed::PostgresQuery {
//                 get_username: error,
//                 code_occurence: error_occurence_lib::code_occurence!()
//             }
//         ),
//     }
// }
