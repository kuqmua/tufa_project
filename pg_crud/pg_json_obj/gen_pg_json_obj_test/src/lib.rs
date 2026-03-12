#[cfg(test)]
mod tests {
    use macro_clippy_check_cmn::clippy_check;
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
                let obj_example_ts = quote! {
                    #[derive(Debug, Clone, Copy, optml::Optml)]
                    #[pg_crud::pg_json_obj_config{
                        {
                            "pg_tbl_columns_write_into_pg_tbl_columns_using_pg_json_objs": "False",
                            "whole_write_into_gen_pg_json_obj": "False",
                            "vrt": {
                                "is_nl": "True",
                                "pattern": "Arr",
                                "trait_gen": "PgTypeAndPgJson"
                            }
                        }
                    }]
                    pub struct ObjExample {
                        pub field_0: pg_crud::I8AsNnJsonbNbr,
                        pub field_1: pg_crud::OptI8AsNlJsonbNbr,
                        pub field_2: pg_crud::VecOfI8AsNnArrOfNnJsonbNbr,
                    }
                };
                let ts = gen_pg_json_obj_src::gen_pg_json_obj(obj_example_ts.clone());
                quote! {
                    #ts
                    #obj_example_ts
                }
            }
            .to_string(),
        );
    }
}
