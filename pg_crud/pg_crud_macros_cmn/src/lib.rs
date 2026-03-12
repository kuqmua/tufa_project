mod filters;
use enum_extension_lib::EnumExtension;
pub use filters::*;
use gen_quotes::{dq_str, dq_ts};
use macros_helpers::gen_impl_to_err_string_ts;
use naming::{
    AddOprtrSc, AllVrtsDfltSomeOneElMaxPageSizeSc, AllVrtsDfltSomeOneElSc, ColumnFieldForErMsgSc,
    ColumnFieldSc, ColumnSc, CrForQueryUcc, CrIntoPgJsonOptVecWhLenEqSc,
    CrIntoPgJsonOptVecWhLenGreaterThanSc, CrIntoPgTypeOptVecWhDimOneEqSc, CrQbSc, CrQpSc, CrSc,
    CrTblColumnQpSc, CrUcc, DfltSomeOneElMaxPageSizeSc, DfltSomeOneElSc, DisplayPlusToTokens,
    EqOprtrUcc, FiSc, IncrSc, IsPkSc, JsonbSetAccumulatorSc, JsonbSetPathSc, JsonbSetTargetSc,
    MutSc, NormalizeSc, OptUcc, OptUpdSc, OptVecCrSc, PgJsonTestCasesUcc, PgJsonUcc,
    PgTypeEqOprtrUcc, PgTypeNotPkUcc, PgTypeOptVecWhGreaterThanTestSc, PgTypeTestCasesUcc,
    PgTypeUcc, PgTypeWhFilterUcc, PreviousRdAndOptUpdIntoRdSc, QbSc, QpErUcc, QpSc, QuerySc,
    RdIdsAndCrIntoOptVRdSc, RdIdsAndCrIntoOptVecWhEqToJsonFieldSc,
    RdIdsAndCrIntoPgJsonOptVecWhBtwnSc, RdIdsAndCrIntoPgJsonOptVecWhContainsElGreaterThanSc,
    RdIdsAndCrIntoPgJsonOptVecWhContainsElRgxSc, RdIdsAndCrIntoPgJsonOptVecWhDimFourEqSc,
    RdIdsAndCrIntoPgJsonOptVecWhDimOneEqSc, RdIdsAndCrIntoPgJsonOptVecWhDimThreeEqSc,
    RdIdsAndCrIntoPgJsonOptVecWhDimTwoEqSc, RdIdsAndCrIntoPgJsonOptVecWhGreaterThanSc,
    RdIdsAndCrIntoPgJsonOptVecWhInSc, RdIdsAndCrIntoPgJsonOptVecWhRgxSc, RdIdsAndCrIntoRdSc,
    RdIdsAndCrIntoTtSc, RdIdsAndCrIntoVecWhEqToJsonFieldSc, RdIdsAndCrIntoVecWhEqUsingFieldsSc,
    RdIdsAndCrIntoWhEqSc, RdIdsAndTtIntoPgTypeOptWhGreaterThanSc, RdIdsIntoOptVRdInnSc, RdIdsSc,
    RdIdsTo2DimsVecRdInnSc, RdIdsToOptVRdDfltSomeOneElSc, RdIdsUcc,
    RdInnIntoRdWithNewOrTryNewUnwrapedSc, RdInnIntoUpdWithNewOrTryNewUnwrapedSc, RdInnUcc, RdSc,
    RdUcc, SelOnlyCrdIdsQbSc, SelOnlyCrdIdsQpSc, SelOnlyIdsQpSc, SelOnlyUpddIdsQbSc,
    SelOnlyUpddIdsQpSc, SelQpSc, SelUcc, SelfUcc, TtSc, TtUcc, UpdForQueryUcc, UpdQbSc, UpdQpSc,
    UpdToRdIdsSc, UpdUcc, VSc, VUcc, ValueSc, WhUcc,
    prm::{SelfCrUcc, SelfSelUcc, SelfWhUcc},
};
use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::{Display, EnumIter};
use syn::{Ident, Type};
use token_patterns::{
    AllowClippyArbitrarySrcItemOrdering, Bool, CrateAllEnumVrtsArrDfltSomeOneEl,
    CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize, CrateDfltSomeOneEl, CrateDfltSomeOneElMaxPageSize,
    PgCrudAllEnumVrtsArrDfltSomeOneEl, PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    PgCrudCmnAllEnumVrtsArrDfltSomeOneEl, PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize,
    PgCrudCmnDfltSomeOneEl, PgCrudCmnDfltSomeOneElCall, PgCrudCmnDfltSomeOneElMaxPageSize,
    PgCrudDfltSomeOneEl, PgCrudDfltSomeOneElMaxPageSize, RefStr, StdFmtDisplay, StringTs, U64,
};
#[derive(Debug, Clone, Optml)]
pub enum DeriveOrImpl {
    Derive,
    Impl(Ts2),
}
#[derive(Debug, Optml)]
pub enum IsStdrtNn {
    False,
    True,
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    EnumIter,
    EnumExtension,
    Optml,
)]
pub enum IsNl {
    #[default]
    False,
    True,
}
impl IsNl {
    #[must_use]
    pub fn mb_opt_wrap(&self, ts: Ts2) -> Ts2 {
        match &self {
            Self::False => ts,
            Self::True => quote! {Option<#ts>},
        }
    }
    #[must_use]
    pub fn mb_some_wrap(&self, ts: Ts2) -> Ts2 {
        match &self {
            Self::False => ts,
            Self::True => quote! {Some(#ts)},
        }
    }
    #[must_use]
    pub const fn nn_or_nl_str(&self) -> &str {
        match &self {
            Self::False => "Nn",
            Self::True => "Nl",
        }
    }
    #[must_use]
    pub fn prefix_str(&self) -> String {
        match &self {
            Self::False => String::default(),
            Self::True => String::from("StdOptOpt"),
        }
    }
    #[must_use]
    pub fn rust(&self) -> &'static dyn Display {
        match &self {
            Self::False => &"",
            Self::True => &OptUcc,
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum Import {
    Crate,
    PgCrud,
    PgCrudCmn,
}
impl Import {
    fn all_vrts_dflt_some_one_el(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateAllEnumVrtsArrDfltSomeOneEl,
            Self::PgCrud => &PgCrudAllEnumVrtsArrDfltSomeOneEl,
            Self::PgCrudCmn => &PgCrudCmnAllEnumVrtsArrDfltSomeOneEl,
        }
    }
    fn all_vrts_dflt_some_one_el_max_page_size(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateAllEnumVrtsArrDfltSomeOneElMaxPageSize,
            Self::PgCrud => &PgCrudAllEnumVrtsArrDfltSomeOneElMaxPageSize,
            Self::PgCrudCmn => &PgCrudCmnAllEnumVrtsArrDfltSomeOneElMaxPageSize,
        }
    }
    fn dflt_some_one_el(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateDfltSomeOneEl,
            Self::PgCrud => &PgCrudDfltSomeOneEl,
            Self::PgCrudCmn => &PgCrudCmnDfltSomeOneEl,
        }
    }
    fn dflt_some_one_el_max_page_size(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateDfltSomeOneElMaxPageSize,
            Self::PgCrud => &PgCrudDfltSomeOneElMaxPageSize,
            Self::PgCrudCmn => &PgCrudCmnDfltSomeOneElMaxPageSize,
        }
    }
    #[must_use]
    pub const fn sc_str(&self) -> &'static str {
        match &self {
            Self::Crate => "crate",
            Self::PgCrud => "pg_crud",
            Self::PgCrudCmn => "pg_crud_cmn",
        }
    }
    #[must_use]
    pub const fn to_path(&self) -> &'static str {
        match &self {
            Self::Crate => "crate",
            Self::PgCrud => "pg_crud",
            Self::PgCrudCmn => "pg_crud_cmn",
        }
    }
}
impl ToTokens for Import {
    fn to_tokens(&self, tokens: &mut Ts2) {
        self.sc_str()
            .parse::<Ts2>()
            .expect("d8636ee5")
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum ShouldDSchemarsJsonSchema {
    False,
    True,
}
impl ToTokens for ShouldDSchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => quote! {, schemars::JsonSchema}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum ShouldDeriveUtoipaToSchema {
    False,
    True,
}
impl ToTokens for ShouldDeriveUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => quote! {, utoipa::ToSchema}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsCrQbMut {
    False,
    True,
}
impl ToTokens for IsCrQbMut {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsSelQpSelfSelUsed {
    False,
    True,
}
impl ToTokens for IsSelQpSelfSelUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => VSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsSelQpColumnFieldForErMsgUsed {
    False,
    True,
}
impl ToTokens for IsSelQpColumnFieldForErMsgUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => {
                ColumnFieldForErMsgSc.to_tokens(tokens);
            }
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsSelQpIsPgTypeUsed {
    False,
    True,
}
impl ToTokens for IsSelQpIsPgTypeUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => quote! {is_pg_type}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsUpdQpSelfUpdUsed {
    False,
    True,
}
impl ToTokens for IsUpdQpSelfUpdUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => VSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsUpdQpJsonbSetTargetUsed {
    False,
    True,
}
impl ToTokens for IsUpdQpJsonbSetTargetUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => JsonbSetTargetSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsUpdQbMut {
    False,
    True,
}
impl ToTokens for IsUpdQbMut {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsSelOnlyUpddIdsQbMut {
    False,
    True,
}
impl ToTokens for IsSelOnlyUpddIdsQbMut {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsSelOnlyCrdIdsQbMut {
    False,
    True,
}
impl ToTokens for IsSelOnlyCrdIdsQbMut {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsQbMut {
    False,
    True,
}
impl ToTokens for IsQbMut {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IncrPrmUndrscr {
    False,
    True,
}
impl ToTokens for IncrPrmUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => IncrSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum ColumnPrmUndrscr {
    False,
    True,
}
impl ToTokens for ColumnPrmUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => ColumnSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum AddOprtrUndrscr {
    False,
    True,
}
impl ToTokens for AddOprtrUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => AddOprtrSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum RdOrUpd {
    Rd,
    Upd,
}
impl RdOrUpd {
    #[must_use]
    pub fn ucc(&self) -> &dyn DisplayPlusToTokens {
        match &self {
            Self::Rd => &RdUcc,
            Self::Upd => &UpdUcc,
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum IsPkUndrscr {
    False,
    True,
}
impl ToTokens for IsPkUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => IsPkSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum PgTypeOrPgJson {
    PgJson,
    PgType,
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum DefaultSomeOneOrDefaultSomeOneWithMaxPageSize {
    DefaultSomeOne,
    DefaultSomeOneWithMaxPageSize,
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum EqOrEqUsingFields {
    Eq,
    EqUsingFields,
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum EqOprtrH {
    Eq,
    IsNull,
}
impl EqOprtrH {
    #[must_use]
    pub fn to_tokens_path(&self, import: &Import) -> Ts2 {
        let ts = match &self {
            Self::Eq => quote! {Eq},
            Self::IsNull => quote! {IsNull},
        };
        quote! {#import::#EqOprtrUcc::#ts}
    }
}
//todo mb reuse with other structs
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, Optml)]
pub enum Dim {
    One,
    Two,
    Three,
    Four,
}
impl Dim {
    #[must_use]
    pub fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_nbr_eq_sc(
        &self,
    ) -> Box<dyn DisplayPlusToTokens> {
        match self {
            Self::One => Box::new(RdIdsAndCrIntoPgJsonOptVecWhDimOneEqSc),
            Self::Two => Box::new(RdIdsAndCrIntoPgJsonOptVecWhDimTwoEqSc),
            Self::Three => Box::new(RdIdsAndCrIntoPgJsonOptVecWhDimThreeEqSc),
            Self::Four => Box::new(RdIdsAndCrIntoPgJsonOptVecWhDimFourEqSc),
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, Optml)]
pub enum DimIndexNbr {
    Zero,
    One,
    Two,
    Three,
}
impl From<&Dim> for DimIndexNbr {
    fn from(v: &Dim) -> Self {
        match &v {
            Dim::One => Self::Zero,
            Dim::Two => Self::One,
            Dim::Three => Self::Two,
            Dim::Four => Self::Three,
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum CrQpValueUndrscr {
    False,
    True,
}
impl ToTokens for CrQpValueUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => VSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum CrQpIncrUndrscr {
    False,
    True,
}
impl ToTokens for CrQpIncrUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => IncrSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum CrQbValueUndrscr {
    False,
    True,
}
impl ToTokens for CrQbValueUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => VSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum SelQpValueUndrscr {
    False,
    True,
}
impl ToTokens for SelQpValueUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => VSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum UpdQpValueUndrscr {
    False,
    True,
}
impl ToTokens for UpdQpValueUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => VSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum UpdQpJsonbSetAccumulatorUndrscr {
    False,
    True,
}
impl ToTokens for UpdQpJsonbSetAccumulatorUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {jsonb_set_accumulator}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum UpdQpJsonbSetTargetUndrscr {
    False,
    True,
}
impl ToTokens for UpdQpJsonbSetTargetUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {jsonb_set_target}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, Optml)]
pub enum UpdQpJsonbSetPathUndrscr {
    False,
    True,
}
impl ToTokens for UpdQpJsonbSetPathUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {jsonb_set_path}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
pub fn gen_pg_type_wh_ts(
    attrs_ts: &dyn ToTokens,
    vrts: &Vec<&dyn PgFilter>,
    prefix: &dyn ToTokens,
    should_derive_utoipa_to_schema: &ShouldDeriveUtoipaToSchema,
    should_derive_schemars_json_schema: &ShouldDSchemarsJsonSchema,
    is_qb_mut: &IsQbMut,
) -> Ts2 {
    let ident = SelfWhUcc::from_tokens(&prefix);
    let pg_type_tokens_wh_ts = {
        let vrts_ts = vrts.iter().map(|el| {
            let el_ucc = el.ucc();
            let prefix_wh_self_ucc = el.prefix_wh_self_ucc();
            let opt_type_ts: Option<Ts2> = el.mb_generic();
            let type_ts = opt_type_ts.map_or_else(Ts2::new, |v| quote! {<#v>});
            quote! {#el_ucc(wh_filters::#prefix_wh_self_ucc #type_ts)}
        });
        quote! {
            #attrs_ts
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_derive_utoipa_to_schema #should_derive_schemars_json_schema, optml::Optml)]
            pub enum #ident {
                #(#vrts_ts),*
            }
        }
    };
    let impl_pg_type_pg_type_wh_filter_for_pg_type_tokens_wh_ts =
        impl_pg_type_wh_filter_for_ident_ts(
            &quote! {<'lt>},
            &ident,
            &Ts2::new(),
            &IncrPrmUndrscr::False,
            &ColumnPrmUndrscr::False,
            &AddOprtrUndrscr::False,
            &{
                let vrts_ts = vrts.iter().map(|el| {
                    let el_ucc = el.ucc();
                    quote! {
                        Self::#el_ucc(#VSc) => pg_crud_cmn::PgTypeWhFilter::qp(
                            #VSc,
                            #IncrSc,
                            #ColumnSc,
                            #AddOprtrSc,
                        )
                    }
                });
                quote! {
                    match &self {
                        #(#vrts_ts),*
                    }
                }
            },
            is_qb_mut,
            &{
                let vrts_ts = vrts.iter().map(|el| {
                    let el_ucc = el.ucc();
                    quote! {
                        Self::#el_ucc(#VSc) => pg_crud_cmn::PgTypeWhFilter::qb(
                            #VSc,
                            #QuerySc
                        )
                    }
                });
                quote! {
                    match self {
                        #(#vrts_ts),*
                    }
                }
            },
            &Import::PgCrudCmn,
        );
    let impl_location_lib_to_err_string_for_pg_type_tokens_wh_ts = gen_impl_to_err_string_ts(
        &Ts2::new(),
        &ident,
        &Ts2::new(),
        &quote! {format!("{self:#?}")},
    );
    let impl_all_vrts_dflt_some_one_el_for_pg_type_tokens_wh_ts =
        gen_impl_pg_crud_cmn_all_vrts_dflt_some_one_el_ts(&ident, &{
            let vrts_ts = vrts.iter().map(|el| {
                let el_ucc = el.ucc();
                quote! {Self::#el_ucc(#PgCrudCmnDfltSomeOneElCall)}
            });
            quote! {vec![#(#vrts_ts),*]}
        });
    quote! {
        #pg_type_tokens_wh_ts
        #impl_pg_type_pg_type_wh_filter_for_pg_type_tokens_wh_ts
        #impl_location_lib_to_err_string_for_pg_type_tokens_wh_ts
        #impl_all_vrts_dflt_some_one_el_for_pg_type_tokens_wh_ts
    }
}
#[must_use]
pub fn pg_crud_cmn_qp_er_ts() -> Ts2 {
    quote! {pg_crud_cmn::#QpErUcc}
}
pub fn gen_struct_ident_dq_ts(v: &dyn Display) -> Ts2 {
    dq_ts(&format!("struct {v}"))
}
pub fn gen_struct_ident_with_nbr_els_dq_ts(ident: &dyn DisplayPlusToTokens, len: usize) -> Ts2 {
    dq_ts(&format!("struct {ident} with {len} els"))
}
pub fn gen_sqlx_types_json_type_dcl_ts(type_ts: &dyn ToTokens) -> Ts2 {
    quote! {sqlx::types::Json<#type_ts>}
}
pub fn gen_opt_type_dcl_ts(type_ts: &dyn ToTokens) -> Ts2 {
    quote! {Option<#type_ts>}
}
pub fn gen_vec_tokens_dcl_ts(type_ts: &dyn ToTokens) -> Ts2 {
    quote! {Vec<#type_ts>}
}
pub fn gen_de_dq_ts(ident: &dyn DisplayPlusToTokens, len: usize) -> (Ts2, Ts2, Ts2) {
    let struct_pg_type_ident_wh_tokens_dq_ts = gen_struct_ident_dq_ts(ident);
    let struct_pg_type_ident_wh_tokens_with_nbr_els_dq_ts =
        gen_struct_ident_with_nbr_els_dq_ts(ident, len);
    let pg_type_ident_wh_tokens_dq_ts = dq_ts(&ident);
    (
        struct_pg_type_ident_wh_tokens_dq_ts,
        struct_pg_type_ident_wh_tokens_with_nbr_els_dq_ts,
        pg_type_ident_wh_tokens_dq_ts,
    )
}
pub fn gen_impl_pg_json_ts(
    import: &Import,
    ident: &dyn ToTokens,
    tt_type_ts: &dyn ToTokens,
    cr_type_ts: &dyn ToTokens,
    cr_for_query_type_ts: &dyn ToTokens,
    sel_type_ts: &dyn ToTokens,
    is_sel_qp_self_sel_used: &IsSelQpSelfSelUsed,
    is_sel_qp_column_field_for_er_msg_used: &IsSelQpColumnFieldForErMsgUsed,
    is_sel_qp_is_pg_type_used: &IsSelQpIsPgTypeUsed,
    sel_qp_ts: &dyn ToTokens,
    wh_type_ts: &dyn ToTokens,
    rd_type_ts: &dyn ToTokens,
    rd_ids_type_ts: &dyn ToTokens,
    sel_only_ids_qp_ts: &dyn ToTokens,
    rd_inn_type_ts: &dyn ToTokens,
    into_inn_ts: &dyn ToTokens,
    upd_type_ts: &dyn ToTokens,
    upd_type_for_query_ts: &dyn ToTokens,
    upd_qp_ts: &dyn ToTokens,
    is_upd_qp_self_upd_used: &IsUpdQpSelfUpdUsed,
    is_upd_qp_jsonb_set_target_used: &IsUpdQpJsonbSetTargetUsed,
    is_upd_qb_mut: &IsUpdQbMut,
    upd_qb_ts: &dyn ToTokens,
    sel_only_updd_ids_qp_ts: &dyn ToTokens,
    is_sel_only_updd_ids_qb_mut: &IsSelOnlyUpddIdsQbMut,
    sel_only_updd_ids_qb_ts: &dyn ToTokens,
    sel_only_crd_ids_qp_ts: &dyn ToTokens,
    is_sel_only_crd_ids_qb_mut: &IsSelOnlyCrdIdsQbMut,
    sel_only_crd_ids_qb_ts: &dyn ToTokens,
) -> Ts2 {
    let path_ts = quote! {#import ::};
    let reference_mut_u64_ts = quote! {&mut #U64};
    let query_pg_args_ts =
        quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let query_lt_pg_args_ts =
        quote! {sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>};
    //todo mb reexport sqlx?
    quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #path_ts #PgJsonUcc for #ident {
            type #TtUcc = #tt_type_ts;
            type #CrUcc = #cr_type_ts;
            type #CrForQueryUcc = #cr_for_query_type_ts;
            type #SelUcc = #sel_type_ts;
            fn #SelQpSc(
                #is_sel_qp_self_sel_used: &Self::#SelUcc,
                #FiSc: #RefStr,
                #ColumnFieldSc: #RefStr,
                #is_sel_qp_column_field_for_er_msg_used: #RefStr,
                #is_sel_qp_is_pg_type_used: #Bool,
            ) -> Result<#StringTs, #path_ts #QpErUcc> {
                #sel_qp_ts
            }
            type #WhUcc = #wh_type_ts;
            type #RdUcc = #rd_type_ts;
            type #RdIdsUcc = #rd_ids_type_ts;
            fn #SelOnlyIdsQpSc(
                #ColumnFieldSc: #RefStr,
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #sel_only_ids_qp_ts
            }
            type #RdInnUcc = #rd_inn_type_ts;
            fn into_inn(#VSc: Self::#RdUcc) -> Self::#RdInnUcc {
                #into_inn_ts
            }
            type #UpdUcc = #upd_type_ts;
            type #UpdForQueryUcc = #upd_type_for_query_ts;
            fn #UpdQpSc(
                #is_upd_qp_self_upd_used: &Self::#UpdForQueryUcc,
                #JsonbSetAccumulatorSc: #RefStr,
                #is_upd_qp_jsonb_set_target_used: #RefStr,
                #JsonbSetPathSc: #RefStr,
                #IncrSc: #reference_mut_u64_ts,
            ) -> Result<#StringTs, #path_ts #QpErUcc> {
                #upd_qp_ts
            }
            fn #UpdQbSc(
                #VSc: Self::#UpdForQueryUcc,
                #is_upd_qb_mut #QuerySc: #query_pg_args_ts
            ) -> Result<#query_pg_args_ts, #StringTs> {
                #upd_qb_ts
            }
            fn #SelOnlyUpddIdsQpSc(
                #VSc: &Self::#UpdForQueryUcc,
                #FiSc: #RefStr,
                #ColumnFieldSc: #RefStr,
                #IncrSc: &mut #U64
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #sel_only_updd_ids_qp_ts
            }
            fn #SelOnlyUpddIdsQbSc<'lt>(
                #VSc: &'lt Self::#UpdForQueryUcc,
                #is_sel_only_updd_ids_qb_mut #QuerySc: #query_lt_pg_args_ts
            ) -> Result<#query_lt_pg_args_ts, #StringTs> {
                #sel_only_updd_ids_qb_ts
            }
            fn #SelOnlyCrdIdsQpSc(
                #VSc: &Self::#CrForQueryUcc,
                #FiSc: #RefStr,
                #ColumnFieldSc: #RefStr,
                #IncrSc: &mut #U64
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #sel_only_crd_ids_qp_ts
            }
            fn #SelOnlyCrdIdsQbSc<'lt>(
                #VSc: &'lt Self::#CrForQueryUcc,
                #is_sel_only_crd_ids_qb_mut #QuerySc: #query_lt_pg_args_ts
            ) -> Result<#query_lt_pg_args_ts, #StringTs> {
                #sel_only_crd_ids_qb_ts
            }
        }
    }
}
pub fn gen_impl_dflt_some_one_el_ts(
    impl_generic_ts: &dyn ToTokens,
    import: &Import,
    ident: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import.dflt_some_one_el();
    quote! {
        impl #impl_generic_ts #path_trait_ts for #ident #ident_generic_ts {
            fn #DfltSomeOneElSc() -> Self {
                #ts
            }
        }
    }
}
pub fn gen_impl_all_vrts_dflt_some_one_el_ts(
    import: &Import,
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import.all_vrts_dflt_some_one_el();
    quote! {
        impl #path_trait_ts for #ident {
            fn #AllVrtsDfltSomeOneElSc() -> Vec<Self> {
                #ts
            }
        }
    }
}
pub fn gen_impl_dflt_some_one_el_max_page_size_ts(
    impl_generic_ts: &dyn ToTokens,
    import: &Import,
    ident: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import.dflt_some_one_el_max_page_size();
    quote! {
        impl #impl_generic_ts #path_trait_ts for #ident #ident_generic_ts {
            fn #DfltSomeOneElMaxPageSizeSc() -> Self {
                #ts
            }
        }
    }
}
pub fn gen_impl_all_vrts_dflt_some_one_el_max_page_size_ts(
    import: &Import,
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import.all_vrts_dflt_some_one_el_max_page_size();
    let all_vrts_dflt_some_one_el_max_page_size_sc = AllVrtsDfltSomeOneElMaxPageSizeSc;
    quote! {
        impl #path_trait_ts for #ident {
            fn #all_vrts_dflt_some_one_el_max_page_size_sc() -> Vec<Self> {
                #ts
            }
        }
    }
}
pub fn gen_impl_pg_crud_cmn_dflt_some_one_el_ts(ident: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    gen_impl_dflt_some_one_el_ts(&Ts2::new(), &Import::PgCrudCmn, ident, &Ts2::new(), ts)
}
pub fn gen_impl_pg_crud_dflt_some_one_el_ts(
    ident: &dyn ToTokens,
    lt_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_dflt_some_one_el_ts(&Ts2::new(), &Import::PgCrud, ident, lt_ts, ts)
}
pub fn gen_impl_pg_crud_cmn_all_vrts_dflt_some_one_el_ts(
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_all_vrts_dflt_some_one_el_ts(&Import::PgCrudCmn, ident, ts)
}
pub fn gen_impl_pg_crud_all_vrts_dflt_some_one_el_ts(
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_all_vrts_dflt_some_one_el_ts(&Import::PgCrud, ident, ts)
}
pub fn gen_impl_pg_crud_cmn_dflt_some_one_el_max_page_size_ts(
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_dflt_some_one_el_max_page_size_ts(
        &Ts2::new(),
        &Import::PgCrudCmn,
        ident,
        &Ts2::new(),
        ts,
    )
}
pub fn gen_impl_pg_crud_dflt_some_one_el_max_page_size_ts(
    ident: &dyn ToTokens,
    lt_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_dflt_some_one_el_max_page_size_ts(&Ts2::new(), &Import::PgCrud, ident, lt_ts, ts)
}
pub fn gen_impl_pg_crud_all_vrts_dflt_some_one_el_max_page_size_ts(
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_all_vrts_dflt_some_one_el_max_page_size_ts(&Import::PgCrud, ident, ts)
}
pub fn impl_pg_type_wh_filter_for_ident_ts(
    impl_generic_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    incr_prm_undrscr: &IncrPrmUndrscr,
    column_prm_undrscr: &ColumnPrmUndrscr,
    add_oprtr_undrscr: &AddOprtrUndrscr,
    qp_ts: &dyn ToTokens,
    is_qb_mut: &IsQbMut,
    qb_ts: &dyn ToTokens,
    import: &Import,
) -> Ts2 {
    quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #impl_generic_ts #import ::#PgTypeWhFilterUcc<'lt> for #ident_ts #ident_generic_ts {
            fn #QpSc(
                &self,
                #incr_prm_undrscr: &mut #U64,
                #column_prm_undrscr: &dyn #StdFmtDisplay,
                #add_oprtr_undrscr: #Bool
            ) -> Result<#StringTs, #import::#QpErUcc> {
                #qp_ts
            }
            fn #QbSc(self, #is_qb_mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
                sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
                String
            > {
                #qb_ts
            }
        }
    }
}
pub fn gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(
    ident_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #ident_ts {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#ts, buf)
            }
        }
    }
}
pub fn gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
    ident_ts: &dyn ToTokens,
    type_ts: &dyn ToTokens,
    ok_v_match_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #ident_ts {
            fn decode(#ValueSc: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#type_ts as sqlx::Decode<sqlx::Postgres>>::decode(#ValueSc) {
                    Ok(v) => #ok_v_match_ts,
                    Err(er) => Err(er),
                }
            }
        }
    }
}
pub fn gen_impl_sqlx_type_for_ident_ts(ident_ts: &dyn ToTokens, type_ts: &dyn ToTokens) -> Ts2 {
    quote! {
        impl sqlx::Type<sqlx::Postgres> for #ident_ts {
            fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
                <#type_ts as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
               <#type_ts as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }
    }
}
pub fn gen_impl_pg_type_ts(
    import: &Import,
    ident: &dyn ToTokens,
    ident_tt_ucc: &dyn ToTokens,
    is_pk_undrscr: &IsPkUndrscr,
    cr_tbl_column_qp_ts: &dyn ToTokens,
    ident_cr_ucc: &dyn ToTokens,
    cr_qp_v_undrscr: &CrQpValueUndrscr,
    cr_qp_incr_undrscr: &CrQpIncrUndrscr,
    cr_qp_ts: &dyn ToTokens,
    cr_qb_v_undrscr: &CrQbValueUndrscr,
    is_cr_qb_mut: &IsCrQbMut,
    cr_qb_ts: &dyn ToTokens,
    ident_sel_ucc: &dyn ToTokens,
    sel_qp_v_undrscr: &SelQpValueUndrscr,
    sel_qp_ts: &dyn ToTokens,
    ident_wh_ucc: &dyn ToTokens,
    ident_rd_ucc: &dyn ToTokens,
    normalize_ts: &dyn ToTokens,
    rd_ids_ts: &dyn ToTokens,
    sel_only_ids_qp_ts: &dyn ToTokens,
    ident_rd_inn_ucc: &dyn ToTokens,
    into_inn_ts: &dyn ToTokens,
    ident_upd_ucc: &dyn ToTokens,
    ident_upd_for_query_ucc: &dyn ToTokens,
    upd_qp_v_undrscr: &UpdQpValueUndrscr,
    upd_qp_jsonb_set_accumulator_undrscr: &UpdQpJsonbSetAccumulatorUndrscr,
    upd_qp_jsonb_set_target_undrscr: &UpdQpJsonbSetTargetUndrscr,
    upd_qp_jsonb_set_path_undrscr: &UpdQpJsonbSetPathUndrscr,
    upd_qp_ts: &dyn ToTokens,
    is_upd_qb_mut: &IsUpdQbMut,
    upd_qb_ts: &dyn ToTokens,
    sel_only_updd_ids_qp_ts: &dyn ToTokens,
    is_sel_only_updd_ids_qb_mut: &IsSelOnlyUpddIdsQbMut,
    sel_only_updd_ids_qb_ts: &dyn ToTokens,
) -> Ts2 {
    let query_pg_args_ts =
        quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #import :: #PgTypeUcc for #ident {
            type #TtUcc = #ident_tt_ucc;
            fn #CrTblColumnQpSc(#ColumnSc: &dyn #StdFmtDisplay, #is_pk_undrscr: #Bool) -> impl #StdFmtDisplay {
                #cr_tbl_column_qp_ts
            }
            type #CrUcc = #ident_cr_ucc;
            fn #CrQpSc(
                #cr_qp_v_undrscr: &Self::#CrUcc,
                #cr_qp_incr_undrscr: &mut #U64
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #cr_qp_ts
            }
            fn #CrQbSc(
                #cr_qb_v_undrscr: Self::#CrUcc,
                #is_cr_qb_mut #QuerySc: #query_pg_args_ts
            ) -> Result<
                #query_pg_args_ts,
                String
            > {
                #cr_qb_ts
            }
            type #SelUcc = #ident_sel_ucc;
            fn #SelQpSc(
                #sel_qp_v_undrscr: &Self::#SelUcc,
                #ColumnSc: #RefStr,
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #sel_qp_ts
            }
            type #WhUcc = #ident_wh_ucc;
            type #RdUcc = #ident_rd_ucc;
            fn #NormalizeSc(#VSc: Self::#RdUcc) -> Self::#RdUcc {
                #normalize_ts
            }
            type #RdIdsUcc = #rd_ids_ts;
            fn #SelOnlyIdsQpSc(
                #ColumnSc: #RefStr
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #sel_only_ids_qp_ts
            }
            type #RdInnUcc = #ident_rd_inn_ucc;
            fn into_inn(#VSc: Self::#RdUcc) -> Self::#RdInnUcc {
                #into_inn_ts
            }
            type #UpdUcc = #ident_upd_ucc;
            type #UpdForQueryUcc = #ident_upd_for_query_ucc;
            fn #UpdQpSc(
                #upd_qp_v_undrscr: &Self::#UpdForQueryUcc,
                #upd_qp_jsonb_set_accumulator_undrscr: #RefStr,
                #upd_qp_jsonb_set_target_undrscr: #RefStr,
                #upd_qp_jsonb_set_path_undrscr: #RefStr,
                #IncrSc: &mut #U64
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #upd_qp_ts
            }
            fn #UpdQbSc(
                #VSc: Self::#UpdForQueryUcc,
                #is_upd_qb_mut #QuerySc: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<
                sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                String
            > {
                #upd_qb_ts
            }
            fn #SelOnlyUpddIdsQpSc(
                #VSc: &Self::#UpdForQueryUcc,
                #ColumnSc: #RefStr,
                #IncrSc: &mut #U64,
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #sel_only_updd_ids_qp_ts
            }
            fn #SelOnlyUpddIdsQbSc<'lt>(
                #VSc: &'lt Self::#UpdForQueryUcc,
                #is_sel_only_updd_ids_qb_mut #QuerySc: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
                #sel_only_updd_ids_qb_ts
            }
        }
    }
}
pub fn gen_impl_pg_type_not_pk_for_ident_ts(import: &Import, ident: &dyn ToTokens) -> Ts2 {
    let ident_cr_ucc = SelfCrUcc::from_tokens(&ident);
    quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #import::#PgTypeNotPkUcc for #ident {
            type #PgTypeUcc = Self;
            type #CrUcc = #ident_cr_ucc;
        }
    }
}
// fn gen_rd_ids_and_cr_into_wh_method_ts(
//     import: &Import,
//     method_name_ts: &dyn ToTokens,
//     ts: &dyn ToTokens,
//     pg_type_or_pg_json: &PgTypeOrPgJson,
// ) -> Ts2 {
//     let self_ucc = SelfUcc;
//     let rd_ids_sc = RdIdsSc;
//     let rd_ids_ucc = RdIdsUcc;
//     let cr_sc = CrSc;
//     let cr_ucc = CrUcc;
//     let wh_ucc = WhUcc;
//     let self_pg_type_or_pg_json_as_pg_json_ts = {
//         let pg_type_or_pg_json_ts: &dyn ToTokens = match &pg_type_or_pg_json {
//             PgTypeOrPgJson::PgType => &PgTypeUcc,
//             PgTypeOrPgJson::PgJson => &PgJsonUcc,
//         };
//         quote! {
//             <#SelfUcc::#pg_type_or_pg_json_ts as #import::#pg_type_or_pg_json_ts>
//         }
//     };
//     quote!{
//         fn #method_name_ts(
//             #RdIdsSc: #self_pg_type_or_pg_json_as_pg_json_ts::#RdIdsUcc,
//             #CrSc: #self_pg_type_or_pg_json_as_pg_json_ts::#CrUcc
//         ) -> Vec<#self_pg_type_or_pg_json_as_pg_json_ts::#WhUcc> {
//             #ts
//         }
//     }
// }
fn gen_opt_vec_cr_ts(path_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #OptVecCrSc() -> Option<Vec<#path_ts::#CrUcc>> {
            #ts
        }
    }
}
fn gen_rd_ids_to_2_dims_vec_rd_inn_ts(path_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #RdIdsTo2DimsVecRdInnSc(
            #RdIdsSc: &#path_ts::#RdIdsUcc
        ) -> Vec<Vec<#path_ts::#RdInnUcc>> {
            #ts
        }
    }
}
fn gen_rd_inn_into_rd_with_new_or_try_new_unwraped_ts(
    type_ts: &dyn ToTokens,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #RdInnIntoRdWithNewOrTryNewUnwrapedSc(
            #VSc: #type_ts
        ) -> #path_ts::#RdUcc {
            #ts
        }
    }
}
fn gen_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(
    type_ts: &dyn ToTokens,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #RdInnIntoUpdWithNewOrTryNewUnwrapedSc(#VSc: #type_ts) -> #path_ts::#UpdUcc {
            #ts
        }
    }
}
fn gen_upd_to_rd_ids_ts(path_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #UpdToRdIdsSc(
            #VSc: &#path_ts::#UpdUcc
        ) -> #path_ts::#RdIdsUcc {
            #ts
        }
    }
}
fn gen_rd_ids_to_opt_v_rd_dflt_some_one_el_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #RdIdsToOptVRdDfltSomeOneElSc(
            #VSc: &#path_ts::#RdIdsUcc
        ) -> Option<#import::#VUcc<#path_ts::#RdUcc>> {
            #ts
        }
    }
}
fn gen_previous_rd_and_opt_upd_into_rd_ts(path_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #PreviousRdAndOptUpdIntoRdSc(
            #RdSc: #path_ts::#RdUcc,
            #OptUpdSc: Option<#path_ts::#UpdUcc>,
        ) -> #path_ts::#RdUcc {
            #ts
        }
    }
}
fn gen_rd_ids_and_cr_into_rd_ts(path_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #RdIdsAndCrIntoRdSc(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> #path_ts::#RdUcc {
            #ts
        }
    }
}
fn gen_rd_ids_and_cr_into_opt_v_rd_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #RdIdsAndCrIntoOptVRdSc(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> Option<#import::#VUcc<#path_ts::#RdUcc>> {
            #ts
        }
    }
}
fn gen_rd_ids_and_cr_into_tt_ts(path_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #RdIdsAndCrIntoTtSc(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> #path_ts::#TtUcc {
            #ts
        }
    }
}
pub fn gen_rd_ids_and_cr_into_wh_eq_ts(
    rd_ids_ts: &dyn ToTokens,
    cr_ts: &dyn ToTokens,
    wh_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #RdIdsAndCrIntoWhEqSc(
            #RdIdsSc: #rd_ids_ts,
            #CrSc: #cr_ts
        ) -> #wh_ts {
            #ts
        }
    }
}
pub fn gen_rd_ids_and_cr_into_vec_wh_eq_using_fields_ts(
    import: &Import,
    rd_ids_ts: &dyn ToTokens,
    cr_ts: &dyn ToTokens,
    wh_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #RdIdsAndCrIntoVecWhEqUsingFieldsSc(
            #RdIdsSc: #rd_ids_ts,
            #CrSc: #cr_ts
        ) -> #import::NotEmptyUnqVec<#wh_ts> {
            #ts
        }
    }
}
fn gen_rd_ids_and_cr_into_vec_or_opt_vec_wh_eq_to_json_field_pg_type_or_pg_json_ts(
    import: Import,
    rd_ids_ts: &dyn ToTokens,
    cr_ts: &dyn ToTokens,
    wh_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
    pg_type_or_pg_json: PgTypeOrPgJson,
) -> Ts2 {
    let return_type_ts = {
        let return_type_h_ts = quote! {#import::NotEmptyUnqVec<#wh_ts>};
        match &pg_type_or_pg_json {
            PgTypeOrPgJson::PgType => gen_opt_type_dcl_ts(&return_type_h_ts),
            PgTypeOrPgJson::PgJson => return_type_h_ts,
        }
    };
    let name_ts: &dyn ToTokens = match &pg_type_or_pg_json {
        PgTypeOrPgJson::PgType => &RdIdsAndCrIntoOptVecWhEqToJsonFieldSc,
        PgTypeOrPgJson::PgJson => &RdIdsAndCrIntoVecWhEqToJsonFieldSc,
    };
    quote! {
        fn #name_ts(
            #RdIdsSc: #rd_ids_ts,
            #CrSc: #cr_ts
        ) -> #return_type_ts {
            #ts
        }
    }
}
pub fn gen_rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts(
    import: Import,
    rd_ids_ts: &dyn ToTokens,
    cr_ts: &dyn ToTokens,
    wh_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_vec_or_opt_vec_wh_eq_to_json_field_pg_type_or_pg_json_ts(
        import,
        &rd_ids_ts,
        &cr_ts,
        &wh_ts,
        &ts,
        PgTypeOrPgJson::PgJson,
    )
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_nbr_eq_ts(
    import: Import,
    name_ts: &dyn ToTokens,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #name_ts(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> Option<#import::NotEmptyUnqVec<#path_ts::#WhUcc>> {
            #ts
        }
    }
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_nbr_eq_ts(
        import,
        &RdIdsAndCrIntoPgJsonOptVecWhDimOneEqSc,
        &path_ts,
        &ts,
    )
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_nbr_eq_ts(
        import,
        &RdIdsAndCrIntoPgJsonOptVecWhDimTwoEqSc,
        &path_ts,
        &ts,
    )
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_nbr_eq_ts(
        import,
        &RdIdsAndCrIntoPgJsonOptVecWhDimThreeEqSc,
        &path_ts,
        &ts,
    )
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_nbr_eq_ts(
        import,
        &RdIdsAndCrIntoPgJsonOptVecWhDimFourEqSc,
        &path_ts,
        &ts,
    )
}
fn gen_cr_into_pg_json_opt_vec_wh_len_eq_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #CrIntoPgJsonOptVecWhLenEqSc(
            #CrSc: #path_ts::#CrUcc
        ) -> Option<#import::NotEmptyUnqVec<#path_ts::#WhUcc>> {
            #ts
        }
    }
}
fn gen_cr_into_pg_json_opt_vec_wh_len_greater_than_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #CrIntoPgJsonOptVecWhLenGreaterThanSc(
            #CrSc: #path_ts::#CrUcc
        ) -> Option<#import::NotEmptyUnqVec<#path_ts::#WhUcc>> {
            #ts
        }
    }
}
fn gen_rd_ids_and_cr_into_pg_json_opt_not_empty_unq_vec_single_or_multiple_wh_ts(
    method_name_ts: &dyn ToTokens,
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #method_name_ts(
            #RdIdsSc: #path_ts::#RdIdsUcc,
            #CrSc: #path_ts::#CrUcc
        ) -> Option<#import::NotEmptyUnqVec<#import::SingleOrMultiple<#path_ts::#WhUcc>>> {
            #ts
        }
    }
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_not_empty_unq_vec_single_or_multiple_wh_ts(
        &RdIdsAndCrIntoPgJsonOptVecWhGreaterThanSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_not_empty_unq_vec_single_or_multiple_wh_ts(
        &RdIdsAndCrIntoPgJsonOptVecWhBtwnSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_not_empty_unq_vec_single_or_multiple_wh_ts(
        &RdIdsAndCrIntoPgJsonOptVecWhInSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_not_empty_unq_vec_single_or_multiple_wh_ts(
        &RdIdsAndCrIntoPgJsonOptVecWhRgxSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_not_empty_unq_vec_single_or_multiple_wh_ts(
        &RdIdsAndCrIntoPgJsonOptVecWhContainsElGreaterThanSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_rd_ids_and_cr_into_pg_json_opt_not_empty_unq_vec_single_or_multiple_wh_ts(
        &RdIdsAndCrIntoPgJsonOptVecWhContainsElRgxSc,
        import,
        path_ts,
        ts,
    )
}
pub fn gen_impl_pg_type_test_cases_for_ident_ts(
    cfg_ts: &dyn ToTokens,
    import: &Import,
    type_ts: &dyn ToTokens,
    ident: &dyn ToTokens,
    opt_vec_cr_ts: &dyn ToTokens,
    rd_ids_to_2_dims_vec_rd_inn_ts: &dyn ToTokens,
    rd_inn_into_rd_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    rd_inn_into_upd_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    upd_to_rd_ids_ts: &dyn ToTokens,
    rd_ids_to_opt_v_rd_dflt_some_one_el_ts: &dyn ToTokens,
    previous_rd_and_opt_upd_into_rd_ts: &dyn ToTokens,
    rd_ids_and_cr_into_rd_ts: &dyn ToTokens,
    rd_ids_and_cr_into_opt_v_rd_ts: &dyn ToTokens,
    rd_ids_and_cr_into_tt_ts: &dyn ToTokens,
    rd_ids_and_cr_into_wh_eq_ts: &dyn ToTokens,
    rd_ids_and_cr_into_vec_wh_eq_using_fields_ts: &dyn ToTokens,
    rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts: &dyn ToTokens,
    cr_into_pg_type_opt_vec_wh_dim_one_eq_ts: &dyn ToTokens,
    pg_type_opt_vec_wh_greater_than_test_ts: &dyn ToTokens,
    rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_dim_one_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_dim_two_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_dim_three_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_dim_four_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_len_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_len_greater_than_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts: &dyn ToTokens,
) -> Ts2 {
    let self_pg_type_as_pg_type_ts = quote! {<#SelfUcc::#PgTypeUcc as #import::#PgTypeUcc>};
    let self_pg_type_as_pg_type_rd_ids_ts = quote! {#self_pg_type_as_pg_type_ts::#RdIdsUcc};
    let self_pg_type_as_pg_type_cr_ts = quote! {#self_pg_type_as_pg_type_ts::#CrUcc};
    let self_pg_type_as_pg_type_wh_ts = quote! {#self_pg_type_as_pg_type_ts::#WhUcc};
    let ident_sel_ucc = SelfSelUcc::from_tokens(&ident);
    let opt_vec_cr_ts_2d58042f = gen_opt_vec_cr_ts(&self_pg_type_as_pg_type_ts, &opt_vec_cr_ts);
    let rd_ids_to_2_dims_vec_rd_inn_ts_513b8046 = gen_rd_ids_to_2_dims_vec_rd_inn_ts(
        &self_pg_type_as_pg_type_ts,
        &rd_ids_to_2_dims_vec_rd_inn_ts,
    );
    let rd_inn_into_rd_with_new_or_try_new_unwraped_ts_affc58f5 =
        gen_rd_inn_into_rd_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &rd_inn_into_rd_with_new_or_try_new_unwraped_ts,
        );
    let rd_inn_into_upd_with_new_or_try_new_unwraped_ts_c38e6621 =
        gen_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &rd_inn_into_upd_with_new_or_try_new_unwraped_ts,
        );
    let upd_to_rd_ids_ts_ee17b828 =
        gen_upd_to_rd_ids_ts(&self_pg_type_as_pg_type_ts, &upd_to_rd_ids_ts);
    let rd_ids_to_opt_v_rd_dflt_some_one_el_ts_18ef45e8 =
        gen_rd_ids_to_opt_v_rd_dflt_some_one_el_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &rd_ids_to_opt_v_rd_dflt_some_one_el_ts,
        );
    let previous_rd_and_opt_upd_into_rd_ts_c48b8ede = gen_previous_rd_and_opt_upd_into_rd_ts(
        &self_pg_type_as_pg_type_ts,
        &previous_rd_and_opt_upd_into_rd_ts,
    );
    let rd_ids_and_cr_into_rd_ts_df48e4b7 =
        gen_rd_ids_and_cr_into_rd_ts(&self_pg_type_as_pg_type_ts, &rd_ids_and_cr_into_rd_ts);
    let rd_ids_and_cr_into_opt_v_rd_ts_8b7e9688 = gen_rd_ids_and_cr_into_opt_v_rd_ts(
        *import,
        &self_pg_type_as_pg_type_ts,
        &rd_ids_and_cr_into_opt_v_rd_ts,
    );
    let rd_ids_and_cr_into_tt_ts_f227db63 =
        gen_rd_ids_and_cr_into_tt_ts(&self_pg_type_as_pg_type_ts, &rd_ids_and_cr_into_tt_ts);
    let rd_ids_and_cr_into_wh_eq_ts_dcde170f = gen_rd_ids_and_cr_into_wh_eq_ts(
        &self_pg_type_as_pg_type_rd_ids_ts,
        &self_pg_type_as_pg_type_cr_ts,
        &self_pg_type_as_pg_type_wh_ts,
        &rd_ids_and_cr_into_wh_eq_ts,
    );
    let rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_076c6ebd =
        gen_rd_ids_and_cr_into_vec_wh_eq_using_fields_ts(
            import,
            &self_pg_type_as_pg_type_rd_ids_ts,
            &self_pg_type_as_pg_type_cr_ts,
            &self_pg_type_as_pg_type_wh_ts,
            &rd_ids_and_cr_into_vec_wh_eq_using_fields_ts,
        );
    let rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts_948ce180 =
        gen_rd_ids_and_cr_into_vec_or_opt_vec_wh_eq_to_json_field_pg_type_or_pg_json_ts(
            *import,
            &self_pg_type_as_pg_type_rd_ids_ts,
            &self_pg_type_as_pg_type_cr_ts,
            &self_pg_type_as_pg_type_wh_ts,
            &rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts,
            PgTypeOrPgJson::PgType,
        );
    let cr_into_pg_type_opt_vec_wh_dim_one_eq_sc = CrIntoPgTypeOptVecWhDimOneEqSc;
    let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_sc = RdIdsAndTtIntoPgTypeOptWhGreaterThanSc;
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts_33093313 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &cr_into_pg_json_opt_vec_wh_dim_one_eq_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts_9522c7a5 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &cr_into_pg_json_opt_vec_wh_dim_two_eq_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts_81696b49 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &cr_into_pg_json_opt_vec_wh_dim_three_eq_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts_2631549b =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &cr_into_pg_json_opt_vec_wh_dim_four_eq_ts,
        );
    let cr_into_pg_json_opt_vec_wh_len_eq_ts_34b74d66 = gen_cr_into_pg_json_opt_vec_wh_len_eq_ts(
        *import,
        &self_pg_type_as_pg_type_ts,
        &cr_into_pg_json_opt_vec_wh_len_eq_ts,
    );
    let cr_into_pg_json_opt_vec_wh_len_greater_than_ts_b196c70f =
        gen_cr_into_pg_json_opt_vec_wh_len_greater_than_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &cr_into_pg_json_opt_vec_wh_len_greater_than_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts_498680a8 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts_b685b98f =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts_ac82295e =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts_bfe19de1 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts_8d2a6cb8 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts_ff2d3a76 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts,
        );
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySrcItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import::#PgTypeTestCasesUcc for #ident {
            type #PgTypeUcc = #SelfUcc;
            type #SelUcc = #ident_sel_ucc;
            #opt_vec_cr_ts_2d58042f
            #rd_ids_to_2_dims_vec_rd_inn_ts_513b8046
            #rd_inn_into_rd_with_new_or_try_new_unwraped_ts_affc58f5
            #rd_inn_into_upd_with_new_or_try_new_unwraped_ts_c38e6621
            #upd_to_rd_ids_ts_ee17b828
            #rd_ids_to_opt_v_rd_dflt_some_one_el_ts_18ef45e8
            #previous_rd_and_opt_upd_into_rd_ts_c48b8ede
            #rd_ids_and_cr_into_rd_ts_df48e4b7
            #rd_ids_and_cr_into_opt_v_rd_ts_8b7e9688
            #rd_ids_and_cr_into_tt_ts_f227db63
            #rd_ids_and_cr_into_wh_eq_ts_dcde170f
            #rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_076c6ebd
            #rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts_948ce180
            fn #cr_into_pg_type_opt_vec_wh_dim_one_eq_sc(
                #CrSc: #self_pg_type_as_pg_type_ts::#CrUcc
            ) -> Option<#import::NotEmptyUnqVec<#self_pg_type_as_pg_type_ts::#WhUcc>> {
                #cr_into_pg_type_opt_vec_wh_dim_one_eq_ts
            }
            fn #PgTypeOptVecWhGreaterThanTestSc() -> Option<
                #import::NotEmptyUnqVec<
                    #import::PgTypeGreaterThanTest<
                        #SelfUcc::#PgTypeUcc
                    >
                >
            > {
                #pg_type_opt_vec_wh_greater_than_test_ts
            }
            fn #rd_ids_and_tt_into_pg_type_opt_wh_greater_than_sc(
                greater_than_vrt: #import::PgTypeGreaterThanVrt,
                #RdIdsSc: #self_pg_type_as_pg_type_ts::#RdIdsUcc,
                #TtSc: #self_pg_type_as_pg_type_ts::#TtUcc,
            ) -> Option<#self_pg_type_as_pg_type_ts::#WhUcc> {
                #rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts
            }
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts_33093313
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts_9522c7a5
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts_81696b49
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts_2631549b
            #cr_into_pg_json_opt_vec_wh_len_eq_ts_34b74d66
            #cr_into_pg_json_opt_vec_wh_len_greater_than_ts_b196c70f
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts_498680a8
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts_b685b98f
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts_ac82295e
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts_bfe19de1
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts_8d2a6cb8
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts_ff2d3a76
        }
    }
}
pub fn gen_impl_pg_json_test_cases_for_ident_ts(
    cfg_ts: &dyn ToTokens,
    import: &Import,
    type_ts: &dyn ToTokens,
    ident: &dyn ToTokens,
    opt_vec_cr_ts: &dyn ToTokens,
    rd_ids_to_2_dims_vec_rd_inn_ts: &dyn ToTokens,
    rd_inn_into_rd_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    rd_inn_into_upd_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    rd_ids_into_opt_v_rd_inn_ts: &dyn ToTokens,
    upd_to_rd_ids_ts: &dyn ToTokens,
    rd_ids_to_opt_v_rd_dflt_some_one_el_ts: &dyn ToTokens,
    previous_rd_and_opt_upd_into_rd_ts: &dyn ToTokens,
    rd_ids_and_cr_into_rd_ts: &dyn ToTokens,
    rd_ids_and_cr_into_opt_v_rd_ts: &dyn ToTokens,
    rd_ids_and_cr_into_tt_ts: &dyn ToTokens,
    rd_ids_and_cr_into_wh_eq_ts: &dyn ToTokens,
    rd_ids_and_cr_into_vec_wh_eq_using_fields_ts: &dyn ToTokens,
    rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_dim_one_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_dim_two_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_dim_three_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_dim_four_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_len_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_len_greater_than_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts: &dyn ToTokens,
) -> Ts2 {
    let self_pg_json_as_pg_json_ts = quote! {<#SelfUcc::#PgJsonUcc as #import::#PgJsonUcc>};
    let self_pg_json_as_pg_json_rd_ids_ts = quote! {#self_pg_json_as_pg_json_ts::#RdIdsUcc};
    let self_pg_json_as_pg_json_cr_ts = quote! {#self_pg_json_as_pg_json_ts::#CrUcc};
    let self_pg_json_as_pg_json_wh_ts = quote! {#self_pg_json_as_pg_json_ts::#WhUcc};
    let ident_sel_ucc = SelfSelUcc::from_tokens(&ident);
    let opt_vec_cr_ts_a442630a = gen_opt_vec_cr_ts(&self_pg_json_as_pg_json_ts, &opt_vec_cr_ts);
    let rd_ids_to_2_dims_vec_rd_inn_ts_da1a7cf8 = gen_rd_ids_to_2_dims_vec_rd_inn_ts(
        &self_pg_json_as_pg_json_ts,
        &rd_ids_to_2_dims_vec_rd_inn_ts,
    );
    let rd_inn_into_rd_with_new_or_try_new_unwraped_ts_ccead2b6 =
        gen_rd_inn_into_rd_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_json_as_pg_json_ts,
            &rd_inn_into_rd_with_new_or_try_new_unwraped_ts,
        );
    let rd_inn_into_upd_with_new_or_try_new_unwraped_ts_b45cde72 =
        gen_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_json_as_pg_json_ts,
            &rd_inn_into_upd_with_new_or_try_new_unwraped_ts,
        );
    let upd_to_rd_ids_ts_d7e0cbf0 =
        gen_upd_to_rd_ids_ts(&self_pg_json_as_pg_json_ts, &upd_to_rd_ids_ts);
    let rd_ids_to_opt_v_rd_dflt_some_one_el_ts_f5d1b395 =
        gen_rd_ids_to_opt_v_rd_dflt_some_one_el_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_to_opt_v_rd_dflt_some_one_el_ts,
        );
    let previous_rd_and_opt_upd_into_rd_ts_ab0384b9 = gen_previous_rd_and_opt_upd_into_rd_ts(
        &self_pg_json_as_pg_json_ts,
        &previous_rd_and_opt_upd_into_rd_ts,
    );
    let rd_ids_and_cr_into_rd_ts_7df2fa10 =
        gen_rd_ids_and_cr_into_rd_ts(&self_pg_json_as_pg_json_ts, &rd_ids_and_cr_into_rd_ts);
    let rd_ids_and_cr_into_opt_v_rd_ts_1f54e2bf = gen_rd_ids_and_cr_into_opt_v_rd_ts(
        *import,
        &self_pg_json_as_pg_json_ts,
        &rd_ids_and_cr_into_opt_v_rd_ts,
    );
    let rd_ids_and_cr_into_tt_ts_b605767e =
        gen_rd_ids_and_cr_into_tt_ts(&self_pg_json_as_pg_json_ts, &rd_ids_and_cr_into_tt_ts);
    let rd_ids_and_cr_into_wh_eq_ts_1009eb88 = gen_rd_ids_and_cr_into_wh_eq_ts(
        &self_pg_json_as_pg_json_rd_ids_ts,
        &self_pg_json_as_pg_json_cr_ts,
        &self_pg_json_as_pg_json_wh_ts,
        &rd_ids_and_cr_into_wh_eq_ts,
    );
    let rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_876245c5 =
        gen_rd_ids_and_cr_into_vec_wh_eq_using_fields_ts(
            import,
            &self_pg_json_as_pg_json_rd_ids_ts,
            &self_pg_json_as_pg_json_cr_ts,
            &self_pg_json_as_pg_json_wh_ts,
            &rd_ids_and_cr_into_vec_wh_eq_using_fields_ts,
        );
    let rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts_11560e7f =
        gen_rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts(
            *import,
            &self_pg_json_as_pg_json_rd_ids_ts,
            &self_pg_json_as_pg_json_cr_ts,
            &self_pg_json_as_pg_json_wh_ts,
            &rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts_aaaa85b2 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &cr_into_pg_json_opt_vec_wh_dim_one_eq_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts_6da8ece7 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &cr_into_pg_json_opt_vec_wh_dim_two_eq_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts_6b473c12 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &cr_into_pg_json_opt_vec_wh_dim_three_eq_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts_b427508f =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &cr_into_pg_json_opt_vec_wh_dim_four_eq_ts,
        );
    let cr_into_pg_json_opt_vec_wh_len_eq_ts_5266addf = gen_cr_into_pg_json_opt_vec_wh_len_eq_ts(
        *import,
        &self_pg_json_as_pg_json_ts,
        &cr_into_pg_json_opt_vec_wh_len_eq_ts,
    );
    let cr_into_pg_json_opt_vec_wh_len_greater_than_ts_93196cce =
        gen_cr_into_pg_json_opt_vec_wh_len_greater_than_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &cr_into_pg_json_opt_vec_wh_len_greater_than_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts_e0be3ff7 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts_9bdb444a =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts_09ea1f4b =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts_1b1057eb =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts_5dc0a6c8 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts_972d3e87 =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts,
        );
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySrcItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import::#PgJsonTestCasesUcc for #ident {
            type #PgJsonUcc = #SelfUcc;
            type #SelUcc = #ident_sel_ucc;
            #opt_vec_cr_ts_a442630a
            #rd_ids_to_2_dims_vec_rd_inn_ts_da1a7cf8
            #rd_inn_into_rd_with_new_or_try_new_unwraped_ts_ccead2b6
            #rd_inn_into_upd_with_new_or_try_new_unwraped_ts_b45cde72
            fn #RdIdsIntoOptVRdInnSc(
                #VSc: #self_pg_json_as_pg_json_ts::#RdIdsUcc
            ) -> Option<#import::#VUcc<#self_pg_json_as_pg_json_ts::#RdInnUcc>> {
                #rd_ids_into_opt_v_rd_inn_ts
            }
            #upd_to_rd_ids_ts_d7e0cbf0
            #rd_ids_to_opt_v_rd_dflt_some_one_el_ts_f5d1b395
            #previous_rd_and_opt_upd_into_rd_ts_ab0384b9
            #rd_ids_and_cr_into_rd_ts_7df2fa10
            #rd_ids_and_cr_into_opt_v_rd_ts_1f54e2bf
            #rd_ids_and_cr_into_tt_ts_b605767e
            #rd_ids_and_cr_into_wh_eq_ts_1009eb88
            #rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_876245c5
            #rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts_11560e7f
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts_aaaa85b2
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts_6da8ece7
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts_6b473c12
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts_b427508f
            #cr_into_pg_json_opt_vec_wh_len_eq_ts_5266addf
            #cr_into_pg_json_opt_vec_wh_len_greater_than_ts_93196cce
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts_e0be3ff7
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts_9bdb444a
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts_09ea1f4b
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts_1b1057eb
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts_5dc0a6c8
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts_972d3e87
        }
    }
}
#[must_use]
pub fn pg_crud_cmn_qp_er_checked_add_init_ts() -> Ts2 {
    quote! {pg_crud_cmn::QpEr::CheckedAdd { loc: location_lib::loc!() }}
}
pub fn gen_impl_crate_is_string_empty_for_ident_ts(ident: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        impl pg_crud_cmn::IsStringEmpty for #ident {
            fn is_string_empty(&self) -> bool {
                #ts
            }
        }
    }
}
pub fn gen_match_try_new_in_de_ts(ident: &dyn ToTokens, init_ts: &dyn ToTokens) -> Ts2 {
    quote! {
        match #ident::try_new(#init_ts) {
            Ok(v) => Ok(v),
            Err(er) => Err(serde::de::Error::custom(format!("{er:?}")))
        }
    }
}
pub fn gen_impl_de_for_struct_ts(
    ident: &dyn DisplayPlusToTokens,
    vec_ident_type: &[(&Ident, &Type)],
    len: usize,
    gen_type_ts: &dyn Fn(&Ident, &Type) -> Ts2,
) -> Ts2 {
    fn gen_undrscr_undrscr_field_i_str(i: usize) -> String {
        format!("f{i}")
    }
    fn gen_undrscr_undrscr_field_i_ts(i: usize) -> Ts2 {
        gen_undrscr_undrscr_field_i_str(i)
            .parse::<Ts2>()
            .expect("ff7433a3")
    }
    fn gen_undrscr_undrscr_field_i_h_ts(i: usize) -> Ts2 {
        format!("{}_h", gen_undrscr_undrscr_field_i_str(i))
            .parse::<Ts2>()
            .expect("09a0c518")
    }
    fn gen_fi_dq_serde_private_ok_field_ts(field_name_dq_ts: &dyn ToTokens, i: usize) -> Ts2 {
        let field_i_ts = gen_undrscr_undrscr_field_i_ts(i);
        quote! {#field_name_dq_ts => Ok(__Field::#field_i_ts)}
    }
    let vec_ident = vec_ident_type
        .iter()
        .map(|el| el.0)
        .collect::<Vec<&Ident>>();
    let field_enum_vrts_ts = {
        let field_enum_vrts_ts = (0..len)
            .map(|i| format!("f{i}").parse::<Ts2>().expect("c46314b0"))
            .collect::<Vec<Ts2>>();
        quote! {#(#field_enum_vrts_ts),*}
    };
    let visit_u64_v_enum_vrts_ts = {
        let visit_u64_v_enum_vrts_ts = (0..len).map(|i| {
            let i_u64_ts = format!("{i}u64").parse::<Ts2>().expect("828ff7b4");
            let field_i_ts = gen_undrscr_undrscr_field_i_ts(i);
            quote! {#i_u64_ts => Ok(__Field::#field_i_ts)}
        });
        quote! {#(#visit_u64_v_enum_vrts_ts),*}
    };
    let visit_str_v_enum_vrts_ts = {
        let visit_str_v_enum_vrts_ts = vec_ident.iter().enumerate().map(|(i, el)| {
            let field_name_dq_ts = dq_ts(&el);
            gen_fi_dq_serde_private_ok_field_ts(&field_name_dq_ts, i)
        });
        quote! {#(#visit_str_v_enum_vrts_ts),*,}
    };
    let visit_bytes_v_enum_vrts_ts = {
        let visit_bytes_v_enum_vrts_ts = vec_ident.iter().enumerate().map(|(i, el)| {
            let b_field_name_dq_ts = format!("b{}", dq_str(&el.to_string()))
                .parse::<Ts2>()
                .expect("9e33625e");
            gen_fi_dq_serde_private_ok_field_ts(&b_field_name_dq_ts, i)
        });
        quote! {#(#visit_bytes_v_enum_vrts_ts),*,}
    };
    let struct_ident_dq_ts = gen_struct_ident_dq_ts(&ident);
    let visit_seq_fields_init_ts = {
        let ts = vec_ident_type.iter().enumerate().map(|(i, (el_ident, el_type))| {
            let field_i_h_ts = gen_undrscr_undrscr_field_i_h_ts(i);
            let type_ts = gen_type_ts(el_ident, el_type);
            let struct_ident_opts_with_dq_ts = dq_ts(&format!("struct {ident} with {len} els"));
            quote! {
                let Some(#field_i_h_ts) = serde::de::SeqAccess::next_element::<#type_ts>(&mut __seq)? else {
                    return Err(serde::de::Error::invalid_length(0usize, &#struct_ident_opts_with_dq_ts));
                };
            }
        });
        quote! {#(#ts)*}
    };
    let match_try_new_in_de_ts = gen_match_try_new_in_de_ts(&ident, &{
        let fields_ts = (0..len).map(gen_undrscr_undrscr_field_i_h_ts);
        quote! {#(#fields_ts),*}
    });
    let visit_map_fields_init_ts = {
        let ts = vec_ident_type
            .iter()
            .enumerate()
            .map(|(i, (el_ident, el_type))| {
                let type_ts = gen_type_ts(el_ident, el_type);
                let field_i_ts = gen_undrscr_undrscr_field_i_ts(i);
                quote! {
                    let mut #field_i_ts: Option<#type_ts> = None;
                }
            });
        quote! {#(#ts)*}
    };
    let visit_map_match_vrts_ts =
        {
            let visit_map_match_vrts_ts = vec_ident_type.iter().enumerate().map(
                |(i, (el_ident, el_type))| {
                    let field_i_ts = gen_undrscr_undrscr_field_i_ts(i);
                    let fi_dq_ts = dq_ts(&el_ident);
                    let type_ts = gen_type_ts(el_ident, el_type);
                    quote! {
                        __Field::#field_i_ts => {
                            if Option::is_some(&#field_i_ts) {
                                return Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(#fi_dq_ts),
                                );
                            }
                            #field_i_ts = Some(
                                serde::de::MapAccess::next_value::<#type_ts>(&mut __map)?,
                            );
                        }
                    }
                },
            );
            quote! {#(#visit_map_match_vrts_ts)*}
        };
    let visit_map_missing_fields_check_ts = {
        let ts = vec_ident.iter().enumerate().map(|(i, el)| {
            let field_i_ts = gen_undrscr_undrscr_field_i_ts(i);
            let field_i_h_ts = gen_undrscr_undrscr_field_i_h_ts(i);
            let fi_dq_ts = dq_ts(&el);
            quote! {
                let #field_i_h_ts = match #field_i_ts {
                    Some(v) => v,
                    None => {
                        serde::__private228::de::missing_field(#fi_dq_ts)?
                    }
                };
            }
        });
        quote! {#(#ts)*}
    };
    let fields_arr_els_ts = {
        let fields_arr_els_ts = vec_ident.iter().map(|el| dq_ts(&el));
        quote! {#(#fields_arr_els_ts),*}
    };
    let ident_dq_ts = dq_ts(&ident);
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySrcItemOrdering
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #ident {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        #field_enum_vrts_ts,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl _serde::de::Visitor<'_> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter<'_>,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            v: u64,
                        ) -> Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match v {
                                #visit_u64_v_enum_vrts_ts,
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            v: &str,
                        ) -> Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match v {
                                #visit_str_v_enum_vrts_ts
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            v: &[u8],
                        ) -> Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match v {
                                #visit_bytes_v_enum_vrts_ts
                                _ => Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<#ident>,
                        lt: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #ident;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter<'_>,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                #struct_ident_dq_ts,
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            #visit_seq_fields_init_ts
                            #match_try_new_in_de_ts
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            #visit_map_fields_init_ts
                            while let Some(__k) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __k {
                                    #visit_map_match_vrts_ts
                                    __Field::__ignore => {
                                        let _: serde::de::IgnoredAny = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            #visit_map_missing_fields_check_ts
                            #match_try_new_in_de_ts
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &[&str] = &[#fields_arr_els_ts];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        #ident_dq_ts,
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<Self>,
                            lt: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
    }
}
pub fn wrap_into_scopes_ts(ts: &dyn ToTokens) -> Ts2 {
    quote! {(#ts)}
}
pub fn mb_wrap_into_braces_ts(ts: &dyn ToTokens, wrap: bool) -> Ts2 {
    if wrap {
        wrap_into_scopes_ts(&ts)
    } else {
        quote! {#ts}
    }
}
pub fn gen_v_dcl_ts(import: &Import, ts: &dyn ToTokens) -> Ts2 {
    quote! {#import::V<#ts>}
}
pub fn gen_v_init_ts(import: &Import, ts: &dyn ToTokens) -> Ts2 {
    quote! {#import::V { #VSc: #ts }}
}
pub fn impl_pg_type_eq_oprtr_for_ident_ts(
    import: &Import,
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl #import::#PgTypeEqOprtrUcc for #ident {
            fn oprtr(&self) -> #import::#EqOprtrUcc {
                #ts
            }
        }
    }
}
#[must_use]
pub fn gen_qp_er_write_into_buffer_ts(import: Import) -> Ts2 {
    quote! {
        #import::QpEr::WriteIntoBuffer {
            loc: location_lib::loc!()
        }
    }
}
#[must_use]
pub fn gen_return_err_qp_er_write_into_buffer_ts(import: Import) -> Ts2 {
    let ts = gen_qp_er_write_into_buffer_ts(import);
    quote! {return Err(#ts);}
}
#[must_use]
pub fn gen_jsonb_build_obj(v: &dyn Display) -> String {
    format!("jsonb_build_object({v})")
}
#[must_use]
pub fn gen_jsonb_build_obj_v(v: &dyn Display) -> String {
    gen_jsonb_build_obj(&format!("'v',{v}"))
}
