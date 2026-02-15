use dotenv::dotenv;
use std::{env, fs, path::PathBuf, process::Command};
fn main() {
    let _unused = dotenv().expect("0964b79a");
    let gitmodules_path_env_name = "GITMODULES_PATH";
    let string_path = env::var(gitmodules_path_env_name).expect("25f5388e");
    let parent_dir_pathbuf = PathBuf::from(string_path);
    let parent_dir_pathbuf_as_string = parent_dir_pathbuf
        .clone()
        .into_os_string()
        .into_string()
        .expect("c46bf30c");
    let canonicalize_pathbuf = fs::canonicalize(&parent_dir_pathbuf).expect("2bcd326b");
    let canonicalize_pathbuf_as_string = canonicalize_pathbuf
        .into_os_string()
        .into_string()
        .expect("9ce61c06");
    let contents =
        fs::read_to_string(format!("{parent_dir_pathbuf_as_string}.gitmodules")).expect("c6dd3528");
    let _unused_git_version = Command::new("git")
        .args(["version"])
        .output()
        .expect("6fb6579e");
    let substring_value = "path = ";
    let paths_vec: Vec<String> = contents
        .lines()
        .filter_map(|el_0731ade5| {
            el_0731ade5.find("path = ").map(|index| {
                el_0731ade5
                    .get(index.checked_add(substring_value.len()).expect("62d029a8")..)
                    .expect("dde185ef")
                    .to_owned()
            })
        })
        .collect();
    println!("{:#?} {}", paths_vec, paths_vec.len());
    println!("working..");
    let _unused_git_reset_hard = Command::new("git")
        .args(["reset", "--hard"])
        .output()
        .expect("4fde85e0");
    for el_f232fe33 in paths_vec {
        let path = format!("{canonicalize_pathbuf_as_string}/{el_f232fe33}");
        println!("start {path}");
        let _unused0 = Command::new("git")
            .args(["checkout", "."])
            .current_dir(&path)
            .output()
            .expect("33aafc5b");
        println!("git checkout . {path}");
        let _unused1 = Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .current_dir(&path)
            .output()
            .expect("763e5b36");
        println!("git submodule update --init --recursive {path}");
        let _unused2 = Command::new("git")
            .args(["pull"])
            .current_dir(&path)
            .output()
            .expect("c2102866");
        println!("git pull {path}");
        let _unused3 = Command::new("git")
            .args(["checkout", "main"])
            .current_dir(&path)
            .output()
            .expect("2992301a");
        println!("git checkout main {path}");
        let _unused4 = Command::new("cargo")
            .args(["build"])
            .current_dir(&path)
            .output()
            .expect("e3eca580");
        println!("cargo build {path}");
    }
}
