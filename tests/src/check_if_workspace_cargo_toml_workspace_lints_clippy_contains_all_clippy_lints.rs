#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    let mut file = std::fs::File::open("../Cargo.toml").unwrap();
    let mut contents = String::new();
    use std::io::Read;
    let _ = file.read_to_string(&mut contents).unwrap();
    let value = contents.parse::<toml::Table>().unwrap();
    let workspace = &value["workspace"];
    let lints = &workspace["lints"];
    let clippy = &lints["clippy"];
    let toml_value_table = match clippy {
        toml::Value::Table(value) => value,
        _ => panic!("not ok")
    };
    let lints_vec_from_file = toml_value_table.keys().map(|element|element).collect::<Vec<&String>>();
    let body = reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/lints.json")
    .unwrap()
    .text()
    .unwrap();
    #[derive(Debug, serde::Deserialize)]
    struct Lint {
        id: std::string::String,
        group: std::string::String,
    }
    let clippy_lints_from_docs = serde_json::from_str::<std::vec::Vec<Lint>>(&body).unwrap()
        .into_iter()
        .filter(|element|element.group != "deprecated")
        .map(|element|element.id)
        .collect::<std::vec::Vec<std::string::String>>();
    let mut lints_not_in_file = vec![];
    for element in clippy_lints_from_docs {
        if !lints_vec_from_file.contains(&&element) {
            lints_not_in_file.push(element);
        }
    }
    if lints_not_in_file.len() != 0 {
        panic!("this clippy lints are not in the [workspace.lints.clippy]: {lints_not_in_file:#?}")
    }
}