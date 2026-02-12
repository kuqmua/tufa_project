#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        use quote::quote;
        macro_clippy_check_common::clippy_check(
            "gen_postgres_types_test_content",
            "../postgres_crud/postgres_types/",
            r#"[dependencies]
chrono.workspace = true
uuid.workspace = true
sqlx.workspace = true
serde.workspace = true
thiserror.workspace = true
error_occurence_lib = {path = "../../../error_occurence_lib"}
postgres_crud_common = {path = "../../postgres_crud_common"}
postgres_types_common = {path = "../postgres_types_common"}
where_filters = {path = "../../where_filters"}

[features]
test-utils = []"#,
            &gen_postgres_types_source::gen_postgres_types(
                &quote! {
                    {
                        "postgres_table_columns_content_write_into_postgres_table_columns_using_postgres_types": "False",
                        "whole_content_write_into_gen_postgres_types": "False",
                        "variant": "All"
                    }
                }
            ).to_string()
        );
    }
}
