#[test]
fn ci_check_env_file_exists() {
    if let Err(e) = std::fs::File::open(format!(
        "{}{}",
        crate::tests::constants::PATH_TO_ENV_FILE,
        crate::tests::constants::ENV_FILE_NAME
    )) {
        panic!(
            "file: {PATH_TO_ENV_FILE}{ENV_FILE_NAME} error: {e}",
            crate::tests::constants::PATH_TO_ENV_FILE,
            crate::tests::constants::ENV_FILE_NAME
        );
    }
}
