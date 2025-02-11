postgresql_crud_types_macro_logic_reuse::generate_postgresql_json_types!();

#[derive(Debug, Clone, PartialEq, PartialOrd, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdVecVecStdVecVecUuidUuid(pub std::vec::Vec<std::vec::Vec<UuidUuid>>);
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for StdVecVecStdVecVecUuidUuid {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self(vec![vec![
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        ]])
    }
}
impl error_occurence_lib::ToStdStringString for StdVecVecStdVecVecUuidUuid {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
pub type PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidToCreate = StdVecVecStdVecVecUuidUuid;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Serialize,
    serde ::
Deserialize,
    utoipa :: ToSchema,
    schemars :: JsonSchema,
)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidFieldReader {
    pagination: crate::pagination::Pagination,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidFieldReader {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            pagination: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub type PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidOptionsToRead = StdVecVecStdVecVecUuidUuid;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed::NotUnique {
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
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere {
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere::try_new(__field0, __field1) {
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
                    let mut __field1: _serde::__private::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>>(&mut __map)?);
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
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere {
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
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: StdVecVecStdVecVecUuidUuid,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementEqual {
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
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual {
    logical_operator: crate::LogicalOperator,
    value: std::primitive::i64,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqualTryNewErrorNamed {
    LengthIsNegative {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::primitive::i64) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqualTryNewErrorNamed> {
        if value >= 0 {
            Ok(Self { logical_operator, value })
        } else {
            Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqualTryNewErrorNamed::LengthIsNegative {
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual {
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual::try_new(__field0, __field1) {
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
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
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(jsonb_array_length({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan {
    logical_operator: crate::LogicalOperator,
    value: std::primitive::i64,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThanTryNewErrorNamed {
    LengthIsNegative {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::i64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::primitive::i64) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThanTryNewErrorNamed> {
        if value >= 0 {
            Ok(Self { logical_operator, value })
        } else {
            Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThanTryNewErrorNamed::LengthIsNegative {
                value,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan {
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::primitive::i64>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan::try_new(__field0, __field1) {
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i64>(&mut __map)?);
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
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(jsonb_array_length({}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals {
    logical_operator: crate::LogicalOperator,
    value: StdVecVecStdVecVecUuidUuid,
    position: std::primitive::i32,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEqualsTryNewErrorNamed {
    PositionIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        position: std::primitive::i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals {
    fn try_new(logical_operator: crate::LogicalOperator, value: StdVecVecStdVecVecUuidUuid, position: std::primitive::i32) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEqualsTryNewErrorNamed> {
        if position >= 0 {
            Ok(Self { logical_operator, value, position })
        } else {
            Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEqualsTryNewErrorNamed::PositionIsLessThanZero {
                position,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals {
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
                        "value" => _serde::__private::Ok(__Field::__field1),
                        "position" => _serde::__private::Ok(__Field::__field2),
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
                        b"position" => _serde::__private::Ok(__Field::__field2),
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<StdVecVecStdVecVecUuidUuid>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals with 3 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals::try_new(__field0, __field1, __field2) {
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
                    let mut __field1: _serde::__private::Option<StdVecVecStdVecVecUuidUuid> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::i32> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<StdVecVecStdVecVecUuidUuid>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("position"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i32>(&mut __map)?);
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
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("position")?,
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value", "position"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            position: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(first_increment) => {
                *increment = first_increment;
                match increment.checked_add(1) {
                    Some(second_increment) => {
                        *increment = second_increment;
                        Ok(format!("{}({}->${} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, first_increment, second_increment,))
                    }
                    None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.position);
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<StdVecVecStdVecVecUuidUuid>,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArrayTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: StdVecVecStdVecVecUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<StdVecVecStdVecVecUuidUuid>) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArrayTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArrayTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArrayTryNewErrorNamed::NotUnique {
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
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<StdVecVecStdVecVecUuidUuid>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray::try_new(__field0, __field1) {
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
                    let mut __field1: _serde::__private::Option<std::vec::Vec<StdVecVecStdVecVecUuidUuid>> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<StdVecVecStdVecVecUuidUuid>>(&mut __map)?);
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
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: vec![crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} @> ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<StdVecVecStdVecVecUuidUuid>,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArrayTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: StdVecVecStdVecVecUuidUuid,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<StdVecVecStdVecVecUuidUuid>) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArrayTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArrayTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArrayTryNewErrorNamed::NotUnique {
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
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray {
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<StdVecVecStdVecVecUuidUuid>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray::try_new(__field0, __field1) {
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
                    let mut __field1: _serde::__private::Option<std::vec::Vec<StdVecVecStdVecVecUuidUuid>> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<StdVecVecStdVecVecUuidUuid>>(&mut __map)?);
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
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: vec![crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()],
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(exists (select 1 from jsonb_array_elements_text({}) as e1 join jsonb_array_elements_text(${}) as e2 on e1.value = e2.value))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: std::vec::Vec<std::vec::Vec<UuidUuid>>,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: vec![vec![
                crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            ]],
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}) as el where (el) <> ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
    position: std::primitive::i32,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpressionTryNewErrorNamed {
    PositionIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        position: std::primitive::i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String, position: std::primitive::i32) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpressionTryNewErrorNamed> {
        if position >= 0 {
            Ok(Self { logical_operator, value, position })
        } else {
            Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpressionTryNewErrorNamed::PositionIsLessThanZero {
                position,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
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
                        "value" => _serde::__private::Ok(__Field::__field1),
                        "position" => _serde::__private::Ok(__Field::__field2),
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
                        b"position" => _serde::__private::Ok(__Field::__field2),
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression with 3 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression::try_new(__field0, __field1, __field2) {
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
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::i32> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("position"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i32>(&mut __map)?);
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
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("position")?,
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value", "position"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
            position: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(first_increment) => {
                *increment = first_increment;
                match increment.checked_add(1) {
                    Some(second_increment) => {
                        *increment = second_increment;
                        Ok(format!("{}({}->>${} ~ ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, first_increment, second_increment,))
                    }
                    None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.position);
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
    position: std::primitive::i32,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpressionTryNewErrorNamed {
    PositionIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        position: std::primitive::i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String, position: std::primitive::i32) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpressionTryNewErrorNamed> {
        if position >= 0 {
            Ok(Self { logical_operator, value, position })
        } else {
            Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpressionTryNewErrorNamed::PositionIsLessThanZero {
                position,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
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
                        "value" => _serde::__private::Ok(__Field::__field1),
                        "position" => _serde::__private::Ok(__Field::__field2),
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
                        b"position" => _serde::__private::Ok(__Field::__field2),
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression with 3 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression::try_new(__field0, __field1, __field2) {
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
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::primitive::i32> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("position"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::i32>(&mut __map)?);
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
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("position")?,
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value", "position"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
            position: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(first_increment) => {
                *increment = first_increment;
                match increment.checked_add(1) {
                    Some(second_increment) => {
                        *increment = second_increment;
                        Ok(format!("{}({}->>${} ~* ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, first_increment, second_increment,))
                    }
                    None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.position);
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression::try_new(__field0, __field1) {
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
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
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
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) ~ ${}))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
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
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
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
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) ~* ${}))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression::try_new(__field0, __field1) {
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
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
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
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(not exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) !~ ${}))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
    logical_operator: crate::LogicalOperator,
    value: std::string::String,
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
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpressionTryNewErrorNamed {
    IsEmpty { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::string::String) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpressionTryNewErrorNamed> {
        if !value.is_empty() {
            Ok(Self { logical_operator, value })
        } else {
            Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpressionTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() })
        }
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
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
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression with 2 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
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
                    let mut __field1: _serde::__private::Option<std::string::String> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::string::String>(&mut __map)?);
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
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression::try_new(__field0, __field1) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!(
                    "{}(not exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) !~* ${}))",
                    &self.logical_operator.to_query_part(is_need_to_add_logical_operator),
                    column,
                    increment
                ))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.value);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement {
    Equal(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementEqual),
    LengthEqual(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthEqual),
    LengthMoreThan(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementLengthMoreThan),
    PositionEquals(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionEquals),
    ContainsAllElementsOfArray(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsAllElementsOfArray),
    OverlapsWithArray(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementOverlapsWithArray),
    AllElementsEqual(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsEqual),
    PositionCaseSensitiveRegularExpression(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseSensitiveRegularExpression),
    PositionCaseInsensitiveRegularExpression(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementPositionCaseInsensitiveRegularExpression),
    ContainsElementCaseSensitiveRegularExpression(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseSensitiveRegularExpression),
    ContainsElementCaseInsensitiveRegularExpression(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementContainsElementCaseInsensitiveRegularExpression),
    AllElementsCaseSensitiveRegularExpression(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseSensitiveRegularExpression),
    AllElementsCaseInsensitiveRegularExpression(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementAllElementsCaseInsensitiveRegularExpression),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::LengthEqual(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::LengthMoreThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::PositionEquals(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::ContainsAllElementsOfArray(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::OverlapsWithArray(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsEqual(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::PositionCaseSensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::PositionCaseInsensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::ContainsElementCaseSensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::ContainsElementCaseInsensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsCaseSensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::AllElementsCaseInsensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::LengthEqual(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::LengthMoreThan(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::PositionEquals(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::ContainsAllElementsOfArray(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::OverlapsWithArray(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::AllElementsEqual(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::PositionCaseSensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::PositionCaseInsensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::ContainsElementCaseSensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::ContainsElementCaseInsensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::AllElementsCaseSensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::AllElementsCaseInsensitiveRegularExpression(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereElementTraits<'_> for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement {}
impl error_occurence_lib::ToStdStringString for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::LengthEqual(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::LengthMoreThan(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::PositionEquals(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::ContainsAllElementsOfArray(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::OverlapsWithArray(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::AllElementsEqual(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::PositionCaseSensitiveRegularExpression(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::PositionCaseInsensitiveRegularExpression(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::ContainsElementCaseSensitiveRegularExpression(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::ContainsElementCaseInsensitiveRegularExpression(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::AllElementsCaseSensitiveRegularExpression(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::AllElementsCaseInsensitiveRegularExpression(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}
pub type PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidOptionToUpdate = StdVecVecStdVecVecUuidUuid;
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonType for StdVecVecStdVecVecUuidUuid {
    type PostgresqlJsonTypeSelfToCreate<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidToCreate;
    fn try_generate_postgresql_json_type_to_create(
        postgresql_json_type_self_to_create: &Self::PostgresqlJsonTypeSelfToCreate<'_>,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonTypeTryGeneratePostgresqlJsonTypeToCreateErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_postgresql_query_part_to_create<'a>(postgresql_json_type_self_to_create: Self::PostgresqlJsonTypeSelfToCreate<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(postgresql_json_type_self_to_create.0));
        query
    }
    type PostgresqlJsonTypeSelfFieldReader<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidFieldReader;
    type PostgresqlJsonTypeSelfOptionsToRead<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidOptionsToRead;
    fn generate_postgresql_json_type_to_read(
        postgresql_json_type_self_field_reader: &Self::PostgresqlJsonTypeSelfFieldReader<'_>,
        field_ident: &std::primitive::str,
        column_name_and_maybe_field_getter: &std::primitive::str,
        column_name_and_maybe_field_getter_for_error_message: &std::primitive::str,
        is_postgresql_type: std::primitive::bool,
    ) -> std::string::String {
        let start = postgresql_json_type_self_field_reader.pagination.start();
        let end = postgresql_json_type_self_field_reader.pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end})))")
    }
    type PostgresqlJsonTypeSelfWhereElement<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement;
    type PostgresqlJsonTypeSelfWhere = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere;
    type PostgresqlJsonTypeSelfOptionToUpdate<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidOptionToUpdate;
    type PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed;
    fn try_generate_postgresql_json_type_to_update(
        postgresql_json_type_self_option_to_update: &Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        jsonb_set_target: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::PostgresqlJsonTypeSelfOptionToUpdateTryGeneratePostgresqlJsonTypeErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(
        postgresql_json_type_self_option_to_update: Self::PostgresqlJsonTypeSelfOptionToUpdate<'_>,
        mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(postgresql_json_type_self_option_to_update.0));
        query
    }
}
