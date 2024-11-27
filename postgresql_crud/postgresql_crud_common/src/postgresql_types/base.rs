#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseTypeTokens,
)]
pub(crate) struct StdPrimitiveI16(std::primitive::i16);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseTypeTokens,
)]
pub(crate) struct StdPrimitiveI32(std::primitive::i32);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseTypeTokens,
)]
pub(crate) struct StdPrimitiveI64(std::primitive::i64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseTypeTokens,
)]
pub(crate) struct StdPrimitiveF32(std::primitive::f32);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseTypeTokens,
)]
pub(crate) struct StdPrimitiveF64(std::primitive::f64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseTypeTokens,
)]
pub(crate) struct StdPrimitiveBool(std::primitive::bool); //todo maybe make it private? //todo column "std_primitive_bool_as_postgresql_bool" is of type boolean but expression is of type bigint
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlCrudBaseTypeTokens,
)]
pub(crate) struct StdStringString(std::string::String);
//
// // pub(crate) struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
// pub(crate) struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
// pub(crate) struct SqlxPostgresTypesPgRangeStdPrimitiveI64(pub sqlx::postgres::types::PgRange<std::primitive::i64>);
// pub(crate) struct SqlxPostgresTypesPgRangeStdPrimitiveI32(pub sqlx::postgres::types::PgRange<std::primitive::i32>);
// pub(crate) struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>);
// pub(crate) struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>);
// pub(crate) struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>);
// pub(crate) struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>);
// pub(crate) struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>);
// pub(crate) struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>);
// pub(crate) struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>);
// pub(crate) struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>);
// pub(crate) struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>);
// pub(crate) struct SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
// pub(crate) struct SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);
// pub(crate) struct SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);
// pub(crate) struct SqlxTypesDecimal(pub sqlx::types::Decimal);
// pub(crate) struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>);
// pub(crate) struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>);
// pub(crate) struct SqlxTypesChronoNaiveDateTime(pub sqlx::types::chrono::NaiveDateTime);
// pub(crate) struct SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
// pub(crate) struct SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);
// pub(crate) struct SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);
// pub(crate) struct SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);
// pub(crate) struct SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
// pub(crate) struct SqlxTypesTimeDate(pub sqlx::types::time::Date);
// pub(crate) struct SqlxTypesTimeTime(pub sqlx::types::time::Time);
// pub(crate) struct SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
// pub(crate) struct SqlxTypesIpnetworkIpNetwork(pub sqlx::types::ipnetwork::IpNetwork);
// pub(crate) struct StdNetIpAddr(pub std::net::IpAddr);
// pub(crate) struct SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
// pub(crate) struct SqlxTypesBitVec(pub sqlx::types::BitVec);
// pub(crate) struct SqlxTypesJson<T>(pub sqlx::types::Json<T>);
// pub(crate) struct WhereSqlxTypesJson<T> {
//     pub value: SqlxTypesJson<T>,
//     pub conjunctive_operator: ConjunctiveOperator,
// }
// pub(crate) struct StdOptionOptionSqlxTypesJson<T>(pub std::option::Option<sqlx::types::Json<T>>);
// pub(crate) struct WhereStdOptionOptionSqlxTypesJson<T> {
//     pub value: StdOptionOptionSqlxTypesJson<T>,
//     pub conjunctive_operator: ConjunctiveOperator,
// }
// pub(crate) struct SerdeJsonValue(pub serde_json::Value);