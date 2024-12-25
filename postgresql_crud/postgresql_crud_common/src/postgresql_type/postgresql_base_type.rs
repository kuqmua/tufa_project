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
    // postgresql_crud_types_macro_logic_reuse::PostgresqlTypeTokensSqlxPostgresTypesPgInterval,
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
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgInterval {
    #[inline]
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(sqlx::postgres::types::PgInterval {
            months: ::core::default::Default::default(),
            days: ::core::default::Default::default(),
            microseconds: ::core::default::Default::default(),
        })
    }
}
//////////////////

impl std::fmt::Display for SqlxPostgresTypesPgInterval {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
// impl error_occurence_lib::ToStdStringString for SqlxPostgresTypesPgInterval {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxPostgresTypesPgInterval {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// impl crate::BindQuerySecond<'_> for SqlxPostgresTypesPgInterval {
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::BindQuerySecond::bind_value_to_query(self.0, query)
//     }
// }
// impl SqlxPostgresTypesPgInterval {
//     pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
//         sqlx::postgres::types::PgInterval::create_table_query_part_handle(value)
//     }
// }
// #[derive(
//     Debug,
//     Default,
//     Clone,
//     PartialEq,
//     serde :: Serialize,
//     serde ::
// Deserialize,
// )]
// pub struct PostgresqlTypeSqlxPostgresTypesPgIntervalColumn;
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxPostgresTypesPgIntervalColumn {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         ::core::default::Default::default()
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeSqlxPostgresTypesPgIntervalToCreate(sqlx::postgres::types::PgInterval);
// impl crate::BindQuerySecond<'_> for PostgresqlTypeSqlxPostgresTypesPgIntervalToCreate {
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::BindQuerySecond::bind_value_to_query(self.0, query)
//     }
// }
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxPostgresTypesPgIntervalToCreate {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToCreateTraits<'_> for PostgresqlTypeSqlxPostgresTypesPgIntervalToCreate {}
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeSqlxPostgresTypesPgIntervalToRead(sqlx::postgres::types::PgInterval);
// impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeSqlxPostgresTypesPgIntervalToRead {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <sqlx::postgres::types::PgInterval as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeSqlxPostgresTypesPgIntervalToRead {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToReadTraits<'_> for PostgresqlTypeSqlxPostgresTypesPgIntervalToRead {}
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeSqlxPostgresTypesPgIntervalToUpdate(sqlx::postgres::types::PgInterval);
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxPostgresTypesPgIntervalToUpdate {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToUpdateTraits<'_> for PostgresqlTypeSqlxPostgresTypesPgIntervalToUpdate {}
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub enum PostgresqlTypeSqlxPostgresTypesPgIntervalToUpdateQueryPartErrorNamed {
//     Todo,
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeSqlxPostgresTypesPgIntervalToDelete(sqlx::postgres::types::PgInterval);
// impl std::fmt::Display for PostgresqlTypeSqlxPostgresTypesPgIntervalToDelete {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", self.0)
//     }
// }
// impl error_occurence_lib::ToStdStringString for PostgresqlTypeSqlxPostgresTypesPgIntervalToDelete {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for PostgresqlTypeSqlxPostgresTypesPgIntervalToDelete {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <sqlx::postgres::types::PgInterval as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeSqlxPostgresTypesPgIntervalToDelete {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <sqlx::postgres::types::PgInterval as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
// }
// impl crate::BindQuerySecond<'_> for PostgresqlTypeSqlxPostgresTypesPgIntervalToDelete {
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::BindQuerySecond::try_generate_bind_increments(&self.0, increment)
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::BindQuerySecond::bind_value_to_query(self.0, query)
//     }
// }
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxPostgresTypesPgIntervalToDelete {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement(pub sqlx::postgres::types::PostgresqlTypePgIntervalWhereElement);
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement {
//     fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(&self.0, increment, column, is_need_to_add_logical_operator)
//     }
//     fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(self.0, query)
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement {}
// impl error_occurence_lib::ToStdStringString for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         < sqlx::postgres::types::PostgresqlTypePgIntervalWhereElement as crate
//         :: generate_postgresql_json_type ::
//         AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//         > ::
//         all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().into_iter().map(|
//         element | Self(element)).collect()
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
// pub struct PostgresqlTypeSqlxPostgresTypesPgIntervalWhere {
//     logical_operator: crate::LogicalOperator,
//     value: std::vec::Vec<PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement>,
// }
// #[derive(
//     Debug,
//     Clone,
//     serde :: Serialize,
//     serde :: Deserialize,
//     thiserror ::
// Error,
//     error_occurence_lib :: ErrorOccurence,
// )]
// pub enum PostgresqlTypeSqlxPostgresTypesPgIntervalWhereTryNewErrorNamed {
//     IsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUnique {
//         #[eo_to_std_string_string_serialize_deserialize]
//         value: PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl PostgresqlTypeSqlxPostgresTypesPgIntervalWhere {
//     fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement>) -> Result<Self, PostgresqlTypeSqlxPostgresTypesPgIntervalWhereTryNewErrorNamed> {
//         if value.is_empty() {
//             return Err(PostgresqlTypeSqlxPostgresTypesPgIntervalWhereTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         {
//             let mut acc = vec![];
//             for element in &value {
//                 if !acc.contains(&element) {
//                     acc.push(element);
//                 } else {
//                     return Err(PostgresqlTypeSqlxPostgresTypesPgIntervalWhereTryNewErrorNamed::NotUnique {
//                         value: element.clone(),
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     });
//                 }
//             }
//         }
//         Ok(Self { logical_operator, value })
//     }
// }
// const _: () = {
//     #[allow(unused_extern_crates, clippy::useless_attribute)]
//     extern crate serde as _serde;
//     #[automatically_derived]
//     impl<'de> _serde::Deserialize<'de> for PostgresqlTypeSqlxPostgresTypesPgIntervalWhere {
//         fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
//         where
//             __D: _serde::Deserializer<'de>,
//         {
//             #[allow(non_camel_case_types)]
//             #[doc(hidden)]
//             enum __Field {
//                 __field0,
//                 __field1,
//                 __ignore,
//             }
//             #[doc(hidden)]
//             struct __FieldVisitor;
//             impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
//                 type Value = __Field;
//                 fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
//                     _serde::__private::Formatter::write_str(__formatter, "field identifier")
//                 }
//                 fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
//                 where
//                     __E: _serde::de::Error,
//                 {
//                     match __value {
//                         0u64 => _serde::__private::Ok(__Field::__field0),
//                         1u64 => _serde::__private::Ok(__Field::__field1),
//                         _ => _serde::__private::Ok(__Field::__ignore),
//                     }
//                 }
//                 fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
//                 where
//                     __E: _serde::de::Error,
//                 {
//                     match __value {
//                         "logical_operator" => _serde::__private::Ok(__Field::__field0),
//                         "value" => _serde::__private::Ok(__Field::__field1),
//                         _ => _serde::__private::Ok(__Field::__ignore),
//                     }
//                 }
//                 fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
//                 where
//                     __E: _serde::de::Error,
//                 {
//                     match __value {
//                         b"logical_operator" => _serde::__private::Ok(__Field::__field0),
//                         b"value" => _serde::__private::Ok(__Field::__field1),
//                         _ => _serde::__private::Ok(__Field::__ignore),
//                     }
//                 }
//             }
//             impl<'de> _serde::Deserialize<'de> for __Field {
//                 #[inline]
//                 fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
//                 where
//                     __D: _serde::Deserializer<'de>,
//                 {
//                     _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
//                 }
//             }
//             #[doc(hidden)]
//             struct __Visitor<'de> {
//                 marker: _serde::__private::PhantomData<PostgresqlTypeSqlxPostgresTypesPgIntervalWhere>,
//                 lifetime: _serde::__private::PhantomData<&'de ()>,
//             }
//             impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
//                 type Value = PostgresqlTypeSqlxPostgresTypesPgIntervalWhere;
//                 fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
//                     _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeSqlxPostgresTypesPgIntervalWhere")
//                 }
//                 #[inline]
//                 fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
//                 where
//                     __A: _serde::de::SeqAccess<'de>,
//                 {
//                     let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
//                         _serde::__private::Some(__value) => __value,
//                         _serde::__private::None => {
//                             return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeSqlxPostgresTypesPgIntervalWhere with 2 elements"));
//                         }
//                     };
//                     let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement>>(&mut __seq)? {
//                         _serde::__private::Some(__value) => __value,
//                         _serde::__private::None => {
//                             return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeSqlxPostgresTypesPgIntervalWhere with 2 elements"));
//                         }
//                     };
//                     match PostgresqlTypeSqlxPostgresTypesPgIntervalWhere::try_new(__field0, __field1) {
//                         Ok(value) => _serde::__private::Ok(value),
//                         Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
//                     }
//                 }
//                 #[inline]
//                 fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
//                 where
//                     __A: _serde::de::MapAccess<'de>,
//                 {
//                     let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
//                     let mut __field1: _serde::__private::Option<std::vec::Vec<PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement>> = _serde::__private::None;
//                     while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
//                         match __key {
//                             __Field::__field0 => {
//                                 if _serde::__private::Option::is_some(&__field0) {
//                                     return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
//                                 }
//                                 __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
//                             }
//                             __Field::__field1 => {
//                                 if _serde::__private::Option::is_some(&__field1) {
//                                     return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
//                                 }
//                                 __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement>>(&mut __map)?);
//                             }
//                             _ => {
//                                 let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
//                             }
//                         }
//                     }
//                     let __field0 = match __field0 {
//                         _serde::__private::Some(__field0) => __field0,
//                         _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
//                     };
//                     let __field1 = match __field1 {
//                         _serde::__private::Some(__field1) => __field1,
//                         _serde::__private::None => _serde::__private::de::missing_field("value")?,
//                     };
//                     match PostgresqlTypeSqlxPostgresTypesPgIntervalWhere::try_new(__field0, __field1) {
//                         Ok(value) => _serde::__private::Ok(value),
//                         Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
//                     }
//                 }
//             }
//             #[doc(hidden)]
//             const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
//             _serde::Deserializer::deserialize_struct(
//                 __deserializer,
//                 "PostgresqlTypeSqlxPostgresTypesPgIntervalWhere",
//                 FIELDS,
//                 __Visitor {
//                     marker: _serde::__private::PhantomData::<PostgresqlTypeSqlxPostgresTypesPgIntervalWhere>,
//                     lifetime: _serde::__private::PhantomData,
//                 },
//             )
//         }
//     }
// };
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxPostgresTypesPgIntervalWhere {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//         }
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlType<'_> for SqlxPostgresTypesPgInterval {
//     type PostgresqlTypeSelf = Self;
//     type PostgresqlTypeSelfColumn = PostgresqlTypeSqlxPostgresTypesPgIntervalColumn;
//     fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String {
//         column.to_string()
//     }
//     type PostgresqlTypeSelfToCreate = PostgresqlTypeSqlxPostgresTypesPgIntervalToCreate;
//     type PostgresqlTypeSelfToRead = PostgresqlTypeSqlxPostgresTypesPgIntervalToRead;
//     type PostgresqlTypeSelfToUpdate = PostgresqlTypeSqlxPostgresTypesPgIntervalToUpdate;
//     type PostgresqlTypeSelfToUpdateQueryPartErrorNamed = PostgresqlTypeSqlxPostgresTypesPgIntervalToUpdateQueryPartErrorNamed;
//     fn postgresql_type_self_to_update_query_part(
//         postgresql_type_self_to_update: &Self::PostgresqlTypeSelfToUpdate,
//         jsonb_set_accumulator: &std::primitive::str,
//         jsonb_set_target: &std::primitive::str,
//         jsonb_set_path: &std::primitive::str,
//         increment: &mut std::primitive::u64,
//     ) -> Result<std::string::String, Self::PostgresqlTypeSelfToUpdateQueryPartErrorNamed> {
//         Ok(crate::BindQuerySecond::try_generate_bind_increments(&postgresql_type_self_to_update.0, increment).unwrap())
//     }
//     fn postgresql_type_self_to_update_bind_query_part<'a>(postgresql_type_self_to_update: Self::PostgresqlTypeSelfToUpdate, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::BindQuerySecond::bind_value_to_query(postgresql_type_self_to_update.0, query)
//     }
//     type PostgresqlTypeSelfWhereElement = PostgresqlTypeSqlxPostgresTypesPgIntervalWhereElement;
//     type PostgresqlTypeSelfWhere = PostgresqlTypeSqlxPostgresTypesPgIntervalWhere;
//     fn postgresql_type_self_where_try_generate_bind_increments(
//         postgresql_type_self_where: &Self::PostgresqlTypeSelfWhere,
//         increment: &mut std::primitive::u64,
//         column: &dyn std::fmt::Display,
//         is_need_to_add_logical_operator: std::primitive::bool,
//     ) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         let mut acc = std::string::String::default();
//         let mut is_need_to_add_logical_operator_inner_handle = false;
//         for element in &postgresql_type_self_where.value {
//             match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
//                 Ok(value) => {
//                     acc.push_str(&format!("{value} "));
//                     is_need_to_add_logical_operator_inner_handle = true;
//                 }
//                 Err(error) => {
//                     return Err(error);
//                 }
//             }
//         }
//         let _ = acc.pop();
//         Ok(format!("{}({acc})", &postgresql_type_self_where.logical_operator.to_query_part(is_need_to_add_logical_operator)))
//     }
//     fn postgresql_type_self_where_bind_value_to_query<'a>(postgresql_type_self_where: Self::PostgresqlTypeSelfWhere, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         for element in postgresql_type_self_where.value {
//             query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
//         }
//         query
//     }
// }












































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