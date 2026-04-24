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
    fn get_git_commit_link(&self) -> String {
        self.get_git_commit_link_cow().into_owned()
    }
    fn get_git_commit_link_cow(&self) -> Cow<'static, str>;
}
pub trait GetGitCommitId {
    fn get_git_commit_id(&self) -> String;
    fn get_git_commit_id_cow(&self) -> Cow<'_, str> {
        with_git_commit_id_ref_or(self, Cow::Borrowed, |src| {
            Cow::Owned(src.get_git_commit_id())
        })
    }
    fn get_git_commit_id_or_else<'commit_id_lt>(
        &'commit_id_lt self,
        fallback: &'commit_id_lt mut Option<String>,
    ) -> &'commit_id_lt str {
        with_git_commit_id_ref_or(
            self,
            |commit_id| commit_id,
            |src| {
                fallback
                    .get_or_insert_with(|| src.get_git_commit_id())
                    .as_str()
            },
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
    fn get_git_commit_link_cow(&self) -> Cow<'static, str> {
        self.with_git_commit_id(git_commit_link_cow)
    }
}
fn with_git_commit_id_ref_or<'src, T, R>(
    src: &'src T,
    on_ref: impl FnOnce(&'src str) -> R,
    on_owned: impl FnOnce(&'src T) -> R,
) -> R
where
    T: ?Sized + GetGitCommitId,
{
    src.get_git_commit_id_ref()
        .map_or_else(|| on_owned(src), on_ref)
}
const fn project_git_commit_id() -> &'static str {
    PROJECT_GIT_INFO.commit
}
#[must_use]
pub fn is_project_commit(commit_id: &str) -> bool {
    commit_id == project_git_commit_id()
}
pub fn validate_project_commit(commit_id: &str) -> Result<(), &'static str> {
    if is_project_commit(commit_id) {
        return Ok(());
    }
    Err(project_git_commit_link_ref())
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
    git_commit_link_cow(commit_id).into_owned()
}
#[must_use]
pub fn git_commit_link_cow(commit_id: &str) -> Cow<'static, str> {
    if is_project_commit(commit_id) {
        return Cow::Borrowed(project_git_commit_link_ref());
    }
    Cow::Owned(build_git_commit_link(commit_id))
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
        git_commit_link, git_commit_link_capacity, git_commit_link_cow, is_project_commit,
        project_git_commit_id, project_git_commit_link, project_git_commit_link_ref,
        validate_project_commit, with_git_commit_id_ref_or,
    };
    use std::{borrow::Cow, cell::Cell, ptr};
    #[derive(Debug)]
    struct TestGitCommit {
        borrow_commit_ref: bool,
        commit: &'static str,
        fallback_calls: Cell<usize>,
    }
    impl GetGitCommitId for TestGitCommit {
        fn get_git_commit_id(&self) -> String {
            let calls = self.fallback_calls.get().saturating_add(1);
            self.fallback_calls.set(calls);
            self.commit.to_owned()
        }
        fn get_git_commit_id_ref(&self) -> Option<&str> {
            self.borrow_commit_ref.then_some(self.commit)
        }
    }
    fn mk_test_git_commit(commit: &'static str, borrow_commit_ref: bool) -> TestGitCommit {
        TestGitCommit {
            commit,
            borrow_commit_ref,
            fallback_calls: Cell::new(0),
        }
    }
    fn mk_owned_test_git_commit(commit: &'static str) -> TestGitCommit {
        mk_test_git_commit(commit, false)
    }
    fn mk_borrowed_test_git_commit(commit: &'static str) -> TestGitCommit {
        mk_test_git_commit(commit, true)
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps fallback-call expectations consistent across owned/borrowed commit-id tests
    fn assert_fallback_calls(v: &TestGitCommit, exp: usize) {
        assert_eq!(v.fallback_calls.get(), exp);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps commit-link equality checks concise across tests
    fn assert_expected_git_commit_link(actual: &str, exp_commit_id: &str) {
        assert_eq!(actual, expected_git_commit_link(exp_commit_id));
    }
    #[allow(clippy::single_call_fn)] // shared helper keeps borrowed/owned Cow-kind assertions consistent across commit-id tests
    fn assert_commit_id_cow_kind(
        commit_id: &str,
        is_borrowed: bool,
        exp_commit_id: &str,
        exp_is_borrowed: bool,
    ) {
        assert_eq!(commit_id, exp_commit_id);
        assert_eq!(is_borrowed, exp_is_borrowed);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps link-output and fallback-call expectations coupled for test clarity
    fn assert_commit_link_and_fallback_calls(
        v: &TestGitCommit,
        exp_commit_id: &str,
        exp_fallback_calls: usize,
    ) {
        let link = v.get_git_commit_link();
        assert_expected_git_commit_link(&link, exp_commit_id);
        assert_fallback_calls(v, exp_fallback_calls);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps borrowed/owned Cow expectations concise across commit-id tests
    fn assert_commit_id_cow_and_fallback_calls(
        v: &TestGitCommit,
        exp_commit_id: &str,
        exp_is_borrowed: bool,
        exp_fallback_calls: usize,
    ) {
        let commit_id = v.get_git_commit_id_cow();
        assert_commit_id_cow_kind(
            commit_id.as_ref(),
            matches!(&commit_id, Cow::Borrowed(_)),
            exp_commit_id,
            exp_is_borrowed,
        );
        assert_fallback_calls(v, exp_fallback_calls);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps commit-length and fallback-call checks coupled across borrowed/owned cases
    fn assert_commit_len_and_fallback_calls(
        v: &TestGitCommit,
        exp_commit_len: usize,
        exp_fallback_calls: usize,
    ) {
        let commit_len = v.with_git_commit_id(str::len);
        assert_eq!(commit_len, exp_commit_len);
        assert_fallback_calls(v, exp_fallback_calls);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps with_git_commit_id_ref_or branch behavior checks reusable across borrowed/owned test cases
    fn assert_with_git_commit_id_ref_or(
        v: &TestGitCommit,
        exp_commit_len: usize,
        exp_fallback_calls: usize,
    ) {
        let commit_len =
            with_git_commit_id_ref_or(v, str::len, |src| src.get_git_commit_id().len());
        assert_eq!(commit_len, exp_commit_len);
        assert_fallback_calls(v, exp_fallback_calls);
    }
    fn expected_git_commit_link(commit_id: &str) -> String {
        let mut output = String::with_capacity(git_commit_link_capacity(commit_id));
        super::write_git_commit_link(&mut output, commit_id);
        output
    }
    #[test]
    fn git_commit_link_builds_expected_url() {
        let link = git_commit_link("abc123");
        assert_expected_git_commit_link(&link, "abc123");
    }
    #[test]
    fn git_commit_link_handles_empty_commit() {
        let link = git_commit_link("");
        assert_expected_git_commit_link(&link, "");
    }
    #[test]
    fn git_commit_link_cow_borrows_cached_project_link_for_project_commit() {
        let project_commit = project_git_commit_id();
        let actual = git_commit_link_cow(project_commit);
        assert!(matches!(actual, Cow::Borrowed(v) if ptr::eq(v, project_git_commit_link_ref())));
    }
    #[test]
    fn git_commit_link_uses_cached_project_link_for_project_commit() {
        let project_commit = project_git_commit_id();
        let actual = git_commit_link(project_commit);
        assert_eq!(actual, project_git_commit_link_ref());
    }
    #[test]
    fn git_commit_link_cow_owns_link_for_non_project_commit() {
        let actual = git_commit_link_cow("deadbeef");
        assert!(matches!(actual, Cow::Owned(v) if v == expected_git_commit_link("deadbeef")));
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
        let link = git_info.get_git_commit_link();
        assert_expected_git_commit_link(&link, "deadbeef");
    }
    #[test]
    fn get_git_commit_link_uses_trait_based_commit_id() {
        let test_git_commit = mk_owned_test_git_commit("f00dbabe");
        assert_commit_link_and_fallback_calls(&test_git_commit, "f00dbabe", 1);
    }
    #[test]
    fn get_git_commit_link_calls_allocating_fallback_once_without_ref() {
        let test_git_commit = mk_owned_test_git_commit("f00dbabe");
        drop(test_git_commit.get_git_commit_link());
        assert_fallback_calls(&test_git_commit, 1);
    }
    #[test]
    fn get_git_commit_id_or_else_computes_fallback_once() {
        let test_git_commit = mk_owned_test_git_commit("f00dbabe");
        let mut fallback = None;
        let first = test_git_commit.get_git_commit_id_or_else(&mut fallback);
        assert_eq!(first, "f00dbabe");
        let second = test_git_commit.get_git_commit_id_or_else(&mut fallback);
        assert_eq!(second, "f00dbabe");
        assert_fallback_calls(&test_git_commit, 1);
    }
    #[test]
    fn get_git_commit_id_or_else_prefers_borrowed_ref_without_fallback() {
        let test_git_commit = mk_borrowed_test_git_commit("cafebabe");
        let mut fallback = None;
        let commit = test_git_commit.get_git_commit_id_or_else(&mut fallback);
        assert_eq!(commit, "cafebabe");
        assert_fallback_calls(&test_git_commit, 0);
        assert!(fallback.is_none());
    }
    #[test]
    fn get_git_commit_id_cow_returns_owned_without_ref() {
        let test_git_commit = mk_owned_test_git_commit("cafebabe");
        assert_commit_id_cow_and_fallback_calls(&test_git_commit, "cafebabe", false, 1);
    }
    #[test]
    fn get_git_commit_link_prefers_borrowed_commit_id() {
        let test_git_commit = mk_borrowed_test_git_commit("cafebabe");
        assert_commit_link_and_fallback_calls(&test_git_commit, "cafebabe", 0);
    }
    #[test]
    fn get_git_commit_link_cow_borrows_project_link_for_project_commit() {
        let git_info = ProjectGitInfo {
            commit: project_git_commit_id(),
        };
        let link = git_info.get_git_commit_link_cow();
        assert!(matches!(link, Cow::Borrowed(v) if ptr::eq(v, project_git_commit_link_ref())));
    }
    #[test]
    fn get_git_commit_id_cow_returns_borrowed_when_ref_is_available() {
        let test_git_commit = mk_borrowed_test_git_commit("cafebabe");
        assert_commit_id_cow_and_fallback_calls(&test_git_commit, "cafebabe", true, 0);
    }
    #[test]
    fn with_git_commit_id_uses_allocating_fallback_once_without_ref() {
        let test_git_commit = mk_owned_test_git_commit("cafebabe");
        assert_commit_len_and_fallback_calls(&test_git_commit, "cafebabe".len(), 1);
    }
    #[test]
    fn with_git_commit_id_prefers_borrowed_ref_when_available() {
        let test_git_commit = mk_borrowed_test_git_commit("cafebabe");
        assert_commit_len_and_fallback_calls(&test_git_commit, "cafebabe".len(), 0);
    }
    #[test]
    fn with_git_commit_id_ref_or_prefers_borrowed_ref_when_available() {
        let test_git_commit = mk_borrowed_test_git_commit("cafebabe");
        assert_with_git_commit_id_ref_or(&test_git_commit, "cafebabe".len(), 0);
    }
    #[test]
    fn with_git_commit_id_ref_or_uses_fallback_without_ref() {
        let test_git_commit = mk_owned_test_git_commit("cafebabe");
        assert_with_git_commit_id_ref_or(&test_git_commit, "cafebabe".len(), 1);
    }
    #[test]
    fn base_git_commit_link_len_matches_expected_prefix_len() {
        let commit_id = "abc123";
        let expected = format!("{GITHUB_URL}/tree/{commit_id}").len();
        assert_eq!(git_commit_link_capacity(commit_id), expected);
    }
    #[test]
    fn get_git_commit_link_works_for_str_and_string() {
        let str_link = "abc123".get_git_commit_link();
        assert_expected_git_commit_link(&str_link, "abc123");
        let string_link = String::from("abc123").get_git_commit_link();
        assert_expected_git_commit_link(&string_link, "abc123");
    }
    #[test]
    fn get_git_commit_link_works_for_cow_str() {
        let borrowed_link = Cow::Borrowed("abc123").get_git_commit_link();
        assert_expected_git_commit_link(&borrowed_link, "abc123");
        assert_expected_git_commit_link(
            &Cow::<'_, str>::Owned("abc123".to_owned()).get_git_commit_link(),
            "abc123",
        );
    }
    #[test]
    fn project_git_info_as_ref_returns_commit() {
        let info = ProjectGitInfo { commit: "abc123" };
        assert_eq!(info.as_ref(), "abc123");
    }
    #[test]
    fn git_commit_link_capacity_handles_empty_commit() {
        assert_eq!(
            git_commit_link_capacity(""),
            GITHUB_URL.len() + TREE_SEGMENT.len()
        );
    }
}
