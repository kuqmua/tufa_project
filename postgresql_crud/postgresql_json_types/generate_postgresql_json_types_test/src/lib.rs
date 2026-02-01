#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        macro_clippy_check_common::clippy_check(
            "generate_postgresql_json_types_test_content",
            "../postgresql_crud/postgresql_json_types/",
            r#"[dependencies]
axum.workspace = true
schemars.workspace = true
regex.workspace = true
chrono.workspace = true
uuid.workspace = true
http.workspace = true
sqlx.workspace = true
reqwest.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
utoipa.workspace = true
git_info = {path = "../../../git_info"}
error_occurence_lib = {path = "../../../error_occurence_lib"}
postgresql_crud = {path = "../../../postgresql_crud", features = ["test-utils"]}
postgresql_crud_common = {path = "../../postgresql_crud_common"}
generate_postgresql_json_types_common = {path = "../generate_postgresql_json_types_common"}
where_filters = {path = "../../where_filters"}

[dev-dependencies]
quote.workspace = true
proc-macro2.workspace = true
num_cpus.workspace = true
futures.workspace = true
secrecy.workspace = true
tokio.workspace = true
tracing-subscriber.workspace = true
uuid.workspace = true
itertools.workspace = true
server_types = {path = "../../../server_types"}
app_state = {path = "../../../app_state"}
config_lib = {path = "../../../config_lib"}
server_app_state = {path = "../../../server_app_state"}
server_config = {path = "../../../server_config"}

[features]
test-utils = []"#,
            &generate_postgresql_json_types_source::generate_postgresql_json_types(quote::quote! {
                {
                    "postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_types": "False",
                    "whole_content_write_into_generate_postgresql_json_types": "False",
                    "variant": "All"
                }
            })
            .to_string(),
        );
    }
}
