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
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensStdVecVecStdPrimitiveU8,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementStdVecVecStdPrimitiveU8,
)]
pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);


#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    //todo utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgInterval,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgInterval,
)]
pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);//why does not impl PartialOrd
impl serde::Serialize for SqlxPostgresTypesPgInterval {
    fn serialize<S>(&self, serializer: S) -> serde::__private::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = serde::Serializer::serialize_struct(serializer, "SqlxPostgresTypesPgInterval", false as usize + 1 + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "months", &self.0.months)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "days", &self.0.days)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "microseconds", &self.0.microseconds)?;
        serde::ser::SerializeStruct::end(serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Months,
            Days,
            Microseconds,
        }
        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl serde::de::Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("`months` or `days` or `microseconds`")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "months" => Ok(Field::Months),
                            "days" => Ok(Field::Days),
                            "microseconds" => Ok(Field::Microseconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SqlxPostgresTypesPgIntervalVisitor;
        impl<'de> serde::de::Visitor<'de> for SqlxPostgresTypesPgIntervalVisitor {
            type Value = SqlxPostgresTypesPgInterval;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SqlxPostgresTypesPgInterval")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<SqlxPostgresTypesPgInterval, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let months = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let days = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let microseconds = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                Ok(SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval { months, days, microseconds }))
            }
            fn visit_map<V>(self, mut map: V) -> Result<SqlxPostgresTypesPgInterval, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut months = None;
                let mut days = None;
                let mut microseconds = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Months => {
                            if months.is_some() {
                                return Err(serde::de::Error::duplicate_field("months"));
                            }
                            months = Some(map.next_value()?);
                        }
                        Field::Days => {
                            if days.is_some() {
                                return Err(serde::de::Error::duplicate_field("days"));
                            }
                            days = Some(map.next_value()?);
                        }
                        Field::Microseconds => {
                            if microseconds.is_some() {
                                return Err(serde::de::Error::duplicate_field("microseconds"));
                            }
                            microseconds = Some(map.next_value()?);
                        }
                    }
                }
                let months = months.ok_or_else(|| serde::de::Error::missing_field("months"))?;
                let days = days.ok_or_else(|| serde::de::Error::missing_field("days"))?;
                let microseconds = microseconds.ok_or_else(|| serde::de::Error::missing_field("microseconds"))?;
                Ok(SqlxPostgresTypesPgInterval(sqlx::postgres::types::PgInterval { months, days, microseconds }))
            }
        }
        const FIELDS: &[&str] = &["months", "days", "microseconds"];
        deserializer.deserialize_struct("SqlxPostgresTypesPgInterval", FIELDS, SqlxPostgresTypesPgIntervalVisitor)
    }
}
////////////////////////////////////////////////////////////

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    //todo utoipa::ToSchema,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgRangeStdPrimitiveI64,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgRangeStdPrimitiveI64,
)]
pub struct SqlxPostgresTypesPgRangeStdPrimitiveI64(pub sqlx::postgres::types::PgRange<std::primitive::i64>);
impl serde::Serialize for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn serialize<S>(&self, serializer: S) -> serde::__private::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = serde::Serializer::serialize_struct(serializer, "SqlxPostgresTypesPgRangeStdPrimitiveI64", false as usize + 1 + 1)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "start", &self.0.start)?;
        serde::ser::SerializeStruct::serialize_field(&mut serde_state, "end", &self.0.end)?;
        serde::ser::SerializeStruct::end(serde_state)
    }
}
impl<'de> serde::Deserialize<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;
                impl serde::de::Visitor<'_> for FieldVisitor {
                    type Value = Field;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("`start` or `end`")
                    }
                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(Field::Start),
                            "end" => Ok(Field::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct SqlxPostgresTypesPgRangeStdPrimitiveI64Visitor;
        impl<'de> serde::de::Visitor<'de> for SqlxPostgresTypesPgRangeStdPrimitiveI64Visitor {
            type Value = SqlxPostgresTypesPgRangeStdPrimitiveI64;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct SqlxPostgresTypesPgRangeStdPrimitiveI64")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI64, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let start = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let end = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange { start, end }))
            }
            fn visit_map<V>(self, mut map: V) -> Result<SqlxPostgresTypesPgRangeStdPrimitiveI64, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut start = None;
                let mut end = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Start => {
                            if start.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start = Some(map.next_value()?);
                        }
                        Field::End => {
                            if end.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end = Some(map.next_value()?);
                        }
                    }
                }
                let start = start.ok_or_else(|| serde::de::Error::missing_field("start"))?;
                let end = end.ok_or_else(|| serde::de::Error::missing_field("end"))?;
                Ok(SqlxPostgresTypesPgRangeStdPrimitiveI64(sqlx::postgres::types::PgRange { start, end }))
            }
        }
        const FIELDS: &[&str] = &["start", "end"];
        deserializer.deserialize_struct("SqlxPostgresTypesPgRangeStdPrimitiveI64", FIELDS, SqlxPostgresTypesPgRangeStdPrimitiveI64Visitor)
    }
}

