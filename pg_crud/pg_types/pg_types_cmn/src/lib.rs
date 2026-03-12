use location_lib::loc::Loc;
use location_lib::{Location, loc};
use optml::Optml;
use pg_crud_cmn::{
    DEFAULT_PAGINATION_LIMIT, DfltSomeOneEl, DfltSomeOneElMaxPageSize, PgTypeWhFlt, PgnBase, QpEr,
};
use schemars::JsonSchema;
use serde::de::{Error as SerdeEr, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{Postgres, postgres::PgArguments, query::Query};
use std::fmt::Display;
use thiserror::Error;
use utoipa::ToSchema;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, ToSchema, JsonSchema, Optml)]
pub struct PgnStartsWithOne(PgnBase);
#[derive(Debug, Serialize, Deserialize, Error, Location, Optml)]
pub enum PgnStartsWithOneTryNewEr {
    LimitIsLessThanOrEqToZero {
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
impl PgnStartsWithOne {
    #[must_use]
    pub const fn end(&self) -> i64 {
        self.0.end()
    }
    #[must_use]
    pub const fn start(&self) -> i64 {
        self.0.start()
    }
    pub fn try_new(limit: i64, offset: i64) -> Result<Self, PgnStartsWithOneTryNewEr> {
        if limit <= 0 || offset < 1 {
            if limit <= 0 {
                Err(PgnStartsWithOneTryNewEr::LimitIsLessThanOrEqToZero { limit, loc: loc!() })
            } else {
                Err(PgnStartsWithOneTryNewEr::OffsetIsLessThanOne {
                    offset,
                    loc: loc!(),
                })
            }
        } else if offset.checked_add(limit).is_some() {
            Ok(Self(PgnBase::new_unchecked(limit, offset)))
        } else {
            Err(PgnStartsWithOneTryNewEr::OffsetPlusLimitIsIntOverflow {
                limit,
                offset,
                loc: loc!(),
            })
        }
    }
}
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
impl<'de> Deserialize<'de> for PgnStartsWithOne {
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
            marker: serde::__private228::PhantomData<PgnStartsWithOne>,
            lt: serde::__private228::PhantomData<&'de ()>,
        }
        impl<'de> Visitor<'de> for __Visitor<'de> {
            type Value = PgnStartsWithOne;
            fn expecting(
                &self,
                __f: &mut serde::__private228::Formatter<'_>,
            ) -> serde::__private228::fmt::Result {
                serde::__private228::Formatter::write_str(__f, "struct PgnStartsWithOne")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
            where
                __A: SeqAccess<'de>,
            {
                let Some(f0) = SeqAccess::next_element::<i64>(&mut __seq)? else {
                    return Err(SerdeEr::invalid_length(
                        0usize,
                        &"struct PgnStartsWithOne with 2 els",
                    ));
                };
                let Some(f1) = SeqAccess::next_element::<i64>(&mut __seq)? else {
                    return Err(SerdeEr::invalid_length(
                        1usize,
                        &"struct PgnStartsWithOne with 2 els",
                    ));
                };
                match PgnStartsWithOne::try_new(f0, f1) {
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
                while let Some(__k) = MapAccess::next_key::<__Field>(&mut __map)? {
                    match __k {
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
                let f0_v = match f0 {
                    Some(v) => v,
                    None => serde::__private228::de::missing_field("limit")?,
                };
                let f1_v = match f1 {
                    Some(v) => v,
                    None => serde::__private228::de::missing_field("offset")?,
                };
                match PgnStartsWithOne::try_new(f0_v, f1_v) {
                    Ok(v) => Ok(v),
                    Err(er) => Err(SerdeEr::custom(format!("{er:?}"))),
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &[&str] = &["limit", "offset"];
        Deserializer::deserialize_struct(
            __deserializer,
            "PgnStartsWithOne",
            FIELDS,
            __Visitor {
                marker: serde::__private228::PhantomData::<Self>,
                lt: serde::__private228::PhantomData,
            },
        )
    }
}
impl<'lt> PgTypeWhFlt<'lt> for PgnStartsWithOne {
    fn qb(
        self,
        query: Query<'lt, Postgres, PgArguments>,
    ) -> Result<Query<'lt, Postgres, PgArguments>, String> {
        self.0.qb(query)
    }
    fn qp(&self, incr: &mut u64, column: &dyn Display, add_oprtr: bool) -> Result<String, QpEr> {
        self.0.qp(incr, column, add_oprtr)
    }
}
impl DfltSomeOneEl for PgnStartsWithOne {
    #[inline]
    fn dflt_some_one_el() -> Self {
        Self(PgnBase::new_unchecked(DEFAULT_PAGINATION_LIMIT, 1))
    }
}
impl DfltSomeOneElMaxPageSize for PgnStartsWithOne {
    #[inline]
    fn dflt_some_one_el_max_page_size() -> Self {
        let one: i32 = 1;
        Self(PgnBase::new_unchecked(
            i32::MAX.checked_sub(one).expect("c0f03c51").into(),
            one.into(),
        ))
    }
}
#[must_use]
pub fn mb_pk(v: bool) -> impl Display {
    if v { "primary key" } else { "" }
}
