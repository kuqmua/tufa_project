#[cfg(test)]
mod tests {
    use macro_clippy_check_common::clippy_check;
    use quote::quote;
    #[test]
    fn clippy() {
        clippy_check(
            "gen_pg_json_types_test_content",
            "../pg_crud/pg_json_types/",
            r#"[dependencies]
schemars.workspace = true
regex.workspace = true
uuid.workspace = true
sqlx.workspace = true
serde.workspace = true
utoipa.workspace = true
error_occurence_lib = {path = "../../../error_occurence_lib"}
pg_crud_common = {path = "../../pg_crud_common"}
gen_pg_json_types_common = {path = "../gen_pg_json_types_common"}
where_filters = {path = "../../where_filters"}

[dev-dependencies]
uuid.workspace = true

[features]
test-utils = []"#,
            &gen_pg_json_types_source::gen_pg_json_types(&quote! {
                {
                    "pg_table_columns_content_write_into_pg_table_columns_using_pg_json_types": "False",
                    "whole_content_write_into_gen_pg_json_types": "False",
                    "variant": "WithDimOne"
                }
            })
            .to_string(),
        );
    }
}
