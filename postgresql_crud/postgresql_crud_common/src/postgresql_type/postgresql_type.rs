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
// pub struct StdPrimitiveBoolAsPostgresqlBool(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveBool);
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
    // postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
)]
pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
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
// pub struct StdPrimitiveI16AsPostgresqlSmallInt(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI16);
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
// pub struct StdPrimitiveI16AsPostgresqlSmallIntNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI16);
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
// pub struct StdPrimitiveI16AsPostgresqlSmallSerial(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI16);
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
// pub struct StdPrimitiveI16AsPostgresqlSmallSerialNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI16);
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
// pub struct StdPrimitiveI16AsPostgresqlInt2(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI16);
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
// pub struct StdPrimitiveI16AsPostgresqlInt2NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI16);
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
// pub struct StdPrimitiveI32AsPostgresqlInt(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI32);
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
// pub struct StdPrimitiveI32AsPostgresqlIntNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI32);
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
// pub struct StdPrimitiveI32AsPostgresqlSerial(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI32);
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
// pub struct StdPrimitiveI32AsPostgresqlSerialNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI32);
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
// pub struct StdPrimitiveI32AsPostgresqlInt4(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI32);
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
// pub struct StdPrimitiveI32AsPostgresqlInt4NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI32);
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
// pub struct StdPrimitiveI64AsPostgresqlBigInt(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI64);
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
// pub struct StdPrimitiveI64AsPostgresqlBigIntNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
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
// pub struct StdPrimitiveI64AsPostgresqlBigSerial(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI64);
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
    // postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    // postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
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
            <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::postgres::PgHasArrayType for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
            <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::postgres::PgHasArrayType>::array_type_info()
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
// pub struct StdPrimitiveI64AsPostgresqlInt8(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveI64);
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
// pub struct StdPrimitiveI64AsPostgresqlInt8NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
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
// pub struct StdPrimitiveF32AsPostgresqlReal(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveF32);
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
// pub struct StdPrimitiveF32AsPostgresqlRealNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveF32);
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
// pub struct StdPrimitiveF32AsPostgresqlFloat4(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveF32);
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
// pub struct StdPrimitiveF32AsPostgresqlFloat4NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveF32);
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
// pub struct StdPrimitiveF64AsPostgresqlDoublePrecision(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveF64);
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
// pub struct StdPrimitiveF64AsPostgresqlDoublePrecisionNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveF64);
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
// pub struct StdPrimitiveF64AsPostgresqlFloat8(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdPrimitiveF64);
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
// pub struct StdPrimitiveF64AsPostgresqlFloat8NotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveF64);
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
// pub struct StdStringStringAsPostgresqlVarchar(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdStringString);
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
// pub struct StdStringStringAsPostgresqlVarcharNotNull(crate::postgresql_type::postgresql_base_type::StdStringString);
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
// // pub struct StdStringStringAsPostgresqlCharN(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdStringString);
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
// // pub struct StdStringStringAsPostgresqlCharNNotNull(crate::postgresql_type::postgresql_base_type::StdStringString);
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
// pub struct StdStringStringAsPostgresqlText(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdStringString);
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
// pub struct StdStringStringAsPostgresqlTextNotNull(crate::postgresql_type::postgresql_base_type::StdStringString);
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
// // pub struct StdStringStringAsPostgresqlCiText(crate::postgresql_type::postgresql_base_type::StdOptionOptionStdStringString);
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
// // pub struct StdStringStringAsPostgresqlCiTextNotNull(crate::postgresql_type::postgresql_base_type::StdStringString);
// // impl crate::CreateTableQueryPart for StdStringStringAsPostgresqlCiTextNotNull {
// //     fn create_table_query_part() -> impl std::fmt::Display {
// //         Self::create_table_query_part_handle(&CITEXT)
// //     }
// // }

