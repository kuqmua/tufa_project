const INT4: &std::primitive::str = "int4";
const INT8: &std::primitive::str = "int8";
const FLOAT4: &std::primitive::str = "float4";
const FLOAT8: &std::primitive::str = "float8";
const SMALLSERIAL: &std::primitive::str = "smallserial";
const SERIAL: &std::primitive::str = "serial";
const BIGSERIAL: &std::primitive::str = "bigserial";
const MONEY: &std::primitive::str = "money";
const NUMERIC: &std::primitive::str = "numeric";
const BOOL: &std::primitive::str = "bool";
const CHAR: &std::primitive::str = "char";
const VARCHAR: &std::primitive::str = "varchar";
const TEXT: &std::primitive::str = "text";
// const CITEXT: &std::primitive::str = "citext";//activates by installing extension
const BYTEA: &std::primitive::str = "bytea";
const DATE: &std::primitive::str = "date";
const TIME: &std::primitive::str = "time";
// const TIMETZ: &std::primitive::str = "timetz";//postgresql recommends do not use it https://wiki.postgresql.org/wiki/Don't_Do_This#Don.27t_use_timetz
const INTERVAL: &std::primitive::str = "interval";
const INT4RANGE: &std::primitive::str = "int4range";
const INT8RANGE: &std::primitive::str = "int8range";
const TSRANGE: &std::primitive::str = "tsrange";
const TSTZRANGE: &std::primitive::str = "tstzrange";
const DATERANGE: &std::primitive::str = "daterange";
const NUMRANGE: &std::primitive::str = "numrange";
const TIMESTAMP: &std::primitive::str = "timestamp";
const TIMESTAMPTZ: &std::primitive::str = "timestamptz";
const UUID: &std::primitive::str = "uuid";
const INET: &std::primitive::str = "inet";
const CIDR: &std::primitive::str = "cidr";
const MACADDR: &std::primitive::str = "macaddr";
const BIT: &std::primitive::str = "bit";

#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    // postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    // postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdPrimitiveI16AsPostgresqlInt2(std::primitive::i16);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedUsingDefaultKeywordByPostgresqlTokens,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartPrimaryKeyTokens,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
// )]
// struct StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql(crate::postgresql_type::postgresql_base_type::StdPrimitiveI16);
// impl crate::CreateTableColumnQueryPart for StdPrimitiveI16AsPostgresqlSmallSerialInitializedByPostgresql {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} {SMALLSERIAL}")
//     }
// }
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedUsingDefaultKeywordByPostgresqlTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartPrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
struct StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql(crate::postgresql_type::postgresql_base_type::StdPrimitiveI32);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI32AsPostgresqlSerialInitializedByPostgresql {
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedUsingDefaultKeywordByPostgresqlTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartPrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
struct StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BIGSERIAL}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdPrimitiveBoolAsPostgresqlBool(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdStringStringAsPostgresqlCharN(crate::postgresql_type::postgresql_base_type::StdStringString);
impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlCharN {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {CHAR}(10)")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct StdStringStringAsPostgresqlVarchar(crate::postgresql_type::postgresql_base_type::StdStringString);
impl crate::CreateTableColumnQueryPart for StdStringStringAsPostgresqlVarchar {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {VARCHAR}(8)")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
// //     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedUsingUuidGenerateV4FunctionByPostgresqlTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartPrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
struct SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql(crate::postgresql_type::postgresql_base_type::SqlxTypesUuidUuid);
impl crate::CreateTableColumnQueryPart for SqlxTypesUuidUuidAsPostgresqlUuidV4InitializedByPostgresql {
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient(crate::postgresql_type::postgresql_base_type::SqlxTypesUuidUuid);
impl crate::CreateTableColumnQueryPart for SqlxTypesUuidUuidAsPostgresqlUuidInitializedByClient {
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesIpnetworkIpNetworkAsPostgresqlInet(crate::postgresql_type::postgresql_base_type::SqlxTypesIpnetworkIpNetwork);
impl crate::CreateTableColumnQueryPart for SqlxTypesIpnetworkIpNetworkAsPostgresqlInet {
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr(crate::postgresql_type::postgresql_base_type::SqlxTypesIpnetworkIpNetwork);
impl crate::CreateTableColumnQueryPart for SqlxTypesIpnetworkIpNetworkAsPostgresqlCidr {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {CIDR}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr(crate::postgresql_type::postgresql_base_type::SqlxTypesMacAddressMacAddress);
impl crate::CreateTableColumnQueryPart for SqlxTypesMacAddressMacAddressAsPostgresqlMacAddr {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {MACADDR}")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesBitVecAsPostgresqlBit(crate::postgresql_type::postgresql_base_type::SqlxTypesBitVec);
impl crate::CreateTableColumnQueryPart for SqlxTypesBitVecAsPostgresqlBit {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BIT}(9)")//todo number elements support. //todo not its work only if number % 8 = 0 coz std::primitive::u8 initialization, conversion and serde
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeInitializedByClientTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeCreateTableColumnQueryPartTokens,
)]
struct SqlxTypesBitVecAsPostgresqlVarbit(crate::postgresql_type::postgresql_base_type::SqlxTypesBitVec);
impl crate::CreateTableColumnQueryPart for SqlxTypesBitVecAsPostgresqlVarbit {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} {BIT} VARYING(9)")//todo number elements support. //todo not its work only if number % 8 = 0 coz std::primitive::u8 initialization, conversion and serde
    }
}

//////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////
/////////////////////////////////////////