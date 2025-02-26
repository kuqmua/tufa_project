postgresql_crud_types_macro_logic_reuse::generate_postgresql_types!();

// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct StdPrimitiveI16AsPostgresqlInt2NotNull(pub std::primitive::i16);
// impl std::fmt::Display for StdPrimitiveI16AsPostgresqlInt2NotNull {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", self.0)
//     }
// }
// impl error_occurence_lib::ToStdStringString for StdPrimitiveI16AsPostgresqlInt2NotNull {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI16AsPostgresqlInt2NotNull {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(::core::default::Default::default())
//     }
// }
// impl crate::BindQuery<'_> for StdPrimitiveI16AsPostgresqlInt2NotNull {
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 Ok(format!("${increment}"))
//             }
//             None => {
//                 return Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
//             }
//         }
        
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(self.0);
//         query
//     }
// }
// impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI16AsPostgresqlInt2NotNull {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <std::primitive::i16 as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
//         <std::primitive::i16 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI16AsPostgresqlInt2NotNull {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <std::primitive::i16 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToReadTraits<'_> for StdPrimitiveI16AsPostgresqlInt2NotNull {}
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToUpdateTraits<'_> for StdPrimitiveI16AsPostgresqlInt2NotNull {}
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfTraits<'_> for StdPrimitiveI16AsPostgresqlInt2NotNull {}

// impl crate::CreateTableColumnQueryPart for StdPrimitiveI16AsPostgresqlInt2NotNull {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} int2 not null")
//     }
// }

// impl crate::postgresql_type::postgresql_type_trait::PostgresqlBaseType<'_> for StdPrimitiveI16AsPostgresqlInt2NotNull {
//     type PostgresqlBaseTypeSelf = Self;
//     type PostgresqlBaseTypeStdOptionOptionSelf = StdPrimitiveI16AsPostgresqlInt2Nullable;
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
// pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullColumn;
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullColumn {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         ::core::default::Default::default()
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullToCreate(StdPrimitiveI16AsPostgresqlInt2NotNull);
// impl crate::BindQuery<'_> for StdPrimitiveI16AsPostgresqlInt2NotNullToCreate {
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::BindQuery::try_generate_bind_increments(&self.0, increment)
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::BindQuery::bind_value_to_query(self.0, query)
//     }
// }
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI16AsPostgresqlInt2NotNullToCreate {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToCreateTraits<'_> for StdPrimitiveI16AsPostgresqlInt2NotNullToCreate {}
// pub type PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullToRead = StdPrimitiveI16AsPostgresqlInt2NotNull;
// pub type PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullToUpdate = StdPrimitiveI16AsPostgresqlInt2NotNull;
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub enum PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullToUpdateQueryPartErrorNamed {
//     Todo,
// }
// pub type PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullToDelete = StdPrimitiveI16AsPostgresqlInt2NotNull;
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement(pub PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElement);
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement {
//     fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(&self.0, increment, column, is_need_to_add_logical_operator)
//     }
//     fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(self.0, query)
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement {}
// impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         < PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElement as crate
//         :: generate_postgresql_json_type ::
//         AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//         > ::
//         all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().into_iter().map(|
//         element | Self(element)).collect()
//     }
// }
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement>,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement>) -> Result<Self, PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { logical_operator, value })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
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
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere with 2 elements"));
                        }
                    };
                    match PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &self.value {
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
        Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.value {
            query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
        }
        query
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}








