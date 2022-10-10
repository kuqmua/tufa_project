use std::env;
use std::process::Command;

fn main() {
    let path = env::current_dir().expect("cannot get current directory");
    println!("The current directory is {}", path.display());
    let first_step = if cfg!(target_os = "linux") {
        Command::new("dir")
            // .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("ls")
            // .arg("-c")
            // .arg("echo hello")
            .output()
            .expect("failed to execute process")
    }
    .stdout;
    println!("{}", String::from_utf8(first_step).unwrap());
}
