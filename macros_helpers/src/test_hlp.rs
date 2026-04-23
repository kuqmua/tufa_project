use crate::rs_file_path::rs_file_path as mk_rs_file_path;
use std::{
    env::temp_dir,
    fs::{read_to_string, remove_file},
    path::{Path, PathBuf},
    process,
    sync::atomic::{AtomicUsize, Ordering},
};
static TEST_SEQ: AtomicUsize = AtomicUsize::new(0);
pub(crate) fn test_path(stem: &str) -> PathBuf {
    let seq = TEST_SEQ.fetch_add(1, Ordering::Relaxed);
    temp_dir().join(format!("{stem}_{}_{}", process::id(), seq))
}
pub(crate) fn cleanup_test_file(path: impl AsRef<Path>) {
    drop(remove_file(path.as_ref()));
}
pub(crate) fn assert_file_content(path: &Path, exp: &str) {
    let cnt = read_to_string(path).expect("d5ec6712");
    assert_eq!(cnt, exp);
}
pub(crate) fn rs_file_path<P>(file_name: P) -> PathBuf
where
    P: AsRef<Path>,
{
    mk_rs_file_path(file_name)
}
