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
    let path_stringified = format!("{file_name}.rs");
    let mut file = std::fs::File::create(std::path::Path::new(&path_stringified))
        .expect("bc64d2fb-f4ec-402f-b38d-30f9f2dabe12");
    std::io::Write::write_all(&mut file, &token_stream.to_string().into_bytes())
        .expect("37c4e1e7-1868-4662-b4d0-509470415bfc");
    if let FormatWithRustfmt::True = &format_with_rustfmt {
        let rustfmt = "rustfmt";
        let dash_dash_emit = "--emit";
        let files = "files";
        let status = std::process::Command::new(rustfmt)
            .arg(dash_dash_emit)
            .arg(files) //overwrite file in place
            .arg(&path_stringified)
            .status()
            .expect("1ce7c988-840f-4fef-be7f-9ebcc32d1cda");
        assert!(status.success(), "42e3ccb5-e0a7-4c13-8112-afc1f1da2519");
    }
}
