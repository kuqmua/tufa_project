use crate::constants::PROJECT_NAME;
use lazy_static::lazy_static;
use tufa_common::helpers::git::git_info::GitInformation;

lazy_static! {
    pub static ref GIT_INFO: GitInformation =
        GitInformation::get_git_commit_info("./", PROJECT_NAME);
}
