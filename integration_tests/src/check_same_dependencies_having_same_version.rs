#[test]
fn check_same_dependencies_having_same_version() {
    let path = std::path::Path::new(&"../");
    fn get_cargo_toml_contents_recursive(path: &std::path::Path) -> std::vec::Vec<std::string::String> {
        let mut acc = vec![];
        if path.is_dir() {
            for entry in std::fs::read_dir(path).expect("error 81837dea-c20f-469a-b365-528f0b9f50a4") {
                let entry = entry.expect("error bb7ee3cf-9f34-4d81-9160-496f7ca5e43b");
                let path = entry.path();
                if path.is_dir() {
                    for element in get_cargo_toml_contents_recursive(&path) {
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
    let cargo_toml_string_vec = get_cargo_toml_contents_recursive(path);
    println!("{}", cargo_toml_string_vec.len());
    #[derive(Debug, serde::Deserialize)]
    struct Name {
        name: std::string::String,
    }
    #[derive(Debug, serde::Deserialize)]
    struct CargoToml {
        package: std::option::Option<Name>,
        dependencies: std::option::Option<std::collections::HashMap<std::string::String, toml::Value>>,
        #[serde(rename = "dev-dependencies")]
        dev_dependencies: std::option::Option<std::collections::HashMap<std::string::String, toml::Value>>,
    }
    fn is_valid_version(value: &std::primitive::str) -> std::primitive::bool {
        let version = match value.strip_prefix('=') {
            Some(value) => value,
            None => return false,
        };
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }
        // parse each part as u64 (ensures it's a valid unsigned number)
        for part in parts {
            if part.is_empty() {
                return false; // prevents "=1..2"
            }
            if part.starts_with('0') && part != "0" {
                return false; // optional: forbid leading zeros
            }
            if part.parse::<std::primitive::u64>().is_err() {
                return false;
            }
        }
        true
    }
    for cargo_toml_string in cargo_toml_string_vec {
        println!("{cargo_toml_string}");
        let cargo_toml: CargoToml = toml::from_str(&cargo_toml_string).expect("error db6c392c-1702-4aa0-a126-269c520e1dd0");
        //todo dev-dependencies
        let handle_dependencies = |deps: std::option::Option<std::collections::HashMap<std::string::String, toml::Value>>|{
            if let Some(value) = deps {
                //todo after fix issue with pg_jsonschema remove this check
                if let Some(package) = &cargo_toml.package {
                    if package.name != "pg_jsonschema" {
                        for (key, value) in value {
                            if let toml::Value::Table(value) = value {
                                if value.len() == 1 {
                                    if let Some(value) = value.get("version") {
                                        //todo reuse
                                        if let toml::Value::String(value) = value {
                                            if !is_valid_version(value) {
                                                panic!("error 862fd6d2-cecb-4631-bcef-1043fb904153");
                                            }
                                        }
                                        else {
                                            panic!("error dfc54bf8-f8ff-4e78-b40c-4045762cb50c");
                                        }
                                    }
                                    else if let Some(value) = value.get("path") {}
                                    else {
                                        panic!("error a2ac9215-0d83-428b-b572-355bd19f6211");
                                    }
                                }
                                else if value.len() == 2 {
                                    println!("package.name {}, key {key}, {value:#?}", package.name);
                                    if let Some(version_value) = value.get("version") && value.get("features").is_some() {
                                        if let toml::Value::String(value) = version_value {
                                            if !is_valid_version(value) {
                                                panic!("error 843946eb-249c-4fcf-aa69-7536324c5811");
                                            }
                                        }
                                        else {
                                            panic!("error 1acc7c7c-a355-42f2-a895-d2c863fb448d");
                                        }
                                    }
                                    else if let Some(path_value) = value.get("path") && value.get("features").is_some() {}
                                    else {
                                        panic!("error 029bed67-2e36-4403-aead-9f415bdb20d9");
                                    }
                                }
                                else if value.len() == 3 {

                                }
                                else {
                                    panic!("error fc975adf-98f9-4f91-9e97-bff3245e06bb");
                                }
                            }
                            else {
                                println!("###{value:#?}");
                                panic!("error c61368cc-42b8-4de4-9df2-3bb30cc2ad79");
                            }
                        }
                    }
                }
            }
        };
        handle_dependencies(cargo_toml.dependencies);
        handle_dependencies(cargo_toml.dev_dependencies);
    }
}
