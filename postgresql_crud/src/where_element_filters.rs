generate_where_element_filters::generate_where_element_filters!();


// #[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema, serde :: Deserialize)]
// pub struct PostgresqlJsonTypeWhereElementEqual<T> {
//     pub logical_operator: crate::LogicalOperator,
//     value: T,
// }
// impl<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementEqual<T> {
//     fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
//         Self {
//             logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//             value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
//         }
//     }
// }
// impl<'a, T: std::marker::Send + serde::Serialize + 'a> crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementEqual<T> {
//     fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
//         match increment.checked_add(1) {
//             Some(value) => {
//                 *increment = value;
//                 Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
//             }
//             None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
//         }
//     }
//     fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
//         query = query.bind(sqlx::types::Json(self.value));
//         query
//     }
// }
/////////////////////////
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<T> {
    logical_operator: crate::LogicalOperator,
    value: T,
    // value:
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
pub enum PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl<T: crate::IsStringEmpty> PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<T> {
    fn try_new(logical_operator: crate::LogicalOperator, value: T) -> Result<Self, PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamed> {
        if !crate::IsStringEmpty::is_string_empty(&value) {
            Ok(Self { logical_operator, value })
        } else {
            Err(PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T> _serde::Deserialize<'de> for PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<T>
    where
        T: std::fmt::Debug + _serde::Deserialize<'de> + crate::IsStringEmpty,
    {
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
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "field identifier")
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
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<T>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: std::fmt::Debug + _serde::Deserialize<'de> + crate::IsStringEmpty,
            {
                type Value = PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<T>;
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "struct PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<T>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<crate::LogicalOperator> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<T> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
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
                    match PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<T>>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl<T: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl
    <
        'a,
        T: std::marker::Send + serde::Serialize + 'a
        
        + sqlx::Type<sqlx::Postgres>
        + sqlx::Encode<'a, sqlx::Postgres>
    > 
    crate::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementCaseInsensitiveRegularExpression<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                // println!("column {column}");
                // column animal_as_not_null_jsonb_object->'doggie_as_not_null_jsonb_object'->'column_113f3662_35a2_4a7a_9326_03bbd441815f'

// SELECT 'cat'::jsonb->>'' ~* 'john';  -- false
// SELECT '"johnny"'::jsonb->>'' ~* 'john';  -- true

                Ok(format!("{}({} ~* ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        println!("bind");
        query = query.bind(
            // sqlx::types::Json(
                self.value
            // )
        );
        query
    }
}
