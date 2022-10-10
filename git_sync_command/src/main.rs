use std::env;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("../tufa_server/src") {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
    let path = env::current_dir().expect("cannot get current directory");
    println!("The current directory is {}", path.display());
    let path = env::home_dir().expect("cannot get home directory");
    println!("The home directory is {}", path.display());
    let first_step = if cfg!(target_os = "linux") {
        Command::new("ls")
            // .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
        // ;
        // Command::new("dir")
        //     // .args(["/C", "echo hello"])
        //     .output()
        //     .expect("failed to execute process");
    } else if cfg!(target_os = "windows") {
        Command::new("dir")
            // .arg("-c")
            // .arg("echo hello")
            .output()
            .expect("failed to execute process")
    } else {
        panic!("cannot find out target os")
    }
    .stdout;
    println!("{}", String::from_utf8(first_step).unwrap());
}
// git config
// git init
// git clone
// git add
// git commit
// git diff
// git reset
// git status
// git rm
// git log
// git show
// git tag
// git branch
// git checkout
// git merge
// git remote
// git push
// git pull
// git stash
