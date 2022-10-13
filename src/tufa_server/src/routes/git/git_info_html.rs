use crate::lazy_static::git_info::GIT_INFO;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use tufa_common::helpers::git::get_git_html_info::get_git_html_info;

pub async fn git_info_html() -> HttpResponse {
    {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(get_git_html_info(
                GIT_INFO.data.commit_id.clone(),
                GIT_INFO.data.repo_link.clone(),
                GIT_INFO.data.author.clone(),
                GIT_INFO.data.author_email.clone(),
                GIT_INFO.data.commit_unix_time.clone(),
                GIT_INFO.data.timezone.clone(),
                GIT_INFO.data.message.clone(),
                GIT_INFO.data.get_commit_link(),
            ))
    }
}
