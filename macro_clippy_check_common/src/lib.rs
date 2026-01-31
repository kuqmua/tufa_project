#[cfg(feature = "test-utils")]
pub fn clippy_check(crate_name: &str, additional_content: &str, content_to_generate: &str) {
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
    std::fs::write(&path_lib_rs, lib_rs_content).expect("404ab180-10f0-4b82-95ef-5635488fe436");
    std::fs::write(
        &path_cargo_toml,
        format!("{cargo_toml_content}\n{additional_content}"),
    )
    .expect("3757da9b-0457-4301-9e68-efb60737dc71");
    std::fs::write(&path_lib_rs, content_to_generate)
        .expect("55124f90-c7c2-40b5-8b66-695635ea6afd");
    let return_to_previous = || {
        std::fs::write(&path_lib_rs, lib_rs_content).expect("79231418-b44a-4dac-8a88-3d8403024827");
        std::fs::write(&path_cargo_toml, cargo_toml_content)
            .expect("ec801a87-2c48-4c64-9c6a-7e686db91094");
    };
    if let Ok(value_90318089) = std::process::Command::new("cargo")
        .args(["fmt", "--", &path_lib_rs])
        .status()
    {
        assert!(
            value_90318089.success(),
            "2a1deb01-ec64-4d13-94d5-47b647b2950d"
        );
    } else {
        return_to_previous();
        panic!("8dc4f045-93a0-46a3-a6f6-0eab002dbb0c");
    }
    if let Ok(value_f263835c) = std::process::Command::new("cargo")
        .args(["clippy", "--all-targets", "--all-features"])
        .status()
    {
        assert!(
            value_f263835c.success(),
            "2c037283-420c-4076-8042-1eac09ba1a23"
        );
        return_to_previous();
    } else {
        return_to_previous();
        panic!("cd48b869-7726-412d-b1b6-b89deca000c3");
    }
}
