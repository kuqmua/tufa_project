mod filters;

pub use filters::*;

use naming::{
    AllVariantsDefaultOptionSomeVecOneElMaxPageSizeSc, AllVariantsDefaultOptionSomeVecOneElSc,
    ColumnNameAndMaybeFieldGetterForErrorMessageSc, ColumnNameAndMaybeFieldGetterSc, ColumnSc,
    CreateForQueryUcc, CreateIntoPostgresqlJsonTypeOptionVecWhereLengthEqualSc,
    CreateIntoPostgresqlJsonTypeOptionVecWhereLengthGreaterThanSc,
    CreateIntoPostgresqlTypeOptionVecWhereDimensionOneEqualSc, CreateQueryBindSc,
    CreateQueryPartSc, CreateSc, CreateTableColumnQueryPartSc, CreateUcc,
    DefaultOptionSomeVecOneElMaxPageSizeSc, DefaultOptionSomeVecOneElSc, EqualOperatorUcc,
    FieldIdentSc, FieldSc, IncrementSc, IsNeedToAddLogicalOperatorSc, IsPrimaryKeySc,
    JsonbSetAccumulatorSc, JsonbSetPathSc, JsonbSetTargetSc, MutSc, NormalizeSc, OptionUcc,
    OptionUpdateSc, OptionVecCreateSc, PostgresqlJsonTypeTestCasesUcc, PostgresqlJsonTypeUcc,
    PostgresqlTypeEqualOperatorUcc, PostgresqlTypeNotPrimaryKeyUcc,
    PostgresqlTypeOptionVecWhereGreaterThanTestSc, PostgresqlTypeTestCasesUcc, PostgresqlTypeUcc,
    PostgresqlTypeWhereFilterUcc, PreviousReadMergedWithOptionUpdateIntoReadSc, QueryBindSc,
    QueryPartErrorNamedUcc, QueryPartSc, QuerySc, ReadInnerIntoReadWithNewOrTryNewUnwrapedSc,
    ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc, ReadInnerUcc,
    ReadOnlyIdsIntoOptionValueReadInnerSc, ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc,
    ReadOnlyIdsMergedWithCreateIntoOptionVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereBetweenSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereContainsElGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereContainsElRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionFourEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionOneEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionThreeEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionTwoEqualSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereInSc,
    ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoReadSc, ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc,
    ReadOnlyIdsMergedWithTableTypeDeclarationIntoPostgresqlTypeOptionWhereGreaterThanSc,
    ReadOnlyIdsSc, ReadOnlyIdsToOptionValueReadDefaultOptionSomeVecOneElSc,
    ReadOnlyIdsToTwoDimensionalVecReadInnerSc, ReadOnlyIdsUcc, ReadSc, ReadUcc,
    SelectOnlyCreatedIdsQueryBindSc, SelectOnlyCreatedIdsQueryPartSc, SelectOnlyIdsQueryPartSc,
    SelectOnlyUpdatedIdsQueryBindSc, SelectOnlyUpdatedIdsQueryPartSc, SelectQueryPartSc, SelectUcc,
    SelfUcc, StdFmtDisplayPlusQuoteToTokens, TableTypeDeclarationSc, TableTypeDeclarationUcc,
    UpdateForQueryUcc, UpdateQueryBindSc, UpdateQueryPartSc, UpdateToReadOnlyIdsSc, UpdateUcc,
    ValueSc, ValueUcc, WhereUcc,
    parameter::{SelfCreateUcc, SelfSelectUcc, SelfWhereUcc},
};
use quote::quote;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum DeriveOrImpl {
    Derive,
    Impl(proc_macro2::TokenStream),
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
    serde::Serialize,
    serde::Deserialize,
    strum_macros::Display,
    strum_macros::EnumIter,
    enum_extension_lib::EnumExtension,
)]
pub enum NotNullOrNullable {
    #[default]
    NotNull,
    Nullable,
}
impl NotNullOrNullable {
    #[must_use]
    pub fn maybe_option_wrap(
        &self,
        content_ts: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        match &self {
            Self::NotNull => content_ts,
            Self::Nullable => quote! {Option<#content_ts>},
        }
    }
    #[must_use]
    pub fn maybe_some_wrap(
        &self,
        content_ts: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        match &self {
            Self::NotNull => content_ts,
            Self::Nullable => quote! {Some(#content_ts)},
        }
    }
    #[must_use]
    pub fn prefix_stringified(&self) -> String {
        match &self {
            Self::NotNull => String::default(),
            Self::Nullable => String::from("StdOptionOption"),
        }
    }
    #[must_use]
    pub fn rust(&self) -> &'static dyn Display {
        match &self {
            Self::NotNull => &"",
            Self::Nullable => &OptionUcc,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ImportPath {
    Crate,
    PostgresqlCrud,
    PostgresqlCrudCommon,
}
impl ImportPath {
    fn all_variants_default_option_some_vec_one_el(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateAllEnumVariantsArrayDefaultOptionSomeVecOneEl,
            Self::PostgresqlCrud => {
                &token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultOptionSomeVecOneEl
            }
            Self::PostgresqlCrudCommon => {
                &token_patterns::PostgresqlCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneEl
            }
        }
    }
    fn all_variants_default_option_some_vec_one_el_max_page_size(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize,
            Self::PostgresqlCrud => &token_patterns::PostgresqlCrudAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize,
            Self::PostgresqlCrudCommon => &token_patterns::PostgresqlCrudCommonAllEnumVariantsArrayDefaultOptionSomeVecOneElMaxPageSize,
        }
    }
    fn default_option_some_vec_one_el(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDefaultOptionSomeVecOneEl,
            Self::PostgresqlCrud => &token_patterns::PostgresqlCrudDefaultOptionSomeVecOneEl,
            Self::PostgresqlCrudCommon => {
                &token_patterns::PostgresqlCrudCommonDefaultOptionSomeVecOneEl
            }
        }
    }
    fn default_option_some_vec_one_el_max_page_size(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDefaultOptionSomeVecOneElMaxPageSize,
            Self::PostgresqlCrud => {
                &token_patterns::PostgresqlCrudDefaultOptionSomeVecOneElMaxPageSize
            }
            Self::PostgresqlCrudCommon => {
                &token_patterns::PostgresqlCrudCommonDefaultOptionSomeVecOneElMaxPageSize
            }
        }
    }
    #[must_use]
    pub const fn sc_std_primitive_str(&self) -> &'static str {
        match &self {
            Self::Crate => "crate",
            Self::PostgresqlCrud => "postgresql_crud",
            Self::PostgresqlCrudCommon => "postgresql_crud_common",
        }
    }
    #[must_use]
    pub const fn to_path(&self) -> &'static str {
        match &self {
            Self::Crate => "crate",
            Self::PostgresqlCrud => "postgresql_crud",
            Self::PostgresqlCrudCommon => "postgresql_crud_common",
        }
    }
}
impl quote::ToTokens for ImportPath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.sc_std_primitive_str()
            .parse::<proc_macro2::TokenStream>()
            .expect("d8636ee5-942b-472d-a025-c6e0700e1b59")
            .to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ShouldDeriveSchemarsJsonSchema {
    False,
    True,
}
impl quote::ToTokens for ShouldDeriveSchemarsJsonSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
            Self::True => quote! {, schemars::JsonSchema}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ShouldDeriveUtoipaToSchema {
    False,
    True,
}
impl quote::ToTokens for ShouldDeriveUtoipaToSchema {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
            Self::True => quote! {, utoipa::ToSchema}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsCreateQueryBindMutable {
    False,
    True,
}
impl quote::ToTokens for IsCreateQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectQueryPartSelfSelectUsed {
    False,
    True,
}
impl quote::ToTokens for IsSelectQueryPartSelfSelectUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => {
                ColumnNameAndMaybeFieldGetterForErrorMessageSc.to_tokens(tokens);
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectQueryPartIsPostgresqlTypeUsed {
    False,
    True,
}
impl quote::ToTokens for IsSelectQueryPartIsPostgresqlTypeUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => quote! {_}.to_tokens(tokens),
            Self::True => quote! {is_postgresql_type}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsUpdateQueryPartSelfUpdateUsed {
    False,
    True,
}
impl quote::ToTokens for IsUpdateQueryPartSelfUpdateUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for IsUpdateQueryPartJsonbSetTargetUsed {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for IsUpdateQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectOnlyUpdatedIdsQueryBindMutable {
    False,
    True,
}
impl quote::ToTokens for IsSelectOnlyUpdatedIdsQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsSelectOnlyCreatedIdsQueryBindMutable {
    False,
    True,
}
impl quote::ToTokens for IsSelectOnlyCreatedIdsQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IsQueryBindMutable {
    False,
    True,
}
impl quote::ToTokens for IsQueryBindMutable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => proc_macro2::TokenStream::new().to_tokens(tokens),
            Self::True => MutSc.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum IncrementParameterUnderscore {
    False,
    True,
}
impl quote::ToTokens for IncrementParameterUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for ColumnParameterUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for IsNeedToAddLogicalOperatorUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
    pub fn ucc(&self) -> &dyn StdFmtDisplayPlusQuoteToTokens {
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
impl quote::ToTokens for IsPrimaryKeyUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => IsPrimaryKeySc.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum PostgresqlTypeOrPostgresqlJsonType {
    PostgresqlJsonType,
    PostgresqlType,
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
    pub fn to_tokens_path(&self, import_path: &ImportPath) -> proc_macro2::TokenStream {
        let equal_operator_ucc = EqualOperatorUcc;
        let content_ts = match &self {
            Self::Equal => quote! {Equal},
            Self::IsNull => quote! {IsNull},
        };
        quote! {#import_path::#equal_operator_ucc::#content_ts}
    }
}
//todo maybe reuse with other structs
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    One,
    Two,
    Three,
    Four,
}
impl Dimension {
    #[must_use]
    pub fn read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_sc(
        &self,
    ) -> Box<dyn StdFmtDisplayPlusQuoteToTokens> {
        match self {
            Self::One => Box::new(ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionOneEqualSc),
            Self::Two => Box::new(ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionTwoEqualSc),
            Self::Three => Box::new(ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionThreeEqualSc),
            Self::Four => Box::new(ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionFourEqualSc),
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy)]
pub enum DimensionIndexNumber {
    Zero,
    One,
    Two,
    Three,
}
impl From<&Dimension> for DimensionIndexNumber {
    fn from(value: &Dimension) -> Self {
        match &value {
            Dimension::One => Self::Zero,
            Dimension::Two => Self::One,
            Dimension::Three => Self::Two,
            Dimension::Four => Self::Three,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum CreateQueryPartValueUnderscore {
    False,
    True,
}
impl quote::ToTokens for CreateQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for CreateQueryPartIncrementUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for CreateQueryBindValueUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for SelectQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for UpdateQueryPartValueUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for UpdateQueryPartJsonbSetAccumulatorUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for UpdateQueryPartJsonbSetTargetUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
impl quote::ToTokens for UpdateQueryPartJsonbSetPathUnderscore {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::False => quote! {jsonb_set_path}.to_tokens(tokens),
            Self::True => quote! {_}.to_tokens(tokens),
        }
    }
}
pub fn generate_postgresql_type_where_ts(
    attributes_ts: &dyn quote::ToTokens,
    variants: &Vec<&dyn PostgresqlFilter>,
    prefix: &dyn quote::ToTokens,
    should_derive_utoipa_to_schema: &ShouldDeriveUtoipaToSchema,
    should_derive_schemars_json_schema: &ShouldDeriveSchemarsJsonSchema,
    is_query_bind_mutable: &IsQueryBindMutable,
) -> proc_macro2::TokenStream {
    let ident = SelfWhereUcc::from_tokens(&prefix);
    let value_sc = ValueSc;
    let column_sc = ColumnSc;
    let increment_sc = IncrementSc;
    let query_sc = QuerySc;
    let is_need_to_add_logical_operator_sc = IsNeedToAddLogicalOperatorSc;
    let postgresql_type_tokens_where_ts = {
        let variants_ts = variants.iter().map(|el_a9dc0e35| {
            let el_ucc = el_a9dc0e35.ucc();
            let prefix_where_self_ucc = el_a9dc0e35.prefix_where_self_ucc();
            let option_type_ts: Option<proc_macro2::TokenStream> = el_a9dc0e35.maybe_generic();
            let type_ts = option_type_ts.map_or_else(
                proc_macro2::TokenStream::new,
                |value_0016edb0| quote! {<#value_0016edb0>},
            );
            quote! {#el_ucc(where_filters::#prefix_where_self_ucc #type_ts)}
        });
        quote! {
            #attributes_ts
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize #should_derive_utoipa_to_schema #should_derive_schemars_json_schema)]
            pub enum #ident {
                #(#variants_ts),*
            }
        }
    };
    let impl_postgresql_type_postgresql_type_where_filter_for_postgresql_type_tokens_where_ts =
        impl_postgresql_type_where_filter_for_ident_ts(
            &quote! {<'lifetime>},
            &ident,
            &proc_macro2::TokenStream::new(),
            &IncrementParameterUnderscore::False,
            &ColumnParameterUnderscore::False,
            &IsNeedToAddLogicalOperatorUnderscore::False,
            &{
                let variants_ts = variants.iter().map(|el_8bf490d9| {
                let el_ucc = el_8bf490d9.ucc();
                quote! {
                    Self::#el_ucc(#value_sc) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_part(
                        #value_sc,
                        #increment_sc,
                        #column_sc,
                        #is_need_to_add_logical_operator_sc,
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
                    Self::#el_ucc(#value_sc) => postgresql_crud_common::PostgresqlTypeWhereFilter::query_bind(
                        #value_sc,
                        #query_sc
                    )
                }
            });
                quote! {
                    match self {
                        #(#variants_ts),*
                    }
                }
            },
            &ImportPath::PostgresqlCrudCommon,
        );
    let impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_ts =
        macros_helpers::generate_impl_error_occurence_lib_to_std_string_string_ts(
            &proc_macro2::TokenStream::new(),
            &ident,
            &proc_macro2::TokenStream::new(),
            &quote! {format!("{self:#?}")},
        );
    let impl_all_variants_default_option_some_vec_one_el_for_postgresql_type_tokens_where_ts =
        generate_impl_postgresql_crud_common_all_variants_default_option_some_vec_one_el_ts(
            &ident,
            &{
                let variants_ts = variants.iter().map(|el_b9724130| {
                    let el_ucc = el_b9724130.ucc();
                    let default_option_some_vec_one_el_call_ts =
                        token_patterns::PostgresqlCrudCommonDefaultOptionSomeVecOneElCall;
                    quote! {
                        Self::#el_ucc(#default_option_some_vec_one_el_call_ts)
                    }
                });
                quote! {vec![#(#variants_ts),*]}
            },
        );
    quote! {
        #postgresql_type_tokens_where_ts
        #impl_postgresql_type_postgresql_type_where_filter_for_postgresql_type_tokens_where_ts
        #impl_error_occurence_lib_to_std_string_string_for_postgresql_type_tokens_where_ts
        #impl_all_variants_default_option_some_vec_one_el_for_postgresql_type_tokens_where_ts
    }
}
#[must_use]
pub fn postgresql_crud_common_query_part_error_named_ts() -> proc_macro2::TokenStream {
    let query_part_error_named_ucc = QueryPartErrorNamedUcc;
    quote! {postgresql_crud_common::#query_part_error_named_ucc}
}
pub fn generate_struct_ident_double_quotes_ts(value: &dyn Display) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_ts(&format!("struct {value}"))
}
pub fn generate_struct_ident_with_number_elements_double_quotes_ts(
    ident: &dyn StdFmtDisplayPlusQuoteToTokens,
    length: usize,
) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_ts(&format!("struct {ident} with {length} elements"))
}
pub fn generate_tuple_struct_ident_double_quotes_ts(
    value: &dyn Display,
) -> proc_macro2::TokenStream {
    generate_quotes::double_quotes_ts(&format!("tuple struct {value}"))
}
pub fn generate_sqlx_types_json_type_declaration_ts(
    type_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote! {sqlx::types::Json<#type_ts>}
}
pub fn generate_std_option_option_tokens_declaration_ts(
    type_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote! {Option<#type_ts>}
}
pub fn generate_std_vec_vec_tokens_declaration_ts(
    type_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote! {Vec<#type_ts>}
}

pub fn generate_serde_deserialize_double_quotes_ts(
    ident: &dyn StdFmtDisplayPlusQuoteToTokens,
    length: usize,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let struct_postgresql_type_ident_where_tokens_double_quotes_ts =
        generate_struct_ident_double_quotes_ts(ident);
    let struct_postgresql_type_ident_where_tokens_with_number_elements_double_quotes_ts =
        generate_struct_ident_with_number_elements_double_quotes_ts(ident, length);
    let postgresql_type_ident_where_tokens_double_quotes_ts =
        generate_quotes::double_quotes_ts(&ident);
    (
        struct_postgresql_type_ident_where_tokens_double_quotes_ts,
        struct_postgresql_type_ident_where_tokens_with_number_elements_double_quotes_ts,
        postgresql_type_ident_where_tokens_double_quotes_ts,
    )
}
pub fn generate_impl_postgresql_json_type_ts(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    table_type_declaration_type_ts: &dyn quote::ToTokens,
    create_type_ts: &dyn quote::ToTokens,
    create_for_query_type_ts: &dyn quote::ToTokens,
    select_type_ts: &dyn quote::ToTokens,
    is_select_query_part_self_select_used: &IsSelectQueryPartSelfSelectUsed,
    is_select_query_part_column_name_and_maybe_field_getter_for_error_message_used: &IsSelectQueryPartColumnNameAndMaybeFieldGetterForErrorMessageUsed,
    is_select_query_part_is_postgresql_type_used: &IsSelectQueryPartIsPostgresqlTypeUsed,
    select_query_part_ts: &dyn quote::ToTokens,
    where_type_ts: &dyn quote::ToTokens,
    read_type_ts: &dyn quote::ToTokens,
    read_only_ids_type_ts: &dyn quote::ToTokens,
    select_only_ids_query_part_ts: &dyn quote::ToTokens,
    read_inner_type_ts: &dyn quote::ToTokens,
    into_inner_ts: &dyn quote::ToTokens,
    update_type_ts: &dyn quote::ToTokens,
    update_type_for_query_ts: &dyn quote::ToTokens,
    update_query_part_ts: &dyn quote::ToTokens,
    is_update_query_part_self_update_used: &IsUpdateQueryPartSelfUpdateUsed,
    is_update_query_part_jsonb_set_target_used: &IsUpdateQueryPartJsonbSetTargetUsed,
    is_update_query_bind_mutable: &IsUpdateQueryBindMutable,
    update_query_bind_ts: &dyn quote::ToTokens,
    select_only_updated_ids_query_part_ts: &dyn quote::ToTokens,
    is_select_only_updated_ids_query_bind_mutable: &IsSelectOnlyUpdatedIdsQueryBindMutable,
    select_only_updated_ids_query_bind_ts: &dyn quote::ToTokens,
    select_only_created_ids_query_part_ts: &dyn quote::ToTokens,
    is_select_only_created_ids_query_bind_mutable: &IsSelectOnlyCreatedIdsQueryBindMutable,
    select_only_created_ids_query_bind_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let path_ts = quote! {#import_path ::};
    let table_type_declaration_ucc = TableTypeDeclarationUcc;
    let create_ucc = CreateUcc;
    let create_for_query_ucc = CreateForQueryUcc;
    let value_sc = ValueSc;
    let select_ucc = SelectUcc;
    let read_ucc = ReadUcc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let select_only_ids_query_part_sc = SelectOnlyIdsQueryPartSc;
    let read_inner_ucc = ReadInnerUcc;
    let where_ucc = WhereUcc;
    let update_ucc = UpdateUcc;
    let update_for_query_ucc = UpdateForQueryUcc;
    let increment_sc = IncrementSc;
    let postgresql_json_type_ucc = PostgresqlJsonTypeUcc;
    let query_sc = QuerySc;
    let field_ident_sc = FieldIdentSc;
    let column_name_and_maybe_field_getter_sc = ColumnNameAndMaybeFieldGetterSc;
    let jsonb_set_accumulator_sc = JsonbSetAccumulatorSc;
    let jsonb_set_path_sc = JsonbSetPathSc;
    let select_query_part_sc = SelectQueryPartSc;
    let update_query_part_sc = UpdateQueryPartSc;
    let update_query_bind_sc = UpdateQueryBindSc;
    let select_only_updated_ids_query_part_sc = SelectOnlyUpdatedIdsQueryPartSc;
    let select_only_updated_ids_query_bind_sc = SelectOnlyUpdatedIdsQueryBindSc;
    let select_only_created_ids_query_part_sc = SelectOnlyCreatedIdsQueryPartSc;
    let select_only_created_ids_query_bind_sc = SelectOnlyCreatedIdsQueryBindSc;
    let query_part_error_named_ucc = QueryPartErrorNamedUcc;
    let reference_std_primitive_str_ts = token_patterns::RefStdPrimitiveStr;
    let std_primitive_bool_ts = token_patterns::StdPrimitiveBool;
    let reference_mut_std_primitive_u64_ts = {
        let std_primitive_u64_ts = token_patterns::StdPrimitiveU64;
        quote! {&mut #std_primitive_u64_ts}
    };
    let std_string_string_ts = token_patterns::StdStringString;
    let std_primitive_u64_ts = token_patterns::StdPrimitiveU64;
    let query_postgres_arguments_ts =
        quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let query_lifetime_postgres_arguments_ts =
        quote! {sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let allow_clippy_arbitrary_source_item_ordering_ts =
        token_patterns::AllowClippyArbitrarySourceItemOrdering;
    //todo maybe reexport sqlx?
    quote! {
        #allow_clippy_arbitrary_source_item_ordering_ts
        impl #path_ts #postgresql_json_type_ucc for #ident {
            type #table_type_declaration_ucc = #table_type_declaration_type_ts;
            type #create_ucc = #create_type_ts;
            type #create_for_query_ucc = #create_for_query_type_ts;
            type #select_ucc = #select_type_ts;
            fn #select_query_part_sc(
                #is_select_query_part_self_select_used: &Self::#select_ucc,
                #field_ident_sc: #reference_std_primitive_str_ts,
                #column_name_and_maybe_field_getter_sc: #reference_std_primitive_str_ts,
                #is_select_query_part_column_name_and_maybe_field_getter_for_error_message_used: #reference_std_primitive_str_ts,
                #is_select_query_part_is_postgresql_type_used: #std_primitive_bool_ts,
            ) -> Result<#std_string_string_ts, #path_ts #query_part_error_named_ucc> {
                #select_query_part_ts
            }
            type #where_ucc = #where_type_ts;
            type #read_ucc = #read_type_ts;
            type #read_only_ids_ucc = #read_only_ids_type_ts;
            fn #select_only_ids_query_part_sc(
                #column_name_and_maybe_field_getter_sc: #reference_std_primitive_str_ts,
            ) -> Result<#std_string_string_ts, #import_path ::#query_part_error_named_ucc> {
                #select_only_ids_query_part_ts
            }
            type #read_inner_ucc = #read_inner_type_ts;
            fn into_inner(#value_sc: Self::#read_ucc) -> Self::#read_inner_ucc {
                #into_inner_ts
            }
            type #update_ucc = #update_type_ts;
            type #update_for_query_ucc = #update_type_for_query_ts;
            fn #update_query_part_sc(
                #is_update_query_part_self_update_used: &Self::#update_for_query_ucc,
                #jsonb_set_accumulator_sc: #reference_std_primitive_str_ts,
                #is_update_query_part_jsonb_set_target_used: #reference_std_primitive_str_ts,
                #jsonb_set_path_sc: #reference_std_primitive_str_ts,
                #increment_sc: #reference_mut_std_primitive_u64_ts,
            ) -> Result<#std_string_string_ts, #path_ts #query_part_error_named_ucc> {
                #update_query_part_ts
            }
            fn #update_query_bind_sc(
                #value_sc: Self::#update_for_query_ucc,
                #is_update_query_bind_mutable #query_sc: #query_postgres_arguments_ts
            ) -> Result<#query_postgres_arguments_ts, #std_string_string_ts> {
                #update_query_bind_ts
            }
            fn #select_only_updated_ids_query_part_sc(
                #value_sc: &Self::#update_for_query_ucc,
                #field_ident_sc: #reference_std_primitive_str_ts,
                #column_name_and_maybe_field_getter_sc: #reference_std_primitive_str_ts,
                #increment_sc: &mut #std_primitive_u64_ts
            ) -> Result<#std_string_string_ts, #import_path ::#query_part_error_named_ucc> {
                #select_only_updated_ids_query_part_ts
            }
            fn #select_only_updated_ids_query_bind_sc<'lifetime>(
                #value_sc: &'lifetime Self::#update_for_query_ucc,
                #is_select_only_updated_ids_query_bind_mutable #query_sc: #query_lifetime_postgres_arguments_ts
            ) -> Result<#query_lifetime_postgres_arguments_ts, #std_string_string_ts> {
                #select_only_updated_ids_query_bind_ts
            }

            fn #select_only_created_ids_query_part_sc(
                #value_sc: &Self::#create_for_query_ucc,
                #field_ident_sc: #reference_std_primitive_str_ts,
                #column_name_and_maybe_field_getter_sc: #reference_std_primitive_str_ts,
                #increment_sc: &mut #std_primitive_u64_ts
            ) -> Result<#std_string_string_ts, #import_path ::#query_part_error_named_ucc> {
                #select_only_created_ids_query_part_ts
            }
            fn #select_only_created_ids_query_bind_sc<'lifetime>(
                #value_sc: &'lifetime Self::#create_for_query_ucc,
                #is_select_only_created_ids_query_bind_mutable #query_sc: #query_lifetime_postgres_arguments_ts
            ) -> Result<#query_lifetime_postgres_arguments_ts, #std_string_string_ts> {
                #select_only_created_ids_query_bind_ts
            }
        }
    }
}
pub fn generate_impl_default_option_some_vec_one_el_ts(
    impl_generic_ts: &dyn quote::ToTokens,
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    ident_generic_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
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
pub fn generate_impl_all_variants_default_option_some_vec_one_el_ts(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
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
pub fn generate_impl_default_option_some_vec_one_el_max_page_size_ts(
    impl_generic_ts: &dyn quote::ToTokens,
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    ident_generic_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
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
pub fn generate_impl_all_variants_default_option_some_vec_one_el_max_page_size_ts(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
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
pub fn generate_impl_postgresql_crud_common_default_option_some_vec_one_el_ts(
    ident: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_option_some_vec_one_el_ts(
        &proc_macro2::TokenStream::new(),
        &ImportPath::PostgresqlCrudCommon,
        ident,
        &proc_macro2::TokenStream::new(),
        content_ts,
    )
}
pub fn generate_impl_postgresql_crud_default_option_some_vec_one_el_ts(
    ident: &dyn quote::ToTokens,
    lifetime_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_option_some_vec_one_el_ts(
        &proc_macro2::TokenStream::new(),
        &ImportPath::PostgresqlCrud,
        ident,
        lifetime_ts,
        content_ts,
    )
}
pub fn generate_impl_postgresql_crud_common_all_variants_default_option_some_vec_one_el_ts(
    ident: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_all_variants_default_option_some_vec_one_el_ts(
        &ImportPath::PostgresqlCrudCommon,
        ident,
        content_ts,
    )
}
pub fn generate_impl_postgresql_crud_all_variants_default_option_some_vec_one_el_ts(
    ident: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_all_variants_default_option_some_vec_one_el_ts(
        &ImportPath::PostgresqlCrud,
        ident,
        content_ts,
    )
}
pub fn generate_impl_postgresql_crud_common_default_option_some_vec_one_el_max_page_size_ts(
    ident: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_option_some_vec_one_el_max_page_size_ts(
        &proc_macro2::TokenStream::new(),
        &ImportPath::PostgresqlCrudCommon,
        ident,
        &proc_macro2::TokenStream::new(),
        content_ts,
    )
}
pub fn generate_impl_postgresql_crud_default_option_some_vec_one_el_max_page_size_ts(
    ident: &dyn quote::ToTokens,
    lifetime_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_default_option_some_vec_one_el_max_page_size_ts(
        &proc_macro2::TokenStream::new(),
        &ImportPath::PostgresqlCrud,
        ident,
        lifetime_ts,
        content_ts,
    )
}
pub fn generate_impl_postgresql_crud_all_variants_default_option_some_vec_one_el_max_page_size_ts(
    ident: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_impl_all_variants_default_option_some_vec_one_el_max_page_size_ts(
        &ImportPath::PostgresqlCrud,
        ident,
        content_ts,
    )
}
pub fn impl_postgresql_type_where_filter_for_ident_ts(
    impl_generic_ts: &dyn quote::ToTokens,
    ident_ts: &dyn quote::ToTokens,
    ident_generic_ts: &dyn quote::ToTokens,
    increment_parameter_underscore: &IncrementParameterUnderscore,
    column_parameter_underscore: &ColumnParameterUnderscore,
    is_need_to_add_logical_operator_underscore: &IsNeedToAddLogicalOperatorUnderscore,
    query_part_content_ts: &dyn quote::ToTokens,
    is_query_bind_mutable: &IsQueryBindMutable,
    query_bind_content_ts: &dyn quote::ToTokens,
    import_path: &ImportPath,
) -> proc_macro2::TokenStream {
    let std_primitive_u64_ts = token_patterns::StdPrimitiveU64;
    let std_fmt_display_ts = token_patterns::StdFmtDisplay;
    let std_primitive_bool_ts = token_patterns::StdPrimitiveBool;
    let std_string_string_ts = token_patterns::StdStringString;
    let query_part_error_named_ucc = QueryPartErrorNamedUcc;
    let query_part_sc = QueryPartSc;
    let query_bind_sc = QueryBindSc;
    let postgresql_type_where_filter_ucc = PostgresqlTypeWhereFilterUcc;
    let allow_clippy_arbitrary_source_item_ordering_ts =
        token_patterns::AllowClippyArbitrarySourceItemOrdering;
    quote! {
        #allow_clippy_arbitrary_source_item_ordering_ts
        impl #impl_generic_ts #import_path ::#postgresql_type_where_filter_ucc<'lifetime> for #ident_ts #ident_generic_ts {
            fn #query_part_sc(
                &self,
                #increment_parameter_underscore: &mut #std_primitive_u64_ts,
                #column_parameter_underscore: &dyn #std_fmt_display_ts,
                #is_need_to_add_logical_operator_underscore: #std_primitive_bool_ts
            ) -> Result<#std_string_string_ts, #import_path::#query_part_error_named_ucc> {
                #query_part_content_ts
            }
            fn #query_bind_sc(self, #is_query_bind_mutable query: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
                sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>,
                String
            > {
                #query_bind_content_ts
            }
        }
    }
}

pub fn generate_impl_sqlx_encode_sqlx_postgres_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote! {
        impl sqlx::Encode<'_, sqlx::Postgres> for #ident_ts {
            fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&#content_ts, buf)
            }
        }
    }
}
pub fn generate_impl_sqlx_decode_sqlx_postgres_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    type_ts: &dyn quote::ToTokens,
    ok_value_match_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let value_sc = ValueSc;
    quote! {
        impl sqlx::Decode<'_, sqlx::Postgres> for #ident_ts {
            fn decode(#value_sc: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
                match <#type_ts as sqlx::Decode<sqlx::Postgres>>::decode(#value_sc) {
                    Ok(value_147c3532) => #ok_value_match_ts,
                    Err(error) => Err(error),
                }
            }
        }
    }
}
pub fn generate_impl_sqlx_type_sqlx_postgres_for_ident_ts(
    ident_ts: &dyn quote::ToTokens,
    type_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
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
pub fn generate_impl_postgresql_type_ts(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    ident_table_type_declaration_ucc: &dyn quote::ToTokens,
    is_primary_key_underscore: &IsPrimaryKeyUnderscore,
    create_table_column_query_part_ts: &dyn quote::ToTokens,
    ident_create_ucc: &dyn quote::ToTokens,
    create_query_part_value_underscore: &CreateQueryPartValueUnderscore,
    create_query_part_increment_underscore: &CreateQueryPartIncrementUnderscore,
    create_query_part_content_ts: &dyn quote::ToTokens,
    create_query_bind_value_underscore: &CreateQueryBindValueUnderscore,
    is_create_query_bind_mutable: &IsCreateQueryBindMutable,
    create_query_bind_content_ts: &dyn quote::ToTokens,
    ident_select_ucc: &dyn quote::ToTokens,
    select_query_part_value_underscore: &SelectQueryPartValueUnderscore,
    select_query_part_content_ts: &dyn quote::ToTokens,
    ident_where_ucc: &dyn quote::ToTokens,
    ident_read_ucc: &dyn quote::ToTokens,
    normalize_ts: &dyn quote::ToTokens,
    read_only_ids_ts: &dyn quote::ToTokens,
    select_only_ids_query_part_ts: &dyn quote::ToTokens,
    ident_read_inner_ucc: &dyn quote::ToTokens,
    into_inner_ts: &dyn quote::ToTokens,
    ident_update_ucc: &dyn quote::ToTokens,
    ident_update_for_query_ucc: &dyn quote::ToTokens,
    update_query_part_value_underscore: &UpdateQueryPartValueUnderscore,
    update_query_part_jsonb_set_accumulator_underscore: &UpdateQueryPartJsonbSetAccumulatorUnderscore,
    update_query_part_jsonb_set_target_underscore: &UpdateQueryPartJsonbSetTargetUnderscore,
    update_query_part_jsonb_set_path_underscore: &UpdateQueryPartJsonbSetPathUnderscore,
    update_query_part_content_ts: &dyn quote::ToTokens,
    is_update_query_bind_mutable: &IsUpdateQueryBindMutable,
    update_query_bind_content_ts: &dyn quote::ToTokens,
    select_only_updated_ids_query_part_ts: &dyn quote::ToTokens,
    is_select_only_updated_ids_query_bind_mutable: &IsSelectOnlyUpdatedIdsQueryBindMutable,
    select_only_updated_ids_query_bind_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_ucc = PostgresqlTypeUcc;
    let table_type_declaration_ucc = TableTypeDeclarationUcc;
    let create_table_column_query_part_sc = CreateTableColumnQueryPartSc;
    let create_ucc = CreateUcc;
    let create_query_part_sc = CreateQueryPartSc;
    let create_query_bind_sc = CreateQueryBindSc;
    let select_ucc = SelectUcc;
    let select_query_part_sc = SelectQueryPartSc;
    let where_ucc = WhereUcc;
    let read_ucc = ReadUcc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let select_only_ids_query_part_sc = SelectOnlyIdsQueryPartSc;
    let normalize_sc = NormalizeSc;
    let read_inner_ucc = ReadInnerUcc;
    let update_ucc = UpdateUcc;
    let update_for_query_ucc = UpdateForQueryUcc;
    let update_query_part_sc = UpdateQueryPartSc;
    let update_query_bind_sc = UpdateQueryBindSc;
    let value_sc = ValueSc;
    let increment_sc = IncrementSc;
    let query_sc = QuerySc;
    let column_sc = ColumnSc;
    let select_only_updated_ids_query_part_sc = SelectOnlyUpdatedIdsQueryPartSc;
    let select_only_updated_ids_query_bind_sc = SelectOnlyUpdatedIdsQueryBindSc;
    let query_part_error_named_ucc = QueryPartErrorNamedUcc;
    let std_string_string_ts = token_patterns::StdStringString;
    let std_primitive_u64_ts = token_patterns::StdPrimitiveU64;
    let reference_std_primitive_str_ts = token_patterns::RefStdPrimitiveStr;
    let query_postgres_arguments_ts =
        quote! {sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>};
    let std_fmt_display_ts = token_patterns::StdFmtDisplay;
    let std_primitive_bool_ts = token_patterns::StdPrimitiveBool;
    let allow_clippy_arbitrary_source_item_ordering_ts =
        token_patterns::AllowClippyArbitrarySourceItemOrdering;
    quote! {
        #allow_clippy_arbitrary_source_item_ordering_ts
        impl #import_path :: #postgresql_type_ucc for #ident {
            type #table_type_declaration_ucc = #ident_table_type_declaration_ucc;
            fn #create_table_column_query_part_sc(#column_sc: &dyn #std_fmt_display_ts, #is_primary_key_underscore: #std_primitive_bool_ts) -> impl #std_fmt_display_ts {
                #create_table_column_query_part_ts
            }
            type #create_ucc = #ident_create_ucc;
            fn #create_query_part_sc(
                #create_query_part_value_underscore: &Self::#create_ucc,
                #create_query_part_increment_underscore: &mut #std_primitive_u64_ts
            ) -> Result<#std_string_string_ts, #import_path ::#query_part_error_named_ucc> {
                #create_query_part_content_ts
            }
            fn #create_query_bind_sc(
                #create_query_bind_value_underscore: Self::#create_ucc,
                #is_create_query_bind_mutable #query_sc: #query_postgres_arguments_ts
            ) -> Result<
                #query_postgres_arguments_ts,
                String
            > {
                #create_query_bind_content_ts
            }
            type #select_ucc = #ident_select_ucc;
            fn #select_query_part_sc(
                #select_query_part_value_underscore: &Self::#select_ucc,
                #column_sc: #reference_std_primitive_str_ts,
            ) -> Result<#std_string_string_ts, #import_path ::#query_part_error_named_ucc> {
                #select_query_part_content_ts
            }
            type #where_ucc = #ident_where_ucc;
            type #read_ucc = #ident_read_ucc;
            fn #normalize_sc(#value_sc: Self::#read_ucc) -> Self::#read_ucc {
                #normalize_ts
            }
            type #read_only_ids_ucc = #read_only_ids_ts;
            fn #select_only_ids_query_part_sc(
                #column_sc: #reference_std_primitive_str_ts
            ) -> Result<#std_string_string_ts, #import_path ::#query_part_error_named_ucc> {
                #select_only_ids_query_part_ts
            }
            type #read_inner_ucc = #ident_read_inner_ucc;
            fn into_inner(#value_sc: Self::#read_ucc) -> Self::#read_inner_ucc {
                #into_inner_ts
            }
            type #update_ucc = #ident_update_ucc;
            type #update_for_query_ucc = #ident_update_for_query_ucc;
            fn #update_query_part_sc(
                #update_query_part_value_underscore: &Self::#update_for_query_ucc,
                #update_query_part_jsonb_set_accumulator_underscore: #reference_std_primitive_str_ts,
                #update_query_part_jsonb_set_target_underscore: #reference_std_primitive_str_ts,
                #update_query_part_jsonb_set_path_underscore: #reference_std_primitive_str_ts,
                #increment_sc: &mut #std_primitive_u64_ts
            ) -> Result<#std_string_string_ts, #import_path ::#query_part_error_named_ucc> {
                #update_query_part_content_ts
            }
            fn #update_query_bind_sc(
                #value_sc: Self::#update_for_query_ucc,
                #is_update_query_bind_mutable #query_sc: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<
                sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                String
            > {
                #update_query_bind_content_ts
            }
            fn #select_only_updated_ids_query_part_sc(
                #value_sc: &Self::#update_for_query_ucc,
                #column_sc: #reference_std_primitive_str_ts,
                #increment_sc: &mut #std_primitive_u64_ts,
            ) -> Result<#std_string_string_ts, #import_path ::#query_part_error_named_ucc> {
                #select_only_updated_ids_query_part_ts
            }
            fn #select_only_updated_ids_query_bind_sc<'lifetime>(
                #value_sc: &'lifetime Self::#update_for_query_ucc,
                #is_select_only_updated_ids_query_bind_mutable #query_sc: sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>
            ) -> Result<sqlx::query::Query<'lifetime, sqlx::Postgres, sqlx::postgres::PgArguments>, String> {
                #select_only_updated_ids_query_bind_ts
            }
        }
    }
}

