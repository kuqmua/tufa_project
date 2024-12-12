pub trait PostgresqlTypeSelfToCreateTraits<'a>: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::BindQuerySecond<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement {}
pub trait PostgresqlTypeSelfToReadTraits<'a>: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + sqlx::Decode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> {}
pub trait PostgresqlTypeSelfToUpdateTraits<'a>: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::BindQuerySecond<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement {}
pub trait PostgresqlTypeSelfWhereTraits<'a>: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::BindQuerySecond<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement {}

//maybe put analog\copy of BindQuerySecond inside this trait?
pub trait PostgresqlType<'a> {
    type PostgresqlTypeSelf: std::fmt::Debug
        // + Clone
        // + PartialEq
        // + serde::Serialize
        // + serde::Deserialize<'a>
        // + std::fmt::Display
        // + error_occurence_lib::ToStdStringString
        // + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
        // + crate::BindQuerySecond<'a>
        + crate::CreateTableColumnQueryPart;
    type PostgresqlTypeSelfColumn: std::fmt::Debug + Clone + PartialEq + serde::Serialize + serde::Deserialize<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    //maybe move it into own trait?
    fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String;
    type PostgresqlTypeSelfToCreate: PostgresqlTypeSelfToCreateTraits<'a>;
    type PostgresqlTypeSelfToRead: PostgresqlTypeSelfToReadTraits<'a>;
    type PostgresqlTypeSelfToUpdate: PostgresqlTypeSelfToUpdateTraits<'a>;
    type PostgresqlTypeSelfToUpdateQueryPartErrorNamed: std::fmt::Debug;// + std::error::Error; //thiserror::Error + error_occurence_lib::ErrorOccurence
    fn postgresql_type_self_to_update_query_part(
        postgresql_type_self_to_update: &Self::PostgresqlTypeSelfToUpdate,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64
    ) -> Result<std::string::String, Self::PostgresqlTypeSelfToUpdateQueryPartErrorNamed>;
    fn postgresql_type_self_to_update_bind_query_part(
        postgresql_type_self_to_update: Self::PostgresqlTypeSelfToUpdate,
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
    type PostgresqlTypeSelfWhere: PostgresqlTypeSelfWhereTraits<'a>;
}
pub(crate) trait PostgresqlTypePrimaryKey<'a> {
    type PostgresqlTypeSelfToCreate: PostgresqlTypeSelfToCreateTraits<'a> + sqlx::Decode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>;
    type PostgresqlTypeSelfToRead: PostgresqlTypeSelfToReadTraits<'a> + crate::BindQuerySecond<'a> + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type PostgresqlTypeSelfToUpdate: PostgresqlTypeSelfToUpdateTraits<'a> + std::fmt::Display + error_occurence_lib::ToStdStringString + sqlx::Encode<'a, sqlx::Postgres> + sqlx::Decode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>;
    type PostgresqlTypeSelfToDelete: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + crate::BindQuerySecond<'a>
        + std::fmt::Display
        + error_occurence_lib::ToStdStringString
        + sqlx::Decode<'a, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>
        + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
}
