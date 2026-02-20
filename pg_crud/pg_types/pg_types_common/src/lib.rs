use location_lib::loc::Loc;
use location_lib::{Location, loc};
use pg_crud_common::{
    DEFAULT_PAGINATION_LIMIT, DefaultOptionSomeVecOneEl, DefaultOptionSomeVecOneElMaxPageSize,
    PaginationBase, PgTypeWhereFilter, QueryPartEr,
};
use schemars::JsonSchema;
use serde::de::{Error as SerdeEr, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{Postgres, postgres::PgArguments, query::Query};
use std::fmt::Display;
use thiserror::Error;
use utoipa::ToSchema;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, ToSchema, JsonSchema)]
pub struct PaginationStartsWithOne(PaginationBase);
#[derive(Debug, Serialize, Deserialize, Error, Location)]
pub enum PaginationStartsWithOneTryNewEr {
    LimitIsLessThanOrEqualToZero {
        #[eo_to_err_string_serde]
        limit: i64,
        loc: Loc,
    },
    OffsetIsLessThanOne {
        #[eo_to_err_string_serde]
        offset: i64,
        loc: Loc,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: i64,
        #[eo_to_err_string_serde]
        offset: i64,
        loc: Loc,
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
    pub fn try_new(limit: i64, offset: i64) -> Result<Self, PaginationStartsWithOneTryNewEr> {
        if limit <= 0 || offset < 1 {
            if limit <= 0 {
                Err(
                    PaginationStartsWithOneTryNewEr::LimitIsLessThanOrEqualToZero {
                        limit,
                        loc: loc!(),
                    },
                )
            } else {
                Err(PaginationStartsWithOneTryNewEr::OffsetIsLessThanOne {
                    offset,
                    loc: loc!(),
                })
            }
        } else if offset.checked_add(limit).is_some() {
            Ok(Self(PaginationBase::new_unchecked(limit, offset)))
        } else {
            Err(
                PaginationStartsWithOneTryNewEr::OffsetPlusLimitIsIntOverflow {
                    limit,
                    offset,
                    loc: loc!(),
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
            f0,
            f1,
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
            fn visit_u64<__E>(self, v: u64) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match v {
                    0u64 => Ok(__Field::f0),
                    1u64 => Ok(__Field::f1),
                    _ => Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(self, v: &str) -> Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match v {
                    "limit" => Ok(__Field::f0),
                    "offset" => Ok(__Field::f1),
                    _ => Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
            where
                __E: SerdeEr,
            {
                match v {
                    b"limit" => Ok(__Field::f0),
                    b"offset" => Ok(__Field::f1),
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
                let Some(f0) = SeqAccess::next_element::<i64>(&mut __seq)? else {
                    return Err(SerdeEr::invalid_length(
                        0usize,
                        &"struct PaginationStartsWithOne with 2 elements",
                    ));
                };
                let Some(f1) = SeqAccess::next_element::<i64>(&mut __seq)? else {
                    return Err(SerdeEr::invalid_length(
                        1usize,
                        &"struct PaginationStartsWithOne with 2 elements",
                    ));
                };
                match PaginationStartsWithOne::try_new(f0, f1) {
                    Ok(v) => Ok(v),
                    Err(er) => Err(SerdeEr::custom(format!("{er:?}"))),
                }
            }
            #[inline]
            fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
            where
                __A: MapAccess<'de>,
            {
                let mut f0: Option<i64> = None;
                let mut f1: Option<i64> = None;
                while let Some(__key) = MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::f0 => {
                            if Option::is_some(&f0) {
                                return Err(<__A::Error as SerdeEr>::duplicate_field("limit"));
                            }
                            f0 = Some(MapAccess::next_value::<i64>(&mut __map)?);
                        }
                        __Field::f1 => {
                            if Option::is_some(&f1) {
                                return Err(<__A::Error as SerdeEr>::duplicate_field("offset"));
                            }
                            f1 = Some(MapAccess::next_value::<i64>(&mut __map)?);
                        }
                        __Field::__ignore => {
                            let _: IgnoredAny = MapAccess::next_value::<IgnoredAny>(&mut __map)?;
                        }
                    }
                }
                let f0_value = match f0 {
                    Some(v) => v,
                    None => serde::__private228::de::missing_field("limit")?,
                };
                let f1_value = match f1 {
                    Some(v) => v,
                    None => serde::__private228::de::missing_field("offset")?,
                };
                match PaginationStartsWithOne::try_new(f0_value, f1_value) {
                    Ok(v) => Ok(v),
                    Err(er) => Err(SerdeEr::custom(format!("{er:?}"))),
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
        incr: &mut u64,
        column: &dyn Display,
        is_need_to_add_logical_operator: bool,
    ) -> Result<String, QueryPartEr> {
        self.0
            .query_part(incr, column, is_need_to_add_logical_operator)
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
