pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> std::string::String;
}

impl<T> GetGitCommitLink for T
where
    T: GetGitCommitId, //todo wtf
{
    fn get_git_commit_link(&self) -> std::string::String {
        format!(
            "{}/tree/{}",
            naming_constants::GITHUB_URL,
            self.get_git_commit_id()
        )
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

impl<'a> GetGitCommitId for git_info::ProjectGitInfo<'a> {
    //todo
    fn get_git_commit_id(&self) -> std::string::String {
        self.commit.to_string()
    }
}