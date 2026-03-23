mod flts;
use enum_extension_lib::EnumExtension;
pub use flts::*;
use gen_quotes::dq_ts;
use macros_helpers::DTsBuilder;
use macros_helpers::gen_impl_display_ts;
use macros_helpers::gen_impl_to_err_string_ts;
use naming::prm::{SelfCrUcc, SelfSelUcc, SelfWhUcc};
use naming::{
    AddOprtrSc, AllVrtsDfltSomeOneElMaxPageSizeSc, AllVrtsDfltSomeOneElSc, ColFieldForErMsgSc,
    ColFieldSc, ColSc, CrForQueryUcc, CrIntoPgJsonOptVecWhLenEqSc,
    CrIntoPgJsonOptVecWhLenGreaterThanSc, CrIntoPgTypeOptVecWhDimOneEqSc, CrQbSc, CrQpSc, CrSc,
    CrTblColQpSc, CrUcc, DfltSomeOneElMaxPageSizeSc, DfltSomeOneElSc, DisplayPlusToTokens,
    EqOprtrUcc, FiSc, IncrSc, IsPkSc, JsonbSetAccumulatorSc, JsonbSetPathSc, JsonbSetTargetSc,
    MutSc, NormalizeSc, OptUcc, OptUpdSc, OptVecCrSc, PgJsonTestCasesUcc, PgJsonUcc,
    PgTypeEqOprtrUcc, PgTypeNotPkUcc, PgTypeOptVecWhGreaterThanTestSc, PgTypeTestCasesUcc,
    PgTypeUcc, PgTypeWhFltUcc, PreviousRdAndOptUpdIntoRdSc, QbSc, QpErUcc, QpSc, QuerySc,
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
macro_rules! bool_enum_to_tokens {
    ($name:ident, false => $false_expr:expr, true => $true_expr:expr) => {
        #[derive(Debug, Clone, Copy, Optml)]
        pub enum $name {
            False,
            True,
        }
        impl ToTokens for $name {
            fn to_tokens(&self, tokens: &mut Ts2) {
                match &self {
                    Self::False => ($false_expr).to_tokens(tokens),
                    Self::True => ($true_expr).to_tokens(tokens),
                }
            }
        }
    };
}
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
bool_enum_to_tokens!(AddOprtrUndrscr, false => AddOprtrSc, true => quote! {_});
bool_enum_to_tokens!(ColPrmUndrscr, false => ColSc, true => quote! {_});
bool_enum_to_tokens!(IncrPrmUndrscr, false => IncrSc, true => quote! {_});
bool_enum_to_tokens!(IsCrQbMut, false => Ts2::new(), true => MutSc);
bool_enum_to_tokens!(IsQbMut, false => Ts2::new(), true => MutSc);
bool_enum_to_tokens!(IsSelOnlyCrdIdsQbMut, false => Ts2::new(), true => MutSc);
bool_enum_to_tokens!(IsSelOnlyUpddIdsQbMut, false => Ts2::new(), true => MutSc);
bool_enum_to_tokens!(IsSelQpColFieldForErMsgUsed, false => quote! {_}, true => ColFieldForErMsgSc);
bool_enum_to_tokens!(IsSelQpIsPgTypeUsed, false => quote! {_}, true => quote! {is_pg_type});
bool_enum_to_tokens!(IsSelQpSelfSelUsed, false => quote! {_}, true => VSc);
bool_enum_to_tokens!(IsUpdQbMut, false => Ts2::new(), true => MutSc);
bool_enum_to_tokens!(IsUpdQpJsonbSetTargetUsed, false => quote! {_}, true => JsonbSetTargetSc);
bool_enum_to_tokens!(IsUpdQpSelfUpdUsed, false => quote! {_}, true => VSc);
bool_enum_to_tokens!(ShouldDSchemarsJsonSchema, false => Ts2::new(), true => quote! {, schemars::JsonSchema});
bool_enum_to_tokens!(ShouldDeriveUtoipaToSchema, false => Ts2::new(), true => quote! {, utoipa::ToSchema});
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
bool_enum_to_tokens!(IsPkUndrscr, false => IsPkSc, true => quote! {_});
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
bool_enum_to_tokens!(CrQbValueUndrscr, false => VSc, true => quote! {_});
bool_enum_to_tokens!(CrQpIncrUndrscr, false => IncrSc, true => quote! {_});
bool_enum_to_tokens!(CrQpValueUndrscr, false => VSc, true => quote! {_});
bool_enum_to_tokens!(SelQpValueUndrscr, false => VSc, true => quote! {_});
bool_enum_to_tokens!(UpdQpJsonbSetAccumulatorUndrscr, false => quote! {jsonb_set_accumulator}, true => quote! {_});
bool_enum_to_tokens!(UpdQpJsonbSetPathUndrscr, false => quote! {jsonb_set_path}, true => quote! {_});
bool_enum_to_tokens!(UpdQpJsonbSetTargetUndrscr, false => quote! {jsonb_set_target}, true => quote! {_});
bool_enum_to_tokens!(UpdQpValueUndrscr, false => VSc, true => quote! {_});
pub fn gen_pg_type_wh_ts(
    attrs_ts: &dyn ToTokens,
    vrts: &Vec<&dyn PgFlt>,
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
            quote! {#el_ucc(wh_flts::#prefix_wh_self_ucc #type_ts)}
        });
        quote! {
            #attrs_ts
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_derive_utoipa_to_schema #should_derive_schemars_json_schema, optml::Optml)]
            pub enum #ident {
                #(#vrts_ts),*
            }
        }
    };
    let impl_pg_type_pg_type_wh_flt_for_pg_type_tokens_wh_ts = impl_pg_type_wh_flt_for_ident_ts(
        &quote! {<'lt>},
        &ident,
        &Ts2::new(),
        &IncrPrmUndrscr::False,
        &ColPrmUndrscr::False,
        &AddOprtrUndrscr::False,
        &{
            let vrts_ts = vrts.iter().map(|el| {
                let el_ucc = el.ucc();
                quote! {
                    Self::#el_ucc(#VSc) => pg_crud_cmn::PgTypeWhFlt::qp(
                        #VSc,
                        #IncrSc,
                        #ColSc,
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
                    Self::#el_ucc(#VSc) => pg_crud_cmn::PgTypeWhFlt::qb(
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
    let impl_loc_lib_to_err_string_for_pg_type_tokens_wh_ts =
        gen_impl_to_err_string_no_generics_ts(&ident, &quote! {format!("{self:#?}")});
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
        #impl_pg_type_pg_type_wh_flt_for_pg_type_tokens_wh_ts
        #impl_loc_lib_to_err_string_for_pg_type_tokens_wh_ts
        #impl_all_vrts_dflt_some_one_el_for_pg_type_tokens_wh_ts
    }
}
pub fn gen_impl_to_err_string_no_generics_ts(ident: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    gen_impl_to_err_string_ts(&Ts2::new(), ident, &Ts2::new(), ts)
}
pub fn gen_impl_display_and_to_err_string_debug_ts(ident: &dyn ToTokens) -> Ts2 {
    let impl_display_ts = gen_impl_display_ts(
        &Ts2::new(),
        ident,
        &Ts2::new(),
        &quote! {write!(f, "{self:?}")},
    );
    let impl_to_err_string_ts =
        gen_impl_to_err_string_no_generics_ts(ident, &quote! {format!("{self:#?}")});
    quote! {
        #impl_display_ts
        #impl_to_err_string_ts
    }
}
#[must_use]
pub fn pg_crud_cmn_qp_er_ts() -> Ts2 {
    quote! {pg_crud_cmn::#QpErUcc}
}
#[must_use]
pub fn gen_dim_nbr_pgn_ts(dim_nbr: usize) -> Ts2 {
    format!("dim{dim_nbr}_pgn")
        .parse::<Ts2>()
        .expect("7c3a91b2")
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
pub fn gen_impl_pg_json_all_methods_ts(
    import: &Import,
    ident: &dyn ToTokens,
    tt_type_ts: &dyn ToTokens,
    cr_type_ts: &dyn ToTokens,
    cr_for_query_type_ts: &dyn ToTokens,
    sel_type_ts: &dyn ToTokens,
    is_sel_qp_self_sel_used: &IsSelQpSelfSelUsed,
    is_sel_qp_col_field_for_er_msg_used: &IsSelQpColFieldForErMsgUsed,
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
    gen_impl_pg_json_ts(
        import,
        ident,
        tt_type_ts,
        cr_type_ts,
        cr_for_query_type_ts,
        sel_type_ts,
        is_sel_qp_self_sel_used,
        is_sel_qp_col_field_for_er_msg_used,
        is_sel_qp_is_pg_type_used,
        sel_qp_ts,
        wh_type_ts,
        rd_type_ts,
        rd_ids_type_ts,
        Some(sel_only_ids_qp_ts),
        rd_inn_type_ts,
        into_inn_ts,
        upd_type_ts,
        upd_type_for_query_ts,
        Some((
            upd_qp_ts,
            is_upd_qp_self_upd_used,
            is_upd_qp_jsonb_set_target_used,
        )),
        is_upd_qb_mut,
        upd_qb_ts,
        Some((
            sel_only_updd_ids_qp_ts,
            is_sel_only_updd_ids_qb_mut,
            sel_only_updd_ids_qb_ts,
        )),
        Some((
            sel_only_crd_ids_qp_ts,
            is_sel_only_crd_ids_qb_mut,
            sel_only_crd_ids_qb_ts,
        )),
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
    is_sel_qp_col_field_for_er_msg_used: &IsSelQpColFieldForErMsgUsed,
    is_sel_qp_is_pg_type_used: &IsSelQpIsPgTypeUsed,
    sel_qp_ts: &dyn ToTokens,
    wh_type_ts: &dyn ToTokens,
    rd_type_ts: &dyn ToTokens,
    rd_ids_type_ts: &dyn ToTokens,
    opt_sel_only_ids_qp_ts: Option<&dyn ToTokens>,
    rd_inn_type_ts: &dyn ToTokens,
    into_inn_ts: &dyn ToTokens,
    upd_type_ts: &dyn ToTokens,
    upd_type_for_query_ts: &dyn ToTokens,
    opt_upd_qp_ts: Option<(
        &dyn ToTokens,
        &IsUpdQpSelfUpdUsed,
        &IsUpdQpJsonbSetTargetUsed,
    )>,
    is_upd_qb_mut: &IsUpdQbMut,
    upd_qb_ts: &dyn ToTokens,
    opt_sel_only_updd_ids: Option<(&dyn ToTokens, &IsSelOnlyUpddIdsQbMut, &dyn ToTokens)>,
    opt_sel_only_crd_ids: Option<(&dyn ToTokens, &IsSelOnlyCrdIdsQbMut, &dyn ToTokens)>,
) -> Ts2 {
    let path_ts = quote! {#import ::};
    let reference_mut_u64_ts = quote! {&mut #U64};
    let query_pg_args_ts =
        quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let query_lt_pg_args_ts =
        quote! {sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let opt_sel_only_ids_qp_method_ts = opt_sel_only_ids_qp_ts.map(|sel_only_ids_qp_ts| {
        quote! {
            fn #SelOnlyIdsQpSc(
                #ColFieldSc: #RefStr,
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #sel_only_ids_qp_ts
            }
        }
    });
    let opt_upd_qp_method_ts = opt_upd_qp_ts.map(
        |(upd_qp_ts, is_upd_qp_self_upd_used, is_upd_qp_jsonb_set_target_used)| {
            quote! {
                fn #UpdQpSc(
                    #is_upd_qp_self_upd_used: &Self::#UpdForQueryUcc,
                    #JsonbSetAccumulatorSc: #RefStr,
                    #is_upd_qp_jsonb_set_target_used: #RefStr,
                    #JsonbSetPathSc: #RefStr,
                    #IncrSc: #reference_mut_u64_ts,
                ) -> Result<#StringTs, #path_ts #QpErUcc> {
                    #upd_qp_ts
                }
            }
        },
    );
    let opt_sel_only_updd_ids_method_ts = opt_sel_only_updd_ids.map(
        |(sel_only_updd_ids_qp_ts, is_sel_only_updd_ids_qb_mut, sel_only_updd_ids_qb_ts)| {
            quote! {
                fn #SelOnlyUpddIdsQpSc(
                    #VSc: &Self::#UpdForQueryUcc,
                    #FiSc: #RefStr,
                    #ColFieldSc: #RefStr,
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
            }
        },
    );
    let opt_sel_only_crd_ids_method_ts = opt_sel_only_crd_ids.map(
        |(sel_only_crd_ids_qp_ts, is_sel_only_crd_ids_qb_mut, sel_only_crd_ids_qb_ts)| {
            quote! {
                fn #SelOnlyCrdIdsQpSc(
                    #VSc: &Self::#CrForQueryUcc,
                    #FiSc: #RefStr,
                    #ColFieldSc: #RefStr,
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
        },
    );
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
                #ColFieldSc: #RefStr,
                #is_sel_qp_col_field_for_er_msg_used: #RefStr,
                #is_sel_qp_is_pg_type_used: #Bool,
            ) -> Result<#StringTs, #path_ts #QpErUcc> {
                #sel_qp_ts
            }
            type #WhUcc = #wh_type_ts;
            type #RdUcc = #rd_type_ts;
            type #RdIdsUcc = #rd_ids_type_ts;
            #opt_sel_only_ids_qp_method_ts
            type #RdInnUcc = #rd_inn_type_ts;
            fn into_inn(#VSc: Self::#RdUcc) -> Self::#RdInnUcc {
                #into_inn_ts
            }
            type #UpdUcc = #upd_type_ts;
            type #UpdForQueryUcc = #upd_type_for_query_ts;
            #opt_upd_qp_method_ts
            fn #UpdQbSc(
                #VSc: Self::#UpdForQueryUcc,
                #is_upd_qb_mut #QuerySc: #query_pg_args_ts
            ) -> Result<#query_pg_args_ts, #StringTs> {
                #upd_qb_ts
            }
            #opt_sel_only_updd_ids_method_ts
            #opt_sel_only_crd_ids_method_ts
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
pub fn impl_pg_type_wh_flt_for_ident_ts(
    impl_generic_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    incr_prm_undrscr: &IncrPrmUndrscr,
    col_prm_undrscr: &ColPrmUndrscr,
    add_oprtr_undrscr: &AddOprtrUndrscr,
    qp_ts: &dyn ToTokens,
    is_qb_mut: &IsQbMut,
    qb_ts: &dyn ToTokens,
    import: &Import,
) -> Ts2 {
    quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #impl_generic_ts #import ::#PgTypeWhFltUcc<'lt> for #ident_ts #ident_generic_ts {
            fn #QpSc(
                &self,
                #incr_prm_undrscr: &mut #U64,
                #col_prm_undrscr: &dyn #StdFmtDisplay,
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
pub fn gen_impl_sqlx_type_and_encode_for_ident_ts(
    ident_ts: &dyn ToTokens,
    type_ts: &dyn ToTokens,
    encode_ts: &dyn ToTokens,
) -> Ts2 {
    let impl_type_ts = gen_impl_sqlx_type_for_ident_ts(ident_ts, type_ts);
    let impl_encode_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(ident_ts, encode_ts);
    quote! {
        #impl_type_ts
        #impl_encode_ts
    }
}
pub fn gen_impl_pg_type_ts(
    import: &Import,
    ident: &dyn ToTokens,
    ident_tt_ucc: &dyn ToTokens,
    is_pk_undrscr: &IsPkUndrscr,
    cr_tbl_col_qp_ts: &dyn ToTokens,
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
            fn #CrTblColQpSc(#ColSc: &dyn #StdFmtDisplay, #is_pk_undrscr: #Bool) -> impl #StdFmtDisplay {
                #cr_tbl_col_qp_ts
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
                #ColSc: #RefStr,
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
                #ColSc: #RefStr
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
                #ColSc: #RefStr,
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
    opt_vec_cr_ts: Option<&Ts2>,
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
    rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts: Option<&Ts2>,
    cr_into_pg_type_opt_vec_wh_dim_one_eq_ts: Option<&Ts2>,
    pg_type_opt_vec_wh_greater_than_test_ts: Option<&Ts2>,
    rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts: Option<&Ts2>,
    cr_into_pg_json_opt_vec_wh_dim_one_eq_ts: Option<&Ts2>,
    cr_into_pg_json_opt_vec_wh_dim_two_eq_ts: Option<&Ts2>,
    cr_into_pg_json_opt_vec_wh_dim_three_eq_ts: Option<&Ts2>,
    cr_into_pg_json_opt_vec_wh_dim_four_eq_ts: Option<&Ts2>,
    cr_into_pg_json_opt_vec_wh_len_eq_ts: Option<&Ts2>,
    cr_into_pg_json_opt_vec_wh_len_greater_than_ts: Option<&Ts2>,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts: Option<&Ts2>,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts: Option<&Ts2>,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts: Option<&Ts2>,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts: Option<&Ts2>,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts: Option<&Ts2>,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts: Option<&Ts2>,
) -> Ts2 {
    let self_pg_type_as_pg_type_ts = quote! {<#SelfUcc::#PgTypeUcc as #import::#PgTypeUcc>};
    let self_pg_type_as_pg_type_rd_ids_ts = quote! {#self_pg_type_as_pg_type_ts::#RdIdsUcc};
    let self_pg_type_as_pg_type_cr_ts = quote! {#self_pg_type_as_pg_type_ts::#CrUcc};
    let self_pg_type_as_pg_type_wh_ts = quote! {#self_pg_type_as_pg_type_ts::#WhUcc};
    let ident_sel_ucc = SelfSelUcc::from_tokens(&ident);
    let opt_vec_cr_ts_gnrtd =
        opt_vec_cr_ts.map(|ts| gen_opt_vec_cr_ts(&self_pg_type_as_pg_type_ts, ts));
    let rd_ids_to_2_dims_vec_rd_inn_ts_gnrtd = gen_rd_ids_to_2_dims_vec_rd_inn_ts(
        &self_pg_type_as_pg_type_ts,
        &rd_ids_to_2_dims_vec_rd_inn_ts,
    );
    let rd_inn_into_rd_with_new_or_try_new_unwraped_ts_gnrtd =
        gen_rd_inn_into_rd_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &rd_inn_into_rd_with_new_or_try_new_unwraped_ts,
        );
    let rd_inn_into_upd_with_new_or_try_new_unwraped_ts_gnrtd =
        gen_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &rd_inn_into_upd_with_new_or_try_new_unwraped_ts,
        );
    let upd_to_rd_ids_ts_gnrtd =
        gen_upd_to_rd_ids_ts(&self_pg_type_as_pg_type_ts, &upd_to_rd_ids_ts);
    let rd_ids_to_opt_v_rd_dflt_some_one_el_ts_gnrtd = gen_rd_ids_to_opt_v_rd_dflt_some_one_el_ts(
        *import,
        &self_pg_type_as_pg_type_ts,
        &rd_ids_to_opt_v_rd_dflt_some_one_el_ts,
    );
    let previous_rd_and_opt_upd_into_rd_ts_gnrtd = gen_previous_rd_and_opt_upd_into_rd_ts(
        &self_pg_type_as_pg_type_ts,
        &previous_rd_and_opt_upd_into_rd_ts,
    );
    let rd_ids_and_cr_into_rd_ts_gnrtd =
        gen_rd_ids_and_cr_into_rd_ts(&self_pg_type_as_pg_type_ts, &rd_ids_and_cr_into_rd_ts);
    let rd_ids_and_cr_into_opt_v_rd_ts_gnrtd = gen_rd_ids_and_cr_into_opt_v_rd_ts(
        *import,
        &self_pg_type_as_pg_type_ts,
        &rd_ids_and_cr_into_opt_v_rd_ts,
    );
    let rd_ids_and_cr_into_tt_ts_gnrtd =
        gen_rd_ids_and_cr_into_tt_ts(&self_pg_type_as_pg_type_ts, &rd_ids_and_cr_into_tt_ts);
    let rd_ids_and_cr_into_wh_eq_ts_gnrtd = gen_rd_ids_and_cr_into_wh_eq_ts(
        &self_pg_type_as_pg_type_rd_ids_ts,
        &self_pg_type_as_pg_type_cr_ts,
        &self_pg_type_as_pg_type_wh_ts,
        &rd_ids_and_cr_into_wh_eq_ts,
    );
    let rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_gnrtd =
        gen_rd_ids_and_cr_into_vec_wh_eq_using_fields_ts(
            import,
            &self_pg_type_as_pg_type_rd_ids_ts,
            &self_pg_type_as_pg_type_cr_ts,
            &self_pg_type_as_pg_type_wh_ts,
            &rd_ids_and_cr_into_vec_wh_eq_using_fields_ts,
        );
    let rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts_gnrtd =
        rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts.map(|ts| {
            gen_rd_ids_and_cr_into_vec_or_opt_vec_wh_eq_to_json_field_pg_type_or_pg_json_ts(
                *import,
                &self_pg_type_as_pg_type_rd_ids_ts,
                &self_pg_type_as_pg_type_cr_ts,
                &self_pg_type_as_pg_type_wh_ts,
                ts,
                PgTypeOrPgJson::PgType,
            )
        });
    let cr_into_pg_type_opt_vec_wh_dim_one_eq_ts_gnrtd = cr_into_pg_type_opt_vec_wh_dim_one_eq_ts
        .map(|ts| {
            let cr_into_pg_type_opt_vec_wh_dim_one_eq_sc = CrIntoPgTypeOptVecWhDimOneEqSc;
            quote! {
                fn #cr_into_pg_type_opt_vec_wh_dim_one_eq_sc(
                    #CrSc: #self_pg_type_as_pg_type_ts::#CrUcc
                ) -> Option<#import::NotEmptyUnqVec<#self_pg_type_as_pg_type_ts::#WhUcc>> {
                    #ts
                }
            }
        });
    let pg_type_opt_vec_wh_greater_than_test_ts_gnrtd = pg_type_opt_vec_wh_greater_than_test_ts
        .map(|ts| {
            quote! {
                fn #PgTypeOptVecWhGreaterThanTestSc() -> Option<
                    #import::NotEmptyUnqVec<
                        #import::PgTypeGreaterThanTest<
                            #SelfUcc::#PgTypeUcc
                        >
                    >
                > {
                    #ts
                }
            }
        });
    let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts_gnrtd =
        rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts.map(|ts| {
            let rd_ids_and_tt_into_pg_type_opt_wh_greater_than_sc =
                RdIdsAndTtIntoPgTypeOptWhGreaterThanSc;
            quote! {
                fn #rd_ids_and_tt_into_pg_type_opt_wh_greater_than_sc(
                    greater_than_vrt: #import::PgTypeGreaterThanVrt,
                    #RdIdsSc: #self_pg_type_as_pg_type_ts::#RdIdsUcc,
                    #TtSc: #self_pg_type_as_pg_type_ts::#TtUcc,
                ) -> Option<#self_pg_type_as_pg_type_ts::#WhUcc> {
                    #ts
                }
            }
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts_gnrtd =
        cr_into_pg_json_opt_vec_wh_dim_one_eq_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts_gnrtd =
        cr_into_pg_json_opt_vec_wh_dim_two_eq_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts_gnrtd =
        cr_into_pg_json_opt_vec_wh_dim_three_eq_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts_gnrtd =
        cr_into_pg_json_opt_vec_wh_dim_four_eq_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let cr_into_pg_json_opt_vec_wh_len_eq_ts_gnrtd =
        cr_into_pg_json_opt_vec_wh_len_eq_ts.map(|ts| {
            gen_cr_into_pg_json_opt_vec_wh_len_eq_ts(*import, &self_pg_type_as_pg_type_ts, ts)
        });
    let cr_into_pg_json_opt_vec_wh_len_greater_than_ts_gnrtd =
        cr_into_pg_json_opt_vec_wh_len_greater_than_ts.map(|ts| {
            gen_cr_into_pg_json_opt_vec_wh_len_greater_than_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts_gnrtd =
        rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts_gnrtd =
        rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts_gnrtd =
        rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts_gnrtd =
        rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts_gnrtd =
        rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts_gnrtd =
        rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts(
                *import,
                &self_pg_type_as_pg_type_ts,
                ts,
            )
        });
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySrcItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import::#PgTypeTestCasesUcc for #ident {
            type #PgTypeUcc = #SelfUcc;
            type #SelUcc = #ident_sel_ucc;
            #opt_vec_cr_ts_gnrtd
            #rd_ids_to_2_dims_vec_rd_inn_ts_gnrtd
            #rd_inn_into_rd_with_new_or_try_new_unwraped_ts_gnrtd
            #rd_inn_into_upd_with_new_or_try_new_unwraped_ts_gnrtd
            #upd_to_rd_ids_ts_gnrtd
            #rd_ids_to_opt_v_rd_dflt_some_one_el_ts_gnrtd
            #previous_rd_and_opt_upd_into_rd_ts_gnrtd
            #rd_ids_and_cr_into_rd_ts_gnrtd
            #rd_ids_and_cr_into_opt_v_rd_ts_gnrtd
            #rd_ids_and_cr_into_tt_ts_gnrtd
            #rd_ids_and_cr_into_wh_eq_ts_gnrtd
            #rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_gnrtd
            #rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field_ts_gnrtd
            #cr_into_pg_type_opt_vec_wh_dim_one_eq_ts_gnrtd
            #pg_type_opt_vec_wh_greater_than_test_ts_gnrtd
            #rd_ids_and_tt_into_pg_type_opt_wh_greater_than_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts_gnrtd
            #cr_into_pg_json_opt_vec_wh_len_eq_ts_gnrtd
            #cr_into_pg_json_opt_vec_wh_len_greater_than_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts_gnrtd
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
    opt_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts: Option<&dyn ToTokens>,
    cr_into_pg_json_opt_vec_wh_len_eq_ts: &dyn ToTokens,
    cr_into_pg_json_opt_vec_wh_len_greater_than_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts: &dyn ToTokens,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts: &dyn ToTokens,
    opt_rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts: Option<&dyn ToTokens>,
    rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts: &dyn ToTokens,
    opt_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts: Option<&dyn ToTokens>,
) -> Ts2 {
    let self_pg_json_as_pg_json_ts = quote! {<#SelfUcc::#PgJsonUcc as #import::#PgJsonUcc>};
    let self_pg_json_as_pg_json_rd_ids_ts = quote! {#self_pg_json_as_pg_json_ts::#RdIdsUcc};
    let self_pg_json_as_pg_json_cr_ts = quote! {#self_pg_json_as_pg_json_ts::#CrUcc};
    let self_pg_json_as_pg_json_wh_ts = quote! {#self_pg_json_as_pg_json_ts::#WhUcc};
    let ident_sel_ucc = SelfSelUcc::from_tokens(&ident);
    let opt_vec_cr_ts_gnrtd = gen_opt_vec_cr_ts(&self_pg_json_as_pg_json_ts, &opt_vec_cr_ts);
    let rd_ids_to_2_dims_vec_rd_inn_ts_gnrtd = gen_rd_ids_to_2_dims_vec_rd_inn_ts(
        &self_pg_json_as_pg_json_ts,
        &rd_ids_to_2_dims_vec_rd_inn_ts,
    );
    let rd_inn_into_rd_with_new_or_try_new_unwraped_ts_gnrtd =
        gen_rd_inn_into_rd_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_json_as_pg_json_ts,
            &rd_inn_into_rd_with_new_or_try_new_unwraped_ts,
        );
    let rd_inn_into_upd_with_new_or_try_new_unwraped_ts_gnrtd =
        gen_rd_inn_into_upd_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_json_as_pg_json_ts,
            &rd_inn_into_upd_with_new_or_try_new_unwraped_ts,
        );
    let upd_to_rd_ids_ts_gnrtd =
        gen_upd_to_rd_ids_ts(&self_pg_json_as_pg_json_ts, &upd_to_rd_ids_ts);
    let rd_ids_to_opt_v_rd_dflt_some_one_el_ts_gnrtd = gen_rd_ids_to_opt_v_rd_dflt_some_one_el_ts(
        *import,
        &self_pg_json_as_pg_json_ts,
        &rd_ids_to_opt_v_rd_dflt_some_one_el_ts,
    );
    let previous_rd_and_opt_upd_into_rd_ts_gnrtd = gen_previous_rd_and_opt_upd_into_rd_ts(
        &self_pg_json_as_pg_json_ts,
        &previous_rd_and_opt_upd_into_rd_ts,
    );
    let rd_ids_and_cr_into_rd_ts_gnrtd =
        gen_rd_ids_and_cr_into_rd_ts(&self_pg_json_as_pg_json_ts, &rd_ids_and_cr_into_rd_ts);
    let rd_ids_and_cr_into_opt_v_rd_ts_gnrtd = gen_rd_ids_and_cr_into_opt_v_rd_ts(
        *import,
        &self_pg_json_as_pg_json_ts,
        &rd_ids_and_cr_into_opt_v_rd_ts,
    );
    let rd_ids_and_cr_into_tt_ts_gnrtd =
        gen_rd_ids_and_cr_into_tt_ts(&self_pg_json_as_pg_json_ts, &rd_ids_and_cr_into_tt_ts);
    let rd_ids_and_cr_into_wh_eq_ts_gnrtd = gen_rd_ids_and_cr_into_wh_eq_ts(
        &self_pg_json_as_pg_json_rd_ids_ts,
        &self_pg_json_as_pg_json_cr_ts,
        &self_pg_json_as_pg_json_wh_ts,
        &rd_ids_and_cr_into_wh_eq_ts,
    );
    let rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_gnrtd =
        gen_rd_ids_and_cr_into_vec_wh_eq_using_fields_ts(
            import,
            &self_pg_json_as_pg_json_rd_ids_ts,
            &self_pg_json_as_pg_json_cr_ts,
            &self_pg_json_as_pg_json_wh_ts,
            &rd_ids_and_cr_into_vec_wh_eq_using_fields_ts,
        );
    let rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts_gnrtd =
        gen_rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts(
            *import,
            &self_pg_json_as_pg_json_rd_ids_ts,
            &self_pg_json_as_pg_json_cr_ts,
            &self_pg_json_as_pg_json_wh_ts,
            &rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts_gnrtd =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &cr_into_pg_json_opt_vec_wh_dim_one_eq_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts_gnrtd =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &cr_into_pg_json_opt_vec_wh_dim_two_eq_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts_gnrtd =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &cr_into_pg_json_opt_vec_wh_dim_three_eq_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts_gnrtd =
        opt_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts(
                *import,
                &self_pg_json_as_pg_json_ts,
                ts,
            )
        });
    let cr_into_pg_json_opt_vec_wh_len_eq_ts_gnrtd = gen_cr_into_pg_json_opt_vec_wh_len_eq_ts(
        *import,
        &self_pg_json_as_pg_json_ts,
        &cr_into_pg_json_opt_vec_wh_len_eq_ts,
    );
    let cr_into_pg_json_opt_vec_wh_len_greater_than_ts_gnrtd =
        gen_cr_into_pg_json_opt_vec_wh_len_greater_than_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &cr_into_pg_json_opt_vec_wh_len_greater_than_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts_gnrtd =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts_gnrtd =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts_gnrtd =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts_gnrtd =
        opt_rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts(
                *import,
                &self_pg_json_as_pg_json_ts,
                ts,
            )
        });
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts_gnrtd =
        gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts(
            *import,
            &self_pg_json_as_pg_json_ts,
            &rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts,
        );
    let rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts_gnrtd =
        opt_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts.map(|ts| {
            gen_rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts(
                *import,
                &self_pg_json_as_pg_json_ts,
                ts,
            )
        });
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySrcItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import::#PgJsonTestCasesUcc for #ident {
            type #PgJsonUcc = #SelfUcc;
            type #SelUcc = #ident_sel_ucc;
            #opt_vec_cr_ts_gnrtd
            #rd_ids_to_2_dims_vec_rd_inn_ts_gnrtd
            #rd_inn_into_rd_with_new_or_try_new_unwraped_ts_gnrtd
            #rd_inn_into_upd_with_new_or_try_new_unwraped_ts_gnrtd
            fn #RdIdsIntoOptVRdInnSc(
                #VSc: #self_pg_json_as_pg_json_ts::#RdIdsUcc
            ) -> Option<#import::#VUcc<#self_pg_json_as_pg_json_ts::#RdInnUcc>> {
                #rd_ids_into_opt_v_rd_inn_ts
            }
            #upd_to_rd_ids_ts_gnrtd
            #rd_ids_to_opt_v_rd_dflt_some_one_el_ts_gnrtd
            #previous_rd_and_opt_upd_into_rd_ts_gnrtd
            #rd_ids_and_cr_into_rd_ts_gnrtd
            #rd_ids_and_cr_into_opt_v_rd_ts_gnrtd
            #rd_ids_and_cr_into_tt_ts_gnrtd
            #rd_ids_and_cr_into_wh_eq_ts_gnrtd
            #rd_ids_and_cr_into_vec_wh_eq_using_fields_ts_gnrtd
            #rd_ids_and_cr_into_vec_wh_eq_to_json_field_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq_ts_gnrtd
            #cr_into_pg_json_opt_vec_wh_len_eq_ts_gnrtd
            #cr_into_pg_json_opt_vec_wh_len_greater_than_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_in_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than_ts_gnrtd
            #rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx_ts_gnrtd
        }
    }
}
#[must_use]
pub fn pg_crud_cmn_qp_er_checked_add_init_ts() -> Ts2 {
    quote! {pg_crud_cmn::QpEr::CheckedAdd { loc: loc_lib::loc!() }}
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
    _len: usize,
    gen_type_ts: &dyn Fn(&Ident, &Type) -> Ts2,
) -> Ts2 {
    let raw_ident_ts = format!("{ident}Raw").parse::<Ts2>().expect("a1b2c3d4");
    let raw_fields_ts = vec_ident_type.iter().map(|(fi, ty)| {
        let type_ts = gen_type_ts(fi, ty);
        quote! { #fi: #type_ts, }
    });
    let try_from_fields_ts = vec_ident_type.iter().map(|(fi, _)| {
        quote! { raw.#fi }
    });
    quote! {
        #[derive(serde::Deserialize)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        struct #raw_ident_ts {
            #(#raw_fields_ts)*
        }
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
                    let raw = <#raw_ident_ts as _serde::Deserialize>::deserialize(__deserializer)?;
                    Self::try_new(#(#try_from_fields_ts),*).map_err(|er| _serde::de::Error::custom(format!("{er:?}")))
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
            loc: loc_lib::loc!()
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
#[must_use]
pub fn gen_jsonb_set(accumulator: &dyn Display, path: &dyn Display, value: &dyn Display) -> String {
    format!("jsonb_set({accumulator},'{{{path}}}',{value})")
}
#[must_use]
pub fn gen_case_jsonb_typeof_null(target: &dyn Display, else_expr: &dyn Display) -> String {
    format!("case when jsonb_typeof({target}) = 'null' then 'null'::jsonb else ({else_expr}) end")
}
#[must_use]
pub fn gen_upd_arr_null_guard_agg(
    target: &dyn Display,
    agg_cnt: &dyn Display,
    mb_wh: &dyn Display,
    mb_jsonb_build_arr: &dyn Display,
) -> String {
    format!(
        "case when jsonb_typeof({target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({agg_cnt}) from jsonb_array_elements({target}) as elem {mb_wh}),'[]'::jsonb)) end {mb_jsonb_build_arr}"
    )
}
#[must_use]
pub fn gen_sel_arr_pgn_agg(
    source: &dyn Display,
    content: &dyn Display,
    start: &dyn Display,
    end_v: &dyn Display,
) -> String {
    format!(
        "(case when (jsonb_array_length({source}) = 0) then '[]'::jsonb else (select jsonb_agg(({content})) from jsonb_array_elements((select {source})) with ordinality where ordinality between {start} and {end_v}) end)"
    )
}
#[must_use]
pub fn gen_jsonb_agg_by_id(
    agg_cnt: &dyn Display,
    source: &dyn Display,
    ids: &dyn Display,
) -> String {
    format!(
        "(select jsonb_agg({agg_cnt}) from jsonb_array_elements({source}) as elem where elem->>'id' in ({ids}))"
    )
}
#[must_use]
pub fn parse_strs_to_ts2_vec(v: Vec<String>, uuid: &str) -> Vec<Ts2> {
    v.into_iter()
        .map(|el| el.parse::<Ts2>().unwrap_or_else(|_| panic!("{uuid}")))
        .collect::<Vec<Ts2>>()
}
#[must_use]
pub fn gen_mod_with_pub_use_ts(mod_name: &dyn ToTokens, content_ts: &[Ts2]) -> Ts2 {
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        mod #mod_name {
            #(#content_ts)*
        }
        pub use #mod_name::*;
    }
}
#[must_use]
pub fn cmn_d_ts_builder() -> DTsBuilder {
    DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_clone()
        .d_partial_eq()
        .d_serde_serialize()
        .d_serde_deserialize()
}
#[must_use]
pub fn serde_er_enum_d_ts_builder() -> DTsBuilder {
    DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_serde_serialize()
        .d_serde_deserialize()
        .d_thiserror_error()
        .d_loc_lib_location()
}
#[must_use]
pub fn er_enum_d_ts_builder() -> DTsBuilder {
    DTsBuilder::new()
        .make_pub()
        .d_debug()
        .d_thiserror_error()
        .d_loc_lib_location()
}
