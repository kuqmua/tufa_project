#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostgresqlTypeWhereElementEqual<T> {
    pub logical_operator: crate::LogicalOperator,
    pub value: T,
}
impl<T: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a + std::marker::Send> crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for PostgresqlTypeWhereElementEqual<T> {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}

#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeWhereElementGreaterThan<T> {
    pub logical_operator: crate::LogicalOperator,
    pub value: T,
}
impl<T: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a + std::marker::Send> crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for PostgresqlTypeWhereElementGreaterThan<T> {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PostgresqlTypeWhereElementBetween<T> {
    logical_operator: crate::LogicalOperator,
    start: T,
    end: T,
}
#[derive(
    Debug,
    Clone,
    serde :: Serialize,
    serde :: Deserialize,
    thiserror ::Error,
    // error_occurence_lib :: ErrorOccurence,
)]
pub enum PostgresqlTypeWhereElementBetweenTryNewErrorNamed<T> {
    StartMoreOrEqualToEnd {
        // #[eo_to_std_string_string_serialize_deserialize]
        start: T,
        // #[eo_to_std_string_string_serialize_deserialize]
        end: T,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
//
impl<T: error_occurence_lib::ToStdStringString> std::fmt::Display for PostgresqlTypeWhereElementBetweenTryNewErrorNamed<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}{}",
            match self {
                Self::StartMoreOrEqualToEnd { start, end, .. } => {
                    format!(
                        "start: {} end: {}",
                        error_occurence_lib::ToStdStringString::to_std_string_string(start),
                        error_occurence_lib::ToStdStringString::to_std_string_string(end)
                    )
                }
            },
            match self {
                Self::StartMoreOrEqualToEnd { code_occurence, .. } => code_occurence,
            }
        )
    }
}
impl<T> PostgresqlTypeWhereElementBetweenTryNewErrorNamed<T> {
    pub fn into_serialize_deserialize_version(self) -> PostgresqlTypeWhereElementBetweenTryNewErrorNamedWithSerializeDeserialize<T> {
        #[allow(clippy::redundant_closure_for_method_calls)]
        match self {
            Self::StartMoreOrEqualToEnd {
                start,
                end,
                code_occurence
            } => PostgresqlTypeWhereElementBetweenTryNewErrorNamedWithSerializeDeserialize::StartMoreOrEqualToEnd {
                start,
                end,
                code_occurence
            },
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeWhereElementBetweenTryNewErrorNamedWithSerializeDeserialize<T> {
    StartMoreOrEqualToEnd {
        start: T,
        end: T,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence
    },
}
impl<T: error_occurence_lib::ToStdStringString> std::fmt::Display for PostgresqlTypeWhereElementBetweenTryNewErrorNamedWithSerializeDeserialize<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}{}",
            match self {
                Self::StartMoreOrEqualToEnd { start, end, .. } => {
                    format!(
                        "start: {} end: {}",
                        error_occurence_lib::ToStdStringString::to_std_string_string(start),
                        error_occurence_lib::ToStdStringString::to_std_string_string(end)
                    )
                }
            },
            match self {
                Self::StartMoreOrEqualToEnd { code_occurence, .. } => code_occurence,
            }
        )
    }
}
impl<T: error_occurence_lib::ToStdStringString> error_occurence_lib::ToStdStringString for PostgresqlTypeWhereElementBetweenTryNewErrorNamedWithSerializeDeserialize<T> {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self}")
    }
}
//
impl<T: std::cmp::PartialOrd> PostgresqlTypeWhereElementBetween<T> {
    fn try_new(
        logical_operator: crate::LogicalOperator,
        start: T,
        end: T,
    ) -> Result<Self, PostgresqlTypeWhereElementBetweenTryNewErrorNamed<T>> {
        if start < end {//removed .0
            Ok(Self { logical_operator, start, end })
        } else {
            Err(PostgresqlTypeWhereElementBetweenTryNewErrorNamed::StartMoreOrEqualToEnd {
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
    impl<'de, T> _serde::Deserialize<'de> for PostgresqlTypeWhereElementBetween<T>
    where
        T: _serde::Deserialize<'de> + std::cmp::PartialOrd + std::fmt::Debug,
    {
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
                __field2,
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
                        2u64 => _serde::__private::Ok(__Field::__field2),
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
                        "logical_operator" => {
                            _serde::__private::Ok(__Field::__field0)
                        }
                        "start" => _serde::__private::Ok(__Field::__field1),
                        "end" => _serde::__private::Ok(__Field::__field2),
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
                        b"logical_operator" => {
                            _serde::__private::Ok(__Field::__field0)
                        }
                        b"start" => _serde::__private::Ok(__Field::__field1),
                        b"end" => _serde::__private::Ok(__Field::__field2),
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
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<
                    PostgresqlTypeWhereElementBetween<T>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de> + std::cmp::PartialOrd + std::fmt::Debug,
            {
                type Value = PostgresqlTypeWhereElementBetween<T>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter<'_>,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct PostgresqlTypeWhereElementBetween",
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
                                    &"struct PostgresqlTypeWhereElementBetween with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        T,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct PostgresqlTypeWhereElementBetween with 3 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        T,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct PostgresqlTypeWhereElementBetween with 3 elements",
                                ),
                            );
                        }
                    };
                    match PostgresqlTypeWhereElementBetween::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
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
                    let mut __field1: _serde::__private::Option<T> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<T> = _serde::__private::None;
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<T>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<T>(&mut __map)?,
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
                            _serde::__private::de::missing_field("start")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("end")?
                        }
                    };
                    match PostgresqlTypeWhereElementBetween::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "logical_operator",
                "start",
                "end",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeWhereElementBetween",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        PostgresqlTypeWhereElementBetween<T>,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl<T: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementBetween<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            start: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            end: crate::generate_postgresql_json_type::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Encode<'a, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + 'a + std::marker::Send> crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter<'a> for PostgresqlTypeWhereElementBetween<T> {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::QueryPartErrorNamed> {
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
                    None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.start);//here change
        query = query.bind(self.end);//here change
        query
    }
}