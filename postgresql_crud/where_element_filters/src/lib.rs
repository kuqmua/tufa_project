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

//difference between NotEmptyUniqueEnumVec and PostgresqlJsonTypeNotEmptyUniqueVec only in postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement impl with different generic requirement and PostgresqlTypeWhereFilter
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

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, schemars::JsonSchema)]
pub struct UnsignedPartOfStdPrimitiveI32(std::primitive::i32); //todo why exactly i32? maybe different types for postgresql type and postgresql json type
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence, schemars::JsonSchema)]
pub enum UnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed {
    UnsignedPartOfStdPrimitiveI32IsLessThanZero {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<std::primitive::i32> for UnsignedPartOfStdPrimitiveI32 {
    type Error = UnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed;
    fn try_from(value: std::primitive::i32) -> Result<Self, Self::Error> {
        if value >= 0 { Ok(Self(value)) } else { Err(Self::Error::UnsignedPartOfStdPrimitiveI32IsLessThanZero { value, code_occurence: error_occurence_lib::code_occurence!() }) }
    }
}
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for UnsignedPartOfStdPrimitiveI32 {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<UnsignedPartOfStdPrimitiveI32>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = UnsignedPartOfStdPrimitiveI32;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct UnsignedPartOfStdPrimitiveI32")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::primitive::i32 = <std::primitive::i32 as _serde::Deserialize>::deserialize(__e)?;
                    match UnsignedPartOfStdPrimitiveI32::try_from(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct UnsignedPartOfStdPrimitiveI32 with 1 element"));
                        }
                    };
                    match UnsignedPartOfStdPrimitiveI32::try_from(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "UnsignedPartOfStdPrimitiveI32",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Self>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl error_occurence_lib::ToStdStringString for UnsignedPartOfStdPrimitiveI32 {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl sqlx::Type<sqlx::Postgres> for UnsignedPartOfStdPrimitiveI32 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UnsignedPartOfStdPrimitiveI32 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl UnsignedPartOfStdPrimitiveI32 {
    pub const fn get(&self) -> std::primitive::i32 {
        self.0
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UnsignedPartOfStdPrimitiveI32 {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(0)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, schemars::JsonSchema)]
pub struct NotZeroUnsignedPartOfStdPrimitiveI32(std::primitive::i32); //todo why exactly i32? maybe different types for postgresql type and postgresql json type
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence, schemars::JsonSchema)]
pub enum NotZeroUnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed {
    NotZeroUnsignedPartOfStdPrimitiveI32IsLessThanOne {
        #[eo_to_std_string_string_serialize_deserialize]
        value: std::primitive::i32,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<std::primitive::i32> for NotZeroUnsignedPartOfStdPrimitiveI32 {
    type Error = NotZeroUnsignedPartOfStdPrimitiveI32TryFromStdPrimitiveI32ErrorNamed;
    fn try_from(value: std::primitive::i32) -> Result<Self, Self::Error> {
        if value >= 1 { Ok(Self(value)) } else { Err(Self::Error::NotZeroUnsignedPartOfStdPrimitiveI32IsLessThanOne { value, code_occurence: error_occurence_lib::code_occurence!() }) }
    }
}
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for NotZeroUnsignedPartOfStdPrimitiveI32 {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<NotZeroUnsignedPartOfStdPrimitiveI32>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = NotZeroUnsignedPartOfStdPrimitiveI32;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct NotZeroUnsignedPartOfStdPrimitiveI32")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::primitive::i32 = <std::primitive::i32 as _serde::Deserialize>::deserialize(__e)?;
                    match NotZeroUnsignedPartOfStdPrimitiveI32::try_from(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::primitive::i32>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct NotZeroUnsignedPartOfStdPrimitiveI32 with 1 element"));
                        }
                    };
                    match NotZeroUnsignedPartOfStdPrimitiveI32::try_from(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "NotZeroUnsignedPartOfStdPrimitiveI32",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Self>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl error_occurence_lib::ToStdStringString for NotZeroUnsignedPartOfStdPrimitiveI32 {
    fn to_std_string_string(&self) -> std::string::String {
        self.0.to_string()
    }
}
impl sqlx::Type<sqlx::Postgres> for NotZeroUnsignedPartOfStdPrimitiveI32 {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <std::primitive::i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <std::primitive::i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for NotZeroUnsignedPartOfStdPrimitiveI32 {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&self.0, buf)
    }
}
impl NotZeroUnsignedPartOfStdPrimitiveI32 {
    pub const fn get(&self) -> std::primitive::i32 {
        self.0
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for NotZeroUnsignedPartOfStdPrimitiveI32 {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, schemars::JsonSchema)]
pub struct Between<T>
where
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
{
    start: T,
    end: T,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum BetweenTryNewErrorNamed<T> {
    StartMoreOrEqualToEnd {
        #[eo_to_std_string_string_serialize_deserialize]
        start: T,
        #[eo_to_std_string_string_serialize_deserialize]
        end: T,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::cmp::PartialOrd> Between<T> {
    fn try_new(start: T, end: T) -> Result<Self, BetweenTryNewErrorNamed<T>> {
        if start < end { Ok(Self { start, end }) } else { Err(BetweenTryNewErrorNamed::StartMoreOrEqualToEnd { start, end, code_occurence: error_occurence_lib::code_occurence!() }) }
    }
}
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T> _serde::Deserialize<'de> for Between<T>
    where
        T: std::fmt::Debug + _serde::Deserialize<'de> + std::cmp::PartialOrd + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
    {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[expect(non_camel_case_types)]
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
                        1u64 => _serde::__private::Ok(__Field::__field0),
                        2u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => _serde::__private::Ok(__Field::__field0),
                        "end" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"start" => _serde::__private::Ok(__Field::__field0),
                        b"end" => _serde::__private::Ok(__Field::__field1),
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
                T: _serde::Deserialize<'de> + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
            {
                marker: _serde::__private::PhantomData<Between<T>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: std::fmt::Debug + _serde::Deserialize<'de> + std::cmp::PartialOrd + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
            {
                type Value = Between<T>;
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "struct Between")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<T>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct Between with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<T>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct Between with 2 elements"));
                        }
                    };
                    match Between::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<T> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<T> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("start"));
                                }
                                __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("end"));
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
                        _serde::__private::None => _serde::__private::de::missing_field("start")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("end")?,
                    };
                    match Between::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&str] = &["start", "end"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Between",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Self>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for Between<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            start: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            end: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::marker::Send + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for Between<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, _: &dyn std::fmt::Display, _: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let start_increment = match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                value
            }
            None => {
                return Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        };
        let end_increment = match increment.checked_add(1) {
            Some(value) => {
                *increment = value;
                value
            }
            None => {
                return Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
            }
        };
        Ok(format!("between ${start_increment} and ${end_increment}"))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = query.bind(self.start);
        query = query.bind(self.end);
        query
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct PostgresqlTypeNotEmptyUniqueVec<T>(std::vec::Vec<T>);
impl<T: std::cmp::PartialEq + Clone> PostgresqlTypeNotEmptyUniqueVec<T> {
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
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + _serde::Deserialize<'de>> _serde::Deserialize<'de> for PostgresqlTypeNotEmptyUniqueVec<T> {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<PostgresqlTypeNotEmptyUniqueVec<T>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + _serde::Deserialize<'de>> _serde::de::Visitor<'de> for __Visitor<'de, T> {
                type Value = PostgresqlTypeNotEmptyUniqueVec<T>;
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "tuple struct PostgresqlTypeNotEmptyUniqueVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::vec::Vec<T> = <std::vec::Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    _serde::__private::Ok(PostgresqlTypeNotEmptyUniqueVec(__field0))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<T>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlTypeNotEmptyUniqueVec with 1 element"));
                        }
                    };
                    match PostgresqlTypeNotEmptyUniqueVec::try_new(__field0) {
                        Ok(value) => _serde::__private::Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "PostgresqlTypeNotEmptyUniqueVec",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Self>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeNotEmptyUniqueVec<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl<T> std::default::Default for PostgresqlTypeNotEmptyUniqueVec<T> {
    fn default() -> Self {
        Self(std::vec::Vec::default())
    }
}
impl<T> std::convert::From<PostgresqlTypeNotEmptyUniqueVec<T>> for Vec<T> {
    fn from(value: PostgresqlTypeNotEmptyUniqueVec<T>) -> Self {
        value.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, schemars::JsonSchema)]
pub struct BoundedStdVecVec<T, const LENGTH: std::primitive::usize>(std::vec::Vec<T>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence, schemars::JsonSchema)]
pub enum BoundedStdVecVecTryNewErrorNamed {
    LengthIsNotCorrect {
        #[eo_to_std_string_string_serialize_deserialize]
        wrong_length: std::primitive::usize,
        #[eo_to_std_string_string_serialize_deserialize]
        expected: std::primitive::usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
enum PostgresqlTypeOrPostgresqlJsonType {
    PostgresqlType,
    PostgresqlJsonType,
}
enum Variant {
    Normal,
    MinusOne,
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'a, const LENGTH: std::primitive::usize> BoundedStdVecVec<T, LENGTH> {
    pub const fn to_inner(&self) -> &std::vec::Vec<T> {
        &self.0
    }
    pub fn into_inner(self) -> std::vec::Vec<T> {
        self.0
    }
    fn query_part(&self, increment: &mut std::primitive::u64, _: &dyn std::fmt::Display, _is_need_to_add_logical_operator: std::primitive::bool, postgresql_type_or_postgresql_json_type: PostgresqlTypeOrPostgresqlJsonType, variant: &Variant) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let mut acc = std::string::String::new();
        let current_len = match &variant {
            Variant::Normal => self.0.len(),
            Variant::MinusOne => self.0.len().saturating_sub(1),
        };
        for _ in 0..current_len {
            match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    acc.push_str(&match &postgresql_type_or_postgresql_json_type {
                        PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => format!("[${value}]"),
                        PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => {
                            format!("->${value}")
                        }
                    });
                }
                None => {
                    return Err(postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                }
            }
        }
        Ok(acc)
    }
    pub fn postgresql_type_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, _is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        self.query_part(increment, column, _is_need_to_add_logical_operator, PostgresqlTypeOrPostgresqlJsonType::PostgresqlType, &Variant::Normal)
    }
    pub fn postgresql_json_type_query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, _is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        self.query_part(increment, column, _is_need_to_add_logical_operator, PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType, &Variant::Normal)
    }
    pub fn postgresql_type_query_part_minus_one(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, _is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        self.query_part(increment, column, _is_need_to_add_logical_operator, PostgresqlTypeOrPostgresqlJsonType::PostgresqlType, &Variant::MinusOne)
    }
    pub fn postgresql_json_type_query_part_minus_one(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, _is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        self.query_part(increment, column, _is_need_to_add_logical_operator, PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType, &Variant::MinusOne)
    }
    pub fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            query = query.bind(element);
        }
        query
    }
}
impl<T, const LENGTH: std::primitive::usize> std::convert::TryFrom<std::vec::Vec<T>> for BoundedStdVecVec<T, LENGTH> {
    type Error = BoundedStdVecVecTryNewErrorNamed;
    fn try_from(value: std::vec::Vec<T>) -> Result<Self, Self::Error> {
        let len = value.len();
        if len == LENGTH {
            Ok(Self(value))
        } else {
            Err(BoundedStdVecVecTryNewErrorNamed::LengthIsNotCorrect {
                wrong_length: len,
                expected: LENGTH,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T, const LENGTH: std::primitive::usize> _serde::Deserialize<'de> for BoundedStdVecVec<T, LENGTH>
    where
        T: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T, const LENGTH: std::primitive::usize>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<BoundedStdVecVec<T, LENGTH>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T, const LENGTH: std::primitive::usize> _serde::de::Visitor<'de> for __Visitor<'de, T, LENGTH>
            where
                T: _serde::Deserialize<'de>,
            {
                type Value = BoundedStdVecVec<T, LENGTH>;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct BoundedStdVecVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: std::vec::Vec<T> = <std::vec::Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    match BoundedStdVecVec::try_from(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<std::vec::Vec<T>>(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"tuple struct BoundedStdVecVec with 1 element"));
                        }
                    };
                    match BoundedStdVecVec::try_from(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "BoundedStdVecVec",
                __Visitor {
                    marker: _serde::__private::PhantomData::<Self>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl<T: Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement, const LENGTH: std::primitive::usize> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for BoundedStdVecVec<T, LENGTH> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![<T as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(); LENGTH])
    }
}
