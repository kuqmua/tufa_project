pub use naming::GITHUB_URL;
use optml::Optml;
use serde_derive::{Deserialize, Serialize};
use std::{borrow::Cow, sync::OnceLock};
const TREE_SEGMENT: &str = "/tree/";
const BASE_GIT_COMMIT_LINK_LEN: usize = GITHUB_URL.len() + TREE_SEGMENT.len();
static PROJECT_GIT_COMMIT_LINK: OnceLock<String> = OnceLock::new();
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
        if let Some(commit_id) = self.get_git_commit_id_ref() {
            return Cow::Borrowed(commit_id);
        }
        Cow::Owned(self.get_git_commit_id())
    }
    fn get_git_commit_id_or_else<'commit_id_lt>(
        &'commit_id_lt self,
        fallback: &'commit_id_lt mut Option<String>,
    ) -> &'commit_id_lt str {
        self.get_git_commit_id_ref().map_or_else(
            || {
                fallback
                    .get_or_insert_with(|| self.get_git_commit_id())
                    .as_str()
            },
            |commit_id| commit_id,
        )
    }
    fn get_git_commit_id_ref(&self) -> Option<&str> {
        None
    }
    fn with_git_commit_id<R>(&self, f: impl FnOnce(&str) -> R) -> R {
        let mut fallback = None;
        f(self.get_git_commit_id_or_else(&mut fallback))
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
        self.with_git_commit_id(build_git_commit_link)
    }
}
const fn project_git_commit_id() -> &'static str {
    PROJECT_GIT_INFO.commit
}
#[must_use]
pub fn is_project_commit(commit_id: &str) -> bool {
    commit_id == project_git_commit_id()
}
pub fn validate_project_commit(commit_id: &str) -> Result<(), &'static str> {
    is_project_commit(commit_id)
        .then_some(())
        .ok_or_else(project_git_commit_link_ref)
}
#[must_use]
pub fn project_git_commit_link() -> String {
    project_git_commit_link_ref().to_owned()
}
#[must_use]
pub fn project_git_commit_link_ref() -> &'static str {
    PROJECT_GIT_COMMIT_LINK
        .get_or_init(|| build_git_commit_link(project_git_commit_id()))
        .as_str()
}
#[must_use]
pub fn git_commit_link(commit_id: &str) -> String {
    build_git_commit_link(commit_id)
}
fn build_git_commit_link(commit_id: &str) -> String {
    let cap = git_commit_link_capacity(commit_id);
    let mut output = String::with_capacity(cap);
    write_git_commit_link(&mut output, commit_id);
    output
}
#[allow(clippy::single_call_fn)] // shared writer keeps link assembly consistent across builders and tests
fn write_git_commit_link(output: &mut String, commit_id: &str) {
    output.push_str(GITHUB_URL);
    output.push_str(TREE_SEGMENT);
    output.push_str(commit_id);
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
        project_git_commit_link, project_git_commit_link_ref, validate_project_commit,
    };
    use std::{borrow::Cow, cell::Cell, ptr};
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
    fn mk_test_git_commit(commit: &'static str) -> TestGitCommit {
        TestGitCommit {
            commit,
            fallback_calls: Cell::new(0),
        }
    }
    fn mk_borrowed_test_git_commit(commit: &'static str) -> BorrowedTestGitCommit {
        BorrowedTestGitCommit {
            commit,
            fallback_calls: Cell::new(0),
        }
    }
    fn expected_git_commit_link(commit_id: &str) -> String {
        let mut output = String::with_capacity(git_commit_link_capacity(commit_id));
        super::write_git_commit_link(&mut output, commit_id);
        output
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
            Err(project_git_commit_link_ref())
        );
    }
    #[test]
    fn validate_project_commit_reuses_cached_project_link_ref() {
        let er = validate_project_commit("deadbeef").expect_err("46bc13a9");
        let cached = project_git_commit_link_ref();
        assert!(ptr::eq(er, cached));
    }
    #[test]
    fn project_git_commit_link_matches_project_commit() {
        assert_eq!(
            project_git_commit_link(),
            expected_git_commit_link(project_git_commit_id())
        );
    }
    #[test]
    fn project_git_commit_link_ref_is_cached_and_stable() {
        let first = project_git_commit_link_ref();
        let second = project_git_commit_link_ref();
        assert_eq!(first, second);
        assert!(ptr::eq(first, second));
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
        let test_git_commit = mk_test_git_commit("f00dbabe");
        assert_eq!(
            test_git_commit.get_git_commit_link(),
            expected_git_commit_link("f00dbabe")
        );
        assert_eq!(test_git_commit.fallback_calls.get(), 1);
    }
    #[test]
    fn get_git_commit_link_calls_allocating_fallback_once_without_ref() {
        let test_git_commit = mk_test_git_commit("f00dbabe");
        drop(test_git_commit.get_git_commit_link());
        assert_eq!(test_git_commit.fallback_calls.get(), 1);
    }
    #[test]
    fn get_git_commit_id_or_else_computes_fallback_once() {
        let test_git_commit = mk_test_git_commit("f00dbabe");
        let mut fallback = None;
        let first = test_git_commit.get_git_commit_id_or_else(&mut fallback);
        assert_eq!(first, "f00dbabe");
        let second = test_git_commit.get_git_commit_id_or_else(&mut fallback);
        assert_eq!(second, "f00dbabe");
        assert_eq!(test_git_commit.fallback_calls.get(), 1);
    }
    #[test]
    fn get_git_commit_id_or_else_prefers_borrowed_ref_without_fallback() {
        let test_git_commit = mk_borrowed_test_git_commit("cafebabe");
        let mut fallback = None;
        let commit = test_git_commit.get_git_commit_id_or_else(&mut fallback);
        assert_eq!(commit, "cafebabe");
        assert_eq!(test_git_commit.fallback_calls.get(), 0);
        assert!(fallback.is_none());
    }
    #[test]
    fn get_git_commit_id_cow_returns_owned_without_ref() {
        let test_git_commit = mk_test_git_commit("cafebabe");
        let commit_id = test_git_commit.get_git_commit_id_cow();
        assert!(matches!(commit_id, Cow::Owned(v) if v == "cafebabe"));
        assert_eq!(test_git_commit.fallback_calls.get(), 1);
    }
    #[test]
    fn get_git_commit_link_prefers_borrowed_commit_id() {
        let test_git_commit = mk_borrowed_test_git_commit("cafebabe");
        let link = test_git_commit.get_git_commit_link();
        assert_eq!(link, expected_git_commit_link("cafebabe"));
        assert_eq!(test_git_commit.fallback_calls.get(), 0);
    }
    #[test]
    fn get_git_commit_id_cow_returns_borrowed_when_ref_is_available() {
        let test_git_commit = mk_borrowed_test_git_commit("cafebabe");
        let commit_id = test_git_commit.get_git_commit_id_cow();
        assert!(matches!(commit_id, Cow::Borrowed("cafebabe")));
        assert_eq!(test_git_commit.fallback_calls.get(), 0);
    }
    #[test]
    fn with_git_commit_id_uses_allocating_fallback_once_without_ref() {
        let test_git_commit = mk_test_git_commit("cafebabe");
        let commit_len = test_git_commit.with_git_commit_id(str::len);
        assert_eq!(commit_len, "cafebabe".len());
        assert_eq!(test_git_commit.fallback_calls.get(), 1);
    }
    #[test]
    fn with_git_commit_id_prefers_borrowed_ref_when_available() {
        let test_git_commit = mk_borrowed_test_git_commit("cafebabe");
        let commit_len = test_git_commit.with_git_commit_id(str::len);
        assert_eq!(commit_len, "cafebabe".len());
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
