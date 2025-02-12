#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    // postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    // postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdPrimitiveI16AsPostgresqlInt2(std::primitive::i16);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI16AsPostgresqlInt2 {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} int2")
    }
}