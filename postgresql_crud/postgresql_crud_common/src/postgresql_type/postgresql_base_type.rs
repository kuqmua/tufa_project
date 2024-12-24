#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypePrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveI16(pub std::primitive::i16);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypePrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveI32(pub std::primitive::i32);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypePrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveI64(pub std::primitive::i64);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveF32(pub std::primitive::f32);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementNumber,
)]
pub struct StdPrimitiveF64(pub std::primitive::f64);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementBool,
)]
pub struct StdPrimitiveBool(pub std::primitive::bool); //todo maybe make it private? //todo column "std_primitive_bool_as_postgresql_bool" is of type boolean but expression is of type bigint
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementStdStringString,
)]
pub struct StdStringString(pub std::string::String);
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     // postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
//     postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementText,
// )]
// pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);



// pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
// pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(pub sqlx::postgres::types::PgRange<std::primitive::i64>);
// pub struct SqlxPostgresTypesPgRangeStdPrimitiveI32(pub sqlx::postgres::types::PgRange<std::primitive::i32>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::time::OffsetDateTime>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDateTime>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime(pub sqlx::postgres::types::PgRange<sqlx::types::time::PrimitiveDateTime>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate(pub sqlx::postgres::types::PgRange<sqlx::types::chrono::NaiveDate>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesTimeDate(pub sqlx::postgres::types::PgRange<sqlx::types::time::Date>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesBigDecimal(pub sqlx::postgres::types::PgRange<sqlx::types::BigDecimal>);
// pub struct SqlxPostgresTypesPgRangeSqlxTypesDecimal(pub sqlx::postgres::types::PgRange<sqlx::types::Decimal>);
// pub struct SqlxPostgresTypesPgMoney(pub sqlx::postgres::types::PgMoney);
// pub struct SqlxPostgresTypesPgCiText(pub sqlx::postgres::types::PgCiText);
// pub struct SqlxTypesBigDecimal(pub sqlx::types::BigDecimal);
// pub struct SqlxTypesDecimal(pub sqlx::types::Decimal);
// pub struct SqlxTypesChronoDateTimeSqlxTypesChronoUtc(pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>);
// pub struct SqlxTypesChronoDateTimeSqlxTypesChronoLocal(pub sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>);
// pub struct SqlxTypesChronoNaiveDateTime(pub sqlx::types::chrono::NaiveDateTime);
// pub struct SqlxTypesChronoNaiveDate(pub sqlx::types::chrono::NaiveDate);
// pub struct SqlxTypesChronoNaiveTime(pub sqlx::types::chrono::NaiveTime);
// pub struct SqlxPostgresTypesPgTimeTz(pub sqlx::postgres::types::PgTimeTz);
// pub struct SqlxTypesTimePrimitiveDateTime(pub sqlx::types::time::PrimitiveDateTime);
// pub struct SqlxTypesTimeOffsetDateTime(pub sqlx::types::time::OffsetDateTime);
// pub struct SqlxTypesTimeDate(pub sqlx::types::time::Date);
// pub struct SqlxTypesTimeTime(pub sqlx::types::time::Time);
// pub struct SqlxTypesUuidUuid(pub sqlx::types::uuid::Uuid);
// pub struct SqlxTypesIpnetworkIpNetwork(pub sqlx::types::ipnetwork::IpNetwork);
// pub struct StdNetIpAddr(pub std::net::IpAddr);
// pub struct SqlxTypesMacAddressMacAddress(pub sqlx::types::mac_address::MacAddress);
// pub struct SqlxTypesBitVec(pub sqlx::types::BitVec);
// pub struct SqlxTypesJson<T>(pub sqlx::types::Json<T>);
// pub struct WhereSqlxTypesJson<T> {
//     pub value: SqlxTypesJson<T>,
//     pub logical_operator: LogicalOperator,
// }
// pub struct StdOptionOptionSqlxTypesJson<T>(pub std::option::Option<sqlx::types::Json<T>>);
// pub struct WhereStdOptionOptionSqlxTypesJson<T> {
//     pub value: StdOptionOptionSqlxTypesJson<T>,
//     pub logical_operator: LogicalOperator,
// }
// pub struct SerdeJsonValue(pub serde_json::Value);


/////////////////////////
// When using the WHERE clause in PostgreSQL with a TEXT column type, you can apply a variety of operations and expressions to filter the results based on the content of the text data. Here are some common operations that can be used with TEXT columns:

// 1. Comparison Operators
// You can use standard comparison operators to compare text values:

// Equal to: =
// Not equal to: <> or !=
// Greater than: >
// Greater than or equal to: >=
// Less than: <
// Less than or equal to: <=
// Example:

// sql

// Копировать код
// SELECT * FROM articles WHERE title = 'PostgreSQL Basics';
// 2. LIKE Operator
// The LIKE operator is used for pattern matching in text comparisons. You can use % as a wildcard for zero or more characters and _ for a single character.

// Example:

// sql

// Копировать код
// SELECT * FROM articles WHERE title LIKE 'Post%';  -- Titles starting with 'Post'
// 3. ILIKE Operator
// The ILIKE operator is similar to LIKE, but it is case-insensitive.

// Example:

// sql

// Копировать код
// SELECT * FROM articles WHERE title ILIKE '%postgresql%';  -- Titles containing 'postgresql' regardless of case
// 4. IN Operator
// You can use the IN operator to check if a TEXT column matches any value in a specified list.

// Example:

// sql

// Копировать код
// SELECT * FROM articles WHERE author IN ('Alice', 'Bob', 'Charlie');
// 5. BETWEEN Operator
// While BETWEEN is typically used for numeric or date ranges, you can use it with TEXT for lexicographical comparisons.

// Example:

// sql

// Копировать код
// SELECT * FROM articles WHERE title BETWEEN 'A' AND 'M';  -- Titles that start with letters A to M
// 6. IS NULL and IS NOT NULL
// You can check for NULL values in a TEXT column.

// Example:

// sql

// Копировать код
// SELECT * FROM articles WHERE summary IS NULL;  -- Articles with no summary
// 7. Regular Expressions
// PostgreSQL supports regular expressions, allowing for complex pattern matching with the ~ (matches) and !~ (does not match) operators.

// Example:

// sql

// Копировать код
// SELECT * FROM articles WHERE title ~ '^[A-Z].*';  -- Titles starting with an uppercase letter
// 8. String Functions
// You can use string functions in the WHERE clause to manipulate or evaluate text data. For example, you can use LENGTH, UPPER, LOWER, etc.

// Example:

// sql

// Копировать код
// SELECT * FROM articles WHERE LENGTH(title) > 50;  -- Articles with titles longer than 50 characters
// 9. Subqueries
// You can use subqueries in the WHERE clause to filter results based on the results of another query.

// Example:

// sql

// Копировать код
// SELECT * FROM articles WHERE author_id IN (SELECT id FROM authors WHERE name LIKE 'J%');
// Summary
// When working with TEXT columns in PostgreSQL, you have a wide range of operations available in the WHERE clause, including comparison operators, pattern matching with LIKE and ILIKE, regular expressions, and string functions. These capabilities allow you to filter and manipulate text data effectively in your queries.



//////////////////////////////