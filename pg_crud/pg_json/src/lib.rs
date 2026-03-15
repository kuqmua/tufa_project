#[cfg(feature = "all-dims")]
gen_pg_json::gen_pg_json!({
    "pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json": "False",
    "whole_cnt_write_into_gen_pg_json": "False",
    "vrt": "All"
});
#[cfg(all(feature = "dim4", not(feature = "all-dims")))]
gen_pg_json::gen_pg_json!({
    "pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json": "False",
    "whole_cnt_write_into_gen_pg_json": "False",
    "vrt": "WithDimFour"
});
#[cfg(all(feature = "dim3", not(feature = "dim4")))]
gen_pg_json::gen_pg_json!({
    "pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json": "False",
    "whole_cnt_write_into_gen_pg_json": "False",
    "vrt": "WithDimThree"
});
#[cfg(all(feature = "dim2", not(feature = "dim3")))]
gen_pg_json::gen_pg_json!({
    "pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json": "False",
    "whole_cnt_write_into_gen_pg_json": "False",
    "vrt": "WithDimTwo"
});
#[cfg(all(feature = "dim1", not(feature = "dim2")))]
gen_pg_json::gen_pg_json!({
    "pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json": "False",
    "whole_cnt_write_into_gen_pg_json": "False",
    "vrt": "WithDimOne"
});
#[cfg(not(feature = "dim1"))]
gen_pg_json::gen_pg_json!({
    "pg_tbl_cols_cnt_write_into_pg_tbl_cols_using_pg_json": "False",
    "whole_cnt_write_into_gen_pg_json": "False",
    "vrt": "WithoutDims"
});
