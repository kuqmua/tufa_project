pub trait PostgresqlTypeSelfWhereFilter {
    fn where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed>;
    fn where_bind_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

//maybe put analog\copy of BindQuery inside this trait?
pub trait PostgresqlType<'a> {
    type PostgresqlTypeSelf: std::fmt::Debug;
    type Column: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    //maybe move it into own trait?
    fn column_query_part(value: &Self::Column, column: &std::primitive::str) -> std::string::String;
    type Create: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::BindQuery<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type Read: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + sqlx::Decode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>;
    type Update: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type UpdateQueryPartErrorNamed: std::fmt::Debug; //todo + std::error::Error; //thiserror::Error + error_occurence_lib::ErrorOccurence
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, Self::UpdateQueryPartErrorNamed>;
    fn update_bind_query_part(value: Self::Update, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
    type WhereElement: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter;
    type Where: std::fmt::Debug + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    fn where_try_generate_bind_increments(value: &Self::Where, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed>;
    fn where_bind_value_to_query(value: Self::Where, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}
