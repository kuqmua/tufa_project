#[cfg(test)]
mod tests {
    use macro_clippy_check_cmn::clippy_check;
    use quote::quote;
    #[test]
    fn clippy() {
        clippy_check(
            "gen_pg_types_test_cnt",
            "../pg_crud/pg_types/",
            r#"[dependencies]
chrono.workspace = true
uuid.workspace = true
sqlx.workspace = true
serde.workspace = true
thiserror.workspace = true
location_lib = {path = "../../../location_lib"}
pg_crud_cmn = {path = "../../pg_crud_cmn"}
pg_types_cmn = {path = "../pg_types_cmn"}
wh_filters = {path = "../../wh_filters"}
optml = {path = "../../../optml"}
[features]
test-utils = []"#,
            &gen_pg_types_src::gen_pg_types(&quote! {
                {
                    "pg_tbl_columns_write_into_file": "False",
                    "whole_write_into_file": "False",
                    "vrt": "All"
                }
            })
            .to_string(),
        );
    }
}
