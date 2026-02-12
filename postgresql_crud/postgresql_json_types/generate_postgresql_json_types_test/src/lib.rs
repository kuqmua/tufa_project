#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        use quote::quote;
        macro_clippy_check_common::clippy_check(
            "generate_postgresql_json_types_test_content",
            "../postgresql_crud/postgresql_json_types/",
            r#"[dependencies]
schemars.workspace = true
regex.workspace = true
uuid.workspace = true
sqlx.workspace = true
serde.workspace = true
utoipa.workspace = true
error_occurence_lib = {path = "../../../error_occurence_lib"}
postgresql_crud_common = {path = "../../postgresql_crud_common"}
generate_postgresql_json_types_common = {path = "../generate_postgresql_json_types_common"}
where_filters = {path = "../../where_filters"}

[dev-dependencies]
uuid.workspace = true

[features]
test-utils = []"#,
            &generate_postgresql_json_types_source::generate_postgresql_json_types(&quote! {
                {
                    "postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_types": "False",
                    "whole_content_write_into_generate_postgresql_json_types": "False",
                    "variant": "WithDimensionOne"
                }
            })
            .to_string(),
        );
    }
}
