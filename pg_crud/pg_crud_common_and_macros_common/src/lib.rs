use naming::{AndSc, NotSc, OrSc};
use optimal_pack::OptimalPack;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as StdFmtResult};
pub trait DfltOptSomeVecOneEl: Sized {
    fn dflt_opt_some_vec_one_el() -> Self;
}
pub trait AllEnumVrtsArrDfltOptSomeVecOneEl: Sized {
    fn all_vrts_dflt_opt_some_vec_one_el() -> Vec<Self>;
}
pub trait DfltOptSomeVecOneElMaxPageSize: Sized {
    fn dflt_opt_some_vec_one_el_max_page_size() -> Self;
}
pub trait AllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize: Sized {
    fn all_vrts_dflt_opt_some_vec_one_el_max_page_size() -> Vec<Self>;
}
#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, JsonSchema, OptimalPack,
)]
pub enum Oprtr {
    And,
    AndNot,
    #[default]
    Or,
    OrNot,
}
impl Oprtr {
    #[must_use]
    pub fn to_qp(&self, add_oprtr: bool) -> String {
        let not_space = format!("{NotSc} ");
        if add_oprtr {
            let and_space = format!("{AndSc} ");
            let or_space = format!("{OrSc} ");
            match *self {
                Self::And => and_space,
                Self::Or => or_space,
                Self::AndNot => format!("{and_space}{not_space}"),
                Self::OrNot => format!("{or_space}{not_space}"),
            }
        } else {
            match *self {
                Self::And | Self::Or => String::default(),
                Self::AndNot | Self::OrNot => not_space,
            }
        }
    }
}
impl Display for Oprtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
        write!(f, "{self:?}")
    }
}
impl DfltOptSomeVecOneEl for Oprtr {
    fn dflt_opt_some_vec_one_el() -> Self {
        Self::default()
    }
}
impl ToTokens for Oprtr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match *self {
            Self::And => quote! {And},
            Self::Or => quote! {Or},
            Self::AndNot => quote! {AndNot},
            Self::OrNot => quote! {OrNot},
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, OptimalPack)]
pub enum PgTypeGreaterThanVrt {
    EqualNotGreaterThan,
    GreaterThan,
    NotGreaterThan,
}
impl PgTypeGreaterThanVrt {
    #[must_use]
    pub const fn oprtr(&self) -> Oprtr {
        match *self {
            Self::GreaterThan => Oprtr::Or,
            Self::NotGreaterThan | Self::EqualNotGreaterThan => Oprtr::OrNot,
        }
    }
}
impl ToTokens for PgTypeGreaterThanVrt {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match *self {
            Self::EqualNotGreaterThan => quote! {EqualNotGreaterThan},
            Self::GreaterThan => quote! {GreaterThan},
            Self::NotGreaterThan => quote! {NotGreaterThan},
        }
        .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum PgJsonTypeLengthGreaterThanVrt {
    EqualNotLengthGreaterThan,
    LengthGreaterThan,
    NotLengthGreaterThan,
}
impl PgJsonTypeLengthGreaterThanVrt {
    #[must_use]
    pub const fn oprtr(&self) -> Oprtr {
        match *self {
            Self::LengthGreaterThan => Oprtr::Or,
            Self::NotLengthGreaterThan | Self::EqualNotLengthGreaterThan => Oprtr::OrNot,
        }
    }
}
impl ToTokens for PgJsonTypeLengthGreaterThanVrt {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match *self {
            Self::EqualNotLengthGreaterThan => quote! {EqualNotLengthGreaterThan},
            Self::LengthGreaterThan => quote! {LengthGreaterThan},
            Self::NotLengthGreaterThan => quote! {NotLengthGreaterThan},
        }
        .to_tokens(tokens);
    }
}
