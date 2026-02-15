use error_occurence_lib::code_occurence::CodeOccurence;
use error_occurence_lib::{ErrorOccurence, code_occurence};
use pg_crud_common::{
    DEFAULT_PAGINATION_LIMIT, DefaultOptionSomeVecOneEl, DefaultOptionSomeVecOneElMaxPageSize,
    PaginationBase, PgTypeWhereFilter, QueryPartError,
};
use schemars::JsonSchema;
use serde::de::{Error as SerdeError, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{Postgres, postgres::PgArguments, query::Query};
use std::fmt::Display;
use thiserror::Error;
use utoipa::ToSchema;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, ToSchema, JsonSchema)]
pub struct PaginationStartsWithOne(PaginationBase);
#[derive(Debug, Serialize, Deserialize, Error, ErrorOccurence)]
pub enum PaginationStartsWithOneTryNewError {
    LimitIsLessThanOrEqualToZero {
        #[eo_to_err_string_serialize_deserialize]
        limit: i64,
        code_occurence: CodeOccurence,
    },
    OffsetIsLessThanOne {
        #[eo_to_err_string_serialize_deserialize]
        offset: i64,
        code_occurence: CodeOccurence,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serialize_deserialize]
        limit: i64,
        #[eo_to_err_string_serialize_deserialize]
        offset: i64,
        code_occurence: CodeOccurence,
    },
}
impl PaginationStartsWithOne {
    #[must_use]
    pub const fn end(&self) -> i64 {
        self.0.end()
    }
    #[must_use]
    pub const fn start(&self) -> i64 {
        self.0.start()
    }
    pub fn try_new(limit: i64, offset: i64) -> Result<Self, PaginationStartsWithOneTryNewError> {
        if limit <= 0 || offset < 1 {
            if limit <= 0 {
                Err(
                    PaginationStartsWithOneTryNewError::LimitIsLessThanOrEqualToZero {
                        limit,
                        code_occurence: code_occurence!(),
                    },
                )
            } else {
                Err(PaginationStartsWithOneTryNewError::OffsetIsLessThanOne {
                    offset,
                    code_occurence: code_occurence!(),
                })
            }
        } else if offset.checked_add(limit).is_some() {
            Ok(Self(PaginationBase::new_unchecked(limit, offset)))
        } else {
            Err(
                PaginationStartsWithOneTryNewError::OffsetPlusLimitIsIntOverflow {
                    limit,
                    offset,
                    code_occurence: code_occurence!(),
                },
            )
        }
    }
}
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
impl<'de> Deserialize<'de> for PaginationStartsWithOne {
    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
    where
        __D: Deserializer<'de>,
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
        impl Visitor<'_> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __f: &mut serde::__private228::Formatter<'_>,
            ) -> serde::__private228::fmt::Result {
                serde::__private228::Formatter::write_str(__f, "field identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => Ok(__Field::__field0),
                    1u64 => Ok(__Field::__field1),
                    _ => Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "limit" => Ok(__Field::__field0),
                    "offset" => Ok(__Field::__field1),
                    _ => Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
            where
                __E: SerdeError,
            {
                match __value {
                    b"limit" => Ok(__Field::__field0),
                    b"offset" => Ok(__Field::__field1),
                    _ => Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: Deserializer<'de>,
            {
                Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        #[doc(hidden)]
        struct __Visitor<'de> {
            marker: serde::__private228::PhantomData<PaginationStartsWithOne>,
            lifetime: serde::__private228::PhantomData<&'de ()>,
        }
        impl<'de> Visitor<'de> for __Visitor<'de> {
            type Value = PaginationStartsWithOne;
            fn expecting(
                &self,
                __f: &mut serde::__private228::Formatter<'_>,
            ) -> serde::__private228::fmt::Result {
                serde::__private228::Formatter::write_str(__f, "struct PaginationStartsWithOne")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
            where
                __A: SeqAccess<'de>,
            {
                let Some(__field0) = SeqAccess::next_element::<i64>(&mut __seq)? else {
                    return Err(SerdeError::invalid_length(
                        0usize,
                        &"struct PaginationStartsWithOne with 2 elements",
                    ));
                };
                let Some(__field1) = SeqAccess::next_element::<i64>(&mut __seq)? else {
                    return Err(SerdeError::invalid_length(
                        1usize,
                        &"struct PaginationStartsWithOne with 2 elements",
                    ));
                };
                match PaginationStartsWithOne::try_new(__field0, __field1) {
                    Ok(value) => Ok(value),
                    Err(error) => Err(SerdeError::custom(format!("{error:?}"))),
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
            where
                __A: MapAccess<'de>,
            {
                let mut __field0: Option<i64> = None;
                let mut __field1: Option<i64> = None;
                while let Some(__key) = MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if Option::is_some(&__field0) {
                                return Err(<__A::Error as SerdeError>::duplicate_field("limit"));
                            }
                            __field0 = Some(MapAccess::next_value::<i64>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if Option::is_some(&__field1) {
                                return Err(<__A::Error as SerdeError>::duplicate_field("offset"));
                            }
                            __field1 = Some(MapAccess::next_value::<i64>(&mut __map)?);
                        }
                        __Field::__ignore => {
                            let _: IgnoredAny = MapAccess::next_value::<IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let __field0_value = match __field0 {
                    Some(value) => value,
                    None => serde::__private228::de::missing_field("limit")?,
                };
                let __field1_value = match __field1 {
                    Some(value) => value,
                    None => serde::__private228::de::missing_field("offset")?,
                };
                match PaginationStartsWithOne::try_new(__field0_value, __field1_value) {
                    Ok(value) => Ok(value),
                    Err(error) => Err(SerdeError::custom(format!("{error:?}"))),
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &[&str] = &["limit", "offset"];
        Deserializer::deserialize_struct(
            __deserializer,
            "PaginationStartsWithOne",
            FIELDS,
            __Visitor {
                marker: serde::__private228::PhantomData::<Self>,
                lifetime: serde::__private228::PhantomData,
            },
        )
    }
}
impl<'lifetime> PgTypeWhereFilter<'lifetime> for PaginationStartsWithOne {
    fn query_bind(
        self,
        query: Query<'lifetime, Postgres, PgArguments>,
    ) -> Result<Query<'lifetime, Postgres, PgArguments>, String> {
        self.0.query_bind(query)
    }
    fn query_part(
        &self,
        increment: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartError> {
        self.0
            .query_part(increment, column, is_need_to_add_logical_operator)
    }
}
impl DefaultOptionSomeVecOneEl for PaginationStartsWithOne {
    #[inline]
    fn default_option_some_vec_one_el() -> Self {
        Self(PaginationBase::new_unchecked(DEFAULT_PAGINATION_LIMIT, 1))
    }
}
impl DefaultOptionSomeVecOneElMaxPageSize for PaginationStartsWithOne {
    #[inline]
    fn default_option_some_vec_one_el_max_page_size() -> Self {
        let one: i32 = 1;
        Self(PaginationBase::new_unchecked(
            i32::MAX.checked_sub(one).expect("c0f03c51").into(),
            one.into(),
        ))
    }
}
#[must_use]
pub fn maybe_primary_key(is_primary_key: bool) -> impl Display {
    if is_primary_key { "primary key" } else { "" }
}
