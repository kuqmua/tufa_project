#[cfg(feature = "test-utils")]
use std::{
    fs::{read_to_string, remove_file, write},
    io::ErrorKind,
    path::Path,
    process::{Command, ExitStatus},
};
#[cfg(feature = "test-utils")]
#[derive(Debug, Clone, PartialEq, Eq)]
enum FileSnapshot {
    Missing,
    Present(String),
}
#[cfg(feature = "test-utils")]
struct RestoreToPrevious<'restore> {
    cargo_toml_snapshot: &'restore FileSnapshot,
    lib_rs_snapshot: &'restore FileSnapshot,
    path_cargo_toml: &'restore Path,
    path_lib_rs: &'restore Path,
}
#[cfg(feature = "test-utils")]
impl Drop for RestoreToPrevious<'_> {
    fn drop(&mut self) {
        restore_snapshot(
            self.path_lib_rs,
            self.lib_rs_snapshot,
            "79231418",
            "e28698f2",
        );
        restore_snapshot(
            self.path_cargo_toml,
            self.cargo_toml_snapshot,
            "ec801a87",
            "5fd52cd7",
        );
    }
}
#[cfg(feature = "test-utils")]
fn write_or_panic(path: &Path, cnt: &str, write_er_id: &str) {
    write(path, cnt).unwrap_or_else(|_| panic!("{write_er_id}"));
}
#[cfg(feature = "test-utils")]
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
            if path.exists() {
                remove_file(path).unwrap_or_else(|_| panic!("{rm_er_id}"));
            }
        }
    }
}
#[cfg(feature = "test-utils")]
fn run_cargo(target_crate_dir: &Path, args: &[&str], cmd_spawn_er_id: &str) -> ExitStatus {
    Command::new("cargo")
        .current_dir(target_crate_dir)
        .args(args)
        .status()
        .unwrap_or_else(|_| panic!("{cmd_spawn_er_id}"))
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
    let cargo_toml_snapshot = capture_snapshot(&path_cargo_toml, "bf40d675");
    let lib_rs_snapshot = capture_snapshot(&path_lib_rs, "adf9f42b");
    let cargo_toml_full = format!("{cargo_toml_cnt}\n{extra_cnt}");
    write_or_panic(&path_cargo_toml, &cargo_toml_full, "3757da9b");
    write_or_panic(&path_lib_rs, content_to_gen, "55124f90");
    let _restore_to_previous = RestoreToPrevious {
        cargo_toml_snapshot: &cargo_toml_snapshot,
        lib_rs_snapshot: &lib_rs_snapshot,
        path_cargo_toml: &path_cargo_toml,
        path_lib_rs: &path_lib_rs,
    };
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_crate_dir = manifest_dir.join(format!("{cmd_path}{crate_name}"));
    let fmt_status = run_cargo(&target_crate_dir, &["fmt"], "8dc4f045");
    assert!(fmt_status.success(), "2a1deb01");
    let clippy_status = run_cargo(
        &target_crate_dir,
        &[
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-A",
            "warnings",
        ],
        "cd48b869",
    );
    assert!(clippy_status.success(), "2c037283");
}
#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {
    use super::{FileSnapshot, capture_snapshot, restore_snapshot};
    use std::{
        env::temp_dir,
        fs::{create_dir_all, read_to_string, remove_dir_all, write},
        path::PathBuf,
        process::id,
        time::{SystemTime, UNIX_EPOCH},
    };
    fn mk_tmp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("86cdd731")
            .as_nanos();
        let pid = id();
        temp_dir().join(format!("macro_clippy_check_cmn_{pid}_{nanos}"))
    }
    #[test]
    fn capture_snapshot_returns_missing_for_absent_file() {
        let dir = mk_tmp_dir();
        create_dir_all(&dir).expect("2b24ef1a");
        let path = dir.join("missing.txt");
        let snapshot = capture_snapshot(&path, "9b0e24f1");
        assert_eq!(snapshot, FileSnapshot::Missing);
        remove_dir_all(&dir).expect("15ab6a8d");
    }
    #[test]
    fn restore_snapshot_restores_existing_file_content() {
        let dir = mk_tmp_dir();
        create_dir_all(&dir).expect("9165cd97");
        let path = dir.join("a.txt");
        write(&path, "old").expect("420e5e9a");
        let snapshot = capture_snapshot(&path, "6de5509e");
        write(&path, "new").expect("29aa4cf7");
        restore_snapshot(&path, &snapshot, "de731978", "6af34450");
        let restored = read_to_string(&path).expect("1ec15e06");
        assert_eq!(restored, "old");
        remove_dir_all(&dir).expect("d55ec6ad");
    }
    #[test]
    fn restore_snapshot_removes_generated_file_for_missing_snapshot() {
        let dir = mk_tmp_dir();
        create_dir_all(&dir).expect("45416731");
        let path = dir.join("g.txt");
        let snapshot = capture_snapshot(&path, "f39c05aa");
        write(&path, "generated").expect("1ebbee98");
        restore_snapshot(&path, &snapshot, "9fa0dd47", "8e31012d");
        assert!(!path.exists());
        remove_dir_all(&dir).expect("7be0cf86");
    }
}
