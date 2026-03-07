mod filters;
use enum_extension_lib::EnumExtension;
pub use filters::*;
use gen_quotes::{dq_str, dq_ts};
use macros_helpers::gen_impl_to_err_string_ts;
use naming::{
    AddOprtrSc, AllVrtsDfltOptSomeVecOneElMaxPageSizeSc, AllVrtsDfltOptSomeVecOneElSc,
    ColumnFieldForErMessageSc, ColumnFieldSc, ColumnSc, CreateForQueryUcc,
    CreateIntoPgJsonTypeOptVecWhereLengthEqualSc,
    CreateIntoPgJsonTypeOptVecWhereLengthGreaterThanSc, CreateIntoPgTypeOptVecWhereDimOneEqualSc,
    CreateQbSc, CreateQpSc, CreateSc, CreateTableColumnQpSc, CreateUcc,
    DfltOptSomeVecOneElMaxPageSizeSc, DfltOptSomeVecOneElSc, DisplayPlusToTokens, EqualOprtrUcc,
    FiSc, IncrSc, IsPkSc, JsonbSetAccumulatorSc, JsonbSetPathSc, JsonbSetTargetSc, MutSc,
    NormalizeSc, OptUcc, OptUpdateSc, OptVecCreateSc, PgJsonTypeTestCasesUcc, PgJsonTypeUcc,
    PgTypeEqualOprtrUcc, PgTypeNotPkUcc, PgTypeOptVecWhereGreaterThanTestSc, PgTypeTestCasesUcc,
    PgTypeUcc, PgTypeWhereFilterUcc, PreviousReadMergedWithOptUpdateIntoReadSc, QbSc, QpErUcc,
    QpSc, QuerySc, ReadInnerIntoReadWithNewOrTryNewUnwrapedSc,
    ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc, ReadInnerUcc, ReadOnlyIdsIntoOptVReadInnerSc,
    ReadOnlyIdsMergedWithCreateIntoOptVReadSc,
    ReadOnlyIdsMergedWithCreateIntoOptVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereBetweenSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereContainsElGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereContainsElRegexSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimFourEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimOneEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimThreeEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimTwoEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereInSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereRegexSc,
    ReadOnlyIdsMergedWithCreateIntoReadSc, ReadOnlyIdsMergedWithCreateIntoTableTypeSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc,
    ReadOnlyIdsMergedWithTableTypeIntoPgTypeOptWhereGreaterThanSc, ReadOnlyIdsSc,
    ReadOnlyIdsToOptVReadDfltOptSomeVecOneElSc, ReadOnlyIdsToTwoDimsVecReadInnerSc, ReadOnlyIdsUcc,
    ReadSc, ReadUcc, SelectOnlyCreatedIdsQbSc, SelectOnlyCreatedIdsQpSc, SelectOnlyIdsQpSc,
    SelectOnlyUpdatedIdsQbSc, SelectOnlyUpdatedIdsQpSc, SelectQpSc, SelectUcc, SelfUcc,
    TableTypeSc, TableTypeUcc, UpdateForQueryUcc, UpdateQbSc, UpdateQpSc, UpdateToReadOnlyIdsSc,
    UpdateUcc, VSc, VUcc, ValueSc, WhereUcc,
    param::{SelfCreateUcc, SelfSelectUcc, SelfWhereUcc},
};
use optimal_pack::OptimalPack;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::{Display, EnumIter};
use syn::{Ident, Type};
use token_patterns::{
    AllowClippyArbitrarySourceItemOrdering, Bool, CrateAllEnumVrtsArrDfltOptSomeVecOneEl,
    CrateAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize, CrateDfltOptSomeVecOneEl,
    CrateDfltOptSomeVecOneElMaxPageSize, PgCrudAllEnumVrtsArrDfltOptSomeVecOneEl,
    PgCrudAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize,
    PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneEl,
    PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize, PgCrudCommonDfltOptSomeVecOneEl,
    PgCrudCommonDfltOptSomeVecOneElCall, PgCrudCommonDfltOptSomeVecOneElMaxPageSize,
    PgCrudDfltOptSomeVecOneEl, PgCrudDfltOptSomeVecOneElMaxPageSize, RefStr, StdFmtDisplay,
    StringTs, U64,
};
#[derive(Debug, Clone, OptimalPack)]
pub enum DeriveOrImpl {
    Derive,
    Impl(Ts2),
}
#[derive(Debug, OptimalPack)]
pub enum IsStdrtNotNull {
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
    OptimalPack,
)]
pub enum IsNullable {
    #[default]
    False,
    True,
}
impl IsNullable {
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
    pub const fn not_null_or_nullable_str(&self) -> &str {
        match &self {
            Self::False => "NotNull",
            Self::True => "Nullable",
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
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum Import {
    Crate,
    PgCrud,
    PgCrudCommon,
}
impl Import {
    fn all_vrts_dflt_opt_some_vec_one_el(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateAllEnumVrtsArrDfltOptSomeVecOneEl,
            Self::PgCrud => &PgCrudAllEnumVrtsArrDfltOptSomeVecOneEl,
            Self::PgCrudCommon => &PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneEl,
        }
    }
    fn all_vrts_dflt_opt_some_vec_one_el_max_page_size(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize,
            Self::PgCrud => &PgCrudAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize,
            Self::PgCrudCommon => &PgCrudCommonAllEnumVrtsArrDfltOptSomeVecOneElMaxPageSize,
        }
    }
    fn dflt_opt_some_vec_one_el(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateDfltOptSomeVecOneEl,
            Self::PgCrud => &PgCrudDfltOptSomeVecOneEl,
            Self::PgCrudCommon => &PgCrudCommonDfltOptSomeVecOneEl,
        }
    }
    fn dflt_opt_some_vec_one_el_max_page_size(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateDfltOptSomeVecOneElMaxPageSize,
            Self::PgCrud => &PgCrudDfltOptSomeVecOneElMaxPageSize,
            Self::PgCrudCommon => &PgCrudCommonDfltOptSomeVecOneElMaxPageSize,
        }
    }
    #[must_use]
    pub const fn sc_str(&self) -> &'static str {
        match &self {
            Self::Crate => "crate",
            Self::PgCrud => "pg_crud",
            Self::PgCrudCommon => "pg_crud_common",
        }
    }
    #[must_use]
    pub const fn to_path(&self) -> &'static str {
        match &self {
            Self::Crate => "crate",
            Self::PgCrud => "pg_crud",
            Self::PgCrudCommon => "pg_crud_common",
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
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum ShouldDeriveSchemarsJsonSchema {
    False,
    True,
}
impl ToTokens for ShouldDeriveSchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => quote! {, schemars::JsonSchema}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
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
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IsCreateQbMut {
    False,
    True,
}
impl ToTokens for IsCreateQbMut {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IsSelectQpSelfSelectUsed {
    False,
    True,
}
impl ToTokens for IsSelectQpSelfSelectUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => VSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IsSelectQpColumnFieldForErMessageUsed {
    False,
    True,
}
impl ToTokens for IsSelectQpColumnFieldForErMessageUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => {
                ColumnFieldForErMessageSc.to_tokens(tokens);
            }
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IsSelectQpIsPgTypeUsed {
    False,
    True,
}
impl ToTokens for IsSelectQpIsPgTypeUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => quote! {is_pg_type}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IsUpdateQpSelfUpdateUsed {
    False,
    True,
}
impl ToTokens for IsUpdateQpSelfUpdateUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => VSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IsUpdateQpJsonbSetTargetUsed {
    False,
    True,
}
impl ToTokens for IsUpdateQpJsonbSetTargetUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => JsonbSetTargetSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IsUpdateQbMut {
    False,
    True,
}
impl ToTokens for IsUpdateQbMut {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IsSelectOnlyUpdatedIdsQbMut {
    False,
    True,
}
impl ToTokens for IsSelectOnlyUpdatedIdsQbMut {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IsSelectOnlyCreatedIdsQbMut {
    False,
    True,
}
impl ToTokens for IsSelectOnlyCreatedIdsQbMut {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
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
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum IncrParamUndrscr {
    False,
    True,
}
impl ToTokens for IncrParamUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => IncrSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum ColumnParamUndrscr {
    False,
    True,
}
impl ToTokens for ColumnParamUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => ColumnSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
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
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum ReadOrUpdate {
    Read,
    Update,
}
impl ReadOrUpdate {
    #[must_use]
    pub fn ucc(&self) -> &dyn DisplayPlusToTokens {
        match &self {
            Self::Read => &ReadUcc,
            Self::Update => &UpdateUcc,
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
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
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum PgTypeOrPgJsonType {
    PgJsonType,
    PgType,
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum DefaultSomeOneOrDefaultSomeOneWithMaxPageSize {
    DefaultSomeOne,
    DefaultSomeOneWithMaxPageSize,
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum EqualOrEqualUsingFields {
    Equal,
    EqualUsingFields,
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum EqualOprtrHandle {
    Equal,
    IsNull,
}
impl EqualOprtrHandle {
    #[must_use]
    pub fn to_tokens_path(&self, import: &Import) -> Ts2 {
        let ts = match &self {
            Self::Equal => quote! {Equal},
            Self::IsNull => quote! {IsNull},
        };
        quote! {#import::#EqualOprtrUcc::#ts}
    }
}
//todo mb reuse with other structs
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum Dim {
    One,
    Two,
    Three,
    Four,
}
impl Dim {
    #[must_use]
    pub fn read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_nbr_equal_sc(
        &self,
    ) -> Box<dyn DisplayPlusToTokens> {
        match self {
            Self::One => {
                Box::new(ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimOneEqualSc)
            }
            Self::Two => {
                Box::new(ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimTwoEqualSc)
            }
            Self::Three => {
                Box::new(ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimThreeEqualSc)
            }
            Self::Four => {
                Box::new(ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimFourEqualSc)
            }
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, OptimalPack)]
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
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum CreateQpValueUndrscr {
    False,
    True,
}
impl ToTokens for CreateQpValueUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => VSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum CreateQpIncrUndrscr {
    False,
    True,
}
impl ToTokens for CreateQpIncrUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => IncrSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum CreateQbValueUndrscr {
    False,
    True,
}
impl ToTokens for CreateQbValueUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => VSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum SelectQpValueUndrscr {
    False,
    True,
}
impl ToTokens for SelectQpValueUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => VSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum UpdateQpValueUndrscr {
    False,
    True,
}
impl ToTokens for UpdateQpValueUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => VSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum UpdateQpJsonbSetAccumulatorUndrscr {
    False,
    True,
}
impl ToTokens for UpdateQpJsonbSetAccumulatorUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {jsonb_set_accumulator}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum UpdateQpJsonbSetTargetUndrscr {
    False,
    True,
}
impl ToTokens for UpdateQpJsonbSetTargetUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {jsonb_set_target}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy, OptimalPack)]
pub enum UpdateQpJsonbSetPathUndrscr {
    False,
    True,
}
impl ToTokens for UpdateQpJsonbSetPathUndrscr {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {jsonb_set_path}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
pub fn gen_pg_type_where_ts(
    attrs_ts: &dyn ToTokens,
    vrts: &Vec<&dyn PgFilter>,
    prefix: &dyn ToTokens,
    should_derive_utoipa_to_schema: &ShouldDeriveUtoipaToSchema,
    should_derive_schemars_json_schema: &ShouldDeriveSchemarsJsonSchema,
    is_qb_mut: &IsQbMut,
) -> Ts2 {
    let ident = SelfWhereUcc::from_tokens(&prefix);
    let pg_type_tokens_where_ts = {
        let vrts_ts = vrts.iter().map(|el| {
            let el_ucc = el.ucc();
            let prefix_where_self_ucc = el.prefix_where_self_ucc();
            let opt_type_ts: Option<Ts2> = el.mb_generic();
            let type_ts = opt_type_ts.map_or_else(Ts2::new, |v| quote! {<#v>});
            quote! {#el_ucc(where_filters::#prefix_where_self_ucc #type_ts)}
        });
        quote! {
            #attrs_ts
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_derive_utoipa_to_schema #should_derive_schemars_json_schema, optimal_pack::OptimalPack)]
            pub enum #ident {
                #(#vrts_ts),*
            }
        }
    };
    let impl_pg_type_pg_type_where_filter_for_pg_type_tokens_where_ts =
        impl_pg_type_where_filter_for_ident_ts(
            &quote! {<'lt>},
            &ident,
            &Ts2::new(),
            &IncrParamUndrscr::False,
            &ColumnParamUndrscr::False,
            &AddOprtrUndrscr::False,
            &{
                let vrts_ts = vrts.iter().map(|el| {
                    let el_ucc = el.ucc();
                    quote! {
                        Self::#el_ucc(#VSc) => pg_crud_common::PgTypeWhereFilter::qp(
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
                        Self::#el_ucc(#VSc) => pg_crud_common::PgTypeWhereFilter::qb(
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
            &Import::PgCrudCommon,
        );
    let impl_location_lib_to_err_string_for_pg_type_tokens_where_ts = gen_impl_to_err_string_ts(
        &Ts2::new(),
        &ident,
        &Ts2::new(),
        &quote! {format!("{self:#?}")},
    );
    let impl_all_vrts_dflt_opt_some_vec_one_el_for_pg_type_tokens_where_ts =
        gen_impl_pg_crud_common_all_vrts_dflt_opt_some_vec_one_el_ts(&ident, &{
            let vrts_ts = vrts.iter().map(|el| {
                let el_ucc = el.ucc();
                quote! {Self::#el_ucc(#PgCrudCommonDfltOptSomeVecOneElCall)}
            });
            quote! {vec![#(#vrts_ts),*]}
        });
    quote! {
        #pg_type_tokens_where_ts
        #impl_pg_type_pg_type_where_filter_for_pg_type_tokens_where_ts
        #impl_location_lib_to_err_string_for_pg_type_tokens_where_ts
        #impl_all_vrts_dflt_opt_some_vec_one_el_for_pg_type_tokens_where_ts
    }
}
#[must_use]
pub fn pg_crud_common_qp_er_ts() -> Ts2 {
    quote! {pg_crud_common::#QpErUcc}
}
pub fn gen_struct_ident_dq_ts(v: &dyn Display) -> Ts2 {
    dq_ts(&format!("struct {v}"))
}
pub fn gen_struct_ident_with_nbr_els_dq_ts(ident: &dyn DisplayPlusToTokens, length: usize) -> Ts2 {
    dq_ts(&format!("struct {ident} with {length} els"))
}
pub fn gen_tuple_struct_ident_dq_ts(v: &dyn Display) -> Ts2 {
    dq_ts(&format!("tuple struct {v}"))
}
pub fn gen_sqlx_types_json_type_decl_ts(type_ts: &dyn ToTokens) -> Ts2 {
    quote! {sqlx::types::Json<#type_ts>}
}
pub fn gen_opt_type_decl_ts(type_ts: &dyn ToTokens) -> Ts2 {
    quote! {Option<#type_ts>}
}
pub fn gen_vec_tokens_decl_ts(type_ts: &dyn ToTokens) -> Ts2 {
    quote! {Vec<#type_ts>}
}
pub fn gen_serde_deserialize_dq_ts(
    ident: &dyn DisplayPlusToTokens,
    length: usize,
) -> (Ts2, Ts2, Ts2) {
    let struct_pg_type_ident_where_tokens_dq_ts = gen_struct_ident_dq_ts(ident);
    let struct_pg_type_ident_where_tokens_with_nbr_els_dq_ts =
        gen_struct_ident_with_nbr_els_dq_ts(ident, length);
    let pg_type_ident_where_tokens_dq_ts = dq_ts(&ident);
    (
        struct_pg_type_ident_where_tokens_dq_ts,
        struct_pg_type_ident_where_tokens_with_nbr_els_dq_ts,
        pg_type_ident_where_tokens_dq_ts,
    )
}
pub fn gen_impl_pg_json_type_ts(
    import: &Import,
    ident: &dyn ToTokens,
    table_type_type_ts: &dyn ToTokens,
    create_type_ts: &dyn ToTokens,
    create_for_query_type_ts: &dyn ToTokens,
    select_type_ts: &dyn ToTokens,
    is_select_qp_self_select_used: &IsSelectQpSelfSelectUsed,
    is_select_qp_column_field_for_er_message_used: &IsSelectQpColumnFieldForErMessageUsed,
    is_select_qp_is_pg_type_used: &IsSelectQpIsPgTypeUsed,
    select_qp_ts: &dyn ToTokens,
    where_type_ts: &dyn ToTokens,
    read_type_ts: &dyn ToTokens,
    read_only_ids_type_ts: &dyn ToTokens,
    select_only_ids_qp_ts: &dyn ToTokens,
    read_inner_type_ts: &dyn ToTokens,
    into_inner_ts: &dyn ToTokens,
    update_type_ts: &dyn ToTokens,
    update_type_for_query_ts: &dyn ToTokens,
    update_qp_ts: &dyn ToTokens,
    is_update_qp_self_update_used: &IsUpdateQpSelfUpdateUsed,
    is_update_qp_jsonb_set_target_used: &IsUpdateQpJsonbSetTargetUsed,
    is_update_qb_mut: &IsUpdateQbMut,
    update_qb_ts: &dyn ToTokens,
    select_only_updated_ids_qp_ts: &dyn ToTokens,
    is_select_only_updated_ids_qb_mut: &IsSelectOnlyUpdatedIdsQbMut,
    select_only_updated_ids_qb_ts: &dyn ToTokens,
    select_only_created_ids_qp_ts: &dyn ToTokens,
    is_select_only_created_ids_qb_mut: &IsSelectOnlyCreatedIdsQbMut,
    select_only_created_ids_qb_ts: &dyn ToTokens,
) -> Ts2 {
    let path_ts = quote! {#import ::};
    let reference_mut_u64_ts = quote! {&mut #U64};
    let query_pg_args_ts =
        quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let query_lifetime_pg_args_ts =
        quote! {sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>};
    //todo mb reexport sqlx?
    quote! {
        #AllowClippyArbitrarySourceItemOrdering
        impl #path_ts #PgJsonTypeUcc for #ident {
            type #TableTypeUcc = #table_type_type_ts;
            type #CreateUcc = #create_type_ts;
            type #CreateForQueryUcc = #create_for_query_type_ts;
            type #SelectUcc = #select_type_ts;
            fn #SelectQpSc(
                #is_select_qp_self_select_used: &Self::#SelectUcc,
                #FiSc: #RefStr,
                #ColumnFieldSc: #RefStr,
                #is_select_qp_column_field_for_er_message_used: #RefStr,
                #is_select_qp_is_pg_type_used: #Bool,
            ) -> Result<#StringTs, #path_ts #QpErUcc> {
                #select_qp_ts
            }
            type #WhereUcc = #where_type_ts;
            type #ReadUcc = #read_type_ts;
            type #ReadOnlyIdsUcc = #read_only_ids_type_ts;
            fn #SelectOnlyIdsQpSc(
                #ColumnFieldSc: #RefStr,
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #select_only_ids_qp_ts
            }
            type #ReadInnerUcc = #read_inner_type_ts;
            fn into_inner(#VSc: Self::#ReadUcc) -> Self::#ReadInnerUcc {
                #into_inner_ts
            }
            type #UpdateUcc = #update_type_ts;
            type #UpdateForQueryUcc = #update_type_for_query_ts;
            fn #UpdateQpSc(
                #is_update_qp_self_update_used: &Self::#UpdateForQueryUcc,
                #JsonbSetAccumulatorSc: #RefStr,
                #is_update_qp_jsonb_set_target_used: #RefStr,
                #JsonbSetPathSc: #RefStr,
                #IncrSc: #reference_mut_u64_ts,
            ) -> Result<#StringTs, #path_ts #QpErUcc> {
                #update_qp_ts
            }
            fn #UpdateQbSc(
                #VSc: Self::#UpdateForQueryUcc,
                #is_update_qb_mut #QuerySc: #query_pg_args_ts
            ) -> Result<#query_pg_args_ts, #StringTs> {
                #update_qb_ts
            }
            fn #SelectOnlyUpdatedIdsQpSc(
                #VSc: &Self::#UpdateForQueryUcc,
                #FiSc: #RefStr,
                #ColumnFieldSc: #RefStr,
                #IncrSc: &mut #U64
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #select_only_updated_ids_qp_ts
            }
            fn #SelectOnlyUpdatedIdsQbSc<'lt>(
                #VSc: &'lt Self::#UpdateForQueryUcc,
                #is_select_only_updated_ids_qb_mut #QuerySc: #query_lifetime_pg_args_ts
            ) -> Result<#query_lifetime_pg_args_ts, #StringTs> {
                #select_only_updated_ids_qb_ts
            }
            fn #SelectOnlyCreatedIdsQpSc(
                #VSc: &Self::#CreateForQueryUcc,
                #FiSc: #RefStr,
                #ColumnFieldSc: #RefStr,
                #IncrSc: &mut #U64
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #select_only_created_ids_qp_ts
            }
            fn #SelectOnlyCreatedIdsQbSc<'lt>(
                #VSc: &'lt Self::#CreateForQueryUcc,
                #is_select_only_created_ids_qb_mut #QuerySc: #query_lifetime_pg_args_ts
            ) -> Result<#query_lifetime_pg_args_ts, #StringTs> {
                #select_only_created_ids_qb_ts
            }
        }
    }
}
pub fn gen_impl_dflt_opt_some_vec_one_el_ts(
    impl_generic_ts: &dyn ToTokens,
    import: &Import,
    ident: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import.dflt_opt_some_vec_one_el();
    quote! {
        impl #impl_generic_ts #path_trait_ts for #ident #ident_generic_ts {
            fn #DfltOptSomeVecOneElSc() -> Self {
                #ts
            }
        }
    }
}
pub fn gen_impl_all_vrts_dflt_opt_some_vec_one_el_ts(
    import: &Import,
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import.all_vrts_dflt_opt_some_vec_one_el();
    quote! {
        impl #path_trait_ts for #ident {
            fn #AllVrtsDfltOptSomeVecOneElSc() -> Vec<Self> {
                #ts
            }
        }
    }
}
pub fn gen_impl_dflt_opt_some_vec_one_el_max_page_size_ts(
    impl_generic_ts: &dyn ToTokens,
    import: &Import,
    ident: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import.dflt_opt_some_vec_one_el_max_page_size();
    quote! {
        impl #impl_generic_ts #path_trait_ts for #ident #ident_generic_ts {
            fn #DfltOptSomeVecOneElMaxPageSizeSc() -> Self {
                #ts
            }
        }
    }
}
pub fn gen_impl_all_vrts_dflt_opt_some_vec_one_el_max_page_size_ts(
    import: &Import,
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import.all_vrts_dflt_opt_some_vec_one_el_max_page_size();
    let all_vrts_dflt_opt_some_vec_one_el_max_page_size_sc =
        AllVrtsDfltOptSomeVecOneElMaxPageSizeSc;
    quote! {
        impl #path_trait_ts for #ident {
            fn #all_vrts_dflt_opt_some_vec_one_el_max_page_size_sc() -> Vec<Self> {
                #ts
            }
        }
    }
}
pub fn gen_impl_pg_crud_common_dflt_opt_some_vec_one_el_ts(
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_dflt_opt_some_vec_one_el_ts(&Ts2::new(), &Import::PgCrudCommon, ident, &Ts2::new(), ts)
}
pub fn gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(
    ident: &dyn ToTokens,
    lifetime_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_dflt_opt_some_vec_one_el_ts(&Ts2::new(), &Import::PgCrud, ident, lifetime_ts, ts)
}
pub fn gen_impl_pg_crud_common_all_vrts_dflt_opt_some_vec_one_el_ts(
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_all_vrts_dflt_opt_some_vec_one_el_ts(&Import::PgCrudCommon, ident, ts)
}
pub fn gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_ts(
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_all_vrts_dflt_opt_some_vec_one_el_ts(&Import::PgCrud, ident, ts)
}
pub fn gen_impl_pg_crud_common_dflt_opt_some_vec_one_el_max_page_size_ts(
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_dflt_opt_some_vec_one_el_max_page_size_ts(
        &Ts2::new(),
        &Import::PgCrudCommon,
        ident,
        &Ts2::new(),
        ts,
    )
}
pub fn gen_impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_ts(
    ident: &dyn ToTokens,
    lifetime_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_dflt_opt_some_vec_one_el_max_page_size_ts(
        &Ts2::new(),
        &Import::PgCrud,
        ident,
        lifetime_ts,
        ts,
    )
}
pub fn gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_max_page_size_ts(
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_all_vrts_dflt_opt_some_vec_one_el_max_page_size_ts(&Import::PgCrud, ident, ts)
}
pub fn impl_pg_type_where_filter_for_ident_ts(
    impl_generic_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    incr_param_undrscr: &IncrParamUndrscr,
    column_param_undrscr: &ColumnParamUndrscr,
    add_oprtr_undrscr: &AddOprtrUndrscr,
    qp_ts: &dyn ToTokens,
    is_qb_mut: &IsQbMut,
    qb_ts: &dyn ToTokens,
    import: &Import,
) -> Ts2 {
    quote! {
        #AllowClippyArbitrarySourceItemOrdering
        impl #impl_generic_ts #import ::#PgTypeWhereFilterUcc<'lt> for #ident_ts #ident_generic_ts {
            fn #QpSc(
                &self,
                #incr_param_undrscr: &mut #U64,
                #column_param_undrscr: &dyn #StdFmtDisplay,
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
    ident_table_type_ucc: &dyn ToTokens,
    is_pk_undrscr: &IsPkUndrscr,
    create_table_column_qp_ts: &dyn ToTokens,
    ident_create_ucc: &dyn ToTokens,
    create_qp_v_undrscr: &CreateQpValueUndrscr,
    create_qp_incr_undrscr: &CreateQpIncrUndrscr,
    create_qp_ts: &dyn ToTokens,
    create_qb_v_undrscr: &CreateQbValueUndrscr,
    is_create_qb_mut: &IsCreateQbMut,
    create_qb_ts: &dyn ToTokens,
    ident_select_ucc: &dyn ToTokens,
    select_qp_v_undrscr: &SelectQpValueUndrscr,
    select_qp_ts: &dyn ToTokens,
    ident_where_ucc: &dyn ToTokens,
    ident_read_ucc: &dyn ToTokens,
    normalize_ts: &dyn ToTokens,
    read_only_ids_ts: &dyn ToTokens,
    select_only_ids_qp_ts: &dyn ToTokens,
    ident_read_inner_ucc: &dyn ToTokens,
    into_inner_ts: &dyn ToTokens,
    ident_update_ucc: &dyn ToTokens,
    ident_update_for_query_ucc: &dyn ToTokens,
    update_qp_v_undrscr: &UpdateQpValueUndrscr,
    update_qp_jsonb_set_accumulator_undrscr: &UpdateQpJsonbSetAccumulatorUndrscr,
    update_qp_jsonb_set_target_undrscr: &UpdateQpJsonbSetTargetUndrscr,
    update_qp_jsonb_set_path_undrscr: &UpdateQpJsonbSetPathUndrscr,
    update_qp_ts: &dyn ToTokens,
    is_update_qb_mut: &IsUpdateQbMut,
    update_qb_ts: &dyn ToTokens,
    select_only_updated_ids_qp_ts: &dyn ToTokens,
    is_select_only_updated_ids_qb_mut: &IsSelectOnlyUpdatedIdsQbMut,
    select_only_updated_ids_qb_ts: &dyn ToTokens,
) -> Ts2 {
    let query_pg_args_ts =
        quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    quote! {
        #AllowClippyArbitrarySourceItemOrdering
        impl #import :: #PgTypeUcc for #ident {
            type #TableTypeUcc = #ident_table_type_ucc;
            fn #CreateTableColumnQpSc(#ColumnSc: &dyn #StdFmtDisplay, #is_pk_undrscr: #Bool) -> impl #StdFmtDisplay {
                #create_table_column_qp_ts
            }
            type #CreateUcc = #ident_create_ucc;
            fn #CreateQpSc(
                #create_qp_v_undrscr: &Self::#CreateUcc,
                #create_qp_incr_undrscr: &mut #U64
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #create_qp_ts
            }
            fn #CreateQbSc(
                #create_qb_v_undrscr: Self::#CreateUcc,
                #is_create_qb_mut #QuerySc: #query_pg_args_ts
            ) -> Result<
                #query_pg_args_ts,
                String
            > {
                #create_qb_ts
            }
            type #SelectUcc = #ident_select_ucc;
            fn #SelectQpSc(
                #select_qp_v_undrscr: &Self::#SelectUcc,
                #ColumnSc: #RefStr,
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #select_qp_ts
            }
            type #WhereUcc = #ident_where_ucc;
            type #ReadUcc = #ident_read_ucc;
            fn #NormalizeSc(#VSc: Self::#ReadUcc) -> Self::#ReadUcc {
                #normalize_ts
            }
            type #ReadOnlyIdsUcc = #read_only_ids_ts;
            fn #SelectOnlyIdsQpSc(
                #ColumnSc: #RefStr
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #select_only_ids_qp_ts
            }
            type #ReadInnerUcc = #ident_read_inner_ucc;
            fn into_inner(#VSc: Self::#ReadUcc) -> Self::#ReadInnerUcc {
                #into_inner_ts
            }
            type #UpdateUcc = #ident_update_ucc;
            type #UpdateForQueryUcc = #ident_update_for_query_ucc;
            fn #UpdateQpSc(
                #update_qp_v_undrscr: &Self::#UpdateForQueryUcc,
                #update_qp_jsonb_set_accumulator_undrscr: #RefStr,
                #update_qp_jsonb_set_target_undrscr: #RefStr,
                #update_qp_jsonb_set_path_undrscr: #RefStr,
                #IncrSc: &mut #U64
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #update_qp_ts
            }
            fn #UpdateQbSc(
                #VSc: Self::#UpdateForQueryUcc,
                #is_update_qb_mut #QuerySc: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<
                sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                String
            > {
                #update_qb_ts
            }
            fn #SelectOnlyUpdatedIdsQpSc(
                #VSc: &Self::#UpdateForQueryUcc,
                #ColumnSc: #RefStr,
                #IncrSc: &mut #U64,
            ) -> Result<#StringTs, #import ::#QpErUcc> {
                #select_only_updated_ids_qp_ts
            }
            fn #SelectOnlyUpdatedIdsQbSc<'lt>(
                #VSc: &'lt Self::#UpdateForQueryUcc,
                #is_select_only_updated_ids_qb_mut #QuerySc: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
                #select_only_updated_ids_qb_ts
            }
        }
    }
}
pub fn gen_impl_pg_type_not_pk_for_ident_ts(import: &Import, ident: &dyn ToTokens) -> Ts2 {
    let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
    quote! {
        #AllowClippyArbitrarySourceItemOrdering
        impl #import::#PgTypeNotPkUcc for #ident {
            type #PgTypeUcc = Self;
            type #CreateUcc = #ident_create_ucc;
        }
    }
}
// fn gen_read_only_ids_merged_with_create_into_where_method_ts(
//     import: &Import,
//     method_name_ts: &dyn ToTokens,
//     ts: &dyn ToTokens,
//     pg_type_or_pg_json_type: &PgTypeOrPgJsonType,
// ) -> Ts2 {
//     let self_ucc = SelfUcc;
//     let read_only_ids_sc = ReadOnlyIdsSc;
//     let read_only_ids_ucc = ReadOnlyIdsUcc;
//     let create_sc = CreateSc;
//     let create_ucc = CreateUcc;
//     let where_ucc = WhereUcc;
//     let self_pg_type_or_pg_json_type_as_pg_json_type_ts = {
//         let pg_type_or_pg_json_type_ts: &dyn ToTokens = match &pg_type_or_pg_json_type {
//             PgTypeOrPgJsonType::PgType => &PgTypeUcc,
//             PgTypeOrPgJsonType::PgJsonType => &PgJsonTypeUcc,
//         };
//         quote! {
//             <#SelfUcc::#pg_type_or_pg_json_type_ts as #import::#pg_type_or_pg_json_type_ts>
//         }
//     };
//     quote!{
//         fn #method_name_ts(
//             #ReadOnlyIdsSc: #self_pg_type_or_pg_json_type_as_pg_json_type_ts::#ReadOnlyIdsUcc,
//             #CreateSc: #self_pg_type_or_pg_json_type_as_pg_json_type_ts::#CreateUcc
//         ) -> Vec<#self_pg_type_or_pg_json_type_as_pg_json_type_ts::#WhereUcc> {
//             #ts
//         }
//     }
// }
fn gen_opt_vec_create_ts(path_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #OptVecCreateSc() -> Option<Vec<#path_ts::#CreateUcc>> {
            #ts
        }
    }
}
fn gen_read_only_ids_to_two_dims_vec_read_inner_ts(
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsToTwoDimsVecReadInnerSc(
            #ReadOnlyIdsSc: &#path_ts::#ReadOnlyIdsUcc
        ) -> Vec<Vec<#path_ts::#ReadInnerUcc>> {
            #ts
        }
    }
}
fn gen_read_inner_into_read_with_new_or_try_new_unwraped_ts(
    type_ts: &dyn ToTokens,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadInnerIntoReadWithNewOrTryNewUnwrapedSc(
            #VSc: #type_ts
        ) -> #path_ts::#ReadUcc {
            #ts
        }
    }
}
fn gen_read_inner_into_update_with_new_or_try_new_unwraped_ts(
    type_ts: &dyn ToTokens,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc(#VSc: #type_ts) -> #path_ts::#UpdateUcc {
            #ts
        }
    }
}
fn gen_update_to_read_only_ids_ts(path_ts: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #UpdateToReadOnlyIdsSc(
            #VSc: &#path_ts::#UpdateUcc
        ) -> #path_ts::#ReadOnlyIdsUcc {
            #ts
        }
    }
}
fn gen_read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsToOptVReadDfltOptSomeVecOneElSc(
            #VSc: &#path_ts::#ReadOnlyIdsUcc
        ) -> Option<#import::#VUcc<#path_ts::#ReadUcc>> {
            #ts
        }
    }
}
fn gen_previous_read_merged_with_opt_update_into_read_ts(
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #PreviousReadMergedWithOptUpdateIntoReadSc(
            #ReadSc: #path_ts::#ReadUcc,
            #OptUpdateSc: Option<#path_ts::#UpdateUcc>,
        ) -> #path_ts::#ReadUcc {
            #ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_read_ts(
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoReadSc(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> #path_ts::#ReadUcc {
            #ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_opt_v_read_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoOptVReadSc(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import::#VUcc<#path_ts::#ReadUcc>> {
            #ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_table_type_ts(
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoTableTypeSc(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> #path_ts::#TableTypeUcc {
            #ts
        }
    }
}
pub fn gen_read_only_ids_merged_with_create_into_where_equal_ts(
    read_only_ids_ts: &dyn ToTokens,
    create_ts: &dyn ToTokens,
    where_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoWhereEqualSc(
            #ReadOnlyIdsSc: #read_only_ids_ts,
            #CreateSc: #create_ts
        ) -> #where_ts {
            #ts
        }
    }
}
pub fn gen_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
    import: &Import,
    read_only_ids_ts: &dyn ToTokens,
    create_ts: &dyn ToTokens,
    where_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc(
            #ReadOnlyIdsSc: #read_only_ids_ts,
            #CreateSc: #create_ts
        ) -> #import::NotEmptyUniqueVec<#where_ts> {
            #ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_vec_or_opt_vec_where_equal_to_json_field_pg_type_or_pg_json_type_ts(
    import: Import,
    read_only_ids_ts: &dyn ToTokens,
    create_ts: &dyn ToTokens,
    where_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
    pg_type_or_pg_json_type: PgTypeOrPgJsonType,
) -> Ts2 {
    let return_type_ts = {
        let return_type_handle_ts = quote! {#import::NotEmptyUniqueVec<#where_ts>};
        match &pg_type_or_pg_json_type {
            PgTypeOrPgJsonType::PgType => gen_opt_type_decl_ts(&return_type_handle_ts),
            PgTypeOrPgJsonType::PgJsonType => return_type_handle_ts,
        }
    };
    let name_ts: &dyn ToTokens = match &pg_type_or_pg_json_type {
        PgTypeOrPgJsonType::PgType => &ReadOnlyIdsMergedWithCreateIntoOptVecWhereEqualToJsonFieldSc,
        PgTypeOrPgJsonType::PgJsonType => {
            &ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc
        }
    };
    quote! {
        fn #name_ts(
            #ReadOnlyIdsSc: #read_only_ids_ts,
            #CreateSc: #create_ts
        ) -> #return_type_ts {
            #ts
        }
    }
}
pub fn gen_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts(
    import: Import,
    read_only_ids_ts: &dyn ToTokens,
    create_ts: &dyn ToTokens,
    where_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_vec_or_opt_vec_where_equal_to_json_field_pg_type_or_pg_json_type_ts(
        import,
        &read_only_ids_ts,
        &create_ts,
        &where_ts,
        &ts,
        PgTypeOrPgJsonType::PgJsonType
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_nbr_equal_ts(
    import: Import,
    name_ts: &dyn ToTokens,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #name_ts(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import::NotEmptyUniqueVec<#path_ts::#WhereUcc>> {
            #ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_nbr_equal_ts(
        import,
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimOneEqualSc,
        &path_ts,
        &ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_nbr_equal_ts(
        import,
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimTwoEqualSc,
        &path_ts,
        &ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_nbr_equal_ts(
        import,
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimThreeEqualSc,
        &path_ts,
        &ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_nbr_equal_ts(
        import,
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereDimFourEqualSc,
        &path_ts,
        &ts,
    )
}
fn gen_create_into_pg_json_type_opt_vec_where_length_equal_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #CreateIntoPgJsonTypeOptVecWhereLengthEqualSc(
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import::NotEmptyUniqueVec<#path_ts::#WhereUcc>> {
            #ts
        }
    }
}
fn gen_create_into_pg_json_type_opt_vec_where_length_greater_than_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #CreateIntoPgJsonTypeOptVecWhereLengthGreaterThanSc(
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import::NotEmptyUniqueVec<#path_ts::#WhereUcc>> {
            #ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_not_empty_unique_vec_single_or_multiple_where_ts(
    method_name_ts: &dyn ToTokens,
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #method_name_ts(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import::NotEmptyUniqueVec<#import::SingleOrMultiple<#path_ts::#WhereUcc>>> {
            #ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereGreaterThanSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereBetweenSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereInSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereRegexSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereContainsElGreaterThanSc,
        import,
        path_ts,
        ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts(
    import: Import,
    path_ts: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_opt_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereContainsElRegexSc,
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
    opt_vec_create_ts: &dyn ToTokens,
    read_only_ids_to_two_dims_vec_read_inner_ts: &dyn ToTokens,
    read_inner_into_read_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    read_inner_into_update_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    update_to_read_only_ids_ts: &dyn ToTokens,
    read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts: &dyn ToTokens,
    previous_read_merged_with_opt_update_into_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_opt_v_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_table_type_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_where_equal_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_ts: &dyn ToTokens,
    create_into_pg_type_opt_vec_where_dim_one_equal_ts: &dyn ToTokens,
    pg_type_opt_vec_where_greater_than_test_ts: &dyn ToTokens,
    read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_dim_one_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_dim_two_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_dim_three_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_dim_four_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_length_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_length_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts: &dyn ToTokens,
) -> Ts2 {
    let self_pg_type_as_pg_type_ts = quote! {<#SelfUcc::#PgTypeUcc as #import::#PgTypeUcc>};
    let self_pg_type_as_pg_type_read_only_ids_ts =
        quote! {#self_pg_type_as_pg_type_ts::#ReadOnlyIdsUcc};
    let self_pg_type_as_pg_type_create_ts = quote! {#self_pg_type_as_pg_type_ts::#CreateUcc};
    let self_pg_type_as_pg_type_where_ts = quote! {#self_pg_type_as_pg_type_ts::#WhereUcc};
    let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
    let opt_vec_create_ts_2d58042f =
        gen_opt_vec_create_ts(&self_pg_type_as_pg_type_ts, &opt_vec_create_ts);
    let read_only_ids_to_two_dims_vec_read_inner_ts_513b8046 =
        gen_read_only_ids_to_two_dims_vec_read_inner_ts(
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_to_two_dims_vec_read_inner_ts,
        );
    let read_inner_into_read_with_new_or_try_new_unwraped_ts_affc58f5 =
        gen_read_inner_into_read_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &read_inner_into_read_with_new_or_try_new_unwraped_ts,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_ts_c38e6621 =
        gen_read_inner_into_update_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &read_inner_into_update_with_new_or_try_new_unwraped_ts,
        );
    let update_to_read_only_ids_ts_ee17b828 =
        gen_update_to_read_only_ids_ts(&self_pg_type_as_pg_type_ts, &update_to_read_only_ids_ts);
    let read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts_18ef45e8 =
        gen_read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts,
        );
    let previous_read_merged_with_opt_update_into_read_ts_c48b8ede =
        gen_previous_read_merged_with_opt_update_into_read_ts(
            &self_pg_type_as_pg_type_ts,
            &previous_read_merged_with_opt_update_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_read_ts_df48e4b7 =
        gen_read_only_ids_merged_with_create_into_read_ts(
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_opt_v_read_ts_8b7e9688 =
        gen_read_only_ids_merged_with_create_into_opt_v_read_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_opt_v_read_ts,
        );
    let read_only_ids_merged_with_create_into_table_type_ts_f227db63 =
        gen_read_only_ids_merged_with_create_into_table_type_ts(
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_table_type_ts,
        );
    let read_only_ids_merged_with_create_into_where_equal_ts_dcde170f =
        gen_read_only_ids_merged_with_create_into_where_equal_ts(
            &self_pg_type_as_pg_type_read_only_ids_ts,
            &self_pg_type_as_pg_type_create_ts,
            &self_pg_type_as_pg_type_where_ts,
            &read_only_ids_merged_with_create_into_where_equal_ts,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts_076c6ebd =
        gen_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
            import,
            &self_pg_type_as_pg_type_read_only_ids_ts,
            &self_pg_type_as_pg_type_create_ts,
            &self_pg_type_as_pg_type_where_ts,
            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
        );
    let read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_ts_948ce180 = gen_read_only_ids_merged_with_create_into_vec_or_opt_vec_where_equal_to_json_field_pg_type_or_pg_json_type_ts(
        *import,
        &self_pg_type_as_pg_type_read_only_ids_ts,
        &self_pg_type_as_pg_type_create_ts,
        &self_pg_type_as_pg_type_where_ts,
        &read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_ts,
        PgTypeOrPgJsonType::PgType,
    );
    let create_into_pg_type_opt_vec_where_dim_one_equal_sc =
        CreateIntoPgTypeOptVecWhereDimOneEqualSc;
    let read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_sc =
        ReadOnlyIdsMergedWithTableTypeIntoPgTypeOptWhereGreaterThanSc;
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts_33093313 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_opt_vec_where_dim_one_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts_9522c7a5 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_opt_vec_where_dim_two_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts_81696b49 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_opt_vec_where_dim_three_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts_2631549b =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_opt_vec_where_dim_four_equal_ts,
        );
    let create_into_pg_json_type_opt_vec_where_length_equal_ts_34b74d66 =
        gen_create_into_pg_json_type_opt_vec_where_length_equal_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_opt_vec_where_length_equal_ts,
        );
    let create_into_pg_json_type_opt_vec_where_length_greater_than_ts_b196c70f =
        gen_create_into_pg_json_type_opt_vec_where_length_greater_than_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_opt_vec_where_length_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts_498680a8 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts_b685b98f =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts_ac82295e =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts_bfe19de1 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts_8d2a6cb8 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts_ff2d3a76 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts(
            *import,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts,
        );
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySourceItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import::#PgTypeTestCasesUcc for #ident {
            type #PgTypeUcc = #SelfUcc;
            type #SelectUcc = #ident_select_ucc;
            #opt_vec_create_ts_2d58042f
            #read_only_ids_to_two_dims_vec_read_inner_ts_513b8046
            #read_inner_into_read_with_new_or_try_new_unwraped_ts_affc58f5
            #read_inner_into_update_with_new_or_try_new_unwraped_ts_c38e6621
            #update_to_read_only_ids_ts_ee17b828
            #read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts_18ef45e8
            #previous_read_merged_with_opt_update_into_read_ts_c48b8ede
            #read_only_ids_merged_with_create_into_read_ts_df48e4b7
            #read_only_ids_merged_with_create_into_opt_v_read_ts_8b7e9688
            #read_only_ids_merged_with_create_into_table_type_ts_f227db63
            #read_only_ids_merged_with_create_into_where_equal_ts_dcde170f
            #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts_076c6ebd
            #read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_ts_948ce180
            fn #create_into_pg_type_opt_vec_where_dim_one_equal_sc(
                #CreateSc: #self_pg_type_as_pg_type_ts::#CreateUcc
            ) -> Option<#import::NotEmptyUniqueVec<#self_pg_type_as_pg_type_ts::#WhereUcc>> {
                #create_into_pg_type_opt_vec_where_dim_one_equal_ts
            }
            fn #PgTypeOptVecWhereGreaterThanTestSc() -> Option<
                #import::NotEmptyUniqueVec<
                    #import::PgTypeGreaterThanTest<
                        #SelfUcc::#PgTypeUcc
                    >
                >
            > {
                #pg_type_opt_vec_where_greater_than_test_ts
            }
            fn #read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_sc(
                greater_than_vrt: #import::PgTypeGreaterThanVrt,
                #ReadOnlyIdsSc: #self_pg_type_as_pg_type_ts::#ReadOnlyIdsUcc,
                #TableTypeSc: #self_pg_type_as_pg_type_ts::#TableTypeUcc,
            ) -> Option<#self_pg_type_as_pg_type_ts::#WhereUcc> {
                #read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_ts
            }
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts_33093313
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts_9522c7a5
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts_81696b49
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts_2631549b
            #create_into_pg_json_type_opt_vec_where_length_equal_ts_34b74d66
            #create_into_pg_json_type_opt_vec_where_length_greater_than_ts_b196c70f
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts_498680a8
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts_b685b98f
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts_ac82295e
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts_bfe19de1
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts_8d2a6cb8
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts_ff2d3a76
        }
    }
}
pub fn gen_impl_pg_json_type_test_cases_for_ident_ts(
    cfg_ts: &dyn ToTokens,
    import: &Import,
    type_ts: &dyn ToTokens,
    ident: &dyn ToTokens,
    opt_vec_create_ts: &dyn ToTokens,
    read_only_ids_to_two_dims_vec_read_inner_ts: &dyn ToTokens,
    read_inner_into_read_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    read_inner_into_update_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    read_only_ids_into_opt_v_read_inner_ts: &dyn ToTokens,
    update_to_read_only_ids_ts: &dyn ToTokens,
    read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts: &dyn ToTokens,
    previous_read_merged_with_opt_update_into_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_opt_v_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_table_type_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_where_equal_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_dim_one_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_dim_two_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_dim_three_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_dim_four_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_length_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_opt_vec_where_length_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts: &dyn ToTokens,
) -> Ts2 {
    let self_pg_json_type_as_pg_json_type_ts =
        quote! {<#SelfUcc::#PgJsonTypeUcc as #import::#PgJsonTypeUcc>};
    let self_pg_json_type_as_pg_json_type_read_only_ids_ts =
        quote! {#self_pg_json_type_as_pg_json_type_ts::#ReadOnlyIdsUcc};
    let self_pg_json_type_as_pg_json_type_create_ts =
        quote! {#self_pg_json_type_as_pg_json_type_ts::#CreateUcc};
    let self_pg_json_type_as_pg_json_type_where_ts =
        quote! {#self_pg_json_type_as_pg_json_type_ts::#WhereUcc};
    let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
    let opt_vec_create_ts_a442630a =
        gen_opt_vec_create_ts(&self_pg_json_type_as_pg_json_type_ts, &opt_vec_create_ts);
    let read_only_ids_to_two_dims_vec_read_inner_ts_da1a7cf8 =
        gen_read_only_ids_to_two_dims_vec_read_inner_ts(
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_to_two_dims_vec_read_inner_ts,
        );
    let read_inner_into_read_with_new_or_try_new_unwraped_ts_ccead2b6 =
        gen_read_inner_into_read_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_inner_into_read_with_new_or_try_new_unwraped_ts,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_ts_b45cde72 =
        gen_read_inner_into_update_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_inner_into_update_with_new_or_try_new_unwraped_ts,
        );
    let update_to_read_only_ids_ts_d7e0cbf0 = gen_update_to_read_only_ids_ts(
        &self_pg_json_type_as_pg_json_type_ts,
        &update_to_read_only_ids_ts,
    );
    let read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts_f5d1b395 =
        gen_read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts,
        );
    let previous_read_merged_with_opt_update_into_read_ts_ab0384b9 =
        gen_previous_read_merged_with_opt_update_into_read_ts(
            &self_pg_json_type_as_pg_json_type_ts,
            &previous_read_merged_with_opt_update_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_read_ts_7df2fa10 =
        gen_read_only_ids_merged_with_create_into_read_ts(
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_opt_v_read_ts_1f54e2bf =
        gen_read_only_ids_merged_with_create_into_opt_v_read_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_opt_v_read_ts,
        );
    let read_only_ids_merged_with_create_into_table_type_ts_b605767e =
        gen_read_only_ids_merged_with_create_into_table_type_ts(
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_table_type_ts,
        );
    let read_only_ids_merged_with_create_into_where_equal_ts_1009eb88 =
        gen_read_only_ids_merged_with_create_into_where_equal_ts(
            &self_pg_json_type_as_pg_json_type_read_only_ids_ts,
            &self_pg_json_type_as_pg_json_type_create_ts,
            &self_pg_json_type_as_pg_json_type_where_ts,
            &read_only_ids_merged_with_create_into_where_equal_ts,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts_876245c5 =
        gen_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
            import,
            &self_pg_json_type_as_pg_json_type_read_only_ids_ts,
            &self_pg_json_type_as_pg_json_type_create_ts,
            &self_pg_json_type_as_pg_json_type_where_ts,
            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts_11560e7f =
        gen_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_read_only_ids_ts,
            &self_pg_json_type_as_pg_json_type_create_ts,
            &self_pg_json_type_as_pg_json_type_where_ts,
            &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts_aaaa85b2 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_opt_vec_where_dim_one_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts_6da8ece7 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_opt_vec_where_dim_two_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts_6b473c12 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_opt_vec_where_dim_three_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts_b427508f =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_opt_vec_where_dim_four_equal_ts,
        );
    let create_into_pg_json_type_opt_vec_where_length_equal_ts_5266addf =
        gen_create_into_pg_json_type_opt_vec_where_length_equal_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_opt_vec_where_length_equal_ts,
        );
    let create_into_pg_json_type_opt_vec_where_length_greater_than_ts_93196cce =
        gen_create_into_pg_json_type_opt_vec_where_length_greater_than_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_opt_vec_where_length_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts_e0be3ff7 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts_9bdb444a =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts_09ea1f4b =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts_1b1057eb =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts_5dc0a6c8 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts_972d3e87 =
        gen_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts(
            *import,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts,
        );
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySourceItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import::#PgJsonTypeTestCasesUcc for #ident {
            type #PgJsonTypeUcc = #SelfUcc;
            type #SelectUcc = #ident_select_ucc;
            #opt_vec_create_ts_a442630a
            #read_only_ids_to_two_dims_vec_read_inner_ts_da1a7cf8
            #read_inner_into_read_with_new_or_try_new_unwraped_ts_ccead2b6
            #read_inner_into_update_with_new_or_try_new_unwraped_ts_b45cde72
            fn #ReadOnlyIdsIntoOptVReadInnerSc(
                #VSc: #self_pg_json_type_as_pg_json_type_ts::#ReadOnlyIdsUcc
            ) -> Option<#import::#VUcc<#self_pg_json_type_as_pg_json_type_ts::#ReadInnerUcc>> {
                #read_only_ids_into_opt_v_read_inner_ts
            }
            #update_to_read_only_ids_ts_d7e0cbf0
            #read_only_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts_f5d1b395
            #previous_read_merged_with_opt_update_into_read_ts_ab0384b9
            #read_only_ids_merged_with_create_into_read_ts_7df2fa10
            #read_only_ids_merged_with_create_into_opt_v_read_ts_1f54e2bf
            #read_only_ids_merged_with_create_into_table_type_ts_b605767e
            #read_only_ids_merged_with_create_into_where_equal_ts_1009eb88
            #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts_876245c5
            #read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts_11560e7f
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts_aaaa85b2
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts_6da8ece7
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts_6b473c12
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts_b427508f
            #create_into_pg_json_type_opt_vec_where_length_equal_ts_5266addf
            #create_into_pg_json_type_opt_vec_where_length_greater_than_ts_93196cce
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts_e0be3ff7
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts_9bdb444a
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts_09ea1f4b
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts_1b1057eb
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts_5dc0a6c8
            #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts_972d3e87
        }
    }
}
#[must_use]
pub fn pg_crud_common_qp_er_checked_add_init_ts() -> Ts2 {
    quote! {pg_crud_common::QpEr::CheckedAdd { loc: location_lib::loc!() }}
}
pub fn gen_impl_crate_is_string_empty_for_ident_ts(ident: &dyn ToTokens, ts: &dyn ToTokens) -> Ts2 {
    quote! {
        impl pg_crud_common::IsStringEmpty for #ident {
            fn is_string_empty(&self) -> bool {
                #ts
            }
        }
    }
}
pub fn gen_match_try_new_in_deserialize_ts(ident: &dyn ToTokens, init_ts: &dyn ToTokens) -> Ts2 {
    quote! {
        match #ident::try_new(#init_ts) {
            Ok(v) => Ok(v),
            Err(er) => Err(serde::de::Error::custom(format!("{er:?}")))
        }
    }
}
pub fn gen_impl_serde_deserialize_for_struct_ts(
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
    fn gen_undrscr_undrscr_field_i_handle_ts(i: usize) -> Ts2 {
        format!("{}_handle", gen_undrscr_undrscr_field_i_str(i))
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
            let field_i_handle_ts = gen_undrscr_undrscr_field_i_handle_ts(i);
            let type_ts = gen_type_ts(el_ident, el_type);
            let struct_ident_opts_with_dq_ts = dq_ts(&format!("struct {ident} with {len} els"));
            quote! {
                let Some(#field_i_handle_ts) = serde::de::SeqAccess::next_element::<#type_ts>(&mut __seq)? else {
                    return Err(serde::de::Error::invalid_length(0usize, &#struct_ident_opts_with_dq_ts));
                };
            }
        });
        quote! {#(#ts)*}
    };
    let match_try_new_in_deserialize_ts = gen_match_try_new_in_deserialize_ts(&ident, &{
        let fields_ts = (0..len).map(gen_undrscr_undrscr_field_i_handle_ts);
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
            let field_i_handle_ts = gen_undrscr_undrscr_field_i_handle_ts(i);
            let fi_dq_ts = dq_ts(&el);
            quote! {
                let #field_i_handle_ts = match #field_i_ts {
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
        #AllowClippyArbitrarySourceItemOrdering
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
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
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
                            #match_try_new_in_deserialize_ts
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
                            #match_try_new_in_deserialize_ts
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
                            lifetime: _serde::__private228::PhantomData,
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
pub fn gen_v_decl_ts(import: &Import, ts: &dyn ToTokens) -> Ts2 {
    quote! {#import::V<#ts>}
}
pub fn gen_v_init_ts(import: &Import, ts: &dyn ToTokens) -> Ts2 {
    quote! {#import::V { #VSc: #ts }}
}
pub fn impl_pg_type_equal_oprtr_for_ident_ts(
    import: &Import,
    ident: &dyn ToTokens,
    ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl #import::#PgTypeEqualOprtrUcc for #ident {
            fn oprtr(&self) -> #import::#EqualOprtrUcc {
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
pub fn gen_jsonb_build_object(v: &dyn Display) -> String {
    format!("jsonb_build_object({v})")
}
#[must_use]
pub fn gen_jsonb_build_object_v(v: &dyn Display) -> String {
    gen_jsonb_build_object(&format!("'v',{v}"))
}
