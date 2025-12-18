#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Copy)]
    enum RustOrClippy {
        Rust,
        Clippy,
    }
    impl RustOrClippy {
        fn name(&self) -> &str {
            match self {
                Self::Rust => "rust",
                Self::Clippy => "clippy",
            }
        }
    }
    fn lints_vec_from_cargo_toml(value: RustOrClippy) -> Vec<String> {
        let mut file = std::fs::File::open("../Cargo.toml").expect("error 39a0d238-d776-4b4e-ac2a-62f76a60f527");
        let mut contents = String::new();
        let _: usize = std::io::Read::read_to_string(&mut file, &mut contents).expect("error 2f5914f2-bff0-40d3-9948-07f2c562779b");
        let table = contents.parse::<toml::Table>().expect("error beb11586-c73d-4686-9ae2-a219f3a3ef4a");
        let workspace = table.get("workspace").expect("error f728192d-b3e6-470a-b304-8c58adabada5");
        let lints = workspace.get("lints").expect("error 82eaea37-3726-4e28-837f-a3063ff3d3fc");
        let toml_value_table = match lints.get(value.name()).expect("error dbd02f72-2647-4e41-a26f-a04cef447957") {
            toml::Value::Table(value) => value,
            toml::Value::String(_) | toml::Value::Integer(_) | toml::Value::Float(_) | toml::Value::Boolean(_) | toml::Value::Datetime(_) | toml::Value::Array(_) => panic!("not ok"),
        };
        toml_value_table.keys().cloned().collect::<Vec<String>>()
    }
    fn compare_lints_vec_from_cargo_toml_with_lints_to_check(rust_or_clippy: RustOrClippy, lints_vec_from_cargo_toml: Vec<String>, lints_to_check: Vec<String>, lints_not_in_cargo_toml_vec_exceptions: Vec<String>) {
        let rust_or_clippy_name = rust_or_clippy.name();
        let mut lints_not_in_cargo_toml = vec![];
        for element in &lints_to_check {
            if !lints_vec_from_cargo_toml.contains(element) {
                if lints_not_in_cargo_toml_vec_exceptions.contains(element) {
                    println!("todo!() {rust_or_clippy_name} {element} 158b5c43-05fa-4b8f-b6fe-9cda49d26997");
                } else {
                    lints_not_in_cargo_toml.push(element);
                }
            }
        }
        assert!(lints_not_in_cargo_toml.is_empty(), "this {rust_or_clippy_name} lints are not in the [workspace.lints.{rust_or_clippy_name}]: {lints_not_in_cargo_toml:#?}");
        let mut outdated_lints_in_file = vec![];
        for element in &lints_vec_from_cargo_toml {
            if !lints_to_check.contains(element) {
                outdated_lints_in_file.push(element);
            }
        }
        assert!(outdated_lints_in_file.is_empty(), "this {rust_or_clippy_name} lints are outdated but still in [workspace.lints.{rust_or_clippy_name}]: {outdated_lints_in_file:#?}");
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
        let rust_or_clippy = RustOrClippy::Rust;
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml(rust_or_clippy);
        let lints_from_command = {
            let output = std::process::Command::new("rustc").args(["-W", "help"]).stdout(std::process::Stdio::piped()).output().expect("error 7c939ff3-1c10-4188-afe8-36bb5c769ea2");
            assert!(output.status.success(), "error 0c000f24-afad-4397-88a4-913d0c113a34");
            {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(stderr.trim().is_empty(), "error 0a4b2082-a764-4080-9d72-61e009f03f27 {stderr}");
            };
            let stdout = String::from_utf8_lossy(&output.stdout);
            regex::Regex::new(r"(?m)^\s*([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b")
                .expect("error 60d99c87-273a-48ac-8daa-4f0a853d16bd")
                .captures_iter(&stdout)
                .map(|element| element[1].to_string().replace('-', "_").to_lowercase())
                .collect::<Vec<String>>()
        };
        compare_lints_vec_from_cargo_toml_with_lints_to_check(
            rust_or_clippy,
            lints_vec_from_cargo_toml,
            lints_from_command,
            //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
            vec![
                String::from("fuzzy_provenance_casts"),
                String::from("lossy_provenance_casts"),
                String::from("multiple_supertrait_upcastable"),
                String::from("must_not_suspend"),
                String::from("non_exhaustive_omitted_patterns"),
                String::from("supertrait_item_shadowing_definition"),
                String::from("supertrait_item_shadowing_usage"),
                String::from("aarch_64_softfloat_neon"),
                String::from("default_overrides_default_fields"),
                String::from("test_unstable_lint"),
                String::from("resolving_to_items_shadowing_supertrait_items"),
                String::from("shadowing_supertrait_items"),
                String::from("unqualified_local_imports"),//need to use some kind of defferent test flag or something for this
            ],
        );
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
        let rust_or_clippy = RustOrClippy::Clippy;
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml(rust_or_clippy);
        let clippy_lints_from_docs = {
            let document = scraper::Html::parse_document(&reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/index.html").expect("error d1a0544a-566e-4bf4-a37e-7dac73be02fd").text().expect("error 012e3328-53a4-4266-b403-24ac3b8dcbf3"));
            let html_selector = scraper::Selector::parse("html").expect("error 80427609-cfed-4b38-bdea-0794535ef84a");
            let body_selector = scraper::Selector::parse("body").expect("error 620c597c-0faa-408f-b9bc-29059d179951");
            let div_container_selector = scraper::Selector::parse(r#"div[class="container"]"#).expect("error eb483b13-e70e-40f4-b83a-3eeb00413d57");
            let article_selector = scraper::Selector::parse("article").expect("error d21dbe55-6f9f-4695-bf08-78da4f2424ea");
            let label_selector = scraper::Selector::parse("label").expect("error fe3d9f11-f3b0-4e54-a54a-842fabe3d8a7");
            let h2_lint_title_selector = scraper::Selector::parse(r#"h2[class="lint-title"]"#).expect("error f1473d4e-e26a-491d-9980-e1874301a6b2");
            let span_label_label_default_lint_group_group_deprecated_selector = scraper::Selector::parse(r#"span[class="label label-default lint-group group-deprecated"]"#).expect("error e86d5496-f62b-428c-ac6c-d533e0f6f775");
            let mut ids = Vec::new();
            for html_element in document.select(&html_selector) {
                for body_element in html_element.select(&body_selector) {
                    for div_container_element in body_element.select(&div_container_selector) {
                        for article_element in div_container_element.select(&article_selector) {
                            let mut is_deprecated = false;
                            for label_selector_element in article_element.select(&label_selector) {
                                if is_deprecated {
                                    break;
                                }
                                for h2_lint_title_selector_element in label_selector_element.select(&h2_lint_title_selector) {
                                    if is_deprecated {
                                        break;
                                    }
                                    if h2_lint_title_selector_element.select(&span_label_label_default_lint_group_group_deprecated_selector).next().is_some() {
                                        is_deprecated = true;
                                        break;
                                    }
                                }
                            }
                            if let Some(id) = article_element.value().attr("id")
                                && !is_deprecated
                            {
                                ids.push(id.to_owned());
                            }
                        }
                    }
                }
            }
            ids
        };
        compare_lints_vec_from_cargo_toml_with_lints_to_check(
            rust_or_clippy,
            lints_vec_from_cargo_toml,
            clippy_lints_from_docs,
            //todo on commit momment seems like this lints still not added to clippy, but in the list in clippy site
            vec![
                // String::from(""),
            ],
        );
    }
    #[test]
    fn check_dependencies_having_same_exact_version_in_the_project_and_lints_workspace_true() {
        fn get_cargo_toml_contents_recursive(path: &std::path::Path) -> Vec<String> {
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
                        let mut contents = String::new();
                        let _: usize = std::io::Read::read_to_string(&mut file, &mut contents).expect("error 38d0aeb2-eb33-447f-8843-af674b4eeabb");
                        acc.push(contents);
                    }
                }
            }
            acc
        }
        #[derive(Debug, serde::Deserialize)]
        struct Name {
            name: String,
        }
        #[derive(Debug, PartialEq, serde::Deserialize)]
        struct Lints {
            workspace: bool,
        }
        #[derive(Debug, serde::Deserialize)]
        struct CargoToml {
            package: Option<Name>,
            dependencies: Option<std::collections::HashMap<String, toml::Value>>,
            #[serde(rename = "dev-dependencies")]
            dev_dependencies: Option<std::collections::HashMap<String, toml::Value>>,
            #[serde(rename = "build-dependencies")]
            build_dependencies: Option<std::collections::HashMap<String, toml::Value>>,
            lints: Option<Lints>,
        }
        let cargo_toml_string_vec = get_cargo_toml_contents_recursive(std::path::Path::new(&"../"));
        let mut acc: Vec<(String, toml::Value)> = vec![];
        for cargo_toml_string in &cargo_toml_string_vec {
            let cargo_toml: CargoToml = toml::from_str(cargo_toml_string).expect("error db6c392c-1702-4aa0-a126-269c520e1dd0");
            //todo after fix issue with pg_jsonschema remove this check
            if let Some(package) = &cargo_toml.package
                && package.name != "pg_jsonschema"
            {
                assert!(cargo_toml.lints == Some(Lints { workspace: true }), "error 69f77fff-0b46-4c15-9c1b-7cb5fcb628bc");
                let mut handle_dependencies = |deps: Option<std::collections::HashMap<String, toml::Value>>| {
                    if let Some(value) = deps {
                        let mut keys = value.keys().clone().collect::<Vec<_>>();
                        keys.sort();
                        for key in keys {
                            let value = &value.get(key).expect("error c0b03ca9-80b3-444f-ab58-3522fb438c91");
                            if let toml::Value::Table(value) = value {
                                let mut handle_toml_value_string_valid_version = |version_value: &toml::Value| {
                                    if let toml::Value::String(value) = version_value {
                                        fn is_valid_version(value: &str) -> bool {
                                            let Some(version) = value.strip_prefix('=') else { return false };
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
                                                if part.parse::<u64>().is_err() {
                                                    return false;
                                                }
                                            }
                                            true
                                        }
                                        assert!(is_valid_version(value), "error 862fd6d2-cecb-4631-bcef-1043fb904153");
                                    } else {
                                        panic!("error dfc54bf8-f8ff-4e78-b40c-4045762cb50c");
                                    }
                                    {
                                        let mut is_found = false;
                                        for (acc_key, acc_version_value) in &acc {
                                            if key == acc_key {
                                                if version_value == acc_version_value {
                                                    is_found = true;
                                                } else {
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
                                    } else if value.get("path").is_some() {
                                    } else {
                                        panic!("error a2ac9215-0d83-428b-b572-355bd19f6211");
                                    }
                                } else if value.len() == 2 {
                                    if let Some(version_value) = value.get("version")
                                        && value.get("features").is_some()
                                    {
                                        handle_toml_value_string_valid_version(version_value);
                                    } else if value.get("path").is_some() && value.get("features").is_some() {
                                    } else {
                                        panic!("error 029bed67-2e36-4403-aead-9f415bdb20d9");
                                    }
                                } else if value.len() == 3 {
                                    if let Some(version_value) = value.get("version")
                                        && value.get("features").is_some()
                                        && value.get("default-features").is_some()
                                    {
                                        handle_toml_value_string_valid_version(version_value);
                                    } else {
                                        panic!("error 2233a0a2-c162-42ae-afb5-3b714d63612b");
                                    }
                                } else {
                                    panic!("error fc975adf-98f9-4f91-9e97-bff3245e06bb");
                                }
                            } else {
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
}
