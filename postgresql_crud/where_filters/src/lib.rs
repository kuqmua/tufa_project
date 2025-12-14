generate_where_filters::generate_where_filters!();

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum EncodeFormat {
    #[default]
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
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for EncodeFormat {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self::default()
    }
}

//difference between NotEmptyUniqueEnumVec and PostgresqlJsonTypeNotEmptyUniqueVec only in postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement impl with different generic requirement and PostgresqlTypeWhereFilter
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct PostgresqlJsonTypeNotEmptyUniqueVec<T>(Vec<T>);
impl<T: PartialEq + Clone> PostgresqlJsonTypeNotEmptyUniqueVec<T> {
    pub fn try_new(value: Vec<T>) -> Result<Self, postgresql_crud_common::NotEmptyUniqueVecTryNewErrorNamed<T>> {
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
    pub const fn to_vec(&self) -> &Vec<T> {
        &self.0
    }
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
impl<T: PartialEq + Clone + serde::Serialize> PostgresqlJsonTypeNotEmptyUniqueVec<T> {
    pub fn query_part_one_by_one(&self, increment: &mut u64, _: &dyn std::fmt::Display, _is_need_to_add_logical_operator: bool) -> Result<String, postgresql_crud_common::QueryPartErrorNamed> {
        let mut acc = String::default();
        for _ in self.to_vec() {
            match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
                Ok(value) => {
                    use std::fmt::Write as _;
                    if write!(acc, "${value},").is_err() {
                        return Err(postgresql_crud_common::QueryPartErrorNamed::WriteIntoBuffer {
                            code_occurence: error_occurence_lib::code_occurence!()
                        });
                    }
                },
                Err(error) => {
                    return Err(error);
                },
            }
        }
        let _: Option<char> = acc.pop();
        Ok(acc)
    }
    pub fn query_bind_one_by_one<'a>(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
        sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
        String
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
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>> _serde::Deserialize<'de> for PostgresqlJsonTypeNotEmptyUniqueVec<T> {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
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
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>> _serde::de::Visitor<'de> for __Visitor<'de, T> {
                type Value = PostgresqlJsonTypeNotEmptyUniqueVec<T>;
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "tuple struct PostgresqlJsonTypeNotEmptyUniqueVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    Ok(PostgresqlJsonTypeNotEmptyUniqueVec(__field0))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)? {
                        Some(__value) => __value,
                        None => {
                            return Err(_serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlJsonTypeNotEmptyUniqueVec with 1 element"));
                        }
                    };
                    match PostgresqlJsonTypeNotEmptyUniqueVec::try_new(__field0) {
                        Ok(value) => Ok(value),
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
impl<T> Default for PostgresqlJsonTypeNotEmptyUniqueVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> From<PostgresqlJsonTypeNotEmptyUniqueVec<T>> for Vec<T> {
    fn from(value: PostgresqlJsonTypeNotEmptyUniqueVec<T>) -> Self {
        value.0
    }
}
impl<'a, T> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for PostgresqlJsonTypeNotEmptyUniqueVec<T>
where
    T: serde::Serialize + 'a,
{
    fn query_part(&self, increment: &mut u64, _: &dyn std::fmt::Display, _is_need_to_add_logical_operator: bool) -> Result<String, postgresql_crud_common::QueryPartErrorNamed> {
        match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("${value}")),
            Err(error) => Err(error)
        }
    }
    fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
        sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
        String
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
        fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
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
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
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
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: String = <String as _serde::Deserialize>::deserialize(__e)?;
                    Ok(RegexRegex(match regex::Regex::new(&__field0) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                        Some(__value) => __value,
                        None => {
                            return Err(_serde::de::Error::invalid_length(0usize, &"tuple struct RegexRegex with 1 element"));
                        }
                    };
                    Ok(RegexRegex(match regex::Regex::new(&__field0) {
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
        fn schema_name() -> String {
            "RegexRegex".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("testing::RegexRegex")
        }
        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::schema::Schema {
            generator.subschema_for::<String>()
        }
    }
};
impl std::fmt::Display for RegexRegex {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
impl postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for RegexRegex {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(regex::Regex::new("[a-z]+").expect("error 22a9eda5-7898-41d7-8176-8acb97786e1e"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
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
    pub const fn postgreql_syntax(&self) -> &'static str {
        match &self {
            Self::Sensitive => "~",
            Self::Insensitive => "~*",
        }
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
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + PartialOrd> Between<T> {
    fn try_new(start: T, end: T) -> Result<Self, BetweenTryNewErrorNamed<T>> {
        if start < end { Ok(Self { start, end }) } else { Err(BetweenTryNewErrorNamed::StartMoreOrEqualToEnd { start, end, code_occurence: error_occurence_lib::code_occurence!() }) }
    }
}
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T> _serde::Deserialize<'de> for Between<T>
    where
        T: std::fmt::Debug + _serde::Deserialize<'de> + PartialOrd + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
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
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        1u64 => Ok(__Field::__field0),
                        2u64 => Ok(__Field::__field1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "start" => Ok(__Field::__field0),
                        "end" => Ok(__Field::__field1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"start" => Ok(__Field::__field0),
                        b"end" => Ok(__Field::__field1),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
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
                T: std::fmt::Debug + _serde::Deserialize<'de> + PartialOrd + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
            {
                type Value = Between<T>;
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "struct Between")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<T>(&mut __seq)? {
                        Some(__value) => __value,
                        None => {
                            return Err(_serde::de::Error::invalid_length(1usize, &"struct Between with 2 elements"));
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<T>(&mut __seq)? {
                        Some(__value) => __value,
                        None => {
                            return Err(_serde::de::Error::invalid_length(2usize, &"struct Between with 2 elements"));
                        }
                    };
                    match Between::try_new(__field0, __field1) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: Option<T> = None;
                    let mut __field1: Option<T> = None;
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(<__A::Error as _serde::de::Error>::duplicate_field("start"));
                                }
                                __field0 = Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(<__A::Error as _serde::de::Error>::duplicate_field("end"));
                                }
                                __field1 = Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        Some(__field0) => __field0,
                        None => _serde::__private::de::missing_field("start")?,
                    };
                    let __field1 = match __field1 {
                        Some(__field1) => __field1,
                        None => _serde::__private::de::missing_field("end")?,
                    };
                    match Between::try_new(__field0, __field1) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &[&str] = &["start", "end"];
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
impl<'a, T: Send + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'a> postgresql_crud_common::PostgresqlTypeWhereFilter<'a> for Between<T> {
    fn query_part(&self, increment: &mut u64, _: &dyn std::fmt::Display, _: bool) -> Result<String, postgresql_crud_common::QueryPartErrorNamed> {
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
        String
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
pub struct PostgresqlTypeNotEmptyUniqueVec<T>(Vec<T>);
impl<T: PartialEq + Clone> PostgresqlTypeNotEmptyUniqueVec<T> {
    pub fn try_new(value: Vec<T>) -> Result<Self, postgresql_crud_common::NotEmptyUniqueVecTryNewErrorNamed<T>> {
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
    pub const fn to_vec(&self) -> &Vec<T> {
        &self.0
    }
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>> _serde::Deserialize<'de> for PostgresqlTypeNotEmptyUniqueVec<T> {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
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
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>> _serde::de::Visitor<'de> for __Visitor<'de, T> {
                type Value = PostgresqlTypeNotEmptyUniqueVec<T>;
                fn expecting(&self, __f: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__f, "tuple struct PostgresqlTypeNotEmptyUniqueVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    Ok(PostgresqlTypeNotEmptyUniqueVec(__field0))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)? {
                        Some(__value) => __value,
                        None => {
                            return Err(_serde::de::Error::invalid_length(0usize, &"tuple struct PostgresqlTypeNotEmptyUniqueVec with 1 element"));
                        }
                    };
                    match PostgresqlTypeNotEmptyUniqueVec::try_new(__field0) {
                        Ok(value) => Ok(value),
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
impl<T> Default for PostgresqlTypeNotEmptyUniqueVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> From<PostgresqlTypeNotEmptyUniqueVec<T>> for Vec<T> {
    fn from(value: PostgresqlTypeNotEmptyUniqueVec<T>) -> Self {
        value.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, schemars::JsonSchema)]
pub struct BoundedStdVecVec<T, const LENGTH: usize>(Vec<T>);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, thiserror::Error, error_occurence_lib::ErrorOccurence, schemars::JsonSchema)]
pub enum BoundedStdVecVecTryNewErrorNamed {
    LengthIsNotCorrect {
        #[eo_to_std_string_string_serialize_deserialize]
        wrong_length: usize,
        #[eo_to_std_string_string_serialize_deserialize]
        expected: usize,
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
impl<'a, T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'a, const LENGTH: usize> BoundedStdVecVec<T, LENGTH> {
    pub const fn to_inner(&self) -> &Vec<T> {
        &self.0
    }
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
    fn query_part(&self, increment: &mut u64, _: &dyn std::fmt::Display, _is_need_to_add_logical_operator: bool, postgresql_type_or_postgresql_json_type: &PostgresqlTypeOrPostgresqlJsonType, variant: &Variant) -> Result<String, postgresql_crud_common::QueryPartErrorNamed> {
        let mut acc = String::new();
        let current_len = match &variant {
            Variant::Normal => self.0.len(),
            Variant::MinusOne => self.0.len().saturating_sub(1),
        };
        for _ in 0..current_len {
            match postgresql_crud_common::increment_checked_add_one_returning_increment(increment) {
                Ok(value) => {
                    use std::fmt::Write as _;
                    if write!(
                        acc,
                        "{}",
                        &match &postgresql_type_or_postgresql_json_type {
                            PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => format!("[${value}]"),
                            PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => {
                                format!("->${value}")
                            }
                        }
                    ).is_err() {
                        return Err(postgresql_crud_common::QueryPartErrorNamed::WriteIntoBuffer {
                            code_occurence: error_occurence_lib::code_occurence!()
                        });
                    }
                },
                Err(error) => {
                    return Err(error);
                },
            }
        }
        Ok(acc)
    }
    pub fn postgresql_type_query_part(&self, increment: &mut u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: bool) -> Result<String, postgresql_crud_common::QueryPartErrorNamed> {
        self.query_part(increment, column, is_need_to_add_logical_operator, &PostgresqlTypeOrPostgresqlJsonType::PostgresqlType, &Variant::Normal)
    }
    pub fn postgresql_json_type_query_part(&self, increment: &mut u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: bool) -> Result<String, postgresql_crud_common::QueryPartErrorNamed> {
        self.query_part(increment, column, is_need_to_add_logical_operator, &PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType, &Variant::Normal)
    }
    pub fn postgresql_type_query_part_minus_one(&self, increment: &mut u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: bool) -> Result<String, postgresql_crud_common::QueryPartErrorNamed> {
        self.query_part(increment, column, is_need_to_add_logical_operator, &PostgresqlTypeOrPostgresqlJsonType::PostgresqlType, &Variant::MinusOne)
    }
    pub fn postgresql_json_type_query_part_minus_one(&self, increment: &mut u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: bool) -> Result<String, postgresql_crud_common::QueryPartErrorNamed> {
        self.query_part(increment, column, is_need_to_add_logical_operator, &PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType, &Variant::MinusOne)
    }
    pub fn query_bind(self, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
        sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
        String
    > {
        for element in self.0 {
            if let Err(error) = query.try_bind(element) {
                return Err(error.to_string());
            }
        }
        Ok(query)
    }
}
impl<T, const LENGTH: usize> TryFrom<Vec<T>> for BoundedStdVecVec<T, LENGTH> {
    type Error = BoundedStdVecVecTryNewErrorNamed;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
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
    impl<'de, T, const LENGTH: usize> _serde::Deserialize<'de> for BoundedStdVecVec<T, LENGTH>
    where
        T: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T, const LENGTH: usize>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private::PhantomData<BoundedStdVecVec<T, LENGTH>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T, const LENGTH: usize> _serde::de::Visitor<'de> for __Visitor<'de, T, LENGTH>
            where
                T: _serde::Deserialize<'de>,
            {
                type Value = BoundedStdVecVec<T, LENGTH>;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct BoundedStdVecVec")
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    match BoundedStdVecVec::try_from(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)? {
                        Some(__value) => __value,
                        None => {
                            return Err(_serde::de::Error::invalid_length(0usize, &"tuple struct BoundedStdVecVec with 1 element"));
                        }
                    };
                    match BoundedStdVecVec::try_from(__field0) {
                        Ok(value) => Ok(value),
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
impl<T: Clone + postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement, const LENGTH: usize> postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for BoundedStdVecVec<T, LENGTH> {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![<T as postgresql_crud_common::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(); LENGTH])
    }
}
////////////////
