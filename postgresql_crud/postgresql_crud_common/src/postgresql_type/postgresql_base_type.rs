#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypePrimaryKeyTokens,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokensWhereElementInt,
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
)]
pub struct StdPrimitiveI64(pub std::primitive::i64);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
)]
pub struct StdPrimitiveF32(pub std::primitive::f32);
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    postgresql_crud_types_macro_logic_reuse::PostgresqlBaseTypeTokens,
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
)]
pub struct StdStringString(pub std::string::String);
//
// // pub struct StdVecVecStdPrimitiveU8(pub std::vec::Vec<std::primitive::u8>);
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


////////////////////////////////////////
// #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
// pub struct PostgresqlTypeStdPrimitiveI16WhereElementIn {
//     logical_operator: crate::LogicalOperator,
//     value: std::vec::Vec<std::primitive::i16>,
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
// pub enum PostgresqlTypeStdPrimitiveI16WhereElementInTryNewErrorNamed {
//     IsEmpty {
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotUnique {
//         #[eo_to_std_string_string_serialize_deserialize]
//         value: std::primitive::i16,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }
// impl PostgresqlTypeStdPrimitiveI16WhereElementIn {
//     fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<std::primitive::i16>) -> Result<Self, PostgresqlTypeStdPrimitiveI16WhereElementInTryNewErrorNamed> {
//         if value.is_empty() {
//             return Err(PostgresqlTypeStdPrimitiveI16WhereElementInTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
//         }
//         {
//             let mut acc = vec![];
//             for element in &value {
//                 if !acc.contains(&element) {
//                     acc.push(element);
//                 } else {
//                     return Err(PostgresqlTypeStdPrimitiveI16WhereElementInTryNewErrorNamed::NotUnique {
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
//     impl<'de> _serde::Deserialize<'de> for PostgresqlTypeStdPrimitiveI16WhereElementIn {
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
//                 marker: _serde::__private::PhantomData<PostgresqlTypeStdPrimitiveI16WhereElementIn>,
//                 lifetime: _serde::__private::PhantomData<&'de ()>,
//             }
//             impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
//                 type Value = PostgresqlTypeStdPrimitiveI16WhereElementIn;
//                 fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
//                     _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeStdPrimitiveI16WhereElementIn")
//                 }
//                 #[inline]
//                 fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
//                 where
//                     __A: _serde::de::SeqAccess<'de>,
//                 {
//                     let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
//                         _serde::__private::Some(__value) => __value,
//                         _serde::__private::None => {
//                             return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeStdPrimitiveI16WhereElementIn with 2 elements"));
//                         }
//                     };
//                     let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<std::primitive::i16>>(&mut __seq)? {
//                         _serde::__private::Some(__value) => __value,
//                         _serde::__private::None => {
//                             return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeStdPrimitiveI16WhereElementIn with 2 elements"));
//                         }
//                     };
//                     match PostgresqlTypeStdPrimitiveI16WhereElementIn::try_new(__field0, __field1) {
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
//                     let mut __field1: _serde::__private::Option<std::vec::Vec<std::primitive::i16>> = _serde::__private::None;
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
//                                 __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<std::primitive::i16>>(&mut __map)?);
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
//                     match PostgresqlTypeStdPrimitiveI16WhereElementIn::try_new(__field0, __field1) {
//                         Ok(value) => _serde::__private::Ok(value),
//                         Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
//                     }
//                 }
//             }
//             #[doc(hidden)]
//             const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
//             _serde::Deserializer::deserialize_struct(
//                 __deserializer,
//                 "PostgresqlTypeStdPrimitiveI16WhereElementIn",
//                 FIELDS,
//                 __Visitor {
//                     marker: _serde::__private::PhantomData::<PostgresqlTypeStdPrimitiveI16WhereElementIn>,
//                     lifetime: _serde::__private::PhantomData,
//                 },
//             )
//         }
//     }
// };
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16WhereElementIn {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self {
//             logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
//             value: vec![crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
//         }
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16WhereElementIn {
//     fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         let mut acc = std::string::String::default();
//         for element in &self.value {
//             match crate::BindQuerySecond::try_generate_bind_increments(element, increment) {
//                 Ok(value) => {
//                     acc.push_str(&format!("{value},"));
//                 }
//                 Err(error) => {
//                     return Err(error);
//                 }
//             }
//         }
//         let _ = acc.pop();
//         let in_snake_case = naming::InSnakeCase;
//         Ok(format!("{}({column} {in_snake_case} ({acc}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
//     }
//     fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         for element in self.value {
//             query = crate::BindQuerySecond::bind_value_to_query(element, query);
//         }
//         query
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub enum PostgresqlTypeStdPrimitiveI16WhereElement {
//     Equal(PostgresqlTypeStdPrimitiveI16WhereElementEqual),
//     GreaterThan(PostgresqlTypeStdPrimitiveI16WhereElementGreaterThan),
//     Between(PostgresqlTypeStdPrimitiveI16WhereElementBetween),
//     In(PostgresqlTypeStdPrimitiveI16WhereElementIn),
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16WhereElement {
//     fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         match &self {
//             Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
//             Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
//             Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
//             Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
//         }
//     }
//     fn postgresql_type_self_where_bind_value_to_query<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         match self {
//             Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
//             Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
//             Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
//             Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
//         }
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI16WhereElement {}
// impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI16WhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16WhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         vec![
//             Self::Equal(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
//             Self::GreaterThan(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
//             Self::Between(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
//             Self::In(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
//         ]
//     }
// }
