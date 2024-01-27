pub fn write_token_stream_into_file(
    file_name: &str,
    token_stream: &proc_macro2::TokenStream,
    proc_macro_name_ident_stringified: &str
) {
    use std::io::Write;
    let path_stringified = format!("{file_name}.rs");
    let mut file = std::fs::File::create(std::path::Path::new(&path_stringified))
    .unwrap_or_else(|_| 
        panic!("{proc_macro_name_ident_stringified} std::fs::File::create {path_stringified} failed")
    );
    file.write_all(&token_stream.to_string().into_bytes()).unwrap_or_else(|_| 
        panic!("{proc_macro_name_ident_stringified} file.write_all {path_stringified} failed")
    );
}