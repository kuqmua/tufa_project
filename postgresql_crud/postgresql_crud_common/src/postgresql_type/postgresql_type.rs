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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens
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
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlTypePrimaryKeyTokens,
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
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementEqual(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(
        &self,
        increment: &mut std::primitive::u64,
        column: &dyn std::fmt::Display,
    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment) {
            Ok(value) => {
                Ok(format!("{column} = ${value}"))
            }
            Err(error) => Err(error),//todo another checked add? 
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(
        self,
        query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuerySecond::bind_value_to_query(self.0, query)
    }
}



#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementGreaterThan(crate::postgresql_type::postgresql_base_type::StdPrimitiveBool);
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementGreaterThan {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementGreaterThan {
    fn postgresql_type_self_where_try_generate_bind_increments(
        &self,
        increment: &mut std::primitive::u64,
        column: &dyn std::fmt::Display,
    ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(incr) => {
                *increment = incr;
                Ok(format!("{column} > ${increment}"))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(
        self,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.0));
        query
    }
}



#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementBetween {
    start: crate::postgresql_type::postgresql_base_type::StdPrimitiveBool,
    end: crate::postgresql_type::postgresql_base_type::StdPrimitiveBool
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementBetween {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            start: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            end: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        }
    }
}
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementBetween {
//     fn postgresql_type_self_where_try_generate_bind_increments(
//         &self,
//         increment: &mut std::primitive::u64,
//         column: &dyn std::fmt::Display,
//     ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(first_increment) => {
//                 *increment = first_increment;
//                 match increment.checked_add(1) {
//                     Some(second_increment) => {
//                         *increment = second_increment;
//                         let between_snake_case = naming::BetweenSnakeCase;
//                         let and_snake_case = naming::AndSnakeCase;
//                         Ok(format!("{column} {between_snake_case} ${first_increment} {and_snake_case} ${second_increment}"))
//                     }
//                     None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//                 }
//             }
//             None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn postgresql_type_self_where_bind_value_to_query<'a>(
//         self,
//         mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
//     ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(sqlx::types::Json(self.0));
//         query
//     }
// }



#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementIn(std::vec::Vec<crate::postgresql_type::postgresql_base_type::StdPrimitiveBool>);
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementIn {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
        ])
    }
}





#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementFilter {
    Equal(PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementEqual),
    GreaterThan(PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementGreaterThan),
    Between(PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementBetween),
    In(PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementIn)
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



/////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElement {
    // pub value: crate::postgresql_type::postgresql_base_type::StdPrimitiveBool,
    pub value: PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementFilter,
    pub conjunctive_operator: crate::ConjunctiveOperator,
}
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElement {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             value: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//         }
//     }
// }
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElement {}



















#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhereElement {
    pub value: crate::postgresql_type::postgresql_base_type::StdPrimitiveI64,
    pub conjunctive_operator: crate::ConjunctiveOperator,
}
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhereElement {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             value: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//         }
//     }
// }
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhereElement {}









////////////////////////////
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    conjunctive_operator: crate::ConjunctiveOperator,
    value: std::vec::Vec<PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElement>,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: vec![
                PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElement {
                    value: PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementFilter::Equal(
                        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
                    ),
                    conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                },
                PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElement {
                    value: PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementFilter::GreaterThan(
                        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
                    ),
                    conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                },
                PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElement {
                    value: PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementFilter::Between(
                        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
                    ),
                    conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                },
                PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElement {
                    value: PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhereElementFilter::In(
                        crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()
                    ),
                    conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
                }
            ]
        }
    }
}
impl crate::GetConjunctiveOperator for PostgresqlTypeStdPrimitiveBoolAsPostgresqlBoolNotNullWhere {
    fn get_conjunctive_operator(&self) -> &crate::ConjunctiveOperator {
        &self.conjunctive_operator
    }
}





#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    conjunctive_operator: crate::ConjunctiveOperator,
    value: std::vec::Vec<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhereElement>,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            conjunctive_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: vec![]
        }
    }
}
impl crate::GetConjunctiveOperator for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialNotNullWhere {
    fn get_conjunctive_operator(&self) -> &crate::ConjunctiveOperator {
        &self.conjunctive_operator
    }
}
