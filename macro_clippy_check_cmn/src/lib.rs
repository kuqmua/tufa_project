#[cfg(feature = "test-utils")]
pub fn clippy_check(crate_name: &str, cmd_path: &str, extra_cnt: &str, content_to_gen: &str) {
    use std::{
        fs::write,
        path::{Path, PathBuf},
        process::{Command, ExitStatus},
    };
    fn write_or_panic(path: &Path, cnt: &str, write_er_id: &str) {
        write(path, cnt).unwrap_or_else(|_| panic!("{write_er_id}"));
    }
    fn run_cargo(target_crate_dir: &Path, args: &[&str], cmd_spawn_er_id: &str) -> ExitStatus {
        Command::new("cargo")
            .current_dir(target_crate_dir)
            .args(args)
            .status()
            .unwrap_or_else(|_| panic!("{cmd_spawn_er_id}"))
    }
    struct RestoreToPrevious<'restore> {
        cargo_toml_cnt: &'restore str,
        lib_rs_cnt: &'restore str,
        path_cargo_toml: &'restore Path,
        path_lib_rs: &'restore Path,
    }
    impl Drop for RestoreToPrevious<'_> {
        fn drop(&mut self) {
            write_or_panic(self.path_lib_rs, self.lib_rs_cnt, "79231418");
            write_or_panic(self.path_cargo_toml, self.cargo_toml_cnt, "ec801a87");
        }
    }
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
    let lib_rs_cnt = "\n";
    let path_lib_rs = crate_path.join("src/lib.rs");
    let path_cargo_toml = crate_path.join("Cargo.toml");
    write_or_panic(&path_lib_rs, lib_rs_cnt, "404ab180");
    let cargo_toml_full = format!("{cargo_toml_cnt}\n{extra_cnt}");
    write_or_panic(&path_cargo_toml, &cargo_toml_full, "3757da9b");
    write_or_panic(&path_lib_rs, content_to_gen, "55124f90");
    let _restore_to_previous = RestoreToPrevious {
        cargo_toml_cnt: &cargo_toml_cnt,
        lib_rs_cnt,
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