pub fn generate_impl_postgresql_type_not_primary_key_for_ident_ts(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_not_primary_key_ucc = PostgresqlTypeNotPrimaryKeyUcc;
    let postgresql_type_ucc = PostgresqlTypeUcc;
    let create_ucc = CreateUcc;
    let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
    let allow_clippy_arbitrary_source_item_ordering_ts =
        token_patterns::AllowClippyArbitrarySourceItemOrdering;
    quote! {
        #allow_clippy_arbitrary_source_item_ordering_ts
        impl #import_path::#postgresql_type_not_primary_key_ucc for #ident {
            type #postgresql_type_ucc = Self;
            type #create_ucc = #ident_create_ucc;
        }
    }
}

// fn generate_read_only_ids_merged_with_create_into_where_method_ts(
//     import_path: &ImportPath,
//     method_name_ts: &dyn quote::ToTokens,
//     content_ts: &dyn quote::ToTokens,
//     postgresql_type_or_postgresql_json_type: &PostgresqlTypeOrPostgresqlJsonType,
// ) -> proc_macro2::TokenStream {
//     let self_ucc = SelfUcc;
//     let read_only_ids_sc = ReadOnlyIdsSc;
//     let read_only_ids_ucc = ReadOnlyIdsUcc;
//     let create_sc = CreateSc;
//     let create_ucc = CreateUcc;
//     let where_ucc = WhereUcc;
//     let self_postgresql_type_or_postgresql_json_type_as_postgresql_json_type_ts = {
//         let postgresql_type_or_postgresql_json_type_ts: &dyn quote::ToTokens = match &postgresql_type_or_postgresql_json_type {
//             PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => &PostgresqlTypeUcc,
//             PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => &PostgresqlJsonTypeUcc,
//         };
//         quote! {
//             <#self_ucc::#postgresql_type_or_postgresql_json_type_ts as #import_path::#postgresql_type_or_postgresql_json_type_ts>
//         }
//     };
//     quote!{
//         fn #method_name_ts(
//             #read_only_ids_sc: #self_postgresql_type_or_postgresql_json_type_as_postgresql_json_type_ts::#read_only_ids_ucc,
//             #create_sc: #self_postgresql_type_or_postgresql_json_type_as_postgresql_json_type_ts::#create_ucc
//         ) -> Vec<#self_postgresql_type_or_postgresql_json_type_as_postgresql_json_type_ts::#where_ucc> {
//             #content_ts
//         }
//     }
// }

