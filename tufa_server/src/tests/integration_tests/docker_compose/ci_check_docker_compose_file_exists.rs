#[test]
fn ci_check_docker_compose_file_exists() {
    if let Err(e) = fs::read_to_string(&format!(
        "{}{}",
        crate::tests::constants::PATH_TO_DOCKER_COMPOSE_FILE,
        crate::tests::constants::DOCKER_COMPOSE_FILE_NAME
    )) {
        panic!(
            "file: {}{} error: {e}",
            crate::tests::constants::PATH_TO_DOCKER_COMPOSE_FILE,
            crate::tests::constants::DOCKER_COMPOSE_FILE_NAME
        );
    }
}
