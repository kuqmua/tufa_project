#![deny(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#[proc_macro]
pub fn compile_time_git_info(repo_name: proc_macro::TokenStream) -> proc_macro::TokenStream {
    get_git_info(&repo_name.to_string(), "error_occurence_lib")
}

#[proc_macro]
pub fn compile_time_git_info_common(
    _input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    get_git_info("common", "error_occurence_lib")
}

fn get_git_info(repo_name: &str, path_to_git_into_start_source: &str) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let path_to_git_modules = format!(".git/modules/src/{repo_name}/");
    let path = if std::path::Path::new(&path_to_git_modules).is_dir() {
        path_to_git_modules
    } else {
        panic!("{path_to_git_modules} is not a dir");
    };
    let full_path = &format!("{}{}", path, "logs/HEAD");
    let file = std::fs::File::open(std::path::Path::new(full_path))
        .unwrap_or_else(|e| panic!("cannot open logs/HEAD file, error: \"{e}\""));
    let mut buf_reader = std::io::BufReader::new(file);
    let mut git_logs_head_content = std::string::String::new();
    std::io::Read::read_to_string(&mut buf_reader, &mut git_logs_head_content)
        .unwrap_or_else(|e| panic!("cannot read_to_string from HEAD file, error: \"{e}\""));
    let from_handle = "from ";
    let from_handle_index = git_logs_head_content
        .find(from_handle)
        .unwrap_or_else(|| panic!("no \"{from_handle}\" inside git_logs_head_content"));
    let git_extenstion_name = ".git";
    let dot_git_index = git_logs_head_content
        .find(git_extenstion_name)
        .unwrap_or_else(|| panic!("no \"{git_extenstion_name}\" inside git_logs_head_content"));
    let first_index = match from_handle_index.checked_add(from_handle.len()) {
        Some(index) => index,
        None => panic!("from_handle_index.checked_add(from_handle.len()) result int overflow"),
    };
    let repo_link_token_stream = git_logs_head_content
        .get(first_index..dot_git_index)
        .unwrap_or_else(|| panic!("failed to get slice from git_logs_head_content"))
        .to_string();
    let head_file_lines: Vec<&str> = git_logs_head_content.lines().collect::<Vec<&str>>();
    let last_head_file_line = head_file_lines
        .last()
        .expect("no last element inside git head file lines");
    let line_parts: Vec<&str> = last_head_file_line.split(' ').collect();
    let commit_id = line_parts
        .get(1)
        .unwrap_or_else(|| panic!("failed to get 1 element from line_parts as commit_id"))
        .to_string();
    let commit_id_replaced = commit_id.replace('"', "\\\""); //bad, bad decision
    let commit_id_token_stream = format!("\"{commit_id_replaced}\"")
        .parse::<proc_macro2::TokenStream>()
        .expect("commit_id parse failed");
    // let author = line_parts
    //     .get(2)
    //     .unwrap_or_else(|| panic!("failed to get 2 element from line_parts as author"))
    //     .to_string();
    // let author_replaced = author.replace('"', "\\\""); //bad, bad decision
    // let author_token_stream = format!("\"{author_replaced}\"")
    //     .parse::<proc_macro2::TokenStream>()
    //     .expect("author parse failed");
    // let unhandled_author_email = line_parts
    //     .get(3)
    //     .unwrap_or_else(|| {
    //         panic!("failed to get 3 element from line_parts as slice for author_email")
    //     })
    //     .to_string();
    // let author_email = unhandled_author_email
    //     .get(1..unhandled_author_email.len() - 1)
    //     .unwrap_or_else(|| panic!("failed to get slice from line_parts as author_email"))
    //     .to_string();
    // let author_email_replaced = author_email.replace('"', "\\\""); //bad, bad decision
    // let author_email_token_stream = format!("\"{author_email_replaced}\"")
    //     .parse::<proc_macro2::TokenStream>()
    //     .expect("author_email parse failed");
    // let commit_unix_time = line_parts
    //     .get(4)
    //     .unwrap_or_else(|| panic!("failed to get 4 element from line_parts as commit_unix_time"))
    //     .to_string();
    // let commit_unix_time_replaced = commit_unix_time.replace('"', "\\\""); //bad, bad decision
    // let commit_unix_time_token_stream = format!("\"{commit_unix_time_replaced}\"")
    //     .parse::<proc_macro2::TokenStream>()
    //     .expect("path parse failed");
    // let commit_unix_time_index = last_head_file_line
    //     .find(&commit_unix_time)
    //     .unwrap_or_else(|| {
    //         panic!(
    //             "cannot find \"{commit_unix_time}\" for the second time inside {git_logs_head_content}"
    //         )
    //     });
    // let part_after_commit_unix_time = last_head_file_line
    //     .get(commit_unix_time_index + commit_unix_time.len() + 1..)
    //     .unwrap_or_else(|| {
    //         panic!("failed to get slice from last_head_file_line as part_after_commit_unix_time")
    //     })
    //     .to_string();
    // let backslash_t = "\t";
    // let backslash_t_index = part_after_commit_unix_time
    //     .find(backslash_t)
    //     .unwrap_or_else(|| panic!("no \"{backslash_t}\" inside \"{part_after_commit_unix_time}\""));
    // let timezone = part_after_commit_unix_time
    //     .get(..backslash_t_index)
    //     .unwrap_or_else(|| {
    //         panic!("failed to get slice from part_after_commit_unix_time as timezone")
    //     })
    //     .to_string();
    // let timezone_replaced = timezone.replace('"', "\\\""); //bad, bad decision
    // let timezone_token_stream = format!("\"{timezone_replaced}\"")
    //     .parse::<proc_macro2::TokenStream>()
    //     .expect("path parse failed");
    // let message = part_after_commit_unix_time
    //     .get(backslash_t_index + 1..)
    //     .unwrap_or_else(|| {
    //         panic!("failed to get slice from part_after_commit_unix_time as message")
    //     });
    // let message_replaced = message.replace('"', "\\\""); //bad, bad decision
    // let message_token_stream = format!("\"{message_replaced}\"")
    //     .parse::<proc_macro2::TokenStream>()
    //     .unwrap_or_else(|_| {
    //         panic!("failed to parse message_token_stream");
    //     });
    let path_to_git_info_token_stream =
        format!("{path_to_git_into_start_source}::git_info::GitInfo")
            .parse::<proc_macro2::TokenStream>()
            .expect("path_to_git_info parse failed");
    // println!("{repo_link_token_stream}");
    let gen = quote::quote! {
        #path_to_git_info_token_stream {
            git_commit_id: #commit_id_token_stream ,
            git_repo_link: #repo_link_token_stream ,
            // git_author: #author_token_stream ,
            // git_author_email: #author_email_token_stream ,
            // git_commit_unix_time: #commit_unix_time_token_stream ,
            // git_timezone: #timezone_token_stream ,
            // git_message: #message_token_stream ,
        }
    };
    gen.into()
}

