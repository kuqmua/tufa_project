#[derive(Debug, Clone, Copy)]
pub enum FormatWithCargofmt {
    True,
    False,
}
//todo move it into maybe_write_token_stream_into_file
pub fn write_token_stream_into_file(
    file_name: &str,
    token_stream: &proc_macro2::TokenStream,
    format_with_cargofmt: &FormatWithCargofmt,
) {
    let path_string = format!("{file_name}.rs");
    let path = std::path::Path::new(&path_string);
    {
        let mut file = std::fs::File::create(path).expect("create output file failed");
        std::io::Write::write_all(&mut file, token_stream.to_string().as_bytes())
            .expect("write token stream failed");
    };
    //no other way to format only one file. it formats all files in project
    if let FormatWithCargofmt::True = format_with_cargofmt {
        let status = std::process::Command::new("cargo")
            .arg("fmt")
            .arg("--")
            .arg(path)
            .status()
            .expect("failed to run cargo fmt");
        assert!(status.success(), "cargo fmt failed for {}", path.display());
    }
}
#[derive(Debug, Copy, Clone, serde::Deserialize)]
pub enum ShouldWriteTokenStreamIntoFile {
    True,
    False,
}
pub fn maybe_write_token_stream_into_file(
    should_write_token_stream_into_file: ShouldWriteTokenStreamIntoFile,
    file_name: &str,
    token_stream: &proc_macro2::TokenStream,
    format_with_cargofmt: &FormatWithCargofmt,
) {
    if let ShouldWriteTokenStreamIntoFile::True = should_write_token_stream_into_file {
        write_token_stream_into_file(file_name, token_stream, format_with_cargofmt);
    }
}
