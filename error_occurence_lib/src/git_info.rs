#[derive(
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    Clone,
    Eq,
    Hash,
    PartialEq,
    Default,
    utoipa::ToSchema,
)]
pub struct GitInfoWithoutLifetime {
    pub git_commit_id: std::string::String,
    pub git_repo_link: std::string::String,
}

impl crate::git_fields::GetGitCommitId for GitInfoWithoutLifetime {
    fn get_git_commit_id(&self) -> std::string::String {
        self.git_commit_id.to_string()
    }
}