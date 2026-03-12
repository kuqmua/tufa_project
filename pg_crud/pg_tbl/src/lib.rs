use app_state::{
    GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetPgPool, GetSrcPlaceType,
    GetTimezone,
};
pub use gen_pg_tbl::*;
pub trait CombinationOfAppStateLogicTraits:
    GetEnableApiGitCommitCheck
    + GetMaximumSizeOfHttpBodyInBytes
    + GetSrcPlaceType
    + GetTimezone
    + GetPgPool
    + Send
    + Sync
{
}
#[must_use]
pub fn gen_cm_query_string(tbl: &str, cols: &str, values: &str, cols_to_return: &str) -> String {
    format!("insert into {tbl} ({cols}) values {values} returning {cols_to_return}")
}
#[must_use]
pub fn gen_co_query_string(tbl: &str, cols: &str, values: &str, cols_to_return: &str) -> String {
    format!("insert into {tbl} ({cols}) values ({values}) returning {cols_to_return}")
}
#[must_use]
pub fn gen_rm_query_string(tbl: &str, sel_string: &str, wh_string: &str) -> String {
    format!("select {sel_string} from {tbl} {wh_string}")
}
#[must_use]
pub fn gen_ro_query_string(tbl: &str, sel_string: &str, wh_string: &str) -> String {
    format!("select {sel_string} from {tbl} where {wh_string}")
}
#[must_use]
pub fn gen_col_queals_v_comma_uo_qp(col: &str, value: &str) -> String {
    format!("{col} = {value},")
}
#[must_use]
pub fn gen_when_col_id_then_v_um_qp(col: &str, id: &str, value: &str) -> String {
    format!("when {col} = {id} then {value} ")
}
#[must_use]
pub fn gen_col_eqs_case_acc_else_col_end_comma_um_qp(col: &str, acc: &str) -> String {
    format!("{col} = case {acc}else {col} end,")
}
//todo extra param for cols_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_um_query_string(
    tbl: &str,
    els: &str,
    pk_field_name: &str,
    pks: &str,
    cols_to_return: &str,
) -> String {
    format!("update {tbl} set {els} where {pk_field_name} in ({pks}) returning {cols_to_return}")
}
//todo extra param for cols_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_uo_query_string(
    tbl: &str,
    cols: &str,
    pk_field_name: &str,
    pk_qp: &str,
    cols_to_return: &str,
) -> String {
    format!("update {tbl} set {cols} where {pk_field_name} = {pk_qp} returning {cols_to_return}")
}
#[must_use]
pub fn gen_dm_query_string(tbl: &str, wh_string: &str, pk_field_name: &str) -> String {
    format!("delete from {tbl} {wh_string} returning {pk_field_name}")
}
#[must_use]
pub fn gen_dlo_query_string(tbl: &str, pk_field_name: &str) -> String {
    format!("delete from {tbl} where {pk_field_name} = $1 returning {pk_field_name}")
}
