#[cfg(test)]
mod tests {
    use macro_clippy_check_cmn::clippy_check;
    use quote::quote;
    #[test]
    fn clippy() {
        clippy_check(
            "gen_wh_flts_test_cnt",
            "../pg_crud/wh_flts/",
            r#"[dependencies]
sqlx.workspace = true
serde.workspace = true
schemars.workspace = true
utoipa.workspace = true
regex.workspace = true
thiserror.workspace = true
loc_lib = {path = "../../../loc_lib"}
pg_crud_cmn = {path = "../../pg_crud_cmn"}
wh_flts = {path = "../../wh_flts"}
optml = {path = "../../../optml"}
[features]
test-utils = []"#,
            &format!(
                "#![allow(clippy::absolute_paths)]\n#![allow(clippy::arbitrary_source_item_ordering)]\n#![allow(clippy::let_underscore_untyped)]\n#![allow(clippy::equatable_if_let)]\nuse wh_flts::{{BoundedVec, Btwn, PgTypeNotEmptyUnqVec, RgxCase, RgxRgx, EncodeFormat, PgJsonNotEmptyUnqVec}};\n{}",
                gen_wh_flts_src::gen_wh_flts(&quote! {
                    {
                        "pg_types_write_into_file": "False",
                        "pg_json_write_into_file": "False",
                        "whole_write_into_file": "False"
                    }
                })
            ),
        );
    }
}
