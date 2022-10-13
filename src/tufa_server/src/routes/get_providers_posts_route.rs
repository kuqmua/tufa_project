use crate::lazy_static::git_info::GIT_INFO;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;
use crate::providers::get_providers_posts::get_providers_posts;
use actix_web::HttpResponse;
use std::time::Instant;

// #[tracing::instrument(
//     name = "get_providers_posts_routee",
//     skip_all,
//     fields(user_id=%*user_id)
// )]
pub async fn get_providers_posts_route() -> Result<HttpResponse, actix_web::Error> {
    let time = Instant::now();
    if let Err(e) = get_providers_posts().await {
        return Ok(HttpResponse::InternalServerError().finish());
    };
    let message = format!(
        "get_providers_posts done in {} seconds",
        time.elapsed().as_secs()
    );
    print_colorful_message(
        None,
        PrintType::TimeMeasurement,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        message,
    );
    Ok(HttpResponse::Ok().finish())
}
