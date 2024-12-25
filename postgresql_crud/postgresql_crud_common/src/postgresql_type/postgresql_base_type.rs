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
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementStdVecVecStdPrimitiveU8,
)]
pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);


#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    //todo utoipa::ToSchema,
    // postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensSqlxPostgresTypesPgInterval,
    // postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementSqlxPostgresTypesPgInterval,
)]
pub struct SqlxPostgresTypesPgInterval(pub sqlx::postgres::types::PgInterval);
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
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgInterval {
//     #[inline]
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(sqlx::postgres::types::PgInterval {
//             months: ::core::default::Default::default(),
//             days: ::core::default::Default::default(),
//             microseconds: ::core::default::Default::default(),
//         })
//     }
// }
//////////////////
impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgInterval {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl sqlx::Type<sqlx::Postgres> for SqlxPostgresTypesPgInterval {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxPostgresTypesPgInterval {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::postgres::types::PgInterval as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl crate::BindQuerySecond<'_> for SqlxPostgresTypesPgInterval {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                acc.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(acc)
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.0);
        query
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgInterval {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        // Self(::core::default::Default::default())
        //here
        Self(sqlx::postgres::types::PgInterval {
            months: ::core::default::Default::default(),
            days: ::core::default::Default::default(),
            microseconds: ::core::default::Default::default(),
        })
    }
}
impl SqlxPostgresTypesPgInterval {
    pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
        format!("{value} NOT NULL")
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub(crate) struct StdOptionOptionSqlxPostgresTypesPgInterval(pub std::option::Option<SqlxPostgresTypesPgInterval>);
impl sqlx::Type<sqlx::Postgres> for StdOptionOptionSqlxPostgresTypesPgInterval {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<sqlx::postgres::types::PgInterval> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdOptionOptionSqlxPostgresTypesPgInterval {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::option::Option<SqlxPostgresTypesPgInterval> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl crate::BindQuerySecond<'_> for StdOptionOptionSqlxPostgresTypesPgInterval {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                acc.push_str(&format!("${increment}"));
            }
            None => {
                return Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        }
        Ok(acc)
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(match self.0 {
            Some(value) => Some(value.0),
            None => None,
        });
        query
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionSqlxPostgresTypesPgInterval {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
impl StdOptionOptionSqlxPostgresTypesPgInterval {
    pub(crate) fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
        format!("{value}")
    }
}
impl crate::postgresql_type::postgresql_base_type_trait::PostgresqlBaseTypeSelfTraits<'_> for SqlxPostgresTypesPgInterval {}
impl crate::postgresql_type::postgresql_base_type_trait::PostgresqlBaseType<'_> for SqlxPostgresTypesPgInterval {
    type PostgresqlBaseTypeSelf = Self;
    type PostgresqlBaseTypeStdOptionOptionSelf = StdOptionOptionSqlxPostgresTypesPgInterval;
}








































////////////////////////////////////////////////////////////

// '6 years 5 months 4 days 3 hours 2 minutes 1 second';
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: SqlxPostgresTypesPgInterval,//here
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            //here
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElementEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value.0);//here
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement {
    Equal(PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElementEqual),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Equal(
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        )]
    }
}








// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeStdOptionOptionSqlxPostgresTypesPgIntervalWhereElementEqual {
//     pub logical_operator: crate::LogicalOperator,
//     pub value: std::option::Option<sqlx::postgres::types::PgInterval>,
// }
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionSqlxPostgresTypesPgIntervalWhereElementEqual {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             value: Some(::core::default::Default::default()),
//         }
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdOptionOptionSqlxPostgresTypesPgIntervalWhereElementEqual {
//     fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         let generate_query_part = |value: std::option::Option<std::primitive::u64>| {
//             let value = match value {
//                 Some(value) => format!("= ${}", value),
//                 None => "is null".to_string(),
//             };
//             format!("{}({} {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value)
//         };
//         if (&self.value).is_some() {
//             match increment.checked_add(1) {
//                 Some(value) => {
//                     *increment = value;
//                     Ok(generate_query_part(Some(value)))
//                 }
//                 None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//             }
//         } else {
//             Ok(generate_query_part(None))
//         }
//     }
//     fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         if let Some(value) = self.value {
//             query = query.bind(value);
//         }
//         query
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub enum PostgresqlTypeStdOptionOptionSqlxPostgresTypesPgIntervalWhereElement {
//     Equal(PostgresqlTypeStdOptionOptionSqlxPostgresTypesPgIntervalWhereElementEqual),
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdOptionOptionSqlxPostgresTypesPgIntervalWhereElement {
//     fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         match &self {
//             Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
//         }
//     }
//     fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         match self {
//             Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
//         }
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdOptionOptionSqlxPostgresTypesPgIntervalWhereElement {}
// impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdOptionOptionSqlxPostgresTypesPgIntervalWhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionSqlxPostgresTypesPgIntervalWhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         vec![Self::Equal(
//             crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//         )]
//     }
// }

////




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