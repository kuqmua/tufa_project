use std::{
    env::temp_dir,
    fs::remove_file,
    path::PathBuf,
    process,
    sync::atomic::{AtomicUsize, Ordering},
};
static TEST_SEQ: AtomicUsize = AtomicUsize::new(0);
pub(crate) fn test_path(stem: &str) -> PathBuf {
    let seq = TEST_SEQ.fetch_add(1, Ordering::Relaxed);
    temp_dir().join(format!("{stem}_{}_{}", process::id(), seq))
}
pub(crate) fn cleanup_test_file(path: PathBuf) {
    drop(remove_file(path));
}
