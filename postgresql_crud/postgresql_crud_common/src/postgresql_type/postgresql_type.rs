postgresql_crud_types_macro_logic_reuse::generate_postgresql_types!();

#[derive(Debug, Clone, PartialEq)]
pub struct SqlxTypesBitVecAsPostgresqlBitNotNull(pub sqlx::types::BitVec);
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SqlxTypesBitVecAsPostgresqlBitNotNull {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "SqlxTypesBitVecAsPostgresqlBitNotNull", &self.0.iter().collect::<std::vec::Vec<std::primitive::bool>>())
        }
    }
};
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SqlxTypesBitVecAsPostgresqlBitNotNull {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: serde::__private::PhantomData<SqlxTypesBitVecAsPostgresqlBitNotNull>,
                lifetime: serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesBitVecAsPostgresqlBitNotNull;
                fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                    serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesBitVecAsPostgresqlBit")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: serde::Deserializer<'de>,
                {
                    let __field0 = <std::vec::Vec<std::primitive::bool> as serde::Deserialize>::deserialize(__e)?;
                    serde::__private::Ok(SqlxTypesBitVecAsPostgresqlBitNotNull({
                        let mut bit_vec = sqlx::types::BitVec::from_elem(__field0.len(), false);
                        __field0.into_iter().enumerate().for_each(|(index, element)| {
                            bit_vec.set(index, element);
                        });
                        bit_vec
                    }))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<std::primitive::bool>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct SqlxTypesBitVecAsPostgresqlBit with 1 elements"));
                        }
                    };
                    serde::__private::Ok(SqlxTypesBitVecAsPostgresqlBitNotNull({
                        let mut bit_vec = sqlx::types::BitVec::from_elem(__field0.len(), false);
                        __field0.into_iter().enumerate().for_each(|(index, element)| {
                            bit_vec.set(index, element);
                        });
                        bit_vec
                    }))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "SqlxTypesBitVecAsPostgresqlBitNotNull",
                __Visitor {
                    marker: serde::__private::PhantomData::<SqlxTypesBitVecAsPostgresqlBitNotNull>,
                    lifetime: serde::__private::PhantomData,
                },
            )
        }
    }
};
impl std::fmt::Display for SqlxTypesBitVecAsPostgresqlBitNotNull {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl error_occurence_lib::ToStdStringString for SqlxTypesBitVecAsPostgresqlBitNotNull {
    fn to_std_string_string(&self) -> std::string::String {
        self.to_string()
    }
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesBitVecAsPostgresqlBitNotNull {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self({
            let mut value = sqlx::types::BitVec::new();
            value.push(false);
            value
        })
    }
}
impl crate::BindQuery<'_> for SqlxTypesBitVecAsPostgresqlBitNotNull {
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
impl sqlx::Type<sqlx::Postgres> for SqlxTypesBitVecAsPostgresqlBitNotNull {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::BitVec as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::BitVec as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for SqlxTypesBitVecAsPostgresqlBitNotNull {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::BitVec as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}
impl SqlxTypesBitVecAsPostgresqlBitNotNull {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} bit(1) not null")
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
pub struct SqlxTypesBitVecAsPostgresqlBitNotNullColumn;
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesBitVecAsPostgresqlBitNotNullColumn {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        ::core::default::Default::default()
    }
}
pub type SqlxTypesBitVecAsPostgresqlBitNotNullToCreate = SqlxTypesBitVecAsPostgresqlBitNotNull;
pub type SqlxTypesBitVecAsPostgresqlBitNotNullToRead = SqlxTypesBitVecAsPostgresqlBitNotNull;
pub type SqlxTypesBitVecAsPostgresqlBitNotNullToUpdate = SqlxTypesBitVecAsPostgresqlBitNotNull;
pub type SqlxTypesBitVecAsPostgresqlBitNotNullToDelete = SqlxTypesBitVecAsPostgresqlBitNotNull;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementEqual {
    pub logical_operator: crate::LogicalOperator,
    pub value: SqlxTypesBitVecAsPostgresqlBitNotNull,
}
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementEqual {
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
        query = query.bind(self.value.0);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual {
    logical_operator: crate::LogicalOperator,
    value: std::primitive::bool,
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
pub enum PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqualTryNewErrorNamed {
    PositionIsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        position: std::primitive::i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::primitive::bool, position: std::primitive::i32) -> Result<Self, PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqualTryNewErrorNamed> {
        if position >= 0 {
            Ok(Self { logical_operator, value, position })
        } else {
            Err(PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqualTryNewErrorNamed::PositionIsLessThanZero {
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
    impl<'de> _serde::Deserialize<'de> for PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual {
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
                marker: _serde::__private::PhantomData<PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual with 3 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::primitive::bool>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual with 3 elements"));
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual with 3 elements"));
                        }
                    };
                    match PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual::try_new(__field0, __field1, __field2) {
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
                    let mut __field1: _serde::__private::Option<std::primitive::bool> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::primitive::bool>(&mut __map)?);
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
                    match PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual::try_new(__field0, __field1, __field2) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value", "position"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
            position: ::core::default::Default::default(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match increment.checked_add(1) {
            Some(first_increment) => {
                *increment = first_increment;
                match increment.checked_add(1) {
                    Some(second_increment) => {
                        *increment = second_increment;
                        Ok(format!("{}(get_bit({}, ${}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, first_increment, second_increment,))
                    }
                    None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
                }
            }
            None => Err(crate::TryGenerateBindIncrementsErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.position);
        query = query.bind(if self.value { 1 } else { 0 });
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement {
    Equal(PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementEqual),
    BitVecPositionEqual(PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElementBitVecPositionEqual),
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement {
    fn postgresql_type_self_where_try_generate_bind_increments(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, crate::TryGenerateBindIncrementsErrorNamed> {
        match &self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
            Self::BitVecPositionEqual(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_try_generate_bind_increments(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn postgresql_type_self_where_bind_value_to_query<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
            Self::BitVecPositionEqual(value) => crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter::postgresql_type_self_where_bind_value_to_query(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Equal(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
            Self::BitVecPositionEqual(crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize)]
pub struct SqlxTypesBitVecAsPostgresqlBitNotNullWhere {
    logical_operator: crate::LogicalOperator,
    value: std::vec::Vec<PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement>,
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
pub enum PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereTryNewErrorNamed {
    IsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUnique {
        #[eo_to_std_string_string_serialize_deserialize]
        value: PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl SqlxTypesBitVecAsPostgresqlBitNotNullWhere {
    fn try_new(logical_operator: crate::LogicalOperator, value: std::vec::Vec<PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement>) -> Result<Self, PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereTryNewErrorNamed> {
        if value.is_empty() {
            return Err(PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if !acc.contains(&element) {
                    acc.push(element);
                } else {
                    return Err(PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereTryNewErrorNamed::NotUnique {
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
    impl<'de> _serde::Deserialize<'de> for SqlxTypesBitVecAsPostgresqlBitNotNullWhere {
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
                marker: _serde::__private::PhantomData<SqlxTypesBitVecAsPostgresqlBitNotNullWhere>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SqlxTypesBitVecAsPostgresqlBitNotNullWhere;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct SqlxTypesBitVecAsPostgresqlBitNotNullWhere")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<crate::LogicalOperator>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct SqlxTypesBitVecAsPostgresqlBitNotNullWhere with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct SqlxTypesBitVecAsPostgresqlBitNotNullWhere with 2 elements"));
                        }
                    };
                    match SqlxTypesBitVecAsPostgresqlBitNotNullWhere::try_new(__field0, __field1) {
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
                    let mut __field1: _serde::__private::Option<std::vec::Vec<PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement>> = _serde::__private::None;
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
                                __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<std::vec::Vec<PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement>>(&mut __map)?);
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
                    match SqlxTypesBitVecAsPostgresqlBitNotNullWhere::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["logical_operator", "value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SqlxTypesBitVecAsPostgresqlBitNotNullWhere",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SqlxTypesBitVecAsPostgresqlBitNotNullWhere>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl crate::postgresql_type::postgresql_type_trait::PostgresqlTypeSelfWhereFilter for SqlxTypesBitVecAsPostgresqlBitNotNullWhere {
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
impl crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement for SqlxTypesBitVecAsPostgresqlBitNotNullWhere {
    fn std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: crate::generate_postgresql_json_type::StdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::std_default_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            value: crate::generate_postgresql_json_type::AllEnumVariantsArrayStdDefaultDefaultButStdOptionOptionIsAlwaysSomeAndStdVecVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
impl crate::postgresql_type::postgresql_type_trait::PostgresqlType<'_> for SqlxTypesBitVecAsPostgresqlBitNotNull {
    type PostgresqlTypeSelf = Self;
    type PostgresqlTypeSelfColumn = SqlxTypesBitVecAsPostgresqlBitNotNullColumn;
    fn postgresql_type_self_column_query_part(postgresql_type_self_column: &Self::PostgresqlTypeSelfColumn, column: &std::primitive::str) -> std::string::String {
        column.to_string()
    }
    type PostgresqlTypeSelfToCreate = SqlxTypesBitVecAsPostgresqlBitNotNullToCreate;
    type PostgresqlTypeSelfToRead = SqlxTypesBitVecAsPostgresqlBitNotNullToRead;
    type PostgresqlTypeSelfToUpdate = SqlxTypesBitVecAsPostgresqlBitNotNullToUpdate;
    type PostgresqlTypeSelfToUpdateQueryPartErrorNamed = crate::TryGenerateBindIncrementsErrorNamed;
    fn postgresql_type_self_to_update_query_part(
        postgresql_type_self_to_update: &Self::PostgresqlTypeSelfToUpdate,
        _: &std::primitive::str,
        _: &std::primitive::str,
        _: &std::primitive::str,
        increment: &mut std::primitive::u64,
    ) -> Result<std::string::String, Self::PostgresqlTypeSelfToUpdateQueryPartErrorNamed> {
        crate::BindQuery::try_generate_bind_increments(postgresql_type_self_to_update, increment)
    }
    fn postgresql_type_self_to_update_bind_query_part<'a>(postgresql_type_self_to_update: Self::PostgresqlTypeSelfToUpdate, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        crate::BindQuery::bind_value_to_query(postgresql_type_self_to_update, query)
    }
    type PostgresqlTypeSelfWhereElement = PostgresqlTypeSqlxTypesBitVecAsPostgresqlBitNotNullWhereElement;
    type PostgresqlTypeSelfWhere = SqlxTypesBitVecAsPostgresqlBitNotNullWhere;
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