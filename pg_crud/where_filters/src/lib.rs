use error_occurence_lib::ErrorOccurence;
use error_occurence_lib::code_occurence::CodeOccurence;
use pg_crud_common::{
    DefaultOptionSomeVecOneEl, NotEmptyUniqueVecTryNewErrorNamed, PgTypeWhereFilter,
    QueryPartErrorNamed, increment_checked_add_one_returning_increment,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgArguments, query::Query, types::Json};
use std::fmt::{Display, Formatter, Result as StdFmtResult, Write as _};
use thiserror::Error;
use utoipa::ToSchema;
gen_where_filters::gen_where_filters!({
    "pg_types_content_write_into_gen_where_filters_pg_types": "False",
    "pg_json_types_content_write_into_gen_where_filters_pg_json_types": "False",
    "whole_content_write_into_gen_where_filters": "False"
});

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum EncodeFormat {
    #[default]
    Base64,
    Escape,
    Hex,
}
impl Display for EncodeFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
        match &self {
            Self::Base64 => write!(f, "base64"),
            Self::Escape => write!(f, "escape"),
            Self::Hex => write!(f, "hex"),
        }
    }
}
impl DefaultOptionSomeVecOneEl for EncodeFormat {
    fn default_option_some_vec_one_el() -> Self {
        Self::default()
    }
}

