#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        use quote::quote;
        macro_clippy_check_common::clippy_check(
            "gen_postgres_json_object_type_test_content",
            "../postgres_crud/postgres_json_object_type/",
            r#"[dependencies]
sqlx.workspace = true
serde.workspace = true
serde_json.workspace = true
schemars.workspace = true
utoipa.workspace = true
uuid.workspace = true
thiserror.workspace = true
error_occurence_lib = {path = "../../../error_occurence_lib"}
postgres_crud = {path = "../../../postgres_crud", features = ["test-utils"]}

[features]
test-utils = []"#,
            &{
                let object_example_ts = quote!{
                    #[derive(Debug, Clone, Copy)]
                    #[postgres_crud::postgres_json_object_type_config{
                        {
                            "postgres_table_columns_content_write_into_postgres_table_columns_using_postgres_json_object_types": "False",
                            "whole_content_write_into_gen_postgres_json_object_type": "False",
                            "variant": {
                                "not_null_or_nullable": "Nullable",
                                "pattern": "Array",
                                "trait_gen": "PostgresTypeAndPostgresJsonType"
                            }
                        }
                    }]
                    pub struct ObjectExample {
                        pub field_0: postgres_crud::StdPrimitiveI8AsNotNullJsonbNumber,
                        pub field_1: postgres_crud::OptionStdPrimitiveI8AsNullableJsonbNumber,
                        pub field_2: postgres_crud::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
                    }
                };
                let ts = gen_postgres_json_object_type_source::gen_postgres_json_object_type(object_example_ts.clone());
                quote! {
                    #ts
                    #object_example_ts
                }
            }
            .to_string()
        );
    }
}
