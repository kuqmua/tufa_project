use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use quote::ToTokens;
use std::fmt::Display;
use strum_macros::{Display, EnumIter};
#[derive(Debug, Display, EnumIter, Optml)]
pub enum IsStdrtWithId {
    False,
    True,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub enum IdentPattern {
    StdrtNnWithoutId,
    StdrtNnWithId,
    StdrtNlWithoutId,
    ArrNnWithId,
    ArrNlWithIdentifier,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Display, Optml)]
pub enum PgJsonSubtype {
    Tt,
    Cr,
    CrForQuery,
    Sel,
    Wh,
    Rd,
    RdIds,
    RdInn,
    Upd,
    UpdForQuery,
}
fn display_to_tokens(v: &dyn Display, tokens: &mut Ts2) {
    v.to_string()
        .parse::<Ts2>()
        .expect("43ac0b62")
        .to_tokens(tokens);
}
impl ToTokens for PgJsonSubtype {
    fn to_tokens(&self, tokens: &mut Ts2) {
        display_to_tokens(self, tokens);
    }
}
#[derive(Debug, Clone, Display, Optml)]
pub enum PgTypeSubtype {
    Rd,
    Upd,
}
impl ToTokens for PgTypeSubtype {
    fn to_tokens(&self, tokens: &mut Ts2) {
        display_to_tokens(self, tokens);
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug)]
pub enum PgJsonSubtypeTtOrCr {
    Tt,
    Cr,
}
impl From<&PgJsonSubtypeTtOrCr> for PgJsonSubtype {
    fn from(v: &PgJsonSubtypeTtOrCr) -> Self {
        match &v {
            PgJsonSubtypeTtOrCr::Tt => Self::Tt,
            PgJsonSubtypeTtOrCr::Cr => Self::Cr,
        }
    }
}
#[derive(Debug)]
pub enum RdWithOrWithoutAnnOrInn {
    Inn,
    WithSerdeOptIsNoneAnn,
    WithoutSerdeOptIsNoneAnn,
}
#[derive(Debug)]
pub enum AddSerdeSkipSerializingIfVecIsEmptyAnn {
    False,
    True,
}
#[derive(Debug)]
pub enum NewTypeOrStructDcl {
    NewType,
    StructDcl,
}
