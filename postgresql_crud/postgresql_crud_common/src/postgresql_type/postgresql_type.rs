const INT2: &std::primitive::str = "INT2";
const INT4: &std::primitive::str = "INT4";
const INT8: &std::primitive::str = "INT8";
const FLOAT4: &std::primitive::str = "FLOAT4";
const FLOAT8: &std::primitive::str = "FLOAT8";
const SMALLSERIAL: &std::primitive::str = "SMALLSERIAL";
const SERIAL: &std::primitive::str = "SERIAL";
const BIGSERIAL: &std::primitive::str = "BIGSERIAL";
const MONEY: &std::primitive::str = "MONEY";
const NUMERIC: &std::primitive::str = "NUMERIC";
const BOOL: &std::primitive::str = "BOOL";
// const CHAR: &std::primitive::str = "CHAR";
// const CHARN: &std::primitive::str = "CHAR(N)";//todo length not supported yet
// const VARCHAR: &std::primitive::str = "VARCHAR";
const TEXT: &std::primitive::str = "TEXT";
// const CITEXT: &std::primitive::str = "CITEXT";//activates by installing extension
const BYTEA: &std::primitive::str = "BYTEA";
const DATE: &std::primitive::str = "DATE";
const TIME: &std::primitive::str = "TIME";
// const TIMETZ: &std::primitive::str = "TIMETZ";//postgresql recommends do not use it https://wiki.postgresql.org/wiki/Don't_Do_This#Don.27t_use_timetz
const INTERVAL: &std::primitive::str = "INTERVAL";
const INT4RANGE: &std::primitive::str = "INT4RANGE";
const INT8RANGE: &std::primitive::str = "INT8RANGE";
const TSRANGE: &std::primitive::str = "TSRANGE";
const TSTZRANGE: &std::primitive::str = "TSTZRANGE";
const DATERANGE: &std::primitive::str = "DATERANGE";
const NUMRANGE: &std::primitive::str = "NUMRANGE";
const TIMESTAMP: &std::primitive::str = "TIMESTAMP";
const TIMESTAMPTZ: &std::primitive::str = "TIMESTAMPTZ";
const UUID: &std::primitive::str = "UUID";
const INET: &std::primitive::str = "INET";
// const CIDR: &std::primitive::str = "CIDR";
const MACADDR: &std::primitive::str = "MACADDR";
// const BIT: &std::primitive::str = "BIT";
// const VARBIT: &std::primitive::str = "VARBIT";

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdPrimitiveI16AsPostgresqlInt2(crate::postgresql_type::postgresql_base_type::StdPrimitiveI16);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdPrimitiveI32AsPostgresqlInt4(crate::postgresql_type::postgresql_base_type::StdPrimitiveI32);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdPrimitiveI64AsPostgresqlInt8(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdPrimitiveF32AsPostgresqlFloat4(crate::postgresql_type::postgresql_base_type::StdPrimitiveF32);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdPrimitiveF64AsPostgresqlFloat8(crate::postgresql_type::postgresql_base_type::StdPrimitiveF64);
impl crate::CreateTableColumnQueryPart for StdPrimitiveF64AsPostgresqlFloat8 {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {FLOAT8}")
    }
}
// todo remove nullable version coz its cannot be nullable in postgresql https://www.tutorialsteacher.com/postgresql/serial-type .
// // todo rewrite create impl for serial types coz its autoincrement type by postgresql
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartPrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
pub struct StdPrimitiveI16AsPostgresqlSmallSerial(crate::postgresql_type::postgresql_base_type::StdPrimitiveI16);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI16AsPostgresqlSmallSerial {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {SMALLSERIAL}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartPrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
pub struct StdPrimitiveI32AsPostgresqlSerial(crate::postgresql_type::postgresql_base_type::StdPrimitiveI32);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI32AsPostgresqlSerial {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {SERIAL}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartPrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
struct StdPrimitiveI64AsPostgresqlBigSerial(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI64AsPostgresqlBigSerial {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BIGSERIAL}")
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgMoneyAsPostgresqlMoney(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgMoney);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesDecimalAsPostgresqlNumeric(crate::postgresql_type::postgresql_base_type::SqlxTypesDecimal);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesBigDecimalAsPostgresqlNumeric(crate::postgresql_type::postgresql_base_type::SqlxTypesBigDecimal);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdPrimitiveBoolAsPostgresqlBool(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
impl crate::CreateTableColumnQueryPart for StdPrimitiveBoolAsPostgresqlBool {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BOOL}")
    }
}
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
// )]
// struct StdStringStringAsPostgresqlVarchar(crate::postgresql_type::postgresql_base_type::StdStringString);
// impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlVarchar {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {VARCHAR}")
//     }
// }
// // #[derive(
// //     Debug,
// //     Clone,
// //     PartialEq,
// //     serde::Serialize,
// //     serde::Deserialize,
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
// // )]
// // struct StdStringStringAsPostgresqlCharN(crate::postgresql_type::postgresql_base_type::StdStringString);
// // impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlCharN {
// //     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
// //         format!("{column} {CHARN}")
// //     }
// // }
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdStringStringAsPostgresqlText(crate::postgresql_type::postgresql_base_type::StdStringString);
impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlText {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TEXT}")
    }
}
// // #[derive(
// //     Debug,
// //     Clone,
// //     PartialEq,
// //     serde::Serialize,
// //     serde::Deserialize,
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
// // )]
// // struct StdStringStringAsPostgresqlCiText(crate::postgresql_type::postgresql_base_type::StdStringString);
// // impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlCiText {
// //     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
// //         format!("{column} {CITEXT}")
// //     }
// // }
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdVecVecStdPrimitiveU8AsPostgresqlBytea(crate::postgresql_type::postgresql_base_type::StdVecVecStdPrimitiveU8);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesTimeDateAsPostgresqlDate(crate::postgresql_type::postgresql_base_type::SqlxTypesTimeDate);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesChronoNaiveDateAsPostgresqlDate(crate::postgresql_type::postgresql_base_type::SqlxTypesChronoNaiveDate);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesChronoNaiveTimeAsPostgresqlTime(crate::postgresql_type::postgresql_base_type::SqlxTypesChronoNaiveTime);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesTimeTimeAsPostgresqlTime(crate::postgresql_type::postgresql_base_type::SqlxTypesTimeTime);
impl crate::CreateTableColumnQueryPart for SqlxTypesTimeTimeAsPostgresqlTime {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {TIME}")
    }
}
//todo add prefix  for nullable types
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgIntervalAsPostgresqlInterval(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgInterval);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeStdPrimitiveI32AsPostgresqlInt4Range(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeStdPrimitiveI32);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeStdPrimitiveI64AsPostgresqlInt8Range(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeStdPrimitiveI64);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsPostgresqlTsRange(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsPostgresqlTsRange(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTsTzRange(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTsTzRange(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeAsPostgresqlTsTzRange(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsPostgresqlDateRange(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsPostgresqlDateRange(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDate);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeSqlxTypesDecimalAsPostgresqlNumRange(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesDecimal);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsPostgresqlNumRange(crate::postgresql_type::postgresql_base_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesChronoNaiveDateTimeAsPostgresqlTimestamp(crate::postgresql_type::postgresql_base_type::SqlxTypesChronoNaiveDateTime);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesTimePrimitiveDateTimeAsPostgresqlTimestamp(crate::postgresql_type::postgresql_base_type::SqlxTypesTimePrimitiveDateTime);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesTimeOffsetDateTimeAsPostgresqlTimestampTz(crate::postgresql_type::postgresql_base_type::SqlxTypesTimeOffsetDateTime);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz(crate::postgresql_type::postgresql_base_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtc);
impl crate::CreateTableColumnQueryPart for SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsPostgresqlTimestampTz {
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz(crate::postgresql_type::postgresql_base_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocal);
impl crate::CreateTableColumnQueryPart for SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsPostgresqlTimestampTz {
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesUuidUuidAsPostgresqlUuid(crate::postgresql_type::postgresql_base_type::SqlxTypesUuidUuid);
impl crate::CreateTableColumnQueryPart for SqlxTypesUuidUuidAsPostgresqlUuid {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {UUID}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdNetIpAddrAsPostgresqlInet(crate::postgresql_type::postgresql_base_type::StdNetIpAddr);
impl crate::CreateTableColumnQueryPart for StdNetIpAddrAsPostgresqlInet {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {INET}")
    }
}
// todo mismatched types; Rust type `postgresql_crud_common::postgresql_type::postgresql_type::PostgresqlTypeSqlxTypesIpnetworkIpNetworkAsPostgresqlCidrNotNullToRead` (as SQL type `INET`) is not compatible with SQL type `CIDR`
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
// )]
// struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr(crate::postgresql_type::postgresql_base_type::SqlxTypesIpnetworkIpNetwork);
// impl crate::CreateTableColumnQueryPart for SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {CIDR}")
//     }
// }
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr(crate::postgresql_type::postgresql_base_type::SqlxTypesMacAddressMacAddress);
impl crate::CreateTableColumnQueryPart for SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {MACADDR}")
    }
}
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
// )]
// struct SqlxTypesBitVecAsPostgresqlBit(crate::postgresql_type::postgresql_base_type::SqlxTypesBitVec);
// impl crate::CreateTableColumnQueryPart for SqlxTypesBitVecAsPostgresqlBit {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {BIT}")
//     }
// }