#[proc_macro]
pub fn compile_time_project_git_info(
    _input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("d6b0f81a-1fc1-42fe-a20c-5e3cfd8a6403");
    assert!(
        output.status.success(),
        "f7185b72-c92e-4558-8615-1ae11bcaa131"
    );
    let hash = String::from_utf8(output.stdout)
        .expect("9a3f659d-aca1-47d7-be32-cc3ba06e1b01")
        .trim()
        .to_owned();
    // Validate SHA-1 (40 hex chars)
    assert!(
        hash.len() == 40 && hash.chars().all(|element_e7daeee7| element_e7daeee7.is_ascii_hexdigit()),
        "093516ae-a89f-42df-8b01-9b2897111705"
    );
    let commit_id_token_stream = format!("\"{hash}\"")
        .parse::<proc_macro2::TokenStream>()
        .expect("842e75e8-1c25-44af-bb71-f15ee1c0c67d");
    let generated = quote::quote! {
        ProjectGitInfo {
            commit: #commit_id_token_stream,
        }
    };
    generated.into()
}
