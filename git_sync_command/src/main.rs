use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[feature(absolute_path)]
fn main() {
    let parent_dir_pathbuf = PathBuf::from("../");
    let canonicalize_pathbuf = match fs::canonicalize(&parent_dir_pathbuf) {
        Ok(pathbuf) => pathbuf,
        Err(e) => panic!("{e}"),
    };
    let canonicalize_pathbuf_as_string =
        canonicalize_pathbuf.into_os_string().into_string().unwrap();
    let contents =
        fs::read_to_string("../.gitmodules").expect("Should have been able to read the file");
    let substring_value = "path = ";
    let paths_vec: Vec<String> = contents
        .lines()
        .filter_map(|e| match e.find("path = ") {
            Some(index) => Some(e[index + substring_value.len()..].to_string()),
            None => None,
        })
        .collect();
    println!("g {:#?} )))", paths_vec);
    if cfg!(target_os = "linux") {
        // let mut list_dir = Command::new("ls");
        // list_dir.status().expect("process failed to execute");

        // println!("{}", b);
        // list_dir.current_dir(b);

        // // And then execute `ls` again but in the root directory.
        // list_dir.status().expect("process failed to execute");
        // // let f = list_dir.get_current_dir().unwrap();
        // println!("##{}##", list_dir.get_current_dir().unwrap().display());
        // Command::new("cd").arg("home").status().unwrap();
        // let first_step = Command::new("ls")
        //     .current_dir("/home")
        //     // .args([".."])
        //     .output()
        //     .expect("failed to execute process")
        //     .stdout;
        // println!("@@@{}@@@", String::from_utf8(first_step).unwrap());
        paths_vec.iter().for_each(|path| {
            let path = format!("{}/{}", canonicalize_pathbuf_as_string, path);
            println!("path {}", path);
            let first_step = Command::new("ls")
                // .args(["/C", "echo hello"])
                .current_dir(path)
                .output()
                .expect("failed to execute process")
                .stdout;
            println!("{}\n", String::from_utf8(first_step).unwrap());
        });
    } else if cfg!(target_os = "windows") {
        todo!("todo on windows")
    } else {
        panic!("cannot find out target os")
    };
    // .stdout;
    // println!("{}", String::from_utf8(first_step).unwrap());
    //
    // let f: Vec<_> = contents.match_indices("path = ").collect();
    // println!("f {:#?} )))", f);
    // println!("With text:\n{contents}");
    // let walker = WalkDir::new("../tufa_server/src").into_iter();
    // for entry in walker.filter_entry(|e| is_hidden(e)) {
    //     let entry = entry.unwrap();
    //     println!("{}", entry.path().display());
    // }

    // // for entry in WalkDir::new("../tufa_server/src") {
    // //     let entry = entry.unwrap();
    // //     println!("{}", entry.path().display());
    // // }
    // let path = env::current_dir().expect("cannot get current directory");
    // println!("The current directory is {}", path.display());
    // let path = env::home_dir().expect("cannot get home directory");
    // println!("The home directory is {}", path.display());
    // let first_step = if cfg!(target_os = "linux") {
    //     Command::new("ls")
    //         // .args(["/C", "echo hello"])
    //         .output()
    //         .expect("failed to execute process")
    //     // ;
    //     // Command::new("dir")
    //     //     // .args(["/C", "echo hello"])
    //     //     .output()
    //     //     .expect("failed to execute process");
    // } else if cfg!(target_os = "windows") {
    //     Command::new("dir")
    //         // .arg("-c")
    //         // .arg("echo hello")
    //         .output()
    //         .expect("failed to execute process")
    // } else {
    //     panic!("cannot find out target os")
    // }
    // .stdout;
    // println!("{}", String::from_utf8(first_step).unwrap());
    // git checkout .
}

