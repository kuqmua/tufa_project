fn main() {
    let _unused = dotenv::dotenv().expect("0964b79a-fd4d-40ef-83dc-e06e2862122e");
    let gitmodules_path_env_name = "GITMODULES_PATH";
    let string_path = match std::env::var(gitmodules_path_env_name) {
        Err(error) => panic!("failed to find std::env::var {gitmodules_path_env_name} {error:#?}"),
        Ok(string_handle) => string_handle,
    };
    let parent_dir_pathbuf = std::path::PathBuf::from(string_path);
    let parent_dir_pathbuf_as_string = parent_dir_pathbuf
        .clone()
        .into_os_string()
        .into_string()
        .expect("c46bf30c-5dbb-4e94-8eb5-6c7926b007c9");
    let canonicalize_pathbuf = match std::fs::canonicalize(&parent_dir_pathbuf) {
        Ok(pathbuf) => pathbuf,
        Err(error) => panic!("error: {error}, path: {parent_dir_pathbuf_as_string}"),
    };
    let canonicalize_pathbuf_as_string = canonicalize_pathbuf
        .into_os_string()
        .into_string()
        .expect("9ce61c06-b90d-4570-8092-083ea375faa2");
    let contents = std::fs::read_to_string(format!("{parent_dir_pathbuf_as_string}.gitmodules"))
        .expect("c6dd3528-6f4a-426a-954b-57067b56c506");
    let _unused_git_version = std::process::Command::new("git")
        .args(["version"])
        .output()
        .expect("6fb6579e-e72a-4c7c-9a3e-81e771e86c0b");
    let substring_value = "path = ";
    let paths_vec: Vec<String> = contents
        .lines()
        .filter_map(|element| {
            element.find("path = ").map(|index| {
                element
                    .get(
                        index
                            .checked_add(substring_value.len())
                            .expect("62d029a8-1f60-490b-bb70-5e51c1034af2")..,
                    )
                    .expect("dde185ef-97fc-4652-b67c-76064cff7091")
                    .to_owned()
            })
        })
        .collect();
    println!("{:#?} {}", paths_vec, paths_vec.len());
    println!("working..");
    let _unused_git_reset_hard = std::process::Command::new("git")
        .args(["reset", "--hard"])
        .output()
        .expect("4fde85e0-22b3-46ab-b3c1-137558da2091");
    for path_element in paths_vec {
        let path = format!("{canonicalize_pathbuf_as_string}/{path_element}");
        println!("start {path}");
        if let Err(error) = std::process::Command::new("git")
            .args(["checkout", "."])
            .current_dir(&path)
            .output()
        {
            panic!("git checkout . error: {error}, path: {path}")
        }
        println!("git checkout . {path}");
        if let Err(error) = std::process::Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .current_dir(&path)
            .output()
        {
            panic!("git submodule update error: {error}, path: {path}");
        }
        println!("git submodule update --init --recursive {path}");
        if let Err(error) = std::process::Command::new("git")
            .args(["pull"])
            .current_dir(&path)
            .output()
        {
            panic!("git pull error: {error}, path: {path}");
        }
        println!("git pull {path}");
        if let Err(error) = std::process::Command::new("git")
            .args(["checkout", "main"])
            .current_dir(&path)
            .output()
        {
            panic!("git checkout main error: {error}, path: {path}");
        }
        println!("git checkout main {path}");
        if let Err(error) = std::process::Command::new("cargo")
            .args(["build"])
            .current_dir(&path)
            .output()
        {
            panic!("cargo build error: {error}, path: {path}");
        }
        println!("cargo build {path}");
    }
}
