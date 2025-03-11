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
    //todo could not implement multi dimension pagination
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
    //here
    dimension_1: std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>>,
    dimension_2: std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecUuidUuidWhereElementSecondDimension>>,
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
    NoDimensionFilters {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Dimension1IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Dimension1NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        element: PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Dimension2IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Dimension2NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        element: PostgresqlJsonTypeStdVecVecUuidUuidWhereElementSecondDimension,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere {
    fn try_new(
        logical_operator: crate::LogicalOperator,
        dimension_1: std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>>,
        dimension_2: std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecUuidUuidWhereElementSecondDimension>>,
    ) -> Result<Self, PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed> {
        if let (None, None) = (&dimension_1, &dimension_2) {
            return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed::NoDimensionFilters { code_occurence: error_occurence_lib::code_occurence!() });
        }
        //todo change logic
        if let Some(value) = &dimension_1 {
            if value.is_empty() {
                return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed::Dimension1IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
            }
            {
                let mut acc = vec![];
                for element in value {
                    if !acc.contains(&element) {
                        acc.push(element);
                    } else {
                        return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed::Dimension1NotUnique {
                            element: element.clone(),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
        }
        if let Some(value) = &dimension_2 {
            if value.is_empty() {
                return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed::Dimension2IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
            }
            {
                let mut acc = vec![];
                for element in value {
                    if !acc.contains(&element) {
                        acc.push(element);
                    } else {
                        return Err(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereTryNewErrorNamed::Dimension2NotUnique {
                            element: element.clone(),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
            }
        }
        Ok(Self { logical_operator, dimension_1, dimension_2 })
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
                __field2,
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
                        "dimension_1" => _serde::__private::Ok(__Field::__field1),
                        "dimension_2" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"logical_operator" => _serde::__private::Ok(__Field::__field0),
                        b"dimension_1" => _serde::__private::Ok(__Field::__field1),
                        b"dimension_2" => _serde::__private::Ok(__Field::__field2),
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
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecUuidUuidWhereElementSecondDimension>>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere with 3 elements"));
                        }
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere::try_new(__field0, __field1, __field2) {
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
                    let mut __field1: _serde::__private::Option<std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>>> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecUuidUuidWhereElementSecondDimension>>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("logical_operator"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<crate::LogicalOperator>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("dimension_1"));
                                }
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement>>>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("dimension_2"));
                                }
                                __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::option::Option<std::vec::Vec<PostgresqlJsonTypeStdVecVecUuidUuidWhereElementSecondDimension>>>(&mut __map)?);
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
                        _serde::__private::None => _serde::__private::de::missing_field("dimension_1")?,
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => _serde::__private::de::missing_field("dimension_2")?,
                    };
                    match PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere::try_new(__field0, __field1, __field2) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "dimension_1", "dimension_2"];
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
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        let mut acc = std::string::String::default();
        let mut is_need_to_add_logical_operator_inner_handle = false;
        if let Some(value) = &self.dimension_1 {
            for element in value {
                match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                    Ok(value) => {
                        acc.push_str(&format!("{value} "));
                        is_need_to_add_logical_operator_inner_handle = true;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
        }
        if let Some(value) = &self.dimension_2 {
            for element in value {
                match crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(element, increment, column, is_need_to_add_logical_operator_inner_handle) {
                    Ok(value) => {
                        acc.push_str(&format!("{value} "));
                        is_need_to_add_logical_operator_inner_handle = true;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
        }
        let _ = acc.pop();
        Ok(format!("{}({acc})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator)))
    }
    fn where_query_bind<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if let Some(value) = self.dimension_1 {
            for element in value {
                query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(element, query);
            }
        }
        if let Some(value) = self.dimension_2 {
            for element in value {
                query = crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(element, query);
            }
        }
        query
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            dimension_1: Some(crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            dimension_2: Some(crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
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
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment))
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn where_query_bind<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.value));
        query
    }
}

#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub enum PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement {
    Equal(PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElementEqual),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement {
    fn where_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn where_query_bind<'a>(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::where_query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Equal(
            crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        )]
    }
}
pub type PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidOptionToUpdate = StdVecVecStdVecVecUuidUuid;
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdVecVecStdVecVecUuidUuidOptionToUpdateTryGenerateErrorNamed {
    CheckedAdd { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl crate::postgresql_json_type::postgresql_json_type_trait::PostgresqlJsonType for StdVecVecStdVecVecUuidUuid {
    type Create<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidToCreate;
    fn create_query_part(_: &Self::Create<'_>, increment: &mut std::primitive::u64) -> Result<std::string::String, crate::postgresql_json_type::postgresql_json_type_trait::TryGenerateToCreateErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${increment}"))
            }
            None => Err(crate::postgresql_json_type::postgresql_json_type_trait::TryGenerateToCreateErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn create_query_bind<'a>(value: Self::Create<'a>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(value.0));
        query
    }
    type FieldReader<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidFieldReader;
    type OptionsToRead<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidOptionsToRead;
    fn field_reader_query_part(value: &Self::FieldReader<'_>, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, _: &std::primitive::str, _: std::primitive::bool) -> std::string::String {
        // println!("fn field_reader_query_part");
        //todo change
        let start = value.pagination.start();
        let end = value.pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg(value) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {start} and {end})))")
    }
    type SelfWhereElement<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhereElement;
    type SelfWhere = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidWhere;
    type SelfOptionToUpdate<'a> = PostgresqlJsonTypeStdVecVecStdVecVecUuidUuidOptionToUpdate;
    type SelfOptionToUpdateTryGenerateErrorNamed = StdVecVecStdVecVecUuidUuidOptionToUpdateTryGenerateErrorNamed;
    fn try_generate_postgresql_json_type_to_update(
        _: &Self::SelfOptionToUpdate<'_>,
        jsonb_set_accumulator: &std::primitive::str,
        _: &std::primitive::str,
        jsonb_set_path: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::SelfOptionToUpdateTryGenerateErrorNamed> {
        println!("fn try_generate_postgresql_json_type_to_update");
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
            }
            None => Err(Self::SelfOptionToUpdateTryGenerateErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn bind_value_to_postgresql_query_part_to_update<'a>(postgresql_json_type_self_option_to_update: Self::SelfOptionToUpdate<'_>, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(postgresql_json_type_self_option_to_update.0));
        query
    }
}