///////////////////////////////////////
impl std::fmt::Display for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::BindQuerySecond<'_> for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
impl StdPrimitiveBoolAsPostgresqlBoolNotNull {
    pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
        crate::postgresql_type::postgresql_base_type::StdPrimitiveBool::create_table_query_part_handle(value)
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde ::
Deserialize,
)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullColumn;
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullColumn {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToCreate(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
impl crate::BindQuerySecond<'_> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToCreate {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToCreate {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToCreateTraits<'_> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToCreate {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToRead(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_type::postgresql_base_type::StdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <crate::postgresql_type::postgresql_base_type::StdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToRead {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToUpdateQueryPartErrorNamed {
    Todo,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToDelete(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
impl std::fmt::Display for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_type::postgresql_base_type::StdPrimitiveBool as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <crate::postgresql_type::postgresql_base_type::StdPrimitiveBool as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::BindQuerySecond<'_> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToDelete {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    pub conjunctive_operator: crate::ConjunctiveOperator,
}
impl crate::BindQuerySecond<'_> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereTraits<'_> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere {}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlType<'_> for StdPrimitiveBoolAsPostgresqlBoolNotNull {
    type PostgresqlTypeSelf = Self;
    type PostgresqlTypeSelfColumn = PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullColumn;
    fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type PostgresqlTypeSelfToCreate = PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToCreate;
    type PostgresqlTypeSelfToRead = PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToRead;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToUpdate;
    type PostgresqlTypeSelfToUpdateQueryPartErrorNamed = PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullToUpdateQueryPartErrorNamed;
    fn postgresql_type_self_to_update_query_part(
        postgresql_type_self_to_update: &Self::PostgresqlTypeSelfToUpdate,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlTypeSelfToUpdateQueryPartErrorNamed> {
        Ok(crate::BindQuerySecond::try_generate_bind_increments(&postgresql_type_self_to_update.0, increment).unwrap())
    }
    fn postgresql_type_self_to_update_bind_query_part<'a>(postgresql_type_self_to_update: Self::PostgresqlTypeSelfToUpdate, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(postgresql_type_self_to_update.0, query)
    }
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere;
}
/////////////////////////////////////////
impl std::fmt::Display for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::BindQuerySecond<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
impl StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
        crate::postgresql_type::postgresql_base_type::StdPrimitiveI64::create_table_query_part_handle(value)
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    serde :: Serialize,
    serde ::
Deserialize,
)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullColumn;
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullColumn {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
impl crate::BindQuerySecond<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToCreateTraits<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdateQueryPartErrorNamed {
    Todo,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
impl std::fmt::Display for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::BindQuerySecond<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    pub conjunctive_operator: crate::ConjunctiveOperator,
}
impl crate::BindQuerySecond<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn bind_value_to_query(self, 
        // mut
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereTraits<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlType<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    type PostgresqlTypeSelf = Self;
    type PostgresqlTypeSelfColumn = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullColumn;
    fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type PostgresqlTypeSelfToCreate = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate;
    type PostgresqlTypeSelfToRead = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate;
    type PostgresqlTypeSelfToUpdateQueryPartErrorNamed = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdateQueryPartErrorNamed;
    fn postgresql_type_self_to_update_query_part(
        postgresql_type_self_to_update: &Self::PostgresqlTypeSelfToUpdate,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlTypeSelfToUpdateQueryPartErrorNamed> {
        Ok(crate::BindQuerySecond::try_generate_bind_increments(&postgresql_type_self_to_update.0, increment).unwrap())
    }
    fn postgresql_type_self_to_update_bind_query_part<'a>(postgresql_type_self_to_update: Self::PostgresqlTypeSelfToUpdate, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(postgresql_type_self_to_update.0, query)
    }
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere;
}
//////////////////////////////////////
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::BindQuerySecond<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl std::fmt::Display for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete(crate::postgresql_type::postgresql_base_type::StdPrimitiveI64);
impl crate::BindQuerySecond<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, 
        // mut 
        query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}
impl std::fmt::Display for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <crate::postgresql_type::postgresql_base_type::StdPrimitiveI64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypePrimaryKey<'_> for StdPrimitiveI64AsPostgresqlBigSerialNotNull {
    type PostgresqlTypeSelfToCreate = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToCreate;
    type PostgresqlTypeSelfToRead = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToRead;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullToUpdate;
    type PostgresqlTypeSelfToDelete = StdPrimitiveI64AsPostgresqlBigSerialNotNullToDelete;
}
