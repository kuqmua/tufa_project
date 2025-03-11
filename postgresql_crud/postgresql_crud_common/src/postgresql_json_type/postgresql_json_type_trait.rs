#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum TryGenerateToCreateErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
pub trait PostgresqlJsonType {
    type Create<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    fn try_generate_create(postgresql_json_type_self_to_create: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, TryGenerateToCreateErrorNamed>;
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::Create<'a>, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
    type SelfFieldReader<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type SelfOptionsToRead<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    fn generate_postgresql_json_type_to_read(
        postgresql_json_type_self_field_reader: &Self::SelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        //todo remove this coz its used properly now
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_postgresql_type: std::primitive::bool,
    ) -> std::string::String;
    //new
    type SelfWhereElement<'a>: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        // + schemars::JsonSchema //todo
        + crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter
        + crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type SelfWhere: std::fmt::Debug + crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    //endnew
    type SelfOptionToUpdate<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type SelfOptionToUpdateTryGenerateErrorNamed: std::fmt::Debug + std::error::Error; //thiserror::Error + error_occurence_lib::ErrorOccurence
    fn try_generate_postgresql_json_type_to_update(
        postgresql_json_type_self_option_to_update: &Self::SelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::SelfOptionToUpdateTryGenerateErrorNamed>;
    fn bind_value_to_postgresql_query_part_to_update<'a>(postgresql_json_type_self_option_to_update: Self::SelfOptionToUpdate<'_>, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}
