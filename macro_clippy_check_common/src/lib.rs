#[cfg(feature = "test-utils")]
pub fn clippy_check(
    crate_name: &str,
    command_path: &str,
    additional_content: &str,
    content_to_gen: &str,
) {
    use std::{fs::write, process::Command};
    let path = format!("../{crate_name}/");
    let cargo_toml_content = format!(
        r#"[package]
name = "{crate_name}"
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
    let lib_rs_content = "\n";
    let path_lib_rs = format!("{path}src/lib.rs");
    let path_cargo_toml = format!("{path}Cargo.toml");
    write(&path_lib_rs, lib_rs_content).expect("404ab180");
    write(
        &path_cargo_toml,
        format!("{cargo_toml_content}\n{additional_content}"),
    )
    .expect("3757da9b");
    write(&path_lib_rs, content_to_gen).expect("55124f90");
    let return_to_previous = || {
        write(&path_lib_rs, lib_rs_content).expect("79231418");
        write(&path_cargo_toml, cargo_toml_content).expect("ec801a87");
    };
    let manifest_dir = {
        use std::path::PathBuf;
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    };
    let target_crate_dir = manifest_dir.join(format!("{command_path}{crate_name}"));
    if let Ok(value_90318089) = Command::new("cargo")
        .current_dir(&target_crate_dir)
        .args(["fmt"])
        .status()
    {
        assert!(value_90318089.success(), "2a1deb01");
    } else {
        return_to_previous();
        panic!("8dc4f045");
    }
    if let Ok(value_f263835c) = Command::new("cargo")
        .current_dir(&target_crate_dir)
        .args([
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-A",
            "warnings",
        ])
        .status()
    {
        assert!(value_f263835c.success(), "2c037283");
        return_to_previous();
    } else {
        return_to_previous();
        panic!("cd48b869");
    }
}
