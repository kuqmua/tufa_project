use crate::lazy_static::git_info::GIT_INFO;
use actix_web::{web, Responder};
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitJsonInformation {
    pub commit_id: String,
    pub repo_link: String,
    pub author: String,
    pub author_email: String,
    pub commit_unix_time: String,
    pub timezone: String,
    pub message: String,
    pub commit_link: String,
}

pub async fn git_info_json() -> impl Responder {
    web::Json(GitJsonInformation {
        commit_id: GIT_INFO.data.commit_id.clone(),
        repo_link: GIT_INFO.data.repo_link.clone(),
        author: GIT_INFO.data.author.clone(),
        author_email: GIT_INFO.data.author_email.clone(),
        commit_unix_time: GIT_INFO.data.commit_unix_time.clone(),
        timezone: GIT_INFO.data.timezone.clone(),
        message: GIT_INFO.data.message.clone(),
        commit_link: GIT_INFO.data.get_commit_link(),
    })
}
