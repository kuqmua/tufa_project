#[cfg(test)]
mod tests {
    use macro_clippy_check_common::clippy_check;
    use quote::quote;
    #[test]
    fn clippy() {
        clippy_check(
            "gen_pg_json_object_type_test_content",
            "../pg_crud/pg_json_object_type/",
            r#"[dependencies]
sqlx.workspace = true
serde.workspace = true
serde_json.workspace = true
schemars.workspace = true
utoipa.workspace = true
uuid.workspace = true
thiserror.workspace = true
location_lib = {path = "../../../location_lib"}
pg_crud = {path = "../../../pg_crud", features = ["test-utils"]}
[features]
test-utils = []"#,
            &{
                let object_example_ts = quote!{
                    #[derive(Debug, Clone, Copy)]
                    #[pg_crud::pg_json_object_type_config{
                        {
                            "pg_table_columns_content_write_into_pg_table_columns_using_pg_json_object_types": "False",
                            "whole_content_write_into_gen_pg_json_object_type": "False",
                            "variant": {
                                "is_nullable": "True",
                                "pattern": "Array",
                                "trait_gen": "PgTypeAndPgJsonType"
                            }
                        }
                    }]
                    pub struct ObjectExample {
                        pub field_0: pg_crud::I8AsNotNullJsonbNumber,
                        pub field_1: pg_crud::OptionI8AsNullableJsonbNumber,
                        pub field_2: pg_crud::VecOfI8AsNotNullArrayOfNotNullJsonbNumber,
                    }
                };
                let ts = gen_pg_json_object_type_source::gen_pg_json_object_type(object_example_ts.clone());
                quote! {
                    #ts
                    #object_example_ts
                }
            }
            .to_string()
        );
    }
}
