use naming::{AndSc, NotSc, OrSc};
use quote::quote;
use std::fmt::{Display, Formatter, Result as StdFmtResult};

pub trait DefaultOptionSomeVecOneEl: Sized {
    fn default_option_some_vec_one_el() -> Self;
}
pub trait AllEnumVariantsArrayDefaultOptionSomeVecOneEl: Sized {
    fn all_variants_default_option_some_vec_one_el() -> Vec<Self>;
}
pub trait DefaultOptionSomeVecOneElMaxPageSize: Sized {
    fn default_option_some_vec_one_el_max_page_size() -> Self;
}
pub trait AllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize: Sized {
    fn all_variants_default_option_some_vec_one_el_max_page_size() -> Vec<Self>;
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    schemars::JsonSchema,
)]
pub enum LogicalOperator {
    And,
    AndNot,
    #[default]
    Or,
    OrNot,
}
impl LogicalOperator {
    #[must_use]
    pub fn to_query_part(&self, is_need_to_add_logical_operator: bool) -> String {
        let not_space = format!("{NotSc} ");
        if is_need_to_add_logical_operator {
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
impl Display for LogicalOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
        write!(f, "{self:?}")
    }
}
impl DefaultOptionSomeVecOneEl for LogicalOperator {
    fn default_option_some_vec_one_el() -> Self {
        Self::default()
    }
}
impl quote::ToTokens for LogicalOperator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            Self::And => quote! {And},
            Self::Or => quote! {Or},
            Self::AndNot => quote! {AndNot},
            Self::OrNot => quote! {OrNot},
        }
        .to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostgresTypeGreaterThanVariant {
    EqualNotGreaterThan,
    GreaterThan,
    NotGreaterThan,
}
impl PostgresTypeGreaterThanVariant {
    #[must_use]
    pub const fn logical_operator(&self) -> LogicalOperator {
        match *self {
            Self::GreaterThan => LogicalOperator::Or,
            Self::NotGreaterThan | Self::EqualNotGreaterThan => LogicalOperator::OrNot,
        }
    }
}
impl quote::ToTokens for PostgresTypeGreaterThanVariant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            Self::EqualNotGreaterThan => quote! {EqualNotGreaterThan},
            Self::GreaterThan => quote! {GreaterThan},
            Self::NotGreaterThan => quote! {NotGreaterThan},
        }
        .to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PostgresJsonTypeLengthGreaterThanVariant {
    EqualNotLengthGreaterThan,
    LengthGreaterThan,
    NotLengthGreaterThan,
}
impl PostgresJsonTypeLengthGreaterThanVariant {
    #[must_use]
    pub const fn logical_operator(&self) -> LogicalOperator {
        match *self {
            Self::LengthGreaterThan => LogicalOperator::Or,
            Self::NotLengthGreaterThan | Self::EqualNotLengthGreaterThan => LogicalOperator::OrNot,
        }
    }
}
impl quote::ToTokens for PostgresJsonTypeLengthGreaterThanVariant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match *self {
            Self::EqualNotLengthGreaterThan => quote! {EqualNotLengthGreaterThan},
            Self::LengthGreaterThan => quote! {LengthGreaterThan},
            Self::NotLengthGreaterThan => quote! {NotLengthGreaterThan},
        }
        .to_tokens(tokens);
    }
}
