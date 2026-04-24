#[cfg(feature = "test-utils")]
use std::{
    fs::{read_to_string, remove_file, write},
    io::ErrorKind,
    path::Path,
    process::Command,
};
#[cfg(feature = "test-utils")]
const CARGO_FMT_ARGS: [&str; 1] = ["fmt"];
#[cfg(feature = "test-utils")]
const CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 6] = [
    "clippy",
    "--all-targets",
    "--all-features",
    "--",
    "-A",
    "warnings",
];
#[cfg(feature = "test-utils")]
const CARGO_CHECK_STEPS: [CargoCheckStep<'static>; 2] = [
    (&CARGO_FMT_ARGS, "8dc4f045", "2a1deb01"),
    (
        &CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS,
        "cd48b869",
        "2c037283",
    ),
];
#[cfg(feature = "test-utils")]
type CargoCheckStep<'step_lt> = (&'step_lt [&'step_lt str], &'step_lt str, &'step_lt str);
#[cfg(feature = "test-utils")]
#[derive(Debug, Clone, PartialEq, Eq)]
enum FileSnapshot {
    Missing,
    Present(String),
}
#[cfg(feature = "test-utils")]
struct RestoreToPrevious<'restore> {
    cargo_toml_snapshot: FileSnapshot,
    lib_rs_snapshot: FileSnapshot,
    path_cargo_toml: &'restore Path,
    path_lib_rs: &'restore Path,
}
#[cfg(feature = "test-utils")]
impl Drop for RestoreToPrevious<'_> {
    fn drop(&mut self) {
        restore_snapshot(
            self.path_lib_rs,
            &self.lib_rs_snapshot,
            "79231418",
            "e28698f2",
        );
        restore_snapshot(
            self.path_cargo_toml,
            &self.cargo_toml_snapshot,
            "ec801a87",
            "5fd52cd7",
        );
    }
}
#[cfg(feature = "test-utils")]
fn write_or_panic(path: &Path, cnt: &str, write_er_id: &str) {
    write(path, cnt).unwrap_or_else(|er| panic!("{write_er_id}: {er}"));
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // shared with tests; keep error-id based read behavior in one place
fn capture_snapshot(path: &Path, read_er_id: &str) -> FileSnapshot {
    match read_to_string(path) {
        Ok(cnt) => FileSnapshot::Present(cnt),
        Err(er) if er.kind() == ErrorKind::NotFound => FileSnapshot::Missing,
        Err(er) => panic!("{read_er_id}: {er}"),
    }
}
#[cfg(feature = "test-utils")]
fn restore_snapshot(path: &Path, snapshot: &FileSnapshot, write_er_id: &str, rm_er_id: &str) {
    match snapshot {
        FileSnapshot::Present(cnt) => write_or_panic(path, cnt, write_er_id),
        FileSnapshot::Missing => {
            if let Err(er) = remove_file(path)
                && er.kind() != ErrorKind::NotFound
            {
                panic!("{rm_er_id}: {er}");
            }
        }
    }
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // split out intentionally to keep low-level cargo spawn/status check reusable from orchestration helper
fn run_cargo_checked(
    target_crate_dir: &Path,
    args: &[&str],
    cmd_spawn_er_id: &str,
    failed_id: &str,
) {
    let status = Command::new("cargo")
        .current_dir(target_crate_dir)
        .args(args)
        .status()
        .unwrap_or_else(|er| panic!("{cmd_spawn_er_id}: {er}"));
    assert!(status.success(), "{failed_id}: {status}");
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // centralizes ordered cargo check execution to keep command flow reusable and consistent
fn run_cargo_check_steps(target_crate_dir: &Path, steps: &[CargoCheckStep<'_>]) {
    for (args, cmd_spawn_er_id, failed_id) in steps {
        run_cargo_checked(target_crate_dir, args, cmd_spawn_er_id, failed_id);
    }
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // shared helper centralizes snapshot capture + restore guard construction for generated files
fn mk_restore_to_previous<'restore>(
    path_cargo_toml: &'restore Path,
    path_lib_rs: &'restore Path,
) -> RestoreToPrevious<'restore> {
    let cargo_toml_snapshot = capture_snapshot(path_cargo_toml, "bf40d675");
    let lib_rs_snapshot = capture_snapshot(path_lib_rs, "adf9f42b");
    RestoreToPrevious {
        cargo_toml_snapshot,
        lib_rs_snapshot,
        path_cargo_toml,
        path_lib_rs,
    }
}
#[cfg(feature = "test-utils")]
#[allow(clippy::single_call_fn)] // shared helper keeps generated-file write flow consistent for clippy-check setup
fn write_generated_files(
    path_cargo_toml: &Path,
    cargo_toml_full: &str,
    path_lib_rs: &Path,
    content_to_gen: &str,
) {
    write_or_panic(path_cargo_toml, cargo_toml_full, "3757da9b");
    write_or_panic(path_lib_rs, content_to_gen, "55124f90");
}
#[cfg(feature = "test-utils")]
pub fn clippy_check(crate_name: &str, cmd_path: &str, extra_cnt: &str, content_to_gen: &str) {
    use std::path::PathBuf;
    let crate_path = PathBuf::from("..").join(crate_name);
    let cargo_toml_cnt = format!(
        r#"[package]
name = "{crate_name}"
publish = false
version = "0.1.0"
edition = "2024"
description = "description"
repository = "repository"
readme = "readme"
license = "license"
keywords = ["keyword"]
categories = ["category"]
[lints]
workspace = true"#
    );
    let path_lib_rs = crate_path.join("src/lib.rs");
    let path_cargo_toml = crate_path.join("Cargo.toml");
    let cargo_toml_full = format!("{cargo_toml_cnt}\n{extra_cnt}");
    let _restore_to_previous = mk_restore_to_previous(&path_cargo_toml, &path_lib_rs);
    write_generated_files(
        &path_cargo_toml,
        &cargo_toml_full,
        &path_lib_rs,
        content_to_gen,
    );
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_crate_dir = manifest_dir.join(PathBuf::from(cmd_path).join(crate_name));
    run_cargo_check_steps(&target_crate_dir, &CARGO_CHECK_STEPS);
}
#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {
    use super::{FileSnapshot, capture_snapshot, restore_snapshot};
    use std::{
        env::temp_dir,
        fs::{create_dir_all, read_to_string, remove_dir_all, write},
        path::{Path, PathBuf},
        process,
        sync::atomic::{AtomicUsize, Ordering},
    };
    static TEST_SEQ: AtomicUsize = AtomicUsize::new(0);
    struct TmpDir(PathBuf);
    impl TmpDir {
        fn new() -> Self {
            let seq = TEST_SEQ.fetch_add(1, Ordering::Relaxed);
            let path = temp_dir().join(format!("macro_clippy_check_cmn_{}_{}", process::id(), seq));
            create_dir_all(&path).expect("2b24ef1a");
            Self(path)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TmpDir {
        fn drop(&mut self) {
            remove_dir_all(&self.0).unwrap_or_else(|er| panic!("15ab6a8d: {er}"));
        }
    }
    #[test]
    fn capture_snapshot_returns_missing_for_absent_file() {
        let dir = TmpDir::new();
        let path = dir.path().join("missing.txt");
        let snapshot = capture_snapshot(&path, "9b0e24f1");
        assert_eq!(snapshot, FileSnapshot::Missing);
    }
    #[test]
    fn restore_snapshot_restores_existing_file_content() {
        let dir = TmpDir::new();
        let path = dir.path().join("a.txt");
        write(&path, "old").expect("420e5e9a");
        let snapshot = capture_snapshot(&path, "6de5509e");
        write(&path, "new").expect("29aa4cf7");
        restore_snapshot(&path, &snapshot, "de731978", "6af34450");
        let restored = read_to_string(&path).expect("1ec15e06");
        assert_eq!(restored, "old");
    }
    #[test]
    fn restore_snapshot_removes_generated_file_for_missing_snapshot() {
        let dir = TmpDir::new();
        let path = dir.path().join("g.txt");
        let snapshot = capture_snapshot(&path, "f39c05aa");
        write(&path, "generated").expect("1ebbee98");
        restore_snapshot(&path, &snapshot, "9fa0dd47", "8e31012d");
        assert!(!path.exists());
    }
    #[test]
    fn restore_snapshot_keeps_absent_file_absent_for_missing_snapshot() {
        let dir = TmpDir::new();
        let path = dir.path().join("never_created.txt");
        let snapshot = FileSnapshot::Missing;
        restore_snapshot(&path, &snapshot, "fd2ab7e0", "7df145a8");
        assert!(!path.exists());
    }
}
