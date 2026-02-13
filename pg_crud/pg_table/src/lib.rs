pub use gen_pg_table::*;

pub trait CombinationOfAppStateLogicTraits:
    app_state::GetEnableApiGitCommitCheck
    + app_state::GetMaximumSizeOfHttpBodyInBytes
    + app_state::GetSourcePlaceType
    + app_state::GetTimezone
    + app_state::GetPgPool
    + Send
    + Sync
{
}

#[must_use]
pub fn gen_create_many_query_string(
    table: &str,
    columns: &str,
    values: &str,
    columns_to_return: &str,
) -> String {
    format!("insert into {table} ({columns}) values {values} returning {columns_to_return}")
}
#[must_use]
pub fn gen_create_one_query_string(
    table: &str,
    columns: &str,
    values: &str,
    columns_to_return: &str,
) -> String {
    format!("insert into {table} ({columns}) values ({values}) returning {columns_to_return}")
}
#[must_use]
pub fn gen_read_many_query_string(table: &str, select_string: &str, where_string: &str) -> String {
    format!("select {select_string} from {table} {where_string}")
}
#[must_use]
pub fn gen_read_one_query_string(table: &str, select_string: &str, where_string: &str) -> String {
    format!("select {select_string} from {table} where {where_string}")
}
#[must_use]
pub fn gen_column_queals_value_comma_update_one_query_part(column: &str, value: &str) -> String {
    format!("{column} = {value},")
}
#[must_use]
pub fn gen_when_column_id_then_value_update_many_query_part(
    column: &str,
    id: &str,
    value: &str,
) -> String {
    format!("when {column} = {id} then {value} ")
}
#[must_use]
pub fn gen_column_equals_case_acc_else_column_end_comma_update_many_query_part(
    column: &str,
    acc: &str,
) -> String {
    format!("{column} = case {acc}else {column} end,")
}
//todo additional parameter for columns_to_return instead of primary_key_field_name in "returning {primary_key_field_name}""
#[must_use]
pub fn gen_update_many_query_string(
    table: &str,
    elements: &str,
    primary_key_field_name: &str,
    primary_keys: &str,
    columns_to_return: &str,
) -> String {
    format!(
        "update {table} set {elements} where {primary_key_field_name} in ({primary_keys}) returning {columns_to_return}"
    )
}
//todo additional parameter for columns_to_return instead of primary_key_field_name in "returning {primary_key_field_name}""
#[must_use]
pub fn gen_update_one_query_string(
    table: &str,
    columns: &str,
    primary_key_field_name: &str,
    primary_key_query_part: &str,
    columns_to_return: &str,
) -> String {
    format!(
        "update {table} set {columns} where {primary_key_field_name} = {primary_key_query_part} returning {columns_to_return}"
    )
}
#[must_use]
pub fn gen_delete_many_query_string(
    table: &str,
    where_string: &str,
    primary_key_field_name: &str,
) -> String {
    format!("delete from {table} {where_string} returning {primary_key_field_name}")
}
#[must_use]
pub fn gen_delete_one_query_string(table: &str, primary_key_field_name: &str) -> String {
    format!(
        "delete from {table} where {primary_key_field_name} = $1 returning {primary_key_field_name}"
    )
}
