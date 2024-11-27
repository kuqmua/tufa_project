pub(crate) trait PostgresqlCrudBaseTypeSelfToCreateType<'a>: std::fmt::Debug
    + Clone
    + PartialEq
    + serde::Serialize
    + serde::Deserialize<'a>
    + crate::BindQuerySecond<'a>
    + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement {}
pub(crate) trait PostgresqlCrudBaseTypeSelfToReadType<'a>: std::fmt::Debug
    + Clone
    + PartialEq
    + serde::Serialize
    + serde::Deserialize<'a>
    + sqlx::Decode<'a, sqlx::Postgres>
    + sqlx::Type<sqlx::Postgres> {}
pub(crate) trait PostgresqlCrudBaseTypeSelfToUpdateType<'a>: std::fmt::Debug
    + Clone
    + PartialEq
    + serde::Serialize
    + serde::Deserialize<'a>
    + crate::BindQuerySecond<'a>
    + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement {}
pub(crate) trait PostgresqlCrudBaseTypeSelfToDeleteType<'a>: std::fmt::Debug
    + Clone
    + PartialEq
    + serde::Serialize
    + serde::Deserialize<'a>
    + crate::BindQuerySecond<'a>
    + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement{}

pub(crate) trait PostgresqlCrudBaseWrapType<'a> {
    type SelfType: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + std::fmt::Display
        + error_occurence_lib::ToStdStringString
        + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
        + crate::BindQuerySecond<'a>;
    type SelfColumnType: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + crate::generate_postgresql_query_part::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type SelfToCreateType: PostgresqlCrudBaseTypeSelfToCreateType<'a>;
    type SelToReadType: PostgresqlCrudBaseTypeSelfToReadType<'a>;
    type SelfToUpdateType: PostgresqlCrudBaseTypeSelfToUpdateType<'a>;
    type SelfWhereType: PostgresqlCrudBaseTypeSelfToDeleteType<'a>;
}
// pub(crate) trait PostgresqlCrudBaseWrapTypePrimaryKey<'a> {
//     type SelfToCreate: std::fmt::Debug
//         + Clone
//         + PartialEq
//         + serde::Serialize
//         + serde::Deserialize<'a>
//         + sqlx::Decode<'a, sqlx::Postgres>
//         + sqlx::Type<sqlx::Postgres>
//         + crate::BindQuerySecond<'a>
//         + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//     type SelfToRead
//     type SelfToUpdate
//     type SelfToDelete

        
        
        
        
        
//         ;
// }

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseWrapTypeTokens
)]
pub struct StdPrimitiveBoolAsPostgresqlBool(crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool);
impl crate::CreateTableQueryPart for StdPrimitiveBoolAsPostgresqlBool {
    fn create_table_query_part() -> impl std::fmt::Display {
        "BOOL"
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseWrapTypeTokens
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(crate::postgresql_types::base::StdPrimitiveBool);
impl crate::CreateTableQueryPart for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        "BOOL NOT NULL"
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseWrapTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseWrapTypeTokensPrimaryKey,
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(crate::postgresql_types::base::StdPrimitiveI64);
impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        "BIGSERIAL"
    }
}
//exception for offset and limit for now
const _: () = {
    impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
        fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
        }
    }
    impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
        fn type_info() -> sqlx::postgres::PgTypeInfo {
            <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::postgres::PgHasArrayType for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
            <crate::postgresql_types::base::StdPrimitiveI64 as sqlx::postgres::PgHasArrayType>::array_type_info()
        }
    }
};