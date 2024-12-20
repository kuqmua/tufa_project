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
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     serde::Serialize,
//     serde::Deserialize,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
// )]
// pub struct StdPrimitiveBoolAsPostgresqlBoolNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
// impl crate::CreateTableQueryPart for StdPrimitiveBoolAsPostgresqlBoolNotNull {
//     fn create_table_query_part() -> impl std::fmt::Display {
//         Self::create_table_query_part_handle(&BOOL)
//     }
// }
// impl crate::CreateTableColumnQueryPart for StdPrimitiveBoolAsPostgresqlBoolNotNull {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} BOOL NOT NULL")
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
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokensWhereElementInt,
)]
pub struct StdPrimitiveI32AsPostgresqlSerialNotNull(crate::postgresql_type::postgresql_base_type::StdPrimitiveI32);
impl crate::CreateTableQueryPart for StdPrimitiveI32AsPostgresqlSerialNotNull {
    fn create_table_query_part() -> impl std::fmt::Display {
        Self::create_table_query_part_handle(&SERIAL)
    }
}
impl crate::CreateTableColumnQueryPart for StdPrimitiveI32AsPostgresqlSerialNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} SERIAL NOT NULL")
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokensWhereElementInt,
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




/////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement {
    Equal(PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElementEqual),
    GreaterThan(PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElementGreaterThan),
    Between(PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElementBetween),
    In(PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElementIn)
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(
        &self,
        increment: &mut std::primitive::u64,
        column: &dyn std::fmt::Display,
        is_need_to_add_logical_operator: std::primitive::bool,
    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                value,
                increment,
                column,
                is_need_to_add_logical_operator,
            ),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                value,
                increment,
                column,
                is_need_to_add_logical_operator,
            ),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                value,
                increment,
                column,
                is_need_to_add_logical_operator,
            ),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(
                value,
                increment,
                column,
                is_need_to_add_logical_operator,
            ),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(
        self,
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                value,
                query
            ),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                value,
                query
            ),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                value,
                query
            ),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(
                value,
                query
            ),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::GreaterThan(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::Between(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::In(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}






#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhereElement {
    pub logical_operator: crate::LogicalOperator,
    pub value: crate::postgresql_type::postgresql_base_type::StdPrimitiveI64,
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhereElement {}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(
        &self,
        increment: &mut std::primitive::u64,
        column: &dyn std::fmt::Display,
        is_need_to_add_logical_operator: std::primitive::bool,
    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        todo!()
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(
        self,
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}

// In PostgreSQL, the WHERE clause can include a variety of expressions to filter records based on specific conditions. Here’s a comprehensive list of expressions that can be used in a WHERE clause:

// 1. Comparison Expressions
// Equal to: column_name = value
// Not equal to: column_name <> value or column_name != value
// Greater than: column_name > value
// Less than: column_name < value
// Greater than or equal to: column_name >= value
// Less than or equal to: column_name <= value
// 2. BETWEEN Expression
// Between: column_name BETWEEN value1 AND value2
// 3. LIKE and ILIKE Expressions
// Like: column_name LIKE 'pattern'
// Case-insensitive Like: column_name ILIKE 'pattern'
// 4. IN Expression
// In: column_name IN (value1, value2, ...)
// 5. NULL Checks
// Is Null: column_name IS NULL
// Is Not Null: column_name IS NOT NULL
// 6. Boolean Expressions
// Boolean Conditions: column_name = TRUE, column_name = FALSE
// Logical Operators:
// AND: condition1 AND condition2
// OR: condition1 OR condition2
// NOT: NOT condition
// 7. Subquery Expressions
// Exists: EXISTS (subquery)
// Any / Some: column_name > ANY (subquery)
// All: column_name > ALL (subquery)
// 8. String Functions
// String Functions: You can use string functions in the WHERE clause, such as:
// LENGTH(column_name) > 5
// UPPER(column_name) = 'VALUE'
// 9. Date and Time Functions
// Date Functions: You can use date functions, such as:
// column_name > CURRENT_DATE
// column_name BETWEEN '2023-01-01' AND '2023-12-31'
// 10. JSON and Array Expressions
// JSON Operators: For querying JSON data types:
// json_column ->> 'key' = 'value'
// Array Operators: For querying array data types:
// column_name @> ARRAY[value1, value2] (contains)
// column_name && ARRAY[value1, value2] (overlaps)
// 11. Full-Text Search
// Full-Text Search: Using @@ for full-text search:
// to_tsvector(column_name) @@ to_tsquery('search_term')
// 12. Custom Functions
// You can also use custom functions in the WHERE clause:
// custom_function(column_name) = value
// Summary
// These expressions can be combined in various ways to create complex filtering conditions in your SQL queries. Always ensure that the data types of the columns and values being compared are compatible to avoid errors. The flexibility of the WHERE clause allows for powerful querying capabilities in PostgreSQL.



















////////////////////////////
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement>,//todo check if not empty
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere {
    fn try_new(
        logical_operator: crate::LogicalOperator,
        value: std::vec::Vec<PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement>,
    ) -> Result<Self, PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereTryNewErrorNamed::IsEmpty {
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        {
            //todo maybe not correct?
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self {
            logical_operator,
            value,
        })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<
                    PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        crate::LogicalOperator,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        std::vec::Vec<
                            PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement,
                        >,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere with 2 elements",
                                ),
                            );
                        }
                    };
                    match PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        crate::LogicalOperator,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        std::vec::Vec<
                            PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement,
                        >,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "logical_operator",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        crate::LogicalOperator,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        std::vec::Vec<
                                            PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("logical_operator")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("value")?
                        }
                    };
                    match PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}")))
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        }
    }
}








