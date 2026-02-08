fn main() {
    let _unused = dotenv::dotenv().expect("0964b79a-fd4d-40ef-83dc-e06e2862122e");
    let gitmodules_path_env_name = "GITMODULES_PATH";
    let string_path =
        std::env::var(gitmodules_path_env_name).expect("25f5388e-57cf-46df-ba63-98d259cfe4da");
    let parent_dir_pathbuf = std::path::PathBuf::from(string_path);
    let parent_dir_pathbuf_as_string = parent_dir_pathbuf
        .clone()
        .into_os_string()
        .into_string()
        .expect("c46bf30c-5dbb-4e94-8eb5-6c7926b007c9");
    let canonicalize_pathbuf =
        std::fs::canonicalize(&parent_dir_pathbuf).expect("2bcd326b-6576-4f00-836f-5857cc617770");
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
        .filter_map(|el_0731ade5| {
            el_0731ade5.find("path = ").map(|index| {
                el_0731ade5
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
    for el_f232fe33 in paths_vec {
        let path = format!("{canonicalize_pathbuf_as_string}/{el_f232fe33}");
        println!("start {path}");
        let _unused0 = std::process::Command::new("git")
            .args(["checkout", "."])
            .current_dir(&path)
            .output()
            .expect("33aafc5b-ff75-42a8-9c69-ed1ff55c12a5");
        println!("git checkout . {path}");
        let _unused1 = std::process::Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .current_dir(&path)
            .output()
            .expect("763e5b36-cadb-4ab9-85df-96e297a24d7c");
        println!("git submodule update --init --recursive {path}");
        let _unused2 = std::process::Command::new("git")
            .args(["pull"])
            .current_dir(&path)
            .output()
            .expect("c2102866-7e6e-4c99-bacf-879f582be6a6");
        println!("git pull {path}");
        let _unused3 = std::process::Command::new("git")
            .args(["checkout", "main"])
            .current_dir(&path)
            .output()
            .expect("2992301a-d98e-41d4-b292-0c57e59f4ebd");
        println!("git checkout main {path}");
        let _unused4 = std::process::Command::new("cargo")
            .args(["build"])
            .current_dir(&path)
            .output()
            .expect("e3eca580-f4dc-4a7b-9d40-3d9a9d070b4b");
        println!("cargo build {path}");
    }
}