fn generate_option_vec_create_ts(
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let option_vec_create_sc = OptionVecCreateSc;
    let create_ucc = CreateUcc;
    quote! {
        fn #option_vec_create_sc() -> Option<Vec<#path_ts::#create_ucc>> {
            #content_ts
        }
    }
}
fn generate_read_only_ids_to_two_dimensional_vec_read_inner_ts(
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_to_two_dimensional_vec_read_inner_sc =
        ReadOnlyIdsToTwoDimensionalVecReadInnerSc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let read_inner_ucc = ReadInnerUcc;
    let read_only_ids_sc = ReadOnlyIdsSc;
    quote! {
        fn #read_only_ids_to_two_dimensional_vec_read_inner_sc(
            #read_only_ids_sc: &#path_ts::#read_only_ids_ucc
        ) -> Vec<Vec<#path_ts::#read_inner_ucc>> {
            #content_ts
        }
    }
}
fn generate_read_inner_into_read_with_new_or_try_new_unwraped_ts(
    type_ts: &dyn quote::ToTokens,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_inner_into_read_with_new_or_try_new_unwraped_sc =
        ReadInnerIntoReadWithNewOrTryNewUnwrapedSc;
    let value_sc = ValueSc;
    let read_ucc = ReadUcc;
    quote! {
        fn #read_inner_into_read_with_new_or_try_new_unwraped_sc(
            #value_sc: #type_ts
        ) -> #path_ts::#read_ucc {
            #content_ts
        }
    }
}
fn generate_read_inner_into_update_with_new_or_try_new_unwraped_ts(
    type_ts: &dyn quote::ToTokens,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_inner_into_update_with_new_or_try_new_unwraped_sc =
        ReadInnerIntoUpdateWithNewOrTryNewUnwrapedSc;
    let update_ucc = UpdateUcc;
    let value_sc = ValueSc;
    quote! {
        fn #read_inner_into_update_with_new_or_try_new_unwraped_sc(#value_sc: #type_ts) -> #path_ts::#update_ucc {
            #content_ts
        }
    }
}
fn generate_update_to_read_only_ids_ts(
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let update_to_read_only_ids_sc = UpdateToReadOnlyIdsSc;
    let update_ucc = UpdateUcc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let value_sc = ValueSc;
    quote! {
        fn #update_to_read_only_ids_sc(
            #value_sc: &#path_ts::#update_ucc
        ) -> #path_ts::#read_only_ids_ucc {
            #content_ts
        }
    }
}
fn generate_read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_to_option_value_read_default_option_some_vec_one_el_sc =
        ReadOnlyIdsToOptionValueReadDefaultOptionSomeVecOneElSc;
    let value_ucc = ValueUcc;
    let value_sc = ValueSc;
    let read_ucc = ReadUcc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    quote! {
        fn #read_only_ids_to_option_value_read_default_option_some_vec_one_el_sc(
            #value_sc: &#path_ts::#read_only_ids_ucc
        ) -> Option<#import_path::#value_ucc<#path_ts::#read_ucc>> {
            #content_ts
        }
    }
}
fn generate_previous_read_merged_with_option_update_into_read_ts(
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let previous_read_merged_with_option_update_into_read_sc =
        PreviousReadMergedWithOptionUpdateIntoReadSc;
    let read_ucc = ReadUcc;
    let read_sc = ReadSc;
    let update_ucc = UpdateUcc;
    let option_update_sc = OptionUpdateSc;
    quote! {
        fn #previous_read_merged_with_option_update_into_read_sc(
            #read_sc: #path_ts::#read_ucc,
            #option_update_sc: Option<#path_ts::#update_ucc>,
        ) -> #path_ts::#read_ucc {
            #content_ts
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_read_ts(
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_read_sc = ReadOnlyIdsMergedWithCreateIntoReadSc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let read_only_ids_sc = ReadOnlyIdsSc;
    let create_ucc = CreateUcc;
    let create_sc = CreateSc;
    let read_ucc = ReadUcc;
    quote! {
        fn #read_only_ids_merged_with_create_into_read_sc(
            #read_only_ids_sc: #path_ts::#read_only_ids_ucc,
            #create_sc: #path_ts::#create_ucc
        ) -> #path_ts::#read_ucc {
            #content_ts
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_option_value_read_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_option_value_read_sc =
        ReadOnlyIdsMergedWithCreateIntoOptionValueReadSc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let read_only_ids_sc = ReadOnlyIdsSc;
    let create_ucc = CreateUcc;
    let create_sc = CreateSc;
    let value_ucc = ValueUcc;
    let read_ucc = ReadUcc;
    quote! {
        fn #read_only_ids_merged_with_create_into_option_value_read_sc(
            #read_only_ids_sc: #path_ts::#read_only_ids_ucc,
            #create_sc: #path_ts::#create_ucc
        ) -> Option<#import_path::#value_ucc<#path_ts::#read_ucc>> {
            #content_ts
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_table_type_declaration_ts(
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_table_type_declaration_sc =
        ReadOnlyIdsMergedWithCreateIntoTableTypeDeclarationSc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let read_only_ids_sc = ReadOnlyIdsSc;
    let create_ucc = CreateUcc;
    let create_sc = CreateSc;
    let table_type_declaration_ucc = TableTypeDeclarationUcc;
    quote! {
        fn #read_only_ids_merged_with_create_into_table_type_declaration_sc(
            #read_only_ids_sc: #path_ts::#read_only_ids_ucc,
            #create_sc: #path_ts::#create_ucc
        ) -> #path_ts::#table_type_declaration_ucc {
            #content_ts
        }
    }
}