//difference between NotEmptyUniqueVec and PgJsonTypeNotEmptyUniqueVec only in pg_crud_common::DefaultOptionSomeVecOneEl impl with different generic requirement and PgTypeWhereFilter
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct PgJsonTypeNotEmptyUniqueVec<T>(Vec<T>);
impl<T: PartialEq + Clone> PgJsonTypeNotEmptyUniqueVec<T> {
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
    #[must_use]
    pub const fn to_vec(&self) -> &Vec<T> {
        &self.0
    }
    pub fn try_new(value: Vec<T>) -> Result<Self, NotEmptyUniqueVecTryNewErrorNamed<T>> {
        if value.is_empty() {
            return Err(NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        {
            let mut acc_72940a4c = Vec::new();
            for el_7721a8da in &value {
                if acc_72940a4c.contains(&el_7721a8da) {
                    return Err(NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                        value: el_7721a8da.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
                acc_72940a4c.push(el_7721a8da);
            }
        }
        Ok(Self(value))
    }
}
impl<T: PartialEq + Clone + Serialize> PgJsonTypeNotEmptyUniqueVec<T> {
    pub fn query_bind_one_by_one<'query_lifetime>(
        self,
        mut query: Query<'query_lifetime, sqlx::Postgres, PgArguments>,
    ) -> Result<Query<'query_lifetime, sqlx::Postgres, PgArguments>, String>
    where
        T: 'query_lifetime,
    {
        for el_cc499cbc in self.0 {
            if let Err(error) = query.try_bind(Json(el_cc499cbc)) {
                return Err(error.to_string());
            }
        }
        Ok(query)
    }
    pub fn query_part_one_by_one(
        &self,
        increment: &mut u64,
        _: &dyn Display,
        _is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        let mut acc_ecd78d3a = String::default();
        for _ in self.to_vec() {
            match increment_checked_add_one_returning_increment(increment) {
                Ok(value) => {
                    if write!(acc_ecd78d3a, "${value},").is_err() {
                        return Err(QueryPartErrorNamed::WriteIntoBuffer {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        let _: Option<char> = acc_ecd78d3a.pop();
        Ok(acc_ecd78d3a)
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>>
        _serde::Deserialize<'de> for PgJsonTypeNotEmptyUniqueVec<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private228::PhantomData<PgJsonTypeNotEmptyUniqueVec<T>>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PgJsonTypeNotEmptyUniqueVec<T>;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __f,
                        "tuple struct PgJsonTypeNotEmptyUniqueVec",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    Ok(PgJsonTypeNotEmptyUniqueVec(__field0))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct PgJsonTypeNotEmptyUniqueVec with 1 element",
                        ));
                    };
                    match PgJsonTypeNotEmptyUniqueVec::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "PgJsonTypeNotEmptyUniqueVec",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: DefaultOptionSomeVecOneEl> DefaultOptionSomeVecOneEl for PgJsonTypeNotEmptyUniqueVec<T> {
    fn default_option_some_vec_one_el() -> Self {
        Self(vec![
            DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ])
    }
}
impl<T> Default for PgJsonTypeNotEmptyUniqueVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> From<PgJsonTypeNotEmptyUniqueVec<T>> for Vec<T> {
    fn from(value: PgJsonTypeNotEmptyUniqueVec<T>) -> Self {
        value.0
    }
}
impl<'lifetime, T> PgTypeWhereFilter<'lifetime> for PgJsonTypeNotEmptyUniqueVec<T>
where
    T: Serialize + 'lifetime,
{
    fn query_bind(
        self,
        mut query: Query<'lifetime, sqlx::Postgres, PgArguments>,
    ) -> Result<Query<'lifetime, sqlx::Postgres, PgArguments>, String> {
        if let Err(error) = query.try_bind(Json(self.0)) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut u64,
        _: &dyn Display,
        _is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        match increment_checked_add_one_returning_increment(increment) {
            Ok(value) => Ok(format!("${value}")),
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegexRegex(pub regex::Regex);
// #[automatically_derived]
// impl ::core::marker::StructuralPartialEq for RegexRegex {}
// #[automatically_derived]
impl PartialEq for RegexRegex {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
#[doc(hidden)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for RegexRegex {
        fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(
                __serializer,
                "RegexRegex",
                &self.0.to_string(),
            )
        }
    }
};
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
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
                marker: _serde::__private228::PhantomData<RegexRegex>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = RegexRegex;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "tuple struct RegexRegex",
                    )
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
                    let Some(__field0) = _serde::de::SeqAccess::next_element::<String>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct RegexRegex with 1 element",
                        ));
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
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
//todo add some logic? for regex validation?
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for RegexRegex {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed("RegexRegex")
        }
        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
            schemars::_private::alloc::borrow::Cow::Borrowed("tests::RegexRegex")
        }
        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
            { generator.subschema_for::<String>() }
        }
        fn inline_schema() -> bool {
            false
        }
    }
};
impl Display for RegexRegex {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
        write!(f, "{}", self.0)
    }
}
impl DefaultOptionSomeVecOneEl for RegexRegex {
    fn default_option_some_vec_one_el() -> Self {
        Self(regex::Regex::new("[a-z]+").expect("22a9eda5"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RegularExpressionCase {
    Insensitive,
    Sensitive,
}
impl DefaultOptionSomeVecOneEl for RegularExpressionCase {
    fn default_option_some_vec_one_el() -> Self {
        Self::Sensitive
    }
}
impl RegularExpressionCase {
    #[must_use]
    pub const fn postgreql_syntax(&self) -> &'static str {
        match &self {
            Self::Insensitive => "~*",
            Self::Sensitive => "~",
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct Between<T>
where
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
{
    start: T,
    end: T,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Error, ErrorOccurence)]
pub enum BetweenTryNewErrorNamed<T> {
    StartMoreOrEqualToEnd {
        #[eo_to_err_string_serialize_deserialize]
        start: T,
        #[eo_to_err_string_serialize_deserialize]
        end: T,
        code_occurence: CodeOccurence,
    },
}
impl<T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + PartialOrd>
    Between<T>
{
    pub fn try_new(start: T, end: T) -> Result<Self, BetweenTryNewErrorNamed<T>> {
        if start < end {
            Ok(Self { start, end })
        } else {
            Err(BetweenTryNewErrorNamed::StartMoreOrEqualToEnd {
                start,
                end,
                code_occurence: error_occurence_lib::code_occurence!(),
            })
        }
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T> _serde::Deserialize<'de> for Between<T>
    where
        T: std::fmt::Debug
            + _serde::Deserialize<'de>
            + PartialOrd
            + sqlx::Type<sqlx::Postgres>
            + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
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
                fn expecting(
                    &self,
                    __f: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__f, "field identifier")
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
                T: _serde::Deserialize<'de>
                    + sqlx::Type<sqlx::Postgres>
                    + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
            {
                marker: _serde::__private228::PhantomData<Between<T>>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
            where
                T: std::fmt::Debug
                    + _serde::Deserialize<'de>
                    + PartialOrd
                    + sqlx::Type<sqlx::Postgres>
                    + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
            {
                type Value = Between<T>;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(__f, "struct Between")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = _serde::de::SeqAccess::next_element::<T>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            1usize,
                            &"struct Between with 2 elements",
                        ));
                    };
                    let Some(__field1) = _serde::de::SeqAccess::next_element::<T>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            2usize,
                            &"struct Between with 2 elements",
                        ));
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
                    while let Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if Option::is_some(&__field0) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("start"),
                                    );
                                }
                                __field0 =
                                    Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if Option::is_some(&__field1) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("end"),
                                    );
                                }
                                __field1 =
                                    Some(_serde::de::MapAccess::next_value::<T>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let __field0_value = match __field0 {
                        Some(value) => value,
                        None => _serde::__private228::de::missing_field("start")?,
                    };
                    let __field1_value = match __field1 {
                        Some(value) => value,
                        None => _serde::__private228::de::missing_field("end")?,
                    };
                    match Between::try_new(__field0_value, __field1_value) {
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
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<
    T: DefaultOptionSomeVecOneEl
        + sqlx::Type<sqlx::Postgres>
        + for<'__> sqlx::Encode<'__, sqlx::Postgres>,
> DefaultOptionSomeVecOneEl for Between<T>
{
    fn default_option_some_vec_one_el() -> Self {
        Self {
            start: DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
            end: DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        }
    }
}
impl<
    'lifetime,
    T: Send + sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'lifetime,
> PgTypeWhereFilter<'lifetime> for Between<T>
{
    fn query_bind(
        self,
        mut query: Query<'lifetime, sqlx::Postgres, PgArguments>,
    ) -> Result<Query<'lifetime, sqlx::Postgres, PgArguments>, String> {
        if let Err(error) = query.try_bind(self.start) {
            return Err(error.to_string());
        }
        if let Err(error) = query.try_bind(self.end) {
            return Err(error.to_string());
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut u64,
        _: &dyn Display,
        _: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        let start_increment = match increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let end_increment = match increment_checked_add_one_returning_increment(increment) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        Ok(format!("between ${start_increment} and ${end_increment}"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema, JsonSchema)]
pub struct PgTypeNotEmptyUniqueVec<T>(Vec<T>);
#[allow(clippy::arbitrary_source_item_ordering)]
impl<T: PartialEq + Clone> PgTypeNotEmptyUniqueVec<T> {
    pub fn try_new(value: Vec<T>) -> Result<Self, NotEmptyUniqueVecTryNewErrorNamed<T>> {
        if value.is_empty() {
            return Err(NotEmptyUniqueVecTryNewErrorNamed::IsEmpty {
                code_occurence: error_occurence_lib::code_occurence!(),
            });
        }
        {
            let mut acc_6be6ccee = Vec::new();
            for el_b3d83e60 in &value {
                if acc_6be6ccee.contains(&el_b3d83e60) {
                    return Err(NotEmptyUniqueVecTryNewErrorNamed::NotUnique {
                        value: el_b3d83e60.clone(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
                acc_6be6ccee.push(el_b3d83e60);
            }
        }
        Ok(Self(value))
    }
    #[must_use]
    pub const fn to_vec(&self) -> &Vec<T> {
        &self.0
    }
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>>
        _serde::Deserialize<'de> for PgTypeNotEmptyUniqueVec<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: _serde::Deserialize<'de>,
            {
                marker: _serde::__private228::PhantomData<PgTypeNotEmptyUniqueVec<T>>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + _serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PgTypeNotEmptyUniqueVec<T>;
                fn expecting(
                    &self,
                    __f: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __f,
                        "tuple struct PgTypeNotEmptyUniqueVec",
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: Vec<T> = <Vec<T> as _serde::Deserialize>::deserialize(__e)?;
                    Ok(PgTypeNotEmptyUniqueVec(__field0))
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(__field0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct PgTypeNotEmptyUniqueVec with 1 element",
                        ));
                    };
                    match PgTypeNotEmptyUniqueVec::try_new(__field0) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "PgTypeNotEmptyUniqueVec",
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: DefaultOptionSomeVecOneEl> DefaultOptionSomeVecOneEl for PgTypeNotEmptyUniqueVec<T> {
    fn default_option_some_vec_one_el() -> Self {
        Self(vec![
            DefaultOptionSomeVecOneEl::default_option_some_vec_one_el(),
        ])
    }
}
impl<T> Default for PgTypeNotEmptyUniqueVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> From<PgTypeNotEmptyUniqueVec<T>> for Vec<T> {
    fn from(value: PgTypeNotEmptyUniqueVec<T>) -> Self {
        value.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Serialize, JsonSchema)]
pub struct BoundedStdVecVec<T, const LENGTH: usize>(Vec<T>);
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error, ErrorOccurence, JsonSchema,
)]
pub enum BoundedStdVecVecTryNewErrorNamed {
    LengthIsNotCorrect {
        #[eo_to_err_string_serialize_deserialize]
        wrong_length: usize,
        #[eo_to_err_string_serialize_deserialize]
        expected: usize,
        code_occurence: CodeOccurence,
    },
}
enum PgTypeOrPgJsonType {
    PgJsonType,
    PgType,
}
enum Variant {
    MinusOne,
    Normal,
}
impl<
    'lifetime,
    T: sqlx::Type<sqlx::Postgres> + for<'__> sqlx::Encode<'__, sqlx::Postgres> + 'lifetime,
    const LENGTH: usize,
> BoundedStdVecVec<T, LENGTH>
{
    #[must_use]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
    pub fn pg_json_type_query_part(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        self.query_part(
            increment,
            column,
            is_need_to_add_logical_operator,
            &PgTypeOrPgJsonType::PgJsonType,
            &Variant::Normal,
        )
    }
    pub fn pg_json_type_query_part_minus_one(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        self.query_part(
            increment,
            column,
            is_need_to_add_logical_operator,
            &PgTypeOrPgJsonType::PgJsonType,
            &Variant::MinusOne,
        )
    }
    pub fn pg_type_query_part(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        self.query_part(
            increment,
            column,
            is_need_to_add_logical_operator,
            &PgTypeOrPgJsonType::PgType,
            &Variant::Normal,
        )
    }
    pub fn pg_type_query_part_minus_one(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartErrorNamed> {
        self.query_part(
            increment,
            column,
            is_need_to_add_logical_operator,
            &PgTypeOrPgJsonType::PgType,
            &Variant::MinusOne,
        )
    }
    pub fn query_bind(
        self,
        mut query: Query<'lifetime, sqlx::Postgres, PgArguments>,
    ) -> Result<Query<'lifetime, sqlx::Postgres, PgArguments>, String> {
        for el_a05046df in self.0 {
            if let Err(error) = query.try_bind(el_a05046df) {
                return Err(error.to_string());
            }
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut u64,
        _: &dyn Display,
        _is_need_to_add_logical_operator: bool,
        pg_type_or_pg_json_type: &PgTypeOrPgJsonType,
        variant: &Variant,
    ) -> Result<String, QueryPartErrorNamed> {
        let mut acc_24eb25aa = String::new();
        let current_len = match &variant {
            Variant::MinusOne => self.0.len().saturating_sub(1),
            Variant::Normal => self.0.len(),
        };
        for _ in 0..current_len {
            match increment_checked_add_one_returning_increment(increment) {
                Ok(value) => {
                    if write!(
                        acc_24eb25aa,
                        "{}",
                        &match &pg_type_or_pg_json_type {
                            PgTypeOrPgJsonType::PgType => format!("[${value}]"),
                            PgTypeOrPgJsonType::PgJsonType => {
                                format!("->${value}")
                            }
                        }
                    )
                    .is_err()
                    {
                        return Err(QueryPartErrorNamed::WriteIntoBuffer {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(acc_24eb25aa)
    }
    #[must_use]
    pub const fn to_inner(&self) -> &Vec<T> {
        &self.0
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
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
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
                marker: _serde::__private228::PhantomData<BoundedStdVecVec<T, LENGTH>>,
                lifetime: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T, const LENGTH: usize> _serde::de::Visitor<'de> for __Visitor<'de, T, LENGTH>
            where
                T: _serde::Deserialize<'de>,
            {
                type Value = BoundedStdVecVec<T, LENGTH>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private228::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __formatter,
                        "tuple struct BoundedStdVecVec",
                    )
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
                    let Some(__field0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &"tuple struct BoundedStdVecVec with 1 element",
                        ));
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
                    marker: _serde::__private228::PhantomData::<Self>,
                    lifetime: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: Clone + DefaultOptionSomeVecOneEl, const LENGTH: usize> DefaultOptionSomeVecOneEl
    for BoundedStdVecVec<T, LENGTH>
{
    fn default_option_some_vec_one_el() -> Self {
        Self(vec![
                <T as DefaultOptionSomeVecOneEl>::default_option_some_vec_one_el();
                LENGTH
            ])
    }
}
////////////////
