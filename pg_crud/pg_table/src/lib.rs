use app_state::{
    GetEnableApiGitCommitCheck, GetMaximumSizeOfHttpBodyInBytes, GetPgPool, GetSourcePlaceType,
    GetTimezone,
};
pub use gen_pg_table::*;
pub trait CombinationOfAppStateLogicTraits:
    GetEnableApiGitCommitCheck
    + GetMaximumSizeOfHttpBodyInBytes
    + GetSourcePlaceType
    + GetTimezone
    + GetPgPool
    + Send
    + Sync
{
}
#[must_use]
pub fn gen_cm_query_string(
    table: &str,
    columns: &str,
    values: &str,
    columns_to_return: &str,
) -> String {
    format!("insert into {table} ({columns}) values {values} returning {columns_to_return}")
}
#[must_use]
pub fn gen_co_query_string(
    table: &str,
    columns: &str,
    values: &str,
    columns_to_return: &str,
) -> String {
    format!("insert into {table} ({columns}) values ({values}) returning {columns_to_return}")
}
#[must_use]
pub fn gen_rm_query_string(table: &str, select_string: &str, where_string: &str) -> String {
    format!("select {select_string} from {table} {where_string}")
}
#[must_use]
pub fn gen_ro_query_string(table: &str, select_string: &str, where_string: &str) -> String {
    format!("select {select_string} from {table} where {where_string}")
}
#[must_use]
pub fn gen_column_queals_v_comma_uo_qp(column: &str, value: &str) -> String {
    format!("{column} = {value},")
}
#[must_use]
pub fn gen_when_column_id_then_v_um_qp(column: &str, id: &str, value: &str) -> String {
    format!("when {column} = {id} then {value} ")
}
#[must_use]
pub fn gen_column_equals_case_acc_else_column_end_comma_um_qp(column: &str, acc: &str) -> String {
    format!("{column} = case {acc}else {column} end,")
}
//todo extra param for columns_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_um_query_string(
    table: &str,
    els: &str,
    pk_field_name: &str,
    pks: &str,
    columns_to_return: &str,
) -> String {
    format!(
        "update {table} set {els} where {pk_field_name} in ({pks}) returning {columns_to_return}"
    )
}
//todo extra param for columns_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_uo_query_string(
    table: &str,
    columns: &str,
    pk_field_name: &str,
    pk_qp: &str,
    columns_to_return: &str,
) -> String {
    format!(
        "update {table} set {columns} where {pk_field_name} = {pk_qp} returning {columns_to_return}"
    )
}
#[must_use]
pub fn gen_dm_query_string(table: &str, where_string: &str, pk_field_name: &str) -> String {
    format!("delete from {table} {where_string} returning {pk_field_name}")
}
#[must_use]
pub fn gen_dlo_query_string(table: &str, pk_field_name: &str) -> String {
    format!("delete from {table} where {pk_field_name} = $1 returning {pk_field_name}")
}
