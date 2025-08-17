#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    let mut file = std::fs::File::open("../Cargo.toml").unwrap();
    let mut contents = String::new();
    let _: usize = std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    let value = contents.parse::<toml::Table>().unwrap();
    let workspace = &value.get("workspace").unwrap();
    let lints = &workspace.get("lints").unwrap();
    let clippy = &lints.get("clippy").unwrap();
    let toml_value_table = match clippy {
        toml::Value::Table(value) => value,
        toml::Value::String(_) | toml::Value::Integer(_) | toml::Value::Float(_) | toml::Value::Boolean(_) | toml::Value::Datetime(_) | toml::Value::Array(_) => panic!("not ok"),
    };
    let lints_vec_from_file = toml_value_table.keys().collect::<Vec<&String>>();
    let body = reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/lints.json").unwrap().text().unwrap();
    #[derive(Debug, serde::Deserialize)]
    struct Lint {
        id: std::string::String,
        group: std::string::String,
    }
    let clippy_lints_from_docs = serde_json::from_str::<std::vec::Vec<Lint>>(&body).unwrap().into_iter().filter(|element| element.group != "deprecated").map(|element| element.id).collect::<std::vec::Vec<std::string::String>>();
    let mut lints_not_in_file = vec![];
    for element in &clippy_lints_from_docs {
        if !lints_vec_from_file.contains(&element) {
            lints_not_in_file.push(element);
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
