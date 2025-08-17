#[proc_macro]
pub fn compile_time_project_git_info(_input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let path_to_git_modules = ".git";
    let path = if std::path::Path::new(&path_to_git_modules).is_dir() {
        path_to_git_modules
    } else {
        panic!("{path_to_git_modules} is not a dir");
    };
    let file_name = "FETCH_HEAD";
    let full_path = format!("{path}/{file_name}");
    let file = std::fs::File::open(std::path::Path::new(&full_path)).unwrap_or_else(|error| panic!("cannot open {full_path} file, error: \"{error}\""));
    let mut buf_reader = std::io::BufReader::new(file);
    let mut git_logs_head_content = std::string::String::new();
    let _: std::primitive::usize = std::io::Read::read_to_string(&mut buf_reader, &mut git_logs_head_content).unwrap_or_else(|error| panic!("cannot read_to_string from {full_path} file, error: \"{error}\""));
    let hash = git_logs_head_content.get(0..40).map_or_else(
        || {
            panic!("{full_path} file content length < 40");
        },
        |value| value,
    );
    //todo check if its a valid commit id.
    let commit_id_token_stream = format!("\"{hash}\"").parse::<proc_macro2::TokenStream>().expect("commit_id parse failed");
    let generated = quote::quote! {
        //crate::common::git::project_git_info::
        ProjectGitInfo {
            commit: #commit_id_token_stream,
        }
    };
    generated.into()
}
