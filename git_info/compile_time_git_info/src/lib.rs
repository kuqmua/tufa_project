#[proc_macro]
pub fn compile_time_project_git_info(
    _input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let path_to_git_modules = ".git";
    let path = if std::path::Path::new(&path_to_git_modules).is_dir() {
        path_to_git_modules
    } else {
        panic!("3e5265b6-7447-4785-85ad-2aaa2ebcf46d");
    };
    let file_name = "FETCH_HEAD";
    let full_path = format!("{path}/{file_name}");
    let file = std::fs::File::open(std::path::Path::new(&full_path))
        .expect("2075925d-64a5-4499-82c3-02651f8d8171");
    let mut buf_reader = std::io::BufReader::new(file);
    let mut git_logs_head_content = String::new();
    let _: usize = std::io::Read::read_to_string(&mut buf_reader, &mut git_logs_head_content)
        .expect("a1df8c05-a6e6-4882-b8ab-1aa0a1069019");
    let hash = git_logs_head_content
        .get(0..40)
        .expect("ceb73b63-9cec-4d4b-84ae-123a110485a7");
    //todo check if its a valid commit id.
    let commit_id_token_stream = format!("\"{hash}\"")
        .parse::<proc_macro2::TokenStream>()
        .expect("07edecef-6213-4aab-9491-449f5509108a");
    let generated = quote::quote! {
        ProjectGitInfo {
            commit: #commit_id_token_stream,
        }
    };
    generated.into()
}
