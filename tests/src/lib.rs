#[cfg(test)]
mod tests {
    use optml::Optml;
    use regex::Regex;
    use reqwest::blocking;
    use scraper::{Html, Selector};
    use std::{
        collections::HashSet,
        fs::read_to_string,
        path::Path,
        process::{Command, Stdio},
    };
    use syn::{
        Expr, ExprLit, ExprMethodCall, Lit, parse_file,
        visit::{Visit, visit_expr_method_call},
    };
    use toml::{Table as TomlTable, Value, value::Table};
    use uuid::Uuid;
    use walkdir::WalkDir;
    #[derive(Debug, Clone, Copy, Optml)]
    enum ExpectOrPanic {
        Expect,
        Panic,
    }
    #[derive(Debug, Clone, Copy, Optml)]
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
    struct DbgVisitor {
        found: bool,
    }
    impl<'ast> Visit<'ast> for DbgVisitor {
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if i.path
                .segments
                .last()
                .is_some_and(|v_4b8e1c7a| v_4b8e1c7a.ident == "dbg")
            {
                self.found = true;
            }
        }
    }
    struct TodoUnimplVisitor {
        found: Vec<String>,
    }
    impl<'ast> Visit<'ast> for TodoUnimplVisitor {
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if let Some(last_segment) = i.path.segments.last() {
                let name = last_segment.ident.to_string();
                if name == "todo" || name == "unimplemented" {
                    self.found.push(name);
                }
            }
        }
    }
    struct UnwrapVisitor {
        found: Vec<String>,
    }
    impl<'ast> Visit<'ast> for UnwrapVisitor {
        fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
            if i.method == "unwrap" && i.args.is_empty() {
                self.found.push(String::from("unwrap() call"));
            }
            visit_expr_method_call(self, i);
        }
    }
    #[test]
    fn all_crates_have_publish_false() {
        let exceptions = ["../Cargo.toml"];
        let mut ers = Vec::new();
        for el_a8d3e6f1 in cargo_toml_project_files() {
            let path = el_a8d3e6f1.path();
            if is_exception(path, &exceptions) {
                continue;
            }
            let Some(parsed) = read_toml_tbl(path) else {
                continue;
            };
            let publish = parsed
                .get("package")
                .and_then(|v_1c7b4e9d| v_1c7b4e9d.get("publish"));
            if publish != Some(&Value::Boolean(false)) {
                ers.push(format!("{}: missing `publish = false`", path.display()));
            }
        }
        assert!(ers.is_empty(), "f2a8c5d3\n{}", ers.join("\n"));
    }
    #[test]
    fn all_crates_have_workspace_lints() {
        let exceptions = ["../Cargo.toml"];
        let mut ers = Vec::new();
        for el_e3a17c4b in cargo_toml_project_files() {
            let path = el_e3a17c4b.path();
            if is_exception(path, &exceptions) {
                continue;
            }
            let Some(parsed) = read_toml_tbl(path) else {
                continue;
            };
            match parsed
                .get("lints")
                .and_then(|v_8f2a3d6b| v_8f2a3d6b.as_table())
            {
                Some(lints_tbl) => {
                    if lints_tbl.get("workspace") != Some(&Value::Boolean(true)) {
                        ers.push(format!(
                            "{}: [lints] missing `workspace = true`",
                            path.display()
                        ));
                    }
                }
                None => {
                    ers.push(format!("{}: missing [lints] section", path.display()));
                }
            }
        }
        assert!(ers.is_empty(), "d5f1a4e7\n{}", ers.join("\n"));
    }
    #[test]
    fn all_crates_use_edition_2024() {
        let exceptions = ["../Cargo.toml"];
        let mut ers = Vec::new();
        for el_b4c7e1a3 in cargo_toml_project_files() {
            let path = el_b4c7e1a3.path();
            if is_exception(path, &exceptions) {
                continue;
            }
            let Some(parsed) = read_toml_tbl(path) else {
                continue;
            };
            let edition = parsed
                .get("package")
                .and_then(|v_6d9f2a3e| v_6d9f2a3e.get("edition"));
            if edition != Some(&Value::String(String::from("2024"))) {
                ers.push(format!("{}: edition is not \"2024\"", path.display()));
            }
        }
        assert!(ers.is_empty(), "a3d7f1c8\n{}", ers.join("\n"));
    }
    #[test]
    fn all_files_are_english_only() {
        let mut ers = Vec::new();
        let exceptions = [
            "../pg_crud/pg_crud_cmn/src/lib.rs", //contain utf-8 String test
            "../CODE_IMPROVEMENT_PLAN.md",
            "../DEVELOPMENT_PLAN.md",
        ];
        for el_d87f0495 in project_dir()
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
            if is_exception(path, &exceptions) {
                continue;
            }
            let Ok(v) = read_to_string(path) else {
                continue; //skip binary non-utf8 files
            };
            for (k_0fa16fc1, v_3d676d2e) in v.lines().enumerate() {
                for el_c0fa9fc2 in v_3d676d2e.chars() {
                    if !(matches!(el_c0fa9fc2, '\n' | '\r' | '\t' | '\u{2014}' | '\u{2194}')
                        || el_c0fa9fc2.is_ascii())
                    {
                        ers.push(format!(
                            "{}:{} non-english symbol `{}` (U+{:04X})",
                            path.display(),
                            k_0fa16fc1 + 1,
                            el_c0fa9fc2,
                            u32::from(el_c0fa9fc2)
                        ));
                    }
                }
            }
        }
        assert!(ers.is_empty(), "non-english symbols:\n{}", ers.join("\n"));
    }
    fn cargo_toml_project_files() -> impl Iterator<Item = walkdir::DirEntry> {
        project_dir()
            .into_iter()
            .filter_entry(|el| el.file_name() != "target" && el.file_name() != ".git")
            .filter_map(Result::ok)
            .filter(|el| el.file_name() == "Cargo.toml")
    }
    fn check_expect_or_panic_contains_only_unq_uuid_v4(expect_or_panic: ExpectOrPanic) {
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
                        if let Some(Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        })) = i.args.first()
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
        for el_fcc35079 in rs_project_files() {
            let Ok(ts) = read_to_string(el_fcc35079.path()) else {
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
                    .map(|el_2b9891bd| format!("{:?}: {}", el_fcc35079.path(), el_2b9891bd)),
            );
        }
        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();
        for el_45f4b8bc in &all_uuids {
            if !seen.insert(el_45f4b8bc.as_str()) {
                duplicates.push(el_45f4b8bc.clone());
            }
        }
        if !duplicates.is_empty() {
            all_ers.push(format!("duplicate UUIDs found: {duplicates:?}"));
        }
        assert!(all_ers.is_empty(), "6062a9e9 {all_ers:#?}",);
    }
    #[test]
    fn check_expect_contains_only_unq_uuid_v4() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Expect);
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
            for el_fda975ef in
                document.select(&sel(r#"div[class="container"] article"#, "eb483b13"))
            {
                let mut is_deprecated = false;
                for el_ae33b117 in el_fda975ef.select(&sel("label", "fe3d9f11")) {
                    if is_deprecated {
                        break;
                    }
                    for el_87a06075 in
                        el_ae33b117.select(&sel(r#"h2[class="lint-title"]"#, "f1473d4e"))
                    {
                        if is_deprecated {
                            break;
                        }
                        if el_87a06075
                            .select(&sel(
                                r#"span[class="label label-dflt lint-group group-deprecated"]"#,
                                "e86d5496",
                            ))
                            .next()
                            .is_some()
                        {
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
            ids
        };
        compare_lints_vecs(
            rust_or_clippy,
            &lints_vec_from_cargo_toml,
            &clippy_lints_from_docs,
            &[
                "disallowed_fields".to_owned(),
                "unnecessary_trailing_comma".to_owned(),
                "manual_pop_if".to_owned(),
                "assign_ops".to_owned(),
                "extend_from_slice".to_owned(),
                "match_on_vec_items".to_owned(),
                "misaligned_transmute".to_owned(),
                "option_map_or_err_ok".to_owned(),
                "pub_enum_variant_names".to_owned(),
                "range_step_by_zero".to_owned(),
                "regex_macro".to_owned(),
                "replace_consts".to_owned(),
                "should_assert_eq".to_owned(),
                "string_to_string".to_owned(),
                "unsafe_vector_initialization".to_owned(),
                "unstable_as_mut_slice".to_owned(),
                "unstable_as_slice".to_owned(),
                "unused_collect".to_owned(),
                "wrong_pub_self_convention".to_owned(),
                "manual_noop_waker".to_owned(),
                "manual_option_zip".to_owned(),
                "useless_borrows_in_formatting".to_owned(),
            ],
        );
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
        let rust_or_clippy = RustOrClippy::Rust;
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml_workspace(rust_or_clippy);
        let lints_from_cmd = {
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
            &lints_from_cmd,
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
                String::from("dflt_overrides_dflt_fields"),
                String::from("test_unstable_lint"),
                String::from("resolving_to_items_shadowing_supertrait_items"),
                String::from("shadowing_supertrait_items"),
                String::from("unqualified_local_imports"), //need to use some kind of different test flag or something for this
                String::from("unreachable_cfg_select_predicates"),
                String::from("default_overrides_default_fields"),
                String::from("linker_info"),
                String::from("duplicate_features"),
                String::from("deprecated_llvm_intrinsic"),
            ],
        );
    }
    #[test]
    fn check_panic_contains_only_unq_uuid_v4() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Panic);
    }
    #[test]
    fn check_rs_files_contains_only_unq_uuid_v4() {
        let rgx = Regex::new(
            r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\b"
        ).expect("e098a1ff");
        let mut seen = HashSet::new();
        for el_44a8aa56 in rs_project_files() {
            let Ok(v) = read_to_string(el_44a8aa56.path()) else {
                continue;
            };
            for el_714b3d9c in rgx.find_iter(&v) {
                let uuid = Uuid::parse_str(el_714b3d9c.as_str()).expect("c9711efd");
                assert!(uuid.get_version_num() == 4, "49b49b21");
                assert!(seen.insert(uuid), "4cf9d239");
            }
        }
    }
    #[test]
    fn check_workspace_dependencies_having_exact_version() {
        for (_, v_5c36cb98) in toml_val_as_tbl(
            toml_v_from_from_cargo_toml_workspace()
                .get("dependencies")
                .expect("2376f58e")
                .clone(),
            "e117fa5a",
        ) {
            let v_tbl = toml_val_as_tbl(v_5c36cb98, "cb693a3f");
            if let Some(path_v) = v_tbl.get("path") {
                match path_v {
                    Value::String(_) => continue,
                    Value::Table(_)
                    | Value::Integer(_)
                    | Value::Float(_)
                    | Value::Boolean(_)
                    | Value::Datetime(_)
                    | Value::Array(_) => panic!("6ca03a1f"),
                }
            }
            let v_tbl_len = v_tbl.len();
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
            if v_tbl_len == 1 {
                check_version(&v_tbl);
            } else if v_tbl_len == 2 {
                check_version(&v_tbl);
                check_features(&v_tbl);
            } else if v_tbl_len == 3 {
                check_version(&v_tbl);
                check_features(&v_tbl);
                match v_tbl.get("default-features").expect("847a138f") {
                    &Value::Boolean(_) => (),
                    &Value::String(_)
                    | &Value::Table(_)
                    | &Value::Integer(_)
                    | &Value::Float(_)
                    | &Value::Datetime(_)
                    | &Value::Array(_) => panic!("b320164b"),
                }
            } else {
                panic!("f1139378 {v_tbl:#?}")
            }
        }
    }
    fn compare_lints_vecs(
        rust_or_clippy: RustOrClippy,
        lints_vec_from_cargo_toml: &[String],
        lints_to_check: &[String],
        lints_not_in_cargo_toml_vec_exceptions: &[String],
    ) {
        let rust_or_clippy_name = rust_or_clippy.name();
        let lints_from_cargo_set = lints_vec_from_cargo_toml
            .iter()
            .map(String::as_str)
            .collect::<HashSet<&str>>();
        let lints_to_check_set = lints_to_check
            .iter()
            .map(String::as_str)
            .collect::<HashSet<&str>>();
        let lints_exceptions_set = lints_not_in_cargo_toml_vec_exceptions
            .iter()
            .map(String::as_str)
            .collect::<HashSet<&str>>();
        let mut lints_not_in_cargo_toml = Vec::new();
        for el_31af38d6 in lints_to_check {
            if !lints_from_cargo_set.contains(el_31af38d6.as_str()) {
                if lints_exceptions_set.contains(el_31af38d6.as_str()) {
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
            if !lints_to_check_set.contains(el_d3c0c904.as_str()) {
                outdated_lints_in_file.push(el_d3c0c904);
            }
        }
        assert!(outdated_lints_in_file.is_empty(), "93787d2d");
    }
    fn env_keys_from_file(path: &str) -> Vec<String> {
        read_to_string(path)
            .expect("b3a7c1e4")
            .lines()
            .filter(|line| !line.starts_with('#') && line.contains('='))
            .map(|line| line.split('=').next().expect("d1f4a7c2").to_owned())
            .collect()
    }
    #[test]
    fn env_and_envexample_have_same_keys() {
        let env_keys = env_keys_from_file("../server/.env");
        let example_keys = env_keys_from_file("../server/.envexample");
        let env_keys_set = env_keys
            .iter()
            .map(String::as_str)
            .collect::<HashSet<&str>>();
        let example_keys_set = example_keys
            .iter()
            .map(String::as_str)
            .collect::<HashSet<&str>>();
        let mut ers = Vec::new();
        for el_a1b2c3d4 in &env_keys {
            if !example_keys_set.contains(el_a1b2c3d4.as_str()) {
                ers.push(format!(
                    "key `{el_a1b2c3d4}` in .env but missing from .envexample"
                ));
            }
        }
        for el_e5f6a7b8 in &example_keys {
            if !env_keys_set.contains(el_e5f6a7b8.as_str()) {
                ers.push(format!(
                    "key `{el_e5f6a7b8}` in .envexample but missing from .env"
                ));
            }
        }
        ers.sort();
        assert!(ers.is_empty(), "c8d2f1a3\n{}", ers.join("\n"));
    }
    fn is_exception(path: &Path, exceptions: &[&str]) -> bool {
        exceptions.contains(&path.display().to_string().as_str())
    }
    fn lints_vec_from_cargo_toml_workspace(rust_or_clippy: RustOrClippy) -> Vec<String> {
        let workspace = toml_v_from_from_cargo_toml_workspace();
        let lints = workspace.get("lints").expect("82eaea37");
        let toml_v_tbl = toml_val_as_tbl(
            lints.get(rust_or_clippy.name()).expect("dbd02f72").clone(),
            "cae226cd",
        );
        toml_v_tbl.keys().cloned().collect::<Vec<String>>()
    }
    fn read_toml_tbl(path: &Path) -> Option<TomlTable> {
        let v = read_to_string(path).ok()?;
        v.parse::<TomlTable>().ok()
    }
    #[test]
    fn no_dbg_macro_in_source_code() {
        let mut ers = Vec::new();
        for el_d7a2c4e9 in rs_project_files() {
            let path = el_d7a2c4e9.path();
            let Ok(v) = read_to_string(path) else {
                continue;
            };
            let Ok(ast) = parse_file(&v) else {
                continue;
            };
            let mut visitor = DbgVisitor { found: false };
            Visit::visit_file(&mut visitor, &ast);
            if visitor.found {
                ers.push(format!("{}: contains dbg!()", path.display()));
            }
        }
        assert!(ers.is_empty(), "f1c7a4e3 dbg!() found:\n{}", ers.join("\n"));
    }
    #[test]
    fn no_empty_lines_in_rust_files() {
        let mut ers = Vec::new();
        for entry in rs_project_files() {
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
    #[test]
    fn no_todo_or_unimplemented_macro_in_source_code() {
        let mut ers = Vec::new();
        for el_a4d8e2f6 in rs_project_files() {
            let path = el_a4d8e2f6.path();
            let Ok(v) = read_to_string(path) else {
                continue;
            };
            let Ok(ast) = parse_file(&v) else {
                continue;
            };
            let mut visitor = TodoUnimplVisitor { found: Vec::new() };
            Visit::visit_file(&mut visitor, &ast);
            for el_8c3a5e1d in &visitor.found {
                ers.push(format!("{}: contains {el_8c3a5e1d}!()", path.display()));
            }
        }
        assert!(
            ers.is_empty(),
            "c4e9a2d7 todo!/unimplemented! found:\n{}",
            ers.join("\n")
        );
    }
    #[test]
    fn no_unwrap_in_source_code() {
        let mut ers = Vec::new();
        for el_c1e8a3d5 in rs_project_files() {
            let path = el_c1e8a3d5.path();
            let Ok(v) = read_to_string(path) else {
                continue;
            };
            let Ok(ast) = parse_file(&v) else {
                continue;
            };
            let mut visitor = UnwrapVisitor { found: Vec::new() };
            Visit::visit_file(&mut visitor, &ast);
            for el_5d2f8a1c in visitor.found {
                ers.push(format!("{}: {el_5d2f8a1c}", path.display()));
            }
        }
        assert!(
            ers.is_empty(),
            "e8b3a6d2 unwrap() found:\n{}",
            ers.join("\n")
        );
    }
    fn project_dir() -> WalkDir {
        WalkDir::new("../")
    }
    fn rs_project_files() -> impl Iterator<Item = walkdir::DirEntry> {
        project_dir()
            .into_iter()
            .filter_entry(|el| {
                el.file_name() != "target"
                    && el.file_name() != ".git"
                    && (el.file_type().is_dir()
                        || el.path().extension().and_then(|e| e.to_str()) == Some("rs"))
            })
            .filter_map(Result::ok)
            .filter(|el| el.path().extension().and_then(|e| e.to_str()) == Some("rs"))
    }
    fn sel(s: &str, uuid: &str) -> Selector {
        Selector::parse(s).unwrap_or_else(|_| panic!("{uuid}"))
    }
    fn toml_v_from_from_cargo_toml_workspace() -> Value {
        let tbl = read_to_string("../Cargo.toml")
            .expect("39a0d238")
            .parse::<TomlTable>()
            .expect("beb11586");
        tbl.get("workspace").expect("f728192d").clone()
    }
    fn toml_val_as_tbl(v: Value, uuid: &str) -> Table {
        match v {
            Value::Table(t) => t,
            Value::String(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Boolean(_)
            | Value::Datetime(_)
            | Value::Array(_) => panic!("{uuid}"),
        }
    }
    #[test]
    fn workspace_crates_must_use_workspace_dependencies() {
        let exceptions = [
            "../Cargo.toml", //workspace
        ];
        for el_bebb7b9d in project_dir()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|el_6870bc3d| el_6870bc3d.file_name() == "Cargo.toml")
        {
            let path = el_bebb7b9d.path();
            if is_exception(path, &exceptions) {
                continue;
            }
            let parsed: Table = read_to_string(path)
                .expect("bbb0d1fe")
                .parse()
                .expect("49012f1f");
            for el_3c618c8f in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if let Some(deps) = parsed
                    .get(el_3c618c8f)
                    .clone()
                    .and_then(|v_5e0a4d6a| v_5e0a4d6a.as_table())
                {
                    for (k_794900d4, v_07583f81) in deps {
                        let panic_with_msg = || {
                            panic!(
                                "{}: dependency `{}` in [{}] must use `.workspace = true` \
                                 (only `path = ...` is allowed as exception)",
                                path.display(),
                                k_794900d4,
                                el_3c618c8f
                            )
                        };
                        match v_07583f81.clone() {
                            Value::Table(v_bba39a72) => {
                                if !(v_bba39a72.contains_key("path")
                                    || (v_bba39a72.get("workspace") == Some(&Value::Boolean(true))))
                                {
                                    panic_with_msg();
                                }
                            }
                            Value::String(_)
                            | Value::Integer(_)
                            | Value::Float(_)
                            | Value::Boolean(_)
                            | Value::Datetime(_)
                            | Value::Array(_) => panic_with_msg(),
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn workspace_members_exist_on_disk() {
        let workspace = toml_v_from_from_cargo_toml_workspace();
        let root = workspace
            .get("members")
            .and_then(|v_2a6c1b0e| v_2a6c1b0e.as_array());
        let Some(members) = root else {
            panic!("7f3a1c4e");
        };
        let mut ers = Vec::new();
        for el_7b2d9e5a in members {
            if let Some(member_str) = el_7b2d9e5a.as_str() {
                let path = Path::new("..").join(member_str).join("Cargo.toml");
                if !path.exists() {
                    ers.push(format!(
                        "member `{member_str}` Cargo.toml not found at {}",
                        path.display()
                    ));
                }
            }
        }
        assert!(ers.is_empty(), "a4e3b8d1 {ers:#?}");
    }
    #[test]
    fn workspace_members_sorted_alphabetically() {
        let workspace = toml_v_from_from_cargo_toml_workspace();
        let Some(members) = workspace
            .get("members")
            .and_then(|v_6e8f2c1a| v_6e8f2c1a.as_array())
        else {
            panic!("c1d4f7a2");
        };
        let members_vec: Vec<String> = members
            .iter()
            .filter_map(|v_0d3a7b9f| v_0d3a7b9f.as_str().map(String::from))
            .collect();
        let mut sorted = members_vec.clone();
        sorted.sort();
        let mut ers = Vec::new();
        for (k_4b1e6a8c, (got, expected)) in members_vec.iter().zip(sorted.iter()).enumerate() {
            if got != expected {
                ers.push(format!(
                    "index {k_4b1e6a8c}: got `{got}`, expected `{expected}`"
                ));
            }
        }
        assert!(
            ers.is_empty(),
            "b7c2e5f8 members not sorted:\n{}",
            ers.join("\n")
        );
    }
}
