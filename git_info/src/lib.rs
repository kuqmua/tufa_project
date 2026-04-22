pub use naming::GITHUB_URL;
use optml::Optml;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
const TREE_SEGMENT: &str = "/tree/";
const BASE_GIT_COMMIT_LINK_LEN: usize = GITHUB_URL.len() + TREE_SEGMENT.len();
pub const PROJECT_GIT_INFO: ProjectGitInfo<'_> = ProjectGitInfo {
    commit: git_version::git_version!(args = ["--always", "--abbrev=40"]),
};
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Default, Optml)]
pub struct ProjectGitInfo<'commit_lt> {
    pub commit: &'commit_lt str,
}
impl AsRef<str> for ProjectGitInfo<'_> {
    fn as_ref(&self) -> &str {
        self.commit
    }
}
pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> String;
}
pub trait GetGitCommitId {
    fn get_git_commit_id(&self) -> String;
    fn get_git_commit_id_cow(&self) -> Cow<'_, str> {
        self.get_git_commit_id_ref()
            .map_or_else(|| Cow::Owned(self.get_git_commit_id()), Cow::Borrowed)
    }
    fn get_git_commit_id_or_else<'commit_id_lt>(
        &'commit_id_lt self,
        fallback: &'commit_id_lt mut Option<String>,
    ) -> &'commit_id_lt str {
        if let Some(commit_id) = self.get_git_commit_id_ref() {
            return commit_id;
        }
        fallback
            .get_or_insert_with(|| self.get_git_commit_id())
            .as_str()
    }
    fn get_git_commit_id_ref(&self) -> Option<&str> {
        None
    }
}
impl<T: ?Sized + AsRef<str>> GetGitCommitId for T {
    fn get_git_commit_id(&self) -> String {
        self.as_ref().to_owned()
    }
    fn get_git_commit_id_ref(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}
impl<T: ?Sized + GetGitCommitId> GetGitCommitLink for T {
    fn get_git_commit_link(&self) -> String {
        let commit_id = self.get_git_commit_id_cow();
        build_git_commit_link(commit_id.as_ref())
    }
}
const fn project_git_commit_id() -> &'static str {
    PROJECT_GIT_INFO.commit
}
#[must_use]
pub fn is_project_commit(commit_id: &str) -> bool {
    commit_id == project_git_commit_id()
}
pub fn validate_project_commit(commit_id: &str) -> Result<(), String> {
    is_project_commit(commit_id)
        .then_some(())
        .ok_or_else(project_git_commit_link)
}
#[must_use]
pub fn project_git_commit_link() -> String {
    build_git_commit_link(project_git_commit_id())
}
#[must_use]
pub fn git_commit_link(commit_id: &str) -> String {
    build_git_commit_link(commit_id)
}
fn build_git_commit_link(commit_id: &str) -> String {
    let cap = git_commit_link_capacity(commit_id);
    let mut output = String::with_capacity(cap);
    output.push_str(GITHUB_URL);
    output.push_str(TREE_SEGMENT);
    output.push_str(commit_id);
    output
}
#[must_use]
pub const fn git_commit_link_capacity(commit_id: &str) -> usize {
    BASE_GIT_COMMIT_LINK_LEN.saturating_add(commit_id.len())
}
#[cfg(test)]
mod tests {
    use super::{
        GITHUB_URL, GetGitCommitId, GetGitCommitLink as _, ProjectGitInfo, TREE_SEGMENT,
        git_commit_link, git_commit_link_capacity, is_project_commit, project_git_commit_id,
        project_git_commit_link, validate_project_commit,
    };
    use std::{borrow::Cow, cell::Cell};
    #[derive(Debug)]
    struct TestGitCommit {
        commit: &'static str,
        fallback_calls: Cell<usize>,
    }
    impl GetGitCommitId for TestGitCommit {
        fn get_git_commit_id(&self) -> String {
            let calls = self.fallback_calls.get().saturating_add(1);
            self.fallback_calls.set(calls);
            self.commit.to_owned()
        }
    }
    #[derive(Debug)]
    struct BorrowedTestGitCommit {
        commit: &'static str,
        fallback_calls: Cell<usize>,
    }
    impl GetGitCommitId for BorrowedTestGitCommit {
        fn get_git_commit_id(&self) -> String {
            let calls = self.fallback_calls.get().saturating_add(1);
            self.fallback_calls.set(calls);
            self.commit.to_owned()
        }
        fn get_git_commit_id_ref(&self) -> Option<&str> {
            Some(self.commit)
        }
    }
    fn expected_git_commit_link(commit_id: &str) -> String {
        format!("{GITHUB_URL}{TREE_SEGMENT}{commit_id}")
    }
    #[test]
    fn git_commit_link_builds_expected_url() {
        assert_eq!(
            git_commit_link("abc123"),
            expected_git_commit_link("abc123")
        );
    }
    #[test]
    fn git_commit_link_handles_empty_commit() {
        assert_eq!(git_commit_link(""), expected_git_commit_link(""));
    }
    #[test]
    fn is_project_commit_returns_true_for_project_commit() {
        assert!(is_project_commit(project_git_commit_id()));
    }
    #[test]
    fn is_project_commit_returns_false_for_other_commit() {
        assert!(!is_project_commit("deadbeef"));
    }
    #[test]
    fn validate_project_commit_returns_ok_for_project_commit() {
        assert_eq!(validate_project_commit(project_git_commit_id()), Ok(()));
    }
    #[test]
    fn validate_project_commit_returns_project_link_for_non_project_commit() {
        assert_eq!(
            validate_project_commit("deadbeef"),
            Err(project_git_commit_link())
        );
    }
    #[test]
    fn project_git_commit_link_matches_project_commit() {
        assert_eq!(
            project_git_commit_link(),
            expected_git_commit_link(project_git_commit_id())
        );
    }
    #[test]
    fn project_git_info_returns_commit_link() {
        let git_info = ProjectGitInfo { commit: "deadbeef" };
        assert_eq!(
            git_info.get_git_commit_link(),
            expected_git_commit_link("deadbeef")
        );
    }
    #[test]
    fn get_git_commit_link_uses_trait_based_commit_id() {
        let test_git_commit = TestGitCommit {
            commit: "f00dbabe",
            fallback_calls: Cell::new(0),
        };
        assert_eq!(
            test_git_commit.get_git_commit_link(),
            expected_git_commit_link("f00dbabe")
        );
        assert_eq!(test_git_commit.fallback_calls.get(), 1);
    }
    #[test]
    fn get_git_commit_link_calls_allocating_fallback_once_without_ref() {
        let test_git_commit = TestGitCommit {
            commit: "f00dbabe",
            fallback_calls: Cell::new(0),
        };
        drop(test_git_commit.get_git_commit_link());
        assert_eq!(test_git_commit.fallback_calls.get(), 1);
    }
    #[test]
    fn get_git_commit_id_or_else_computes_fallback_once() {
        let test_git_commit = TestGitCommit {
            commit: "f00dbabe",
            fallback_calls: Cell::new(0),
        };
        let mut fallback = None;
        let first = test_git_commit.get_git_commit_id_or_else(&mut fallback);
        assert_eq!(first, "f00dbabe");
        let second = test_git_commit.get_git_commit_id_or_else(&mut fallback);
        assert_eq!(second, "f00dbabe");
        assert_eq!(test_git_commit.fallback_calls.get(), 1);
    }
    #[test]
    fn get_git_commit_id_or_else_prefers_borrowed_ref_without_fallback() {
        let test_git_commit = BorrowedTestGitCommit {
            commit: "cafebabe",
            fallback_calls: Cell::new(0),
        };
        let mut fallback = None;
        let commit = test_git_commit.get_git_commit_id_or_else(&mut fallback);
        assert_eq!(commit, "cafebabe");
        assert_eq!(test_git_commit.fallback_calls.get(), 0);
        assert!(fallback.is_none());
    }
    #[test]
    fn get_git_commit_link_prefers_borrowed_commit_id() {
        let test_git_commit = BorrowedTestGitCommit {
            commit: "cafebabe",
            fallback_calls: Cell::new(0),
        };
        let link = test_git_commit.get_git_commit_link();
        assert_eq!(link, expected_git_commit_link("cafebabe"));
        assert_eq!(test_git_commit.fallback_calls.get(), 0);
    }
    #[test]
    fn get_git_commit_id_cow_returns_borrowed_when_ref_is_available() {
        let test_git_commit = BorrowedTestGitCommit {
            commit: "cafebabe",
            fallback_calls: Cell::new(0),
        };
        let commit_id = test_git_commit.get_git_commit_id_cow();
        assert!(matches!(commit_id, Cow::Borrowed("cafebabe")));
        assert_eq!(test_git_commit.fallback_calls.get(), 0);
    }
    #[test]
    fn base_git_commit_link_len_matches_expected_prefix_len() {
        let commit_id = "abc123";
        let expected = format!("{GITHUB_URL}/tree/{commit_id}").len();
        assert_eq!(git_commit_link_capacity(commit_id), expected);
    }
    #[test]
    fn git_commit_link_capacity_handles_empty_commit() {
        let expected = format!("{GITHUB_URL}{TREE_SEGMENT}").len();
        assert_eq!(git_commit_link_capacity(""), expected);
    }
    #[test]
    fn get_git_commit_link_works_for_str_and_string() {
        let commit_str = "1337beef";
        assert_eq!(
            commit_str.get_git_commit_link(),
            expected_git_commit_link(commit_str)
        );
        let commit_string = String::from("c0ffee00");
        assert_eq!(
            commit_string.get_git_commit_link(),
            expected_git_commit_link("c0ffee00")
        );
        let commit_string_ref = &commit_string;
        assert_eq!(
            commit_string_ref.get_git_commit_link(),
            expected_git_commit_link("c0ffee00")
        );
    }
    #[test]
    fn get_git_commit_link_works_for_cow_str() {
        let borrowed = Cow::Borrowed("abcddcba");
        assert_eq!(
            borrowed.get_git_commit_link(),
            expected_git_commit_link("abcddcba")
        );
        let owned = Cow::<'_, str>::Owned(String::from("12344321"));
        assert_eq!(
            owned.get_git_commit_link(),
            expected_git_commit_link("12344321")
        );
    }
    #[test]
    fn project_git_info_as_ref_returns_commit() {
        let git_info = ProjectGitInfo { commit: "ab12cd34" };
        assert_eq!(git_info.as_ref(), "ab12cd34");
    }
}
