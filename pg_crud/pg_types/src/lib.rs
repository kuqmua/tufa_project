//todo
pub use pg_types_cmn::{PgnStartsWithOne, PgnStartsWithOneTryNewEr};
#[cfg(feature = "all-dims")]
gen_pg_types::gen_pg_types!({
    "pg_tbl_cols_write_into_file": "False",
    "whole_write_into_file": "False",
    "vrt": "All"
});
#[cfg(all(feature = "dim1", not(feature = "all-dims")))]
gen_pg_types::gen_pg_types!({
    "pg_tbl_cols_write_into_file": "False",
    "whole_write_into_file": "False",
    "vrt": "WithDimOne"
});
#[cfg(not(feature = "dim1"))]
gen_pg_types::gen_pg_types!({
    "pg_tbl_cols_write_into_file": "False",
    "whole_write_into_file": "False",
    "vrt": "WithoutDims"
});
