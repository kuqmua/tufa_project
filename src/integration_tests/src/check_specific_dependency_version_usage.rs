#[test]
fn check_specific_dependency_version_usage() {
    let cargo_toml = "../Cargo.toml";
    let cannot_open_file = "cannot open ";
    let file_error = " file, error: ";
    let mut buf_reader = std::io::BufReader::new(
        std::fs::File::open(cargo_toml)
            .unwrap_or_else(|e| panic!("{cannot_open_file}{cargo_toml}{file_error}\"{e}\"")),
    );
    let mut cargo_toml_workspace_content = String::new();
    {
        std::io::Read::read_to_string(&mut buf_reader, &mut cargo_toml_workspace_content)
            .unwrap_or_else(|e| {
                panic!("cannot read_to_string from {cargo_toml}{file_error}\"{e}\"")
            });
    }
    let toml_table_map = cargo_toml_workspace_content
        .parse::<toml::Table>()
        .unwrap_or_else(|e| {
            panic!("cannot parse::<toml::Table>() cargo_toml_workspace_content, error:\"{e}\"")
        });
    let toml_table_workspace_members_map_vec = if let Some(toml_table_map_value) =
        toml_table_map.get("workspace")
    {
        if let toml::Value::Table(toml_table_workspace_map) = toml_table_map_value {
            if let Some(toml_table_workspace_map_value) = toml_table_workspace_map.get("members") {
                if let toml::Value::Array(toml_table_workspace_members_map_vec) =
                    toml_table_workspace_map_value
                {
                    toml_table_workspace_members_map_vec
                        .iter()
                        .map(|path_value| {
                            if let toml::Value::String(path) = path_value {
                                path
                            } else {
                                panic!("path is not a toml::Value::String");
                            }
                        })
                        .collect::<Vec<&String>>()
                } else {
                    panic!("toml_table_workspace_map is not a toml::Value::Array");
                }
            } else {
                panic!("no members in toml_table_workspace_map");
            }
        } else {
            panic!("toml_table_map_value is not a toml::Value::Table");
        }
    } else {
        panic!("no workspace in toml_table_map");
    };
    let forbidden_dependency_logic_symbols = ['>', '<', '*', '~', '^'];
    let toml_keys = ["dependencies", "dev-dependencies", "build-dependencies"];
    let mut is_logic_executed = true;
    let unspecified_dependencies = toml_table_workspace_members_map_vec
        .iter()
        .fold(Vec::new(), |mut acc, member| {
            let path_to_cargo_toml_member = format!("../{member}/Cargo.toml");
            let mut buf_reader_member = std::io::BufReader::new(
                std::fs::File::open(path_to_cargo_toml_member.clone())
                    .unwrap_or_else(|e| {
                        panic!("{cannot_open_file}{path_to_cargo_toml_member}{file_error}\"{e}\"")
                    }),
            );
            let mut cargo_toml_member_content = String::new();
            {
                std::io::Read::read_to_string(
                    &mut buf_reader_member,
                    &mut cargo_toml_member_content
                )
                .unwrap_or_else(|e| {
                    panic!("cannot read_to_string from {path_to_cargo_toml_member}{file_error}\"{e}\"")
                });
            }
            let cargo_toml_member_map = cargo_toml_member_content.parse::<toml::Table>().unwrap_or_else(|e| {
                panic!("cannot parse::<toml::Table>() cargo_toml_member_content for {member} {file_error}\"{e}\"")
            });
            toml_keys.iter().for_each(|toml_key|{
                let f = if let Some(toml_member_table_map_value) = cargo_toml_member_map.get(*toml_key) {
                    if let toml::Value::Table(toml_member_table_dependencies_map) = toml_member_table_map_value {
                        toml_member_table_dependencies_map
                        .iter()
                        .filter_map(|(crate_name, crate_value)| {
                            if let toml::Value::Table(crate_value_map) = crate_value {
                                if let Some(version_value) = crate_value_map.get("version") {
                                    if let toml::Value::String(version) = version_value {
                                        forbidden_dependency_logic_symbols.iter().for_each(|symbol|{
                                            if let true = version.contains(*symbol) {
                                                panic!("{crate_name} version of {member} contains forbidden symbol {symbol}");
                                            }
                                        });
                                        match version.starts_with('=') {
                                            true => None,
                                            false => Some(format!("{member} {toml_key} {crate_name} {version}")),
                                        }
                                    }
                                    else {
                                        panic!("{crate_name} version_value is not a toml::Value::String {member}");
                                    }
                                }
                                else {
                                    None
                                }
                            }
                            else {
                                panic!("{crate_name} crate_value is not a toml::Value::Table {member}");
                            }
                        })
                        .collect::<Vec<std::string::String>>()
                    } else {
                        panic!("no {toml_key} in cargo_toml_member_map of {member}");
                    }
                } else {
                    Vec::new()
                };
                f.into_iter().for_each(|g|{
                    acc.push(g);
                });
            });
            is_logic_executed = true;
            acc
        });
    if let false = is_logic_executed {
        panic!("logic is not executed, please check tokenized crate name(input parameter for check_specific_dependency_version_usage!(HERE)");
    }
    if let false = unspecified_dependencies.is_empty() {
        let mut error_message = std::string::String::from(
            "must use concrete versions with '=' symbol(like \"=1.2.3\") for: ",
        );
        unspecified_dependencies
            .iter()
            .enumerate()
            .for_each(|(index, unspecified_dependency)| {
                error_message.push_str(&format!("\n{}. {unspecified_dependency}", index + 1));
            });
        panic!("{error_message}\nMORE INFORMATION: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html");
    }
}