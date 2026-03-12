#[cfg(test)]
mod tests {
    use macro_clippy_check_cmn::clippy_check;
    use quote::quote;
    #[test]
    fn clippy() {
        clippy_check(
            "gen_pg_json_test_cnt",
            "../pg_crud/pg_json/",
            r#"[dependencies]
schemars.workspace = true
regex.workspace = true
uuid.workspace = true
sqlx.workspace = true
serde.workspace = true
utoipa.workspace = true
loc_lib = {path = "../../../loc_lib"}
pg_crud_cmn = {path = "../../pg_crud_cmn"}
gen_pg_json_cmn = {path = "../gen_pg_json_cmn"}
wh_flts = {path = "../../wh_flts"}
optml = {path = "../../../optml"}
[dev-dependencies]
uuid.workspace = true
[features]
test-utils = []"#,
            &gen_pg_json_src::gen_pg_json(&quote! {
                {
                    "pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json": "False",
                    "whole_cnt_write_into_gen_pg_json": "False",
                    "vrt": "WithDimOne"
                }
            })
            .to_string(),
        );
    }
}
