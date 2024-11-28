pub trait PostgresqlJsonType {
    type ToCreate<'a>: std::fmt::Debug 
        + Clone 
        + PartialEq 
        + Default 
        + serde::Serialize 
        + serde::Deserialize<'a> 
        + utoipa::ToSchema<'a> 
        + schemars::JsonSchema 
        + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    fn try_generate_postgresql_query_part_to_create(
        to_create: &Self::ToCreate<'_>,
        increment: &mut std::primitive::u64
    ) -> Result<std::string::String, PostgresqlJsonTypeTryGeneratePostgresqlQueryPartToCreateErrorNamed>;
    fn bind_value_to_postgresql_query_part_to_create<'a>(
        to_create: Self::ToCreate<'a>,
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
    type FieldReader<'a>: std::fmt::Debug
        + Clone
        + PartialEq
        + Default
        + serde::Serialize
        + serde::Deserialize<'a>
        + utoipa::ToSchema<'a>
        + schemars::JsonSchema
        + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type OptionsToRead<'a>: std::fmt::Debug + Clone + PartialEq + Default + serde::Serialize + serde::Deserialize<'a> + utoipa::ToSchema<'a> + schemars::JsonSchema + StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    fn generate_postgresql_query_part_to_read(
        field_reader: &Self::FieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        //todo remove this coz its used properly now
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str
    ) -> std::string::String;
    type OptionToUpdate<'a>: std::fmt::Debug
        + Clone
        + PartialEq
        + Default
        + serde::Serialize
        + serde::Deserialize<'a>
        + utoipa::ToSchema<'a>
        + schemars::JsonSchema
        + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed: std::fmt::Debug
        + std::error::Error;//thiserror::Error + error_occurence_lib::ErrorOccurence
    fn try_generate_postgresql_query_part_to_update(
        option_to_update: &Self::OptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64
    ) -> Result<std::string::String, Self::OptionToUpdateTryGeneratePostgresqlQueryPartErrorNamed>;
    fn bind_value_to_postgresql_query_part_to_update<'a>(
        option_to_update: Self::OptionToUpdate<'_>,
        query: sqlx::query::Query<'a, sqlx::Postgres,
        sqlx::postgres::PgArguments>
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>;
}




pub(crate) trait PostgresqlCrudBaseWrapType<'a> {
    type SelfType: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + std::fmt::Display
        + error_occurence_lib::ToStdStringString
        + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
        + crate::BindQuerySecond<'a>
        + crate::CreateTableQueryPart;
    type SelfColumnType: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + crate::generate_postgresql_query_part::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type SelfToCreateType: PostgresqlCrudBaseTypeSelfToCreateType<'a>;
    type SelToReadType: PostgresqlCrudBaseTypeSelfToReadType<'a>;
    type SelfToUpdateType: PostgresqlCrudBaseTypeSelfToUpdateType<'a>;
    type SelfWhereType: PostgresqlCrudBaseTypeSelfWhereType<'a>;
}


