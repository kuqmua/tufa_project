#[test]
fn check_dependencies_having_same_exact_version_in_the_project_and_lints_workspace_true() {
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
                }
                if path.is_file() && path.file_name().expect("error 9f17bfed-4612-4644-8551-f4547874ff16") == "Cargo.toml" {
                    let mut file = std::fs::File::open(&path).expect("error d211d2ff-8217-4270-b4e6-8a718a140363");
                    let mut contents = std::string::String::new();
                    let _: std::primitive::usize = std::io::Read::read_to_string(&mut file, &mut contents).expect("error 38d0aeb2-eb33-447f-8843-af674b4eeabb");
                    acc.push(contents);
                }
            }
        }
        acc
    }
    let cargo_toml_string_vec = get_cargo_toml_contents_recursive(path);
    #[derive(Debug, serde::Deserialize)]
    struct Name {
        name: std::string::String,
    }
    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct Lints {
        workspace: std::primitive::bool,
    }
    #[derive(Debug, serde::Deserialize)]
    struct CargoToml {
        package: std::option::Option<Name>,
        dependencies: std::option::Option<std::collections::HashMap<std::string::String, toml::Value>>,
        #[serde(rename = "dev-dependencies")]
        dev_dependencies: std::option::Option<std::collections::HashMap<std::string::String, toml::Value>>,
        #[serde(rename = "build-dependencies")]
        build_dependencies: std::option::Option<std::collections::HashMap<std::string::String, toml::Value>>,
        lints: std::option::Option<Lints>,
    }
    let mut acc: std::vec::Vec<(std::string::String, toml::Value)> = vec![];
    for cargo_toml_string in &cargo_toml_string_vec {
        let cargo_toml: CargoToml = toml::from_str(cargo_toml_string).expect("error db6c392c-1702-4aa0-a126-269c520e1dd0");
        //todo after fix issue with pg_jsonschema remove this check
        if let Some(package) = &cargo_toml.package && package.name != "pg_jsonschema" {
            assert!(cargo_toml.lints == Some(Lints {workspace: true}), "error 69f77fff-0b46-4c15-9c1b-7cb5fcb628bc");
            let mut handle_dependencies = |deps: std::option::Option<std::collections::HashMap<std::string::String, toml::Value>>|{
                if let Some(value) = deps {
                    let mut keys = value.keys().clone().collect::<std::vec::Vec<_>>();
                    keys.sort();
                    for key in keys {
                        let value = &value.get(key).expect("error c0b03ca9-80b3-444f-ab58-3522fb438c91");
                        if let toml::Value::Table(value) = value {
                            let mut handle_toml_value_string_valid_version = |version_value: &toml::Value|{
                                if let toml::Value::String(value) = version_value {
                                    fn is_valid_version(value: &std::primitive::str) -> std::primitive::bool {
                                        let version = match value.strip_prefix('=') {
                                            Some(value) => value,
                                            None => return false,
                                        };
                                        let parts: Vec<&str> = version.split('.').collect();
                                        if parts.len() != 3 {
                                            return false;
                                        }
                                        //// parse each part as u64 (ensures it's a valid unsigned number)
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
                                    assert!(is_valid_version(value), "error 862fd6d2-cecb-4631-bcef-1043fb904153");
                                }
                                else {
                                    panic!("error dfc54bf8-f8ff-4e78-b40c-4045762cb50c");
                                }
                                {
                                    let mut is_found = false;
                                    for (acc_key, acc_version_value) in &acc {
                                        if key == acc_key {
                                            if version_value == acc_version_value {
                                                is_found = true;
                                            }
                                            else {
                                                panic!("error 1defaf02-9db9-4432-9bc1-654dde6209c0");
                                            }
                                        }
                                    }
                                    if !is_found {
                                        acc.push((key.clone(), version_value.clone()));
                                    }
                                }
                            };
                            if value.len() == 1 {
                                if let Some(version_value) = value.get("version") {
                                    handle_toml_value_string_valid_version(version_value);
                                }
                                else if value.get("path").is_some() {}
                                else {
                                    panic!("error a2ac9215-0d83-428b-b572-355bd19f6211");
                                }
                            }
                            else if value.len() == 2 {
                                if let Some(version_value) = value.get("version") && value.get("features").is_some() {
                                    handle_toml_value_string_valid_version(version_value);
                                }
                                else if value.get("path").is_some() && value.get("features").is_some() {}
                                else {
                                    panic!("error 029bed67-2e36-4403-aead-9f415bdb20d9");
                                }
                            }
                            else if value.len() == 3 {
                                if let Some(version_value) = value.get("version") && value.get("features").is_some() && value.get("default-features").is_some() {
                                    handle_toml_value_string_valid_version(version_value);
                                }
                                else {
                                    panic!("error 2233a0a2-c162-42ae-afb5-3b714d63612b");
                                }
                            }
                            else {
                                panic!("error fc975adf-98f9-4f91-9e97-bff3245e06bb");
                            }
                        }
                        else {
                            panic!("error c61368cc-42b8-4de4-9df2-3bb30cc2ad79");
                        }
                    }
                }
            };
            handle_dependencies(cargo_toml.dependencies);
            handle_dependencies(cargo_toml.dev_dependencies);
            handle_dependencies(cargo_toml.build_dependencies);
        }
    }
}
