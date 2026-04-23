use crate::panic_if_err::panic_if_err;
use crate::rs_file_path::rs_file_path;
use std::{
    fs::{read_to_string, write},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WritePathOutcome {
    Changed(PathBuf),
    Unchanged(PathBuf),
}
impl WritePathOutcome {
    pub(crate) const fn is_changed(&self) -> bool {
        matches!(self, Self::Changed(_))
    }
    pub(crate) fn path(&self) -> &Path {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => path.as_path(),
        }
    }
}
#[allow(clippy::single_call_fn)] // write-decision logic is split out to keep file write path minimal and focused
fn should_write_string_into_file(path: &Path, string_cnt: &str) -> io::Result<bool> {
    match read_to_string(path) {
        Ok(v) => Ok(v != string_cnt),
        Err(er) if er.kind() == ErrorKind::NotFound => Ok(true),
        Err(er) => Err(er),
    }
}
#[allow(clippy::single_call_fn)] // preserves write/no-write state so callers can skip extra work (e.g. formatting) on unchanged files
pub(crate) fn try_write_string_into_path_with_outcome(
    path: PathBuf,
    string_cnt: &str,
) -> io::Result<WritePathOutcome> {
    if should_write_string_into_file(&path, string_cnt)? {
        write(&path, string_cnt)?;
        Ok(WritePathOutcome::Changed(path))
    } else {
        Ok(WritePathOutcome::Unchanged(path))
    }
}
#[allow(clippy::single_call_fn)] // shared write helper retains simple path-returning API used by file-level wrappers and tests
pub(crate) fn try_write_string_into_path(path: PathBuf, string_cnt: &str) -> io::Result<PathBuf> {
    try_write_string_into_path_with_outcome(path, string_cnt).map(|outcome| match outcome {
        WritePathOutcome::Changed(output_path) | WritePathOutcome::Unchanged(output_path) => {
            output_path
        }
    })
}
pub fn try_write_string_into_file<P>(file_name: P, string_cnt: &str) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    try_write_string_into_path(rs_file_path(file_name), string_cnt)
}
pub fn write_string_into_file<P>(file_name: P, string_cnt: &str)
where
    P: AsRef<Path>,
{
    let _pth = panic_if_err(try_write_string_into_file(file_name, string_cnt), |er| {
        format!("4f3094e1:{er}")
    });
}
#[cfg(test)]
mod tests {
    use super::{
        WritePathOutcome, should_write_string_into_file, try_write_string_into_file,
        try_write_string_into_path, try_write_string_into_path_with_outcome,
        write_string_into_file,
    };
    use crate::test_hlp::{assert_file_content, cleanup_test_file, rs_file_path, test_path};
    use std::fs::{metadata, write};
    #[test]
    fn try_write_string_into_path_writes_exact_content() {
        let path = test_path("macros_helpers_write_path").with_extension("txt");
        let result_path = try_write_string_into_path(path.clone(), "abc").expect("dcb22948");
        assert_eq!(result_path, path);
        assert_file_content(&path, "abc");
        cleanup_test_file(path);
    }
    #[test]
    fn write_string_into_file_adds_rs_extension() {
        let base = test_path("macros_helpers_write_file");
        let path = rs_file_path(&base);
        write_string_into_file(&base, "xyz");
        assert_file_content(&path, "xyz");
        cleanup_test_file(path);
    }
    #[test]
    fn try_write_string_into_file_returns_path() {
        let base = test_path("macros_helpers_try_write_file");
        let path = try_write_string_into_file(&base, "qwe").expect("6676e082");
        assert_file_content(&path, "qwe");
        cleanup_test_file(path);
    }
    #[test]
    fn try_write_string_into_path_writes_exact_path_without_extension_rewrite() {
        let path = test_path("macros_helpers_try_write_path_passthrough").with_extension("txt");
        let result_path = try_write_string_into_path(path.clone(), "abc").expect("b6b47a2c");
        assert_eq!(result_path, path);
        assert_file_content(&path, "abc");
        cleanup_test_file(path);
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_missing_file() {
        let path = test_path("macros_helpers_should_write_missing").with_extension("txt");
        let should_write = should_write_string_into_file(&path, "abc").expect("f5d2cb68");
        assert!(should_write);
    }
    #[test]
    fn should_write_string_into_file_returns_false_when_content_is_eq() {
        let path = test_path("macros_helpers_should_write_same").with_extension("txt");
        write(&path, "same").expect("68e4f52d");
        let should_write = should_write_string_into_file(&path, "same").expect("3e7adf2f");
        assert!(!should_write);
        cleanup_test_file(path);
    }
    #[test]
    fn should_write_string_into_file_returns_true_when_content_differs() {
        let path = test_path("macros_helpers_should_write_diff").with_extension("txt");
        write(&path, "old").expect("a2fd8473");
        let should_write = should_write_string_into_file(&path, "new").expect("52c9a1db");
        assert!(should_write);
        cleanup_test_file(path);
    }
    #[test]
    fn path_with_rs_extension_accepts_path_input() {
        let path = rs_file_path(test_path("macros_helpers_rs_ext_path"));
        assert_eq!(path.extension().and_then(|v| v.to_str()), Some("rs"));
    }
    #[test]
    fn try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged() {
        let base = test_path("macros_helpers_write_if_changed");
        let path = rs_file_path(&base);
        write(&path, "same").expect("0242e1a9");
        let metadata_before = metadata(&path).expect("974bc327");
        let _path = try_write_string_into_file(&base, "same").expect("07d9fd90");
        let metadata_after = metadata(&path).expect("83087942");
        assert_eq!(metadata_before.len(), metadata_after.len());
        assert_file_content(&path, "same");
        cleanup_test_file(path);
    }
    #[test]
    fn try_write_string_into_file_writes_when_cnt_differs() {
        let base = test_path("macros_helpers_write_if_changed_diff");
        let path = rs_file_path(&base);
        write(&path, "old").expect("d870b82e");
        let _path = try_write_string_into_file(&base, "new").expect("c6fd2bc8");
        assert_file_content(&path, "new");
        cleanup_test_file(path);
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_changed_for_new_content() {
        let path = test_path("macros_helpers_write_outcome_changed").with_extension("txt");
        let outcome =
            try_write_string_into_path_with_outcome(path.clone(), "abc").expect("947faed1");
        assert_eq!(outcome, WritePathOutcome::Changed(path.clone()));
        assert_eq!(outcome.path(), path.as_path());
        assert!(outcome.is_changed());
        assert_file_content(&path, "abc");
        cleanup_test_file(path);
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_unchanged_for_same_content() {
        let path = test_path("macros_helpers_write_outcome_unchanged").with_extension("txt");
        write(&path, "abc").expect("d293f783");
        let outcome =
            try_write_string_into_path_with_outcome(path.clone(), "abc").expect("b8f8eaf1");
        assert_eq!(outcome, WritePathOutcome::Unchanged(path.clone()));
        assert_eq!(outcome.path(), path.as_path());
        assert!(!outcome.is_changed());
        cleanup_test_file(path);
    }
}
