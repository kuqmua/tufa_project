use core::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[feature(absolute_path)]
fn main() {
    let parent_dir_pathbuf = PathBuf::from("../");
    let parent_dir_pathbuf_as_string = parent_dir_pathbuf
        .clone()
        .into_os_string()
        .into_string()
        .expect("cannot convert parent_dir_pathbuf to string");
    let canonicalize_pathbuf = match fs::canonicalize(&parent_dir_pathbuf) {
        Ok(pathbuf) => pathbuf,
        Err(e) => panic!("error: {e}, path: {parent_dir_pathbuf_as_string}"),
    };
    let canonicalize_pathbuf_as_string = canonicalize_pathbuf
        .into_os_string()
        .into_string()
        .expect("cannot convert canonicalize_pathbuf_as_string to string");
    let contents = fs::read_to_string("../.gitmodules").expect("cannot read .gitmodules file");
    Command::new("git")
        .args(["version"])
        .output()
        .expect("failed use git version (just to check is there git installed or not)");
    let substring_value = "path = ";
    let paths_vec: Vec<String> = contents
        .lines()
        .filter_map(|e| match e.find("path = ") {
            Some(index) => Some(e[index + substring_value.len()..].to_string()),
            None => None,
        })
        .collect();
    if cfg!(target_os = "linux") {
        threads_logic(paths_vec, canonicalize_pathbuf_as_string);
    } else if cfg!(target_os = "windows") {
        threads_logic(paths_vec, canonicalize_pathbuf_as_string);
    } else {
        panic!("cannot find out target os")
    };
}

fn threads_logic(paths_vec: Vec<String>, canonicalize_pathbuf_as_string: String) {
    println!("working..");
    let mut threads_vector = Vec::with_capacity(paths_vec.len());
    let error_vec_arc_mutex = Arc::new(Mutex::new(Vec::<GitCommandError>::new()));
    paths_vec.into_iter().for_each(|path| {
        let error_vec_arc_mutex_arc_cloned = Arc::clone(&error_vec_arc_mutex);
        let canonicalize_pathbuf_as_string_cloned = canonicalize_pathbuf_as_string.clone();
        let handle = thread::spawn(move || {
            if let Err(e) = commands(canonicalize_pathbuf_as_string_cloned, path) {
                let mut error_vec_arc_mutex_arc_cloned_locked = error_vec_arc_mutex_arc_cloned
                    .lock()
                    .expect("cannot lock error_vec_arc_mutex_arc_cloned");
                error_vec_arc_mutex_arc_cloned_locked.push(e);
            }
        });
        threads_vector.push(handle);
    });
    threads_vector.into_iter().for_each(|t| {
        t.join().expect("cannot join one of the threads");
    });
    let error_vec_arc_mutex_done = error_vec_arc_mutex
        .lock()
        .expect("cannot lock error_vec_arc_mutex")
        .to_vec();
    match error_vec_arc_mutex_done.is_empty() {
        true => println!("done!"),
        false => {
            eprint!("{:#?}", error_vec_arc_mutex_done)
        }
    }
}

#[derive(Clone, Debug)]
enum GitCommandError {
    CheckoutDot { path: String, error: String },
    CheckoutMain { path: String, error: String },
    Pull { path: String, error: String },
}

impl Display for GitCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GitCommandError::CheckoutDot { path, error } => {
                write!(f, "git checkout . error: {}, path: {}", error, path)
            }
            GitCommandError::CheckoutMain { path, error } => {
                write!(f, "git checkout main error: {}, path: {}", error, path)
            }
            GitCommandError::Pull { path, error } => {
                write!(f, "git pull error: {}, path: {}", error, path)
            }
        }
    }
}

fn commands(canonicalize_pathbuf_as_string: String, path: String) -> Result<(), GitCommandError> {
    let path = format!("{}/{}", canonicalize_pathbuf_as_string, path);
    if let Err(e) = Command::new("git")
        .args(["checkout", "."])
        .current_dir(&path)
        .output()
    {
        return Err(GitCommandError::CheckoutDot {
            path: path,
            error: format!("{e}"),
        });
    }
    if let Err(e) = Command::new("git")
        .args(["checkout", "main"])
        .current_dir(&path)
        .output()
    {
        return Err(GitCommandError::CheckoutMain {
            path: path,
            error: format!("{e}"),
        });
    }
    if let Err(e) = Command::new("git")
        .args(["pull"])
        .current_dir(&path)
        .output()
    {
        return Err(GitCommandError::Pull {
            path: path,
            error: format!("{e}"),
        });
    }
    Ok(())
}
