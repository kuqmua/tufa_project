#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone, Hash, PartialEq, Eq, Default)]
pub struct ProjectGitInfo<'a> {
    pub commit: &'a str,
}

pub const PROJECT_GIT_INFO: ProjectGitInfo<'_> = compile_time_git_info::compile_time_project_git_info!();

// pub const PROJECT_GIT_INFO: crate::common::git::project_git_info::ProjectGitInfo =
//     crate::common::git::project_git_info::ProjectGitInfo {
//         commit: "moch",
//     };

pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> std::string::String;
}

impl<T> GetGitCommitLink for T
where
    T: GetGitCommitId, //todo wtf
{
    fn get_git_commit_link(&self) -> std::string::String {
        format!("{}/tree/{}", naming::GITHUB_URL, self.get_git_commit_id())
    }
}

pub trait GetGitCommitId {
    //todo remove
    fn get_git_commit_id(&self) -> std::string::String;
}

// pub trait GetGitRepoLink {
//     fn get_git_repo_link(&self) -> std::string::String;
// }

// pub trait GetGitAuthor<'a> {
//     fn get_git_author(&self) -> &'a str;
// }

// pub trait GetGitAuthorEmail<'a> {
//     fn get_git_author_email(&self) -> &'a str;
// }

// pub trait GetGitCommitUnixTime<'a> {
//     fn get_git_commit_unix_time(&self) -> &'a str;
// }

// pub trait GetGitTimezone<'a> {
//     fn get_git_timezone(&self) -> &'a str;
// }

// pub trait GetGitMessage<'a> {
//     fn get_git_message(&self) -> &'a str;
// }

impl GetGitCommitId for ProjectGitInfo<'_> {
    //todo
    fn get_git_commit_id(&self) -> std::string::String {
        self.commit.to_owned()
    }
}
