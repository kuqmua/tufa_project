pub enum FormatWithRustfmt {
    True,
    False
}
pub fn write_token_stream_into_file(file_name: &str, token_stream: &proc_macro2::TokenStream, format_with_rustfmt: &FormatWithRustfmt) {
    use std::io::Write;
    let path_stringified = format!("{file_name}.rs");
    let mut file = std::fs::File::create(std::path::Path::new(&path_stringified)).unwrap_or_else(|error| panic!("error 7a495d4e-4437-45f1-add1-99028841f6e1, std::fs::File::create {path_stringified} failed {error:#?}"));
    file.write_all(&token_stream.to_string().into_bytes()).unwrap_or_else(|error| panic!("error f452a294-85b6-4488-b1cc-03093ec0cbfe, file.write_all {path_stringified} failed {error:#?}"));
    if let FormatWithRustfmt::True = &format_with_rustfmt {
        let rustfmt = "rustfmt";
        let dash_dash_emit = "--emit";
        let files = "files";
        let command = format!("'{rustfmt} {dash_dash_emit} {files}'");
        let status = std::process::Command::new(rustfmt)
            .arg(dash_dash_emit)
            .arg(files) //overwrite file in place
            .arg(&path_stringified)
            .status().expect("error 741f1cce-76db-4529-b25e-5d00078f952a, cannot get execution status of {command}");
        if !status.success() {
            panic!("error f59653b5-5a66-4626-a75d-ef38d6fd8e29, execution of {command} status is not success: {status:#?}")
        }
    }
}
