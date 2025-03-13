#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateQueryPartErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
pub trait PostgresqlJsonType {
    type Create<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    fn create_query_part(value: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, CreateQueryPartErrorNamed>;
    fn create_query_bind<'a>(value: Self::Create<'a>, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
    type FieldReader<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type OptionsToRead<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    fn field_reader_query_part(
        value: &Self::FieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        //todo remove this coz its used properly now
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_postgresql_type: std::primitive::bool,
    ) -> std::string::String;
    type WhereElement<'a>: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        // + schemars::JsonSchema //todo
        + crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter
        + crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type Where: std::fmt::Debug + crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type Update<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type UpdateQueryPartErrorNamed: std::fmt::Debug + std::error::Error; //thiserror::Error + error_occurence_lib::ErrorOccurence
    fn update_query_part(value: &Self::Update<'_>, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, Self::UpdateQueryPartErrorNamed>;
    fn update_query_bind<'a>(value: Self::Update<'_>, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}
