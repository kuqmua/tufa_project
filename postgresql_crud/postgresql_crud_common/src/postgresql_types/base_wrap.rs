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
pub(crate) trait PostgresqlCrudBaseTypeSelfWhereType<'a>: std::fmt::Debug
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
pub(crate) trait PostgresqlCrudBaseWrapTypePrimaryKey<'a> {
    type SelfToCreate: PostgresqlCrudBaseTypeSelfToCreateType<'a>
        + sqlx::Decode<'a, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>;
    type SelfToRead: PostgresqlCrudBaseTypeSelfToReadType<'a>
        + crate::BindQuerySecond<'a>
        + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type SelfToUpdate: PostgresqlCrudBaseTypeSelfToUpdateType<'a>
        + std::fmt::Display
        + error_occurence_lib::ToStdStringString
        + sqlx::Encode<'a, sqlx::Postgres>
        + sqlx::Decode<'a, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>;
    type SelfToDelete: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + crate::BindQuerySecond<'a>
        + std::fmt::Display
        + error_occurence_lib::ToStdStringString
        + sqlx::Decode<'a, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>
        + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
}

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
        crate::postgresql_types::base::StdOptionOptionStdPrimitiveBool::create_table_query_part_handle(&"BOOL")
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
        crate::postgresql_types::base::StdPrimitiveBool::create_table_query_part_handle(&"BOOL")
    }
}

///////////////////
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseWrapTypeTokens
)]
pub struct StdPrimitiveI16AsPostgresqlSmallInt(crate::postgresql_types::base::StdOptionOptionStdPrimitiveI16);
impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlSmallInt {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI16AsPostgresqlSmallIntNotNull(crate::postgresql_types::base::StdPrimitiveI16);
impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlSmallIntNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI16AsPostgresqlSmallSerial(crate::postgresql_types::base::StdOptionOptionStdPrimitiveI16);
impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlSmallSerial {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI16AsPostgresqlSmallSerialNotNull(crate::postgresql_types::base::StdPrimitiveI16);
impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlSmallSerialNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI16AsPostgresqlInt2(crate::postgresql_types::base::StdOptionOptionStdPrimitiveI16);
impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlInt2 {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI16AsPostgresqlInt2NotNull(crate::postgresql_types::base::StdPrimitiveI16);
impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlInt2NotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI32AsPostgresqlInt(crate::postgresql_types::base::StdOptionOptionStdPrimitiveI32);
impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlInt {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI32AsPostgresqlIntNotNull(crate::postgresql_types::base::StdPrimitiveI32);
impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlIntNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI32AsPostgresqlSerial(crate::postgresql_types::base::StdOptionOptionStdPrimitiveI32);
impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlSerial {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI32AsPostgresqlSerialNotNull(crate::postgresql_types::base::StdPrimitiveI32);
impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlSerialNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI32AsPostgresqlInt4(crate::postgresql_types::base::StdOptionOptionStdPrimitiveI32);
impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlInt4 {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI32AsPostgresqlInt4NotNull(crate::postgresql_types::base::StdPrimitiveI32);
impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlInt4NotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI64AsPostgresqlBigInt(crate::postgresql_types::base::StdOptionOptionStdPrimitiveI64);
impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigInt {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI64AsPostgresqlBigIntNotNull(crate::postgresql_types::base::StdPrimitiveI64);
impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigIntNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveI64AsPostgresqlBigSerial(crate::postgresql_types::base::StdOptionOptionStdPrimitiveI64);
impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigSerial {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveF32AsPostgresqlReal(crate::postgresql_types::base::StdOptionOptionStdPrimitiveF32);
impl crate::CreateTableQueryPart for StdPrimitiveF32AsPostgresqlReal {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveF32AsPostgresqlRealNotNull(crate::postgresql_types::base::StdPrimitiveF32);
impl crate::CreateTableQueryPart for StdPrimitiveF32AsPostgresqlRealNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveF32AsPostgresqlFloat4(crate::postgresql_types::base::StdOptionOptionStdPrimitiveF32);
impl crate::CreateTableQueryPart for StdPrimitiveF32AsPostgresqlFloat4 {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveF32AsPostgresqlFloat4NotNull(crate::postgresql_types::base::StdPrimitiveF32);
impl crate::CreateTableQueryPart for StdPrimitiveF32AsPostgresqlFloat4NotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveF64AsPostgresqlDoublePrecision(crate::postgresql_types::base::StdOptionOptionStdPrimitiveF64);
impl crate::CreateTableQueryPart for StdPrimitiveF64AsPostgresqlDoublePrecision {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull(crate::postgresql_types::base::StdPrimitiveF64);
impl crate::CreateTableQueryPart for StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveF64AsPostgresqlFloat8(crate::postgresql_types::base::StdOptionOptionStdPrimitiveF64);
impl crate::CreateTableQueryPart for StdPrimitiveF64AsPostgresqlFloat8 {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdPrimitiveF64AsPostgresqlFloat8NotNull(crate::postgresql_types::base::StdPrimitiveF64);
impl crate::CreateTableQueryPart for StdPrimitiveF64AsPostgresqlFloat8NotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdStringStringAsPostgresqlVarchar(crate::postgresql_types::base::StdOptionOptionStdStringString);
impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlVarchar {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdStringStringAsPostgresqlVarcharNotNull(crate::postgresql_types::base::StdStringString);
impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlVarcharNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdStringStringAsPostgresqlCharN(crate::postgresql_types::base::StdOptionOptionStdStringString);
impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlCharN {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdStringStringAsPostgresqlCharNNotNull(crate::postgresql_types::base::StdStringString);
impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlCharNNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdStringStringAsPostgresqlText(crate::postgresql_types::base::StdOptionOptionStdStringString);
impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlText {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdStringStringAsPostgresqlTextNotNull(crate::postgresql_types::base::StdStringString);
impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlTextNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdStringStringAsPostgresqlCiText(crate::postgresql_types::base::StdOptionOptionStdStringString);
impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlCiText {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
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
pub struct StdStringStringAsPostgresqlCiTextNotNull(crate::postgresql_types::base::StdStringString);
impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlCiTextNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        ""
    }
}
///////////////////////


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
//todo exception for offset and limit for now
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
// // pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(crate::postgresql_types::base::StdPrimitiveI64);
// // pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKey(crate::postgresql_types::base::StdPrimitiveI64);
// pub struct StdPrimitiveI64AsPostgresqlInt8(crate::postgresql_types::base::StdOptionOptionStdPrimitiveI64);
// pub struct StdPrimitiveI64AsPostgresqlInt8NotNull(crate::postgresql_types::base::StdPrimitiveI64);