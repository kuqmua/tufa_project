use loc_lib::loc::Loc;
use loc_lib::{Location, loc};
use optml::Optml;
use pg_crud_cmn::{
    DEFAULT_PAGINATION_LIMIT, DfltSomeOneEl, DfltSomeOneElMaxPageSize, PgTypeWhFlt, PgnBase, QpEr,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, postgres::PgArguments, query::Query};
use std::fmt::Display;
use thiserror::Error;
use utoipa::ToSchema;
#[derive(Debug, Deserialize, JsonSchema, Optml)]
struct PgnStartsWithOneRaw {
    limit: i64,
    offset: i64,
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema, JsonSchema, Optml,
)]
#[serde(try_from = "PgnStartsWithOneRaw")]
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
impl TryFrom<PgnStartsWithOneRaw> for PgnStartsWithOne {
    type Error = PgnStartsWithOneTryNewEr;
    fn try_from(v: PgnStartsWithOneRaw) -> Result<Self, Self::Error> {
        Self::try_new(v.limit, v.offset)
    }
}
impl<'lt> PgTypeWhFlt<'lt> for PgnStartsWithOne {
    fn qb(
        self,
        query: Query<'lt, Postgres, PgArguments>,
    ) -> Result<Query<'lt, Postgres, PgArguments>, String> {
        self.0.qb(query)
    }
    fn qp(&self, incr: &mut u64, col: &dyn Display, add_oprtr: bool) -> Result<String, QpEr> {
        self.0.qp(incr, col, add_oprtr)
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
