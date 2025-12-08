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
    let paths_vec: std::vec::Vec<String> = contents.lines()
        .filter_map(|element| {
            element.find("path = ")
            .map(|index| {
                element.get(index.checked_add(substring_value.len()).expect("error 62d029a8-1f60-490b-bb70-5e51c1034af2")..)
                .expect("error dde185ef-97fc-4652-b67c-76064cff7091")
                .to_string()
            })
        }).collect();
    println!("{:#?} {}", paths_vec, paths_vec.len());
    println!("working..");
    let _unused = std::process::Command::new("git").args(["reset", "--hard"]).output().expect("failed use git reset --hard");
    // let mut threads_vector = Vec::with_capacity(paths_vec.len());
    // let error_vec_arc_mutex =
    //     std::sync::Arc::new(std::sync::Mutex::new(Vec::<CommandError>::new()));
    for path in paths_vec {
        // let error_vec_arc_mutex_arc_cloned = std::sync::Arc::clone(&error_vec_arc_mutex);
        let canonicalize_pathbuf_as_string_cloned = canonicalize_pathbuf_as_string.clone();
        // let handle = std::thread::spawn(move || {
        if let Err(error) = commands(&canonicalize_pathbuf_as_string_cloned, &path) {
            // let mut error_vec_arc_mutex_arc_cloned_locked = error_vec_arc_mutex_arc_cloned
            //     .lock()
            //     .expect("cannot lock error_vec_arc_mutex_arc_cloned");
            // error_vec_arc_mutex_arc_cloned_locked.push(e);
            panic!("command error: {error:#?}")
        }
        // });
        // threads_vector.push(handle);
    }
    // threads_vector.into_iter().for_each(|t| {
    //     t.join().expect("cannot join one of the threads");
    // });
    // let error_vec_arc_mutex_done = error_vec_arc_mutex
    //     .lock()
    //     .expect("cannot lock error_vec_arc_mutex")
    //     .to_vec();
    // match error_vec_arc_mutex_done.is_empty() {
    //     true => println!("done!"),
    //     false => {
    //         eprint!("{:#?}", error_vec_arc_mutex_done)
    //     }
    // }
}

#[derive(Clone, Debug)]
enum CommandError {
    CheckoutDot { path: String, error: String },
    SubmoduleUpdate { path: String, error: String },
    CheckoutMain { path: String, error: String },
    Pull { path: String, error: String },
    CargoBuild { path: String, error: String },
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CheckoutDot { path, error } => {
                write!(formatter, "git checkout . error: {error}, path: {path}")
            }
            Self::SubmoduleUpdate { path, error } => {
                write!(formatter, "git submodule update error: {error}, path: {path}")
            }
            Self::CheckoutMain { path, error } => {
                write!(formatter, "git checkout main error: {error}, path: {path}")
            }
            Self::Pull { path, error } => {
                write!(formatter, "git pull error: {error}, path: {path}")
            }
            Self::CargoBuild { path, error } => {
                write!(formatter, "cargo build error: {error}, path: {path}")
            }
        }
    }
}

fn commands(canonicalize_pathbuf_as_string: &str, path: &str) -> Result<(), CommandError> {
    let path = format!("{canonicalize_pathbuf_as_string}/{path}");
    println!("start {path}");
    if let Err(error) = std::process::Command::new("git").args(["checkout", "."]).current_dir(&path).output() {
        return Err(CommandError::CheckoutDot { path, error: format!("{error}") });
    }
    println!("git checkout . {path}");
    if let Err(error) = std::process::Command::new("git").args(["submodule", "update", "--init", "--recursive"]).current_dir(&path).output() {
        return Err(CommandError::SubmoduleUpdate { path, error: format!("{error}") });
    }
    println!("git submodule update --init --recursive {path}");
    if let Err(error) = std::process::Command::new("git").args(["pull"]).current_dir(&path).output() {
        return Err(CommandError::Pull { path, error: format!("{error}") });
    }
    println!("git pull {path}");
    if let Err(error) = std::process::Command::new("git").args(["checkout", "main"]).current_dir(&path).output() {
        return Err(CommandError::CheckoutMain { path, error: format!("{error}") });
    }
    println!("git checkout main {path}");
    if let Err(error) = std::process::Command::new("cargo").args(["build"]).current_dir(&path).output() {
        return Err(CommandError::CargoBuild { path, error: format!("{error}") });
    }
    println!("cargo build {path}");
    Ok(())
}
