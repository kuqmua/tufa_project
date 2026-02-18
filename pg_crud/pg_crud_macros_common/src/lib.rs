mod filters;
use enum_extension_lib::EnumExtension;
pub use filters::*;
use gen_quotes::{dq_str, dq_ts};
use macros_helpers::gen_impl_to_err_string_ts;
use naming::{
    AllVariantsDefaultOptionSomeVecOneElMaxPageSizeSc, AllVariantsDefaultOptionSomeVecOneElSc,
    ColumnNameAndMaybeFieldGetterForErrorMessageSc, ColumnNameAndMaybeFieldGetterSc, ColumnSc,
    CreateForQueryUcc, CreateIntoPgJsonTypeOptionVecWhereLengthEqualSc,
    CreateIntoPgJsonTypeOptionVecWhereLengthGreaterThanSc,
    CreateIntoPgTypeOptionVecWhereDimOneEqualSc, CreateQueryBindSc, CreateQueryPartSc, CreateSc,
    CreateTableColumnQueryPartSc, CreateUcc, DefaultOptionSomeVecOneElMaxPageSizeSc,
    DefaultOptionSomeVecOneElSc, DisplayPlusToTokens, EqualOperatorUcc, FieldIdentSc, FieldSc,
    IncrementSc, IsNeedToAddLogicalOperatorSc, IsPrimaryKeySc, JsonbSetAccumulatorSc,
    JsonbSetPathSc, JsonbSetTargetSc, MutSc, NormalizeSc, OptionUcc, OptionUpdateSc,
    OptionVecCreateSc, PgJsonTypeTestCasesUcc, PgJsonTypeUcc, PgTypeEqualOperatorUcc,
    PgTypeNotPrimaryKeyUcc, PgTypeOptionVecWhereGreaterThanTestSc, PgTypeTestCasesUcc, PgTypeUcc,
    PgTypeWhereFilterUcc, PreviousReadMergedWithOptionUpdateIntoReadSc, QueryBindSc,
    QueryPartErrorUcc, QueryPartSc, QuerySc, ReadInnerIntoReadWithNewOrTryNewUnwrapedSc,
    ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc, ReadInnerUcc,
    ReadOnlyIdsIntoOptionValueReadInnerSc, ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc,
    ReadOnlyIdsMergedWithCreateIntoOptionVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereBetweenSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimFourEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimOneEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimThreeEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimTwoEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereInSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoReadSc, ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc,
    ReadOnlyIdsMergedWithTableTypeDeclarationIntoPgTypeOptionWhereGreaterThanSc, ReadOnlyIdsSc,
    ReadOnlyIdsToOptionValueReadDefaultOptionSomeVecOneElSc, ReadOnlyIdsToTwoDimalVecReadInnerSc,
    ReadOnlyIdsUcc, ReadSc, ReadUcc, SelectOnlyCreatedIdsQueryBindSc,
    SelectOnlyCreatedIdsQueryPartSc, SelectOnlyIdsQueryPartSc, SelectOnlyUpdatedIdsQueryBindSc,
    SelectOnlyUpdatedIdsQueryPartSc, SelectQueryPartSc, SelectUcc, SelfUcc, TableTypeDeclarationSc,
    TableTypeDeclarationUcc, UpdateForQueryUcc, UpdateQueryBindSc, UpdateQueryPartSc,
    UpdateToReadOnlyIdsSc, UpdateUcc, ValueSc, ValueUcc, WhereUcc,
    parameter::{SelfCreateUcc, SelfSelectUcc, SelfWhereUcc},
};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::{Display, EnumIter};
use syn::{Ident, Type};
use token_patterns::{
    AllowClippyArbitrarySourceItemOrdering, Bool,
    CrateAllEnumVariantsArrayDefaultOptionSomeVecOneEl,
    CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize, CrateDefaultOptionSomeVecOneEl,
    CrateDefaultOptionSomeVecOneElMaxPageSize, PgCrudAllEnumVariantsArrayDefaultOptionSomeVecOneEl,
    PgCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize,
    PgCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneEl,
    PgCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize,
    PgCrudCommonDefaultOptionSomeVecOneEl, PgCrudCommonDefaultOptionSomeVecOneElCall,
    PgCrudCommonDefaultOptionSomeVecOneElMaxPageSize, PgCrudDefaultOptionSomeVecOneEl,
    PgCrudDefaultOptionSomeVecOneElMaxPageSize, RefStr, StdFmtDisplay, StringTs, U64,
};
#[derive(Debug, Clone)]
pub enum DeriveOrImpl {
    Derive,
    Impl(Ts2),
}
#[derive(Debug)]
pub enum IsStandartNotNull {
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
)]
pub enum IsNullable {
    #[default]
    False,
    True,
}
impl IsNullable {
    #[must_use]
    pub fn maybe_option_wrap(&self, content_ts: Ts2) -> Ts2 {
        match &self {
            Self::False => content_ts,
            Self::True => quote! {Option<#content_ts>},
        }
    }
    #[must_use]
    pub fn maybe_some_wrap(&self, content_ts: Ts2) -> Ts2 {
        match &self {
            Self::False => content_ts,
            Self::True => quote! {Some(#content_ts)},
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
            Self::True => String::from("StdOptionOption"),
        }
    }
    #[must_use]
    pub fn rust(&self) -> &'static dyn Display {
        match &self {
            Self::False => &"",
            Self::True => &OptionUcc,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ImportPath {
    Crate,
    PgCrud,
    PgCrudCommon,
}
impl ImportPath {
    fn all_variants_default_option_some_vec_one_el(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateAllEnumVariantsArrayDefaultOptionSomeVecOneEl,
            Self::PgCrud => &PgCrudAllEnumVariantsArrayDefaultOptionSomeVecOneEl,
            Self::PgCrudCommon => &PgCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneEl,
        }
    }
    fn all_variants_default_option_some_vec_one_el_max_page_size(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize,
            Self::PgCrud => &PgCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize,
            Self::PgCrudCommon => {
                &PgCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize
            }
        }
    }
    fn default_option_some_vec_one_el(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateDefaultOptionSomeVecOneEl,
            Self::PgCrud => &PgCrudDefaultOptionSomeVecOneEl,
            Self::PgCrudCommon => &PgCrudCommonDefaultOptionSomeVecOneEl,
        }
    }
    fn default_option_some_vec_one_el_max_page_size(&self) -> &dyn ToTokens {
        match &self {
            Self::Crate => &CrateDefaultOptionSomeVecOneElMaxPageSize,
            Self::PgCrud => &PgCrudDefaultOptionSomeVecOneElMaxPageSize,
            Self::PgCrudCommon => &PgCrudCommonDefaultOptionSomeVecOneElMaxPageSize,
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
impl ToTokens for ImportPath {
    fn to_tokens(&self, tokens: &mut Ts2) {
        self.sc_str()
            .parse::<Ts2>()
            .expect("d8636ee5")
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub enum IsCreateQueryBindMutable {
    False,
    True,
}
impl ToTokens for IsCreateQueryBindMutable {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectQueryPartSelfSelectUsed {
    False,
    True,
}
impl ToTokens for IsSelectQueryPartSelfSelectUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => ValueSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed {
    False,
    True,
}
impl ToTokens for IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => {
                ColumnNameAndMaybeFieldGetterForErrorMessageSc.to_tokens(tokens);
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectQueryPartIsPgTypeUsed {
    False,
    True,
}
impl ToTokens for IsSelectQueryPartIsPgTypeUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => quote! {is_pg_type}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsUpdateQueryPartSelfUpdateUsed {
    False,
    True,
}
impl ToTokens for IsUpdateQueryPartSelfUpdateUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => ValueSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsUpdateQueryPartJsonbSetTargetUsed {
    False,
    True,
}
impl ToTokens for IsUpdateQueryPartJsonbSetTargetUsed {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => JsonbSetTargetSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsUpdateQueryBindMutable {
    False,
    True,
}
impl ToTokens for IsUpdateQueryBindMutable {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectOnlyUpdatedIdsQueryBindMutable {
    False,
    True,
}
impl ToTokens for IsSelectOnlyUpdatedIdsQueryBindMutable {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectOnlyCreatedIdsQueryBindMutable {
    False,
    True,
}
impl ToTokens for IsSelectOnlyCreatedIdsQueryBindMutable {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsQueryBindMutable {
    False,
    True,
}
impl ToTokens for IsQueryBindMutable {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => Ts2::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IncrementParameterUnderscore {
    False,
    True,
}
impl ToTokens for IncrementParameterUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => IncrementSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ColumnParameterUnderscore {
    False,
    True,
}
impl ToTokens for ColumnParameterUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => ColumnSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsNeedToAddLogicalOperatorUnderscore {
    False,
    True,
}
impl ToTokens for IsNeedToAddLogicalOperatorUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => IsNeedToAddLogicalOperatorSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub enum IsPrimaryKeyUnderscore {
    False,
    True,
}
impl ToTokens for IsPrimaryKeyUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => IsPrimaryKeySc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum PgTypeOrPgJsonType {
    PgJsonType,
    PgType,
}
#[derive(Debug, Clone, Copy)]
pub enum DefaultSomeOneOrDefaultSomeOneWithMaxPageSize {
    DefaultSomeOne,
    DefaultSomeOneWithMaxPageSize,
}
#[derive(Debug, Clone, Copy)]
pub enum EqualOrEqualUsingFields {
    Equal,
    EqualUsingFields,
}
#[derive(Debug, Clone, Copy)]
pub enum EqualOperatorHandle {
    Equal,
    IsNull,
}
impl EqualOperatorHandle {
    #[must_use]
    pub fn to_tokens_path(&self, import_path: &ImportPath) -> Ts2 {
        let content_ts = match &self {
            Self::Equal => quote! {Equal},
            Self::IsNull => quote! {IsNull},
        };
        quote! {#import_path::#EqualOperatorUcc::#content_ts}
    }
}
//todo maybe reuse with other structs
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy)]
pub enum Dim {
    One,
    Two,
    Three,
    Four,
}
impl Dim {
    #[must_use]
    pub fn read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc(
        &self,
    ) -> Box<dyn DisplayPlusToTokens> {
        match self {
            Self::One => {
                Box::new(ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimOneEqualSc)
            }
            Self::Two => {
                Box::new(ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimTwoEqualSc)
            }
            Self::Three => {
                Box::new(ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimThreeEqualSc)
            }
            Self::Four => {
                Box::new(ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimFourEqualSc)
            }
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy)]
pub enum DimIndexNumber {
    Zero,
    One,
    Two,
    Three,
}
impl From<&Dim> for DimIndexNumber {
    fn from(value: &Dim) -> Self {
        match &value {
            Dim::One => Self::Zero,
            Dim::Two => Self::One,
            Dim::Three => Self::Two,
            Dim::Four => Self::Three,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum CreateQueryPartValueUnderscore {
    False,
    True,
}
impl ToTokens for CreateQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => ValueSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum CreateQueryPartIncrementUnderscore {
    False,
    True,
}
impl ToTokens for CreateQueryPartIncrementUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => IncrementSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum CreateQueryBindValueUnderscore {
    False,
    True,
}
impl ToTokens for CreateQueryBindValueUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => ValueSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum SelectQueryPartValueUnderscore {
    False,
    True,
}
impl ToTokens for SelectQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => ValueSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum UpdateQueryPartValueUnderscore {
    False,
    True,
}
impl ToTokens for UpdateQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => ValueSc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum UpdateQueryPartJsonbSetAccumulatorUnderscore {
    False,
    True,
}
impl ToTokens for UpdateQueryPartJsonbSetAccumulatorUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {jsonb_set_accumulator}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum UpdateQueryPartJsonbSetTargetUnderscore {
    False,
    True,
}
impl ToTokens for UpdateQueryPartJsonbSetTargetUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {jsonb_set_target}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum UpdateQueryPartJsonbSetPathUnderscore {
    False,
    True,
}
impl ToTokens for UpdateQueryPartJsonbSetPathUnderscore {
    fn to_tokens(&self, tokens: &mut Ts2) {
        match &self {
            Self::False => quote! {jsonb_set_path}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
pub fn gen_pg_type_where_ts(
    attrs_ts: &dyn ToTokens,
    variants: &Vec<&dyn PgFilter>,
    prefix: &dyn ToTokens,
    should_derive_utoipa_to_schema: &ShouldDeriveUtoipaToSchema,
    should_derive_schemars_json_schema: &ShouldDeriveSchemarsJsonSchema,
    is_query_bind_mutable: &IsQueryBindMutable,
) -> Ts2 {
    let ident = SelfWhereUcc::from_tokens(&prefix);
    let pg_type_tokens_where_ts = {
        let variants_ts = variants.iter().map(|el_a9dc0e35| {
            let el_ucc = el_a9dc0e35.ucc();
            let prefix_where_self_ucc = el_a9dc0e35.prefix_where_self_ucc();
            let option_type_ts: Option<Ts2> = el_a9dc0e35.maybe_generic();
            let type_ts =
                option_type_ts.map_or_else(Ts2::new, |value_0016edb0| quote! {<#value_0016edb0>});
            quote! {#el_ucc(where_filters::#prefix_where_self_ucc #type_ts)}
        });
        quote! {
            #attrs_ts
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_derive_utoipa_to_schema #should_derive_schemars_json_schema)]
            pub enum #ident {
                #(#variants_ts),*
            }
        }
    };
    let impl_pg_type_pg_type_where_filter_for_pg_type_tokens_where_ts =
        impl_pg_type_where_filter_for_ident_ts(
            &quote! {<'lifetime>},
            &ident,
            &Ts2::new(),
            &IncrementParameterUnderscore::False,
            &ColumnParameterUnderscore::False,
            &IsNeedToAddLogicalOperatorUnderscore::False,
            &{
                let variants_ts = variants.iter().map(|el_8bf490d9| {
                    let el_ucc = el_8bf490d9.ucc();
                    quote! {
                        Self::#el_ucc(#ValueSc) => pg_crud_common::PgTypeWhereFilter::query_part(
                            #ValueSc,
                            #IncrementSc,
                            #ColumnSc,
                            #IsNeedToAddLogicalOperatorSc,
                        )
                    }
                });
                quote! {
                    match &self {
                        #(#variants_ts),*
                    }
                }
            },
            is_query_bind_mutable,
            &{
                let variants_ts = variants.iter().map(|el_93e5c1bc| {
                    let el_ucc = el_93e5c1bc.ucc();
                    quote! {
                        Self::#el_ucc(#ValueSc) => pg_crud_common::PgTypeWhereFilter::query_bind(
                            #ValueSc,
                            #QuerySc
                        )
                    }
                });
                quote! {
                    match self {
                        #(#variants_ts),*
                    }
                }
            },
            &ImportPath::PgCrudCommon,
        );
    let impl_error_occurence_lib_to_err_string_for_pg_type_tokens_where_ts =
        gen_impl_to_err_string_ts(
            &Ts2::new(),
            &ident,
            &Ts2::new(),
            &quote! {format!("{self:#?}")},
        );
    let impl_all_variants_default_option_some_vec_one_el_for_pg_type_tokens_where_ts =
        gen_impl_pg_crud_common_all_variants_default_option_some_vec_one_el_ts(&ident, &{
            let variants_ts = variants.iter().map(|el_b9724130| {
                let el_ucc = el_b9724130.ucc();
                let default_option_some_vec_one_el_call_ts =
                    PgCrudCommonDefaultOptionSomeVecOneElCall;
                quote! {
                    Self::#el_ucc(#default_option_some_vec_one_el_call_ts)
                }
            });
            quote! {vec![#(#variants_ts),*]}
        });
    quote! {
        #pg_type_tokens_where_ts
        #impl_pg_type_pg_type_where_filter_for_pg_type_tokens_where_ts
        #impl_error_occurence_lib_to_err_string_for_pg_type_tokens_where_ts
        #impl_all_variants_default_option_some_vec_one_el_for_pg_type_tokens_where_ts
    }
}
#[must_use]
pub fn pg_crud_common_query_part_error_ts() -> Ts2 {
    quote! {pg_crud_common::#QueryPartErrorUcc}
}
pub fn gen_struct_ident_dq_ts(value: &dyn Display) -> Ts2 {
    dq_ts(&format!("struct {value}"))
}
pub fn gen_struct_ident_with_number_elements_dq_ts(
    ident: &dyn DisplayPlusToTokens,
    length: usize,
) -> Ts2 {
    dq_ts(&format!("struct {ident} with {length} elements"))
}
pub fn gen_tuple_struct_ident_dq_ts(value: &dyn Display) -> Ts2 {
    dq_ts(&format!("tuple struct {value}"))
}
pub fn gen_sqlx_types_json_type_declaration_ts(type_ts: &dyn ToTokens) -> Ts2 {
    quote! {sqlx::types::Json<#type_ts>}
}
pub fn gen_option_tokens_declaration_ts(type_ts: &dyn ToTokens) -> Ts2 {
    quote! {Option<#type_ts>}
}
pub fn gen_vec_tokens_declaration_ts(type_ts: &dyn ToTokens) -> Ts2 {
    quote! {Vec<#type_ts>}
}
pub fn gen_serde_deserialize_dq_ts(
    ident: &dyn DisplayPlusToTokens,
    length: usize,
) -> (Ts2, Ts2, Ts2) {
    let struct_pg_type_ident_where_tokens_dq_ts = gen_struct_ident_dq_ts(ident);
    let struct_pg_type_ident_where_tokens_with_number_elements_dq_ts =
        gen_struct_ident_with_number_elements_dq_ts(ident, length);
    let pg_type_ident_where_tokens_dq_ts = dq_ts(&ident);
    (
        struct_pg_type_ident_where_tokens_dq_ts,
        struct_pg_type_ident_where_tokens_with_number_elements_dq_ts,
        pg_type_ident_where_tokens_dq_ts,
    )
}
pub fn gen_impl_pg_json_type_ts(
    import_path: &ImportPath,
    ident: &dyn ToTokens,
    table_type_declaration_type_ts: &dyn ToTokens,
    create_type_ts: &dyn ToTokens,
    create_for_query_type_ts: &dyn ToTokens,
    select_type_ts: &dyn ToTokens,
    is_select_query_part_self_select_used: &IsSelectQueryPartSelfSelectUsed,
    is_select_query_part_column_name_and_maybe_field_getter_for_error_message_used: &IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed,
    is_select_query_part_is_pg_type_used: &IsSelectQueryPartIsPgTypeUsed,
    select_query_part_ts: &dyn ToTokens,
    where_type_ts: &dyn ToTokens,
    read_type_ts: &dyn ToTokens,
    read_only_ids_type_ts: &dyn ToTokens,
    select_only_ids_query_part_ts: &dyn ToTokens,
    read_inner_type_ts: &dyn ToTokens,
    into_inner_ts: &dyn ToTokens,
    update_type_ts: &dyn ToTokens,
    update_type_for_query_ts: &dyn ToTokens,
    update_query_part_ts: &dyn ToTokens,
    is_update_query_part_self_update_used: &IsUpdateQueryPartSelfUpdateUsed,
    is_update_query_part_jsonb_set_target_used: &IsUpdateQueryPartJsonbSetTargetUsed,
    is_update_query_bind_mutable: &IsUpdateQueryBindMutable,
    update_query_bind_ts: &dyn ToTokens,
    select_only_updated_ids_query_part_ts: &dyn ToTokens,
    is_select_only_updated_ids_query_bind_mutable: &IsSelectOnlyUpdatedIdsQueryBindMutable,
    select_only_updated_ids_query_bind_ts: &dyn ToTokens,
    select_only_created_ids_query_part_ts: &dyn ToTokens,
    is_select_only_created_ids_query_bind_mutable: &IsSelectOnlyCreatedIdsQueryBindMutable,
    select_only_created_ids_query_bind_ts: &dyn ToTokens,
) -> Ts2 {
    let path_ts = quote! {#import_path ::};
    let reference_mut_u64_ts = quote! {&mut #U64};
    let query_pg_arguments_ts =
        quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let query_lifetime_pg_arguments_ts =
        quote! {sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>};
    //todo maybe reexport sqlx?
    quote! {
        #AllowClippyArbitrarySourceItemOrdering
        impl #path_ts #PgJsonTypeUcc for #ident {
            type #TableTypeDeclarationUcc = #table_type_declaration_type_ts;
            type #CreateUcc = #create_type_ts;
            type #CreateForQueryUcc = #create_for_query_type_ts;
            type #SelectUcc = #select_type_ts;
            fn #SelectQueryPartSc(
                #is_select_query_part_self_select_used: &Self::#SelectUcc,
                #FieldIdentSc: #RefStr,
                #ColumnNameAndMaybeFieldGetterSc: #RefStr,
                #is_select_query_part_column_name_and_maybe_field_getter_for_error_message_used: #RefStr,
                #is_select_query_part_is_pg_type_used: #Bool,
            ) -> Result<#StringTs, #path_ts #QueryPartErrorUcc> {
                #select_query_part_ts
            }
            type #WhereUcc = #where_type_ts;
            type #ReadUcc = #read_type_ts;
            type #ReadOnlyIdsUcc = #read_only_ids_type_ts;
            fn #SelectOnlyIdsQueryPartSc(
                #ColumnNameAndMaybeFieldGetterSc: #RefStr,
            ) -> Result<#StringTs, #import_path ::#QueryPartErrorUcc> {
                #select_only_ids_query_part_ts
            }
            type #ReadInnerUcc = #read_inner_type_ts;
            fn into_inner(#ValueSc: Self::#ReadUcc) -> Self::#ReadInnerUcc {
                #into_inner_ts
            }
            type #UpdateUcc = #update_type_ts;
            type #UpdateForQueryUcc = #update_type_for_query_ts;
            fn #UpdateQueryPartSc(
                #is_update_query_part_self_update_used: &Self::#UpdateForQueryUcc,
                #JsonbSetAccumulatorSc: #RefStr,
                #is_update_query_part_jsonb_set_target_used: #RefStr,
                #JsonbSetPathSc: #RefStr,
                #IncrementSc: #reference_mut_u64_ts,
            ) -> Result<#StringTs, #path_ts #QueryPartErrorUcc> {
                #update_query_part_ts
            }
            fn #UpdateQueryBindSc(
                #ValueSc: Self::#UpdateForQueryUcc,
                #is_update_query_bind_mutable #QuerySc: #query_pg_arguments_ts
            ) -> Result<#query_pg_arguments_ts, #StringTs> {
                #update_query_bind_ts
            }
            fn #SelectOnlyUpdatedIdsQueryPartSc(
                #ValueSc: &Self::#UpdateForQueryUcc,
                #FieldIdentSc: #RefStr,
                #ColumnNameAndMaybeFieldGetterSc: #RefStr,
                #IncrementSc: &mut #U64
            ) -> Result<#StringTs, #import_path ::#QueryPartErrorUcc> {
                #select_only_updated_ids_query_part_ts
            }
            fn #SelectOnlyUpdatedIdsQueryBindSc<'lifetime>(
                #ValueSc: &'lifetime Self::#UpdateForQueryUcc,
                #is_select_only_updated_ids_query_bind_mutable #QuerySc: #query_lifetime_pg_arguments_ts
            ) -> Result<#query_lifetime_pg_arguments_ts, #StringTs> {
                #select_only_updated_ids_query_bind_ts
            }
            fn #SelectOnlyCreatedIdsQueryPartSc(
                #ValueSc: &Self::#CreateForQueryUcc,
                #FieldIdentSc: #RefStr,
                #ColumnNameAndMaybeFieldGetterSc: #RefStr,
                #IncrementSc: &mut #U64
            ) -> Result<#StringTs, #import_path ::#QueryPartErrorUcc> {
                #select_only_created_ids_query_part_ts
            }
            fn #SelectOnlyCreatedIdsQueryBindSc<'lifetime>(
                #ValueSc: &'lifetime Self::#CreateForQueryUcc,
                #is_select_only_created_ids_query_bind_mutable #QuerySc: #query_lifetime_pg_arguments_ts
            ) -> Result<#query_lifetime_pg_arguments_ts, #StringTs> {
                #select_only_created_ids_query_bind_ts
            }
        }
    }
}
pub fn gen_impl_default_option_some_vec_one_el_ts(
    impl_generic_ts: &dyn ToTokens,
    import_path: &ImportPath,
    ident: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import_path.default_option_some_vec_one_el();
    let default_option_some_vec_one_el_sc = DefaultOptionSomeVecOneElSc;
    quote! {
        impl #impl_generic_ts #path_trait_ts for #ident #ident_generic_ts {
            fn #default_option_some_vec_one_el_sc() -> Self {
                #content_ts
            }
        }
    }
}
pub fn gen_impl_all_variants_default_option_some_vec_one_el_ts(
    import_path: &ImportPath,
    ident: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import_path.all_variants_default_option_some_vec_one_el();
    let all_variants_default_option_some_vec_one_el_sc = AllVariantsDefaultOptionSomeVecOneElSc;
    quote! {
        impl #path_trait_ts for #ident {
            fn #all_variants_default_option_some_vec_one_el_sc() -> Vec<Self> {
                #content_ts
            }
        }
    }
}
pub fn gen_impl_default_option_some_vec_one_el_max_page_size_ts(
    impl_generic_ts: &dyn ToTokens,
    import_path: &ImportPath,
    ident: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import_path.default_option_some_vec_one_el_max_page_size();
    let default_option_some_vec_one_el_max_page_size_sc = DefaultOptionSomeVecOneElMaxPageSizeSc;
    quote! {
        impl #impl_generic_ts #path_trait_ts for #ident #ident_generic_ts {
            fn #default_option_some_vec_one_el_max_page_size_sc() -> Self {
                #content_ts
            }
        }
    }
}
pub fn gen_impl_all_variants_default_option_some_vec_one_el_max_page_size_ts(
    import_path: &ImportPath,
    ident: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    let path_trait_ts = import_path.all_variants_default_option_some_vec_one_el_max_page_size();
    let all_variants_default_option_some_vec_one_el_max_page_size_sc =
        AllVariantsDefaultOptionSomeVecOneElMaxPageSizeSc;
    quote! {
        impl #path_trait_ts for #ident {
            fn #all_variants_default_option_some_vec_one_el_max_page_size_sc() -> Vec<Self> {
                #content_ts
            }
        }
    }
}
pub fn gen_impl_pg_crud_common_default_option_some_vec_one_el_ts(
    ident: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_default_option_some_vec_one_el_ts(
        &Ts2::new(),
        &ImportPath::PgCrudCommon,
        ident,
        &Ts2::new(),
        content_ts,
    )
}
pub fn gen_impl_pg_crud_default_option_some_vec_one_el_ts(
    ident: &dyn ToTokens,
    lifetime_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_default_option_some_vec_one_el_ts(
        &Ts2::new(),
        &ImportPath::PgCrud,
        ident,
        lifetime_ts,
        content_ts,
    )
}
pub fn gen_impl_pg_crud_common_all_variants_default_option_some_vec_one_el_ts(
    ident: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_all_variants_default_option_some_vec_one_el_ts(
        &ImportPath::PgCrudCommon,
        ident,
        content_ts,
    )
}
pub fn gen_impl_pg_crud_all_variants_default_option_some_vec_one_el_ts(
    ident: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_all_variants_default_option_some_vec_one_el_ts(&ImportPath::PgCrud, ident, content_ts)
}
pub fn gen_impl_pg_crud_common_default_option_some_vec_one_el_max_page_size_ts(
    ident: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_default_option_some_vec_one_el_max_page_size_ts(
        &Ts2::new(),
        &ImportPath::PgCrudCommon,
        ident,
        &Ts2::new(),
        content_ts,
    )
}
pub fn gen_impl_pg_crud_default_option_some_vec_one_el_max_page_size_ts(
    ident: &dyn ToTokens,
    lifetime_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_default_option_some_vec_one_el_max_page_size_ts(
        &Ts2::new(),
        &ImportPath::PgCrud,
        ident,
        lifetime_ts,
        content_ts,
    )
}
pub fn gen_impl_pg_crud_all_variants_default_option_some_vec_one_el_max_page_size_ts(
    ident: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_impl_all_variants_default_option_some_vec_one_el_max_page_size_ts(
        &ImportPath::PgCrud,
        ident,
        content_ts,
    )
}
pub fn impl_pg_type_where_filter_for_ident_ts(
    impl_generic_ts: &dyn ToTokens,
    ident_ts: &dyn ToTokens,
    ident_generic_ts: &dyn ToTokens,
    increment_parameter_underscore: &IncrementParameterUnderscore,
    column_parameter_underscore: &ColumnParameterUnderscore,
    is_need_to_add_logical_operator_underscore: &IsNeedToAddLogicalOperatorUnderscore,
    query_part_content_ts: &dyn ToTokens,
    is_query_bind_mutable: &IsQueryBindMutable,
    query_bind_content_ts: &dyn ToTokens,
    import_path: &ImportPath,
) -> Ts2 {
    quote! {
        #AllowClippyArbitrarySourceItemOrdering
        impl #impl_generic_ts #import_path ::#PgTypeWhereFilterUcc<'lifetime> for #ident_ts #ident_generic_ts {
            fn #QueryPartSc(
                &self,
                #increment_parameter_underscore: &mut #U64,
                #column_parameter_underscore: &dyn #StdFmtDisplay,
                #is_need_to_add_logical_operator_underscore: #Bool
            ) -> Result<#StringTs, #import_path::#QueryPartErrorUcc> {
                #query_part_content_ts
            }
            fn #QueryBindSc(self, #is_query_bind_mutable query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
                sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
                String
            > {
                #query_bind_content_ts
            }
        }
    }
}
pub fn gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(
    ident_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #ident_ts {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#content_ts, buf)
            }
        }
    }
}
pub fn gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
    ident_ts: &dyn ToTokens,
    type_ts: &dyn ToTokens,
    ok_value_match_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #ident_ts {
            fn decode(#ValueSc: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#type_ts as sqlx::Decode<sqlx::Postgres>>::decode(#ValueSc) {
                    Ok(value_147c3532) => #ok_value_match_ts,
                    Err(error) => Err(error),
                }
            }
        }
    }
}
pub fn gen_impl_sqlx_type_sqlx_pg_for_ident_ts(
    ident_ts: &dyn ToTokens,
    type_ts: &dyn ToTokens,
) -> Ts2 {
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
    import_path: &ImportPath,
    ident: &dyn ToTokens,
    ident_table_type_declaration_ucc: &dyn ToTokens,
    is_primary_key_underscore: &IsPrimaryKeyUnderscore,
    create_table_column_query_part_ts: &dyn ToTokens,
    ident_create_ucc: &dyn ToTokens,
    create_query_part_value_underscore: &CreateQueryPartValueUnderscore,
    create_query_part_increment_underscore: &CreateQueryPartIncrementUnderscore,
    create_query_part_content_ts: &dyn ToTokens,
    create_query_bind_value_underscore: &CreateQueryBindValueUnderscore,
    is_create_query_bind_mutable: &IsCreateQueryBindMutable,
    create_query_bind_content_ts: &dyn ToTokens,
    ident_select_ucc: &dyn ToTokens,
    select_query_part_value_underscore: &SelectQueryPartValueUnderscore,
    select_query_part_content_ts: &dyn ToTokens,
    ident_where_ucc: &dyn ToTokens,
    ident_read_ucc: &dyn ToTokens,
    normalize_ts: &dyn ToTokens,
    read_only_ids_ts: &dyn ToTokens,
    select_only_ids_query_part_ts: &dyn ToTokens,
    ident_read_inner_ucc: &dyn ToTokens,
    into_inner_ts: &dyn ToTokens,
    ident_update_ucc: &dyn ToTokens,
    ident_update_for_query_ucc: &dyn ToTokens,
    update_query_part_value_underscore: &UpdateQueryPartValueUnderscore,
    update_query_part_jsonb_set_accumulator_underscore: &UpdateQueryPartJsonbSetAccumulatorUnderscore,
    update_query_part_jsonb_set_target_underscore: &UpdateQueryPartJsonbSetTargetUnderscore,
    update_query_part_jsonb_set_path_underscore: &UpdateQueryPartJsonbSetPathUnderscore,
    update_query_part_content_ts: &dyn ToTokens,
    is_update_query_bind_mutable: &IsUpdateQueryBindMutable,
    update_query_bind_content_ts: &dyn ToTokens,
    select_only_updated_ids_query_part_ts: &dyn ToTokens,
    is_select_only_updated_ids_query_bind_mutable: &IsSelectOnlyUpdatedIdsQueryBindMutable,
    select_only_updated_ids_query_bind_ts: &dyn ToTokens,
) -> Ts2 {
    let query_pg_arguments_ts =
        quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    quote! {
        #AllowClippyArbitrarySourceItemOrdering
        impl #import_path :: #PgTypeUcc for #ident {
            type #TableTypeDeclarationUcc = #ident_table_type_declaration_ucc;
            fn #CreateTableColumnQueryPartSc(#ColumnSc: &dyn #StdFmtDisplay, #is_primary_key_underscore: #Bool) -> impl #StdFmtDisplay {
                #create_table_column_query_part_ts
            }
            type #CreateUcc = #ident_create_ucc;
            fn #CreateQueryPartSc(
                #create_query_part_value_underscore: &Self::#CreateUcc,
                #create_query_part_increment_underscore: &mut #U64
            ) -> Result<#StringTs, #import_path ::#QueryPartErrorUcc> {
                #create_query_part_content_ts
            }
            fn #CreateQueryBindSc(
                #create_query_bind_value_underscore: Self::#CreateUcc,
                #is_create_query_bind_mutable #QuerySc: #query_pg_arguments_ts
            ) -> Result<
                #query_pg_arguments_ts,
                String
            > {
                #create_query_bind_content_ts
            }
            type #SelectUcc = #ident_select_ucc;
            fn #SelectQueryPartSc(
                #select_query_part_value_underscore: &Self::#SelectUcc,
                #ColumnSc: #RefStr,
            ) -> Result<#StringTs, #import_path ::#QueryPartErrorUcc> {
                #select_query_part_content_ts
            }
            type #WhereUcc = #ident_where_ucc;
            type #ReadUcc = #ident_read_ucc;
            fn #NormalizeSc(#ValueSc: Self::#ReadUcc) -> Self::#ReadUcc {
                #normalize_ts
            }
            type #ReadOnlyIdsUcc = #read_only_ids_ts;
            fn #SelectOnlyIdsQueryPartSc(
                #ColumnSc: #RefStr
            ) -> Result<#StringTs, #import_path ::#QueryPartErrorUcc> {
                #select_only_ids_query_part_ts
            }
            type #ReadInnerUcc = #ident_read_inner_ucc;
            fn into_inner(#ValueSc: Self::#ReadUcc) -> Self::#ReadInnerUcc {
                #into_inner_ts
            }
            type #UpdateUcc = #ident_update_ucc;
            type #UpdateForQueryUcc = #ident_update_for_query_ucc;
            fn #UpdateQueryPartSc(
                #update_query_part_value_underscore: &Self::#UpdateForQueryUcc,
                #update_query_part_jsonb_set_accumulator_underscore: #RefStr,
                #update_query_part_jsonb_set_target_underscore: #RefStr,
                #update_query_part_jsonb_set_path_underscore: #RefStr,
                #IncrementSc: &mut #U64
            ) -> Result<#StringTs, #import_path ::#QueryPartErrorUcc> {
                #update_query_part_content_ts
            }
            fn #UpdateQueryBindSc(
                #ValueSc: Self::#UpdateForQueryUcc,
                #is_update_query_bind_mutable #QuerySc: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<
                sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                String
            > {
                #update_query_bind_content_ts
            }
            fn #SelectOnlyUpdatedIdsQueryPartSc(
                #ValueSc: &Self::#UpdateForQueryUcc,
                #ColumnSc: #RefStr,
                #IncrementSc: &mut #U64,
            ) -> Result<#StringTs, #import_path ::#QueryPartErrorUcc> {
                #select_only_updated_ids_query_part_ts
            }
            fn #SelectOnlyUpdatedIdsQueryBindSc<'lifetime>(
                #ValueSc: &'lifetime Self::#UpdateForQueryUcc,
                #is_select_only_updated_ids_query_bind_mutable #QuerySc: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
                #select_only_updated_ids_query_bind_ts
            }
        }
    }
}
pub fn gen_impl_pg_type_not_primary_key_for_ident_ts(
    import_path: &ImportPath,
    ident: &dyn ToTokens,
) -> Ts2 {
    let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
    quote! {
        #AllowClippyArbitrarySourceItemOrdering
        impl #import_path::#PgTypeNotPrimaryKeyUcc for #ident {
            type #PgTypeUcc = Self;
            type #CreateUcc = #ident_create_ucc;
        }
    }
}
// fn gen_read_only_ids_merged_with_create_into_where_method_ts(
//     import_path: &ImportPath,
//     method_name_ts: &dyn ToTokens,
//     content_ts: &dyn ToTokens,
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
//             <#SelfUcc::#pg_type_or_pg_json_type_ts as #import_path::#pg_type_or_pg_json_type_ts>
//         }
//     };
//     quote!{
//         fn #method_name_ts(
//             #ReadOnlyIdsSc: #self_pg_type_or_pg_json_type_as_pg_json_type_ts::#ReadOnlyIdsUcc,
//             #CreateSc: #self_pg_type_or_pg_json_type_as_pg_json_type_ts::#CreateUcc
//         ) -> Vec<#self_pg_type_or_pg_json_type_as_pg_json_type_ts::#WhereUcc> {
//             #content_ts
//         }
//     }
// }
fn gen_option_vec_create_ts(path_ts: &dyn ToTokens, content_ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #OptionVecCreateSc() -> Option<Vec<#path_ts::#CreateUcc>> {
            #content_ts
        }
    }
}
fn gen_read_only_ids_to_two_dimal_vec_read_inner_ts(
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsToTwoDimalVecReadInnerSc(
            #ReadOnlyIdsSc: &#path_ts::#ReadOnlyIdsUcc
        ) -> Vec<Vec<#path_ts::#ReadInnerUcc>> {
            #content_ts
        }
    }
}
fn gen_read_inner_into_read_with_new_or_try_new_unwraped_ts(
    type_ts: &dyn ToTokens,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadInnerIntoReadWithNewOrTryNewUnwrapedSc(
            #ValueSc: #type_ts
        ) -> #path_ts::#ReadUcc {
            #content_ts
        }
    }
}
fn gen_read_inner_into_update_with_new_or_try_new_unwraped_ts(
    type_ts: &dyn ToTokens,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc(#ValueSc: #type_ts) -> #path_ts::#UpdateUcc {
            #content_ts
        }
    }
}
fn gen_update_to_read_only_ids_ts(path_ts: &dyn ToTokens, content_ts: &dyn ToTokens) -> Ts2 {
    quote! {
        fn #UpdateToReadOnlyIdsSc(
            #ValueSc: &#path_ts::#UpdateUcc
        ) -> #path_ts::#ReadOnlyIdsUcc {
            #content_ts
        }
    }
}
fn gen_read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsToOptionValueReadDefaultOptionSomeVecOneElSc(
            #ValueSc: &#path_ts::#ReadOnlyIdsUcc
        ) -> Option<#import_path::#ValueUcc<#path_ts::#ReadUcc>> {
            #content_ts
        }
    }
}
fn gen_previous_read_merged_with_option_update_into_read_ts(
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #PreviousReadMergedWithOptionUpdateIntoReadSc(
            #ReadSc: #path_ts::#ReadUcc,
            #OptionUpdateSc: Option<#path_ts::#UpdateUcc>,
        ) -> #path_ts::#ReadUcc {
            #content_ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_read_ts(
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoReadSc(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> #path_ts::#ReadUcc {
            #content_ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_option_value_read_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import_path::#ValueUcc<#path_ts::#ReadUcc>> {
            #content_ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_table_type_declaration_ts(
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> #path_ts::#TableTypeDeclarationUcc {
            #content_ts
        }
    }
}
pub fn gen_read_only_ids_merged_with_create_into_where_equal_ts(
    read_only_ids_ts: &dyn ToTokens,
    create_ts: &dyn ToTokens,
    where_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoWhereEqualSc(
            #ReadOnlyIdsSc: #read_only_ids_ts,
            #CreateSc: #create_ts
        ) -> #where_ts {
            #content_ts
        }
    }
}
pub fn gen_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
    import_path: &ImportPath,
    read_only_ids_ts: &dyn ToTokens,
    create_ts: &dyn ToTokens,
    where_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc(
            #ReadOnlyIdsSc: #read_only_ids_ts,
            #CreateSc: #create_ts
        ) -> #import_path::NotEmptyUniqueVec<#where_ts> {
            #content_ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_vec_or_option_vec_where_equal_to_json_field_pg_type_or_pg_json_type_ts(
    import_path: ImportPath,
    read_only_ids_ts: &dyn ToTokens,
    create_ts: &dyn ToTokens,
    where_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
    pg_type_or_pg_json_type: PgTypeOrPgJsonType,
) -> Ts2 {
    let return_type_ts = {
        let return_type_handle_ts = quote! {#import_path::NotEmptyUniqueVec<#where_ts>};
        match &pg_type_or_pg_json_type {
            PgTypeOrPgJsonType::PgType => gen_option_tokens_declaration_ts(&return_type_handle_ts),
            PgTypeOrPgJsonType::PgJsonType => return_type_handle_ts,
        }
    };
    let name_ts: &dyn ToTokens = match &pg_type_or_pg_json_type {
        PgTypeOrPgJsonType::PgType => {
            &ReadOnlyIdsMergedWithCreateIntoOptionVecWhereEqualToJsonFieldSc
        }
        PgTypeOrPgJsonType::PgJsonType => {
            &ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc
        }
    };
    quote! {
        fn #name_ts(
            #ReadOnlyIdsSc: #read_only_ids_ts,
            #CreateSc: #create_ts
        ) -> #return_type_ts {
            #content_ts
        }
    }
}
pub fn gen_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts(
    import_path: ImportPath,
    read_only_ids_ts: &dyn ToTokens,
    create_ts: &dyn ToTokens,
    where_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_vec_or_option_vec_where_equal_to_json_field_pg_type_or_pg_json_type_ts(
        import_path,
        &read_only_ids_ts,
        &create_ts,
        &where_ts,
        &content_ts,
        PgTypeOrPgJsonType::PgJsonType
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts(
    import_path: ImportPath,
    name_ts: &dyn ToTokens,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #name_ts(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import_path::NotEmptyUniqueVec<#path_ts::#WhereUcc>> {
            #content_ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts(
        import_path,
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimOneEqualSc,
        &path_ts,
        &content_ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts(
        import_path,
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimTwoEqualSc,
        &path_ts,
        &content_ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts(
        import_path,
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimThreeEqualSc,
        &path_ts,
        &content_ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts(
        import_path,
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereDimFourEqualSc,
        &path_ts,
        &content_ts,
    )
}
fn gen_create_into_pg_json_type_option_vec_where_length_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #CreateIntoPgJsonTypeOptionVecWhereLengthEqualSc(
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import_path::NotEmptyUniqueVec<#path_ts::#WhereUcc>> {
            #content_ts
        }
    }
}
fn gen_create_into_pg_json_type_option_vec_where_length_greater_than_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #CreateIntoPgJsonTypeOptionVecWhereLengthGreaterThanSc(
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import_path::NotEmptyUniqueVec<#path_ts::#WhereUcc>> {
            #content_ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
    method_name_ts: &dyn ToTokens,
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        fn #method_name_ts(
            #ReadOnlyIdsSc: #path_ts::#ReadOnlyIdsUcc,
            #CreateSc: #path_ts::#CreateUcc
        ) -> Option<#import_path::NotEmptyUniqueVec<#import_path::SingleOrMultiple<#path_ts::#WhereUcc>>> {
            #content_ts
        }
    }
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereGreaterThanSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereBetweenSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereInSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereRegularExpressionSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElGreaterThanSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts(
    import_path: ImportPath,
    path_ts: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    gen_read_only_ids_merged_with_create_into_pg_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElRegularExpressionSc,
        import_path,
        path_ts,
        content_ts,
    )
}
pub fn gen_impl_pg_type_test_cases_for_ident_ts(
    cfg_ts: &dyn ToTokens,
    import_path: &ImportPath,
    type_ts: &dyn ToTokens,
    ident: &dyn ToTokens,
    option_vec_create_ts: &dyn ToTokens,
    read_only_ids_to_two_dimal_vec_read_inner_ts: &dyn ToTokens,
    read_inner_into_read_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    read_inner_into_update_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    update_to_read_only_ids_ts: &dyn ToTokens,
    read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts: &dyn ToTokens,
    previous_read_merged_with_option_update_into_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_option_value_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_table_type_declaration_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_where_equal_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts: &dyn ToTokens,
    create_into_pg_type_option_vec_where_dim_one_equal_ts: &dyn ToTokens,
    pg_type_option_vec_where_greater_than_test_ts: &dyn ToTokens,
    read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_dim_one_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_dim_two_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_dim_three_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_dim_four_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_length_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_length_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts: &dyn ToTokens,
) -> Ts2 {
    let self_pg_type_as_pg_type_ts = quote! {<#SelfUcc::#PgTypeUcc as #import_path::#PgTypeUcc>};
    let self_pg_type_as_pg_type_read_only_ids_ts =
        quote! {#self_pg_type_as_pg_type_ts::#ReadOnlyIdsUcc};
    let self_pg_type_as_pg_type_create_ts = quote! {#self_pg_type_as_pg_type_ts::#CreateUcc};
    let self_pg_type_as_pg_type_where_ts = quote! {#self_pg_type_as_pg_type_ts::#WhereUcc};
    let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
    let option_vec_create_content_ts =
        gen_option_vec_create_ts(&self_pg_type_as_pg_type_ts, &option_vec_create_ts);
    let read_only_ids_to_two_dimal_vec_read_inner_content_ts =
        gen_read_only_ids_to_two_dimal_vec_read_inner_ts(
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_to_two_dimal_vec_read_inner_ts,
        );
    let read_inner_into_read_with_new_or_try_new_unwraped_content_ts =
        gen_read_inner_into_read_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &read_inner_into_read_with_new_or_try_new_unwraped_ts,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_content_ts =
        gen_read_inner_into_update_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_type_as_pg_type_ts,
            &read_inner_into_update_with_new_or_try_new_unwraped_ts,
        );
    let update_to_read_only_ids_content_ts =
        gen_update_to_read_only_ids_ts(&self_pg_type_as_pg_type_ts, &update_to_read_only_ids_ts);
    let read_only_ids_to_option_value_read_default_option_some_vec_one_el_content_ts =
        gen_read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts,
        );
    let previous_read_merged_with_option_update_into_read_content_ts =
        gen_previous_read_merged_with_option_update_into_read_ts(
            &self_pg_type_as_pg_type_ts,
            &previous_read_merged_with_option_update_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_read_content_ts =
        gen_read_only_ids_merged_with_create_into_read_ts(
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_option_value_read_content_ts =
        gen_read_only_ids_merged_with_create_into_option_value_read_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_option_value_read_ts,
        );
    let read_only_ids_merged_with_create_into_table_type_declaration_content_ts =
        gen_read_only_ids_merged_with_create_into_table_type_declaration_ts(
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_table_type_declaration_ts,
        );
    let read_only_ids_merged_with_create_into_where_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_where_equal_ts(
            &self_pg_type_as_pg_type_read_only_ids_ts,
            &self_pg_type_as_pg_type_create_ts,
            &self_pg_type_as_pg_type_where_ts,
            &read_only_ids_merged_with_create_into_where_equal_ts,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_ts =
        gen_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
            import_path,
            &self_pg_type_as_pg_type_read_only_ids_ts,
            &self_pg_type_as_pg_type_create_ts,
            &self_pg_type_as_pg_type_where_ts,
            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
        );
    let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_content_ts = gen_read_only_ids_merged_with_create_into_vec_or_option_vec_where_equal_to_json_field_pg_type_or_pg_json_type_ts(
        *import_path,
        &self_pg_type_as_pg_type_read_only_ids_ts,
        &self_pg_type_as_pg_type_create_ts,
        &self_pg_type_as_pg_type_where_ts,
        &read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts,
        PgTypeOrPgJsonType::PgType,
    );
    let create_into_pg_type_option_vec_where_dim_one_equal_sc =
        CreateIntoPgTypeOptionVecWhereDimOneEqualSc;
    let pg_type_option_vec_where_greater_than_test_sc = PgTypeOptionVecWhereGreaterThanTestSc;
    let read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_sc =
        ReadOnlyIdsMergedWithTableTypeDeclarationIntoPgTypeOptionWhereGreaterThanSc;
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_option_vec_where_dim_one_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_option_vec_where_dim_two_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_option_vec_where_dim_three_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_option_vec_where_dim_four_equal_ts,
        );
    let create_into_pg_json_type_option_vec_where_length_equal_content_ts =
        gen_create_into_pg_json_type_option_vec_where_length_equal_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_option_vec_where_length_equal_ts,
        );
    let create_into_pg_json_type_option_vec_where_length_greater_than_content_ts =
        gen_create_into_pg_json_type_option_vec_where_length_greater_than_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &create_into_pg_json_type_option_vec_where_length_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts(
            *import_path,
            &self_pg_type_as_pg_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts,
        );
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySourceItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import_path::#PgTypeTestCasesUcc for #ident {
            type #PgTypeUcc = #SelfUcc;
            type #SelectUcc = #ident_select_ucc;
            #option_vec_create_content_ts
            #read_only_ids_to_two_dimal_vec_read_inner_content_ts
            #read_inner_into_read_with_new_or_try_new_unwraped_content_ts
            #read_inner_into_update_with_new_or_try_new_unwraped_content_ts
            #update_to_read_only_ids_content_ts
            #read_only_ids_to_option_value_read_default_option_some_vec_one_el_content_ts
            #previous_read_merged_with_option_update_into_read_content_ts
            #read_only_ids_merged_with_create_into_read_content_ts
            #read_only_ids_merged_with_create_into_option_value_read_content_ts
            #read_only_ids_merged_with_create_into_table_type_declaration_content_ts
            #read_only_ids_merged_with_create_into_where_equal_content_ts
            #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_ts
            #read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_content_ts
            fn #create_into_pg_type_option_vec_where_dim_one_equal_sc(
                #CreateSc: #self_pg_type_as_pg_type_ts::#CreateUcc
            ) -> Option<#import_path::NotEmptyUniqueVec<#self_pg_type_as_pg_type_ts::#WhereUcc>> {
                #create_into_pg_type_option_vec_where_dim_one_equal_ts
            }
            fn #pg_type_option_vec_where_greater_than_test_sc() -> Option<
                #import_path::NotEmptyUniqueVec<
                    #import_path::PgTypeGreaterThanTest<
                        #SelfUcc::#PgTypeUcc
                    >
                >
            > {
                #pg_type_option_vec_where_greater_than_test_ts
            }
            fn #read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_sc(
                greater_than_variant: #import_path::PgTypeGreaterThanVariant,
                #ReadOnlyIdsSc: #self_pg_type_as_pg_type_ts::#ReadOnlyIdsUcc,
                #TableTypeDeclarationSc: #self_pg_type_as_pg_type_ts::#TableTypeDeclarationUcc,
            ) -> Option<#self_pg_type_as_pg_type_ts::#WhereUcc> {
                #read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_ts
            }
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_content_ts
            #create_into_pg_json_type_option_vec_where_length_equal_content_ts
            #create_into_pg_json_type_option_vec_where_length_greater_than_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_content_ts
        }
    }
}
pub fn gen_impl_pg_json_type_test_cases_for_ident_ts(
    cfg_ts: &dyn ToTokens,
    import_path: &ImportPath,
    type_ts: &dyn ToTokens,
    ident: &dyn ToTokens,
    option_vec_create_ts: &dyn ToTokens,
    read_only_ids_to_two_dimal_vec_read_inner_ts: &dyn ToTokens,
    read_inner_into_read_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    read_inner_into_update_with_new_or_try_new_unwraped_ts: &dyn ToTokens,
    read_only_ids_into_option_value_read_inner_ts: &dyn ToTokens,
    update_to_read_only_ids_ts: &dyn ToTokens,
    read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts: &dyn ToTokens,
    previous_read_merged_with_option_update_into_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_option_value_read_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_table_type_declaration_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_where_equal_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_dim_one_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_dim_two_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_dim_three_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_dim_four_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_length_equal_ts: &dyn ToTokens,
    create_into_pg_json_type_option_vec_where_length_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts: &dyn ToTokens,
    read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts: &dyn ToTokens,
) -> Ts2 {
    let self_pg_json_type_as_pg_json_type_ts =
        quote! {<#SelfUcc::#PgJsonTypeUcc as #import_path::#PgJsonTypeUcc>};
    let self_pg_json_type_as_pg_json_type_read_only_ids_ts =
        quote! {#self_pg_json_type_as_pg_json_type_ts::#ReadOnlyIdsUcc};
    let self_pg_json_type_as_pg_json_type_create_ts =
        quote! {#self_pg_json_type_as_pg_json_type_ts::#CreateUcc};
    let self_pg_json_type_as_pg_json_type_where_ts =
        quote! {#self_pg_json_type_as_pg_json_type_ts::#WhereUcc};
    let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
    let option_vec_create_content_ts =
        gen_option_vec_create_ts(&self_pg_json_type_as_pg_json_type_ts, &option_vec_create_ts);
    let read_only_ids_to_two_dimal_vec_read_inner_content_ts =
        gen_read_only_ids_to_two_dimal_vec_read_inner_ts(
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_to_two_dimal_vec_read_inner_ts,
        );
    let read_inner_into_read_with_new_or_try_new_unwraped_content_ts =
        gen_read_inner_into_read_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_inner_into_read_with_new_or_try_new_unwraped_ts,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_content_ts =
        gen_read_inner_into_update_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_inner_into_update_with_new_or_try_new_unwraped_ts,
        );
    let update_to_read_only_ids_content_ts = gen_update_to_read_only_ids_ts(
        &self_pg_json_type_as_pg_json_type_ts,
        &update_to_read_only_ids_ts,
    );
    let read_only_ids_to_option_value_read_default_option_some_vec_one_el_content_ts =
        gen_read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts,
        );
    let previous_read_merged_with_option_update_into_read_content_ts =
        gen_previous_read_merged_with_option_update_into_read_ts(
            &self_pg_json_type_as_pg_json_type_ts,
            &previous_read_merged_with_option_update_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_read_content_ts =
        gen_read_only_ids_merged_with_create_into_read_ts(
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_option_value_read_content_ts =
        gen_read_only_ids_merged_with_create_into_option_value_read_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_option_value_read_ts,
        );
    let read_only_ids_merged_with_create_into_table_type_declaration_content_ts =
        gen_read_only_ids_merged_with_create_into_table_type_declaration_ts(
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_table_type_declaration_ts,
        );
    let read_only_ids_merged_with_create_into_where_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_where_equal_ts(
            &self_pg_json_type_as_pg_json_type_read_only_ids_ts,
            &self_pg_json_type_as_pg_json_type_create_ts,
            &self_pg_json_type_as_pg_json_type_where_ts,
            &read_only_ids_merged_with_create_into_where_equal_ts,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_ts =
        gen_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
            import_path,
            &self_pg_json_type_as_pg_json_type_read_only_ids_ts,
            &self_pg_json_type_as_pg_json_type_create_ts,
            &self_pg_json_type_as_pg_json_type_where_ts,
            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_content_ts =
        gen_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_read_only_ids_ts,
            &self_pg_json_type_as_pg_json_type_create_ts,
            &self_pg_json_type_as_pg_json_type_where_ts,
            &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_option_vec_where_dim_one_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_option_vec_where_dim_two_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_option_vec_where_dim_three_equal_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_option_vec_where_dim_four_equal_ts,
        );
    let create_into_pg_json_type_option_vec_where_length_equal_content_ts =
        gen_create_into_pg_json_type_option_vec_where_length_equal_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_option_vec_where_length_equal_ts,
        );
    let create_into_pg_json_type_option_vec_where_length_greater_than_content_ts =
        gen_create_into_pg_json_type_option_vec_where_length_greater_than_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &create_into_pg_json_type_option_vec_where_length_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_content_ts =
        gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts(
            *import_path,
            &self_pg_json_type_as_pg_json_type_ts,
            &read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts,
        );
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #AllowClippyArbitrarySourceItemOrdering
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import_path::#PgJsonTypeTestCasesUcc for #ident {
            type #PgJsonTypeUcc = #SelfUcc;
            type #SelectUcc = #ident_select_ucc;
            #option_vec_create_content_ts
            #read_only_ids_to_two_dimal_vec_read_inner_content_ts
            #read_inner_into_read_with_new_or_try_new_unwraped_content_ts
            #read_inner_into_update_with_new_or_try_new_unwraped_content_ts
            fn #ReadOnlyIdsIntoOptionValueReadInnerSc(
                #ValueSc: #self_pg_json_type_as_pg_json_type_ts::#ReadOnlyIdsUcc
            ) -> Option<#import_path::#ValueUcc<#self_pg_json_type_as_pg_json_type_ts::#ReadInnerUcc>> {
                #read_only_ids_into_option_value_read_inner_ts
            }
            #update_to_read_only_ids_content_ts
            #read_only_ids_to_option_value_read_default_option_some_vec_one_el_content_ts
            #previous_read_merged_with_option_update_into_read_content_ts
            #read_only_ids_merged_with_create_into_read_content_ts
            #read_only_ids_merged_with_create_into_option_value_read_content_ts
            #read_only_ids_merged_with_create_into_table_type_declaration_content_ts
            #read_only_ids_merged_with_create_into_where_equal_content_ts
            #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_ts
            #read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_content_ts
            #create_into_pg_json_type_option_vec_where_length_equal_content_ts
            #create_into_pg_json_type_option_vec_where_length_greater_than_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_content_ts
            #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_content_ts
        }
    }
}
#[must_use]
pub fn pg_crud_common_query_part_error_checked_add_initialization_ts() -> Ts2 {
    quote! {pg_crud_common::QueryPartError::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }}
}
pub fn gen_impl_crate_is_string_empty_for_ident_content_ts(
    ident: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl pg_crud_common::IsStringEmpty for #ident {
            fn is_string_empty(&self) -> bool {
                #content_ts
            }
        }
    }
}
pub fn gen_match_try_new_in_deserialize_ts(
    ident: &dyn ToTokens,
    initialization_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        match #ident::try_new(#initialization_ts) {
            Ok(value) => Ok(value),
            Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
        }
    }
}
pub fn gen_impl_serde_deserialize_for_struct_ts(
    ident: &dyn DisplayPlusToTokens,
    vec_ident_type: &[(&Ident, &Type)],
    len: usize,
    gen_type_ts: &dyn Fn(&Ident, &Type) -> Ts2,
) -> Ts2 {
    fn gen_underscore_underscore_field_index_str(index: usize) -> String {
        format!("__field{index}")
    }
    fn gen_underscore_underscore_field_index_ts(index: usize) -> Ts2 {
        gen_underscore_underscore_field_index_str(index)
            .parse::<Ts2>()
            .expect("ff7433a3")
    }
    fn gen_underscore_underscore_field_index_handle_ts(index: usize) -> Ts2 {
        format!(
            "{}_handle",
            gen_underscore_underscore_field_index_str(index)
        )
        .parse::<Ts2>()
        .expect("09a0c518")
    }
    fn gen_field_ident_dq_serde_private_ok_field_ts(
        field_name_dq_ts: &dyn ToTokens,
        index: usize,
    ) -> Ts2 {
        let field_index_ts = gen_underscore_underscore_field_index_ts(index);
        quote! {#field_name_dq_ts => Ok(__Field::#field_index_ts)}
    }
    let vec_ident = vec_ident_type
        .iter()
        .map(|el_00a99fdb| el_00a99fdb.0)
        .collect::<Vec<&Ident>>();
    let field_enum_variants_ts = {
        let field_enum_variants_ts = (0..len)
            .map(|i| format!("__{FieldSc}{i}").parse::<Ts2>().expect("c46314b0"))
            .collect::<Vec<Ts2>>();
        quote! {#(#field_enum_variants_ts),*}
    };
    let visit_u64_value_enum_variants_ts = {
        let visit_u64_value_enum_variants_ts = (0..len).map(|index| {
            let index_u64_ts = {
                let value = format!("{index}u64");
                value.parse::<Ts2>().expect("828ff7b4")
            };
            let field_index_ts = gen_underscore_underscore_field_index_ts(index);
            quote! {#index_u64_ts => Ok(__Field::#field_index_ts)}
        });
        quote! {#(#visit_u64_value_enum_variants_ts),*}
    };
    let visit_str_value_enum_variants_ts = {
        let visit_str_value_enum_variants_ts =
            vec_ident.iter().enumerate().map(|(index, element)| {
                let field_name_dq_ts = dq_ts(&element);
                gen_field_ident_dq_serde_private_ok_field_ts(&field_name_dq_ts, index)
            });
        quote! {#(#visit_str_value_enum_variants_ts),*,}
    };
    let visit_bytes_value_enum_variants_ts = {
        let visit_bytes_value_enum_variants_ts =
            vec_ident.iter().enumerate().map(|(index, element)| {
                let b_field_name_dq_ts = {
                    let el_ident_dq_str = dq_str(&element.to_string());
                    let value = format!("b{el_ident_dq_str}");
                    value.parse::<Ts2>().expect("9e33625e")
                };
                gen_field_ident_dq_serde_private_ok_field_ts(&b_field_name_dq_ts, index)
            });
        quote! {#(#visit_bytes_value_enum_variants_ts),*,}
    };
    let struct_ident_dq_ts = gen_struct_ident_dq_ts(&ident);
    let visit_seq_fields_initialization_ts = {
        let content_ts = vec_ident_type.iter().enumerate().map(|(index, (el_ident, el_type))| {
            let field_index_handle_ts = gen_underscore_underscore_field_index_handle_ts(index);
            let type_ts = gen_type_ts(el_ident, el_type);
            let struct_ident_options_with_dq_ts = dq_ts(&format!("struct {ident} with {len} elements"));
            quote! {
                let Some(#field_index_handle_ts) = serde::de::SeqAccess::next_element::<#type_ts>(&mut __seq)? else {
                    return Err(serde::de::Error::invalid_length(0usize, &#struct_ident_options_with_dq_ts));
                };
            }
        });
        quote! {#(#content_ts)*}
    };
    let match_try_new_in_deserialize_ts = gen_match_try_new_in_deserialize_ts(&ident, &{
        let fields_ts = (0..len).map(gen_underscore_underscore_field_index_handle_ts);
        quote! {#(#fields_ts),*}
    });
    let visit_map_fields_initialization_ts = {
        let content_ts = vec_ident_type
            .iter()
            .enumerate()
            .map(|(index, (el_ident, el_type))| {
                let type_ts = gen_type_ts(el_ident, el_type);
                let field_index_ts = gen_underscore_underscore_field_index_ts(index);
                quote! {
                    let mut #field_index_ts: Option<#type_ts> = None;
                }
            });
        quote! {#(#content_ts)*}
    };
    let visit_map_match_variants_ts = {
        let visit_map_match_variants_ts = vec_ident_type.iter().enumerate().map(|(index, (el_ident, el_type))| {
            let field_index_ts = gen_underscore_underscore_field_index_ts(index);
            let field_ident_dq_ts = dq_ts(&el_ident);
            let type_ts = gen_type_ts(el_ident, el_type);
            quote! {
                __Field::#field_index_ts => {
                    if Option::is_some(&#field_index_ts) {
                        return Err(
                            <__A::Error as serde::de::Error>::duplicate_field(#field_ident_dq_ts),
                        );
                    }
                    #field_index_ts = Some(
                        serde::de::MapAccess::next_value::<#type_ts>(&mut __map)?,
                    );
                }
            }
        });
        quote! {#(#visit_map_match_variants_ts)*}
    };
    let visit_map_missing_fields_check_ts = {
        let content_ts = vec_ident.iter().enumerate().map(|(index, el_a1d37c97)| {
            let field_index_ts = gen_underscore_underscore_field_index_ts(index);
            let field_index_handle_ts = gen_underscore_underscore_field_index_handle_ts(index);
            let field_ident_dq_ts = dq_ts(&el_a1d37c97);
            quote! {
                let #field_index_handle_ts = match #field_index_ts {
                    Some(value_4f8faf03) => value_4f8faf03,
                    None => {
                        serde::__private228::de::missing_field(#field_ident_dq_ts)?
                    }
                };
            }
        });
        quote! {#(#content_ts)*}
    };
    let fields_array_elements_ts = {
        let fields_array_elements_ts = vec_ident.iter().map(|el_43a33e0b| dq_ts(&el_43a33e0b));
        quote! {#(#fields_array_elements_ts),*}
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
                        #field_enum_variants_ts,
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
                            __value: u64,
                        ) -> Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #visit_u64_value_enum_variants_ts,
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #visit_str_value_enum_variants_ts
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #visit_bytes_value_enum_variants_ts
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
                            #visit_seq_fields_initialization_ts
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
                            #visit_map_fields_initialization_ts
                            while let Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    #visit_map_match_variants_ts
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
                    const FIELDS: &[&str] = &[#fields_array_elements_ts];
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
pub fn wrap_content_into_scopes_ts(content_ts: &dyn ToTokens) -> Ts2 {
    quote! {(#content_ts)}
}
pub fn maybe_wrap_into_braces_ts(content_ts: &dyn ToTokens, bool: bool) -> Ts2 {
    if bool {
        wrap_content_into_scopes_ts(&content_ts)
    } else {
        quote! {#content_ts}
    }
}
pub fn gen_value_initialization_ts(import_path: &ImportPath, content_ts: &dyn ToTokens) -> Ts2 {
    quote! {#import_path::Value { #ValueSc: #content_ts }}
}
pub fn impl_pg_type_equal_operator_for_ident_ts(
    import_path: &ImportPath,
    ident: &dyn ToTokens,
    content_ts: &dyn ToTokens,
) -> Ts2 {
    quote! {
        impl #import_path::#PgTypeEqualOperatorUcc for #ident {
            fn operator(&self) -> #import_path::#EqualOperatorUcc {
                #content_ts
            }
        }
    }
}
#[must_use]
pub fn gen_query_part_error_write_into_buffer_ts(import_path: ImportPath) -> Ts2 {
    quote! {
        #import_path::QueryPartError::WriteIntoBuffer {
            code_occurence: error_occurence_lib::code_occurence!()
        }
    }
}
#[must_use]
pub fn gen_return_err_query_part_error_write_into_buffer_ts(import_path: ImportPath) -> Ts2 {
    let content_ts = gen_query_part_error_write_into_buffer_ts(import_path);
    quote! {return Err(#content_ts);}
}