#[proc_macro]
pub fn compile_time_project_git_info(
    _input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_common::panic_location::panic_location();
    let path_to_git_modules = ".git";
    let path = if std::path::Path::new(&path_to_git_modules).is_dir() {
        path_to_git_modules
    } else {
        panic!("{path_to_git_modules} is not a dir");
    };
    let file_name = "FETCH_HEAD";
    let full_path = format!("{path}/{file_name}");
    let file = std::fs::File::open(std::path::Path::new(&full_path))
        .unwrap_or_else(|e| panic!("cannot open {full_path} file, error: \"{e}\""));
    let mut buf_reader = std::io::BufReader::new(file);
    let mut git_logs_head_content = std::string::String::new();
    std::io::Read::read_to_string(&mut buf_reader, &mut git_logs_head_content)
        .unwrap_or_else(|e| panic!("cannot read_to_string from {full_path} file, error: \"{e}\""));
    let hash = match git_logs_head_content.get(0..40) {
        Some(hash) => hash,
        None => panic!("{full_path} file content length < 40"),
    };
    //todo check if its a valid commit id.
    let commit_id_token_stream = format!("\"{hash}\"")
        .parse::<proc_macro2::TokenStream>()
        .expect("commit_id parse failed");
    let gen = quote::quote! {
        crate::common::git::project_git_info::ProjectGitInfo {
            project_commit: #commit_id_token_stream,
        }
    };
    gen.into()
}
