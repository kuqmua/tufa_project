pub(crate) trait PostgresqlBaseTypeSelfTraits<'a>: std::fmt::Debug
    + Clone
    + PartialEq
    + serde::Serialize
    + serde::Deserialize<'a>
    + sqlx::Type<sqlx::Postgres>
    + sqlx::Decode<'a, sqlx::Postgres>
    + crate::BindQuery<'a>
    + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement {}
pub(crate) trait PostgresqlBaseType<'a> {
    type PostgresqlBaseTypeSelf: PostgresqlBaseTypeSelfTraits<'a>;
    type PostgresqlBaseTypeStdOptionOptionSelf: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + sqlx::Type<sqlx::Postgres>
        + sqlx::Decode<'a, sqlx::Postgres>
        + crate::BindQuery<'a>
        + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
}
pub(crate) trait PostgresqlBaseTypePrimaryKey<'a> {
    type PostgresqlBaseTypeSelf: PostgresqlBaseTypeSelfTraits<'a>
        + sqlx::Encode<'a, sqlx::Postgres>
        + sqlx::postgres::PgHasArrayType;
}