//todo rename PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2 to StdPrimitiveI16AsPostgresqlInt2NotNull
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::primitive::i16,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementEqual {
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
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementGreaterThan {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::primitive::i16,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementGreaterThan {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementGreaterThan {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween {
    logical_operator: crate::LogicalOperator,
    start: std::primitive::i16,
    end: std::primitive::i16,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetweenTryNewErrorNamed {
    StartMoreOrEqualToEnd {
        #[eo_to_std_string_string_serialize_deserialize]
        start: std::primitive::i16,
        #[eo_to_std_string_string_serialize_deserialize]
        end: std::primitive::i16,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween {
    fn try_new(logical_operator: crate::LogicalOperator, start: std::primitive::i16, end: std::primitive::i16) -> Result<Self, PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetweenTryNewErrorNamed> {
        if start < end {
            Ok(Self { logical_operator, start, end })
        } else {
            Err(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetweenTryNewErrorNamed::StartMoreOrEqualToEnd {
                start,
                end,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "start" => _serde::__private::Ok(__Field::__field1),
                        "end" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"start" => _serde::__private::Ok(__Field::__field1),
                        b"end" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::primitive::i16>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::i16>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween with 3 elements"));
                        }
                    };
                    match PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::primitive::i16> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::i16> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("start"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i16>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("end"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i16>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("start")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("end")?,
                    };
                    match PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            start: ::core::default::Default::default(),
            end: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(first_value) => {
                *increment = first_value;
                match increment.checked_add(1) {
                    Some(second_value) => {
                        *increment = second_value;
                        let between_snake_case = naming::BetweenSnakeCase;
                        let and_snake_case = naming::AndSnakeCase;
                        Ok(format!("{}({column} {between_snake_case} ${first_value} {and_snake_case} ${second_value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                    }
                    None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.start);
        query = query.bind(self.end);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<std::primitive::i16>,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementInTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::i16,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<std::primitive::i16>) -> Result<Self, PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementInTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementInTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementInTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { logical_operator, value })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
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
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<std::primitive::i16>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn with 2 elements"));
                        }
                    };
                    match PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::vec::Vec<std::primitive::i16>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<std::primitive::i16>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: vec![::core::default::Default::default()],
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        for element in &self.value {
            match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    acc.push_str(&format!("${},", value));
                }
                None => {
                    return Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
        }
        let _ = acc.pop();
        let in_snake_case = naming::InSnakeCase;
        Ok(format!("{}({} {in_snake_case} ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, acc))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.value {
            query = query.bind(element);
        }
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement {
    Equal(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementEqual),
    GreaterThan(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementGreaterThan),
    Between(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween),
    In(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::GreaterThan(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::Between(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::In(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}




impl crate::postgresql_type::postgresql_type_trait::PostgresqlType<'_> for StdPrimitiveI16AsPostgresqlInt2NotNull {
    type PostgresqlTypeSelf = Self;
    type PostgresqlTypeSelfColumn = StdPrimitiveI16AsPostgresqlInt2NotNullColumn;
    fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type PostgresqlTypeSelfToCreate = StdPrimitiveI16AsPostgresqlInt2NotNullToCreate;
    type PostgresqlTypeSelfToRead = StdPrimitiveI16AsPostgresqlInt2NotNullToRead;
    type PostgresqlTypeSelfToUpdate = StdPrimitiveI16AsPostgresqlInt2NotNullToUpdate;
    type PostgresqlTypeSelfToUpdateQueryPartErrorNamed = StdPrimitiveI16AsPostgresqlInt2NotNullToUpdateQueryPartErrorNamed;
    fn postgresql_type_self_to_update_query_part(
        postgresql_type_self_to_update: &Self::PostgresqlTypeSelfToUpdate,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlTypeSelfToUpdateQueryPartErrorNamed> {
        Ok(crate::BindQuery::try_generate_bind_increments(postgresql_type_self_to_update, increment).unwrap())
    }
    fn postgresql_type_self_to_update_bind_query_part<'a>(postgresql_type_self_to_update: Self::PostgresqlTypeSelfToUpdate, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(postgresql_type_self_to_update, query)
    }
    type PostgresqlTypeSelfWhereElement = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhereElement;
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NotNullWhere;
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





















































// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct StdPrimitiveI16AsPostgresqlInt2Nullable(pub std::option::Option<StdPrimitiveI16AsPostgresqlInt2NotNull>);
// impl std::fmt::Display for StdPrimitiveI16AsPostgresqlInt2Nullable {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{:?}", self.0)
//     }
// }
// impl error_occurence_lib::ToStdStringString for StdPrimitiveI16AsPostgresqlInt2Nullable {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self}")
//     }
// }
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI16AsPostgresqlInt2Nullable {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(Some(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()))
//     }
// }
// impl crate::BindQuery<'_> for StdPrimitiveI16AsPostgresqlInt2Nullable {
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 Ok(format!("${increment}"))
//             }
//             None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() })
//         }
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(match self.0 {
//             Some(value) => Some(value.0),
//             None => None,
//         });
//         query
//     }
// }

// impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI16AsPostgresqlInt2Nullable {
//     fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
//         <std::option::Option<StdPrimitiveI16AsPostgresqlInt2NotNull> as sqlx::Type<sqlx::Postgres>>::type_info()
//     }
//     fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
//         <std::option::Option<StdPrimitiveI16AsPostgresqlInt2NotNull> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
// impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI16AsPostgresqlInt2Nullable {
//     fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
//         match <std::option::Option<StdPrimitiveI16AsPostgresqlInt2NotNull> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
//             Ok(value) => Ok(Self(value)),
//             Err(error) => Err(error),
//         }
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToReadTraits<'_> for StdPrimitiveI16AsPostgresqlInt2Nullable {}
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToUpdateTraits<'_> for StdPrimitiveI16AsPostgresqlInt2Nullable {}
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfTraits<'_> for StdPrimitiveI16AsPostgresqlInt2Nullable {}

// impl crate::CreateTableColumnQueryPart for StdPrimitiveI16AsPostgresqlInt2Nullable {
//     fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
//         format!("{column} int2")
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
// pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableColumn;
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableColumn {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         ::core::default::Default::default()
//     }
// }
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableToCreate(StdPrimitiveI16AsPostgresqlInt2Nullable);
// impl crate::BindQuery<'_> for StdPrimitiveI16AsPostgresqlInt2NullableToCreate {
//     fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::BindQuery::try_generate_bind_increments(&self.0, increment)
//     }
//     fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::BindQuery::bind_value_to_query(self.0, query)
//     }
// }
// impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI16AsPostgresqlInt2NullableToCreate {
//     fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
//         Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToCreateTraits<'_> for StdPrimitiveI16AsPostgresqlInt2NullableToCreate {}
// pub type PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableToRead = StdPrimitiveI16AsPostgresqlInt2Nullable;
// pub type PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableToUpdate = StdPrimitiveI16AsPostgresqlInt2Nullable;
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub enum PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableToUpdateQueryPartErrorNamed {
//     Todo,
// }
// pub type PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableToDelete = StdPrimitiveI16AsPostgresqlInt2Nullable;
// #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
// pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement(pub PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElement);
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement {
//     fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
//         crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(&self.0, increment, column, is_need_to_add_logical_operator)
//     }
//     fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(self.0, query)
//     }
// }
// impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement {}
// impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement {
//     fn to_std_string_string(&self) -> std::string::String {
//         format!("{self:#?}")
//     }
// }
// impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement {
//     fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
//         <
//         PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElement
//         as crate :: generate_postgresql_json_type ::
//         AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
//         > ::
//         all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().into_iter().map(|
//         element | Self(element)).collect()
//     }
// }
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement>,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement>) -> Result<Self, PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { logical_operator, value })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
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
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere with 2 elements"));
                        }
                    };
                    match PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &self.value {
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
        Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.value {
            query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
        }
        query
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::option::Option<std::primitive::i16>,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: Some(::core::default::Default::default()),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        if self.value.is_some() {
            match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
                }
                None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            }
        } else {
            Ok(format!("{}({} is null)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column,))
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let Some(value) = self.value {
            query = query.bind(value);
        }
        query
    }
}
pub type PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementGreaterThan = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementGreaterThan;
pub type PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementBetween = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementBetween;
pub type PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementIn = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2WhereElementIn;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement {
    Equal(PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementEqual),
    GreaterThan(PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementGreaterThan),
    Between(PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementBetween),
    In(PostgresqlTypeStdOptionOptionStdPrimitiveI16AsPostgresqlInt2WhereElementIn),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::GreaterThan(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::Between(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::In(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}



impl crate::postgresql_type::postgresql_type_trait::PostgresqlType<'_> for StdPrimitiveI16AsPostgresqlInt2Nullable {
    type PostgresqlTypeSelf = Self;
    type PostgresqlTypeSelfColumn = StdPrimitiveI16AsPostgresqlInt2NullableColumn;
    fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type PostgresqlTypeSelfToCreate = StdPrimitiveI16AsPostgresqlInt2NullableToCreate;
    type PostgresqlTypeSelfToRead = StdPrimitiveI16AsPostgresqlInt2NullableToRead;
    type PostgresqlTypeSelfToUpdate = StdPrimitiveI16AsPostgresqlInt2NullableToUpdate;
    type PostgresqlTypeSelfToUpdateQueryPartErrorNamed = StdPrimitiveI16AsPostgresqlInt2NullableToUpdateQueryPartErrorNamed;
    fn postgresql_type_self_to_update_query_part(
        postgresql_type_self_to_update: &Self::PostgresqlTypeSelfToUpdate,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlTypeSelfToUpdateQueryPartErrorNamed> {
        Ok(crate::BindQuery::try_generate_bind_increments(postgresql_type_self_to_update, increment).unwrap())
    }
    fn postgresql_type_self_to_update_bind_query_part<'a>(postgresql_type_self_to_update: Self::PostgresqlTypeSelfToUpdate, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(postgresql_type_self_to_update, query)
    }
    type PostgresqlTypeSelfWhereElement = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhereElement;
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdPrimitiveI16AsPostgresqlInt2NullableWhere;
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


///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////



#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql(std::primitive::i64);
impl crate::CreateTableColumnQueryPart for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} bigserial")
    }
}
impl std::fmt::Display for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl sqlx::Type<sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::primitive::i64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::primitive::i64 as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl crate::BindQuery<'_> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
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
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
        format!("{value} not null")
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToReadTraits<'_> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToUpdateTraits<'_> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql(pub std::option::Option<StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql>);
impl sqlx::Type<sqlx::Postgres> for StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::option::Option<StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <std::option::Option<StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <std::option::Option<StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl crate::BindQuery<'_> for StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
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
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(Some(
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ))
    }
}
impl StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
        format!("{value}")
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToReadTraits<'_> for StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToUpdateTraits<'_> for StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfTraits<'_> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfTraits<'_> for StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlBaseType<'_> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    type PostgresqlBaseTypeSelf = Self;
    type PostgresqlBaseTypeStdOptionOptionSelf = StdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql;
}
impl sqlx::Encode<'_, sqlx::Postgres> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl sqlx::postgres::PgHasArrayType for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <std::primitive::i64 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlBaseTypePrimaryKey<'_> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql {
    type PostgresqlBaseTypeSelf = Self;
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::primitive::i64,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementEqual {
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
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementGreaterThan {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::primitive::i64,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementGreaterThan {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementGreaterThan {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween {
    logical_operator: crate::LogicalOperator,
    start: std::primitive::i64,
    end: std::primitive::i64,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetweenTryNewErrorNamed {
    StartMoreOrEqualToEnd {
        #[eo_to_std_string_string_serialize_deserialize]
        start: std::primitive::i64,
        #[eo_to_std_string_string_serialize_deserialize]
        end: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween {
    fn try_new(logical_operator: crate::LogicalOperator, start: std::primitive::i64, end: std::primitive::i64) -> Result<Self, PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetweenTryNewErrorNamed> {
        if start < end {
            Ok(Self { logical_operator, start, end })
        } else {
            Err(PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetweenTryNewErrorNamed::StartMoreOrEqualToEnd {
                start,
                end,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "start" => _serde::__private::Ok(__Field::__field1),
                        "end" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"start" => _serde::__private::Ok(__Field::__field1),
                        b"end" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween with 3 elements"));
                        }
                    };
                    match PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::primitive::i64> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::i64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("start"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("end"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("start")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("end")?,
                    };
                    match PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            start: ::core::default::Default::default(),
            end: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(first_value) => {
                *increment = first_value;
                match increment.checked_add(1) {
                    Some(second_value) => {
                        *increment = second_value;
                        let between_snake_case = naming::BetweenSnakeCase;
                        let and_snake_case = naming::AndSnakeCase;
                        Ok(format!("{}({column} {between_snake_case} ${first_value} {and_snake_case} ${second_value})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
                    }
                    None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.start);
        query = query.bind(self.end);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<std::primitive::i64>,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementInTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<std::primitive::i64>) -> Result<Self, PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementInTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementInTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementInTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { logical_operator, value })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
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
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<std::primitive::i64>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn with 2 elements"));
                        }
                    };
                    match PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::vec::Vec<std::primitive::i64>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<std::primitive::i64>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: vec![::core::default::Default::default()],
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        for element in &self.value {
            match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    acc.push_str(&format!("${},", value));
                }
                None => {
                    return Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
        }
        let _ = acc.pop();
        let in_snake_case = naming::InSnakeCase;
        Ok(format!("{}({} {in_snake_case} ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, acc))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.value {
            query = query.bind(element);
        }
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {
    Equal(PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementEqual),
    GreaterThan(PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementGreaterThan),
    Between(PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween),
    In(PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {
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
pub struct PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::option::Option<std::primitive::i64>,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: Some(::core::default::Default::default()),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        if self.value.is_some() {
            match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
                }
                None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            }
        } else {
            Ok(format!("{}({} is null)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column,))
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let Some(value) = self.value {
            query = query.bind(value);
        }
        query
    }
}
pub type PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementGreaterThan = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementGreaterThan;
pub type PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween;
pub type PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {
    Equal(PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementEqual),
    GreaterThan(PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementGreaterThan),
    Between(PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementBetween),
    In(PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElementIn),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::GreaterThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::Between(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::In(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdOptionOptionStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement {
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
pub struct StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull(StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql);
impl std::fmt::Display for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}
impl error_occurence_lib::ToStdStringString for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element())
    }
}
impl crate::BindQuery<'_> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(&self.0, increment)
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(self.0, query)
    }
}
impl StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull {
    pub fn create_table_query_part_handle(value: &dyn std::fmt::Display) -> impl std::fmt::Display {
        StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql::create_table_query_part_handle(value)
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
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullColumn;
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullColumn {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToCreate(());
impl crate::BindQuery<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToCreate {
    fn try_generate_bind_increments(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        Ok(std::string::String::from("DEFAULT"))
    }
    fn bind_value_to_query(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToCreate {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(::core::default::Default::default())
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfToCreateTraits<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToCreate {}
pub type PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToRead = StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql;
pub type PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToUpdate = StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToUpdateQueryPartErrorNamed {
    Todo,
}
pub type PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToDelete = StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement(pub PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement);
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(&self.0, increment, column, is_need_to_add_logical_operator)
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(self.0, query)
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        <
        PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlWhereElement
        as crate :: generate_postgresql_json_type ::
        AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement
        > ::
        all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element().into_iter().map(|
        element | Self(element)).collect()
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement>,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::
Error,
    error_occurence_lib :: ErrorOccurence,
)]
pub enum PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement>) -> Result<Self, PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { logical_operator, value })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "logical_operator" => _serde::__private::Ok(__Field::__field0),
                        "value" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
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
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere with 2 elements"));
                        }
                    };
                    match PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<std::vec::Vec<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("value"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("logical_operator")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    match PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        for element in &self.value {
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
        Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.value {
            query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(element, query);
        }
        query
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlType<'_> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull {
    type PostgresqlTypeSelf = Self;
    type PostgresqlTypeSelfColumn = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullColumn;
    fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type PostgresqlTypeSelfToCreate = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToCreate;
    type PostgresqlTypeSelfToRead = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToRead;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToUpdate;
    type PostgresqlTypeSelfToUpdateQueryPartErrorNamed = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToUpdateQueryPartErrorNamed;
    fn postgresql_type_self_to_update_query_part(
        postgresql_type_self_to_update: &Self::PostgresqlTypeSelfToUpdate,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlTypeSelfToUpdateQueryPartErrorNamed> {
        Ok(crate::BindQuery::try_generate_bind_increments(postgresql_type_self_to_update, increment).unwrap())
    }
    fn postgresql_type_self_to_update_bind_query_part<'a>(postgresql_type_self_to_update: Self::PostgresqlTypeSelfToUpdate, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(postgresql_type_self_to_update, query)
    }
    type PostgresqlTypeSelfWhereElement = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhereElement;
    type PostgresqlTypeSelfWhere = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullWhere;
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
impl crate::CreateTableColumnQueryPart for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull {
    fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!(
            "{} not null {}",
            <StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql as crate::CreateTableColumnQueryPart>::create_table_column_query_part(column, is_primary_key),
            crate::maybe_primary_key(is_primary_key)
        )
    }
}
impl sqlx::Type<sqlx::Postgres> for PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToCreate {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresql as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypePrimaryKey<'_> for StdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNull {
    type PostgresqlTypeSelfToCreate = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToCreate;
    type PostgresqlTypeSelfToRead = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToRead;
    type PostgresqlTypeSelfToUpdate = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToUpdate;
    type PostgresqlTypeSelfToDelete = PostgresqlTypeStdPrimitiveI64AsPostgresqlBigSerialInitializedByPostgresqlNotNullToDelete;
}




///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////
///////////////////////////
///////////////////////////
///////////////////////////
///////////////////////////
////////////////////////////
///////////////////////////
///////////////////////////
//////////////////////////
/////////////////////
////////////////////////
/////////////////////////
////////////////////////
////////////////////////
////////////////////////
//////////////////