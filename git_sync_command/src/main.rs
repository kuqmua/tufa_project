use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "linux") {
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
    };
    let hello = output.stdout;
    println!("{}", String::from_utf8(hello).unwrap());
}
