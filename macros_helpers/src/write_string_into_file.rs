pub fn write_string_into_file(file_name: &str, string_content: &std::primitive::str) {
    use std::io::Write;
    let path_stringified = format!("{file_name}.rs");
    let mut file = std::fs::File::create(std::path::Path::new(&path_stringified))
        .unwrap_or_else(|_| panic!("std::fs::File::create {path_stringified} failed"));
    file.write_all(&string_content.to_string().into_bytes())
        .unwrap_or_else(|_| panic!("file.write_all {path_stringified} failed"));
}
