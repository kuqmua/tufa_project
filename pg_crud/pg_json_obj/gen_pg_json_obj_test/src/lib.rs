#[cfg(test)]
mod tests {
    use macro_clippy_check_common::clippy_check;
    use quote::quote;
    #[test]
    fn clippy() {
        clippy_check(
            "gen_pg_json_obj_test_cnt",
            "../pg_crud/pg_json_obj/",
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
optml = {path = "../../../optml"}
[features]
test-utils = []"#,
            &{
                let object_example_ts = quote!{
                    #[derive(Debug, Clone, Copy, optml::Optml)]
                    #[pg_crud::pg_json_obj_config{
                        {
                            "pg_table_columns_write_into_pg_table_columns_using_pg_json_objs": "False",
                            "whole_write_into_gen_pg_json_obj": "False",
                            "vrt": {
                                "is_nullable": "True",
                                "pattern": "Arr",
                                "trait_gen": "PgTypeAndPgJson"
                            }
                        }
                    }]
                    pub struct ObjectExample {
                        pub field_0: pg_crud::I8AsNotNullJsonbNbr,
                        pub field_1: pg_crud::OptI8AsNullableJsonbNbr,
                        pub field_2: pg_crud::VecOfI8AsNotNullArrOfNotNullJsonbNbr,
                    }
                };
                let ts = gen_pg_json_obj_src::gen_pg_json_obj(object_example_ts.clone());
                quote! {
                    #ts
                    #object_example_ts
                }
            }
            .to_string()
        );
    }
}
