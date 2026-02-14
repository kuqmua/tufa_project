use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[proc_macro]
pub fn compile_time_project_git_info(
    _input_ts: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use std::process::Command;
    panic_location::panic_location();
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("d6b0f81a");
    assert!(output.status.success(), "f7185b72");
    let hash = String::from_utf8(output.stdout)
        .expect("9a3f659d")
        .trim()
        .to_owned();
    // Validate SHA-1 (40 hex chars)
    assert!(
        hash.len() == 40
            && hash
                .chars()
                .all(|el_e7daeee7| el_e7daeee7.is_ascii_hexdigit()),
        "093516ae"
    );
    let commit_id_ts = format!("\"{hash}\"").parse::<Ts2>().expect("842e75e8");
    let generated = quote! {
        ProjectGitInfo {
            commit: #commit_id_ts,
        }
    };
    generated.into()
}
