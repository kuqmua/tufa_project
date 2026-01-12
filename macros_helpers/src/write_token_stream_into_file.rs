#[derive(Debug, Clone, Copy)]
pub enum FormatWithRustfmt {
    True,
    False,
}
pub fn write_token_stream_into_file(
    file_name: &str,
    token_stream: &proc_macro2::TokenStream,
    format_with_rustfmt: &FormatWithRustfmt,
) {
    let path_string = format!("{file_name}.rs");
    let path = std::path::Path::new(&path_string);
    {
        let mut file = std::fs::File::create(path).expect("create output file failed");
        std::io::Write::write_all(&mut file, token_stream.to_string().as_bytes())
            .expect("write token stream failed");
    };
    //no other way to format only one file. it formats all files in project
    if let FormatWithRustfmt::True = format_with_rustfmt {
        let status = std::process::Command::new("cargo")
            .arg("fmt")
            .arg("--")
            .arg(path)
            .status()
            .expect("failed to run cargo fmt");
        assert!(status.success(), "cargo fmt failed for {}", path.display());
    }
}
