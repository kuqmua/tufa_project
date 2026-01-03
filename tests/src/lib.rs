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
    fn toml_value_from_from_cargo_toml_workspace() -> toml::Value {
        let mut file =
            std::fs::File::open("../Cargo.toml").expect("39a0d238-d776-4b4e-ac2a-62f76a60f527");
        let mut contents = String::new();
        let _: usize = std::io::Read::read_to_string(&mut file, &mut contents)
            .expect("2f5914f2-bff0-40d3-9948-07f2c562779b");
        let table = contents
            .parse::<toml::Table>()
            .expect("beb11586-c73d-4686-9ae2-a219f3a3ef4a");
        table
            .get("workspace")
            .expect("f728192d-b3e6-470a-b304-8c58adabada5")
            .clone()
    }
    fn lints_vec_from_cargo_toml_workspace(rust_or_clippy: RustOrClippy) -> Vec<String> {
        let workspace = toml_value_from_from_cargo_toml_workspace();
        let lints = workspace
            .get("lints")
            .expect("82eaea37-3726-4e28-837f-a3063ff3d3fc");
        let toml_value_table = match lints
            .get(rust_or_clippy.name())
            .expect("dbd02f72-2647-4e41-a26f-a04cef447957")
        {
            toml::Value::Table(value) => value,
            toml::Value::String(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => panic!("cae226cd-1876-4aa2-a46e-8de0698d15bb"),
        };
        toml_value_table.keys().cloned().collect::<Vec<String>>()
    }
    fn compare_lints_vecs(
        rust_or_clippy: RustOrClippy,
        lints_vec_from_cargo_toml: Vec<String>,
        lints_to_check: Vec<String>,
        lints_not_in_cargo_toml_vec_exceptions: Vec<String>,
    ) {
        let rust_or_clippy_name = rust_or_clippy.name();
        let mut lints_not_in_cargo_toml = Vec::new();
        for element in &lints_to_check {
            if !lints_vec_from_cargo_toml.contains(element) {
                if lints_not_in_cargo_toml_vec_exceptions.contains(element) {
                    println!(
                        "todo!() {rust_or_clippy_name} {element} 158b5c43-05fa-4b8f-b6fe-9cda49d26997"
                    );
                } else {
                    lints_not_in_cargo_toml.push(element);
                }
            }
        }
        assert!(
            lints_not_in_cargo_toml.is_empty(),
            "d2b7ba9f-d133-496c-a29d-67503c3d9e8a"
        );
        let mut outdated_lints_in_file = Vec::new();
        for element in &lints_vec_from_cargo_toml {
            if !lints_to_check.contains(element) {
                outdated_lints_in_file.push(element);
            }
        }
        assert!(
            outdated_lints_in_file.is_empty(),
            "93787d2d-47b8-4f26-ba5a-341d3c60ca15"
        );
    }
    #[derive(Debug, Clone, Copy)]
    enum ExpectOrPanic {
        Expect,
        Panic,
    }
    fn check_expect_or_panic_contains_only_unique_uuid_v4(expect_or_panic: ExpectOrPanic) {
        struct ExpectVisitor {
            expect_or_panic: ExpectOrPanic,
            uuids: Vec<String>,
            errors: Vec<String>,
        }
        impl<'ast> syn::visit::Visit<'ast> for ExpectVisitor {
            fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
                let expect_or_panic_str = match self.expect_or_panic {
                    ExpectOrPanic::Expect => "expect",
                    ExpectOrPanic::Panic => "panic",
                };
                if i.method == expect_or_panic_str {
                    if i.args.len() == 1 {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = &i.args.get(0).expect("d5ad7bff-2125-4fe2-a132-d7f6446a1710")
                        {
                            let value = lit_str.value();
                            match uuid::Uuid::parse_str(&value) {
                                Ok(uuid) if uuid.get_version() == Some(uuid::Version::Random) => {
                                    self.uuids.push(value);
                                }
                                _ => {
                                    self.errors
                                        .push(format!("arg is not valid UUID v4: {value}"));
                                }
                            }
                        } else {
                            self.errors.push("arg is not string literal".to_owned());
                        }
                    } else {
                        self.errors.push("with != 1 arg".to_owned());
                    }
                }
                syn::visit::visit_expr_method_call(self, i);
            }
        }
        let mut all_uuids = Vec::new();
        let mut all_errors = Vec::new();
        for entry in project_directory()
            .into_iter()
            .filter_entry(|element| element.file_name() != "target")
            .filter_map(Result::ok)
            .filter(|element| {
                element
                    .path()
                    .extension()
                    .and_then(|current_element| current_element.to_str())
                    == Some("rs")
            })
        {
            let Ok(content) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            let ast = syn::parse_file(&content).expect("5e7a83eb-2556-47b7-8677-66f8612242ad");
            let mut visitor = ExpectVisitor {
                expect_or_panic,
                uuids: Vec::new(),
                errors: Vec::new(),
            };
            syn::visit::Visit::visit_file(&mut visitor, &ast);
            all_uuids.extend(visitor.uuids);
            all_errors.extend(
                visitor
                    .errors
                    .into_iter()
                    .map(|element| format!("{:?}: {}", entry.path(), element)),
            );
        }
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();
        for uuid in all_uuids {
            if !seen.insert(uuid.clone()) {
                duplicates.push(uuid);
            }
        }
        if !duplicates.is_empty() {
            all_errors.push(format!("duplicate UUIDs found: {duplicates:?}"));
        }
        assert!(
            all_errors.is_empty(),
            "52eb15f1-7b88-4dfa-869b-a7b6f241df08",
        );
    }
    fn project_directory() -> walkdir::WalkDir {
        walkdir::WalkDir::new("../")
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
        let rust_or_clippy = RustOrClippy::Rust;
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml_workspace(rust_or_clippy);
        let lints_from_command = {
            let output = std::process::Command::new("rustc")
                .args(["-W", "help"])
                .stdout(std::process::Stdio::piped())
                .output()
                .expect("7c939ff3-1c10-4188-afe8-36bb5c769ea2");
            assert!(
                output.status.success(),
                "0c000f24-afad-4397-88a4-913d0c113a34"
            );
            {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(
                    stderr.trim().is_empty(),
                    "0a4b2082-a764-4080-9d72-61e009f03f27"
                );
            };
            let stdout = String::from_utf8_lossy(&output.stdout);
            regex::Regex::new(r"(?m)^\s*([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b")
                .expect("60d99c87-273a-48ac-8daa-4f0a853d16bd")
                .captures_iter(&stdout)
                .map(|element| element[1].to_string().replace('-', "_").to_lowercase())
                .collect::<Vec<String>>()
        };
        compare_lints_vecs(
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
                String::from("unqualified_local_imports"), //need to use some kind of defferent test flag or something for this
            ],
        );
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
        let rust_or_clippy = RustOrClippy::Clippy;
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml_workspace(rust_or_clippy);
        let clippy_lints_from_docs = {
            let document = scraper::Html::parse_document(
                &reqwest::blocking::get(
                    "https://rust-lang.github.io/rust-clippy/master/index.html",
                )
                .expect("d1a0544a-566e-4bf4-a37e-7dac73be02fd")
                .text()
                .expect("012e3328-53a4-4266-b403-24ac3b8dcbf3"),
            );
            let html_selector =
                scraper::Selector::parse("html").expect("80427609-cfed-4b38-bdea-0794535ef84a");
            let body_selector =
                scraper::Selector::parse("body").expect("620c597c-0faa-408f-b9bc-29059d179951");
            let div_container_selector = scraper::Selector::parse(r#"div[class="container"]"#)
                .expect("eb483b13-e70e-40f4-b83a-3eeb00413d57");
            let article_selector =
                scraper::Selector::parse("article").expect("d21dbe55-6f9f-4695-bf08-78da4f2424ea");
            let label_selector =
                scraper::Selector::parse("label").expect("fe3d9f11-f3b0-4e54-a54a-842fabe3d8a7");
            let h2_lint_title_selector = scraper::Selector::parse(r#"h2[class="lint-title"]"#)
                .expect("f1473d4e-e26a-491d-9980-e1874301a6b2");
            let span_label_label_default_lint_group_group_deprecated_selector =
                scraper::Selector::parse(
                    r#"span[class="label label-default lint-group group-deprecated"]"#,
                )
                .expect("e86d5496-f62b-428c-ac6c-d533e0f6f775");
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
                                for h2_lint_title_selector_element in
                                    label_selector_element.select(&h2_lint_title_selector)
                                {
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
        compare_lints_vecs(
            rust_or_clippy,
            lints_vec_from_cargo_toml,
            clippy_lints_from_docs,
            //todo on commit momment seems like this lints still not added to clippy, but in the list in clippy site
            vec![String::from("same_length_and_capacity")],
        );
    }
    #[test]
    fn check_workspace_dependencies_having_exact_version() {
        let workspace = toml_value_from_from_cargo_toml_workspace();
        let dependencies = workspace
            .get("dependencies")
            .expect("2376f58e-394d-4759-96c1-e5379fdbb0b1");
        let table_value = match dependencies {
            toml::Value::Table(value) => value,
            toml::Value::String(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => panic!("e117fa5a-cc55-4ca8-a885-3d0c275592ea"),
        };
        for (_, value) in table_value {
            let value_table = match value {
                toml::Value::Table(table) => table,
                toml::Value::String(_)
                | toml::Value::Integer(_)
                | toml::Value::Float(_)
                | toml::Value::Boolean(_)
                | toml::Value::Datetime(_)
                | toml::Value::Array(_) => panic!("cb693a3f-ff75-47ba-b747-94361925e2e6"),
            };
            let value_table_len = value_table.len();
            let check_version = |current_value_table: &toml::value::Table| match current_value_table
                .get("version")
                .expect("d5b2b269-d832-4c94-887b-ec44a7e2045f")
            {
                toml::Value::String(version_string) => {
                    fn check_version_string(value: &str) -> Option<()> {
                        let rest = value.strip_prefix('=')?;
                        let mut iter = rest.split('.');
                        let _: u64 = iter.next()?.parse::<u64>().ok()?;
                        let _: u64 = iter.next()?.parse::<u64>().ok()?;
                        let _: u64 = iter.next()?.parse::<u64>().ok()?;
                        if iter.next().is_some() {
                            return None;
                        }
                        Some(())
                    }
                    check_version_string(version_string)
                        .expect("6640b9bf-8fd4-4a00-8c88-72087ba83f60");
                }
                toml::Value::Table(_)
                | toml::Value::Integer(_)
                | toml::Value::Float(_)
                | toml::Value::Boolean(_)
                | toml::Value::Datetime(_)
                | toml::Value::Array(_) => panic!("a3410a37-d6f8-4a5d-acb6-8449b02181ab"),
            };
            let check_features =
                |current_value_table: &toml::value::Table| match current_value_table
                    .get("features")
                    .expect("473577d5-0482-4460-b211-60131d9b7c2a")
                {
                    toml::Value::Array(_) => (),
                    toml::Value::String(_)
                    | toml::Value::Table(_)
                    | toml::Value::Integer(_)
                    | toml::Value::Float(_)
                    | toml::Value::Boolean(_)
                    | toml::Value::Datetime(_) => {
                        panic!("38ba32e9-fe34-4628-8505-414b937c645f")
                    }
                };
            if value_table_len == 1 {
                check_version(value_table);
            } else if value_table_len == 2 {
                check_version(value_table);
                check_features(value_table);
            } else if value_table_len == 3 {
                check_version(value_table);
                check_features(value_table);
                match value_table
                    .get("default-features")
                    .expect("847a138f-421b-47e5-a658-3789a8281b5c")
                {
                    toml::Value::Boolean(_) => (),
                    toml::Value::String(_)
                    | toml::Value::Table(_)
                    | toml::Value::Integer(_)
                    | toml::Value::Float(_)
                    | toml::Value::Datetime(_)
                    | toml::Value::Array(_) => panic!("b320164b-7082-45f0-9f89-1f5f28f6b779"),
                }
            } else {
                panic!("f1139378-0a18-4195-9b90-f3248a63253e {value_table:#?}")
            }
        }
    }
    #[test]
    fn check_expect_contains_only_unique_uuid_v4() {
        check_expect_or_panic_contains_only_unique_uuid_v4(ExpectOrPanic::Expect);
    }
    #[test]
    fn check_panic_contains_only_unique_uuid_v4() {
        check_expect_or_panic_contains_only_unique_uuid_v4(ExpectOrPanic::Panic);
    }
    #[test]
    fn check_rs_files_contains_only_unique_uuid_v4() {
        let regex = regex::Regex::new(
            r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\b"
        ).expect("e098a1ff-0e70-44f5-a75e-ffe6042ee9f5");
        let mut seen = std::collections::HashSet::new();
        for entry in project_directory()
            .into_iter()
            .filter_entry(|element| element.file_name() != "target")
            .filter_map(Result::ok)
            .filter(|element| {
                element
                    .path()
                    .extension()
                    .and_then(|current_element| current_element.to_str())
                    == Some("rs")
            })
        {
            let Ok(content) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            for element in regex.find_iter(&content) {
                let uuid = uuid::Uuid::parse_str(element.as_str())
                    .expect("c9711efd-eb37-4b10-b689-831fa916cb82");
                assert!(
                    uuid.get_version_num() == 4,
                    "49b49b21-0cc6-4aee-8c28-3003492f2a80"
                );
                assert!(seen.insert(uuid), "4cf9d239-d701-49bd-9f6b-8843c5a8814e");
            }
        }
    }
    #[test]
    fn all_files_are_english_only() {
        let mut errors = Vec::new();
        let exceptions = [
            "../postgresql_crud/postgresql_crud_common/src/lib.rs", //contain utf-8 String test
        ];
        for entry in project_directory()
            .into_iter()
            .filter_entry(|element| {
                let name = element.file_name().to_string_lossy();
                name != "target" && name != ".git"
            })
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if !path.is_file()
                || !path
                    .extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|some_value| {
                        matches!(
                            some_value,
                            "rs" | "toml" | "md" | "txt" | "yml" | "yaml" | "json"
                        )
                    })
            {
                continue;
            }
            if exceptions.contains(&path.display().to_string().as_str()) {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(path) else {
                continue; //skip binary non-utf8 files
            };
            for (line_index, line) in content.lines().enumerate() {
                for char_value in line.chars() {
                    if !(matches!(char_value, '\n' | '\r' | '\t') || char_value.is_ascii()) {
                        errors.push(format!(
                            "{}:{} non-english symbol `{}` (U+{:04X})",
                            path.display(),
                            line_index + 1,
                            char_value,
                            u32::from(char_value)
                        ));
                    }
                }
            }
        }
        assert!(errors.is_empty(), "non-english symbols:\n{}", errors.join("\n"));
    }
}
