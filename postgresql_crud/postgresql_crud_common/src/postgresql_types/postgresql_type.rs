pub trait PostgresqlTypeSelfToCreateTraits<'a>: std::fmt::Debug
    + Clone
    + PartialEq
    + serde::Serialize
    + serde::Deserialize<'a>
    + crate::BindQuerySecond<'a>
    + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement {}
pub trait PostgresqlTypeSelfToReadTraits<'a>: std::fmt::Debug
    + Clone
    + PartialEq
    + serde::Serialize
    + serde::Deserialize<'a>
    + sqlx::Decode<'a, sqlx::Postgres>
    + sqlx::Type<sqlx::Postgres> {}
pub trait PostgresqlTypeSelfToUpdateTraits<'a>: std::fmt::Debug
    + Clone
    + PartialEq
    + serde::Serialize
    + serde::Deserialize<'a>
    + crate::BindQuerySecond<'a>
    + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement {}
pub trait PostgresqlTypeSelfWhereTraits<'a>: std::fmt::Debug
    + Clone
    + PartialEq
    + serde::Serialize
    + serde::Deserialize<'a>
    + crate::BindQuerySecond<'a>
    + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement{}

pub trait PostgresqlType<'a> {
    type PostgresqlTypeSelf: std::fmt::Debug
        // + Clone
        // + PartialEq
        // + serde::Serialize
        // + serde::Deserialize<'a>
        // + std::fmt::Display
        // + error_occurence_lib::ToStdStringString
        // + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
        // + crate::BindQuerySecond<'a>
        + crate::CreateTableQueryPart;
    type PostgresqlTypeSelfColumn: std::fmt::Debug
        + Clone
        + PartialEq
        + serde::Serialize
        + serde::Deserialize<'a>
        + crate::generate_postgresql_query_part::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type PostgresqlTypeSelfToCreate: PostgresqlTypeSelfToCreateTraits<'a>;
    type PostgresqlTypeSelfToRead: PostgresqlTypeSelfToReadTraits<'a>;
    type PostgresqlTypeSelfToUpdate: PostgresqlTypeSelfToUpdateTraits<'a>;
    type PostgresqlTypeSelfWhere: PostgresqlTypeSelfWhereTraits<'a>;
}
pub(crate) trait PostgresqlTypePrimaryKey<'a> {
    type PostgresqlTypeSelfToCreate: PostgresqlTypeSelfToCreateTraits<'a>
        + sqlx::Decode<'a, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>;
    type PostgresqlTypeSelfToRead: PostgresqlTypeSelfToReadTraits<'a>
        + crate::BindQuerySecond<'a>
        + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
    type PostgresqlTypeSelfToUpdate: PostgresqlTypeSelfToUpdateTraits<'a>
        + std::fmt::Display
        + error_occurence_lib::ToStdStringString
        + sqlx::Encode<'a, sqlx::Postgres>
        + sqlx::Decode<'a, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>;
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
        + crate::generate_postgresql_query_part::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement;
}