#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::vec::Vec<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhereElement>,//todo check if not empty
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: vec![]
        }
    }
}











///////////////////
impl crate::postgresql_type::postgresql_type_trait::PostgresqlType<'_> for StdPrimitiveI32AsPostgresqlSerialNotNull {
    type PostgresqlTypeSelf = Self;
    type PostgresqlTypeSelfColumn = PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullColumn;
    fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type PostgresqlTypeSelfToCreate = PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullToCreate;
    type PostgresqlTypeSelfToRead = PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullToRead;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullToUpdate;
    type PostgresqlTypeSelfToUpdateQueryPartErrorNamed = PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullToUpdateQueryPartErrorNamed;
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
    type PostgresqlTypeSelfWhereElement = PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhereElement;
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdPrimitiveI32AsPostgresqlSerialNotNullWhere;
    fn postgresql_type_self_where_try_generate_bind_increments(
        postgresql_type_self_where: &Self::PostgresqlTypeSelfWhere,
        increment: &mut std::primitive::u64,
        column: &dyn std::fmt::Display,
        is_need_to_add_logical_operator: std::primitive::bool,
    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &postgresql_type_self_where.value {
            match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                Ok(value) => {
                    acc.push_str(&format!("{value} "));
                    is_need_to_add_logical_operator_inner_handle = true;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _ = acc.pop();
        Ok(format!("{}({acc})", &postgresql_type_self_where.logical_operator.to_query_part(is_need_to_add_logical_operator)))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(postgresql_type_self_where: Self::PostgresqlTypeSelfWhere, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in postgresql_type_self_where.value {
            query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
        }
        query
    }
}
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
    type PostgresqlTypeSelfWhereElement = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhereElement;
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere;
    fn postgresql_type_self_where_try_generate_bind_increments(
        postgresql_type_self_where: &Self::PostgresqlTypeSelfWhere,
        increment: &mut std::primitive::u64,
        column: &dyn std::fmt::Display,
        is_need_to_add_logical_operator: std::primitive::bool,
    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &postgresql_type_self_where.value {
            match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                Ok(value) => {
                    acc.push_str(&format!("{value} "));
                    is_need_to_add_logical_operator_inner_handle = true;
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _ = acc.pop();
        let maybe_logical_operator = if is_need_to_add_logical_operator {
            // format!("{}{} ", &postgresql_type_self_where.logical_operator, &postgresql_type_self_where.equal)
            todo!()
        } else {
            std::string::String::default()
        };
        Ok(format!("{maybe_logical_operator}({acc})"))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(postgresql_type_self_where: Self::PostgresqlTypeSelfWhere, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in postgresql_type_self_where.value {
            query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
        }
        query
    }
}
