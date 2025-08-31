pub use generate_postgresql_table::*;

pub fn generate_create_many_query_string(table: &std::primitive::str, columns: &std::primitive::str, values: std::string::String, columns_to_return: &std::primitive::str) -> std::string::String {
    format!("insert into {table} ({columns}) values {values} returning {columns_to_return}")
}
pub fn generate_create_one_query_string(table: &std::primitive::str, columns: &std::primitive::str, values: std::string::String, columns_to_return: &std::primitive::str) -> std::string::String {
    format!("insert into {table} ({columns}) values ({values}) returning {columns_to_return}")
}
pub fn generate_read_many_query_string(table: &std::primitive::str, select_string: std::string::String, where_string: std::string::String) -> std::string::String {
    format!("select {select_string} from {table} {where_string}")
}
pub fn generate_read_one_query_string(table: &std::primitive::str, select_string: std::string::String, where_string: std::string::String) -> std::string::String {
    format!("select {select_string} from {table} where {where_string}")
}
pub fn generate_column_queals_value_comma_update_one_query_part(column: &std::primitive::str, value: std::string::String) -> std::string::String {
    format!("{column} = {value},")
}
pub fn generate_when_column_id_then_value_update_many_query_part(column: &std::primitive::str, id: std::string::String, value: std::string::String) -> std::string::String {
    format!("when {column} = {id} then {value} ")
}
pub fn generate_column_equals_case_acc_else_column_end_comma_update_many_query_part(column: &std::primitive::str, acc: std::string::String) -> std::string::String {
    format!("{column} = case {acc}else {column} end,")
}
//todo additional parameter for columns_to_return instead of primary_key_field_name in "returning {primary_key_field_name}""
pub fn generate_update_many_query_string(table: &std::primitive::str, elements: std::string::String, primary_key_field_name: &std::primitive::str, primary_keys: std::string::String) -> std::string::String {
    format!("update {table} set {elements} where {primary_key_field_name} in ({primary_keys}) returning {primary_key_field_name}")
}
//todo additional parameter for columns_to_return instead of primary_key_field_name in "returning {primary_key_field_name}""
pub fn generate_update_one_query_string(table: &std::primitive::str, columns: std::string::String, primary_key_field_name: &std::primitive::str, primary_key_query_part: std::string::String, columns_to_return: &std::primitive::str) -> std::string::String {
    format!("update {table} set {columns} where {primary_key_field_name} = {primary_key_query_part} returning {columns_to_return}")
}
pub fn generate_delete_many_query_string(table: &std::primitive::str, where_string: std::string::String, primary_key_field_name: &std::primitive::str) -> std::string::String {
    format!("delete from {table} {where_string} returning {primary_key_field_name}")
}
pub fn generate_delete_one_query_string(table: &std::primitive::str, primary_key_field_name: &std::primitive::str) -> std::string::String {
    format!("delete from {table} where {primary_key_field_name} = $1 returning {primary_key_field_name}")
}