pub fn generate_read_only_ids_merged_with_create_into_where_equal_ts(
    read_only_ids_ts: &dyn quote::ToTokens,
    create_ts: &dyn quote::ToTokens,
    where_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_where_equal_sc =
        ReadOnlyIdsMergedWithCreateIntoWhereEqualSc;
    let read_only_ids_sc = ReadOnlyIdsSc;
    let create_sc = CreateSc;
    quote! {
        fn #read_only_ids_merged_with_create_into_where_equal_sc(
            #read_only_ids_sc: #read_only_ids_ts,
            #create_sc: #create_ts
        ) -> #where_ts {
            #content_ts
        }
    }
}
pub fn generate_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
    import_path: &ImportPath,
    read_only_ids_ts: &dyn quote::ToTokens,
    create_ts: &dyn quote::ToTokens,
    where_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc =
        ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc;
    let read_only_ids_sc = ReadOnlyIdsSc;
    let create_sc = CreateSc;
    quote! {
        fn #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_sc(
            #read_only_ids_sc: #read_only_ids_ts,
            #create_sc: #create_ts
        ) -> #import_path::NotEmptyUniqueVec<#where_ts> {
            #content_ts
        }
    }
}

fn generate_read_only_ids_merged_with_create_into_vec_or_option_vec_where_equal_to_json_field_postgresql_type_or_postgresql_json_type_ts(
    import_path: ImportPath,
    read_only_ids_ts: &dyn quote::ToTokens,
    create_ts: &dyn quote::ToTokens,
    where_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
    postgresql_type_or_postgresql_json_type: PostgresqlTypeOrPostgresqlJsonType,
) -> proc_macro2::TokenStream {
    let read_only_ids_sc = ReadOnlyIdsSc;
    let create_sc = CreateSc;
    let return_type_ts = {
        let return_type_handle_ts = quote! {#import_path::NotEmptyUniqueVec<#where_ts>};
        match &postgresql_type_or_postgresql_json_type {
            PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => {
                generate_std_option_option_tokens_declaration_ts(&return_type_handle_ts)
            }
            PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => return_type_handle_ts,
        }
    };
    let name_ts: &dyn quote::ToTokens = match &postgresql_type_or_postgresql_json_type {
        PostgresqlTypeOrPostgresqlJsonType::PostgresqlType => {
            &ReadOnlyIdsMergedWithCreateIntoOptionVecWhereEqualToJsonFieldSc
        }
        PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType => {
            &ReadOnlyIdsMergedWithCreateIntoVecWhereEqualToJsonFieldSc
        }
    };
    quote! {
        fn #name_ts(
            #read_only_ids_sc: #read_only_ids_ts,
            #create_sc: #create_ts
        ) -> #return_type_ts {
            #content_ts
        }
    }
}
pub fn generate_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts(
    import_path: ImportPath,
    read_only_ids_ts: &dyn quote::ToTokens,
    create_ts: &dyn quote::ToTokens,
    where_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_vec_or_option_vec_where_equal_to_json_field_postgresql_type_or_postgresql_json_type_ts(
        import_path,
        &read_only_ids_ts,
        &create_ts,
        &where_ts,
        &content_ts,
        PostgresqlTypeOrPostgresqlJsonType::PostgresqlJsonType
    )
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts(
    import_path: ImportPath,
    name_ts: &dyn quote::ToTokens,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let read_only_ids_sc = ReadOnlyIdsSc;
    let create_ucc = CreateUcc;
    let create_sc = CreateSc;
    let where_ucc = WhereUcc;
    quote! {
        fn #name_ts(
            #read_only_ids_sc: #path_ts::#read_only_ids_ucc,
            #create_sc: #path_ts::#create_ucc
        ) -> Option<#import_path::NotEmptyUniqueVec<#path_ts::#where_ucc>> {
            #content_ts
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts(
        import_path,
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionOneEqualSc,
        &path_ts,
        &content_ts
    )
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts(
        import_path,
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionTwoEqualSc,
        &path_ts,
        &content_ts
    )
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts(
        import_path,
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionThreeEqualSc,
        &path_ts,
        &content_ts
    )
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_number_equal_ts(
        import_path,
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereDimensionFourEqualSc,
        &path_ts,
        &content_ts
    )
}
fn generate_create_into_postgresql_json_type_option_vec_where_length_equal_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let create_into_postgresql_json_type_option_vec_where_length_equal_sc =
        CreateIntoPostgresqlJsonTypeOptionVecWhereLengthEqualSc;
    let create_ucc = CreateUcc;
    let create_sc = CreateSc;
    let where_ucc = WhereUcc;
    quote! {
        fn #create_into_postgresql_json_type_option_vec_where_length_equal_sc(
            #create_sc: #path_ts::#create_ucc
        ) -> Option<#import_path::NotEmptyUniqueVec<#path_ts::#where_ucc>> {
            #content_ts
        }
    }
}
fn generate_create_into_postgresql_json_type_option_vec_where_length_greater_than_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let create_into_postgresql_json_type_option_vec_where_length_greater_than_sc =
        CreateIntoPostgresqlJsonTypeOptionVecWhereLengthGreaterThanSc;
    let create_ucc = CreateUcc;
    let create_sc = CreateSc;
    let where_ucc = WhereUcc;
    quote! {
        fn #create_into_postgresql_json_type_option_vec_where_length_greater_than_sc(
            #create_sc: #path_ts::#create_ucc
        ) -> Option<#import_path::NotEmptyUniqueVec<#path_ts::#where_ucc>> {
            #content_ts
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
    method_name_ts: &dyn quote::ToTokens,
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let read_only_ids_sc = ReadOnlyIdsSc;
    let create_ucc = CreateUcc;
    let create_sc = CreateSc;
    let where_ucc = WhereUcc;
    quote! {
        fn #method_name_ts(
            #read_only_ids_sc: #path_ts::#read_only_ids_ucc,
            #create_sc: #path_ts::#create_ucc
        ) -> Option<#import_path::NotEmptyUniqueVec<#import_path::SingleOrMultiple<#path_ts::#where_ucc>>> {
            #content_ts
        }
    }
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereGreaterThanSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereBetweenSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereInSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereRegularExpressionSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereContainsElGreaterThanSc,
        import_path,
        path_ts,
        content_ts,
    )
}
fn generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts(
    import_path: ImportPath,
    path_ts: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_not_empty_unique_vec_single_or_multiple_where_ts(
        &ReadOnlyIdsMergedWithCreateIntoPostgresqlJsonTypeOptionVecWhereContainsElRegularExpressionSc,
        import_path,
        path_ts,
        content_ts,
    )
}
pub fn generate_impl_postgresql_type_test_cases_for_ident_ts(
    cfg_ts: &dyn quote::ToTokens,
    import_path: &ImportPath,
    type_ts: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    option_vec_create_ts: &dyn quote::ToTokens,
    read_only_ids_to_two_dimensional_vec_read_inner_ts: &dyn quote::ToTokens,
    read_inner_into_read_with_new_or_try_new_unwraped_ts: &dyn quote::ToTokens,
    read_inner_into_update_with_new_or_try_new_unwraped_ts: &dyn quote::ToTokens,
    update_to_read_only_ids_ts: &dyn quote::ToTokens,
    read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts: &dyn quote::ToTokens,
    previous_read_merged_with_option_update_into_read_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_read_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_option_value_read_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_table_type_declaration_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_where_equal_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts: &dyn quote::ToTokens,
    create_into_postgresql_type_option_vec_where_dimension_one_equal_ts: &dyn quote::ToTokens,
    postgresql_type_option_vec_where_greater_than_test_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_length_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_length_greater_than_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_ucc = PostgresqlTypeUcc;
    let postgresql_type_test_cases_ucc = PostgresqlTypeTestCasesUcc;
    let table_type_declaration_ucc = TableTypeDeclarationUcc;
    let table_type_declaration_sc = TableTypeDeclarationSc;
    let self_ucc = SelfUcc;
    let select_ucc = SelectUcc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let read_only_ids_sc = ReadOnlyIdsSc;
    let where_ucc = WhereUcc;
    let create_ucc = CreateUcc;
    let create_sc = CreateSc;
    let self_postgresql_type_as_postgresql_type_ts =
        quote! {<#self_ucc::#postgresql_type_ucc as #import_path::#postgresql_type_ucc>};
    let self_postgresql_type_as_postgresql_type_read_only_ids_ts =
        quote! {#self_postgresql_type_as_postgresql_type_ts::#read_only_ids_ucc};
    let self_postgresql_type_as_postgresql_type_create_ts =
        quote! {#self_postgresql_type_as_postgresql_type_ts::#create_ucc};
    let self_postgresql_type_as_postgresql_type_where_ts =
        quote! {#self_postgresql_type_as_postgresql_type_ts::#where_ucc};
    let allow_clippy_arbitrary_source_item_ordering_ts =
        token_patterns::AllowClippyArbitrarySourceItemOrdering;
    let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
    let option_vec_create_content_ts = generate_option_vec_create_ts(
        &self_postgresql_type_as_postgresql_type_ts,
        &option_vec_create_ts,
    );
    let read_only_ids_to_two_dimensional_vec_read_inner_content_ts =
        generate_read_only_ids_to_two_dimensional_vec_read_inner_ts(
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_to_two_dimensional_vec_read_inner_ts,
        );
    let read_inner_into_read_with_new_or_try_new_unwraped_content_ts =
        generate_read_inner_into_read_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_inner_into_read_with_new_or_try_new_unwraped_ts,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_content_ts =
        generate_read_inner_into_update_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_inner_into_update_with_new_or_try_new_unwraped_ts,
        );
    let update_to_read_only_ids_content_ts = generate_update_to_read_only_ids_ts(
        &self_postgresql_type_as_postgresql_type_ts,
        &update_to_read_only_ids_ts,
    );
    let read_only_ids_to_option_value_read_default_option_some_vec_one_el_content_ts =
        generate_read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts,
        );
    let previous_read_merged_with_option_update_into_read_content_ts =
        generate_previous_read_merged_with_option_update_into_read_ts(
            &self_postgresql_type_as_postgresql_type_ts,
            &previous_read_merged_with_option_update_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_read_content_ts =
        generate_read_only_ids_merged_with_create_into_read_ts(
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_merged_with_create_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_option_value_read_content_ts =
        generate_read_only_ids_merged_with_create_into_option_value_read_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_merged_with_create_into_option_value_read_ts,
        );
    let read_only_ids_merged_with_create_into_table_type_declaration_content_ts =
        generate_read_only_ids_merged_with_create_into_table_type_declaration_ts(
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_merged_with_create_into_table_type_declaration_ts,
        );
    let read_only_ids_merged_with_create_into_where_equal_content_ts =
        generate_read_only_ids_merged_with_create_into_where_equal_ts(
            &self_postgresql_type_as_postgresql_type_read_only_ids_ts,
            &self_postgresql_type_as_postgresql_type_create_ts,
            &self_postgresql_type_as_postgresql_type_where_ts,
            &read_only_ids_merged_with_create_into_where_equal_ts,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_ts =
        generate_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
            import_path,
            &self_postgresql_type_as_postgresql_type_read_only_ids_ts,
            &self_postgresql_type_as_postgresql_type_create_ts,
            &self_postgresql_type_as_postgresql_type_where_ts,
            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
        );
    let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_content_ts = generate_read_only_ids_merged_with_create_into_vec_or_option_vec_where_equal_to_json_field_postgresql_type_or_postgresql_json_type_ts(
        *import_path,
        &self_postgresql_type_as_postgresql_type_read_only_ids_ts,
        &self_postgresql_type_as_postgresql_type_create_ts,
        &self_postgresql_type_as_postgresql_type_where_ts,
        &read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts,
        PostgresqlTypeOrPostgresqlJsonType::PostgresqlType,
    );
    let create_into_postgresql_type_option_vec_where_dimension_one_equal_sc =
        CreateIntoPostgresqlTypeOptionVecWhereDimensionOneEqualSc;
    let postgresql_type_option_vec_where_greater_than_test_sc =
        PostgresqlTypeOptionVecWhereGreaterThanTestSc;
    let read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_sc =
        ReadOnlyIdsMergedWithTableTypeDeclarationIntoPostgresqlTypeOptionWhereGreaterThanSc;
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_content_ts = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts(
        *import_path,
        &self_postgresql_type_as_postgresql_type_ts,
        &create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts
    );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_content_ts = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts(
        *import_path,
        &self_postgresql_type_as_postgresql_type_ts,
        &create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts
    );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_content_ts = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts(
        *import_path,
        &self_postgresql_type_as_postgresql_type_ts,
        &create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts
    );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_content_ts = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts(
        *import_path,
        &self_postgresql_type_as_postgresql_type_ts,
        &create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts
    );
    let create_into_postgresql_json_type_option_vec_where_length_equal_content_ts =
        generate_create_into_postgresql_json_type_option_vec_where_length_equal_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &create_into_postgresql_json_type_option_vec_where_length_equal_ts,
        );
    let create_into_postgresql_json_type_option_vec_where_length_greater_than_content_ts =
        generate_create_into_postgresql_json_type_option_vec_where_length_greater_than_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &create_into_postgresql_json_type_option_vec_where_length_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts(
            *import_path,
            &self_postgresql_type_as_postgresql_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts,
        );
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #allow_clippy_arbitrary_source_item_ordering_ts
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import_path::#postgresql_type_test_cases_ucc for #ident {
            type #postgresql_type_ucc = #self_ucc;
            type #select_ucc = #ident_select_ucc;
            #option_vec_create_content_ts
            #read_only_ids_to_two_dimensional_vec_read_inner_content_ts
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
            fn #create_into_postgresql_type_option_vec_where_dimension_one_equal_sc(
                #create_sc: #self_postgresql_type_as_postgresql_type_ts::#create_ucc
            ) -> Option<#import_path::NotEmptyUniqueVec<#self_postgresql_type_as_postgresql_type_ts::#where_ucc>> {
                #create_into_postgresql_type_option_vec_where_dimension_one_equal_ts
            }
            fn #postgresql_type_option_vec_where_greater_than_test_sc() -> Option<
                #import_path::NotEmptyUniqueVec<
                    #import_path::PostgresqlTypeGreaterThanTest<
                        #self_ucc::#postgresql_type_ucc
                    >
                >
            > {
                #postgresql_type_option_vec_where_greater_than_test_ts
            }
            fn #read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_sc(
                greater_than_variant: #import_path::PostgresqlTypeGreaterThanVariant,
                #read_only_ids_sc: #self_postgresql_type_as_postgresql_type_ts::#read_only_ids_ucc,
                #table_type_declaration_sc: #self_postgresql_type_as_postgresql_type_ts::#table_type_declaration_ucc,
            ) -> Option<#self_postgresql_type_as_postgresql_type_ts::#where_ucc> {
                #read_only_ids_merged_with_table_type_declaration_into_postgresql_type_option_where_greater_than_ts
            }
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_content_ts
            #create_into_postgresql_json_type_option_vec_where_length_equal_content_ts
            #create_into_postgresql_json_type_option_vec_where_length_greater_than_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_content_ts
        }
    }
}
pub fn generate_impl_postgresql_json_type_test_cases_for_ident_ts(
    cfg_ts: &dyn quote::ToTokens,
    import_path: &ImportPath,
    type_ts: &dyn quote::ToTokens,
    ident: &dyn quote::ToTokens,
    option_vec_create_ts: &dyn quote::ToTokens,
    read_only_ids_to_two_dimensional_vec_read_inner_ts: &dyn quote::ToTokens,
    read_inner_into_read_with_new_or_try_new_unwraped_ts: &dyn quote::ToTokens,
    read_inner_into_update_with_new_or_try_new_unwraped_ts: &dyn quote::ToTokens,
    read_only_ids_into_option_value_read_inner_ts: &dyn quote::ToTokens,
    update_to_read_only_ids_ts: &dyn quote::ToTokens,
    read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts: &dyn quote::ToTokens,
    previous_read_merged_with_option_update_into_read_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_read_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_option_value_read_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_table_type_declaration_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_where_equal_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_length_equal_ts: &dyn quote::ToTokens,
    create_into_postgresql_json_type_option_vec_where_length_greater_than_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts: &dyn quote::ToTokens,
    read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let allow_clippy_arbitrary_source_item_ordering_ts =
        token_patterns::AllowClippyArbitrarySourceItemOrdering;
    let value_ucc = ValueUcc;
    let value_sc = ValueSc;
    let postgresql_json_type_ucc = PostgresqlJsonTypeUcc;
    let postgresql_json_type_test_cases_ucc = PostgresqlJsonTypeTestCasesUcc;
    let read_inner_ucc = ReadInnerUcc;
    let self_ucc = SelfUcc;
    let read_only_ids_ucc = ReadOnlyIdsUcc;
    let read_only_ids_into_option_value_read_inner_sc = ReadOnlyIdsIntoOptionValueReadInnerSc;
    let create_ucc = CreateUcc;
    let select_ucc = SelectUcc;
    let where_ucc = WhereUcc;
    let self_postgresql_json_type_as_postgresql_json_type_ts =
        quote! {<#self_ucc::#postgresql_json_type_ucc as #import_path::#postgresql_json_type_ucc>};
    let self_postgresql_json_type_as_postgresql_json_type_read_only_ids_ts =
        quote! {#self_postgresql_json_type_as_postgresql_json_type_ts::#read_only_ids_ucc};
    let self_postgresql_json_type_as_postgresql_json_type_create_ts =
        quote! {#self_postgresql_json_type_as_postgresql_json_type_ts::#create_ucc};
    let self_postgresql_json_type_as_postgresql_json_type_where_ts =
        quote! {#self_postgresql_json_type_as_postgresql_json_type_ts::#where_ucc};
    let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
    let option_vec_create_content_ts = generate_option_vec_create_ts(
        &self_postgresql_json_type_as_postgresql_json_type_ts,
        &option_vec_create_ts,
    );
    let read_only_ids_to_two_dimensional_vec_read_inner_content_ts =
        generate_read_only_ids_to_two_dimensional_vec_read_inner_ts(
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_to_two_dimensional_vec_read_inner_ts,
        );
    let read_inner_into_read_with_new_or_try_new_unwraped_content_ts =
        generate_read_inner_into_read_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_inner_into_read_with_new_or_try_new_unwraped_ts,
        );
    let read_inner_into_update_with_new_or_try_new_unwraped_content_ts =
        generate_read_inner_into_update_with_new_or_try_new_unwraped_ts(
            &type_ts,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_inner_into_update_with_new_or_try_new_unwraped_ts,
        );
    let update_to_read_only_ids_content_ts = generate_update_to_read_only_ids_ts(
        &self_postgresql_json_type_as_postgresql_json_type_ts,
        &update_to_read_only_ids_ts,
    );
    let read_only_ids_to_option_value_read_default_option_some_vec_one_el_content_ts =
        generate_read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_to_option_value_read_default_option_some_vec_one_el_ts,
        );
    let previous_read_merged_with_option_update_into_read_content_ts =
        generate_previous_read_merged_with_option_update_into_read_ts(
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &previous_read_merged_with_option_update_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_read_content_ts =
        generate_read_only_ids_merged_with_create_into_read_ts(
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_merged_with_create_into_read_ts,
        );
    let read_only_ids_merged_with_create_into_option_value_read_content_ts =
        generate_read_only_ids_merged_with_create_into_option_value_read_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_merged_with_create_into_option_value_read_ts,
        );
    let read_only_ids_merged_with_create_into_table_type_declaration_content_ts =
        generate_read_only_ids_merged_with_create_into_table_type_declaration_ts(
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_merged_with_create_into_table_type_declaration_ts,
        );
    let read_only_ids_merged_with_create_into_where_equal_content_ts =
        generate_read_only_ids_merged_with_create_into_where_equal_ts(
            &self_postgresql_json_type_as_postgresql_json_type_read_only_ids_ts,
            &self_postgresql_json_type_as_postgresql_json_type_create_ts,
            &self_postgresql_json_type_as_postgresql_json_type_where_ts,
            &read_only_ids_merged_with_create_into_where_equal_ts,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_using_fields_content_ts =
        generate_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts(
            import_path,
            &self_postgresql_json_type_as_postgresql_json_type_read_only_ids_ts,
            &self_postgresql_json_type_as_postgresql_json_type_create_ts,
            &self_postgresql_json_type_as_postgresql_json_type_where_ts,
            &read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
        );
    let read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_content_ts =
        generate_read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_read_only_ids_ts,
            &self_postgresql_json_type_as_postgresql_json_type_create_ts,
            &self_postgresql_json_type_as_postgresql_json_type_where_ts,
            &read_only_ids_merged_with_create_into_vec_where_equal_to_json_field_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_content_ts = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts(
        *import_path,
        &self_postgresql_json_type_as_postgresql_json_type_ts,
        &create_into_postgresql_json_type_option_vec_where_dimension_one_equal_ts
    );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_content_ts = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts(
        *import_path,
        &self_postgresql_json_type_as_postgresql_json_type_ts,
        &create_into_postgresql_json_type_option_vec_where_dimension_two_equal_ts
    );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &create_into_postgresql_json_type_option_vec_where_dimension_three_equal_ts
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_content_ts = generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts(
        *import_path,
        &self_postgresql_json_type_as_postgresql_json_type_ts,
        &create_into_postgresql_json_type_option_vec_where_dimension_four_equal_ts
    );
    let create_into_postgresql_json_type_option_vec_where_length_equal_content_ts =
        generate_create_into_postgresql_json_type_option_vec_where_length_equal_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &create_into_postgresql_json_type_option_vec_where_length_equal_ts,
        );
    let create_into_postgresql_json_type_option_vec_where_length_greater_than_content_ts =
        generate_create_into_postgresql_json_type_option_vec_where_length_greater_than_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &create_into_postgresql_json_type_option_vec_where_length_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_ts,
        );
    let read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_content_ts =
        generate_read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts(
            *import_path,
            &self_postgresql_json_type_as_postgresql_json_type_ts,
            &read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_ts,
        );
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #allow_clippy_arbitrary_source_item_ordering_ts
        #cfg_ts
        #[allow(clippy::float_arithmetic)]
        impl #import_path::#postgresql_json_type_test_cases_ucc for #ident {
            type #postgresql_json_type_ucc = #self_ucc;
            type #select_ucc = #ident_select_ucc;
            #option_vec_create_content_ts
            #read_only_ids_to_two_dimensional_vec_read_inner_content_ts
            #read_inner_into_read_with_new_or_try_new_unwraped_content_ts
            #read_inner_into_update_with_new_or_try_new_unwraped_content_ts
            fn #read_only_ids_into_option_value_read_inner_sc(
                #value_sc: #self_postgresql_json_type_as_postgresql_json_type_ts::#read_only_ids_ucc
            ) -> Option<#import_path::#value_ucc<#self_postgresql_json_type_as_postgresql_json_type_ts::#read_inner_ucc>> {
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
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_one_equal_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_two_equal_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_three_equal_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_dimension_four_equal_content_ts
            #create_into_postgresql_json_type_option_vec_where_length_equal_content_ts
            #create_into_postgresql_json_type_option_vec_where_length_greater_than_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_greater_than_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_between_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_in_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_regular_expression_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_greater_than_content_ts
            #read_only_ids_merged_with_create_into_postgresql_json_type_option_vec_where_contains_el_regular_expression_content_ts
        }
    }
}
#[must_use]
pub fn postgresql_crud_common_query_part_error_named_checked_add_initialization_ts()
-> proc_macro2::TokenStream {
    quote! {postgresql_crud_common::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }}
}
pub fn generate_impl_crate_is_string_empty_for_ident_content_ts(
    ident: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote! {
        impl postgresql_crud_common::IsStringEmpty for #ident {
            fn is_string_empty(&self) -> bool {
                #content_ts
            }
        }
    }
}
pub fn generate_match_try_new_in_deserialize_ts(
    ident: &dyn quote::ToTokens,
    initialization_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    quote! {
        match #ident::try_new(#initialization_ts) {
            Ok(value) => Ok(value),
            Err(error) => Err(serde::de::Error::custom(format!("{error:?}")))
        }
    }
}
pub fn generate_impl_serde_deserialize_for_struct_ts(
    ident: &dyn StdFmtDisplayPlusQuoteToTokens,
    vec_ident_type: &[(&syn::Ident, &syn::Type)],
    len: usize,
    generate_type_ts: &dyn Fn(&syn::Ident, &syn::Type) -> proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    fn generate_underscore_underscore_field_index_stringified(index: usize) -> String {
        format!("__field{index}")
    }
    fn generate_underscore_underscore_field_index_ts(index: usize) -> proc_macro2::TokenStream {
        generate_underscore_underscore_field_index_stringified(index)
            .parse::<proc_macro2::TokenStream>()
            .expect("ff7433a3-459b-45f4-a41f-01bf7ce46757")
    }
    fn generate_underscore_underscore_field_index_handle_ts(
        index: usize,
    ) -> proc_macro2::TokenStream {
        format!(
            "{}_handle",
            generate_underscore_underscore_field_index_stringified(index)
        )
        .parse::<proc_macro2::TokenStream>()
        .expect("09a0c518-28da-455b-bce8-fb6defae8a3b")
    }
    fn generate_field_ident_double_quotes_serde_private_ok_field_ts(
        field_name_double_quotes_ts: &dyn quote::ToTokens,
        index: usize,
    ) -> proc_macro2::TokenStream {
        let field_index_ts = generate_underscore_underscore_field_index_ts(index);
        quote! {#field_name_double_quotes_ts => Ok(__Field::#field_index_ts)}
    }
    let allow_clippy_arbitrary_source_item_ordering_ts =
        token_patterns::AllowClippyArbitrarySourceItemOrdering;
    let vec_ident = vec_ident_type
        .iter()
        .map(|el_00a99fdb| el_00a99fdb.0)
        .collect::<Vec<&syn::Ident>>();
    let field_enum_variants_ts = {
        let field_enum_variants_ts = (0..len)
            .map(|i| {
                format!("__{FieldSc}{i}")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("c46314b0-baee-41c8-b9c6-54b888310ca8")
            })
            .collect::<Vec<proc_macro2::TokenStream>>();
        quote! {#(#field_enum_variants_ts),*}
    };
    let visit_u64_value_enum_variants_ts = {
        let visit_u64_value_enum_variants_ts = (0..len).map(|index| {
            let index_u64_ts = {
                let value = format!("{index}u64");
                value
                    .parse::<proc_macro2::TokenStream>()
                    .expect("828ff7b4-5b7c-4109-8739-c6aa240f0f66")
            };
            let field_index_ts = generate_underscore_underscore_field_index_ts(index);
            quote! {#index_u64_ts => Ok(__Field::#field_index_ts)}
        });
        quote! {#(#visit_u64_value_enum_variants_ts),*}
    };
    let visit_str_value_enum_variants_ts = {
        let visit_str_value_enum_variants_ts =
            vec_ident.iter().enumerate().map(|(index, element)| {
                let field_name_double_quotes_ts = generate_quotes::double_quotes_ts(&element);
                generate_field_ident_double_quotes_serde_private_ok_field_ts(
                    &field_name_double_quotes_ts,
                    index,
                )
            });
        quote! {#(#visit_str_value_enum_variants_ts),*,}
    };
    let visit_bytes_value_enum_variants_ts = {
        let visit_bytes_value_enum_variants_ts =
            vec_ident.iter().enumerate().map(|(index, element)| {
                let b_field_name_double_quotes_ts = {
                    let el_ident_double_quotes_stringified =
                        generate_quotes::double_quotes_stringified(&element.to_string());
                    let value = format!("b{el_ident_double_quotes_stringified}");
                    value
                        .parse::<proc_macro2::TokenStream>()
                        .expect("9e33625e-5f3d-4110-9641-204910c7f08e")
                };
                generate_field_ident_double_quotes_serde_private_ok_field_ts(
                    &b_field_name_double_quotes_ts,
                    index,
                )
            });
        quote! {#(#visit_bytes_value_enum_variants_ts),*,}
    };
    let struct_ident_double_quotes_ts = generate_struct_ident_double_quotes_ts(&ident);
    let visit_seq_fields_initialization_ts = {
        let content_ts = vec_ident_type.iter().enumerate().map(|(index, (el_ident, el_type))| {
            let field_index_handle_ts = generate_underscore_underscore_field_index_handle_ts(index);
            let type_ts = generate_type_ts(el_ident, el_type);
            let struct_ident_options_with_double_quotes_ts = generate_quotes::double_quotes_ts(&format!("struct {ident} with {len} elements"));
            quote! {
                let Some(#field_index_handle_ts) = serde::de::SeqAccess::next_element::<#type_ts>(&mut __seq)? else {
                    return Err(serde::de::Error::invalid_length(0usize, &#struct_ident_options_with_double_quotes_ts));
                };
            }
        });
        quote! {#(#content_ts)*}
    };
    let match_try_new_in_deserialize_ts = generate_match_try_new_in_deserialize_ts(&ident, &{
        let fields_ts = (0..len).map(generate_underscore_underscore_field_index_handle_ts);
        quote! {#(#fields_ts),*}
    });
    let visit_map_fields_initialization_ts = {
        let content_ts = vec_ident_type
            .iter()
            .enumerate()
            .map(|(index, (el_ident, el_type))| {
                let type_ts = generate_type_ts(el_ident, el_type);
                let field_index_ts = generate_underscore_underscore_field_index_ts(index);
                quote! {
                    let mut #field_index_ts: Option<#type_ts> = None;
                }
            });
        quote! {#(#content_ts)*}
    };
    let visit_map_match_variants_ts = {
        let visit_map_match_variants_ts = vec_ident_type.iter().enumerate().map(|(index, (el_ident, el_type))| {
            let field_index_ts = generate_underscore_underscore_field_index_ts(index);
            let field_ident_double_quotes_ts = generate_quotes::double_quotes_ts(&el_ident);
            let type_ts = generate_type_ts(el_ident, el_type);
            quote! {
                __Field::#field_index_ts => {
                    if Option::is_some(&#field_index_ts) {
                        return Err(
                            <__A::Error as serde::de::Error>::duplicate_field(#field_ident_double_quotes_ts),
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
            let field_index_ts = generate_underscore_underscore_field_index_ts(index);
            let field_index_handle_ts = generate_underscore_underscore_field_index_handle_ts(index);
            let field_ident_double_quotes_ts = generate_quotes::double_quotes_ts(&el_a1d37c97);
            quote! {
                let #field_index_handle_ts = match #field_index_ts {
                    Some(value_4f8faf03) => value_4f8faf03,
                    None => {
                        serde::__private228::de::missing_field(#field_ident_double_quotes_ts)?
                    }
                };
            }
        });
        quote! {#(#content_ts)*}
    };
    let fields_array_elements_ts = {
        let fields_array_elements_ts = vec_ident
            .iter()
            .map(|el_43a33e0b| generate_quotes::double_quotes_ts(&el_43a33e0b));
        quote! {#(#fields_array_elements_ts),*}
    };
    let ident_double_quotes_ts = generate_quotes::double_quotes_ts(&ident);
    quote! {
        #[allow(unused_qualifications)]
        #[allow(clippy::absolute_paths)]
        #allow_clippy_arbitrary_source_item_ordering_ts
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
                                #struct_ident_double_quotes_ts,
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
                        #ident_double_quotes_ts,
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
pub fn wrap_content_into_scopes_ts(content_ts: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote! {(#content_ts)}
}
pub fn maybe_wrap_into_braces_ts(
    content_ts: &dyn quote::ToTokens,
    std_primitive_bool: bool,
) -> proc_macro2::TokenStream {
    if std_primitive_bool {
        wrap_content_into_scopes_ts(&content_ts)
    } else {
        quote! {#content_ts}
    }
}
pub fn generate_value_initialization_ts(
    import_path: &ImportPath,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let value_sc = ValueSc;
    quote! {#import_path::Value { #value_sc: #content_ts }}
}
pub fn impl_postgresql_type_equal_operator_for_ident_ts(
    import_path: &ImportPath,
    ident: &dyn quote::ToTokens,
    content_ts: &dyn quote::ToTokens,
) -> proc_macro2::TokenStream {
    let postgresql_type_equal_operator_ucc = PostgresqlTypeEqualOperatorUcc;
    let equal_operator_ucc = EqualOperatorUcc;
    quote! {
        impl #import_path::#postgresql_type_equal_operator_ucc for #ident {
            fn operator(&self) -> #import_path::#equal_operator_ucc {
                #content_ts
            }
        }
    }
}
#[must_use]
pub fn generate_query_part_error_named_write_into_buffer_ts(
    import_path: ImportPath,
) -> proc_macro2::TokenStream {
    quote! {
        #import_path::QueryPartErrorNamed::WriteIntoBuffer {
            code_occurence: error_occurence_lib::code_occurence!()
        }
    }
}
#[must_use]
pub fn generate_return_err_query_part_error_named_write_into_buffer_ts(
    import_path: ImportPath,
) -> proc_macro2::TokenStream {
    let content_ts = generate_query_part_error_named_write_into_buffer_ts(import_path);
    quote! {return Err(#content_ts);}
}
