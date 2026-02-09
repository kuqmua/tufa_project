use std::{fs::File, io::Write, path::Path};

pub fn write_string_into_file(file_name: &str, string_content: &str) {
    let path_stringified = format!("{file_name}.rs");
    let mut file =
        File::create(Path::new(&path_stringified)).expect("dcb22948-7ee4-4b9c-9b7a-ddf3d83dd217");
    Write::write_all(&mut file, &string_content.to_owned().into_bytes())
        .expect("9f430999-4159-48e0-8324-147fbbcbd772");
}
