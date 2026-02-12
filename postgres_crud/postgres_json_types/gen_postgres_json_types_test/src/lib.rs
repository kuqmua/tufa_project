#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        use quote::quote;
        macro_clippy_check_common::clippy_check(
            "gen_postgres_json_types_test_content",
            "../postgres_crud/postgres_json_types/",
            r#"[dependencies]
schemars.workspace = true
regex.workspace = true
uuid.workspace = true
sqlx.workspace = true
serde.workspace = true
utoipa.workspace = true
error_occurence_lib = {path = "../../../error_occurence_lib"}
postgres_crud_common = {path = "../../postgres_crud_common"}
gen_postgres_json_types_common = {path = "../gen_postgres_json_types_common"}
where_filters = {path = "../../where_filters"}

[dev-dependencies]
uuid.workspace = true

[features]
test-utils = []"#,
            &gen_postgres_json_types_source::gen_postgres_json_types(&quote! {
                {
                    "postgres_table_columns_content_write_into_postgres_table_columns_using_postgres_json_types": "False",
                    "whole_content_write_into_gen_postgres_json_types": "False",
                    "variant": "WithDimensionOne"
                }
            })
            .to_string(),
        );
    }
}
