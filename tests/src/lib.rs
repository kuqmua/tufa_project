#[cfg(test)]
mod tests {
    use regex::Regex;
    use reqwest::blocking;
    use scraper::{Html, Selector};
    use std::{
        collections::HashSet,
        fs::{File, read_to_string},
        io::Read,
        process::{Command, Stdio},
    };
    use syn::{
        Expr, ExprLit, ExprMethodCall, Lit, parse_file,
        visit::{Visit, visit_expr_method_call},
    };
    use toml::{Table as TomlTable, Value, value::Table};
    use uuid::Uuid;
    use walkdir::WalkDir;
    #[derive(Debug, Clone, Copy)]
    enum RustOrClippy {
        Clippy,
        Rust,
    }
    impl RustOrClippy {
        fn name(&self) -> &str {
            match *self {
                Self::Rust => "rust",
                Self::Clippy => "clippy",
            }
        }
    }
    #[derive(Debug, Clone, Copy)]
    enum ExpectOrPanic {
        Expect,
        Panic,
    }
    fn toml_value_from_from_cargo_toml_workspace() -> Value {
        let mut file = File::open("../Cargo.toml").expect("39a0d238");
        let mut contents = String::new();
        let _: usize = Read::read_to_string(&mut file, &mut contents).expect("2f5914f2");
        let table = contents.parse::<TomlTable>().expect("beb11586");
        table.get("workspace").expect("f728192d").clone()
    }
    fn lints_vec_from_cargo_toml_workspace(rust_or_clippy: RustOrClippy) -> Vec<String> {
        let workspace = toml_value_from_from_cargo_toml_workspace();
        let lints = workspace.get("lints").expect("82eaea37");
        let toml_value_table = match lints.get(rust_or_clippy.name()).expect("dbd02f72").clone() {
            Value::Table(value) => value,
            Value::String(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Boolean(_)
            | Value::Datetime(_)
            | Value::Array(_) => panic!("cae226cd"),
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
        for el_31af38d6 in lints_to_check {
            if !lints_vec_from_cargo_toml.contains(el_31af38d6) {
                if lints_not_in_cargo_toml_vec_exceptions.contains(el_31af38d6) {
                    println!(
                        "todo!() {rust_or_clippy_name} {el_31af38d6} 158b5c43-05fa-4b8f-b6fe-9cda49d26997"
                    );
                } else {
                    lints_not_in_cargo_toml.push(el_31af38d6);
                }
            }
        }
        assert!(lints_not_in_cargo_toml.is_empty(), "d2b7ba9f");
        let mut outdated_lints_in_file = Vec::new();
        for el_d3c0c904 in lints_vec_from_cargo_toml {
            if !lints_to_check.contains(el_d3c0c904) {
                outdated_lints_in_file.push(el_d3c0c904);
            }
        }
        assert!(outdated_lints_in_file.is_empty(), "93787d2d");
    }
    fn check_expect_or_panic_contains_only_unique_uuid_v4(expect_or_panic: ExpectOrPanic) {
        struct ExpectVisitor {
            errors: Vec<String>,
            expect_or_panic: ExpectOrPanic,
            uuids: Vec<String>,
        }
        impl<'ast> Visit<'ast> for ExpectVisitor {
            fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
                let expect_or_panic_str = match self.expect_or_panic {
                    ExpectOrPanic::Expect => "expect",
                    ExpectOrPanic::Panic => "panic",
                };
                if i.method == expect_or_panic_str {
                    if i.args.len() == 1 {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }) = i.args.get(0).expect("d5ad7bff").clone()
                        {
                            let value = lit_str.value();
                            if value.len() == 8 {
                                self.uuids.push(value);
                            } else {
                                self.errors.push(format!("arg len is not 8: {value}"));
                            }
                        } else {
                            self.errors.push("arg is not string literal".to_owned());
                        }
                    } else {
                        self.errors.push("with != 1 arg".to_owned());
                    }
                }
                visit_expr_method_call(self, i);
            }
        }
        let mut all_uuids = Vec::new();
        let mut all_errors = Vec::new();
        for el_fcc35079 in project_directory()
            .into_iter()
            .filter_entry(|el_7e9cb4cf| el_7e9cb4cf.file_name() != "target")
            .filter_map(Result::ok)
            .filter(|el_2b9891bd| {
                el_2b9891bd
                    .path()
                    .extension()
                    .and_then(|el_bdd39cb5| el_bdd39cb5.to_str())
                    == Some("rs")
            })
        {
            let Ok(content) = read_to_string(el_fcc35079.path()) else {
                continue;
            };
            let ast = parse_file(&content).expect("5e7a83eb");
            let mut visitor = ExpectVisitor {
                expect_or_panic,
                uuids: Vec::new(),
                errors: Vec::new(),
            };
            Visit::visit_file(&mut visitor, &ast);
            all_uuids.extend(visitor.uuids);
            all_errors.extend(
                visitor
                    .errors
                    .into_iter()
                    .map(|el_2b9891bd| format!("{:?}: {}", el_fcc35079.path(), el_2b9891bd)),
            );
        }
        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();
        for el_45f4b8bc in all_uuids {
            if !seen.insert(el_45f4b8bc.clone()) {
                duplicates.push(el_45f4b8bc);
            }
        }
        if !duplicates.is_empty() {
            all_errors.push(format!("duplicate UUIDs found: {duplicates:?}"));
        }
        assert!(all_errors.is_empty(), "6062a9e9 {all_errors:#?}",);
    }
    fn project_directory() -> WalkDir {
        WalkDir::new("../")
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
        let rust_or_clippy = RustOrClippy::Rust;
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml_workspace(rust_or_clippy);
        let lints_from_command = {
            let output = Command::new("rustc")
                .args(["-W", "help"])
                .stdout(Stdio::piped())
                .output()
                .expect("7c939ff3");
            assert!(output.status.success(), "0c000f24");
            {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(stderr.trim().is_empty(), "0a4b2082");
            };
            let stdout = String::from_utf8_lossy(&output.stdout);
            Regex::new(r"(?m)^\s*([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b")
                .expect("60d99c87")
                .captures_iter(&stdout)
                .map(|el_70833f93| el_70833f93[1].to_string().replace('-', "_").to_lowercase())
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
                String::from("unqualified_local_imports"), //need to use some kind of different test flag or something for this
                String::from("unreachable_cfg_select_predicates"),
            ],
        );
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
        let rust_or_clippy = RustOrClippy::Clippy;
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml_workspace(rust_or_clippy);
        let clippy_lints_from_docs = {
            let document = Html::parse_document(
                &blocking::get("https://rust-lang.github.io/rust-clippy/master/index.html")
                    .expect("d1a0544a")
                    .text()
                    .expect("012e3328"),
            );
            let mut ids = Vec::new();
            for el_c17d8a0b in document.select(&Selector::parse("html").expect("80427609")) {
                for el_e19e3742 in el_c17d8a0b.select(&Selector::parse("body").expect("620c597c")) {
                    for el_3cd4b8b2 in el_e19e3742
                        .select(&Selector::parse(r#"div[class="container"]"#).expect("eb483b13"))
                    {
                        for el_fda975ef in
                            el_3cd4b8b2.select(&Selector::parse("article").expect("d21dbe55"))
                        {
                            let mut is_deprecated = false;
                            for el_ae33b117 in
                                el_fda975ef.select(&Selector::parse("label").expect("fe3d9f11"))
                            {
                                if is_deprecated {
                                    break;
                                }
                                for el_87a06075 in el_ae33b117.select(
                                    &Selector::parse(r#"h2[class="lint-title"]"#)
                                        .expect("f1473d4e"),
                                ) {
                                    if is_deprecated {
                                        break;
                                    }
                                    if el_87a06075.select(
                                        &Selector::parse(
                                            r#"span[class="label label-default lint-group group-deprecated"]"#,
                                        ).expect("e86d5496")
                                    ).next().is_some() {
                                        is_deprecated = true;
                                        break;
                                    }
                                }
                            }
                            if let Some(id) = el_fda975ef.value().attr("id")
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
            .expect("2376f58e")
            .clone()
        {
            Value::Table(value_270f9bd5) => value_270f9bd5,
            Value::String(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Boolean(_)
            | Value::Datetime(_)
            | Value::Array(_) => panic!("e117fa5a"),
        } {
            let value_table = match value_5c36cb98 {
                Value::Table(value_31495eb6) => value_31495eb6,
                Value::String(_)
                | Value::Integer(_)
                | Value::Float(_)
                | Value::Boolean(_)
                | Value::Datetime(_)
                | Value::Array(_) => panic!("cb693a3f"),
            };
            let value_table_len = value_table.len();
            let check_version = |value_df993c3d: &Table| match value_df993c3d
                .get("version")
                .expect("d5b2b269")
                .clone()
            {
                Value::String(version_string) => {
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
                    check_version_string(&version_string).expect("6640b9bf");
                }
                Value::Table(_)
                | Value::Integer(_)
                | Value::Float(_)
                | Value::Boolean(_)
                | Value::Datetime(_)
                | Value::Array(_) => panic!("a3410a37"),
            };
            let check_features =
                |value_121eb307: &Table| match value_121eb307.get("features").expect("473577d5") {
                    &Value::Array(_) => (),
                    &Value::String(_)
                    | &Value::Table(_)
                    | &Value::Integer(_)
                    | &Value::Float(_)
                    | &Value::Boolean(_)
                    | &Value::Datetime(_) => {
                        panic!("38ba32e9")
                    }
                };
            if value_table_len == 1 {
                check_version(&value_table);
            } else if value_table_len == 2 {
                check_version(&value_table);
                check_features(&value_table);
            } else if value_table_len == 3 {
                check_version(&value_table);
                check_features(&value_table);
                match value_table.get("default-features").expect("847a138f") {
                    &Value::Boolean(_) => (),
                    &Value::String(_)
                    | &Value::Table(_)
                    | &Value::Integer(_)
                    | &Value::Float(_)
                    | &Value::Datetime(_)
                    | &Value::Array(_) => panic!("b320164b"),
                }
            } else {
                panic!("f1139378 {value_table:#?}")
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
        let regex = Regex::new(
            r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\b"
        ).expect("e098a1ff");
        let mut seen = HashSet::new();
        for el_44a8aa56 in project_directory()
            .into_iter()
            .filter_entry(|el_e4adf4c5| el_e4adf4c5.file_name() != "target")
            .filter_map(Result::ok)
            .filter(|el_714b3d9c| {
                el_714b3d9c
                    .path()
                    .extension()
                    .and_then(|el_7ac9041a| el_7ac9041a.to_str())
                    == Some("rs")
            })
        {
            let Ok(content) = read_to_string(el_44a8aa56.path()) else {
                continue;
            };
            for el_714b3d9c in regex.find_iter(&content) {
                let uuid = Uuid::parse_str(el_714b3d9c.as_str()).expect("c9711efd");
                assert!(uuid.get_version_num() == 4, "49b49b21");
                assert!(seen.insert(uuid), "4cf9d239");
            }
        }
    }
    #[test]
    fn all_files_are_english_only() {
        let mut errors = Vec::new();
        let exceptions = [
            "../pg_crud/pg_crud_common/src/lib.rs", //contain utf-8 String test
        ];
        for el_d87f0495 in project_directory()
            .into_iter()
            .filter_entry(|el_6870bc3d| {
                let name = el_6870bc3d.file_name().to_string_lossy();
                name != "target" && name != ".git"
            })
            .filter_map(Result::ok)
        {
            let path = el_d87f0495.path();
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
            let Ok(content) = read_to_string(path) else {
                continue; //skip binary non-utf8 files
            };
            for (key_0fa16fc1, value_3d676d2e) in content.lines().enumerate() {
                for el_c0fa9fc2 in value_3d676d2e.chars() {
                    if !(matches!(el_c0fa9fc2, '\n' | '\r' | '\t') || el_c0fa9fc2.is_ascii()) {
                        errors.push(format!(
                            "{}:{} non-english symbol `{}` (U+{:04X})",
                            path.display(),
                            key_0fa16fc1 + 1,
                            el_c0fa9fc2,
                            u32::from(el_c0fa9fc2)
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
            "../pg_jsonschema/Cargo.toml", //need just for pg extension
        ];
        for el_bebb7b9d in project_directory()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|el_6870bc3d| el_6870bc3d.file_name() == "Cargo.toml")
        {
            let path = el_bebb7b9d.path();
            if exceptions.contains(&path.display().to_string().as_str()) {
                continue;
            }
            let mut file = File::open(path).expect("bbb0d1fe");
            let mut content = String::new();
            let _: usize = Read::read_to_string(&mut file, &mut content).expect("8952ff62");
            let parsed: Table = content.parse().expect("49012f1f");
            for el_3c618c8f in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if let Some(deps) = parsed
                    .get(el_3c618c8f)
                    .clone()
                    .and_then(|value_5e0a4d6a| value_5e0a4d6a.as_table())
                {
                    for (key_794900d4, value_07583f81) in deps {
                        let panic_with_message = || {
                            panic!(
                                "{}: dependency `{}` in [{}] must use `.workspace = true` \
                                 (only `path = ...` is allowed as exception)",
                                path.display(),
                                key_794900d4,
                                el_3c618c8f
                            )
                        };
                        match value_07583f81.clone() {
                            Value::Table(value_bba39a72) => {
                                if !(value_bba39a72.contains_key("path")
                                    || (value_bba39a72.get("workspace")
                                        == Some(&Value::Boolean(true))))
                                {
                                    panic_with_message();
                                }
                            }
                            Value::String(_)
                            | Value::Integer(_)
                            | Value::Float(_)
                            | Value::Boolean(_)
                            | Value::Datetime(_)
                            | Value::Array(_) => panic_with_message(),
                        }
                    }
                }
            }
        }
    }
}
