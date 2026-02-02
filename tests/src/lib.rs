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
        lints_vec_from_cargo_toml: &[String],
        lints_to_check: &[String],
        lints_not_in_cargo_toml_vec_exceptions: &[String],
    ) {
        let rust_or_clippy_name = rust_or_clippy.name();
        let mut lints_not_in_cargo_toml = Vec::new();
        for element_31af38d6 in lints_to_check {
            if !lints_vec_from_cargo_toml.contains(element_31af38d6) {
                if lints_not_in_cargo_toml_vec_exceptions.contains(element_31af38d6) {
                    println!(
                        "todo!() {rust_or_clippy_name} {element_31af38d6} 158b5c43-05fa-4b8f-b6fe-9cda49d26997"
                    );
                } else {
                    lints_not_in_cargo_toml.push(element_31af38d6);
                }
            }
        }
        assert!(
            lints_not_in_cargo_toml.is_empty(),
            "d2b7ba9f-d133-496c-a29d-67503c3d9e8a"
        );
        let mut outdated_lints_in_file = Vec::new();
        for element_d3c0c904 in lints_vec_from_cargo_toml {
            if !lints_to_check.contains(element_d3c0c904) {
                outdated_lints_in_file.push(element_d3c0c904);
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
        for element_fcc35079 in project_directory()
            .into_iter()
            .filter_entry(|element_7e9cb4cf| element_7e9cb4cf.file_name() != "target")
            .filter_map(Result::ok)
            .filter(|element_2b9891bd| {
                element_2b9891bd
                    .path()
                    .extension()
                    .and_then(|element_bdd39cb5| element_bdd39cb5.to_str())
                    == Some("rs")
            })
        {
            let Ok(content) = std::fs::read_to_string(element_fcc35079.path()) else {
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
            all_errors.extend(visitor.errors.into_iter().map(|element_2b9891bd| {
                format!("{:?}: {}", element_fcc35079.path(), element_2b9891bd)
            }));
        }
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();
        for element_45f4b8bc in all_uuids {
            if !seen.insert(element_45f4b8bc.clone()) {
                duplicates.push(element_45f4b8bc);
            }
        }
        if !duplicates.is_empty() {
            all_errors.push(format!("duplicate UUIDs found: {duplicates:?}"));
        }
        assert!(
            all_errors.is_empty(),
            "6062a9e9-c12b-4961-89d2-f52a0ea344b4",
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
                .map(|element_70833f93| {
                    element_70833f93[1]
                        .to_string()
                        .replace('-', "_")
                        .to_lowercase()
                })
                .collect::<Vec<String>>()
        };
        compare_lints_vecs(
            rust_or_clippy,
            &lints_vec_from_cargo_toml,
            &lints_from_command,
            //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
            &vec![
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
            let mut ids = Vec::new();
            for element_c17d8a0b in document.select(
                &scraper::Selector::parse("html").expect("80427609-cfed-4b38-bdea-0794535ef84a"),
            ) {
                for element_e19e3742 in element_c17d8a0b.select(
                    &scraper::Selector::parse("body")
                        .expect("620c597c-0faa-408f-b9bc-29059d179951"),
                ) {
                    for element_3cd4b8b2 in element_e19e3742.select(
                        &scraper::Selector::parse(r#"div[class="container"]"#)
                            .expect("eb483b13-e70e-40f4-b83a-3eeb00413d57"),
                    ) {
                        for element_fda975ef in element_3cd4b8b2.select(
                            &scraper::Selector::parse("article")
                                .expect("d21dbe55-6f9f-4695-bf08-78da4f2424ea"),
                        ) {
                            let mut is_deprecated = false;
                            for element_ae33b117 in element_fda975ef.select(
                                &scraper::Selector::parse("label")
                                    .expect("fe3d9f11-f3b0-4e54-a54a-842fabe3d8a7"),
                            ) {
                                if is_deprecated {
                                    break;
                                }
                                for element_87a06075 in element_ae33b117.select(
                                    &scraper::Selector::parse(r#"h2[class="lint-title"]"#)
                                        .expect("f1473d4e-e26a-491d-9980-e1874301a6b2"),
                                ) {
                                    if is_deprecated {
                                        break;
                                    }
                                    if element_87a06075.select(
                                        &scraper::Selector::parse(
                                            r#"span[class="label label-default lint-group group-deprecated"]"#,
                                        ).expect("e86d5496-f62b-428c-ac6c-d533e0f6f775")
                                    ).next().is_some() {
                                        is_deprecated = true;
                                        break;
                                    }
                                }
                            }
                            if let Some(id) = element_fda975ef.value().attr("id")
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
            &lints_vec_from_cargo_toml,
            &clippy_lints_from_docs,
            &[],
        );
    }
    #[test]
    fn check_workspace_dependencies_having_exact_version() {
        for (_, value_5c36cb98) in match toml_value_from_from_cargo_toml_workspace()
            .get("dependencies")
            .expect("2376f58e-394d-4759-96c1-e5379fdbb0b1")
        {
            toml::Value::Table(value_270f9bd5) => value_270f9bd5,
            toml::Value::String(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => panic!("e117fa5a-cc55-4ca8-a885-3d0c275592ea"),
        } {
            let value_table = match value_5c36cb98 {
                toml::Value::Table(value_31495eb6) => value_31495eb6,
                toml::Value::String(_)
                | toml::Value::Integer(_)
                | toml::Value::Float(_)
                | toml::Value::Boolean(_)
                | toml::Value::Datetime(_)
                | toml::Value::Array(_) => panic!("cb693a3f-ff75-47ba-b747-94361925e2e6"),
            };
            let value_table_len = value_table.len();
            let check_version = |value_df993c3d: &toml::value::Table| match value_df993c3d
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
            let check_features = |value_121eb307: &toml::value::Table| match value_121eb307
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
        for element_44a8aa56 in project_directory()
            .into_iter()
            .filter_entry(|element_e4adf4c5| element_e4adf4c5.file_name() != "target")
            .filter_map(Result::ok)
            .filter(|element_714b3d9c| {
                element_714b3d9c
                    .path()
                    .extension()
                    .and_then(|element_7ac9041a| element_7ac9041a.to_str())
                    == Some("rs")
            })
        {
            let Ok(content) = std::fs::read_to_string(element_44a8aa56.path()) else {
                continue;
            };
            for element_714b3d9c in regex.find_iter(&content) {
                let uuid = uuid::Uuid::parse_str(element_714b3d9c.as_str())
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
        for element_d87f0495 in project_directory()
            .into_iter()
            .filter_entry(|element_6870bc3d| {
                let name = element_6870bc3d.file_name().to_string_lossy();
                name != "target" && name != ".git"
            })
            .filter_map(Result::ok)
        {
            let path = element_d87f0495.path();
            if !path.is_file()
                || !path
                    .extension()
                    .and_then(|value_56829539| value_56829539.to_str())
                    .is_some_and(|value_c980b45f| {
                        matches!(
                            value_c980b45f,
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
            for (key_0fa16fc1, value_3d676d2e) in content.lines().enumerate() {
                for element_c0fa9fc2 in value_3d676d2e.chars() {
                    if !(matches!(element_c0fa9fc2, '\n' | '\r' | '\t')
                        || element_c0fa9fc2.is_ascii())
                    {
                        errors.push(format!(
                            "{}:{} non-english symbol `{}` (U+{:04X})",
                            path.display(),
                            key_0fa16fc1 + 1,
                            element_c0fa9fc2,
                            u32::from(element_c0fa9fc2)
                        ));
                    }
                }
            }
        }
        assert!(
            errors.is_empty(),
            "non-english symbols:\n{}",
            errors.join("\n")
        );
    }
    #[test]
    fn workspace_crates_must_use_workspace_dependencies() {
        let exceptions = [
            "../Cargo.toml",               //workspace
            "../pg_jsonschema/Cargo.toml", //need just for postgresql extension
        ];
        for element_bebb7b9d in project_directory()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|element_6870bc3d| element_6870bc3d.file_name() == "Cargo.toml")
        {
            let path = element_bebb7b9d.path();
            if exceptions.contains(&path.display().to_string().as_str()) {
                continue;
            }
            let mut file = std::fs::File::open(path).expect("bbb0d1fe-e503-4c46-8f2b-954d42090f41");
            let mut content = String::new();
            let _: usize = std::io::Read::read_to_string(&mut file, &mut content)
                .expect("8952ff62-d903-4b93-b46a-85ae5177f98d");
            let parsed: toml::Table = content
                .parse()
                .expect("49012f1f-e721-40b5-8167-5258d206196b");
            for element_3c618c8f in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if let Some(deps) = parsed
                    .get(element_3c618c8f)
                    .and_then(|value_5e0a4d6a| value_5e0a4d6a.as_table())
                {
                    for (key_794900d4, value_07583f81) in deps {
                        let panic_with_message = || {
                            panic!(
                                "{}: dependency `{}` in [{}] must use `.workspace = true` \
                                 (only `path = ...` is allowed as exception)",
                                path.display(),
                                key_794900d4,
                                element_3c618c8f
                            )
                        };
                        match value_07583f81 {
                            toml::Value::Table(value_bba39a72) => {
                                if !(value_bba39a72.contains_key("path")
                                    || (value_bba39a72.get("workspace")
                                        == Some(&toml::Value::Boolean(true))))
                                {
                                    panic_with_message();
                                }
                            }
                            toml::Value::String(_)
                            | toml::Value::Integer(_)
                            | toml::Value::Float(_)
                            | toml::Value::Boolean(_)
                            | toml::Value::Datetime(_)
                            | toml::Value::Array(_) => panic_with_message(),
                        }
                    }
                }
            }
        }
    }
}
