pub trait PostgresqlTypeSelfWhereFilter {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed>;
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}

//maybe put analog\copy of BindQuery inside this trait?
pub trait PostgresqlType<'a> {
    type PostgresqlTypeSelf: std::fmt::Debug;
    type SelfColumn: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    //maybe move it into own trait?
    fn self_column_query_part(self_column: &Self::SelfColumn, column: &std::primitive::str) -> std::string::String;
    type SelfToCreate: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::BindQuery<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type SelfToRead: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + sqlx::Decode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>;
    type SelfToUpdate: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type SelfToUpdateQueryPartErrorNamed: std::fmt::Debug; // + std::error::Error; //thiserror::Error + error_occurence_lib::ErrorOccurence
    fn self_to_update_query_part(
        self_to_update: &Self::SelfToUpdate,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::SelfToUpdateQueryPartErrorNamed>;
    fn self_to_update_bind_query_part(self_to_update: Self::SelfToUpdate, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
    type PostgresqlTypeSelfWhereElement: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter;
    type PostgresqlTypeSelfWhere: std::fmt::Debug + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    fn postgresql_type_self_where_try_generate_bind_increments(
        postgresql_type_self_where: &Self::PostgresqlTypeSelfWhere,
        increment: &mut std::primitive::u64,
        column: &dyn std::fmt::Display,
        is_need_to_add_logical_operator: std::primitive::bool,
    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed>;
    fn postgresql_type_self_where_bind_value_to_query(postgresql_type_self_where: Self::PostgresqlTypeSelfWhere, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}
pub trait PostgresqlTypePrimaryKey<'a> {
    type SelfToCreate: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + sqlx::Type<sqlx::Postgres>
        + sqlx::postgres::PgHasArrayType
        + sqlx::Encode<'a, sqlx::Postgres>
        + sqlx::Decode<'a, sqlx::Postgres>
        + crate::BindQuery<'a>
        + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type SelfToRead: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + sqlx::Type<sqlx::Postgres>
        + sqlx::postgres::PgHasArrayType
        + sqlx::Encode<'a, sqlx::Postgres>
        + sqlx::Decode<'a, sqlx::Postgres>
        + crate::BindQuery<'a>
        + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type PostgresqlTypeSelfToUpdate: std::fmt::Debug
        + Clone
        + std::fmt::Display
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + sqlx::Type<sqlx::Postgres>
        + sqlx::postgres::PgHasArrayType
        + sqlx::Encode<'a, sqlx::Postgres>
        + sqlx::Decode<'a, sqlx::Postgres>
        + error_occurence_lib::ToStdStringString
        + crate::BindQuery<'a>
        + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type PostgresqlTypeSelfToDelete: std::fmt::Debug
        + Clone
        + std::fmt::Display
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + sqlx::Type<sqlx::Postgres>
        + sqlx::postgres::PgHasArrayType
        + sqlx::Encode<'a, sqlx::Postgres>
        + sqlx::Decode<'a, sqlx::Postgres>
        + error_occurence_lib::ToStdStringString
        + crate::BindQuery<'a>
        + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
}