//
// let dir = "./libwally-core";
// if !Path::new(&format!("{}/.git", dir)).exists() {
//     Command::new("git")
//         .args(&["submodule", "update", "--init", "--recursive"])
//         .status()
//         .unwrap();
//     Command::new("cd").arg(dir).status().unwrap();
//     Command::new("git")
//         .args(&["submodule", "sync", "--recursive"])
//         .status()
//         .unwrap();
//     Command::new("git")
//         .args(&["submodule", "update", "--init", "--recursive"])
//         .status()
//         .unwrap();
//     Command::new("cd").arg("--").status().unwrap();
// }
//
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
////////////////////////////////
// git submodule init && git submodule update &&
// cd proc_macros/impl_box_err_source_from_err && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/enum_extension && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/error_display && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/gen_enum && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/gen_enum_without_values && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/git_info && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_display && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_display_for_error_struct && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_display_for_simple_error_enum && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_error_with_tracing_for_struct_with_get_source_with_get_where_was && git pull && cd .. && cd .. &&
// cd proc_macros/impl_error_with_tracing_for_struct_with_get_source_without_get_where_was && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_error_with_tracing_for_struct_without_get_source && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_from_for_upper_struct && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_get_source_with_method && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_get_source_without_method && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_get_where_was_one_or_many_with_method && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_get_where_was_one_or_many_one_for_error_struct && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/init_error && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/init_from_env && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/init_from_env_with_panic_if_failed && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/provider_kind_from_config && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/struct_field_getter && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/struct_field_setter && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/svg_component && git checkout main && git pull && cd .. && cd .. &&
// cd tufa_client && git checkout main && git pull && cd .. &&
// cd tufa_common && git checkout main && git pull && cd .. &&
// cd tufa_server && git checkout main && git pull && cd .. &&
// cd tufa_telegram_bot && git checkout main && git pull && cd .. &&
// cd tufa_grpc_client && git checkout main && git pull && cd .. &&
// cd tufa_grpc_server && git checkout main && git pull && cd ..
////////////////////////////
// git submodule init && git submodule update &&
// cd proc_macros/impl_box_err_source_from_err && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/enum_extension && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/error_display && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/gen_enum && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/gen_enum_without_values && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/git_info && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_display && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_display_for_error_struct && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_display_for_simple_error_enum && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_error_with_tracing_for_struct_with_get_source_with_get_where_was && git pull && cd .. && cd .. &&
// cd proc_macros/impl_error_with_tracing_for_struct_with_get_source_without_get_where_was && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_error_with_tracing_for_struct_without_get_source && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_from_for_upper_struct && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_get_source_with_method && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_get_source_without_method && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_get_where_was_one_or_many_with_method && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/impl_get_where_was_one_or_many_one_for_error_struct && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/init_error && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/init_from_env && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/init_from_env_with_panic_if_failed && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/provider_kind_from_config && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/struct_field_getter && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/struct_field_setter && git checkout main && git pull && cd .. && cd .. &&
// cd proc_macros/svg_component && git checkout main && git pull && cd .. && cd .. &&
// cd tufa_client && git checkout main && git pull && cd .. &&
// cd tufa_common && git checkout main && git pull && cd .. &&
// cd tufa_server && git checkout main && git pull && cd .. &&
// cd tufa_telegram_bot && git checkout main && git pull && cd .. &&
// cd tufa_grpc_client && git checkout main && git pull && cd .. &&
// cd tufa_grpc_server && git checkout main && git pull && cd ..
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// Уже на «main»
// Ваша ветка обновлена в соответствии с «origin/main».
// Уже обновлено.
// M       README.md
// Уже на «main»
// Ваша ветка отстает от «origin/main» на 1 коммит и может быть перемотана вперед.
//   (используйте «git pull», чтобы обновить вашу локальную ветку)
// Обновление a23e16e..f2c1e88
// error: Ваши локальные изменения в указанных файлах будут перезаписаны при слиянии:
//         README.md
// Сделайте коммит или спрячьте ваши изменения перед слиянием веток.
// Прерываю
