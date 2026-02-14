use std::{fs::File, io::Write, path::Path};

pub fn write_string_into_file(file_name: &str, string_content: &str) {
    let path_str = format!("{file_name}.rs");
    let mut file = File::create(Path::new(&path_str)).expect("dcb22948");
    Write::write_all(&mut file, &string_content.to_owned().into_bytes()).expect("9f430999");
}
