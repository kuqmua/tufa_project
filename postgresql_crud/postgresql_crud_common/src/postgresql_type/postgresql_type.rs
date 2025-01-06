
const INT2: &std::primitive::str = "INT2";
const INT4: &std::primitive::str = "INT4";
const INT8: &std::primitive::str = "INT8";
const FLOAT4: &std::primitive::str = "FLOAT4";
const FLOAT8: &std::primitive::str = "FLOAT8";
const SMALLSERIAL: &std::primitive::str = "SMALLSERIAL";
const SERIAL: &std::primitive::str = "SERIAL";
const BIGSERIAL: &std::primitive::str = "BIGSERIAL";
const BOOL: &std::primitive::str = "BOOL";
const CHAR: &std::primitive::str = "CHAR";
const VARCHAR: &std::primitive::str = "VARCHAR";
// const CHARN: &std::primitive::str = "CHAR(N)";
const TEXT: &std::primitive::str = "TEXT";
// const CITEXT: &std::primitive::str = "CITEXT";//activates by installing extension
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

const NOT_NULL: &std::primitive::str = "NOT NULL";


#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveI16AsPostgresqlInt2(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI16);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI16AsPostgresqlInt2 {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT2}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
)]
pub struct StdPrimitiveI16AsPostgresqlInt2NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI16);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI16AsPostgresqlInt2NotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT2} {NOT_NULL}")
    }
}
//int and int4 are the same, so remove int type
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
)]
pub struct StdPrimitiveI32AsPostgresqlInt4(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI32);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI32AsPostgresqlInt4 {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT4}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
)]
pub struct StdPrimitiveI32AsPostgresqlInt4NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI32);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI32AsPostgresqlInt4NotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT4} {NOT_NULL}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveI64AsPostgresqlInt8(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI64);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI64AsPostgresqlInt8 {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT8}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
pub struct StdPrimitiveI64AsPostgresqlInt8NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI64AsPostgresqlInt8NotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT8} {NOT_NULL}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveF32AsPostgresqlFloat4(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveF32);
impl crate::CreateTableColumnQueryPart for StdPrimitiveF32AsPostgresqlFloat4 {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {FLOAT4}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveF32AsPostgresqlFloat4NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveF32);
impl crate::CreateTableColumnQueryPart for StdPrimitiveF32AsPostgresqlFloat4NotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {FLOAT4} {NOT_NULL}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveF64AsPostgresqlFloat8(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveF64);
impl crate::CreateTableColumnQueryPart for StdPrimitiveF64AsPostgresqlFloat8 {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {FLOAT8}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveF64AsPostgresqlFloat8NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveF64);
impl crate::CreateTableColumnQueryPart for StdPrimitiveF64AsPostgresqlFloat8NotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {FLOAT8} {NOT_NULL}")
    }
}
// removed serial types (nullable) coz its cannot be nullable in postgresql https://www.tutorialsteacher.com/postgresql/serial-type .
// todo rewrite create impl for serial types coz its autoincrement type by postgresql
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
// )]
// pub struct StdPrimitiveI16AsPostgresqlSmallSerialNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI16);
// impl crate::CreateTableColumnQueryPart for StdPrimitiveI16AsPostgresqlSmallSerialNotNull {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {SMALLSERIAL} {NOT_NULL}{}", crate::maybe_primary_key(is_primary_key))
//     }
// }
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
// )]
// pub struct StdPrimitiveI32AsPostgresqlSerialNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI32);
// impl crate::CreateTableColumnQueryPart for StdPrimitiveI32AsPostgresqlSerialNotNull {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {SERIAL} {NOT_NULL}{}", crate::maybe_primary_key(is_primary_key))
//     }
// }
//todo test autoincrement on int as primary key
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BIGSERIAL} {NOT_NULL}{}", crate::maybe_primary_key(is_primary_key))
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
            <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::postgres::PgHasArrayType for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
            <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::postgres::PgHasArrayType>::array_type_info()
        }
    }
};
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveBoolAsPostgresqlBool(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveBool);
impl crate::CreateTableColumnQueryPart for StdPrimitiveBoolAsPostgresqlBool {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BOOL}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
impl crate::CreateTableColumnQueryPart for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BOOL} {NOT_NULL}")
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
// pub struct StdStringStringAsPostgresqlVarchar(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdStringString);
// impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlVarchar {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {VARCHAR}")
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
// pub struct StdStringStringAsPostgresqlVarcharNotNull(crate::postgresql_type::postgresql_base_type::StdStringString);
// impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlVarcharNotNull {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {VARCHAR} {NOT_NULL}")
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
// // pub struct StdStringStringAsPostgresqlCharN(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdStringString);
// // impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlCharN {
// //     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
// //         format!("{column} {CHARN}")
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
// // pub struct StdStringStringAsPostgresqlCharNNotNull(crate::postgresql_type::postgresql_base_type::StdStringString);
// // impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlCharNNotNull {
// //     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
// //         format!("{column} {CHARN} {NOT_NULL}")
// //     }
// // }
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdStringStringAsPostgresqlText(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdStringString);
impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlText {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TEXT}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdStringStringAsPostgresqlTextNotNull(crate::postgresql_type::postgresql_base_type::StdStringString);
impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlTextNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TEXT} {NOT_NULL}")
    }
}
// // #[derive(
// //     Debug,
// //     Clone,
// //     PartialEq,
// //     serde::Serialize,
// //     serde::Deserialize,
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// // )]
// // pub struct StdStringStringAsPostgresqlCiText(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdStringString);
// // impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlCiText {
// //     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
// //         format!("{column} {CITEXT}")
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
// // pub struct StdStringStringAsPostgresqlCiTextNotNull(crate::postgresql_type::postgresql_base_type::StdStringString);
// // impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlCiTextNotNull {
// //     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
// //         format!("{column} {CITEXT} {NOT_NULL}")
// //     }
// // }
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdVecVecStdPrimitiveU8AsPostgresqlBytea(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdVecVecStdPrimitiveU8);
impl crate::CreateTableColumnQueryPart for StdVecVecStdPrimitiveU8AsPostgresqlBytea {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BYTEA}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull(crate::postgresql_type::postgresql_base_type::StdVecVecStdPrimitiveU8);
impl crate::CreateTableColumnQueryPart for StdVecVecStdPrimitiveU8AsPostgresqlByteaNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BYTEA} {NOT_NULL}")
    }
}
////
//todo add prefix StdOptionOption for nullable types
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlInterval(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgInterval);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgIntervalAsPostgresqlInterval {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INTERVAL}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgInterval);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgIntervalAsPostgresqlIntervalNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INTERVAL} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI32);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT4RANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeStdPrimitiveI32);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4RangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT4RANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeStdPrimitiveI64);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT8RANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeStdPrimitiveI64);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8RangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INT8RANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSRANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSRANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSTZRANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSTZRANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSTZRANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSTZRANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {DATERANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {DATERANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesDecimal);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {NUMRANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesDecimal);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {NUMRANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesTimeOffsetDateTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIMESTAMPTZ}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesTimeOffsetDateTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTzNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIMESTAMPTZ} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesTimeDateAsPostgresqlDate(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesTimeDate);
impl crate::CreateTableColumnQueryPart for SqlxTypesTimeDateAsPostgresqlDate {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {DATE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesTimeDateAsPostgresqlDateNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesTimeDate);
impl crate::CreateTableColumnQueryPart for SqlxTypesTimeDateAsPostgresqlDateNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {DATE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesChronoNaiveTimeAsPostgresqlTime(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesChronoNaiveTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesChronoNaiveTimeAsPostgresqlTime {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIME}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesChronoNaiveTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesChronoNaiveTimeAsPostgresqlTimeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIME} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesChronoNaiveDateAsPostgresqlDate(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesChronoNaiveDate);
impl crate::CreateTableColumnQueryPart for SqlxTypesChronoNaiveDateAsPostgresqlDate {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {DATE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesChronoNaiveDate);
impl crate::CreateTableColumnQueryPart for SqlxTypesChronoNaiveDateAsPostgresqlDateNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {DATE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesChronoNaiveDateTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIMESTAMP}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesChronoNaiveDateTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestampNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIMESTAMP} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesTimeTimeAsPostgresqlTime(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesTimeTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesTimeTimeAsPostgresqlTime {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIME}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesTimeTimeAsPostgresqlTimeNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesTimeTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesTimeTimeAsPostgresqlTimeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIME} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgTimeTz);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTz {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIMETZ}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgTimeTz);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgTimeTzAsPostgresqlTimeTzNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIMETZ} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesTimePrimitiveDateTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIMESTAMP}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesTimePrimitiveDateTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestampNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIMESTAMP} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesDecimalAsPostgresqlNumeric(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesDecimal);
impl crate::CreateTableColumnQueryPart for SqlxTypesDecimalAsPostgresqlNumeric {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {NUMERIC}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesDecimalAsPostgresqlNumericNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesDecimal);
impl crate::CreateTableColumnQueryPart for SqlxTypesDecimalAsPostgresqlNumericNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {NUMERIC} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesBigDecimalAsPostgresqlNumeric(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesBigDecimal);
impl crate::CreateTableColumnQueryPart for SqlxTypesBigDecimalAsPostgresqlNumeric {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {NUMERIC}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxTypesBigDecimalAsPostgresqlNumericNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesBigDecimal);
impl crate::CreateTableColumnQueryPart for SqlxTypesBigDecimalAsPostgresqlNumericNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {NUMERIC} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSTZRANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSTZRANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSRANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TSRANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesTimeDate);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {DATERANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDate);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {DATERANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimal);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {NUMRANGE}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRangeNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {NUMRANGE} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgMoneyAsPostgresqlMoney(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxPostgresTypesPgMoney);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgMoneyAsPostgresqlMoney {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {MONEY}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgMoney);
impl crate::CreateTableColumnQueryPart for SqlxPostgresTypesPgMoneyAsPostgresqlMoneyNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {MONEY} {NOT_NULL}")
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdNetIpAddrAsPostgresqlInet(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdNetIpAddr);
impl crate::CreateTableColumnQueryPart for StdNetIpAddrAsPostgresqlInet {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INET}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdNetIpAddrAsPostgresqlInetNotNull(crate::postgresql_type::postgresql_base_type::StdNetIpAddr);
impl crate::CreateTableColumnQueryPart for StdNetIpAddrAsPostgresqlInetNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INET} {NOT_NULL}")
    }
}

// todo mismatched types; Rust type `postgresql_crud_common::postgresql_type::postgresql_type::PostgresqlTypeSqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNullToRead` (as SQL type `INET`) is not compatible with SQL type `CIDR`
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr(crate::postgresql_type::postgresql_base_type::StdOptionOptionSqlxTypesIpnetworkIpNetwork);
// impl crate::CreateTableColumnQueryPart for SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {CIDR}")
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
// pub struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull(crate::postgresql_type::postgresql_base_type::SqlxTypesIpnetworkIpNetwork);
// impl crate::CreateTableColumnQueryPart for SqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNull {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {CIDR} {NOT_NULL}")
//     }
// }