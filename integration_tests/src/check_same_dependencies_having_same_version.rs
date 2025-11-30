#[test]
fn check_same_dependencies_having_same_version() {
    let path = std::path::Path::new(&"../");
    fn print_cargo_toml_recursive(path: &std::path::Path) -> std::vec::Vec<std::string::String> {
        let mut acc = vec![];
        if path.is_dir() {
            for entry in std::fs::read_dir(path).expect("error 81837dea-c20f-469a-b365-528f0b9f50a4") {
                let entry = entry.expect("error bb7ee3cf-9f34-4d81-9160-496f7ca5e43b");
                let path = entry.path();
                if path.is_dir() {
                    for element in print_cargo_toml_recursive(&path) {
                        acc.push(element);
                    }
                } else if path.is_file() && path.file_name().expect("error 9f17bfed-4612-4644-8551-f4547874ff16") == "Cargo.toml" {
                    let mut file = std::fs::File::open(&path).expect("error d211d2ff-8217-4270-b4e6-8a718a140363");
                    let mut contents = std::string::String::new();
                    let _ = std::io::Read::read_to_string(&mut file, &mut contents).expect("error 38d0aeb2-eb33-447f-8843-af674b4eeabb");
                    acc.push(contents);
                }
            }
        }
        acc
    }
    let acc = print_cargo_toml_recursive(path);
    println!("{}", acc.len());
}