const BOOL: &std::primitive::str = "BOOL";
const CHAR: &std::primitive::str = "CHAR";
const SMALLINT: &std::primitive::str = "SMALLINT";
const SMALLSERIAL: &std::primitive::str = "SMALLSERIAL";
const INT2: &std::primitive::str = "INT2";
const INT: &std::primitive::str = "INT";
const SERIAL: &std::primitive::str = "SERIAL";
const INT4: &std::primitive::str = "INT4";
const BIGINT: &std::primitive::str = "BIGINT";
const BIGSERIAL: &std::primitive::str = "BIGSERIAL";
const INT8: &std::primitive::str = "INT8";
const REAL: &std::primitive::str = "REAL";
const FLOAT4: &std::primitive::str = "FLOAT4";
const DOUBLE_PRECISION: &std::primitive::str = "DOUBLE PRECISION";
const FLOAT8: &std::primitive::str = "FLOAT8";
const VARCHAR: &std::primitive::str = "VARCHAR";
// const CHARN: &std::primitive::str = "CHAR(N)";
const TEXT: &std::primitive::str = "TEXT";
// const CITEXT: &std::primitive::str = "CITEXT";
const BYTEA: &std::primitive::str = "BYTEA";
const INTERVAL: &std::primitive::str = "INTERVAL";
const INT8RANGE: &std::primitive::str = "INT8RANGE";
const INT4RANGE: &std::primitive::str = "INT4RANGE";
const TSRANGE: &std::primitive::str = "TSRANGE";
const TSTZRANGE: &std::primitive::str = "TSTZRANGE";
const DATERANGE: &std::primitive::str = "DATERANGE";
const NUMRANGE: &std::primitive::str = "NUMRANGE";
const MONEY: &std::primitive::str = "MONEY";
const NUMERIC: &std::primitive::str = "NUMERIC";
const TIMESTAMPTZ: &std::primitive::str = "TIMESTAMPTZ";
const DATE: &std::primitive::str = "DATE";
const TIME: &std::primitive::str = "TIME";
const TIMETZ: &std::primitive::str = "TIMETZ";
const TIMESTAMP: &std::primitive::str = "TIMESTAMP";
const UUID: &std::primitive::str = "UUID";
const INET: &std::primitive::str = "INET";
const CIDR: &std::primitive::str = "CIDR";
const MACADDR: &std::primitive::str = "MACADDR";
const BIT: &std::primitive::str = "BIT";
const VARBIT: &std::primitive::str = "VARBIT";
const JSON: &std::primitive::str = "JSON";
const JSONB: &std::primitive::str = "JSONB";

// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveBoolAsPostgresqlBool(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveBool);
// impl crate::CreateTableQueryPart for StdPrimitiveBoolAsPostgresqlBool {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&BOOL)
//     }
// }
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveBool);
impl crate::CreateTableQueryPart for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        Self::create_table_query_part_handle(&BOOL)
    }
}
impl crate::CreateTableColumnQueryPart for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} BOOL NOT NULL")
    }
}

// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI16AsPostgresqlSmallInt(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveI16);
// impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlSmallInt {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&SMALLINT)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI16AsPostgresqlSmallIntNotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveI16);
// impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlSmallIntNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&SMALLINT)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI16AsPostgresqlSmallSerial(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveI16);
// impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlSmallSerial {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&SMALLSERIAL)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI16AsPostgresqlSmallSerialNotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveI16);
// impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlSmallSerialNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&SMALLSERIAL)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI16AsPostgresqlInt2(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveI16);
// impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlInt2 {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&INT2)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI16AsPostgresqlInt2NotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveI16);
// impl crate::CreateTableQueryPart for StdPrimitiveI16AsPostgresqlInt2NotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&INT2)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI32AsPostgresqlInt(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveI32);
// impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlInt {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&INT)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI32AsPostgresqlIntNotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveI32);
// impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlIntNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&INT)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI32AsPostgresqlSerial(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveI32);
// impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlSerial {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&SERIAL)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI32AsPostgresqlSerialNotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveI32);
// impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlSerialNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&SERIAL)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI32AsPostgresqlInt4(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveI32);
// impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlInt4 {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&INT4)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI32AsPostgresqlInt4NotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveI32);
// impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlInt4NotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&INT4)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI64AsPostgresqlBigInt(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveI64);
// impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigInt {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&BIGINT)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI64AsPostgresqlBigIntNotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveI64);
// impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigIntNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&BIGINT)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI64AsPostgresqlBigSerial(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveI64);
// impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigSerial {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&BIGSERIAL)
//     }
// }
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveI64);
impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        Self::create_table_query_part_handle(&BIGSERIAL)
    }
}
impl crate::CreateTableColumnQueryPart for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} BIGSERIAL NOT NULL{}", crate::maybe_primary_key(is_primary_key))
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
            <crate::postgresql_types::postgresql_base_type::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::postgres::PgHasArrayType for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
            <crate::postgresql_types::postgresql_base_type::StdPrimitiveI64 as sqlx::postgres::PgHasArrayType>::array_type_info()
        }
    }
};
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveI64AsPostgresqlInt8(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveI64);
// impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlInt8 {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&INT8)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
// )]
// pub struct StdPrimitiveI64AsPostgresqlInt8NotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveI64);
// impl crate::CreateTableQueryPart for StdPrimitiveI64AsPostgresqlInt8NotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&INT8)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveF32AsPostgresqlReal(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveF32);
// impl crate::CreateTableQueryPart for StdPrimitiveF32AsPostgresqlReal {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&REAL)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveF32AsPostgresqlRealNotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveF32);
// impl crate::CreateTableQueryPart for StdPrimitiveF32AsPostgresqlRealNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&REAL)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveF32AsPostgresqlFloat4(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveF32);
// impl crate::CreateTableQueryPart for StdPrimitiveF32AsPostgresqlFloat4 {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&FLOAT4)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveF32AsPostgresqlFloat4NotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveF32);
// impl crate::CreateTableQueryPart for StdPrimitiveF32AsPostgresqlFloat4NotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&FLOAT4)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveF64AsPostgresqlDoublePrecision(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveF64);
// impl crate::CreateTableQueryPart for StdPrimitiveF64AsPostgresqlDoublePrecision {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&DOUBLE_PRECISION)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveF64);
// impl crate::CreateTableQueryPart for StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&DOUBLE_PRECISION)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveF64AsPostgresqlFloat8(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdPrimitiveF64);
// impl crate::CreateTableQueryPart for StdPrimitiveF64AsPostgresqlFloat8 {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&FLOAT8)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveF64AsPostgresqlFloat8NotNull(crate::postgresql_types::postgresql_base_type::StdPrimitiveF64);
// impl crate::CreateTableQueryPart for StdPrimitiveF64AsPostgresqlFloat8NotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&FLOAT8)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdStringStringAsPostgresqlVarchar(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdStringString);
// impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlVarchar {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&VARCHAR)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdStringStringAsPostgresqlVarcharNotNull(crate::postgresql_types::postgresql_base_type::StdStringString);
// impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlVarcharNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&VARCHAR)
//     }
// }
// // #[derive(
// //     Debug,
// //     Clone,
// //     PartialEq,
// //     serde::Serialize,
// //     serde::Deserialize,
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// // )]
// // pub struct StdStringStringAsPostgresqlCharN(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdStringString);
// // impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlCharN {
// //     fn create_table_query_part() -> impl std::fmt::Display {
// //         Self::create_table_query_part_handle(&CHARN)
// //     }
// // }
// // #[derive(
// //     Debug,
// //     Clone,
// //     PartialEq,
// //     serde::Serialize,
// //     serde::Deserialize,
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// // )]
// // pub struct StdStringStringAsPostgresqlCharNNotNull(crate::postgresql_types::postgresql_base_type::StdStringString);
// // impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlCharNNotNull {
// //     fn create_table_query_part() -> impl std::fmt::Display {
// //         Self::create_table_query_part_handle(&CHARN)
// //     }
// // }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdStringStringAsPostgresqlText(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdStringString);
// impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlText {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&TEXT)
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdStringStringAsPostgresqlTextNotNull(crate::postgresql_types::postgresql_base_type::StdStringString);
// impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlTextNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&TEXT)
//     }
// }
// // #[derive(
// //     Debug,
// //     Clone,
// //     PartialEq,
// //     serde::Serialize,
// //     serde::Deserialize,
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// // )]
// // pub struct StdStringStringAsPostgresqlCiText(crate::postgresql_types::postgresql_base_type::StdOptionOptionStdStringString);
// // impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlCiText {
// //     fn create_table_query_part() -> impl std::fmt::Display {
// //         Self::create_table_query_part_handle(&CITEXT)
// //     }
// // }
// // #[derive(
// //     Debug,
// //     Clone,
// //     PartialEq,
// //     serde::Serialize,
// //     serde::Deserialize,
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// // )]
// // pub struct StdStringStringAsPostgresqlCiTextNotNull(crate::postgresql_types::postgresql_base_type::StdStringString);
// // impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlCiTextNotNull {
// //     fn create_table_query_part() -> impl std::fmt::Display {
// //         Self::create_table_query_part_handle(&CITEXT)
// //     }
// // }