/////////



// In PostgreSQL, INT8RANGE is a range type that is used to represent a range of 64-bit integers (BIGINT). The range types in PostgreSQL can be used with various operations that allow you to filter, compare, and manipulate the ranges. Below is a comprehensive list of WHERE operations that you can use with the INT8RANGE type.

// 1. Checking if a value is contained within a range
// You can use the @> (contains) operator to check if a value is contained within an INT8RANGE.

// sql
// Копировать код
// WHERE range_column @> 10
// This will return rows where the range contains the value 10.

// 2. Checking if a range contains another range
// You can use the @> operator to check if one range contains another.

// sql
// Копировать код
// WHERE range_column @> '[5, 15]'
// This will return rows where range_column contains the range [5, 15].

// 3. Checking if a value is strictly inside a range (not touching boundaries)
// You can use the << (strictly contained by) operator to check if a value is strictly inside the range.

// sql
// Копировать код
// WHERE 10 << range_column
// This will return rows where range_column does not include the boundaries and strictly contains the value 10.

// 4. Checking if a range is strictly contained by another range
// You can use the << operator to check if one range is strictly contained within another range.

// sql
// Копировать код
// WHERE range_column << '[5, 15]'
// This will return rows where range_column is strictly contained within [5, 15].

// 5. Checking if a range overlaps with another range
// You can use the && (overlaps) operator to check if two ranges overlap.

// sql
// Копировать код
// WHERE range_column && '[10, 20]'
// This will return rows where range_column overlaps with the range [10, 20].

// 6. Checking if a range does not overlap with another range
// You can use the && operator in a negation form to check if two ranges do not overlap.

// sql
// Копировать код
// WHERE NOT (range_column && '[10, 20]')
// This will return rows where range_column does not overlap with the range [10, 20].

// 7. Checking if a range is left of another range (non-overlapping)
// You can use the &< (left of) operator to check if one range is entirely to the left of another.

// sql
// Копировать код
// WHERE range_column &< '[5, 15]'
// This will return rows where range_column is completely to the left of the range [5, 15].

// 8. Checking if a range is right of another range (non-overlapping)
// You can use the &> (right of) operator to check if one range is entirely to the right of another.

// sql
// Копировать код
// WHERE range_column &> '[5, 15]'
// This will return rows where range_column is completely to the right of the range [5, 15].

// 9. Checking if a range is equal to another range
// You can use the = (equal) operator to check if two ranges are equal.

// sql
// Копировать код
// WHERE range_column = '[5, 15]'
// This will return rows where range_column is exactly equal to [5, 15].

// 10. Checking if a range is not equal to another range
// You can use the <> (not equal) operator to check if two ranges are not equal.

// sql
// Копировать код
// WHERE range_column <> '[5, 15]'
// This will return rows where range_column is not equal to [5, 15].

// 11. Checking if a range is empty
// You can use the IS EMPTY condition to check if the range is empty.

// sql
// Копировать код
// WHERE range_column IS EMPTY
// This will return rows where range_column is an empty range.

// 12. Checking if a range is not empty
// You can use the IS NOT EMPTY condition to check if the range is not empty.

// sql
// Копировать код
// WHERE range_column IS NOT EMPTY
// This will return rows where range_column is not an empty range.

// 13. Checking if a range is unbounded (lower bound or upper bound is open)
// You can check for unbounded ranges by testing for -infinity or infinity.

// sql
// Копировать код
// WHERE range_column LOWER = '-infinity'
// This will return rows where the lower bound of range_column is unbounded.

// sql
// Копировать код
// WHERE range_column UPPER = 'infinity'
// This will return rows where the upper bound of range_column is unbounded.

// 14. Checking if a range is not unbounded
// Similarly, you can check if a range is bounded by comparing it to -infinity or infinity.

// sql
// Копировать код
// WHERE range_column LOWER <> '-infinity'
// This will return rows where the lower bound of range_column is bounded.

// sql
// Копировать код
// WHERE range_column UPPER <> 'infinity'
// This will return rows where the upper bound of range_column is bounded.

// 15. Checking for the lower bound of a range
// You can access the lower bound of the range using the LOWER function.

// sql
// Копировать код
// WHERE LOWER(range_column) = 10
// This will return rows where the lower bound of range_column is 10.

// 16. Checking for the upper bound of a range
// You can access the upper bound of the range using the UPPER function.

// sql
// Копировать код
// WHERE UPPER(range_column) = 20
// This will return rows where the upper bound of range_column is 20.


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

////////////////////////////////
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EncodeFormat {
    Base64,
    Hex,
    Escape
}
impl std::fmt::Display for EncodeFormat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Base64 => write!(formatter, "base64"),
            Self::Hex => write!(formatter, "hex"),
            Self::Escape => write!(formatter, "escape"),
        }
    }
}
impl std::default::Default for EncodeFormat {
    fn default() -> Self {
        Self::Base64
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for EncodeFormat {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
///////////////////////////////////////////////////////