#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Copy)]
    enum RustOrClippy {
        Rust,
        Clippy
    }
    impl RustOrClippy {
        fn name(&self) -> &std::primitive::str {
            match self {
                Self::Rust => "rust",
                Self::Clippy => "clippy"
            }
        }
    }
    fn lints_vec_from_cargo_toml(value: RustOrClippy) -> std::vec::Vec<std::string::String> {
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
        toml_value_table.keys().cloned().collect::<std::vec::Vec<std::string::String>>()
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml(RustOrClippy::Rust);
        let lints_from_command = {
            let output = std::process::Command::new("rustc")
                .args(["-W", "help"])
                .stdout(std::process::Stdio::piped())
                .output()
                .expect("error 7c939ff3-1c10-4188-afe8-36bb5c769ea2");
            assert!(output.status.success(), "error 0c000f24-afad-4397-88a4-913d0c113a34");
            {
                let stderr = std::string::String::from_utf8_lossy(&output.stderr);
                assert!(stderr.trim().is_empty(), "error 0a4b2082-a764-4080-9d72-61e009f03f27 {stderr}");
            };
            let stdout = std::string::String::from_utf8_lossy(&output.stdout);
            regex::Regex::new(r"(?m)^\s*([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b")
            .expect("error 60d99c87-273a-48ac-8daa-4f0a853d16bd")
            .captures_iter(&stdout)
            .map(|element| {
                let value = convert_case::Casing::to_case(
                    &element[1].to_string(),
                    convert_case::Case::Snake
                );
                if value == "invalid_from_utf_8" {
                    std::string::String::from("invalid_from_utf8")
                }
                else if value == "invalid_from_utf_8_unchecked" {
                    std::string::String::from("invalid_from_utf8_unchecked")
                }
                else {
                    value
                }
            })
            .collect::<std::vec::Vec<std::string::String>>()
        };
        let mut lints_not_in_file = vec![];
        for element in &lints_from_command {
            if !lints_vec_from_cargo_toml.contains(element) {
                //todo seems like this lints still not added to rustc, but in the list of rustc -W help
                if element == "fuzzy_provenance_casts" ||
                    element == "lossy_provenance_casts" ||
                    element == "multiple_supertrait_upcastable" ||
                    element == "must_not_suspend" ||
                    element == "non_exhaustive_omitted_patterns" ||
                    element == "supertrait_item_shadowing_definition" ||
                    element == "supertrait_item_shadowing_usage" ||
                    element == "aarch_64_softfloat_neon" ||
                    element == "default_overrides_default_fields" ||
                    element == "test_unstable_lint" ||
                    element == "resolving_to_items_shadowing_supertrait_items" ||
                    element == "shadowing_supertrait_items"
                {
                    println!("todo!() 158b5c43-05fa-4b8f-b6fe-9cda49d26997");
                }
                else {
                    lints_not_in_file.push(element);
                }
            }
        }
        assert!(lints_not_in_file.is_empty(), "this rust lints are not in the [workspace.lints.rust]: {lints_not_in_file:#?}");
        let mut outdated_lints_in_file = vec![];
        for element in &lints_vec_from_cargo_toml {
            if !lints_from_command.contains(element) {
                outdated_lints_in_file.push(element);
            }
        }
        assert!(outdated_lints_in_file.is_empty(), "this rust lints are outdated but still in [workspace.lints.rust]: {outdated_lints_in_file:#?}");
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml(RustOrClippy::Clippy);
        let body = reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/index.html").expect("error d1a0544a-566e-4bf4-a37e-7dac73be02fd").text().expect("error 012e3328-53a4-4266-b403-24ac3b8dcbf3");
        fn parse_article_ids_from_file(html: &std::primitive::str) -> std::vec::Vec<std::string::String> {
            let document = scraper::Html::parse_document(html);
            let html_selector = scraper::Selector::parse("html").expect("error 80427609-cfed-4b38-bdea-0794535ef84a");
            let body_selector = scraper::Selector::parse("body").expect("error 620c597c-0faa-408f-b9bc-29059d179951");
            let div_container_selector = scraper::Selector::parse(r#"div[class="container"]"#).expect("error eb483b13-e70e-40f4-b83a-3eeb00413d57");
            let article_selector = scraper::Selector::parse("article").expect("error d21dbe55-6f9f-4695-bf08-78da4f2424ea");
            let label_selector = scraper::Selector::parse("label").expect("error fe3d9f11-f3b0-4e54-a54a-842fabe3d8a7");
            let h2_lint_title_selector = scraper::Selector::parse(r#"h2[class="lint-title"]"#).expect("error f1473d4e-e26a-491d-9980-e1874301a6b2");
            let span_label_label_default_lint_group_group_deprecated_selector = scraper::Selector::parse(r#"span[class="label label-default lint-group group-deprecated"]"#).expect("error e86d5496-f62b-428c-ac6c-d533e0f6f775");
            let mut ids = std::vec::Vec::new();
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
                            if let Some(id) = article_element.value().attr("id") && !is_deprecated {
                                ids.push(id.to_string());
                            }
                        }
                    }
                }
            }
            ids
        }
        let clippy_lints_from_docs = parse_article_ids_from_file(&body);
        let mut lints_not_in_file = vec![];
        for element in &clippy_lints_from_docs {
            if !lints_vec_from_cargo_toml.contains(element) {
                //todo seems like this lints still not added to clippy, but in the list in clippy site
                if element == "decimal_bitwise_operands" ||
                    element == "manual_ilog2" ||
                    element == "ptr_offset_by_literal"
                {
                    println!("todo!() f5525a7e-a54f-44b2-8fb3-a28ec8ec8e77");
                }
                else {
                    lints_not_in_file.push(element);
                }
            }
        }
        assert!(lints_not_in_file.is_empty(), "this clippy lints are not in the [workspace.lints.clippy]: {lints_not_in_file:#?}");
        let mut outdated_lints_in_file = vec![];
        for element in &lints_vec_from_cargo_toml {
            if !clippy_lints_from_docs.contains(element) {
                outdated_lints_in_file.push(element);
            }
        }
        assert!(outdated_lints_in_file.is_empty(), "this clippy lints are outdated but still in [workspace.lints.clippy]: {outdated_lints_in_file:#?}");
    }
}