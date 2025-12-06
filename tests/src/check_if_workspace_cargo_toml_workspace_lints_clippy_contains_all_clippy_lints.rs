#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    let mut file = std::fs::File::open("../Cargo.toml").expect("error 39a0d238-d776-4b4e-ac2a-62f76a60f527");
    let mut contents = String::new();
    let _: usize = std::io::Read::read_to_string(&mut file, &mut contents).expect("error 2f5914f2-bff0-40d3-9948-07f2c562779b");
    let value = contents.parse::<toml::Table>().expect("error beb11586-c73d-4686-9ae2-a219f3a3ef4a");
    let workspace = &value.get("workspace").expect("error f728192d-b3e6-470a-b304-8c58adabada5");
    let lints = &workspace.get("lints").expect("error 82eaea37-3726-4e28-837f-a3063ff3d3fc");
    let clippy = &lints.get("clippy").expect("error dbd02f72-2647-4e41-a26f-a04cef447957");
    let toml_value_table = match clippy {
        toml::Value::Table(value) => value,
        toml::Value::String(_) | toml::Value::Integer(_) | toml::Value::Float(_) | toml::Value::Boolean(_) | toml::Value::Datetime(_) | toml::Value::Array(_) => panic!("not ok"),
    };
    let lints_vec_from_file = toml_value_table.keys().collect::<std::vec::Vec<&std::string::String>>();
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
        if !lints_vec_from_file.contains(&element) {
            if element == "decimal_bitwise_operands" || element == "manual_ilog2" || element == "ptr_offset_by_literal" {
                println!("todo!()");
            }
            else {
                lints_not_in_file.push(element);
            }
        }
    }
    assert!(lints_not_in_file.is_empty(), "this clippy lints are not in the [workspace.lints.clippy]: {lints_not_in_file:#?}");
    let mut outdated_lints_in_file = vec![];
    for element in &lints_vec_from_file {
        if !clippy_lints_from_docs.contains(element) {
            outdated_lints_in_file.push(element);
        }
    }
    assert!(outdated_lints_in_file.is_empty(), "this clippy lints are outdated but still in [workspace.lints.clippy]: {outdated_lints_in_file:#?}");
}
