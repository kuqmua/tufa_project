//maybe put analog\copy of BindQuery inside this trait?
pub trait PostgresqlType {
    type PostgresqlTypeSelf: std::fmt::Debug;
    type Create: std::fmt::Debug + Clone + PartialEq + serde::Serialize  + for<'__> serde::Deserialize<'__> + crate::BindQuery + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::QueryPartErrorNamed>;
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>;
    type Select: std::fmt::Debug + Clone + PartialEq + serde::Serialize + for<'__> serde::Deserialize<'__> + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    //maybe move it into own trait?
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String;
    type WhereElement: std::fmt::Debug + Clone + PartialEq + serde::Serialize  + for<'__> serde::Deserialize<'__> + for<'a> crate::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a>;
    type Read: std::fmt::Debug + Clone + PartialEq + serde::Serialize  + for<'__> serde::Deserialize<'__> + for<'__> sqlx::Decode<'__, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>;
    type Update: std::fmt::Debug + Clone + PartialEq + serde::Serialize  + for<'__> serde::Deserialize<'__> + crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement;
    type UpdateQueryPartErrorNamed: std::fmt::Debug; //todo + std::error::Error; //thiserror::Error + error_occurence_lib::ErrorOccurence
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, Self::UpdateQueryPartErrorNamed>;
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>;
    //todo delete? 
}

pub trait PostgresqlTypeSelfWhereFilter<'a> {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed>;
    fn where_query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}