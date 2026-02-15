#[cfg(test)]
mod tests {
    use macro_clippy_check_common::clippy_check;
    #[test]
    fn clippy() {
        use quote::quote;
        clippy_check(
            "gen_pg_types_test_content",
            "../pg_crud/pg_types/",
            r#"[dependencies]
chrono.workspace = true
uuid.workspace = true
sqlx.workspace = true
serde.workspace = true
thiserror.workspace = true
error_occurence_lib = {path = "../../../error_occurence_lib"}
pg_crud_common = {path = "../../pg_crud_common"}
pg_types_common = {path = "../pg_types_common"}
where_filters = {path = "../../where_filters"}

[features]
test-utils = []"#,
            &gen_pg_types_source::gen_pg_types(&quote! {
                {
                    "pg_table_columns_content_write_into_pg_table_columns_using_pg_types": "False",
                    "whole_content_write_into_gen_pg_types": "False",
                    "variant": "All"
                }
            })
            .to_string(),
        );
    }
}
