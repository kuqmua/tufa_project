// generate_where_element_filters::generate_where_element_filters!();

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
            match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
                Ok(value) => {
                    acc.push_str(&format!("${value},"));
                },
                Err(error) => {
                    return Err(error);
                },
            }
        }
        let _ = acc.pop();
        Ok(acc)
    }
    pub fn query_bind_one_by_one<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
        sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
        std::string::String
    >
    where
        T: 'a,
    {
        for element in self.0 {
            if let Err(error) = query.try_bind(sqlx::types::Json(element)) {
                return Err(error.to_string());
            }
        }
        Ok(query)
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
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("${value}")),
            Err(error) => {
                return Err(error);
            },
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
        sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
        std::string::String
    > {
        if let Err(error) = query.try_bind(sqlx::types::Json(self.0)) {
            return Err(error.to_string());
        }
        Ok(query)
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
        let start_increment = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            },
        };
        let end_increment = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            },
        };
        Ok(format!("between ${start_increment} and ${end_increment}"))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
        sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
        std::string::String
    > {
        if let Err(error) = query.try_bind(self.start) {
            return Err(error.to_string());
        }
        if let Err(error) = query.try_bind(self.end) {
            return Err(error.to_string());
        }
        Ok(query)
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
            match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
                Ok(value) => {
                    acc.push_str(&match &postgresql_type_or_postgresql_json_type {
                        PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => format!("[${value}]"),
                        PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => {
                            format!("->${value}")
                        }
                    });
                },
                Err(error) => {
                    return Err(error);
                },
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
    pub fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
        sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
        std::string::String
    > {
        for element in self.0 {
            if let Err(error) = query.try_bind(element) {
                return Err(error.to_string());
            }
        }
        Ok(query)
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

//////////////////////////////////













//here
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementEqual<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneEqual<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}


















////////////////////////////
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementGreaterThan<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneGreaterThan<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementBetween<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::Between<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementBetween<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementBetween<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.value.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneBetween<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: crate::Between<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneBetween<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneBetween<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        match self.value.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementIn<T: std::fmt::Debug + std::cmp::PartialEq + Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::PostgresqlTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementIn<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementIn<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = {
            let mut acc = std::string::String::default();
            for _ in self.value.to_vec() {
                match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
                    Ok(value) => {
                        acc.push_str(&format!("${},", value));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            let _ = acc.pop();
            acc
        };
        Ok(format!("{}({} in ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        for element in self.value.into_vec() {
            if let Err(error) = query.try_bind(element) {
                return Err(error.to_string());
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneIn<T: std::fmt::Debug + std::cmp::PartialEq + Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: crate::PostgresqlTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneIn<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneIn<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = {
            let mut acc = std::string::String::default();
            for _ in self.value.to_vec() {
                match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
                    Ok(value) => {
                        acc.push_str(&format!("${},", value));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            let _ = acc.pop();
            acc
        };
        Ok(format!("{}({}{} in ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        for element in self.value.into_vec() {
            if let Err(error) = query.try_bind(element) {
                return Err(error.to_string());
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} {} ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} {} ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementBefore<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementBefore<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementBefore<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} < ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneBefore<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneBefore<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneBefore<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} < ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementCurrentDate {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementCurrentDate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementCurrentDate {
    fn query_part(&self, _: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        Ok(format!("{}({} = current_date)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column,))
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneCurrentDate {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneCurrentDate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneCurrentDate {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} = current_date)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes,))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementGreaterThanCurrentDate {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementGreaterThanCurrentDate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementGreaterThanCurrentDate {
    fn query_part(&self, _: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        Ok(format!("{}({} > current_date)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column,))
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneGreaterThanCurrentDate {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneGreaterThanCurrentDate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneGreaterThanCurrentDate {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} > current_date)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes,))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementCurrentTimestamp {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementCurrentTimestamp {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementCurrentTimestamp {
    fn query_part(&self, _: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        Ok(format!("{}({} = current_timestamp)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column,))
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneCurrentTimestamp {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneCurrentTimestamp {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneCurrentTimestamp {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} = current_timestamp)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes,))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementGreaterThanCurrentTimestamp {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementGreaterThanCurrentTimestamp {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementGreaterThanCurrentTimestamp {
    fn query_part(&self, _: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        Ok(format!("{}({} > current_timestamp)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column,))
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneGreaterThanCurrentTimestamp {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneGreaterThanCurrentTimestamp {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneGreaterThanCurrentTimestamp {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} > current_timestamp)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes,))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementCurrentTime {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementCurrentTime {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementCurrentTime {
    fn query_part(&self, _: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        Ok(format!("{}({} = current_time)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column,))
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneCurrentTime {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneCurrentTime {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneCurrentTime {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} = current_time)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes,))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementGreaterThanCurrentTime {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementGreaterThanCurrentTime {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementGreaterThanCurrentTime {
    fn query_part(&self, _: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        Ok(format!("{}({} > current_time)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column,))
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneGreaterThanCurrentTime {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneGreaterThanCurrentTime {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneGreaterThanCurrentTime {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} > current_time)", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes,))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneLengthEqual {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::NotZeroUnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneLengthEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneLengthEqual {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("{}(array_length({}, 1) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment)),
            Err(error) => Err(error),
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneLengthMoreThan {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::NotZeroUnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneLengthMoreThan {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneLengthMoreThan {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("{}(array_length({}, 1) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, increment)),
            Err(error) => Err(error),
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementEqualToEncodedStringRepresentation {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub encode_format: crate::EncodeFormat,
    pub encoded_string_representation: std::string::String,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementEqualToEncodedStringRepresentation {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            encode_format: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            encoded_string_representation: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementEqualToEncodedStringRepresentation {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(encode({}, '{}') = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, &self.encode_format, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.encoded_string_representation) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneEqualToEncodedStringRepresentation {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub encode_format: crate::EncodeFormat,
    pub encoded_string_representation: std::string::String,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneEqualToEncodedStringRepresentation {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            encode_format: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            encoded_string_representation: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneEqualToEncodedStringRepresentation {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(encode({}{}, '{}') = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, &self.encode_format, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.encoded_string_representation) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementFindRangesWithinGivenRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementFindRangesWithinGivenRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementFindRangesWithinGivenRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} <@ ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneFindRangesWithinGivenRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneFindRangesWithinGivenRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneFindRangesWithinGivenRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} <@ ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementFindRangesThatFullyContainTheGivenRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementFindRangesThatFullyContainTheGivenRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementFindRangesThatFullyContainTheGivenRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} @> ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneFindRangesThatFullyContainTheGivenRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneFindRangesThatFullyContainTheGivenRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneFindRangesThatFullyContainTheGivenRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} @> ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementStrictlyToLeftOfRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementStrictlyToLeftOfRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementStrictlyToLeftOfRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} &< ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneStrictlyToLeftOfRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneStrictlyToLeftOfRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneStrictlyToLeftOfRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} &< ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementStrictlyToRightOfRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementStrictlyToRightOfRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementStrictlyToRightOfRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} &> ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneStrictlyToRightOfRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneStrictlyToRightOfRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneStrictlyToRightOfRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} &> ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementIncludedLowerBound<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementIncludedLowerBound<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementIncludedLowerBound<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(lower({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneIncludedLowerBound<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneIncludedLowerBound<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneIncludedLowerBound<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(lower({}{}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementExcludedUpperBound<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementExcludedUpperBound<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementExcludedUpperBound<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(upper({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneExcludedUpperBound<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneExcludedUpperBound<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneExcludedUpperBound<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(upper({}{}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementGreaterThanIncludedLowerBound<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementGreaterThanIncludedLowerBound<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementGreaterThanIncludedLowerBound<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(lower({}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneGreaterThanIncludedLowerBound<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneGreaterThanIncludedLowerBound<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneGreaterThanIncludedLowerBound<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(lower({}{}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementGreaterThanExcludedUpperBound<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementGreaterThanExcludedUpperBound<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementGreaterThanExcludedUpperBound<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(upper({}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneGreaterThanExcludedUpperBound<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneGreaterThanExcludedUpperBound<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneGreaterThanExcludedUpperBound<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(upper({}{}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementOverlapWithRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementOverlapWithRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementOverlapWithRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} && ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneOverlapWithRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneOverlapWithRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneOverlapWithRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} && ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementAdjacentWithRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementAdjacentWithRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementAdjacentWithRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} -|- ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneAdjacentWithRange<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneAdjacentWithRange<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneAdjacentWithRange<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} -|- ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementRangeLength {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::NotZeroUnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementRangeLength {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementRangeLength {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(upper({}) - lower({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlTypeWhereElementDimensionOneRangeLength {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::NotZeroUnsignedPartOfStdPrimitiveI32, 1>,
    pub value: crate::NotZeroUnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlTypeWhereElementDimensionOneRangeLength {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlTypeWhereElementDimensionOneRangeLength {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes1 = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let dimensions_indexes2 = match self.dimensions.postgresql_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(upper({}{}) - lower({}{}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes1, column, dimensions_indexes2, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.clone().query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementAllElementsEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementAllElementsEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementAllElementsEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}) as el where (el) <> ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneAllElementsEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneAllElementsEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneAllElementsEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where (el) <> ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoAllElementsEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoAllElementsEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoAllElementsEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where (el) <> ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeAllElementsEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeAllElementsEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeAllElementsEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where (el) <> ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourAllElementsEqual<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourAllElementsEqual<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourAllElementsEqual<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where (el) <> ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementLengthEqual {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementLengthEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementLengthEqual {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneLengthEqual {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneLengthEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneLengthEqual {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}{}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoLengthEqual {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoLengthEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoLengthEqual {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}{}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeLengthEqual {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeLengthEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeLengthEqual {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}{}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourLengthEqual {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourLengthEqual {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourLengthEqual {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}{}) = ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementLengthMoreThan {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementLengthMoreThan {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementLengthMoreThan {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneLengthMoreThan {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneLengthMoreThan {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneLengthMoreThan {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}{}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoLengthMoreThan {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoLengthMoreThan {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoLengthMoreThan {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}{}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeLengthMoreThan {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeLengthMoreThan {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeLengthMoreThan {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}{}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourLengthMoreThan {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: crate::UnsignedPartOfStdPrimitiveI32,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourLengthMoreThan {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: ::core::default::Default::default(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourLengthMoreThan {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(jsonb_array_length({}{}) > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} > ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementContainsElementGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementContainsElementGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementContainsElementGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}) as el where (el) > ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneContainsElementGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneContainsElementGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneContainsElementGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}{}) as el where (el) > ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoContainsElementGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoContainsElementGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoContainsElementGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}{}) as el where (el) > ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeContainsElementGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeContainsElementGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeContainsElementGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}{}) as el where (el) > ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourContainsElementGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourContainsElementGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourContainsElementGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}{}) as el where (el) > ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementAllElementsGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementAllElementsGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementAllElementsGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}) as el where (el) <= ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneAllElementsGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneAllElementsGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneAllElementsGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where (el) <= ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoAllElementsGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoAllElementsGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoAllElementsGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where (el) <= ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeAllElementsGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeAllElementsGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeAllElementsGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where (el) <= ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourAllElementsGreaterThan<T> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: T,
}
impl<T: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourAllElementsGreaterThan<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourAllElementsGreaterThan<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where (el) <= ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementBetween<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::Between<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementBetween<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementBetween<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneBetween<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: crate::Between<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneBetween<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneBetween<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoBetween<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: crate::Between<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoBetween<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoBetween<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeBetween<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: crate::Between<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeBetween<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeBetween<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourBetween<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: crate::Between<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourBetween<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + PartialOrd + std::clone::Clone + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourBetween<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementIn<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementIn<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementIn<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match self.value.query_part_one_by_one(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} in ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.value.query_bind_one_by_one(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneIn<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneIn<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneIn<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part_one_by_one(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} in ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        match self.value.query_bind_one_by_one(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoIn<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoIn<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoIn<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part_one_by_one(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} in ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        match self.value.query_bind_one_by_one(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeIn<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeIn<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeIn<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part_one_by_one(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} in ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        match self.value.query_bind_one_by_one(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourIn<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourIn<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourIn<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part_one_by_one(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} in ({}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        match self.value.query_bind_one_by_one(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(trim(both '\"' from ({})::text) {} ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part_minus_one(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let last_dimensions_index = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(trim(both '\"' from ({}{}->>${})::text) {} ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, last_dimensions_index, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part_minus_one(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let last_dimensions_index = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(trim(both '\"' from ({}{}->>${})::text) {} ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, last_dimensions_index, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part_minus_one(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let last_dimensions_index = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(trim(both '\"' from ({}{}->>${})::text) {} ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, last_dimensions_index, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part_minus_one(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let last_dimensions_index = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(trim(both '\"' from ({}{}->>${})::text) {} ${})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, last_dimensions_index, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementContainsElementRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementContainsElementRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementContainsElementRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) {} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneContainsElementRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneContainsElementRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneContainsElementRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) {} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoContainsElementRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoContainsElementRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoContainsElementRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) {} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeContainsElementRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeContainsElementRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeContainsElementRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) {} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourContainsElementRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourContainsElementRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourContainsElementRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) {} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementAllElementsRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementAllElementsRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementAllElementsRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}) as el where substring(el::text from 2 for length(el::text) - 2) !{} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneAllElementsRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneAllElementsRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneAllElementsRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoAllElementsRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoAllElementsRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoAllElementsRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeAllElementsRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeAllElementsRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeAllElementsRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourAllElementsRegularExpression {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub regular_expression_case: crate::RegularExpressionCase,
    pub value: crate::RegexRegex,
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourAllElementsRegularExpression {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            regular_expression_case: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourAllElementsRegularExpression {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(not exists(select 1 from jsonb_array_elements({}{}) as el where substring(el::text from 2 for length(el::text) - 2) !{} ${}))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, self.regular_expression_case.postgreql_syntax(), value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(self.value.to_string()) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementContainsAllElementsOfArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementContainsAllElementsOfArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementContainsAllElementsOfArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({} @> {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneContainsAllElementsOfArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} @> {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoContainsAllElementsOfArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoContainsAllElementsOfArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoContainsAllElementsOfArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} @> {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeContainsAllElementsOfArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeContainsAllElementsOfArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeContainsAllElementsOfArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} @> {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourContainsAllElementsOfArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourContainsAllElementsOfArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourContainsAllElementsOfArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}({}{} @> {})", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementOverlapsWithArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementOverlapsWithArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementOverlapsWithArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists (select 1 from jsonb_array_elements_text({}) as e1 join jsonb_array_elements_text({}) as e2 on e1.value = e2.value))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionOneOverlapsWithArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 1>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionOneOverlapsWithArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionOneOverlapsWithArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists (select 1 from jsonb_array_elements_text({}{}) as e1 join jsonb_array_elements_text({}) as e2 on e1.value = e2.value))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionTwoOverlapsWithArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 2>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionTwoOverlapsWithArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionTwoOverlapsWithArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists (select 1 from jsonb_array_elements_text({}{}) as e1 join jsonb_array_elements_text({}) as e2 on e1.value = e2.value))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionThreeOverlapsWithArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 3>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionThreeOverlapsWithArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionThreeOverlapsWithArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists (select 1 from jsonb_array_elements_text({}{}) as e1 join jsonb_array_elements_text({}) as e2 on e1.value = e2.value))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, schemars :: JsonSchema)]
pub struct PostgresqlJsonTypeWhereElementDimensionFourOverlapsWithArray<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone> {
    pub logical_operator: postgresql_crud_common::LogicalOperator,
    pub dimensions: crate::BoundedStdVecVec<crate::UnsignedPartOfStdPrimitiveI32, 4>,
    pub value: crate::PostgresqlJsonTypeNotEmptyUniqueVec<T>,
}
impl<T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for PostgresqlJsonTypeWhereElementDimensionFourOverlapsWithArray<T> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            logical_operator: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimensions: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            value: postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl<'a, T: std::fmt::Debug + std::cmp::PartialEq + std::clone::Clone + serde::Serialize + std::marker::Send + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeWhereElementDimensionFourOverlapsWithArray<T> {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud_common::QueryPartErrorNamed> {
        let dimensions_indexes = match self.dimensions.postgresql_json_type_query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let value = match self.value.query_part(increment, column, is_need_to_add_logical_operator) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("{}(exists (select 1 from jsonb_array_elements_text({}{}) as e1 join jsonb_array_elements_text({}) as e2 on e1.value = e2.value))", &self.logical_operator.to_query_part(is_need_to_add_logical_operator), column, dimensions_indexes, value))
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>, std::string::String> {
        match self.dimensions.query_bind(query) {
            Ok(value) => {
                query = value;
            }
            Err(error) => {
                return Err(error);
            }
        }
        if let Err(error) = query.try_bind(sqlx::types::Json(self.value)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
}
