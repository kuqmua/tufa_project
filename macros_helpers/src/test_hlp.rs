use std::{
    env::temp_dir,
    fs::{read_to_string, remove_file},
    io::ErrorKind,
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
    if let Err(er) = remove_file(path.as_ref())
        && er.kind() != ErrorKind::NotFound
    {
        panic!("33ea4ea2: {er}");
    }
}
pub(crate) fn assert_file_content(path: &Path, exp: &str) {
    let cnt = read_to_string(path).expect("d5ec6712");
    assert_eq!(cnt, exp);
}
