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
        let mut acc = String::new();
        let _: usize = Read::read_to_string(&mut file, &mut acc).expect("2f5914f2");
        let table = acc.parse::<TomlTable>().expect("beb11586");
        table.get("workspace").expect("f728192d").clone()
    }
    fn lints_vec_from_cargo_toml_workspace(rust_or_clippy: RustOrClippy) -> Vec<String> {
        let workspace = toml_value_from_from_cargo_toml_workspace();
        let lints = workspace.get("lints").expect("82eaea37");
        let toml_value_table = match lints.get(rust_or_clippy.name()).expect("dbd02f72").clone() {
            Value::Table(v) => v,
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
        for el in lints_to_check {
            if !lints_vec_from_cargo_toml.contains(el) {
                if lints_not_in_cargo_toml_vec_exceptions.contains(el) {
                    println!(
                        "todo!() {rust_or_clippy_name} {el} 158b5c43-05fa-4b8f-b6fe-9cda49d26997"
                    );
                } else {
                    lints_not_in_cargo_toml.push(el);
                }
            }
        }
        assert!(lints_not_in_cargo_toml.is_empty(), "d2b7ba9f");
        let mut outdated_lints_in_file = Vec::new();
        for el in lints_vec_from_cargo_toml {
            if !lints_to_check.contains(el) {
                outdated_lints_in_file.push(el);
            }
        }
        assert!(outdated_lints_in_file.is_empty(), "93787d2d");
    }
    fn check_expect_or_panic_contains_only_unique_uuid_v4(expect_or_panic: ExpectOrPanic) {
        struct ExpectVisitor {
            ers: Vec<String>,
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
                            let v = lit_str.value();
                            if v.len() == 8 {
                                self.uuids.push(v);
                            } else {
                                self.ers.push(format!("arg len is not 8: {v}"));
                            }
                        } else {
                            self.ers.push("arg is not string literal".to_owned());
                        }
                    } else {
                        self.ers.push("with != 1 arg".to_owned());
                    }
                }
                visit_expr_method_call(self, i);
            }
        }
        let mut all_uuids = Vec::new();
        let mut all_ers = Vec::new();
        for el in project_directory()
            .into_iter()
            .filter_entry(|el| el.file_name() != "target")
            .filter_map(Result::ok)
            .filter(|el| el.path().extension().and_then(|el0| el0.to_str()) == Some("rs"))
        {
            let Ok(ts) = read_to_string(el.path()) else {
                continue;
            };
            let ast = parse_file(&ts).expect("5e7a83eb");
            let mut visitor = ExpectVisitor {
                expect_or_panic,
                uuids: Vec::new(),
                ers: Vec::new(),
            };
            Visit::visit_file(&mut visitor, &ast);
            all_uuids.extend(visitor.uuids);
            all_ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|el0| format!("{:?}: {}", el.path(), el0)),
            );
        }
        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();
        for el in all_uuids {
            if !seen.insert(el.clone()) {
                duplicates.push(el);
            }
        }
        if !duplicates.is_empty() {
            all_ers.push(format!("duplicate UUIDs found: {duplicates:?}"));
        }
        assert!(all_ers.is_empty(), "6062a9e9 {all_ers:#?}",);
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
                .map(|el| el[1].to_string().replace('-', "_").to_lowercase())
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
            for el in document.select(&Selector::parse("html").expect("80427609")) {
                for el0 in el.select(&Selector::parse("body").expect("620c597c")) {
                    for el1 in
                        el0.select(&Selector::parse(r#"div[class="container"]"#).expect("eb483b13"))
                    {
                        for el2 in el1.select(&Selector::parse("article").expect("d21dbe55")) {
                            let mut is_deprecated = false;
                            for el3 in el2.select(&Selector::parse("label").expect("fe3d9f11")) {
                                if is_deprecated {
                                    break;
                                }
                                for el4 in el3.select(
                                    &Selector::parse(r#"h2[class="lint-title"]"#)
                                        .expect("f1473d4e"),
                                ) {
                                    if is_deprecated {
                                        break;
                                    }
                                    if el4.select(
                                        &Selector::parse(
                                            r#"span[class="label label-default lint-group group-deprecated"]"#,
                                        ).expect("e86d5496")
                                    ).next().is_some() {
                                        is_deprecated = true;
                                        break;
                                    }
                                }
                            }
                            if let Some(id) = el2.value().attr("id")
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
            &["disallowed_fields".to_owned()],
        );
    }
    #[test]
    fn check_workspace_dependencies_having_exact_version() {
        for (_, v_5c36cb98) in match toml_value_from_from_cargo_toml_workspace()
            .get("dependencies")
            .expect("2376f58e")
            .clone()
        {
            Value::Table(v_270f9bd5) => v_270f9bd5,
            Value::String(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Boolean(_)
            | Value::Datetime(_)
            | Value::Array(_) => panic!("e117fa5a"),
        } {
            let value_table = match v_5c36cb98 {
                Value::Table(v_31495eb6) => v_31495eb6,
                Value::String(_)
                | Value::Integer(_)
                | Value::Float(_)
                | Value::Boolean(_)
                | Value::Datetime(_)
                | Value::Array(_) => panic!("cb693a3f"),
            };
            let value_table_len = value_table.len();
            let check_version =
                |v_df993c3d: &Table| match v_df993c3d.get("version").expect("d5b2b269").clone() {
                    Value::String(version_string) => {
                        fn check_version_string(v: &str) -> Option<()> {
                            let rest = v.strip_prefix('=')?;
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
                |v_121eb307: &Table| match v_121eb307.get("features").expect("473577d5") {
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
        for el in project_directory()
            .into_iter()
            .filter_entry(|el| el.file_name() != "target")
            .filter_map(Result::ok)
            .filter(|el| el.path().extension().and_then(|el0| el0.to_str()) == Some("rs"))
        {
            let Ok(v) = read_to_string(el.path()) else {
                continue;
            };
            for el0 in regex.find_iter(&v) {
                let uuid = Uuid::parse_str(el0.as_str()).expect("c9711efd");
                assert!(uuid.get_version_num() == 4, "49b49b21");
                assert!(seen.insert(uuid), "4cf9d239");
            }
        }
    }
    #[test]
    fn all_files_are_english_only() {
        let mut ers = Vec::new();
        let exceptions = [
            "../pg_crud/pg_crud_common/src/lib.rs", //contain utf-8 String test
        ];
        for el in project_directory()
            .into_iter()
            .filter_entry(|el| {
                let name = el.file_name().to_string_lossy();
                name != "target" && name != ".git"
            })
            .filter_map(Result::ok)
        {
            let path = el.path();
            if !path.is_file()
                || !path
                    .extension()
                    .and_then(|v_56829539| v_56829539.to_str())
                    .is_some_and(|v_c980b45f| {
                        matches!(
                            v_c980b45f,
                            "rs" | "toml" | "md" | "txt" | "yml" | "yaml" | "json"
                        )
                    })
            {
                continue;
            }
            if exceptions.contains(&path.display().to_string().as_str()) {
                continue;
            }
            let Ok(v) = read_to_string(path) else {
                continue; //skip binary non-utf8 files
            };
            for (key_0fa16fc1, v_3d676d2e) in v.lines().enumerate() {
                for el0 in v_3d676d2e.chars() {
                    if !(matches!(el0, '\n' | '\r' | '\t') || el0.is_ascii()) {
                        ers.push(format!(
                            "{}:{} non-english symbol `{}` (U+{:04X})",
                            path.display(),
                            key_0fa16fc1 + 1,
                            el0,
                            u32::from(el0)
                        ));
                    }
                }
            }
        }
        assert!(ers.is_empty(), "non-english symbols:\n{}", ers.join("\n"));
    }
    #[test]
    fn workspace_crates_must_use_workspace_dependencies() {
        let exceptions = [
            "../Cargo.toml", //workspace
        ];
        for el in project_directory()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|el| el.file_name() == "Cargo.toml")
        {
            let path = el.path();
            if exceptions.contains(&path.display().to_string().as_str()) {
                continue;
            }
            let mut file = File::open(path).expect("bbb0d1fe");
            let mut acc = String::new();
            let _: usize = Read::read_to_string(&mut file, &mut acc).expect("8952ff62");
            let parsed: Table = acc.parse().expect("49012f1f");
            for el0 in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if let Some(deps) = parsed
                    .get(el0)
                    .clone()
                    .and_then(|v_5e0a4d6a| v_5e0a4d6a.as_table())
                {
                    for (key_794900d4, v_07583f81) in deps {
                        let panic_with_message = || {
                            panic!(
                                "{}: dependency `{}` in [{}] must use `.workspace = true` \
                                 (only `path = ...` is allowed as exception)",
                                path.display(),
                                key_794900d4,
                                el0
                            )
                        };
                        match v_07583f81.clone() {
                            Value::Table(v_bba39a72) => {
                                if !(v_bba39a72.contains_key("path")
                                    || (v_bba39a72.get("workspace") == Some(&Value::Boolean(true))))
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
    #[test]
    fn check_no_empty_lines_in_rust_files() {
        let mut ers = Vec::new();
        for entry in project_directory()
            .into_iter()
            .filter_entry(|el| el.file_name() != "target")
            .filter_map(Result::ok)
            .filter(|el0| el0.path().extension().and_then(|el1| el1.to_str()) == Some("rs"))
        {
            let path = entry.path();
            let Ok(v) = read_to_string(path) else {
                continue;
            };
            let mut lines_iter = v.lines();
            if let Some(first_line) = lines_iter.next()
                && first_line.trim().is_empty()
                && lines_iter.next().is_none()
            {
                continue;
            }
            for (line_nbr, line) in v.lines().enumerate() {
                if line.trim().is_empty() {
                    ers.push(format!("{}:{} empty line", path.display(), line_nbr + 1));
                }
            }
        }
        assert!(
            ers.is_empty(),
            "Empty lines found in Rust files:\n{}",
            ers.join("\n")
        );
    }
}
