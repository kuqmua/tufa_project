#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        use quote::quote;
        macro_clippy_check_common::clippy_check(
            "generate_postgresql_json_object_type_test_content",
            "../postgresql_crud/postgresql_json_object_type/",
            r#"[dependencies]
sqlx.workspace = true
serde.workspace = true
serde_json.workspace = true
schemars.workspace = true
utoipa.workspace = true
uuid.workspace = true
thiserror.workspace = true
error_occurence_lib = {path = "../../../error_occurence_lib"}
postgresql_crud = {path = "../../../postgresql_crud", features = ["test-utils"]}

[features]
test-utils = []"#,
            &{
                let object_example_ts = quote!{
                    #[derive(Debug, Clone, Copy)]
                    #[postgresql_crud::postgresql_json_object_type_config{
                        {
                            "postgresql_table_columns_content_write_into_postgresql_table_columns_using_postgresql_json_object_types": "False",
                            "whole_content_write_into_generate_postgresql_json_object_type": "False",
                            "variant": {
                                "not_null_or_nullable": "Nullable",
                                "pattern": "Array",
                                "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
                            }
                        }
                    }]
                    pub struct ObjectExample {
                        pub field_0: postgresql_crud::StdPrimitiveI8AsNotNullJsonbNumber,
                        pub field_1: postgresql_crud::OptionStdPrimitiveI8AsNullableJsonbNumber,
                        pub field_2: postgresql_crud::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
                    }
                };
                let ts = generate_postgresql_json_object_type_source::generate_postgresql_json_object_type(object_example_ts.clone());
                quote! {
                    #ts
                    #object_example_ts
                }
            }
            .to_string()
        );
    }
}
