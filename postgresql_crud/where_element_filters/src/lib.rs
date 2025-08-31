generate_where_element_filters::generate_where_element_filters!();

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum EncodeFormat {
    Base64,
    Hex,
    Escape,
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
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for EncodeFormat {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self::default()
    }
}

//difference between NotEmptyUniqueEnumVec and PostgresqlJsonTypeNotEmptyUniqueVec only in crate::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement impl with different generic requirement and PostgresqlTypeWhereFilter
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct PostgresqlJsonTypeNotEmptyUniqueVec<T>(std::vec::Vec<T>);
impl<T: std::cmp::PartialEq + Clone> PostgresqlJsonTypeNotEmptyUniqueVec<T> {
    pub fn try_new(value: std::vec::Vec<T>) -> Result<Self, postgresql_crud_common::NotEmptyUniqueVecTryNewErrorNamed<T>> {
        if value.is_empty() {
            return Err(postgresql_crud_common::NotEmptyUniqueVecTryNewErrorNamed::IsEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let mut acc = vec![];
            for element in &value {
                if acc.contains(&element) {
                    return Err(postgresql_crud_common::NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                        value: element.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
                acc.push(element);
            }
        }
        Ok(Self(value))
    }
    pub const fn to_vec(&self) -> &std::vec::Vec<T> {
        &self.0
    }
    pub fn into_vec(self) -> std::vec::Vec<T> {
        self.0
    }
}
impl<T: std::cmp::PartialEq + Clone + serde::Serialize> PostgresqlJsonTypeNotEmptyUniqueVec<T> {
    pub fn query_part_one_by_one(&self, increment: &mut std::primitive::u64, _: &dyn std::fmt::Display, _is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        for _ in self.to_vec() {
            match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    acc.push_str(&format!("${value},"));
                }
                None => {
                    return Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
        }
        let _ = acc.pop();
        Ok(acc)
    }
    pub fn query_bind_one_by_one<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>
    where
        T: 'a,
    {
        for element in self.0 {
            query = query.bind(sqlx::types::Json(element));
        }
        query
    }
}
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + _serde::Deserialize<'de>> _serde::Deserialize<'de> for PostgresqlJsonTypeNotEmptyUniqueVec<T> {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<PostgresqlJsonTypeNotEmptyUniqueVec<T>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + _serde::Deserialize<'de>> _serde::de::Visitor<'de> for __Visitor<'de, T> {
                type Value = PostgresqlJsonTypeNotEmptyUniqueVec<T>;
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "tuple struct PostgresqlJsonTypeNotEmptyUniqueVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::vec::Vec<T> = <std::vec::Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    _serde::__private::Ok(PostgresqlJsonTypeNotEmptyUniqueVec(__field0))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<T>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlJsonTypeNotEmptyUniqueVec with 1 element"));
                        }
                    };
                    match PostgresqlJsonTypeNotEmptyUniqueVec::try_new(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "PostgresqlJsonTypeNotEmptyUniqueVec",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Self>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeNotEmptyUniqueVec<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl<T> std::default::Default for PostgresqlJsonTypeNotEmptyUniqueVec<T> {
    fn default() -> Self {
        Self(std::vec::Vec::default())
    }
}
impl<T> std::convert::From<PostgresqlJsonTypeNotEmptyUniqueVec<T>> for Vec<T> {
    fn from(value: PostgresqlJsonTypeNotEmptyUniqueVec<T>) -> Self {
        value.0
    }
}
impl<'a, T> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeNotEmptyUniqueVec<T>
where
    T: serde::Serialize + 'a,
{
    fn query_part(&self, increment: &mut std::primitive::u64, _: &dyn std::fmt::Display, _is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                Ok(format!("${value}"))
            }
            None => Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(sqlx::types::Json(self.0));
        query
    }
}

#[derive(Debug, Clone)]
pub struct RegexRegex(regex::Regex); //regex::Regex
// #[automatically_derived]
// impl ::core::marker::StructuralPartialEq for RegexRegex {}
// #[automatically_derived]
impl ::core::cmp::PartialEq for RegexRegex {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}
#[doc(hidden)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for RegexRegex {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "RegexRegex", &self.0.to_string())
        }
    }
};
#[doc(hidden)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for RegexRegex {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<RegexRegex>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = RegexRegex;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct RegexRegex")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::string::String = <std::string::String as _serde::Deserialize>::deserialize(__e)?;
                    _serde::__private::Ok(RegexRegex(match regex::Regex::new(&__field0) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::string::String>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct RegexRegex with 1 element"));
                        }
                    };
                    _serde::__private::Ok(RegexRegex(match regex::Regex::new(&__field0) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "RegexRegex",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Self>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    impl schemars::JsonSchema for RegexRegex {
        fn schema_name() -> std::string::String {
            "RegexRegex".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("testing::RegexRegex")
        }
        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
            generator.subschema_for::<std::string::String>()
        }
    }
};
impl std::string::ToString for RegexRegex {
    fn to_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for RegexRegex {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(regex::Regex::new("").unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum RegularExpressionCase {
    Sensitive,
    Insensitive,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for RegularExpressionCase {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self::Sensitive
    }
}
impl RegularExpressionCase {
    pub const fn postgreql_syntax(&self) -> &'static std::primitive::str {
        match &self {
            Self::Sensitive => "~",
            Self::Insensitive => "~*",
        }
    }
}