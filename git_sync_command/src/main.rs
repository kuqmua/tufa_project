fn main() {
    let _unused = dotenv::dotenv().expect("cannot initialize dotenv");
    let gitmodules_path_env_name = "GITMODULES_PATH";
    let string_path = match std::env::var(gitmodules_path_env_name) {
        Err(error) => panic!("failed to find std::env::var {gitmodules_path_env_name} {error:#?}"),
        Ok(string_handle) => string_handle,
    };
    let parent_dir_pathbuf = std::path::PathBuf::from(string_path);
    let parent_dir_pathbuf_as_string = parent_dir_pathbuf.clone().into_os_string().into_string().expect("cannot convert parent_dir_pathbuf to string");
    let canonicalize_pathbuf = match std::fs::canonicalize(&parent_dir_pathbuf) {
        Ok(pathbuf) => pathbuf,
        Err(error) => panic!("error: {error}, path: {parent_dir_pathbuf_as_string}"),
    };
    let canonicalize_pathbuf_as_string = canonicalize_pathbuf.into_os_string().into_string().expect("cannot convert canonicalize_pathbuf_as_string to string");
    let contents = std::fs::read_to_string(format!("{parent_dir_pathbuf_as_string}.gitmodules")).expect("cannot read .gitmodules file");
    let _unused = std::process::Command::new("git").args(["version"]).output().expect("failed use git version (just to check is there git installed or not)");
    let substring_value = "path = ";
    let paths_vec: Vec<String> = contents.lines()
        .filter_map(|element| {
            element.find("path = ")
            .map(|index| {
                element.get(index.checked_add(substring_value.len()).expect("error 62d029a8-1f60-490b-bb70-5e51c1034af2")..)
                .expect("error dde185ef-97fc-4652-b67c-76064cff7091")
                .to_owned()
            })
        }).collect();
    println!("{:#?} {}", paths_vec, paths_vec.len());
    println!("working..");
    let _unused = std::process::Command::new("git").args(["reset", "--hard"]).output().expect("failed use git reset --hard");
    for path in paths_vec {
        let path = format!("{canonicalize_pathbuf_as_string}/{path}");
        println!("start {path}");
        if let Err(error) = std::process::Command::new("git").args(["checkout", "."]).current_dir(&path).output() {
            panic!("git checkout . error: {error}, path: {path}")
        }
        println!("git checkout . {path}");
        if let Err(error) = std::process::Command::new("git").args(["submodule", "update", "--init", "--recursive"]).current_dir(&path).output() {
            panic!("git submodule update error: {error}, path: {path}");
        }
        println!("git submodule update --init --recursive {path}");
        if let Err(error) = std::process::Command::new("git").args(["pull"]).current_dir(&path).output() {
            panic!("git pull error: {error}, path: {path}");
        }
        println!("git pull {path}");
        if let Err(error) = std::process::Command::new("git").args(["checkout", "main"]).current_dir(&path).output() {
            panic!("git checkout main error: {error}, path: {path}");
        }
        println!("git checkout main {path}");
        if let Err(error) = std::process::Command::new("cargo").args(["build"]).current_dir(&path).output() {
            panic!("cargo build error: {error}, path: {path}");
        }
        println!("cargo build {path}");
    }
}
