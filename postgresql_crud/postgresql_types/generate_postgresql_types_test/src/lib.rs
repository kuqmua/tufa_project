#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        macro_clippy_check_common::clippy_check(
            "generate_postgresql_types_test_content",
            "../postgresql_crud/postgresql_types/",
            r#"[dependencies]
chrono.workspace = true
uuid.workspace = true
sqlx.workspace = true
serde.workspace = true
thiserror.workspace = true
error_occurence_lib = {path = "../../../error_occurence_lib"}
postgresql_crud_common = {path = "../../postgresql_crud_common"}
postgresql_types_common = {path = "../postgresql_types_common"}
where_filters = {path = "../../where_filters"}

[features]
test-utils = []"#,
            &generate_postgresql_types_source::generate_postgresql_types(
                quote::quote! {
                    {
                        "postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_types": "False",
                        "whole_content_write_into_generate_postgresql_types": "False",
                        "variant": "All"
                    }
                }
            ).to_string()
        );
    }
}
