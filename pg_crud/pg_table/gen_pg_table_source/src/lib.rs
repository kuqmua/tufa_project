use gen_quotes::dq_ts;
use macros_helpers::{
    AttrIdentStr, DeriveClone, DeriveCopy, FormatWithCargofmt, LocationFieldAttr,
    ShouldWriteTokenStreamIntoFile, StatusCode, StructOrEnumDeriveTokenStreamBuilder,
    SynFieldWrapper, gen_field_loc_new_ts, gen_if_write_is_err_curly_braces_ts,
    gen_if_write_is_err_ts, gen_impl_display_ts, gen_impl_pub_try_new_for_ident_ts,
    gen_impl_to_err_string_ts, gen_serde_version_of_named_syn_variant, gen_simple_syn_punct,
    get_macro_attr_meta_list_ts, loc_syn_field, maybe_write_ts_into_file,
};
use naming::{
    AdditionalParametersSc, AppStateSc, AsRefStrEnumWithUnitFieldsToScStr,
    AsRefStrEnumWithUnitFieldsToUccStr, AsRefStrToScStr, AsRefStrToScTs, BeginSc, BindedQuerySc,
    BodyBytesSc, BodySc, BodySizeErUcc, BySc, CheckBodySizeSc, CheckBodySizeUcc, ColumnSc,
    ColumnsSc, CommitSc, CommonAdditionalErVariantsSc, CommonAdditionalLogicSc,
    CommonReadOnlyIdsReturnedFromCreateOneSc, ConfigSc, CreateExtensionIfNotExistsPgJsonschemaUcc,
    CreateExtensionIfNotExistsUuidOsspUcc, CreateIntoPgJsonTypeOptionVecWhereLengthEqualSc,
    CreateIntoPgJsonTypeOptionVecWhereLengthGreaterThanSc,
    CreateIntoPgTypeOptionVecWhereDimOneEqualSc, CreateManyAdditionalErVariantsSc,
    CreateManyAdditionalLogicSc, CreateOneAdditionalErVariantsSc, CreateOneAdditionalLogicSc,
    CreateQueryBindSc, CreateQueryPartSc, CreateSc, CreateTableColumnQueryPartSc, CreateUcc,
    DefaultOptionSomeVecOneElMaxPageSizeSc, DefaultOptionSomeVecOneElMaxPageSizeUcc,
    DefaultOptionSomeVecOneElSc, DefaultOptionSomeVecOneElUcc, DeleteManyAdditionalErVariantsSc,
    DeleteManyAdditionalLogicSc, DeleteOneAdditionalErVariantsSc, DeleteOneAdditionalLogicSc,
    DeserializeResponseUcc, DesirableUcc, DisplayPlusToTokens, DisplayToScStr, ElementSc,
    EndpointLocationSc, ErSc, ExecutorAcquireSc, ExecutorSc, ExpectedResponseSc,
    FailedToGetResponseTextUcc, FalseSc, FromHandleSc, FutureSc,
    GenColumnQuealsValueCommaUpdateOneQueryPartSc, GenPgTablePrimaryKeySc, GenSelectQueryPartSc,
    GenWhenColumnIdThenValueUpdateManyQueryPartSc, HeaderContentTypeApplicationJsonNotFoundUcc,
    HeadersSc, IdentCreateDefaultSc, IncrementSc, IntoSerdeVersionSc, LocSc, NoFieldsProvidedUcc,
    NotUniqueFieldSc, NotUniqueFieldUcc, NotUniquePrimaryKeySc, NotUniquePrimaryKeyUcc,
    OptionVecCreateSc, OrderBySc, OrderByUcc, OrderSc, PaginationSc, ParametersSc, PayloadSc,
    PayloadUcc, PgCrudSc, PgPoolForTokioSpawnSyncMoveSc, PgPoolSc, PgSc,
    PgTypeOptionVecWhereGreaterThanTestSc, PgTypeUcc, PgUcc, PoolConnectionSc, PoolSc, PrefixSc,
    PrepareExtensionsSc, PreparePgSc, PreparePgTableSc, PreparePgUcc, PrimaryKeyQueryPartSc,
    PrimaryKeySc, QueryBindSc, QueryPartErUcc, QueryPartSc, QueryPartUcc, QuerySc, QueryStringSc,
    ReadIntoTableTypeDeclarationSc, ReadManyAdditionalErVariantsSc, ReadManyAdditionalLogicSc,
    ReadOneAdditionalErVariantsSc, ReadOneAdditionalLogicSc, ReadOnlyIdsIntoReadSc,
    ReadOnlyIdsIntoTableTypeDeclarationSc, ReadOnlyIdsIntoUpdateSc,
    ReadOnlyIdsMergedWithCreateIntoOptionVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereBetweenSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereInSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereRegularExpressionSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc,
    ReadOnlyIdsMergedWithTableTypeDeclarationIntoPgTypeOptionWhereGreaterThanSc, ReadOnlyIdsSc,
    ReadOnlyIdsUcc, ReadUcc, RequestSc, ReqwestSc, ReqwestUcc, ResponseSc, ResponseTextSc,
    RollbackSc, RoutesHandleSc, RoutesSc, RowAndRollbackUcc, RowSc, RowsSc,
    SelectOnlyIdsQueryPartSc, SelectOnlyUpdatedIdsQueryPartSc, SelectPrimaryKeySc,
    SelectQueryPartSc, SelectSc, SelectUcc, SerdeJsonSc, SerdeJsonToStringSc, SerdeJsonToStringUcc,
    SerdeJsonUcc, SerdeSc, StatusCodeSc, TableNameSc, TableSc, ToTokensToScStr, ToTokensToUccTs,
    TrueSc, TryBindSc, TryBindUcc, UpdateForQuerySc, UpdateForQueryUcc, UpdateForQueryVecSc,
    UpdateManyAdditionalErVariantsSc, UpdateManyAdditionalLogicSc, UpdateOneAdditionalErVariantsSc,
    UpdateOneAdditionalLogicSc, UpdateQueryBindSc, UpdateQueryPartPrimaryKeySc, UpdateQueryPartSc,
    UpdateSc, UpdateUcc, UrlSc, ValueSc, ValueUcc, WhereManySc, WhereUcc,
    parameter::{
        ErSelfSc, IsSelfUpdateExistSc, SelfCreateUcc, SelfDeleteManyParametersUcc,
        SelfDeleteManyPayloadUcc, SelfDeleteOneErWithSerdeUcc, SelfDeleteOneParametersUcc,
        SelfDeleteOnePayloadUcc, SelfErWithSerdeSc, SelfGenPgTableModSc, SelfHandleSc,
        SelfPayloadExampleSc, SelfPreparePgErUcc, SelfReadOneErWithSerdeUcc,
        SelfReadOnlyIdsToTwoDimalVecReadInnerAccSc, SelfReadOnlyIdsUcc, SelfReadUcc, SelfSelectUcc,
        SelfTableTypeDeclarationUcc, SelfTestsSc, SelfTryDeleteOneErUcc, SelfTryReadOneErUcc,
        SelfUpdateForQueryUcc, SelfUpdateManyParametersUcc, SelfUpdateManyPayloadUcc,
        SelfUpdateTryNewErUcc, SelfUpdateUcc, SelfWhereManyTryNewErUcc, SelfWhereManyUcc,
        SelfWhereUcc, StdOptionOptionSelfWhereManyUcc,
        TryFromSqlxPgPgRowWithNotEmptyUniqueVecSelfSelectSc, TrySelfHandleSc, TrySelfSc,
        UpdateQueryPartSelfSc,
    },
};
use panic_location::panic_location;
use pg_crud_macros_common::{
    ColumnParameterUnderscore, Dim, EqualOrEqualUsingFields, ImportPath,
    IncrementParameterUnderscore, IsNeedToAddLogicalOperatorUnderscore, IsQueryBindMutable,
    gen_impl_pg_crud_all_variants_default_option_some_vec_one_el_ts,
    gen_impl_pg_crud_default_option_some_vec_one_el_ts, gen_impl_serde_deserialize_for_struct_ts,
    gen_match_try_new_in_deserialize_ts, gen_option_tokens_declaration_ts,
    gen_query_part_er_write_into_buffer_ts, gen_return_err_query_part_er_write_into_buffer_ts,
    gen_value_init_ts, gen_vec_tokens_declaration_ts, impl_pg_type_where_filter_for_ident_ts,
    maybe_wrap_into_braces_ts,
};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde::Deserialize;
use serde_json::from_str;
use std::{
    fmt::{Display, Formatter, Result as StdFmtResult, Write as _},
    iter::once,
    str::FromStr,
};
use strum_macros::Display;
use syn::{
    AttrStyle, Attribute, Data, DeriveInput, Field, FieldMutability, Fields, FieldsNamed, Ident,
    Meta, Path, PathArguments, PathSegment, Type, TypePath, Variant, Visibility, parse2,
    punctuated::Punctuated, token::Brace, token::Bracket, token::Colon, token::Comma,
    token::PathSep, token::Pound,
};
#[allow(unused_imports)]
use token_patterns::{
    AllowClippyArbitrarySourceItemOrdering, Bool, Char, CoreDefault,
    DeriveDebugSerdeSerializeSerdeDeserialize, DeriveDebugThiserrorErrorOccurence, Er0, Er1, Er2,
    Er3, F32, F64, FieldAttrSerdeSkipSerializingIfOptionIsNone, I8, I16, I32, I64, MustUse,
    PgCrudCommonDefaultOptionSomeVecOneEl, PgCrudCommonDefaultOptionSomeVecOneElCall,
    PgCrudCommonDefaultOptionSomeVecOneElMaxPageSizeCall, PgCrudDefaultOptionSomeVecOneElCall,
    RefStr, SqlxAcquire, SqlxRow, StringTs, U8, U16, U32, U64,
};
//todo decide where to do er log (maybe add in some places)
//todo gen route what will return columns of the table and their rust and postgersql types
//todo created at and updated at fields + created by + updated by
//todo attrs for activation generation crud methods(like gen create, update_one, delete_one)
//todo authorization for returning concrete er or just minimal info(user role)
//todo gen rules and roles
//todo maybe add unnest sql types?
//todo maybe add unnest to filter parameters if its array ?
//todo swagger ui https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
//todo derive utoipa::ToSchema for what? original structs or with serialize deserialize?
//todo need to add utoipa::ToSchema annotation #[schema(value_type = YourToSchemaTraitImplStruct)] for all fields
//todo remove useless derives like useless serde::Serialize and Deserialize
//todo maybe gen compisite type for user defined type https://docs.rs/sqlx/0.7.3/sqlx/pg/types/index.html#rust_decimal
//todo read again some interesting thoughts about sql as api https://habr.com/ru/companies/timeweb/articles/798937/
//todo reexport all crates what logic depends on (from crates.io) (use of undeclared crate or module `time`)
//todo add transaction isolation level (see pg docs)
//todo check on pg max length value of type
//todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -maybe its er? or maybe use select then update\delete?(rewrite query)
//todo pg json schema validation https://youtu.be/F6X60ln2VNc
//todo gen json schema from rust type https://docs.rs/schemars/laTest/schemars/
//todo support read table length
//todo what is pub what is private
//todo header Retry-After logic
//todo pg json:
//* write json schema in pg
//* validate insert json field with json schema
#[must_use]
pub fn gen_pg_table(input: Ts2) -> Ts2 {
    #[derive(Debug)]
    struct SynVariantWrapper {
        status_code: Option<StatusCode>,
        variant: Variant,
    }
    impl SynVariantWrapper {
        const fn get_option_status_code(&self) -> Option<&StatusCode> {
            self.status_code.as_ref()
        }
        const fn get_syn_variant(&self) -> &Variant {
            &self.variant
        }
    }
    enum ShouldAddBorrow {
        False,
        True,
    }
    impl ToTokens for ShouldAddBorrow {
        fn to_tokens(&self, tokens: &mut Ts2) {
            match &self {
                Self::False => Ts2::new().to_tokens(tokens),
                Self::True => quote! {&}.to_tokens(tokens),
            }
        }
    }
    enum ShouldAddReturn {
        False,
        True,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, Copy, AsRefStrEnumWithUnitFieldsToUccStr, AsRefStrEnumWithUnitFieldsToScStr,
    )]
    enum Operation {
        CreateMany,
        CreateOne,
        ReadMany,
        ReadOne,
        UpdateMany,
        UpdateOne,
        DeleteMany,
        DeleteOne,
    }
    impl Operation {
        const fn derive_clone_and_copy(self) -> (DeriveClone, DeriveCopy) {
            match self {
                Self::CreateMany
                | Self::CreateOne
                | Self::ReadMany
                | Self::ReadOne
                | Self::UpdateMany
                | Self::UpdateOne
                | Self::DeleteMany => (DeriveClone::False, DeriveCopy::False),
                Self::DeleteOne => (DeriveClone::True, DeriveCopy::True),
            }
        }
        const fn desirable_status_code(self) -> StatusCode {
            match self {
                Self::CreateMany | Self::CreateOne => StatusCode::Created201,
                Self::ReadMany
                | Self::ReadOne
                | Self::UpdateMany
                | Self::UpdateOne
                | Self::DeleteMany
                | Self::DeleteOne => StatusCode::Ok200,
            }
        }
        const fn gen_pg_table_attr_additional_er_variants(self) -> GenPgTableAttr {
            match self {
                Self::CreateMany => GenPgTableAttr::CreateManyAdditionalErVariants,
                Self::CreateOne => GenPgTableAttr::CreateOneAdditionalErVariants,
                Self::ReadMany => GenPgTableAttr::ReadManyAdditionalErVariants,
                Self::ReadOne => GenPgTableAttr::ReadOneAdditionalErVariants,
                Self::UpdateMany => GenPgTableAttr::UpdateManyAdditionalErVariants,
                Self::UpdateOne => GenPgTableAttr::UpdateOneAdditionalErVariants,
                Self::DeleteMany => GenPgTableAttr::DeleteManyAdditionalErVariants,
                Self::DeleteOne => GenPgTableAttr::DeleteOneAdditionalErVariants,
            }
        }
        const fn gen_pg_table_attr_additional_logic(self) -> GenPgTableAttr {
            match self {
                Self::CreateMany => GenPgTableAttr::CreateManyAdditionalLogic,
                Self::CreateOne => GenPgTableAttr::CreateOneAdditionalLogic,
                Self::ReadMany => GenPgTableAttr::ReadManyAdditionalLogic,
                Self::ReadOne => GenPgTableAttr::ReadOneAdditionalLogic,
                Self::UpdateMany => GenPgTableAttr::UpdateManyAdditionalLogic,
                Self::UpdateOne => GenPgTableAttr::UpdateOneAdditionalLogic,
                Self::DeleteMany => GenPgTableAttr::DeleteManyAdditionalLogic,
                Self::DeleteOne => GenPgTableAttr::DeleteOneAdditionalLogic,
            }
        }
        const fn http_method(self) -> OperationHttpMethod {
            match self {
                Self::CreateMany | Self::CreateOne | Self::ReadMany | Self::ReadOne => {
                    OperationHttpMethod::Post
                }
                Self::UpdateMany | Self::UpdateOne => OperationHttpMethod::Patch,
                Self::DeleteMany | Self::DeleteOne => OperationHttpMethod::Delete,
            }
        }
        fn operation_er_with_serde_sc(self) -> SelfErWithSerdeSc {
            SelfErWithSerdeSc::from_display(&self)
        }
        fn operation_payload_example_sc(self) -> impl DisplayPlusToTokens {
            SelfPayloadExampleSc::from_display(&self)
        }
        fn self_handle_sc_ts(self) -> Ts2 {
            let value = SelfHandleSc::from_tokens(&self.self_sc_ts());
            quote! {#value}
        }
        fn self_sc_str(self) -> String {
            AsRefStrToScStr::case(&self.to_string())
        }
        fn self_sc_ts(self) -> Ts2 {
            AsRefStrToScTs::case_or_panic(&self.to_string())
        }
        fn try_self_handle_sc_ts(self) -> Ts2 {
            let value = TrySelfHandleSc::from_tokens(&self.self_sc_ts());
            quote! {#value}
        }
        fn try_self_sc_ts(self) -> Ts2 {
            let value = TrySelfSc::from_tokens(&self.self_sc_ts());
            quote! {#value}
        }
    }
    impl Display for Operation {
        fn fmt(&self, f: &mut Formatter<'_>) -> StdFmtResult {
            match &self {
                Self::CreateMany => write!(f, "CreateMany"),
                Self::CreateOne => write!(f, "CreateOne"),
                Self::ReadMany => write!(f, "ReadMany"),
                Self::ReadOne => write!(f, "ReadOne"),
                Self::UpdateMany => write!(f, "UpdateMany"),
                Self::UpdateOne => write!(f, "UpdateOne"),
                Self::DeleteMany => write!(f, "DeleteMany"),
                Self::DeleteOne => write!(f, "DeleteOne"),
            }
        }
    }
    impl From<&CreateOrUpdateOrDeleteMany> for Operation {
        fn from(v: &CreateOrUpdateOrDeleteMany) -> Self {
            match &v {
                CreateOrUpdateOrDeleteMany::Create => Self::CreateMany,
                CreateOrUpdateOrDeleteMany::Update => Self::UpdateMany,
                CreateOrUpdateOrDeleteMany::Delete => Self::DeleteMany,
            }
        }
    }
    impl From<&ReadManyOrDeleteMany> for Operation {
        fn from(v: &ReadManyOrDeleteMany) -> Self {
            match &v {
                ReadManyOrDeleteMany::ReadMany => Self::ReadMany,
                ReadManyOrDeleteMany::DeleteMany => Self::DeleteMany,
            }
        }
    }
    impl From<&ReadManyOrReadOne> for Operation {
        fn from(v: &ReadManyOrReadOne) -> Self {
            match &v {
                ReadManyOrReadOne::ReadMany => Self::ReadMany,
                ReadManyOrReadOne::ReadOne => Self::ReadOne,
            }
        }
    }
    impl From<&CreateOrUpdateOrDeleteOne> for Operation {
        fn from(v: &CreateOrUpdateOrDeleteOne) -> Self {
            match &v {
                CreateOrUpdateOrDeleteOne::Create => Self::CreateOne,
                CreateOrUpdateOrDeleteOne::Update => Self::UpdateOne,
                CreateOrUpdateOrDeleteOne::Delete => Self::DeleteOne,
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(AsRefStrEnumWithUnitFieldsToScStr)]
    enum OperationHttpMethod {
        Post,
        Patch,
        Delete,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum ReadManyOrDeleteMany {
        ReadMany,
        DeleteMany,
    }
    enum ReadManyOrReadOne {
        ReadMany,
        ReadOne,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Display)]
    enum GenPgTableAttr {
        CreateManyAdditionalErVariants,
        CreateOneAdditionalErVariants,
        ReadManyAdditionalErVariants,
        ReadOneAdditionalErVariants,
        UpdateManyAdditionalErVariants,
        UpdateOneAdditionalErVariants,
        DeleteManyAdditionalErVariants,
        DeleteOneAdditionalErVariants,
        CommonAdditionalErVariants,
        CreateManyAdditionalLogic,
        CreateOneAdditionalLogic,
        ReadManyAdditionalLogic,
        ReadOneAdditionalLogic,
        UpdateManyAdditionalLogic,
        UpdateOneAdditionalLogic,
        DeleteManyAdditionalLogic,
        DeleteOneAdditionalLogic,
        CommonAdditionalLogic,
    }
    impl GenPgTableAttr {
        fn gen_path_to_attr(self) -> String {
            let value = match self {
                Self::CreateManyAdditionalErVariants => {
                    CreateManyAdditionalErVariantsSc.to_string()
                }
                Self::CreateOneAdditionalErVariants => CreateOneAdditionalErVariantsSc.to_string(),
                Self::ReadManyAdditionalErVariants => ReadManyAdditionalErVariantsSc.to_string(),
                Self::ReadOneAdditionalErVariants => ReadOneAdditionalErVariantsSc.to_string(),
                Self::UpdateManyAdditionalErVariants => {
                    UpdateManyAdditionalErVariantsSc.to_string()
                }
                Self::UpdateOneAdditionalErVariants => UpdateOneAdditionalErVariantsSc.to_string(),
                Self::DeleteManyAdditionalErVariants => {
                    DeleteManyAdditionalErVariantsSc.to_string()
                }
                Self::DeleteOneAdditionalErVariants => DeleteOneAdditionalErVariantsSc.to_string(),
                Self::CommonAdditionalErVariants => CommonAdditionalErVariantsSc.to_string(),
                Self::CreateManyAdditionalLogic => CreateManyAdditionalLogicSc.to_string(),
                Self::CreateOneAdditionalLogic => CreateOneAdditionalLogicSc.to_string(),
                Self::ReadManyAdditionalLogic => ReadManyAdditionalLogicSc.to_string(),
                Self::ReadOneAdditionalLogic => ReadOneAdditionalLogicSc.to_string(),
                Self::UpdateManyAdditionalLogic => UpdateManyAdditionalLogicSc.to_string(),
                Self::UpdateOneAdditionalLogic => UpdateOneAdditionalLogicSc.to_string(),
                Self::DeleteManyAdditionalLogic => DeleteManyAdditionalLogicSc.to_string(),
                Self::DeleteOneAdditionalLogic => DeleteOneAdditionalLogicSc.to_string(),
                Self::CommonAdditionalLogic => CommonAdditionalLogicSc.to_string(),
            };
            format!("{PgCrudSc}::{value}")
        }
    }
    enum ShouldWrapIntoValue {
        False,
        True,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum CreateOrUpdateOrDeleteMany {
        Create,
        Update,
        Delete,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    enum CreateOrUpdateOrDeleteOne {
        Create,
        Update,
        Delete,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Deserialize)]
    struct GenPgTableConfig {
        create_many_content_write_into_gen_pg_table_create_many: ShouldWriteTokenStreamIntoFile,
        create_one_content_write_into_gen_pg_table_create_one: ShouldWriteTokenStreamIntoFile,
        read_many_content_write_into_gen_pg_table_read_many: ShouldWriteTokenStreamIntoFile,
        read_one_content_write_into_gen_pg_table_read_one: ShouldWriteTokenStreamIntoFile,
        update_many_content_write_into_gen_pg_table_update_many: ShouldWriteTokenStreamIntoFile,
        update_one_content_write_into_gen_pg_table_update_one: ShouldWriteTokenStreamIntoFile,
        delete_many_content_write_into_gen_pg_table_delete_many: ShouldWriteTokenStreamIntoFile,
        delete_one_content_write_into_gen_pg_table_delete_one: ShouldWriteTokenStreamIntoFile,
        tests_content_write_into_gen_pg_table_tests: ShouldWriteTokenStreamIntoFile,
        common_content_write_into_gen_pg_table_common: ShouldWriteTokenStreamIntoFile,
        whole_content_write_into_gen_pg_table: ShouldWriteTokenStreamIntoFile,
    }
    panic_location();
    let import_path = ImportPath::PgCrud;
    let return_err_query_part_er_write_into_buffer_ts =
        gen_return_err_query_part_er_write_into_buffer_ts(import_path);
    let di: DeriveInput = parse2(input).expect("991c614f");
    let gen_pg_table_config = from_str::<GenPgTableConfig>(
        &get_macro_attr_meta_list_ts(
            &di.attrs,
            &format!("{}::gen_pg_table_config", import_path.sc_str()),
        )
        .to_string(),
    )
    .expect("1b6adf7e");
    let ident = &di.ident;
    let ident_sc_str = ToTokensToScStr::case(&ident);
    let ident_sc_dq_ts = dq_ts(&ident_sc_str);
    let self_table_name_call_ts = quote! {Self::#TableNameSc()};
    let (primary_key_field, fields, fields_without_primary_key) =
        if let Data::Struct(data_struct) = &di.data {
            if let Fields::Named(fields_named) = &data_struct.fields {
                let mut option_primary_key_field: Option<SynFieldWrapper> = None;
                let mut fields = Vec::new();
                let mut fields_without_primary_key = Vec::new();
                for el_2e7b44a3 in &fields_named.named {
                    let field_ident = el_2e7b44a3.ident.clone().expect("915ef2ce");
                    let field_ident_len = field_ident.to_string().len();
                    let max_pg_column_length = 63;
                    //todo write runtime check
                    assert!(field_ident_len <= max_pg_column_length, "1266ae5a");
                    fields.push(SynFieldWrapper {
                        field_vis: el_2e7b44a3.vis.clone(),
                        field_ident: field_ident.clone(),
                        field_type: el_2e7b44a3.ty.clone(),
                    });
                    let mut is_primary_key = false;
                    {
                        for el_f4d3785c in &el_2e7b44a3.attrs {
                            if el_f4d3785c.path().segments.len() == 1 {
                                let first_segment_ident =
                                    &el_f4d3785c.path().segments.first().expect("a9c3b38b").ident;
                                let gen_pg_table_primary_key_sc_str =
                                    GenPgTablePrimaryKeySc.to_string();
                                if first_segment_ident == &gen_pg_table_primary_key_sc_str {
                                    if option_primary_key_field.is_some() {
                                        panic!("1a75cea1");
                                    } else {
                                        option_primary_key_field = Some(SynFieldWrapper {
                                            field_vis: el_2e7b44a3.vis.clone(),
                                            field_ident: field_ident.clone(),
                                            field_type: el_2e7b44a3.ty.clone(),
                                        });
                                        is_primary_key = true;
                                    }
                                }
                            }
                        }
                    }
                    if !is_primary_key {
                        fields_without_primary_key.push(SynFieldWrapper {
                            field_vis: el_2e7b44a3.vis.clone(),
                            field_ident: field_ident.clone(),
                            field_type: el_2e7b44a3.ty.clone(),
                        });
                    }
                }
                // explicitly not supporting number of columns more than 100 so its less possibility to cause stack overflow or build process exit
                // assert!((fields.len() <= 100), "d9963f32");
                (
                    option_primary_key_field.expect("6a529a99"),
                    fields,
                    fields_without_primary_key,
                )
            } else {
                panic!("7f31872d");
            }
        } else {
            panic!("bd4718d0");
        };
    let fields_len = fields.len();
    let fields_len_without_primary_key = fields_without_primary_key.len();
    let primary_key_field_type = &primary_key_field.field_type;
    let primary_key_field_type_where_ts =
        SelfWhereUcc::from_type_last_segment(&primary_key_field.field_type);
    //todo must remove this and use trait type instead
    let primary_key_field_type_table_type_declaration_ts =
        SelfTableTypeDeclarationUcc::from_type_last_segment(&primary_key_field.field_type);
    let gen_as_pg_type_ts = |field_type: &dyn ToTokens| {
        quote! {<#field_type as pg_crud::PgType>::}
    };
    let primary_key_field_type_as_pg_type_ts = gen_as_pg_type_ts(&primary_key_field_type);
    let gen_as_pg_type_tokens_ts = |field_type: &dyn ToTokens, tokens: &dyn ToTokens| {
        let as_pg_type_ts = gen_as_pg_type_ts(&field_type);
        quote! {#as_pg_type_ts #tokens}
    };
    // let gen_as_pg_type_table_type_declaration_ts = |field_type: &dyn ToTokens| gen_as_pg_type_tokens_ts(&field_type, &TableTypeDeclarationUcc);
    // let primary_key_field_type_as_pg_type_table_type_declaration_ts = gen_as_pg_type_table_type_declaration_ts(&primary_key_field_type);
    let gen_as_pg_type_create_ts =
        |field_type: &dyn ToTokens| gen_as_pg_type_tokens_ts(&field_type, &CreateUcc);
    let gen_as_pg_type_select_ts =
        |field_type: &dyn ToTokens| gen_as_pg_type_tokens_ts(&field_type, &SelectUcc);
    let primary_key_field_type_as_pg_type_select_ts =
        gen_as_pg_type_select_ts(&primary_key_field_type);
    let gen_as_pg_type_where_ts =
        |field_type: &dyn ToTokens| gen_as_pg_type_tokens_ts(&field_type, &WhereUcc);
    let primary_key_field_type_as_pg_type_where_ts =
        gen_as_pg_type_where_ts(&primary_key_field_type);
    let gen_as_pg_type_read_ts =
        |field_type: &dyn ToTokens| gen_as_pg_type_tokens_ts(&field_type, &ReadUcc);
    let gen_as_pg_type_read_only_ids_ts =
        |field_type: &dyn ToTokens| gen_as_pg_type_tokens_ts(&field_type, &ReadOnlyIdsUcc);
    let primary_key_field_type_as_pg_type_read_ts = gen_as_pg_type_read_ts(&primary_key_field_type);
    let gen_as_pg_type_update_ts =
        |field_type: &dyn ToTokens| gen_as_pg_type_tokens_ts(&field_type, &UpdateUcc);
    let gen_as_pg_type_update_for_query_ts =
        |field_type: &dyn ToTokens| gen_as_pg_type_tokens_ts(&field_type, &UpdateForQueryUcc);
    let primary_key_field_type_as_pg_type_read_ucc =
        quote! {<#primary_key_field_type as pg_crud::#PgTypeUcc>::#ReadUcc};
    let ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident);
    let ident_delete_many_parameters_ucc = SelfDeleteManyParametersUcc::from_tokens(&ident);
    let ident_delete_many_payload_ucc = SelfDeleteManyPayloadUcc::from_tokens(&ident);
    let ident_delete_one_parameters_ucc = SelfDeleteOneParametersUcc::from_tokens(&ident);
    let ident_delete_one_payload_ucc = SelfDeleteOnePayloadUcc::from_tokens(&ident);
    let ident_try_read_one_er_ucc = SelfTryReadOneErUcc::from_tokens(&ident);
    let ident_read_one_er_with_serde_ucc = SelfReadOneErWithSerdeUcc::from_tokens(&ident);
    let ident_try_delete_one_er_ucc = SelfTryDeleteOneErUcc::from_tokens(&ident);
    let ident_delete_one_er_with_serde_ucc = SelfDeleteOneErWithSerdeUcc::from_tokens(&ident);
    let vec_primary_key_field_type_read_ts =
        gen_vec_tokens_declaration_ts(&primary_key_field_type_as_pg_type_read_ucc);
    let vec_ident_read_only_ids_ts = gen_vec_tokens_declaration_ts(&ident_read_only_ids_ucc);
    let primary_key_field_ident = &primary_key_field.field_ident;
    let primary_key_field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&primary_key_field_ident);
    let primary_key_field_type_update_ts =
        &SelfUpdateUcc::from_type_last_segment(primary_key_field_type);
    let primary_key_field_type_update_for_query_ts =
        &SelfUpdateForQueryUcc::from_type_last_segment(primary_key_field_type);
    let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
    let gen_from_handle_ts = |ident_ts: &dyn ToTokens, ts: &dyn ToTokens| {
        quote! {
            fn #FromHandleSc(#ValueSc: #ident_ts) -> Self {
                #ts
            }
        }
    };
    let gen_select_pg_crud_not_empty_unique_vec_ident_select_ts =
        |should_add_borrow: &ShouldAddBorrow| {
            quote! {#SelectSc: #should_add_borrow pg_crud::NotEmptyUniqueVec<#ident_select_ucc>}
        };
    let select_borrow_pg_crud_not_empty_unique_vec_ident_select_ts =
        gen_select_pg_crud_not_empty_unique_vec_ident_select_ts(&ShouldAddBorrow::True);
    let select_pg_crud_not_empty_unique_vec_ident_select_ts =
        gen_select_pg_crud_not_empty_unique_vec_ident_select_ts(&ShouldAddBorrow::False);
    let pub_select_pg_crud_not_empty_unique_vec_ident_select_ts = {
        quote! {pub #select_pg_crud_not_empty_unique_vec_ident_select_ts}
    };
    let gen_fields_named_with_comma_ts = |function: &dyn Fn(&SynFieldWrapper) -> Ts2| -> Ts2 {
        let fields_ts = fields.iter().map(function);
        quote! {#(#fields_ts),*}
    };
    let gen_fields_named_without_comma_ts = |function: &dyn Fn(&SynFieldWrapper) -> Ts2| -> Ts2 {
        let fields_ts = fields.iter().map(function);
        quote! {#(#fields_ts)*}
    };
    let gen_fields_named_without_primary_key_with_comma_ts =
        |function: &dyn Fn(&SynFieldWrapper) -> Ts2| -> Ts2 {
            let fields_ts = fields_without_primary_key.iter().map(function);
            quote! {#(#fields_ts),*}
        };
    let gen_fields_named_without_primary_key_without_comma_ts =
        |function: &dyn Fn(&SynFieldWrapper) -> Ts2| -> Ts2 {
            let fields_ts = fields_without_primary_key.iter().map(function);
            quote! {#(#fields_ts)*}
        };
    let none_ts = quote! {None};
    let fields_named_with_comma_none_ts =
        gen_fields_named_with_comma_ts(&|_: &SynFieldWrapper| -> Ts2 { none_ts.clone() });
    let fields_named_without_primary_key_with_comma_none_ts =
        gen_fields_named_without_primary_key_with_comma_ts(&|_: &SynFieldWrapper| -> Ts2 {
            none_ts.clone()
        });
    let mut impl_ident_vec_ts = Vec::new();
    let impl_ident_ts = {
        let ident_prepare_pg_er_ucc = SelfPreparePgErUcc::from_tokens(&ident);
        let ts = quote! {
            #[eo_to_err_string]
            er: sqlx::Error,
            loc: location_lib::loc::Loc,
        };
        let ident_prepare_pg_er_ts = StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_thiserror_error()
            .derive_location_lib_location()
            .build_enum(
                &ident_prepare_pg_er_ucc,
                &Ts2::new(),
                &quote! {{
                    #CreateExtensionIfNotExistsPgJsonschemaUcc {
                        #ts
                    },
                    #CreateExtensionIfNotExistsUuidOsspUcc {
                        #ts
                    },
                    #PreparePgUcc {
                        #ts
                    },
                }},
            );
        let pub_fn_table_ts = quote! {
            #MustUse
            pub const fn #TableNameSc() -> &'static str {
                #ident_sc_dq_ts
            }
        };
        let fn_primary_key_ts = {
            let primary_key_field_ident_dq_ts = dq_ts(&primary_key_field_ident);
            quote! {
                const fn #PrimaryKeySc() -> &'static str {
                    #primary_key_field_ident_dq_ts
                }
            }
        };
        let pub_async_fn_prepare_extensions_ts = quote! {
            pub async fn #PrepareExtensionsSc(#PoolSc: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #ident_prepare_pg_er_ucc> {
                if let Err(er) = sqlx::query("create extension if not exists pg_jsonschema").execute(#PoolSc).await {
                    return Err(#ident_prepare_pg_er_ucc::#CreateExtensionIfNotExistsPgJsonschemaUcc {
                        er,
                        loc: location_lib::loc!()
                    });
                }
                if let Err(er) = sqlx::query("create extension if not exists \"uuid-ossp\"").execute(#PoolSc).await {
                    return Err(#ident_prepare_pg_er_ucc::#CreateExtensionIfNotExistsUuidOsspUcc {
                        er,
                        loc: location_lib::loc!()
                    });
                }
                Ok(())
            }
        };
        let pub_async_fn_prepare_pg_table_ts = {
            let prepare_pg_dq_ts = dq_ts(&format!(
                "create table if not exists {{table}} ({})",
                fields.iter().map(|_| "{}").collect::<Vec<&str>>().join(",")
            ));
            let serde_json_to_string_schemars_schema_for_generic_unwrap_ts = {
                let gen_field_type_as_pg_crud_create_table_column_query_part_create_table_query_part_ts =
                    |field_type: &Type, field_ident: &Ident, is_primary_key: bool| {
                        let is_primary_key_ts: &dyn ToTokens =
                            if is_primary_key { &TrueSc } else { &FalseSc };
                        let field_ident_dq_ts = dq_ts(&field_ident);
                        let field_type_pg_type_ts = gen_as_pg_type_ts(&field_type);
                        quote! {
                            #field_type_pg_type_ts #CreateTableColumnQueryPartSc(&#field_ident_dq_ts, #is_primary_key_ts)
                        }
                    };
                once(
                    gen_field_type_as_pg_crud_create_table_column_query_part_create_table_query_part_ts(
                        primary_key_field_type,
                        &primary_key_field.field_ident,
                        true,
                    ),
                )
                .chain(fields_without_primary_key.iter().map(|el_e48e222c| {
                    gen_field_type_as_pg_crud_create_table_column_query_part_create_table_query_part_ts(
                        &el_e48e222c.field_type,
                        &el_e48e222c.field_ident,
                        false,
                    )
                })).collect::<Vec<Ts2>>()
            };
            quote! {
                pub async fn #PreparePgTableSc(#PoolSc: &sqlx::Pool<sqlx::Postgres>, table: &str) -> Result<(), #ident_prepare_pg_er_ucc> {
                    if let Err(er) = sqlx::query(&format!(
                        #prepare_pg_dq_ts,
                        #(#serde_json_to_string_schemars_schema_for_generic_unwrap_ts),*
                    )).execute(#PoolSc).await {
                        return Err(#ident_prepare_pg_er_ucc::#PreparePgUcc {
                            er,
                            loc: location_lib::loc!()
                        });
                    }
                    Ok(())
                }
            }
        };
        let pub_async_fn_prepare_pg_ts = quote! {
            pub async fn #PreparePgSc(#PoolSc: &sqlx::Pool<sqlx::Postgres>) -> Result<(), #ident_prepare_pg_er_ucc> {
                Self::#PrepareExtensionsSc(#PoolSc).await?;
                Self::#PreparePgTableSc(#PoolSc, #ident_sc_dq_ts).await?;
                Ok(())
            }
        };
        let pub_fn_allow_methods_ts = {
            let http_method_ts = quote! {http::Method};
            quote! {
                #MustUse
                pub const fn allow_methods() -> [#http_method_ts;4] {[
                    #http_method_ts::GET,
                    #http_method_ts::POST,
                    #http_method_ts::PATCH,
                    #http_method_ts::DELETE
                ]}
            }
        };
        let fn_gen_select_query_part_ts = {
            let variants_ts = gen_fields_named_with_comma_ts(&|element: &SynFieldWrapper| {
                let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&element.field_ident);
                let init_ts = {
                    let field_ident_string_dq_ts = dq_ts(&element.field_ident);
                    let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_ts(&element.field_type);
                    quote! {
                        => match #as_pg_crud_pg_type_pg_type_ts #SelectQueryPartSc(
                            #ColumnSc,
                            #field_ident_string_dq_ts
                        ) {
                            Ok(v_820e1163) => v_820e1163,
                            Err(#ErSc) => {
                                return Err(#ErSc);
                            }
                        }
                    }
                };
                quote! {#ident_select_ucc::#field_ident_ucc_ts(#ColumnSc) #init_ts}
            });
            let option_char_ts = gen_option_tokens_declaration_ts(&Char);
            quote! {
                fn #GenSelectQueryPartSc(#select_borrow_pg_crud_not_empty_unique_vec_ident_select_ts) -> Result<#StringTs, #import_path ::#QueryPartErUcc> {
                    let mut acc_37c883c3 = #StringTs::default();
                    for el_78d2ec39 in #SelectSc.to_vec() {
                        acc_37c883c3.push_str(&match el_78d2ec39 {
                            #variants_ts
                        });
                        acc_37c883c3.push(',');
                    }
                    let _: #option_char_ts = acc_37c883c3.pop();
                    Ok(acc_37c883c3)
                }
            }
        };
        impl_ident_vec_ts.push(quote! {
            #pub_fn_table_ts
            #fn_primary_key_ts
            #pub_async_fn_prepare_extensions_ts
            #pub_async_fn_prepare_pg_table_ts
            #pub_async_fn_prepare_pg_ts
            #pub_fn_allow_methods_ts
            #fn_gen_select_query_part_ts
        });
        quote! {
            #ident_prepare_pg_er_ts
        }
    };
    let wrap_into_axum_response_ts =
        |axum_json_ts: &dyn ToTokens,
         status_code_ts: &dyn ToTokens,
         should_add_return: &ShouldAddReturn| {
            let return_ts = match should_add_return {
                ShouldAddReturn::False => quote! {response},
                ShouldAddReturn::True => quote! {return response;},
            };
            quote! {
                let mut response = axum::response::IntoResponse::into_response(
                    axum::Json(#axum_json_ts)
                );
                *response.status_mut() = #status_code_ts;
                #return_ts
            }
        };
    let gen_ident_operation_er_ucc = |operation: &Operation| {
        format!("{ident}{operation}Er")
            .parse::<Ts2>()
            .expect("79ab147e")
    };
    let gen_ident_operation_response_variants_ucc = |operation: &Operation| {
        format!("{ident}{operation}ResponseVariants")
            .parse::<Ts2>()
            .expect("f386c0d4")
    };
    let gen_init_ts = |syn_variant_wrapper: &SynVariantWrapper,
                       file: &'static str,
                       line: u32,
                       column: u32|
     -> Ts2 {
        let variant_ident = &syn_variant_wrapper.variant.ident;
        let fields_ts = if let Fields::Named(value) = &syn_variant_wrapper.variant.fields {
            value.named.iter().enumerate().map(|(index, element)| {
                let field_ident = &element.ident;
                if *field_ident.as_ref().expect("edbbd08a") == LocSc.to_string() {
                    gen_field_loc_new_ts(file, line, column)
                } else {
                    let er_increment_sc = ErSelfSc::from_display(&index);
                    quote! {#field_ident: #er_increment_sc}
                }
            })
        } else {
            panic!("10773d36");
        };
        quote! {
            #variant_ident {
                #(#fields_ts),*
            }
        }
    };
    let gen_operation_er_init_eprintln_response_creation_ts =
        |operation: &Operation,
         syn_variant_wrapper: &SynVariantWrapper,
         file: &'static str,
         line: u32,
         column: u32| {
            let ident_operation_er_ucc = gen_ident_operation_er_ucc(operation);
            let ident_operation_response_variants_ucc =
                gen_ident_operation_response_variants_ucc(operation);
            let syn_variant_init_ts = gen_init_ts(syn_variant_wrapper, file, line, column);
            let status_code_ts = syn_variant_wrapper
                .get_option_status_code()
                .expect("81efa954")
                .to_http_status_code_ts();
            let wraped_into_axum_response_ts = wrap_into_axum_response_ts(
                &quote! {#ident_operation_response_variants_ucc::#FromHandleSc(#ErSc)},
                &status_code_ts,
                &ShouldAddReturn::True,
            );
            quote! {
                let #ErSc = #ident_operation_er_ucc::#syn_variant_init_ts;
                // eprintln!("{er}");
                #wraped_into_axum_response_ts
            }
        };
    let new_syn_variant_wrapper = |variant_name: &dyn Display,
                                   status_code: Option<StatusCode>,
                                   fields_cd1fd715: Vec<(
        LocationFieldAttr,
        &dyn Display,
        Punctuated<PathSegment, PathSep>,
    )>|
     -> SynVariantWrapper {
        SynVariantWrapper {
            variant: Variant {
                attrs: {
                    let mut attrs = Vec::new();
                    if let Some(value) = status_code.as_ref() {
                        let mut segments = Punctuated::new();
                        segments.push(PathSegment {
                            ident: Ident::new(
                                &AsRefStrToScStr::case(value),
                                proc_macro2::Span::call_site(),
                            ),
                            arguments: PathArguments::None,
                        });
                        attrs.push(Attribute {
                            pound_token: Pound {
                                spans: [proc_macro2::Span::call_site()],
                            },
                            style: AttrStyle::Outer,
                            bracket_token: Bracket::default(),
                            meta: Meta::Path(Path {
                                leading_colon: None,
                                segments,
                            }),
                        });
                    }
                    attrs
                },
                ident: Ident::new(&variant_name.to_string(), proc_macro2::Span::call_site()),
                fields: Fields::Named(FieldsNamed {
                    brace_token: Brace::default(),
                    named: {
                        let mut handle = fields_cd1fd715.into_iter().fold(
                            Punctuated::new(),
                            |mut acc_37be2059, element| {
                                acc_37be2059.push_value(Field {
                                    attrs: vec![Attribute {
                                        pound_token: Pound {
                                            spans: [proc_macro2::Span::call_site()],
                                        },
                                        style: AttrStyle::Outer,
                                        bracket_token: Bracket::default(),
                                        meta: Meta::Path(Path {
                                            leading_colon: None,
                                            segments: {
                                                let mut handle = Punctuated::new();
                                                handle.push(PathSegment {
                                                    ident: Ident::new(
                                                        AttrIdentStr::attr_ident_str(&element.0),
                                                        proc_macro2::Span::call_site(),
                                                    ),
                                                    arguments: PathArguments::None,
                                                });
                                                handle
                                            },
                                        }),
                                    }],
                                    vis: Visibility::Inherited,
                                    mutability: FieldMutability::None,
                                    ident: Some(Ident::new(
                                        &element.1.to_string(),
                                        proc_macro2::Span::call_site(),
                                    )),
                                    colon_token: Some(Colon {
                                        spans: [proc_macro2::Span::call_site()],
                                    }),
                                    ty: Type::Path(TypePath {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: element.2,
                                        },
                                    }),
                                });
                                acc_37be2059.push_punct(Comma {
                                    spans: [proc_macro2::Span::call_site()],
                                });
                                acc_37be2059
                            },
                        );
                        handle.push_value(loc_syn_field());
                        handle
                    },
                }),
                discriminant: None,
            },
            status_code,
        }
    };
    let query_part_syn_variant_wrapper = new_syn_variant_wrapper(
        &QueryPartUcc,
        Some(StatusCode::BadRequest400),
        vec![(
            LocationFieldAttr::EoLocation,
            &ErSc,
            gen_simple_syn_punct(&[&PgCrudSc.to_string(), &QueryPartErUcc.to_string()]),
        )],
    );
    let gen_select_query_part_parameters_payload_select_ts = |operation: &Operation| {
        let ts_59c8df3f = gen_operation_er_init_eprintln_response_creation_ts(
            operation,
            &query_part_syn_variant_wrapper,
            file!(),
            line!(),
            column!(),
        );
        quote! {
            match Self::#GenSelectQueryPartSc(&#ParametersSc.#PayloadSc.#SelectSc) {
                Ok(v_357219fb) => v_357219fb,
                Err(#Er0) => {
                    #ts_59c8df3f
                }
            }
        }
    };
    let ident_read_ucc = SelfReadUcc::from_tokens(&ident);
    let gen_value_declaration_ts = |ts: &dyn ToTokens| {
        quote! {#PgCrudSc::#ValueUcc<#ts>}
    };
    let gen_import_path_value_init_ts = |ts: &dyn ToTokens| gen_value_init_ts(&import_path, &ts);
    let gen_impl_pg_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts =
        |ident_4d69a809: &dyn ToTokens, ts: &dyn ToTokens| {
            gen_impl_pg_crud_default_option_some_vec_one_el_ts(&ident_4d69a809, &Ts2::new(), &ts)
        };
    let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
    let ident_create_ts = {
        let ident_create_ts = StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .derive_utoipa_to_schema()
            .build_struct(&ident_create_ucc, &Ts2::new(), &{
                let ts = gen_fields_named_without_primary_key_with_comma_ts(
                    &|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let el_syn_field_ty_as_pg_type_create_ts =
                            gen_as_pg_type_create_ts(&element.field_type);
                        quote! {
                            pub #field_ident: #el_syn_field_ty_as_pg_type_create_ts
                        }
                    },
                );
                quote! {{#ts}}
            });
        let impl_ident_create_ts = {
            let primary_key_field_type_as_default_option_some_vec_one_el_call_ts = {
                let primary_key_field_type_as_pg_type_create_ts =
                    gen_as_pg_type_create_ts(&primary_key_field_type);
                quote! {
                    <
                        #primary_key_field_type_as_pg_type_create_ts as #PgCrudSc::#DefaultOptionSomeVecOneElUcc
                    >::#DefaultOptionSomeVecOneElSc()
                }
            };
            let fn_create_query_part_ts = {
                let gen_match_as_pg_crud_pg_type_pg_type_create_query_part_ts =
                    |field_type: &Type, ts: &dyn ToTokens| {
                        let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_ts(&field_type);
                        let if_write_is_err_ts = gen_if_write_is_err_ts(
                            &quote! {acc_a097110b, "{v_c3f0b59a},"},
                            &return_err_query_part_er_write_into_buffer_ts,
                        );
                        quote! {
                            match #as_pg_crud_pg_type_pg_type_ts #CreateQueryPartSc(
                                &#ts,
                                #IncrementSc
                            ) {
                                Ok(v_c3f0b59a) => {
                                    #if_write_is_err_ts
                                }
                                Err(#Er0) => {
                                    return Err(#Er0);
                                }
                            }
                        }
                    };
                let primary_key_ts = gen_match_as_pg_crud_pg_type_pg_type_create_query_part_ts(
                    primary_key_field_type,
                    &primary_key_field_type_as_default_option_some_vec_one_el_call_ts,
                );
                let column_increments_ts = gen_fields_named_without_primary_key_without_comma_ts(
                    &|element: &SynFieldWrapper| {
                        gen_match_as_pg_crud_pg_type_pg_type_create_query_part_ts(
                            &element.field_type,
                            &{
                                let el_field_ident = &element.field_ident;
                                quote! {self.#el_field_ident}
                            },
                        )
                    },
                );
                quote! {
                    fn #CreateQueryPartSc(&self, #IncrementSc: &mut u64) -> Result<#StringTs, pg_crud::#QueryPartErUcc> {
                        let mut acc_a097110b = String::default();
                        #primary_key_ts
                        #column_increments_ts
                        let _: Option<char> = acc_a097110b.pop();
                        Ok(acc_a097110b)
                    }
                }
            };
            let fn_create_query_bind_ts = {
                let gen_query_as_pg_crud_pg_type_pg_type_create_query_bind_ts =
                    |field_type: &Type, ts: &dyn ToTokens| {
                        let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_ts(&field_type);
                        quote! {
                            match #as_pg_crud_pg_type_pg_type_ts #CreateQueryBindSc(
                                #ts,
                                #QuerySc
                            ) {
                                Ok(v_3c55d2e1) => {
                                    #QuerySc = v_3c55d2e1;
                                },
                                Err(#Er0) => {
                                    return Err(#Er0);
                                }
                            }
                        }
                    };
                let primary_key_ts = gen_query_as_pg_crud_pg_type_pg_type_create_query_bind_ts(
                    primary_key_field_type,
                    &primary_key_field_type_as_default_option_some_vec_one_el_call_ts,
                );
                let binded_query_modifications_ts =
                    gen_fields_named_without_primary_key_without_comma_ts(
                        &|element: &SynFieldWrapper| {
                            gen_query_as_pg_crud_pg_type_pg_type_create_query_bind_ts(
                                &element.field_type,
                                &{
                                    let field_ident = &element.field_ident;
                                    quote! {self.#field_ident}
                                },
                            )
                        },
                    );
                quote! {
                    fn #CreateQueryBindSc(self, mut #QuerySc: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
                        sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                        String
                    > {
                        #primary_key_ts
                        #binded_query_modifications_ts
                        Ok(#QuerySc)
                    }
                }
            };
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                impl #ident_create_ucc {
                    #fn_create_query_part_ts
                    #fn_create_query_bind_ts
                }
            }
        };
        let impl_pg_crud_default_option_some_vec_one_el_for_ident_create_ts =
            gen_impl_pg_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_create_ucc,
                &{
                    let fields_init_without_primary_key_with_default_option_some_vec_one_el_ts =
                        gen_fields_named_without_primary_key_with_comma_ts(
                            &|element: &SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                quote! {#field_ident: #PgCrudDefaultOptionSomeVecOneElCall}
                            },
                        );
                    quote! {
                        Self{#fields_init_without_primary_key_with_default_option_some_vec_one_el_ts}
                    }
                },
            );
        quote! {
            #ident_create_ts
            #impl_ident_create_ts
            #impl_pg_crud_default_option_some_vec_one_el_for_ident_create_ts
        }
    };
    let ident_where_many_ucc = SelfWhereManyUcc::from_tokens(&ident);
    let ident_where_many_try_new_er_ucc = SelfWhereManyTryNewErUcc::from_tokens(&ident);
    let ident_where_many_ts = {
        let fields_declaration_ts =
            gen_fields_named_with_comma_ts(&|element: &SynFieldWrapper| -> Ts2 {
                let field_ident = &element.field_ident;
                let el_syn_field_ty_as_pg_type_where_ts =
                    gen_as_pg_type_where_ts(&element.field_type);
                let option_pg_type_where_syn_field_ty_as_pg_type_where_ts =
                    gen_option_tokens_declaration_ts(
                        &quote! {pg_crud::PgTypeWhere<#el_syn_field_ty_as_pg_type_where_ts>},
                    );
                quote! {
                    #field_ident: #option_pg_type_where_syn_field_ty_as_pg_type_where_ts
                }
            });
        let ident_where_many_ts = {
            let ts_2ecd6da8 = StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_serde_serialize()
                .derive_utoipa_to_schema()
                .build_struct(
                    &ident_where_many_ucc,
                    &Ts2::new(),
                    &quote! {{#fields_declaration_ts}},
                );
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_2ecd6da8
            }
        };
        let ident_where_many_try_new_er_ts = StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_thiserror_error()
            .derive_location_lib_location()
            .build_enum(
                &ident_where_many_try_new_er_ucc,
                &Ts2::new(),
                &quote! {{
                    #NoFieldsProvidedUcc {
                        #[eo_to_err_string]
                        loc: location_lib::loc::Loc,
                    }
                }},
            );
        let impl_pub_try_new_for_ident_where_many_ts = gen_impl_pub_try_new_for_ident_ts(
            &ident_where_many_ucc,
            &fields_declaration_ts,
            &ident_where_many_try_new_er_ucc,
            &{
                let gen_fields_ts = |should_add_borrow: ShouldAddBorrow| {
                    gen_fields_named_with_comma_ts(&|element: &SynFieldWrapper| -> Ts2 {
                        let field_ident = &element.field_ident;
                        quote! {#should_add_borrow #field_ident}
                    })
                };
                let fields_ts = gen_fields_ts(ShouldAddBorrow::True);
                let fields_inialization_ts = gen_fields_ts(ShouldAddBorrow::False);
                quote! {
                    if matches!((#fields_ts), (#fields_named_with_comma_none_ts)) {
                        return Err(#ident_where_many_try_new_er_ucc::#NoFieldsProvidedUcc {
                            loc: location_lib::loc!(),
                        });
                    }
                    Ok(Self {#fields_inialization_ts})
                }
            },
        );
        let impl_serde_deserialize_for_ident_where_many_ts =
            gen_impl_serde_deserialize_for_struct_ts(
                &ident_where_many_ucc,
                &fields
                    .iter()
                    .map(|el_3e09c5fb| (&el_3e09c5fb.field_ident, &el_3e09c5fb.field_type))
                    .collect::<Vec<(&Ident, &Type)>>(),
                fields_len,
                &|_: &Ident, syn_type: &Type| {
                    let syn_type_as_pg_type_where_ts = gen_as_pg_type_where_ts(&syn_type);
                    gen_option_tokens_declaration_ts(
                        &quote! {pg_crud::PgTypeWhere<#syn_type_as_pg_type_where_ts>},
                    )
                },
            );
        let impl_pg_crud_default_option_some_vec_one_el_for_ident_where_many_ts =
            gen_impl_pg_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_where_many_ucc,
                &{
                    let fields_ts =
                        gen_fields_named_without_comma_ts(&|el_0fd667f6: &SynFieldWrapper| {
                            let field_ident = &el_0fd667f6.field_ident;
                            quote! {
                                #field_ident: Some(
                                    #PgCrudDefaultOptionSomeVecOneElCall
                                ),
                            }
                        });
                    quote! {Self{#fields_ts}}
                },
            );
        quote! {
            #ident_where_many_ts
            #ident_where_many_try_new_er_ts
            #impl_pub_try_new_for_ident_where_many_ts
            #impl_serde_deserialize_for_ident_where_many_ts
            #impl_pg_crud_default_option_some_vec_one_el_for_ident_where_many_ts
        }
    };
    let option_ident_where_many_ucc = StdOptionOptionSelfWhereManyUcc::from_tokens(&ident);
    let option_ident_where_many_ts = {
        let option_ident_where_many_ts = StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .derive_utoipa_to_schema()
            .build_struct(&option_ident_where_many_ucc, &Ts2::new(), &{
                let option_ident_read_only_ids_standart_not_null_ts =
                    gen_option_tokens_declaration_ts(&ident_where_many_ucc);
                quote! {(pub #option_ident_read_only_ids_standart_not_null_ts);}
            });
        let impl_pg_type_where_filter_for_option_ident_where_many_ts =
            impl_pg_type_where_filter_for_ident_ts(
                &quote! {<'lifetime>},
                &option_ident_where_many_ucc,
                &Ts2::new(),
                &IncrementParameterUnderscore::False,
                &ColumnParameterUnderscore::True,
                &IsNeedToAddLogicalOperatorUnderscore::True,
                &{
                    let additional_parameters_modification_ts = fields.iter().enumerate().map(|(index, element)| {
                    let field_ident = &element.field_ident;
                    let field_ident_dq_ts = dq_ts(&field_ident);
                    let maybe_is_first_push_to_additional_parameters_already_happend_true_ts = if index == fields_len_without_primary_key {
                        Ts2::new()
                    } else {
                        quote! {is_first_push_to_additional_parameters_already_happend = true;}
                    };
                    quote! {
                        if let Some(v_da0f0616) = &#ValueSc.#field_ident {
                            match pg_crud::PgTypeWhereFilter::query_part(
                                v_da0f0616,
                                increment,
                                &#field_ident_dq_ts,
                                is_first_push_to_additional_parameters_already_happend,
                            ) {
                                Ok(v_9e3f8fdd) => {
                                    #AdditionalParametersSc.push_str(&v_9e3f8fdd);
                                    #maybe_is_first_push_to_additional_parameters_already_happend_true_ts
                                }
                                Err(#Er0) => {
                                    return Err(#Er0);
                                }
                            }
                        }
                    }
                });
                    quote! {
                        Ok(match &self.0 {
                            Some(value) => {
                                let mut #AdditionalParametersSc = #StringTs::from("where");
                                let mut is_first_push_to_additional_parameters_already_happend = false;
                                #(#additional_parameters_modification_ts)*
                                #AdditionalParametersSc
                            },
                            None => #StringTs::default()
                        })
                    }
                },
                &IsQueryBindMutable::True,
                &{
                    let binded_query_modifications_ts = gen_fields_named_without_comma_ts(
                        &|element: &SynFieldWrapper| {
                            let field_ident = &element.field_ident;
                            quote! {
                                if let Some(v_b12d6fe0) = v_27176ffb.#field_ident {
                                    match pg_crud::PgTypeWhereFilter::query_bind(v_b12d6fe0, #QuerySc) {
                                        Ok(v_edaee3b2) => {
                                            #QuerySc = v_edaee3b2;
                                        },
                                        Err(#Er0) => {
                                            return Err(#Er0);
                                        }
                                    }
                                }
                            }
                        },
                    );
                    quote! {
                        if let Some(v_27176ffb) = self.0 {
                            #binded_query_modifications_ts
                        }
                        Ok(#QuerySc)
                    }
                },
                &ImportPath::PgCrud,
            );
        let impl_pg_crud_default_option_some_vec_one_el_for_option_ident_where_many_ts =
            gen_impl_pg_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                &option_ident_where_many_ucc,
                &quote! {Self(Some(#PgCrudDefaultOptionSomeVecOneElCall))},
            );
        quote! {
            #option_ident_where_many_ts
            #impl_pg_type_where_filter_for_option_ident_where_many_ts
            #impl_pg_crud_default_option_some_vec_one_el_for_option_ident_where_many_ts
        }
    };
    let pub_where_many_option_ident_where_many_ts =
        quote! {pub #WhereManySc: #option_ident_where_many_ucc};
    let where_many_pg_crud_default_option_some_vec_one_el_call_ts = quote! {
        #WhereManySc: #PgCrudDefaultOptionSomeVecOneElCall
    };
    let gen_read_or_delete_many_additional_paramaters_init_ts =
        |read_many_or_delete_many: &ReadManyOrDeleteMany| {
            let ts_b34ec240 = gen_operation_er_init_eprintln_response_creation_ts(
                &Operation::from(read_many_or_delete_many),
                &query_part_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
            );
            quote! {
                match pg_crud::PgTypeWhereFilter::query_part(
                    &#ParametersSc.#PayloadSc.#WhereManySc,
                    &mut #IncrementSc,
                    &"",//useless //todo check if can be optimized
                    false//useless
                ) {
                    Ok(v_d1627695) => v_d1627695,
                    Err(#Er0) => {
                        #ts_b34ec240
                    }
                }
            }
        };
    let macros_helpers_location_location_field_attr_eo_to_err_string_serde =
        LocationFieldAttr::EoToErrStringSerde;
    let string_syn_punct = gen_simple_syn_punct(&["String"]);
    let try_bind_syn_variant_wrapper = new_syn_variant_wrapper(
        &TryBindUcc,
        Some(StatusCode::InternalServerEr500),
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string_serde,
            &TryBindSc,
            string_syn_punct.clone(),
        )],
    );
    let gen_query_pg_type_where_filter_query_bind_parameters_payload_where_many_query_ts =
        |operation: &Operation| {
            let ts_818208f4 = gen_operation_er_init_eprintln_response_creation_ts(
                operation,
                &try_bind_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
            );
            quote! {
                match pg_crud::PgTypeWhereFilter::query_bind(#ParametersSc.#PayloadSc.#WhereManySc, #QuerySc) {
                    Ok(v_03a58371) => {
                        #QuerySc = v_03a58371;
                    },
                    Err(#Er0) => {
                        #ts_818208f4
                    },
                }
            }
        };
    let try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_sc =
        TryFromSqlxPgPgRowWithNotEmptyUniqueVecSelfSelectSc::from_display(&ident);
    let simple_syn_punct_sqlx_error = gen_simple_syn_punct(&["sqlx", "Error"]);
    let macros_helpers_location_location_field_attr_eo_to_err_string =
        LocationFieldAttr::EoToErrString;
    let pg_syn_variant_wrapper = new_syn_variant_wrapper(
        &PgUcc,
        Some(StatusCode::InternalServerEr500),
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string,
            &PgSc,
            simple_syn_punct_sqlx_error.clone(),
        )],
    );
    let gen_match_ident_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts =
        |read_many_or_read_one: &ReadManyOrReadOne| {
            let ts_995d3d1d = gen_operation_er_init_eprintln_response_creation_ts(
                &Operation::from(read_many_or_read_one),
                &pg_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
            );
            quote! {
                match #ident_read_ucc::#try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_sc(
                    &v_b27d7d79,
                    &#ParametersSc.#PayloadSc.#SelectSc
                ) {
                    Ok(v_90535a1d) => v_90535a1d,
                    Err(#Er0) => {
                        #ts_995d3d1d
                    }
                }
            }
        };
    let select_ts = {
        let ident_select_ts = {
            let ts_179037cd = StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_partial_eq()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .build_enum(
                &ident_select_ucc,
                &Ts2::new(),
                &{
                    let variants = gen_fields_named_with_comma_ts(&|element: &SynFieldWrapper| {
                        let serde_ident_ts = dq_ts(&element.field_ident);
                        let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&element.field_ident);
                        let el_syn_field_ty_as_pg_type_select_ts = gen_as_pg_type_select_ts(&element.field_type);
                        quote! {
                            #[serde(rename(serialize = #serde_ident_ts, deserialize = #serde_ident_ts))]
                            #field_ident_ucc_ts(#el_syn_field_ty_as_pg_type_select_ts)
                        }
                    });
                    quote!{{#variants}}
                }
            );
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_179037cd
            }
        };
        let impl_display_for_ident_select_ts = gen_impl_display_ts(
            &Ts2::new(),
            &ident_select_ucc,
            &Ts2::new(),
            &quote! {write!(f, "{}", serde_json::to_string(&self).unwrap_or_else(|el_2636212f|format!("cannot serialize into json: {el_2636212f:?}")))},
        );
        let impl_location_lib_to_err_string_for_ident_select_ts = gen_impl_to_err_string_ts(
            &Ts2::new(),
            &ident_select_ucc,
            &Ts2::new(),
            &quote! {format!("{self}")},
        );
        let impl_pg_crud_all_variants_default_option_some_vec_one_el_for_ident_select_ts =
            gen_impl_pg_crud_all_variants_default_option_some_vec_one_el_ts(&ident_select_ucc, &{
                let elements_ts =
                    gen_fields_named_with_comma_ts(&|el_5282570d: &SynFieldWrapper| {
                        let field_ident_ucc_ts =
                            ToTokensToUccTs::case_or_panic(&el_5282570d.field_ident);
                        quote! {
                            Self::#field_ident_ucc_ts(#PgCrudDefaultOptionSomeVecOneElCall)
                        }
                    });
                quote! {vec![#elements_ts]}
            });
        quote! {
            #ident_select_ts
            #impl_display_for_ident_select_ts
            #impl_location_lib_to_err_string_for_ident_select_ts
            #impl_pg_crud_all_variants_default_option_some_vec_one_el_for_ident_select_ts
        }
    };
    let select_pg_crud_default_option_some_vec_one_el_call_ts = quote! {
        #SelectSc: #PgCrudDefaultOptionSomeVecOneElCall
    };
    let ident_read_ts = {
        let ident_read_ts = {
            let ts_f80f1f3e = StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_partial_eq()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .build_struct(
                &ident_read_ucc,
                &Ts2::new(),
                &{
                    let field_option_primary_key_ts = {
                        let option_value_primary_key_field_type_as_pg_type_read_ts = gen_option_tokens_declaration_ts(&gen_value_declaration_ts(&gen_as_pg_type_read_ts(&primary_key_field_type)));
                        quote! {
                            #FieldAttrSerdeSkipSerializingIfOptionIsNone
                            pub #primary_key_field_ident: #option_value_primary_key_field_type_as_pg_type_read_ts
                        }
                    };
                    let fields_options_without_primary_key_ts = gen_fields_named_without_primary_key_with_comma_ts(&|element: &SynFieldWrapper| -> Ts2 {
                        let field_vis = &element.field_vis;
                        let field_ident = &element.field_ident;
                        let option_value_field_type_as_pg_type_read_ts = gen_option_tokens_declaration_ts(&gen_value_declaration_ts(&gen_as_pg_type_read_ts(&element.field_type)));
                        quote! {
                            #FieldAttrSerdeSkipSerializingIfOptionIsNone
                            #field_vis #field_ident: #option_value_field_type_as_pg_type_read_ts
                        }
                    });
                    quote!{{
                        #field_option_primary_key_ts,
                        #fields_options_without_primary_key_ts
                    }}
                }
            );
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_f80f1f3e
            }
        };
        let impl_ident_read_ts = {
            let fn_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts = {
                let declaration_primary_key_ts = {
                    let option_value_primary_key_field_type_as_primary_key_ts =
                        gen_option_tokens_declaration_ts(&gen_value_declaration_ts(
                            &primary_key_field_type_as_pg_type_read_ucc,
                        ));
                    quote! {
                        let mut #primary_key_field_ident: #option_value_primary_key_field_type_as_primary_key_ts = None;
                    }
                };
                let declaration_without_primary_key_ts =
                    gen_fields_named_without_primary_key_without_comma_ts(
                        &|element: &SynFieldWrapper| {
                            let field_ident = &element.field_ident;
                            let option_value_field_type_as_pg_type_read_ts =
                                gen_option_tokens_declaration_ts(&gen_value_declaration_ts(
                                    &gen_as_pg_type_read_ts(&element.field_type),
                                ));
                            quote! {
                                let mut #field_ident: #option_value_field_type_as_pg_type_read_ts = None;
                            }
                        },
                    );
                //todo reuse code?
                let assignment_variant_primary_key_ts = {
                    let primary_key_field_ident_string_dq_ts = dq_ts(&primary_key_field_ident);
                    quote! {
                        #ident_select_ucc::#primary_key_field_ident_ucc_ts(_) => match sqlx::Row::try_get::<
                            #primary_key_field_type_as_pg_type_read_ucc,
                            #RefStr
                        >(
                            value,
                            #primary_key_field_ident_string_dq_ts
                        ) {
                            Ok(v_dccdf117) => {
                                #primary_key_field_ident = Some(#import_path::#ValueUcc { value: v_dccdf117});
                            },
                            Err(#Er0) => {
                                return Err(#Er0);
                            }
                        }
                    }
                };
                let assignment_variants_without_primary_key_ts = fields_without_primary_key
                    .iter()
                    .map(|el_3ce946f9| {
                        let field_ident = &el_3ce946f9.field_ident;
                        let field_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&el_3ce946f9.field_ident);
                        let field_ident_string_dq_ts = dq_ts(&el_3ce946f9.field_ident);
                        let el_syn_field_ty_as_pg_type_read_ts = gen_as_pg_type_read_ts(&el_3ce946f9.field_type);
                        quote! {
                            #ident_select_ucc::#field_ident_ucc_ts(_) => match sqlx::Row::try_get::<
                                #el_syn_field_ty_as_pg_type_read_ts,
                                #RefStr
                            >(
                                value,
                                #field_ident_string_dq_ts
                            ) {
                                Ok(v_09b0fc09) => {
                                    #field_ident = Some(#import_path::#ValueUcc { value: v_09b0fc09});
                                },
                                Err(#Er0) => {
                                    return Err(#Er0);
                                }
                            }
                        }
                    })
                    .collect::<Vec<Ts2>>();
                let fields_init_ts = &fields
                    .iter()
                    .map(|el_2bfe6afc| &el_2bfe6afc.field_ident)
                    .collect::<Vec<&Ident>>();
                quote! {
                    fn #try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_sc(
                        #ValueSc: &sqlx::postgres::PgRow,
                        #select_borrow_pg_crud_not_empty_unique_vec_ident_select_ts
                    ) -> Result<Self, sqlx::Error> {
                        #declaration_primary_key_ts
                        #declaration_without_primary_key_ts
                        for el_dca9f0b7 in #SelectSc.to_vec() {
                            match el_dca9f0b7 {
                                #assignment_variant_primary_key_ts,
                                #(#assignment_variants_without_primary_key_ts),*
                            }
                        }
                        Ok(Self {#(#fields_init_ts),*})
                    }
                }
            };
            quote! {
                impl #ident_read_ucc {
                    #fn_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts
                }
            }
        };
        quote! {
            #ident_read_ts
            #impl_ident_read_ts
        }
    };
    let ident_read_only_ids_ts = {
        let ident_read_only_ids_ts = {
            let ts_472e3ebf = StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(&ident_read_only_ids_ucc, &Ts2::new(), &{
                    enum WrapIntoOption {
                        False,
                        True,
                    }
                    let gen_field_ts =
                        |field_ident: &dyn ToTokens,
                         field_type: &dyn ToTokens,
                         wrap_into_option: &WrapIntoOption| {
                            let field_type_ts = match &wrap_into_option {
                                WrapIntoOption::False => {
                                    gen_as_pg_type_read_only_ids_ts(&field_type)
                                }
                                WrapIntoOption::True => gen_option_tokens_declaration_ts(
                                    &gen_as_pg_type_read_only_ids_ts(&field_type),
                                ),
                            };
                            quote! {
                                pub #field_ident: #field_type_ts
                            }
                        };
                    let primary_key_ts = gen_field_ts(
                        &primary_key_field_ident,
                        &primary_key_field_type,
                        &WrapIntoOption::False,
                    );
                    let ts = gen_fields_named_without_primary_key_with_comma_ts(
                        &|element: &SynFieldWrapper| {
                            gen_field_ts(
                                &element.field_ident,
                                &element.field_type,
                                &WrapIntoOption::True,
                            )
                        },
                    );
                    quote! {{
                        #primary_key_ts,
                        #ts
                    }}
                });
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_472e3ebf
            }
        };
        let impl_sqlx_row_for_ident_read_only_ids_ts = {
            let undescore_underscore_row = quote! {__row};
            let where_field_types_ts = gen_fields_named_with_comma_ts(
                &|element: &SynFieldWrapper| {
                    let field_type = &element.field_type;
                    let el_syn_field_ty_as_pg_type_read_only_ids_ts =
                        gen_as_pg_type_read_only_ids_ts(&field_type);
                    quote! {
                        #el_syn_field_ty_as_pg_type_read_only_ids_ts: ::sqlx::decode::Decode<'lifetime, R::Database>
                    }
                },
            );
            let primary_key_ts = {
                let el_syn_field_ty_as_pg_type_read_only_ids_ts =
                    gen_as_pg_type_read_only_ids_ts(&primary_key_field_type);
                let field_ident_dq_ts = dq_ts(&primary_key_field_ident);
                quote! {
                    let #primary_key_field_ident = match sqlx::Row::try_get::<#el_syn_field_ty_as_pg_type_read_only_ids_ts, &str>(
                        #undescore_underscore_row,
                        #field_ident_dq_ts
                    ) {
                        Ok(v_283179dd) => v_283179dd,
                        Err(#Er0) => {
                            return Err(#Er0);
                        }
                    };
                }
            };
            let fields_init_ts = gen_fields_named_without_primary_key_without_comma_ts(
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let field_ident_dq_ts = dq_ts(&quote! {#field_ident});
                    let el_syn_field_ty_as_pg_type_read_only_ids_ts =
                        gen_as_pg_type_read_only_ids_ts(&field_type);
                    quote! {
                        let #field_ident = sqlx::Row::try_get::<
                            #el_syn_field_ty_as_pg_type_read_only_ids_ts,
                            &str
                        >(#undescore_underscore_row, #field_ident_dq_ts).ok();
                    }
                },
            );
            let self_fields_ts = gen_fields_named_with_comma_ts(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                quote! {#field_ident}
            });
            quote! {
                impl<'lifetime, R: ::sqlx::Row<Database = sqlx::Postgres>> ::sqlx::FromRow<'lifetime, R> for #ident_read_only_ids_ucc
                where
                    &'lifetime ::std::primitive::str: ::sqlx::ColumnIndex<R>,
                    #where_field_types_ts
                {
                    fn from_row(#undescore_underscore_row: &'lifetime R) -> ::sqlx::Result<Self> {
                        #primary_key_ts
                        #fields_init_ts
                        Ok(Self { #self_fields_ts })
                    }
                }
            }
        };
        quote! {
            #ident_read_only_ids_ts
            #impl_sqlx_row_for_ident_read_only_ids_ts
        }
    };
    // println!("{ident_read_only_ids_ts}");
    let gen_ident_try_operation_er_ucc = |operation: &Operation| {
        format!("{ident}Try{operation}Er")
            .parse::<Ts2>()
            .expect("6a5468b2")
    };
    let ident_try_read_many_er_ucc = gen_ident_try_operation_er_ucc(&Operation::ReadMany);
    let gen_ident_operation_er_with_serde_ucc = |operation: &Operation| {
        format!("{ident}{operation}ErWithSerde")
            .parse::<Ts2>()
            .expect("f9e053d1")
    };
    let pg_crud_order_by_ts = quote! {#PgCrudSc::#OrderByUcc};
    let ident_update_ucc = SelfUpdateUcc::from_tokens(&ident);
    let ident_update_many_parameters_ucc = SelfUpdateManyParametersUcc::from_tokens(&ident);
    let ident_update_many_payload_ucc = SelfUpdateManyPayloadUcc::from_tokens(&ident);
    let ident_update_try_new_er_ucc = SelfUpdateTryNewErUcc::from_tokens(&ident);
    let ident_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident);
    let ident_update_ts = {
        let gen_option_value_field_type_as_pg_type_update_ts = |syn_type: &Type| {
            let path_value_ts = {
                let value = format!("{PgCrudSc}::{ValueUcc}");
                value.parse::<Ts2>().expect("dbdbb7f2")
            };
            let syn_type_as_pg_type_update_ts = gen_as_pg_type_update_ts(&syn_type);
            gen_option_tokens_declaration_ts(
                &quote! {#path_value_ts<#syn_type_as_pg_type_update_ts>},
            )
        };
        let fields_declaration_ts = {
            let fields_named_without_primary_key_ts =
                gen_fields_named_without_primary_key_with_comma_ts(
                    &|element: &SynFieldWrapper| -> Ts2 {
                        let field_ident = &element.field_ident;
                        let option_value_field_type_as_pg_type_update_ts =
                            gen_option_value_field_type_as_pg_type_update_ts(&element.field_type);
                        quote! {
                            #field_ident: #option_value_field_type_as_pg_type_update_ts
                        }
                    },
                );
            quote! {
                #primary_key_field_ident: #primary_key_field_type_update_ts,
                #fields_named_without_primary_key_ts
            }
        };
        let ident_update_ts = {
            let ts_a09c0471 = StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_serde_serialize()
                .derive_utoipa_to_schema()
                .build_struct(
                    &ident_update_ucc,
                    &Ts2::new(),
                    &quote! {{#fields_declaration_ts}},
                );
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_a09c0471
            }
        };
        let ident_update_try_new_er_ts = StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_thiserror_error()
            .derive_location_lib_location()
            .build_enum(
                &ident_update_try_new_er_ucc,
                &Ts2::new(),
                &quote! {{
                    #NoFieldsProvidedUcc {
                        #[eo_to_err_string]
                        loc: location_lib::loc::Loc,
                    }
                }},
            );
        let impl_pub_try_new_for_ident_update_ts = gen_impl_pub_try_new_for_ident_ts(
            &ident_update_ucc,
            &fields_declaration_ts,
            &ident_update_try_new_er_ucc,
            &{
                let (left_ts, right_ts) = {
                    let maybe_wrap_into_braces_handle_ts = |ts: &dyn ToTokens| {
                        maybe_wrap_into_braces_ts(ts, fields_len_without_primary_key > 1)
                    };
                    (
                        maybe_wrap_into_braces_handle_ts(
                            &gen_fields_named_without_primary_key_with_comma_ts(
                                &|element: &SynFieldWrapper| -> Ts2 {
                                    let field_ident = &element.field_ident;
                                    quote! {&#field_ident}
                                },
                            ),
                        ),
                        maybe_wrap_into_braces_handle_ts(
                            &fields_named_without_primary_key_with_comma_none_ts,
                        ),
                    )
                };
                let fields_inialization_ts =
                    gen_fields_named_with_comma_ts(&|element: &SynFieldWrapper| -> Ts2 {
                        let field_ident = &element.field_ident;
                        quote! {#field_ident}
                    });
                quote! {
                    if matches!(#left_ts, #right_ts) {
                        return Err(#ident_update_try_new_er_ucc::#NoFieldsProvidedUcc {
                            loc: location_lib::loc!(),
                        });
                    }
                    Ok(Self {#fields_inialization_ts})
                }
            },
        );
        let impl_serde_deserialize_for_ident_update_ts = gen_impl_serde_deserialize_for_struct_ts(
            &ident_update_ucc,
            &fields
                .iter()
                .map(|el_2bfe6afc| (&el_2bfe6afc.field_ident, &el_2bfe6afc.field_type))
                .collect::<Vec<(&Ident, &Type)>>(),
            fields_len,
            &|syn_ident: &Ident, syn_type: &Type| {
                if syn_ident == primary_key_field_ident {
                    quote! {#primary_key_field_type_update_ts}
                } else {
                    gen_option_value_field_type_as_pg_type_update_ts(syn_type)
                }
            },
        );
        let impl_pg_crud_default_option_some_vec_one_el_for_ident_update_ts =
            gen_impl_pg_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_update_ucc,
                &{
                    let primary_key_field_with_default_option_some_vec_one_el_ts = {
                        quote! {
                            #primary_key_field_ident: #PgCrudDefaultOptionSomeVecOneElCall
                        }
                    };
                    let fields_without_primary_key_with_default_option_some_vec_one_el_ts =
                        gen_fields_named_without_primary_key_with_comma_ts(
                            &|element: &SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                quote! {
                                    #field_ident: Some(pg_crud::Value{
                                        #ValueSc: #PgCrudDefaultOptionSomeVecOneElCall
                                    })
                                }
                            },
                        );
                    quote! {Self{
                        #primary_key_field_with_default_option_some_vec_one_el_ts,
                        #fields_without_primary_key_with_default_option_some_vec_one_el_ts
                    }}
                },
            );
        quote! {
            #ident_update_ts
            #ident_update_try_new_er_ts
            #impl_pub_try_new_for_ident_update_ts
            #impl_serde_deserialize_for_ident_update_ts
            #impl_pg_crud_default_option_some_vec_one_el_for_ident_update_ts
        }
    };
    let ident_update_for_query_ts = {
        let ident_update_for_query_ts = {
            let ts_50ae0c5f = StructOrEnumDeriveTokenStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_serde_serialize()
            .derive_utoipa_to_schema()
            .build_struct(
                &ident_update_for_query_ucc,
                &Ts2::new(),
                &{
                    let fields_named_without_primary_key_ts = gen_fields_named_without_primary_key_with_comma_ts(&|element: &SynFieldWrapper| -> Ts2 {
                        let field_ident = &element.field_ident;
                        let option_value_field_type_as_pg_type_update_for_query_ts = {
                            let path_value_ts = {
                                let value = format!("{PgCrudSc}::{ValueUcc}");
                                value.parse::<Ts2>().expect("2b09d4ae")
                            };
                            let syn_type_as_pg_type_update_for_query_ts = gen_as_pg_type_update_for_query_ts(&element.field_type);
                            gen_option_tokens_declaration_ts(&quote! {#path_value_ts<#syn_type_as_pg_type_update_for_query_ts>})
                        };
                        quote! {#field_ident: #option_value_field_type_as_pg_type_update_for_query_ts}
                    });
                    quote!{{
                        #primary_key_field_ident: #primary_key_field_type_update_for_query_ts,
                        #fields_named_without_primary_key_ts
                    }}
                }
            );
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_50ae0c5f
            }
        };
        let impl_ident_update_for_query_ts = {
            let update_query_part_primary_key_ts = {
                quote! {
                    fn #UpdateQueryPartPrimaryKeySc(&self, #IncrementSc: &mut u64) -> Result<#StringTs, #PgCrudSc::#QueryPartErUcc> {
                        match #primary_key_field_type_as_pg_type_ts #UpdateQueryPartSc(
                            &self.#primary_key_field_ident,
                            "",
                            #ident::#PrimaryKeySc(),
                            "",
                            #IncrementSc,
                        ) {
                            Ok(#ValueSc) => Ok(#ValueSc),
                            Err(#Er0) => Err(#Er0)
                        }
                    }
                }
            };
            let update_query_part_fields_ts = gen_fields_named_without_primary_key_without_comma_ts(
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_ident_dq_ts = dq_ts(&field_ident);
                    let update_query_part_field_ident_sc =
                        UpdateQueryPartSelfSc::from_tokens(&field_ident);
                    let field_type_as_pg_crud_pg_type_pg_type_ts =
                        gen_as_pg_type_ts(&element.field_type);
                    quote! {
                        fn #update_query_part_field_ident_sc(
                            #ValueSc: &pg_crud::Value<#field_type_as_pg_crud_pg_type_pg_type_ts #UpdateForQueryUcc>,
                            #IncrementSc: &mut u64
                        ) -> Result<#StringTs, #PgCrudSc::#QueryPartErUcc> {
                            match #field_type_as_pg_crud_pg_type_pg_type_ts #UpdateQueryPartSc(
                                &#ValueSc.#ValueSc,
                                #field_ident_dq_ts,
                                #field_ident_dq_ts,
                                "",
                                #IncrementSc
                            ) {
                                Ok(v_f75dfd93) => Ok(v_f75dfd93),
                                Err(#Er0) => Err(#Er0),
                            }
                        }
                    }
                },
            );
            let select_only_updated_ids_query_part_ts = {
                let primary_key_ts = {
                    let primary_key_field_ident_dq_ts = dq_ts(&primary_key_field_ident);
                    quote! {
                        acc_88c91f52.push_str(&match <#primary_key_field_type as pg_crud::PgType>::#SelectOnlyUpdatedIdsQueryPartSc(
                            &self.#primary_key_field_ident,
                            #primary_key_field_ident_dq_ts,
                            increment,
                        ){
                            Ok(v) => v,
                            Err(er) => {
                                return Err(er);
                            }
                        });
                    }
                };
                let ts = fields_without_primary_key.iter().map(|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_ident_dq_ts = dq_ts(&field_ident);
                    let field_type_as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_ts(&element.field_type);
                    quote! {
                        if let Some(v_90f79b11) = &self.#field_ident {
                            acc_88c91f52.push_str(&match #field_type_as_pg_crud_pg_type_pg_type_ts #SelectOnlyUpdatedIdsQueryPartSc(
                                &v_90f79b11.#ValueSc,
                                #field_ident_dq_ts,
                                increment,
                            ){
                                Ok(v_47a6f597) => v_47a6f597,
                                Err(#ErSc) => {
                                    return Err(#ErSc);
                                }
                            });
                        }
                    }
                });
                quote! {
                    fn #SelectOnlyUpdatedIdsQueryPartSc(&self, #IncrementSc: &mut u64) -> Result<#StringTs, pg_crud::QueryPartEr> {
                        let mut acc_88c91f52 = String::new();
                        #primary_key_ts
                        #(#ts)*
                        let _: Option<char> = acc_88c91f52.pop();
                        Ok(acc_88c91f52)
                    }
                }
            };
            let update_handle_ts = gen_from_handle_ts(&ident_update_ucc, &{
                let primary_key_field_type_as_pg_type_update_for_query_ts =
                    gen_as_pg_type_update_for_query_ts(&primary_key_field_type);
                let fields_named_without_primary_key_ts =
                    gen_fields_named_without_primary_key_with_comma_ts(
                        &|element: &SynFieldWrapper| -> Ts2 {
                            let field_ident = &element.field_ident;
                            let value_init_ts = gen_import_path_value_init_ts(&{
                                let field_type_as_pg_type_update_for_query_ts =
                                    gen_as_pg_type_update_for_query_ts(&element.field_type);
                                quote! {
                                     #field_type_as_pg_type_update_for_query_ts::from(v_0e64c53a.#ValueSc)
                                }
                            });
                            quote! {
                                #field_ident: value.#field_ident.map(|v_0e64c53a| #value_init_ts)
                            }
                        },
                    );
                quote! {
                    Self {
                        #primary_key_field_ident: #primary_key_field_type_as_pg_type_update_for_query_ts::from(#ValueSc.#primary_key_field_ident),
                        #fields_named_without_primary_key_ts
                    }
                }
            });
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                impl #ident_update_for_query_ucc {
                    #update_query_part_primary_key_ts
                    #update_query_part_fields_ts
                    #select_only_updated_ids_query_part_ts
                    #update_handle_ts
                }
            }
        };
        quote! {
            #ident_update_for_query_ts
            #impl_ident_update_for_query_ts
        }
    };
    let gen_match_update_query_part_primary_key_ts = |operation: &Operation, ts: &dyn ToTokens| {
        let ts_75b4019b = gen_operation_er_init_eprintln_response_creation_ts(
            operation,
            &query_part_syn_variant_wrapper,
            file!(),
            line!(),
            column!(),
        );
        quote! {
            match #ts.#UpdateQueryPartPrimaryKeySc(&mut #IncrementSc) {
                Ok(v_f269a3b2) => v_f269a3b2,
                Err(#Er0) => {
                    #ts_75b4019b
                }
            }
        }
    };
    let row_and_rollback_syn_variant_wrapper = new_syn_variant_wrapper(
        &RowAndRollbackUcc,
        Some(StatusCode::InternalServerEr500),
        vec![
            (
                macros_helpers_location_location_field_attr_eo_to_err_string,
                &RowSc,
                simple_syn_punct_sqlx_error.clone(),
            ),
            (
                macros_helpers_location_location_field_attr_eo_to_err_string,
                &RollbackSc,
                simple_syn_punct_sqlx_error,
            ),
        ],
    );
    let sqlx_query_sqlx_pg_ts = quote! {sqlx::query::<sqlx::Postgres>};
    let (pg_crud_pg_type_where_filter_query_part_ts, pg_crud_pg_type_where_filter_query_bind_ts) = {
        let pg_crud_pg_type_where_filter_ts = quote! {#PgCrudSc::PgTypeWhereFilter::};
        (
            quote! {#pg_crud_pg_type_where_filter_ts #QueryPartSc},
            quote! {#pg_crud_pg_type_where_filter_ts #QueryBindSc},
        )
    };
    let vec_struct_options_ident_ts = gen_vec_tokens_declaration_ts(&ident_read_ucc);
    let not_unique_field_syn_variant_wrapper = new_syn_variant_wrapper(
        &NotUniqueFieldUcc,
        Some(StatusCode::BadRequest400),
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string_serde,
            &NotUniqueFieldSc,
            gen_simple_syn_punct(&[&ident_select_ucc.to_string()]),
        )],
    );
    let simple_syn_punct_serde_error = gen_simple_syn_punct(&["serde_json", "Error"]);
    let serde_json_to_string_syn_variant_wrapper = new_syn_variant_wrapper(
        &SerdeJsonToStringUcc,
        None,
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string,
            &SerdeJsonToStringSc,
            simple_syn_punct_serde_error.clone(),
        )],
    );
    let simple_syn_punct_reqwest_error = gen_simple_syn_punct(&["reqwest", "Error"]);
    let failed_to_get_response_text_syn_variant_wrapper = new_syn_variant_wrapper(
        &FailedToGetResponseTextUcc,
        Some(StatusCode::BadRequest400),
        vec![
            (
                macros_helpers_location_location_field_attr_eo_to_err_string,
                &StatusCodeSc,
                gen_simple_syn_punct(&["reqwest", "StatusCode"]),
            ),
            (
                macros_helpers_location_location_field_attr_eo_to_err_string,
                &HeadersSc,
                gen_simple_syn_punct(&["reqwest", "header", "HeaderMap"]),
            ),
            (
                macros_helpers_location_location_field_attr_eo_to_err_string,
                &ReqwestSc,
                simple_syn_punct_reqwest_error.clone(),
            ),
        ],
    );
    let deserialize_response_syn_variant_wrapper = new_syn_variant_wrapper(
        &DeserializeResponseUcc,
        None,
        vec![
            (
                macros_helpers_location_location_field_attr_eo_to_err_string,
                &StatusCodeSc,
                gen_simple_syn_punct(&["reqwest", "StatusCode"]),
            ),
            (
                macros_helpers_location_location_field_attr_eo_to_err_string,
                &HeadersSc,
                gen_simple_syn_punct(&["reqwest", "header", "HeaderMap"]),
            ),
            (
                macros_helpers_location_location_field_attr_eo_to_err_string_serde,
                &ResponseTextSc,
                string_syn_punct,
            ),
            (
                macros_helpers_location_location_field_attr_eo_to_err_string,
                &SerdeSc,
                simple_syn_punct_serde_error.clone(),
            ),
        ],
    );
    let reqwest_syn_variant_wrapper = new_syn_variant_wrapper(
        &ReqwestUcc,
        None,
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string,
            &ReqwestSc,
            simple_syn_punct_reqwest_error,
        )],
    );
    let check_body_size_syn_variant_wrapper = new_syn_variant_wrapper(
        &CheckBodySizeUcc,
        Some(StatusCode::BadRequest400),
        vec![(
            LocationFieldAttr::EoLocation,
            &CheckBodySizeSc,
            gen_simple_syn_punct(&[
                &PgCrudSc.to_string(),
                "check_body_size",
                &BodySizeErUcc.to_string(),
            ]),
        )],
    );
    let serde_json_syn_variant_wrapper = new_syn_variant_wrapper(
        &SerdeJsonUcc,
        Some(StatusCode::BadRequest400),
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string,
            &SerdeJsonSc,
            simple_syn_punct_serde_error,
        )],
    );
    let header_content_type_application_json_not_found_syn_variant_wrapper =
        new_syn_variant_wrapper(
            &HeaderContentTypeApplicationJsonNotFoundUcc,
            Some(StatusCode::BadRequest400),
            Vec::<(
                LocationFieldAttr,
                &'static dyn Display,
                Punctuated<PathSegment, PathSep>,
            )>::default(),
        );
    let common_http_request_syn_variants = {
        vec![
            serde_json_to_string_syn_variant_wrapper
                .get_syn_variant()
                .clone(),
            failed_to_get_response_text_syn_variant_wrapper
                .get_syn_variant()
                .clone(),
            deserialize_response_syn_variant_wrapper
                .get_syn_variant()
                .clone(),
            reqwest_syn_variant_wrapper.get_syn_variant().clone(),
        ]
    };
    let gen_additional_er_variants = |di_bde7efb1: &DeriveInput,
                                      gen_pg_table_attr: GenPgTableAttr|
     -> Vec<Variant> {
        let gen_pg_table_attr_str = gen_pg_table_attr.to_string();
        let common_additional_er_variants_attr_ts =
            get_macro_attr_meta_list_ts(&di_bde7efb1.attrs, &gen_pg_table_attr.gen_path_to_attr());
        let di_894e3269: DeriveInput =
            parse2((*common_additional_er_variants_attr_ts).clone()).expect("1b80783d");
        assert!(di_894e3269.ident == gen_pg_table_attr_str, "8a66c852");
        let variants = if let Data::Enum(data_enum) = di_894e3269.data {
            data_enum.variants
        } else {
            panic!("f3ddc78c");
        };
        variants.into_iter().collect()
    };
    let common_additional_er_variants =
        gen_additional_er_variants(&di, GenPgTableAttr::CommonAdditionalErVariants);
    let common_route_syn_variants = {
        let mut acc_94f701ab = vec![
            check_body_size_syn_variant_wrapper.get_syn_variant(),
            pg_syn_variant_wrapper.get_syn_variant(),
            serde_json_syn_variant_wrapper.get_syn_variant(),
            header_content_type_application_json_not_found_syn_variant_wrapper.get_syn_variant(),
        ];
        for el_af152d67 in &common_additional_er_variants {
            acc_94f701ab.push(el_af152d67);
        }
        acc_94f701ab
    };
    let common_additional_logic_ts = get_macro_attr_meta_list_ts(
        &di.attrs,
        &GenPgTableAttr::CommonAdditionalLogic.gen_path_to_attr(),
    );
    let gen_pub_handle_ts = |is_pub: bool| {
        if is_pub {
            quote! {pub}
        } else {
            Ts2::new()
        }
    };
    let gen_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts =
        |primary_key_type_ts: &dyn ToTokens| {
            let is_pub = true;
            let pub_handle_ts = gen_pub_handle_ts(is_pub);
            quote! {#pub_handle_ts #primary_key_field_ident: #primary_key_type_ts}
        };
    let gen_match_pg_transaction_rollback_await_ts =
        |operation: &Operation,
         pg_file: &'static str,
         pg_line: u32,
         pg_column: u32,
         row_and_rollback_file: &'static str,
         row_and_rollback_line: u32,
         row_and_rollback_column: u32| {
            let ts_91f19090 = gen_operation_er_init_eprintln_response_creation_ts(
                operation,
                &pg_syn_variant_wrapper,
                pg_file,
                pg_line,
                pg_column,
            );
            let row_and_rollback_syn_variant_er_init_eprintln_response_creation_ts =
                gen_operation_er_init_eprintln_response_creation_ts(
                    operation,
                    &row_and_rollback_syn_variant_wrapper,
                    row_and_rollback_file,
                    row_and_rollback_line,
                    row_and_rollback_column,
                );
            quote! {{
                if let Err(#Er1) = #ExecutorSc.#RollbackSc().await {
                    #row_and_rollback_syn_variant_er_init_eprintln_response_creation_ts
                }
                #ts_91f19090
            }}
        };
    let gen_drop_rows_match_pg_transaction_rollback_await_handle_ts =
        |operation: &Operation,
         pg_file: &'static str,
         pg_line: u32,
         pg_column: u32,
         row_and_rollback_file: &'static str,
         row_and_rollback_line: u32,
         row_and_rollback_column: u32| {
            let match_pg_transaction_rollback_await_ts = gen_match_pg_transaction_rollback_await_ts(
                operation,
                pg_file,
                pg_line,
                pg_column,
                row_and_rollback_file,
                row_and_rollback_line,
                row_and_rollback_column,
            );
            quote! {
                drop(#RowsSc);
                #match_pg_transaction_rollback_await_ts
            }
        };
    let wrap_into_value_ts = |ts: &dyn ToTokens| {
        quote! {
            let #ValueSc = {
                #ts
            };
        }
    };
    let gen_fetch_ts = |executor_name_ts: &dyn ToTokens,
                        value_handle_ts: &dyn ToTokens,
                        try_next_er_init_ts: &dyn ToTokens,
                        should_wrap_into_value: &ShouldWrapIntoValue| {
        let ts = quote! {
            let mut #RowsSc = #BindedQuerySc.fetch(#executor_name_ts.as_mut());
            let mut acc_d16ac269 = Vec::new();
            while let Some(v_d9cc2c36) = match #PgCrudSc::TryStreamExt::try_next(&mut #RowsSc).await {
                Ok(v_19f3d6e1) => match v_19f3d6e1 {
                    Some(v_b27d7d79) => #value_handle_ts,
                    None => None,
                },
                Err(#Er0) => {
                    #try_next_er_init_ts
                }
            }
            {
                acc_d16ac269.push(v_d9cc2c36);
            }
            acc_d16ac269
        };
        match should_wrap_into_value {
            ShouldWrapIntoValue::False => ts,
            ShouldWrapIntoValue::True => wrap_into_value_ts(&ts),
        }
    };
    let gen_fetch_one_ts = |executor_name_ts: &dyn ToTokens,
                            value_handle_ts: &dyn ToTokens,
                            fetch_one_er_init_ts: &dyn ToTokens| {
        quote! {
            match #BindedQuerySc.fetch_one(#executor_name_ts.as_mut()).await {
                Ok(v_b27d7d79) => {
                    #value_handle_ts
                },
                Err(#Er0) => {
                    #fetch_one_er_init_ts
                }
            }
        }
    };
    let gen_sqlx_row_try_get_primary_key_ts =
        |sqlx_row_try_get_type_ts: &dyn ToTokens, ok_ts: &dyn ToTokens, err_ts: &dyn ToTokens| {
            quote! {
                match #SqlxRow::try_get::<
                    #sqlx_row_try_get_type_ts,
                    #RefStr
                >(&v_b27d7d79, Self::#PrimaryKeySc()) {
                    Ok(v_69ecb6a9) => #ok_ts,
                    Err(#Er0) => {
                        #err_ts
                    }
                }
            }
        };
    let wrap_content_into_pg_transaction_begin_commit_value_ts =
        |operation: &Operation, ts: &dyn ToTokens| {
            let pg_transaction_begin_ts = {
                let ts_efebc55b = gen_operation_er_init_eprintln_response_creation_ts(
                    operation,
                    &pg_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
                quote! {
                    let mut #ExecutorSc = match #SqlxAcquire::#BeginSc(#ExecutorAcquireSc).await {
                        Ok(v_1aaca28f) => v_1aaca28f,
                        Err(#Er0) => {
                            #ts_efebc55b
                        }
                    };
                }
            };
            let pg_transaction_commit_ts = {
                let pg_syn_variant_er_init_eprintln_response_creation_ts =
                    gen_operation_er_init_eprintln_response_creation_ts(
                        operation,
                        &pg_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                quote! {
                    if let Err(#Er0) = #ExecutorSc.#CommitSc().await {
                        #pg_syn_variant_er_init_eprintln_response_creation_ts
                    }
                }
            };
            quote! {
                #pg_transaction_begin_ts
                #ts
                #pg_transaction_commit_ts
                #ValueSc
            }
        };
    let gen_location_variant_ts = |er_variant: &Variant| -> Ts2 {
        let variant_ident = &er_variant.ident;
        let Fields::Named(fields_named) = &er_variant.fields else {
            panic!("2acd4725");
        };
        let fields_mapped_into_ts = fields_named.named.iter().map(|field| {
            let field_ident = field.ident.as_ref().expect("a21dc807");
            let location_attr = if *field_ident == *LocSc.to_string() {
                Ts2::new()
            } else {
                let mut location_attr: Option<LocationFieldAttr> = None;
                for el_1c83e302 in &field.attrs {
                    if el_1c83e302.path().segments.len() == 1 {
                        let segment = el_1c83e302.path().segments.first().expect("5bd7ed8d");
                        if let Ok(v) =
                            { <LocationFieldAttr as FromStr>::from_str(&segment.ident.to_string()) }
                        {
                            if location_attr.is_some() {
                                panic!("9a469d36")
                            } else {
                                location_attr = Some(v);
                            }
                        }
                    }
                }
                location_attr.expect("d1003b2e").to_attr_view_ts()
            };
            let field_type = &field.ty;
            quote! {
                #location_attr
                #field_ident: #field_type
            }
        });
        quote! {
            #variant_ident {
                #(#fields_mapped_into_ts),*
            }
        }
    };
    let gen_ident_try_operation_logic_response_variants_ident_operation_er_convert_ts =
        |operation: &Operation,
         desirable_type_ts: &dyn ToTokens,
         type_variants_from_request_response_syn_variants: &Vec<Variant>|
         -> Ts2 {
            let ident_operation_response_variants_ucc =
                gen_ident_operation_response_variants_ucc(operation);
            let ident_try_operation_logic_response_variants_ts = {
                let ts_c997a274 = StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_enum(&ident_operation_response_variants_ucc, &Ts2::new(), &{
                        let variants_ts = type_variants_from_request_response_syn_variants
                            .iter()
                            .map(gen_serde_version_of_named_syn_variant);
                        quote! {{
                            #DesirableUcc(#desirable_type_ts),
                            #(#variants_ts),*
                        }}
                    });
                quote! {
                    #AllowClippyArbitrarySourceItemOrdering
                    #ts_c997a274
                }
            };
            let ident_operation_er_ucc = gen_ident_operation_er_ucc(operation);
            let impl_ident_operation_response_variants_ts = {
                let from_handle_ts = gen_from_handle_ts(&ident_operation_er_ucc, &{
                    let variants_ts = type_variants_from_request_response_syn_variants.iter().map(
                        |el_d80f0707| {
                            let variant_ident = &el_d80f0707.ident;
                            let Fields::Named(fields_named) = &el_d80f0707.fields else {
                                panic!("10764d2b");
                            };
                            let fields_mapped_into_ts = {
                                let fields_ts = fields_named.named.iter().map(|field| &field.ident);
                                quote! {#(#fields_ts),*}
                            };
                            let ident_operation_er_with_serde_ucc =
                                gen_ident_operation_er_with_serde_ucc(operation);
                            quote! {
                                #ident_operation_er_with_serde_ucc::#variant_ident {
                                    #fields_mapped_into_ts
                                } => Self::#variant_ident {
                                    #fields_mapped_into_ts
                                }
                            }
                        },
                    );
                    quote! {
                        match #ValueSc.#IntoSerdeVersionSc() {
                            #(#variants_ts),*
                        }
                    }
                });
                quote! {
                    impl #ident_operation_response_variants_ucc {
                        #from_handle_ts
                    }
                }
            };
            let ident_operation_er_ts = {
                let ts_685e0be8 = StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_thiserror_error()
                    .derive_location_lib_location()
                    .build_enum(&ident_operation_er_ucc, &Ts2::new(), &{
                        let variants_ts = type_variants_from_request_response_syn_variants
                            .iter()
                            .map(gen_location_variant_ts);
                        quote! {{#(#variants_ts),*}}
                    });
                quote! {
                    #AllowClippyArbitrarySourceItemOrdering
                    #ts_685e0be8
                }
            };
            quote! {
                #ident_try_operation_logic_response_variants_ts
                #impl_ident_operation_response_variants_ts
                #ident_operation_er_ts
            }
        };
    let gen_ident_operation_payload_ucc = |operation: &Operation| match &operation {
        Operation::CreateOne => quote! {#ident_create_ucc},
        Operation::UpdateOne => quote! {#ident_update_ucc},
        Operation::CreateMany
        | Operation::ReadMany
        | Operation::ReadOne
        | Operation::UpdateMany
        | Operation::DeleteMany
        | Operation::DeleteOne => format!("{ident}{operation}{PayloadUcc}")
            .parse::<Ts2>()
            .expect("c042f504"),
    };
    let gen_ident_operation_parameters_ucc = |operation: &Operation| {
        format!("{ident}{operation}Parameters")
            .parse::<Ts2>()
            .expect("c1203fc6")
    };
    let gen_parameters_pattern_ts = |operation: &Operation, payload_ts: Ts2| -> Ts2 {
        let parameters_ts = {
            let (derive_clone, derive_copy) = operation.derive_clone_and_copy();
            let ts_0d032fce = StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone_if(derive_clone)
                .derive_copy_if(derive_copy)
                .build_struct(
                    &gen_ident_operation_parameters_ucc(operation),
                    &Ts2::new(),
                    &{
                        let ident_operation_payload_ucc =
                            gen_ident_operation_payload_ucc(operation);
                        quote! {{
                            pub #PayloadSc: #ident_operation_payload_ucc,
                        }}
                    },
                );
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_0d032fce
            }
        };
        quote! {
            #payload_ts
            #parameters_ts
        }
    };
    let gen_parameters_payload_and_default_ts =
        |operation: &Operation, declaration_ts: &dyn ToTokens, default_init_ts: &dyn ToTokens| {
            let ident_operation_payload_ucc = gen_ident_operation_payload_ucc(operation);
            let ident_operation_payload_ts = {
                let (derive_clone, derive_copy) = operation.derive_clone_and_copy();
                let ts_ec5b096c = StructOrEnumDeriveTokenStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone_if(derive_clone)
                    .derive_copy_if(derive_copy)
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_utoipa_to_schema()
                    .build_struct(&ident_operation_payload_ucc, &Ts2::new(), &declaration_ts);
                quote! {
                    #AllowClippyArbitrarySourceItemOrdering
                    #ts_ec5b096c
                }
            };
            let impl_pg_crud_default_option_some_vec_one_el_for_operation_payload_ts =
                gen_impl_pg_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                    &ident_operation_payload_ucc,
                    &quote! {Self #default_init_ts},
                );
            quote! {
                #ident_operation_payload_ts
                #impl_pg_crud_default_option_some_vec_one_el_for_operation_payload_ts
            }
        };
    let gen_type_variants_from_request_response_syn_variants = |syn_variants: &Vec<&Variant>,
                                                                operation: &Operation|
     -> Vec<Variant> {
        let mut type_variants_from_request_response_syn_variants = Vec::new();
        for el_21f2d46c in syn_variants {
            type_variants_from_request_response_syn_variants.push((*el_21f2d46c).clone());
        }
        for el_60533068 in
            gen_additional_er_variants(&di, operation.gen_pg_table_attr_additional_er_variants())
        {
            type_variants_from_request_response_syn_variants.push(el_60533068.clone());
        }
        type_variants_from_request_response_syn_variants
    };
    let gen_ident_try_operation_er_ts =
        |operation: &Operation, syn_variants: &Vec<Variant>| -> Ts2 {
            let ts_930e1a93 = StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_thiserror_error()
                .derive_location_lib_location()
                .build_enum(&gen_ident_try_operation_er_ucc(operation), &Ts2::new(), &{
                    let variants = syn_variants
                        .iter()
                        .cloned()
                        .chain(once({
                            let ident_operation_er_with_serde_ucc =
                                gen_ident_operation_er_with_serde_ucc(operation);
                            new_syn_variant_wrapper(
                                &ident_operation_er_with_serde_ucc,
                                None,
                                vec![(
                                    macros_helpers_location_location_field_attr_eo_to_err_string,
                                    &operation.operation_er_with_serde_sc(),
                                    gen_simple_syn_punct(&[
                                        &ident_operation_er_with_serde_ucc.to_string()
                                    ]),
                                )],
                            )
                            .get_syn_variant()
                            .clone()
                        }))
                        .collect::<Vec<Variant>>();
                    let variants_ts = variants.iter().map(gen_location_variant_ts);
                    quote! {{#(#variants_ts),*}}
                });
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_930e1a93
            }
        };
    let std_sync_arc_combination_of_app_state_logic_traits_ts =
        quote! {std::sync::Arc<dyn #PgCrudSc::CombinationOfAppStateLogicTraits>};
    let gen_operation_ts = |operation: &Operation,
                            additional_logic_ts_20466f5c: &dyn ToTokens,
                            parameters_logic_ts: &dyn ToTokens,
                            expected_updated_primary_keys_ts: &dyn ToTokens,
                            query_string_ts: &dyn ToTokens,
                            binded_query_ts: &dyn ToTokens,
                            pg_logic_ts: &dyn ToTokens|
     -> Ts2 {
        let operation_handle_sc_ts = operation.self_handle_sc_ts();
        let operation_sc_ts = operation.self_sc_ts();
        let request_parts_preparation_ts = {
            let header_content_type_application_json_not_found_syn_variant_wrapper_er_init_eprintln_response_creation_ts =
                &gen_operation_er_init_eprintln_response_creation_ts(
                    operation,
                    &header_content_type_application_json_not_found_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            let check_body_size_syn_variant_wrapper_er_init_eprintln_response_creation_ts =
                &gen_operation_er_init_eprintln_response_creation_ts(
                    operation,
                    &check_body_size_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            quote! {
                let (parts, #BodySc) = #RequestSc.into_parts();
                let headers = parts.headers;
                if !matches!(
                    headers.get(http::header::CONTENT_TYPE),
                    Some(value) if value == http::header::HeaderValue::from_static("application/json")
                ) {
                    #header_content_type_application_json_not_found_syn_variant_wrapper_er_init_eprintln_response_creation_ts
                }
                //todo
                // match axum::body::HttpBody::size_hint(&#BodySc).exact() {
                //     Some(value) => {
                //         println!(
                //             "HttpBody::size_hint {value} byte or {} kilobyte or {} megabyte",
                //             value
                //                 .checked_div(1024)
                //                 .expect("111fd01a"),
                //             value
                //                 .checked_div(1_048_576)
                //                 .expect("efbe0db4"), //(1024*1024)
                //         );
                //     }
                //     None => {
                //         println!("HttpBody::size_hint is None");
                //     }
                // }
                let body_bytes = match #PgCrudSc::check_body_size::check_body_size(#BodySc, *#AppStateSc.get_maximum_size_of_http_body_in_bytes()).await {
                    Ok(v_cfac9140) => v_cfac9140,
                    Err(#Er0) => {
                        #check_body_size_syn_variant_wrapper_er_init_eprintln_response_creation_ts
                    }
                };
            }
        };
        let additional_validators_ts = {
            let operation_additional_logic_ts = get_macro_attr_meta_list_ts(
                &di.attrs,
                &operation
                    .gen_pg_table_attr_additional_logic()
                    .gen_path_to_attr(),
            );
            quote! {
                #additional_logic_ts_20466f5c
                #operation_additional_logic_ts
            }
        };
        let acquire_pool_and_connection_ts = {
            let pg_syn_variant_wrapper_er_init_eprintln_response_creation_ts =
                gen_operation_er_init_eprintln_response_creation_ts(
                    operation,
                    &pg_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
            quote! {
                let mut #PoolConnectionSc = match #AppStateSc.get_pg_pool().acquire().await {
                    Ok(v_4535ee48) => v_4535ee48,
                    Err(#Er0) => {
                        #pg_syn_variant_wrapper_er_init_eprintln_response_creation_ts
                    }
                };
                let #ExecutorAcquireSc = match sqlx::Acquire::acquire(&mut #PoolConnectionSc).await {
                    Ok(v_61ae8f84) => v_61ae8f84,
                    Err(#Er0) => {
                        #pg_syn_variant_wrapper_er_init_eprintln_response_creation_ts
                    }
                };
            }
        };
        let wraped_into_axum_response_ts = wrap_into_axum_response_ts(
            &{
                let ident_operation_response_variants_ucc =
                    gen_ident_operation_response_variants_ucc(operation);
                quote! {#ident_operation_response_variants_ucc::#DesirableUcc(#ValueSc)}
            },
            &operation.desirable_status_code().to_http_status_code_ts(),
            &ShouldAddReturn::False,
        );
        quote! {
            #[allow(clippy::single_call_fn)]
            async fn #operation_handle_sc_ts(
                #AppStateSc: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_ts>,
                #RequestSc: axum::extract::Request,
                #TableSc: &str,
            ) -> axum::response::Response {
                #request_parts_preparation_ts
                #additional_validators_ts
                #parameters_logic_ts
                #expected_updated_primary_keys_ts
                let #QueryStringSc = #query_string_ts;
                // println!("{}", #QueryStringSc);
                let #BindedQuerySc = {
                    #binded_query_ts
                };
                #acquire_pool_and_connection_ts
                let #ValueSc = {
                    #pg_logic_ts
                };
                #wraped_into_axum_response_ts
            }
            #[allow(clippy::single_call_fn)]
            pub async fn #operation_sc_ts(
                #AppStateSc: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_ts>,
                #RequestSc: axum::extract::Request,
            ) -> axum::response::Response {
                Self::#operation_handle_sc_ts(#AppStateSc, #RequestSc, #self_table_name_call_ts).await
            }
        }
    };
    let gen_parameters_logic_ts = |operation: &Operation| -> Ts2 {
        let ident_operation_payload_ucc = gen_ident_operation_payload_ucc(operation);
        let serde_json_syn_variant_wrapper_er_init_eprintln_response_creation_ts =
            gen_operation_er_init_eprintln_response_creation_ts(
                operation,
                &serde_json_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
            );
        let ident_operation_parameters_ucc = gen_ident_operation_parameters_ucc(operation);
        //todo in case of large type there is a stackoverflow. for example it was a 3.5md json file gend by create_many_payload_example. 3400 fields = success. 16000 = stackoverflow
        quote! {
            let #ParametersSc = #ident_operation_parameters_ucc {
                #PayloadSc: match serde_json::from_slice::<#ident_operation_payload_ucc>(
                    &#BodyBytesSc,
                ) {
                    Ok(v_9e6fcd2d) => v_9e6fcd2d,
                    Err(#Er0) => {
                        #serde_json_syn_variant_wrapper_er_init_eprintln_response_creation_ts
                    }
                },
            };
        }
    };
    let gen_try_operation_ts =
        |operation: &Operation,
         type_variants_from_request_response_syn_variants: &[Variant],
         result_ok_type_ts: &dyn ToTokens,
         desirable_from_or_try_from_desirable_with_serde_ts: &dyn ToTokens| {
            let try_operation_sc_ts = operation.try_self_sc_ts();
            let try_operation_handle_sc_ts = operation.try_self_handle_sc_ts();
            let ident_try_operation_er_ucc = gen_ident_try_operation_er_ucc(operation);
            let ident_operation_parameters_ucc = gen_ident_operation_parameters_ucc(operation);
            let payload_ts = {
                let serde_json_to_string_syn_variant_init_ts = gen_init_ts(
                    &serde_json_to_string_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
                quote! {
                    let #PayloadSc = {
                        match serde_json::to_string(&#ParametersSc.#PayloadSc) {
                            Ok(v_1772a83e) => v_1772a83e,
                            Err(#Er0) => {
                                return Err(#ident_try_operation_er_ucc::#serde_json_to_string_syn_variant_init_ts);
                            }
                        }
                    };
                }
            };
            let url_ts = {
                let format_handle_ts = dq_ts(&format!(
                    "{{endpoint_location}}/{{table}}/{}",
                    operation.self_sc_str()
                ));
                quote! {let #UrlSc = format!(#format_handle_ts);}
            };
            let future_ts = {
                let operation_http_method_sc_ts =
                    AsRefStrToScTs::case_or_panic(&operation.http_method());
                let commit_header_addition_ts = quote! {
                    .header(
                        &"commit".to_owned(),
                        git_info::PROJECT_GIT_INFO.commit,
                    )
                };
                let application_json_dq_ts = dq_ts(&"application/json");
                let content_type_application_json_header_addition_ts = quote! {
                    .header(reqwest::header::CONTENT_TYPE, #application_json_dq_ts)
                };
                quote! {
                    let #FutureSc = reqwest::Client::new()
                        .#operation_http_method_sc_ts(&#UrlSc)
                        #commit_header_addition_ts
                        #content_type_application_json_header_addition_ts
                        .#BodySc(#PayloadSc)
                        .send();
                }
            };
            let response_ts = {
                let reqwest_syn_variant_init_ts =
                    gen_init_ts(&reqwest_syn_variant_wrapper, file!(), line!(), column!());
                quote! {
                    let #ResponseSc = match #FutureSc.await {
                        Ok(v_180559e9) => v_180559e9,
                        Err(#Er0) => {
                            return Err(#ident_try_operation_er_ucc::#reqwest_syn_variant_init_ts);
                        }
                    };
                }
            };
            let er_0_response_status_ts = quote! {
                let #Er0 = #ResponseSc.status();
            };
            let headers_ts = quote! {
                let #Er1 = #ResponseSc.headers().clone();
            };
            let response_text_ts = {
                let failed_to_get_response_text_syn_variant_init_ts = gen_init_ts(
                    &failed_to_get_response_text_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
                quote! {
                    let #Er2 = match #ResponseSc.text().await {
                        Ok(v_6a62b2b9) => v_6a62b2b9,
                        Err(#Er2) => {
                            return Err(#ident_try_operation_er_ucc::#failed_to_get_response_text_syn_variant_init_ts);
                        }
                    };
                }
            };
            let ident_operation_response_variants_ucc =
                gen_ident_operation_response_variants_ucc(operation);
            let expected_response_ts = {
                let deserialize_response_syn_variant_init_ts = gen_init_ts(
                    &deserialize_response_syn_variant_wrapper,
                    file!(),
                    line!(),
                    column!(),
                );
                quote! {
                    let #ExpectedResponseSc = match serde_json::from_str::<#ident_operation_response_variants_ucc>(&#Er2) {
                        Ok(v_563d2a75) => v_563d2a75,
                        Err(#Er3) => {
                            return Err(#ident_try_operation_er_ucc::#deserialize_response_syn_variant_init_ts);
                        }
                    };
                }
            };
            let try_operation_logic_er_with_serde_ucc =
                gen_ident_operation_er_with_serde_ucc(operation);
            let operation_er_with_serde_sc = &operation.operation_er_with_serde_sc();
            let try_operation_logic_er_with_serde_ts = {
                let try_operation_logic_response_variants_to_try_operation_logic_er_with_serde = type_variants_from_request_response_syn_variants.iter().map(|el_f83d5272| {
                let variant_ident = &el_f83d5272.ident;
                let fields_idents_ts = if let Fields::Named(fields_named) = &el_f83d5272.fields {
                    let fields_idents = fields_named.named.iter().map(|field| &field.ident);
                    quote! {#(#fields_idents),*}
                } else {
                    panic!("8dcafc1c");
                };
                quote! {
                    #ident_operation_response_variants_ucc::#variant_ident {
                        #fields_idents_ts
                    } => #try_operation_logic_er_with_serde_ucc::#variant_ident { #fields_idents_ts }
                }
            });
                quote! {
                    let #operation_er_with_serde_sc = match #ExpectedResponseSc {
                        #ident_operation_response_variants_ucc::#DesirableUcc(#ValueSc) => {
                            return Ok(#desirable_from_or_try_from_desirable_with_serde_ts);
                        },
                        #(#try_operation_logic_response_variants_to_try_operation_logic_er_with_serde),*
                    };
                }
            };
            let return_er_ts = {
                let field_loc_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_ts =
                    gen_field_loc_new_ts(file!(), line!(), column!());
                quote! {
                    Err(#ident_try_operation_er_ucc::#try_operation_logic_er_with_serde_ucc {
                        #operation_er_with_serde_sc,
                        #field_loc_new_6ac7b78e_da5d_4274_b58c_67bb9625d008_ts,
                    })
                }
            };
            quote! {
                #[allow(clippy::single_call_fn)]
                async fn #try_operation_handle_sc_ts(
                    #EndpointLocationSc: #RefStr,
                    #ParametersSc: #ident_operation_parameters_ucc,
                    #TableSc: &str,
                ) -> Result<#result_ok_type_ts, #ident_try_operation_er_ucc> {
                    #payload_ts
                    #url_ts
                    #future_ts
                    #response_ts
                    #er_0_response_status_ts
                    #headers_ts
                    #response_text_ts
                    #expected_response_ts
                    #try_operation_logic_er_with_serde_ts
                    #return_er_ts
                }
                pub async fn #try_operation_sc_ts(
                    #EndpointLocationSc: #RefStr,
                    #ParametersSc: #ident_operation_parameters_ucc
                ) -> Result<#result_ok_type_ts, #ident_try_operation_er_ucc> {
                    Self::#try_operation_handle_sc_ts(
                        #EndpointLocationSc,
                        #ParametersSc,
                        #self_table_name_call_ts
                    ).await
                }
            }
        };
    let gen_match_ident_read_only_ids_as_from_row_from_row_ts = |ts: &dyn ToTokens| {
        quote! {
            match <#ident_read_only_ids_ucc as sqlx::FromRow<'_, sqlx::postgres::PgRow>>::from_row(&v_b27d7d79) {
                Ok(v_33759463) => v_33759463,
                Err(#Er0) => #ts
            }
        }
    };
    let gen_create_update_delete_many_fetch_ts =
        |create_or_update_or_delete_many: &CreateOrUpdateOrDeleteMany| {
            let operation_d1960edc = Operation::from(create_or_update_or_delete_many);
            gen_fetch_ts(
                &ExecutorSc,
                &match &create_or_update_or_delete_many {
                    CreateOrUpdateOrDeleteMany::Create | CreateOrUpdateOrDeleteMany::Update => {
                        let ts = gen_match_ident_read_only_ids_as_from_row_from_row_ts(&{
                            let ts = gen_drop_rows_match_pg_transaction_rollback_await_handle_ts(
                                &operation_d1960edc,
                                file!(),
                                line!(),
                                column!(),
                                file!(),
                                line!(),
                                column!(),
                            );
                            quote! {{#ts}}
                        });
                        quote! {Some(#ts)}
                    }
                    CreateOrUpdateOrDeleteMany::Delete => gen_sqlx_row_try_get_primary_key_ts(
                        &primary_key_field_type_as_pg_type_read_ucc,
                        &quote! {Some(v_69ecb6a9)},
                        &gen_drop_rows_match_pg_transaction_rollback_await_handle_ts(
                            &operation_d1960edc,
                            file!(),
                            line!(),
                            column!(),
                            file!(),
                            line!(),
                            column!(),
                        ),
                    ),
                },
                &gen_drop_rows_match_pg_transaction_rollback_await_handle_ts(
                    &operation_d1960edc,
                    file!(),
                    line!(),
                    column!(),
                    file!(),
                    line!(),
                    column!(),
                ),
                &ShouldWrapIntoValue::True,
            )
        };
    let gen_create_update_delete_one_fetch_ts =
        |create_or_update_or_delete_one: &CreateOrUpdateOrDeleteOne| {
            let operation_c85ff6d4 = Operation::from(create_or_update_or_delete_one);
            wrap_into_value_ts(&gen_fetch_one_ts(
                &ExecutorSc,
                &gen_sqlx_row_try_get_primary_key_ts(
                    &quote! {#primary_key_field_type_as_pg_type_read_ucc},
                    &quote! {v_69ecb6a9},
                    &gen_match_pg_transaction_rollback_await_ts(
                        &operation_c85ff6d4,
                        file!(),
                        line!(),
                        column!(),
                        file!(),
                        line!(),
                        column!(),
                    ),
                ),
                &gen_match_pg_transaction_rollback_await_ts(
                    &operation_c85ff6d4,
                    file!(),
                    line!(),
                    column!(),
                    file!(),
                    line!(),
                    column!(),
                ),
            ))
        };
    let gen_operation_payload_example_ts = |operation: &Operation| {
        let operation_payload_example_sc = operation.operation_payload_example_sc();
        let wraped_into_axum_response_ts = wrap_into_axum_response_ts(
            &{
                let ident_operation_payload_ucc = gen_ident_operation_payload_ucc(operation);
                quote! {<#ident_operation_payload_ucc as pg_crud::#DefaultOptionSomeVecOneElUcc>::#DefaultOptionSomeVecOneElSc()}
            },
            &quote! {http::StatusCode::OK},
            &ShouldAddReturn::False,
        );
        quote! {
            #MustUse
            pub fn #operation_payload_example_sc() -> axum::response::Response {
                #wraped_into_axum_response_ts
            }
        }
    };
    let increment_init_ts = quote! {let mut #IncrementSc: u64 = 0;};
    let column_names = {
        let mut value = fields
            .iter()
            .fold(String::default(), |mut acc_b2600a1f, element| {
                assert!(
                    write!(acc_b2600a1f, "{}", &element.field_ident).is_ok(),
                    "b9fe50dc"
                );
                acc_b2600a1f.push(',');
                acc_b2600a1f
            });
        let _: Option<char> = value.pop();
        value
    };
    let column_names_dq_ts = dq_ts(&column_names);
    let gen_select_only_ids_query_part_ts = |operation: &Operation| {
        let select_only_ids_query_part_init_ts = fields.iter().map(|element: &SynFieldWrapper| {
            let field_ident = &element.field_ident;
            let field_ident_dq_ts = dq_ts(&field_ident);
            let field_type_as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_ts(&element.field_type);
            let ts_00878df8 = gen_operation_er_init_eprintln_response_creation_ts(operation, &query_part_syn_variant_wrapper, file!(), line!(), column!());
            quote! {
                match #field_type_as_pg_crud_pg_type_pg_type_ts #SelectOnlyIdsQueryPartSc(#field_ident_dq_ts) {
                    Ok(v_aa341baf) => {
                        acc_a35168d8.push_str(&v_aa341baf);
                    },
                    Err(#Er0) => {
                        #ts_00878df8
                    }
                }
            }
        });
        quote! {
            {
                let mut acc_a35168d8 = #StringTs::new();
                #(#select_only_ids_query_part_init_ts)*
                let _: Option<char> = acc_a35168d8.pop();
                acc_a35168d8
            }
        }
    };
    let gen_write_into_buffer_query_part_syn_variant_er_init_eprintln_response_creation_ts =
        |operation: &Operation| {
            let query_part_er_write_into_buffer_ts =
                gen_query_part_er_write_into_buffer_ts(import_path);
            let ts_fa8795ea = gen_operation_er_init_eprintln_response_creation_ts(
                operation,
                &query_part_syn_variant_wrapper,
                file!(),
                line!(),
                column!(),
            );
            quote! {
                let #Er0 = #query_part_er_write_into_buffer_ts;
                #ts_fa8795ea
            }
        };
    let create_many_ts = {
        let operation = Operation::CreateMany;
        let type_variants_from_request_response_syn_variants =
            gen_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = gen_parameters_pattern_ts(
            &operation,
            gen_parameters_payload_and_default_ts(
                &operation,
                &{
                    let vec_ident_create_ts = gen_vec_tokens_declaration_ts(&ident_create_ucc);
                    quote! {(pub #vec_ident_create_ts);}
                },
                &quote! {(vec![#PgCrudDefaultOptionSomeVecOneElCall])},
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts =
                gen_ident_try_operation_logic_response_variants_ident_operation_er_convert_ts(
                    &operation,
                    &vec_ident_read_only_ids_ts,
                    &type_variants_from_request_response_syn_variants,
                );
            {
                let parameters_logic_ts = gen_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let if_write_is_err_ts = gen_if_write_is_err_ts(
                        &quote! {
                            acc_8a58994e,
                            "({v_f4fdd10d}),"
                        },
                        &gen_write_into_buffer_query_part_syn_variant_er_init_eprintln_response_creation_ts(
                            &operation
                        )
                    );
                    let ts_4b2a4911 = gen_operation_er_init_eprintln_response_creation_ts(
                        &operation,
                        &query_part_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                    let select_only_ids_query_part_ts =
                        gen_select_only_ids_query_part_ts(&operation);
                    quote! {#PgCrudSc::gen_create_many_query_string(
                        #TableSc,
                        #column_names_dq_ts,
                        &{
                            #increment_init_ts
                            let mut acc_8a58994e = #StringTs::default();
                            for el_1651705d in &#ParametersSc.#PayloadSc.0 {
                                match el_1651705d.#CreateQueryPartSc(&mut #IncrementSc) {
                                    Ok(v_f4fdd10d) => {
                                        #if_write_is_err_ts
                                    },
                                    Err(#Er0) => {
                                        #ts_4b2a4911
                                    }
                                }
                            }
                            let _: Option<char> = acc_8a58994e.pop();
                            acc_8a58994e
                        },
                        &#select_only_ids_query_part_ts
                    )}
                };
                let binded_query_ts = {
                    let pg_syn_variant_er_init_eprintln_response_creation_ts =
                        gen_operation_er_init_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    quote! {
                        let mut #QuerySc = sqlx::query::<sqlx::Postgres>(&#QueryStringSc);
                        for el_7f862135 in #ParametersSc.#PayloadSc.0 {
                            match el_7f862135.#CreateQueryBindSc(#QuerySc) {
                                Ok(v_011a3eb4) => {
                                    #QuerySc = v_011a3eb4;
                                },
                                Err(#Er0) => {
                                    #pg_syn_variant_er_init_eprintln_response_creation_ts
                                }
                            }
                        }
                        #QuerySc
                    }
                };
                let pg_logic_ts = wrap_content_into_pg_transaction_begin_commit_value_ts(
                    &operation,
                    &gen_create_update_delete_many_fetch_ts(&CreateOrUpdateOrDeleteMany::Create),
                );
                impl_ident_vec_ts.push(gen_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &Ts2::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &pg_logic_ts,
                ));
            };
            quote! {
                #try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts
            }
        };
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_request_syn_variants);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &vec_ident_read_only_ids_ts,
                &ValueSc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        impl_ident_vec_ts.push(gen_operation_payload_example_ts(&operation));
        quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.create_many_content_write_into_gen_pg_table_create_many,
        "gen_pg_table_create_many",
        &create_many_ts,
        &FormatWithCargofmt::True,
    );
    let create_one_ts = {
        let operation = Operation::CreateOne;
        let type_variants_from_request_response_syn_variants =
            gen_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = gen_parameters_pattern_ts(&operation, Ts2::new());
        let operation_ts = {
            let try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts =
                gen_ident_try_operation_logic_response_variants_ident_operation_er_convert_ts(
                    &operation,
                    &ident_read_only_ids_ucc,
                    &type_variants_from_request_response_syn_variants,
                );
            {
                let parameters_logic_ts = gen_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let ts_cfcf1c2a = gen_operation_er_init_eprintln_response_creation_ts(
                        &operation,
                        &query_part_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                    let select_only_ids_query_part_ts =
                        gen_select_only_ids_query_part_ts(&operation);
                    quote! {
                        #PgCrudSc::gen_create_one_query_string(
                            #TableSc,
                            #column_names_dq_ts,
                            &match #ParametersSc.#PayloadSc.#CreateQueryPartSc(&mut 0) {
                                Ok(v_3267d57d) => v_3267d57d,
                                Err(#Er0) => {
                                    #ts_cfcf1c2a
                                }
                            },
                            &#select_only_ids_query_part_ts
                        )
                    }
                };
                let binded_query_ts = {
                    let pg_syn_variant_er_init_eprintln_response_creation_ts =
                        gen_operation_er_init_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    quote! {
                        let mut #QuerySc = #sqlx_query_sqlx_pg_ts(&#QueryStringSc);
                        match #ParametersSc.#PayloadSc.#CreateQueryBindSc(#QuerySc) {
                            Ok(v_06f852cd) => {
                                #QuerySc = v_06f852cd;
                            },
                            Err(#Er0) => {
                                #pg_syn_variant_er_init_eprintln_response_creation_ts
                            }
                        }
                        #QuerySc
                    }
                };
                let pg_logic_ts = wrap_content_into_pg_transaction_begin_commit_value_ts(
                    &operation,
                    // &gen_create_update_delete_one_fetch_ts(&CreateOrUpdateOrDeleteOne::Create)
                    &{
                        let operation_34462e2a =
                            Operation::from(&CreateOrUpdateOrDeleteOne::Create);
                        wrap_into_value_ts(&gen_fetch_one_ts(
                            &ExecutorSc,
                            &gen_match_ident_read_only_ids_as_from_row_from_row_ts(&{
                                let ts = gen_match_pg_transaction_rollback_await_ts(
                                    &operation_34462e2a,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                );
                                quote! {{#ts}}
                            }),
                            &gen_match_pg_transaction_rollback_await_ts(
                                &operation_34462e2a,
                                file!(),
                                line!(),
                                column!(),
                                file!(),
                                line!(),
                                column!(),
                            ),
                        ))
                    },
                );
                impl_ident_vec_ts.push(gen_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &Ts2::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &pg_logic_ts,
                ));
            };
            quote! {
                #try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts
            }
        };
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_request_syn_variants);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &ident_read_only_ids_ucc,
                &ValueSc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        impl_ident_vec_ts.push(gen_operation_payload_example_ts(&operation));
        quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.create_one_content_write_into_gen_pg_table_create_one,
        "gen_pg_table_create_one",
        &create_one_ts,
        &FormatWithCargofmt::True,
    );
    let read_many_ts = {
        let operation = Operation::ReadMany;
        let type_variants_from_request_response_syn_variants =
            gen_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(not_unique_field_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = gen_parameters_pattern_ts(
            &operation,
            gen_parameters_payload_and_default_ts(
                &operation,
                &quote! {{
                    #pub_where_many_option_ident_where_many_ts,
                    #pub_select_pg_crud_not_empty_unique_vec_ident_select_ts,
                    pub #OrderBySc: #pg_crud_order_by_ts<#ident_select_ucc>,
                    pub #PaginationSc: pg_crud::PaginationStartsWithZero,
                }},
                &quote! {{
                    #where_many_pg_crud_default_option_some_vec_one_el_call_ts,
                    #select_pg_crud_default_option_some_vec_one_el_call_ts,
                    #OrderBySc: pg_crud::OrderBy {
                        #ColumnSc: #ident_select_ucc::#primary_key_field_ident_ucc_ts(
                            #PgCrudDefaultOptionSomeVecOneElCall
                        ),
                        #OrderSc: Some(
                            #PgCrudDefaultOptionSomeVecOneElCall
                        ),
                    },
                    #PaginationSc: #PgCrudDefaultOptionSomeVecOneElCall,
                }},
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts =
                gen_ident_try_operation_logic_response_variants_ident_operation_er_convert_ts(
                    &operation,
                    &vec_struct_options_ident_ts,
                    &type_variants_from_request_response_syn_variants,
                );
            {
                let parameters_logic_ts = gen_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let select_query_part_parameters_payload_select_ts =
                        gen_select_query_part_parameters_payload_select_ts(&operation);
                    let additional_paramaters_init_ts =
                        gen_read_or_delete_many_additional_paramaters_init_ts(
                            &ReadManyOrDeleteMany::ReadMany,
                        );
                    let additional_parameters_order_by_handle_ts =
                        dq_ts(&format!("{{}}{OrderSc} {BySc} {{}} {{}}"));
                    let ts_0ec756e2 = gen_operation_er_init_eprintln_response_creation_ts(
                        &operation,
                        &query_part_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                    let order_by_column_match_ts =
                        gen_fields_named_with_comma_ts(&|element: &SynFieldWrapper| {
                            let field_ident_ucc =
                                ToTokensToUccTs::case_or_panic(&element.field_ident);
                            let field_ident_dq_ts = dq_ts(&element.field_ident);
                            quote! {
                                #ident_select_ucc::#field_ident_ucc(_) => #field_ident_dq_ts
                            }
                        });
                    let if_write_is_err_curly_braces_0_ts = gen_if_write_is_err_curly_braces_ts(
                        &quote! {
                            #AdditionalParametersSc,
                            #additional_parameters_order_by_handle_ts,
                            #PrefixSc,
                            &match &#ParametersSc.#PayloadSc.#OrderBySc.#ColumnSc {
                                #order_by_column_match_ts
                            },
                            #ParametersSc.#PayloadSc.#OrderBySc.#OrderSc.as_ref().map_or_else(
                                || pg_crud::Order::default().to_sc_str(),
                                #import_path::Order::to_sc_str
                            )
                        },
                        &gen_write_into_buffer_query_part_syn_variant_er_init_eprintln_response_creation_ts(&operation),
                    );
                    let if_write_is_err_curly_braces_1_ts = gen_if_write_is_err_curly_braces_ts(
                        &quote! {
                            #AdditionalParametersSc,
                            "{prefix}{}",
                            match #pg_crud_pg_type_where_filter_query_part_ts(
                                &#ParametersSc.#PayloadSc.pagination,
                                &mut #IncrementSc,
                                &"",
                                bool::default()
                            ) {
                                Ok(v_742be6cf) => v_742be6cf,
                                Err(#Er0) => {
                                    #ts_0ec756e2
                                },
                            }
                        },
                        &gen_write_into_buffer_query_part_syn_variant_er_init_eprintln_response_creation_ts(&operation)
                    );
                    quote! {#PgCrudSc::gen_read_many_query_string(
                        #TableSc,
                        &#select_query_part_parameters_payload_select_ts,
                        &{
                            #increment_init_ts
                            let mut #AdditionalParametersSc = #additional_paramaters_init_ts;
                            let #PrefixSc = if additional_parameters.is_empty() {""} else {" "};
                            #if_write_is_err_curly_braces_0_ts
                            #if_write_is_err_curly_braces_1_ts
                            #AdditionalParametersSc
                        }
                    )}
                };
                let binded_query_ts = {
                    let query_pg_type_where_filter_query_bind_parameters_payload_where_many_query_ts = gen_query_pg_type_where_filter_query_bind_parameters_payload_where_many_query_ts(&operation);
                    let pg_syn_variant_er_init_eprintln_response_creation_ts =
                        gen_operation_er_init_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    quote! {
                        let mut #QuerySc = #sqlx_query_sqlx_pg_ts(&#QueryStringSc);
                        #query_pg_type_where_filter_query_bind_parameters_payload_where_many_query_ts
                        match #pg_crud_pg_type_where_filter_query_bind_ts(
                            #ParametersSc.#PayloadSc.pagination,
                            #QuerySc,
                        ) {
                            Ok(v_9f7e487b) => {
                                #QuerySc = v_9f7e487b;
                            },
                            Err(#Er0) => {
                                #pg_syn_variant_er_init_eprintln_response_creation_ts
                            }
                        }
                        #QuerySc
                    }
                };
                let pg_logic_ts = {
                    let fetch_ts = gen_fetch_ts(
                        &ExecutorAcquireSc,
                        &{
                            let match_ident_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts = gen_match_ident_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts(&ReadManyOrReadOne::ReadMany);
                            quote! {Some(#match_ident_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts)}
                        },
                        &gen_operation_er_init_eprintln_response_creation_ts(
                            &operation,
                            &pg_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        ),
                        &ShouldWrapIntoValue::False,
                    );
                    quote! {{
                        #fetch_ts
                    }}
                };
                impl_ident_vec_ts.push(gen_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &Ts2::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &pg_logic_ts,
                ));
            };
            quote! {
                #try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts
            }
        };
        let try_operation_ts = {
            let try_operation_er_ts = gen_ident_try_operation_er_ts(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(
                    not_unique_field_syn_variant_wrapper
                        .get_syn_variant()
                        .clone(),
                );
                value
            });
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &vec_struct_options_ident_ts,
                &quote! {
                    #ValueSc
                    .into_iter()
                    .fold(Vec::new(), |mut acc_4adf5a80, el_6a197212| {
                        acc_4adf5a80.push(el_6a197212);
                        acc_4adf5a80
                    })
                },
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        impl_ident_vec_ts.push(gen_operation_payload_example_ts(&operation));
        quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.read_many_content_write_into_gen_pg_table_read_many,
        "gen_pg_table_read_many",
        &read_many_ts,
        &FormatWithCargofmt::True,
    );
    let read_one_ts = {
        let operation = Operation::ReadOne;
        let type_variants_from_request_response_syn_variants =
            gen_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(not_unique_field_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = gen_parameters_pattern_ts(
            &operation,
            gen_parameters_payload_and_default_ts(
                &operation,
                &{
                    let pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts =
                        gen_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts(
                            &SelfReadUcc::from_type_last_segment(primary_key_field_type),
                        );
                    quote! {{
                        #pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts,
                        #pub_select_pg_crud_not_empty_unique_vec_ident_select_ts,
                    }}
                },
                &quote! {{
                    #primary_key_field_ident: #PgCrudDefaultOptionSomeVecOneElCall,
                    #select_pg_crud_default_option_some_vec_one_el_call_ts
                }},
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts =
                gen_ident_try_operation_logic_response_variants_ident_operation_er_convert_ts(
                    &operation,
                    &ident_read_ucc,
                    &type_variants_from_request_response_syn_variants,
                );
            {
                let parameters_logic_ts = gen_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let select_query_part_parameters_payload_select_ts =
                        gen_select_query_part_parameters_payload_select_ts(&operation);
                    let ts_1ead7cf9 = gen_operation_er_init_eprintln_response_creation_ts(
                        &operation,
                        &query_part_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                    quote! {#PgCrudSc::gen_read_one_query_string(
                        #TableSc,
                        &#select_query_part_parameters_payload_select_ts,
                        &match #pg_crud_pg_type_where_filter_query_part_ts(
                            &#ParametersSc.#PayloadSc.#primary_key_field_ident,
                            &mut 0,
                            &Self::#PrimaryKeySc(),
                            false
                        ) {
                            Ok(v_be9e7b7d) => v_be9e7b7d,
                            Err(#Er0) => {
                                #ts_1ead7cf9
                            }
                        }
                    )}
                };
                let binded_query_ts = {
                    let binded_query_modifications_ts = {
                        let pg_syn_variant_er_init_eprintln_response_creation_ts =
                            gen_operation_er_init_eprintln_response_creation_ts(
                                &operation,
                                &try_bind_syn_variant_wrapper,
                                file!(),
                                line!(),
                                column!(),
                            );
                        quote! {
                            match #pg_crud_pg_type_where_filter_query_bind_ts(#ParametersSc.#PayloadSc.#primary_key_field_ident, #QuerySc) {
                                Ok(v_80ee6983) => {
                                    #QuerySc = v_80ee6983;
                                },
                                Err(#Er0) => {
                                    #pg_syn_variant_er_init_eprintln_response_creation_ts
                                }
                            }
                        }
                    };
                    quote! {
                        let mut #QuerySc = #sqlx_query_sqlx_pg_ts(&#QueryStringSc);
                        #binded_query_modifications_ts
                        #QuerySc
                    }
                };
                let pg_logic_ts = gen_fetch_one_ts(
                    &ExecutorAcquireSc,
                    &gen_match_ident_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts(&ReadManyOrReadOne::ReadOne),
                    &gen_operation_er_init_eprintln_response_creation_ts(&operation, &pg_syn_variant_wrapper, file!(), line!(), column!()),
                );
                impl_ident_vec_ts.push(gen_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &Ts2::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &pg_logic_ts,
                ));
            };
            quote! {
                #try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts
            }
        };
        let try_operation_ts = {
            let try_operation_er_ts = gen_ident_try_operation_er_ts(&operation, &{
                let mut value = common_http_request_syn_variants.clone();
                value.push(
                    not_unique_field_syn_variant_wrapper
                        .get_syn_variant()
                        .clone(),
                );
                value
            });
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &ident_read_ucc,
                &ValueSc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        impl_ident_vec_ts.push(gen_operation_payload_example_ts(&operation));
        quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.read_one_content_write_into_gen_pg_table_read_one,
        "gen_pg_table_read_one",
        &read_one_ts,
        &FormatWithCargofmt::True,
    );
    //todo update not only with array of objects with ids but with WHERE and one object
    let update_many_ts = {
        let operation = Operation::UpdateMany;
        let type_variants_from_request_response_syn_variants =
            gen_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = gen_parameters_pattern_ts(&operation, {
            let ident_operation_payload_ucc = gen_ident_operation_payload_ucc(&operation);
            let vec_ident_update_ts = gen_vec_tokens_declaration_ts(&ident_update_ucc);
            let ident_operation_payload_vec_ts = StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_serde_serialize()
                .derive_utoipa_to_schema()
                .build_struct(
                    &ident_operation_payload_ucc,
                    &Ts2::new(),
                    &quote! {(#vec_ident_update_ts);},
                );
            let ident_operation_payload_try_new_er_ucc =
                format!("{ident}{operation}PayloadTryNewEr")
                    .parse::<Ts2>()
                    .expect("3da248bb");
            let ident_operation_payload_try_new_er_ts = StructOrEnumDeriveTokenStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_thiserror_error()
                .derive_location_lib_location()
                .build_enum(
                    &ident_operation_payload_try_new_er_ucc,
                    &Ts2::new(),
                    &quote! {{
                        #NotUniquePrimaryKeyUcc {
                            #[eo_to_err_string]
                            #NotUniquePrimaryKeySc: #primary_key_field_type_update_ts,
                            #[eo_to_err_string]
                            loc: location_lib::loc::Loc,
                        }
                    }},
                );
            let impl_pub_try_new_for_ident_operation_payload_ts = gen_impl_pub_try_new_for_ident_ts(
                &gen_ident_operation_payload_ucc(&operation),
                &quote! {#ValueSc: #vec_ident_update_ts},
                &ident_operation_payload_try_new_er_ucc,
                &quote! {
                    let mut acc_6bf275fc = Vec::new();
                    for el_35facc3a in &#ValueSc {
                        if acc_6bf275fc.contains(&&el_35facc3a.#primary_key_field_ident) {
                            return Err(#ident_operation_payload_try_new_er_ucc::#NotUniquePrimaryKeyUcc {
                                #NotUniquePrimaryKeySc: el_35facc3a.#primary_key_field_ident,
                                loc: location_lib::loc!(),
                            });
                        }
                        acc_6bf275fc.push(&el_35facc3a.#primary_key_field_ident);
                    }
                    Ok(Self(#ValueSc))
                },
            );
            let impl_serde_deserialize_for_ident_update_many_payload_ts = {
                let tuple_struct_ident_operation_payload_dq_ts =
                    dq_ts(&format!("tuple struct {ident_operation_payload_ucc}"));
                let tuple_struct_ident_operation_payload_with_1_el_dq_ts = dq_ts(&format!(
                    "tuple struct {ident_operation_payload_ucc} with 1 element"
                ));
                let match_ident_update_many_payload_try_new_field0_ts =
                    gen_match_try_new_in_deserialize_ts(
                        &ident_operation_payload_ucc,
                        &quote! {__field0},
                    );
                let ident_operation_payload_dq_ts = dq_ts(&ident_operation_payload_ucc);
                quote! {
                    #[allow(unused_qualifications)]
                    #[allow(clippy::absolute_paths)]
                    #AllowClippyArbitrarySourceItemOrdering
                    const _: () = {
                        #[allow(unused_extern_crates, clippy::useless_attribute, clippy::arbitrary_source_item_ordering)]
                        extern crate serde as _serde;
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for #ident_operation_payload_ucc {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private228::PhantomData<#ident_operation_payload_ucc>,
                                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = #ident_operation_payload_ucc;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private228::Formatter<'_>,
                                    ) -> _serde::__private228::fmt::Result {
                                        _serde::__private228::Formatter::write_str(
                                            __formatter,
                                            #tuple_struct_ident_operation_payload_dq_ts,
                                        )
                                    }
                                    #[inline]
                                    fn visit_newtype_struct<__E>(
                                        self,
                                        __e: __E,
                                    ) -> Result<Self::Value, __E::Error>
                                    where
                                        __E: _serde::Deserializer<'de>,
                                    {
                                        let __field0: #vec_ident_update_ts = <#vec_ident_update_ts as _serde::Deserialize>::deserialize(__e)?;
                                        #match_ident_update_many_payload_try_new_field0_ts
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let Some(__field0) = _serde::de::SeqAccess::next_element::<#vec_ident_update_ts>(&mut __seq)? else {
                                            return Err(_serde::de::Error::invalid_length(0usize, &#tuple_struct_ident_operation_payload_with_1_el_dq_ts));
                                        };
                                        #match_ident_update_many_payload_try_new_field0_ts
                                    }
                                }
                                _serde::Deserializer::deserialize_newtype_struct(
                                    __deserializer,
                                    #ident_operation_payload_dq_ts,
                                    __Visitor {
                                        marker: _serde::__private228::PhantomData::<Self>,
                                        lifetime: _serde::__private228::PhantomData,
                                    },
                                )
                            }
                        }
                    };
                }
            };
            let impl_pg_crud_default_option_some_vec_one_el_for_operation_payload_ts =
                gen_impl_pg_crud_default_option_some_vec_one_el_for_tokens_no_lifetime_ts(
                    &ident_operation_payload_ucc,
                    &quote! {
                        Self(vec![#PgCrudDefaultOptionSomeVecOneElCall])
                    },
                );
            quote! {
                #ident_operation_payload_vec_ts
                #ident_operation_payload_try_new_er_ts
                #impl_pub_try_new_for_ident_operation_payload_ts
                #impl_serde_deserialize_for_ident_update_many_payload_ts
                #impl_pg_crud_default_option_some_vec_one_el_for_operation_payload_ts
            }
        });
        let operation_ts = {
            let try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts =
                gen_ident_try_operation_logic_response_variants_ident_operation_er_convert_ts(
                    &operation,
                    &vec_ident_read_only_ids_ts,
                    &type_variants_from_request_response_syn_variants,
                );
            {
                let parameters_logic_ts = {
                    let parameters_logic_ts = gen_parameters_logic_ts(&operation);
                    quote! {
                        #parameters_logic_ts
                        let #UpdateForQueryVecSc = #ParametersSc.#PayloadSc.0.into_iter()
                        .map(#ident_update_for_query_ucc::#FromHandleSc)
                        .collect::<Vec<#ident_update_for_query_ucc>>();
                    }
                };
                let query_string_ts = {
                    let ts_1b64e228 = gen_operation_er_init_eprintln_response_creation_ts(
                        &operation,
                        &query_part_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                    let gen_match_update_query_part_primary_key_operation_ts =
                        |ts: &dyn ToTokens| {
                            gen_match_update_query_part_primary_key_ts(&operation, &ts)
                        };
                    let fields_named_without_primary_key_update_assignment_ts =
                        gen_fields_named_without_primary_key_without_comma_ts(
                            &|element: &SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let field_ident_dq_ts = dq_ts(&field_ident);
                                let is_field_ident_update_exists_sc =
                                    IsSelfUpdateExistSc::from_tokens(&field_ident);
                                let update_query_part_field_ident_sc =
                                    UpdateQueryPartSelfSc::from_tokens(&field_ident);
                                let gen_when_column_id_then_value_update_many_query_part_sc =
                                    GenWhenColumnIdThenValueUpdateManyQueryPartSc;
                                quote! {
                                    {
                                        let mut #is_field_ident_update_exists_sc = false;
                                        for el_a72f3eac in &#UpdateForQueryVecSc {
                                            if el_a72f3eac.#field_ident.is_some() {
                                                #is_field_ident_update_exists_sc = true;
                                                break;
                                            }
                                        }
                                        if #is_field_ident_update_exists_sc {
                                            acc_b86a253a.push_str(&
                                                pg_crud::gen_column_equals_case_acc_else_column_end_comma_update_many_query_part(
                                                    #field_ident_dq_ts,
                                                    &{
                                                        let mut acc_8ad06c8c = #StringTs::default();
                                                        for el_defbc401 in &#UpdateForQueryVecSc {
                                                            if let Some(v_3ea04126) = &el_defbc401.#field_ident {
                                                                acc_8ad06c8c.push_str(&#PgCrudSc::#gen_when_column_id_then_value_update_many_query_part_sc(
                                                                    Self::#PrimaryKeySc(),
                                                                    &match el_defbc401.#UpdateQueryPartPrimaryKeySc(&mut #IncrementSc) {
                                                                        Ok(v_00890100) => v_00890100,
                                                                        Err(#Er0) => {
                                                                            #ts_1b64e228
                                                                        }
                                                                    },
                                                                    &match #ident_update_for_query_ucc::#update_query_part_field_ident_sc(v_3ea04126, &mut #IncrementSc) {
                                                                        Ok(v_8797585c) => v_8797585c,
                                                                        Err(#Er0) => {
                                                                            #ts_1b64e228
                                                                        }
                                                                    },
                                                                ));
                                                            }
                                                        }
                                                        acc_8ad06c8c
                                                    }
                                                )
                                            );
                                        }
                                    }
                                }
                            },
                        );
                    let if_write_is_err_ts = gen_if_write_is_err_ts(
                        &{
                            let match_update_query_part_primary_key_operation_ts = gen_match_update_query_part_primary_key_operation_ts(
                                &quote!{el_9b2b56f8}
                            );
                            quote! {
                                acc_a95eb175,
                                "{},",
                                #match_update_query_part_primary_key_operation_ts
                            }
                        },
                        &gen_write_into_buffer_query_part_syn_variant_er_init_eprintln_response_creation_ts(
                            &operation
                        )
                    );
                    quote! {
                        {
                            #increment_init_ts
                            let elements = {
                                let mut acc_b86a253a = #StringTs::default();
                                #fields_named_without_primary_key_update_assignment_ts
                                let _: Option<char> = acc_b86a253a.pop();
                                acc_b86a253a
                            };
                            let primary_keys = {
                                let mut acc_a95eb175 = #StringTs::default();
                                for el_9b2b56f8 in &#UpdateForQueryVecSc {
                                    #if_write_is_err_ts
                                }
                                let _: Option<char> = acc_a95eb175.pop();
                                acc_a95eb175
                            };
                            let return_columns = {
                                let mut acc_fd44b0aa = String::new();
                                for el_bcf0dde4 in &#UpdateForQueryVecSc {
                                    match el_bcf0dde4.select_only_updated_ids_query_part(&mut #IncrementSc) {
                                        Ok(v_4f536654) => {
                                            acc_fd44b0aa.push_str(&v_4f536654);
                                        },
                                        Err(#Er0) => {
                                            #ts_1b64e228
                                        }
                                    }
                                }
                                acc_fd44b0aa
                            };
                            pg_crud::gen_update_many_query_string(
                                #TableSc,
                                &elements,
                                Self::#PrimaryKeySc(),
                                &primary_keys,
                                &return_columns
                            )
                        }
                    }
                };
                let binded_query_ts = {
                    let pg_syn_variant_er_init_eprintln_response_creation_ts =
                        gen_operation_er_init_eprintln_response_creation_ts(
                            &operation,
                            &try_bind_syn_variant_wrapper,
                            file!(),
                            line!(),
                            column!(),
                        );
                    let fields_named_without_primary_key_update_assignment_ts =
                        gen_fields_named_without_primary_key_without_comma_ts(
                            &|element: &SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let as_pg_crud_pg_type_pg_type_ts =
                                    gen_as_pg_type_ts(&element.field_type);
                                quote! {
                                    for el_4b24f8f0 in &#UpdateForQueryVecSc {
                                        if let Some(v_2edaa480) = &el_4b24f8f0.#field_ident {
                                            if let Err(er_981062db) = #QuerySc.try_bind(el_4b24f8f0.#primary_key_field_ident) {
                                                let #Er0 = er_981062db.to_string();
                                                #pg_syn_variant_er_init_eprintln_response_creation_ts
                                            }
                                            match #as_pg_crud_pg_type_pg_type_ts #UpdateQueryBindSc(
                                                v_2edaa480.#ValueSc.clone(),
                                                #QuerySc,
                                            ) {
                                                Ok(v_600e67dc) => {
                                                    #QuerySc = v_600e67dc;
                                                },
                                                Err(#Er0) => {
                                                    #pg_syn_variant_er_init_eprintln_response_creation_ts
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        );
                    let primary_key_update_assignment_ts = quote! {
                        for el_323f7dfc in &#UpdateForQueryVecSc {
                            match #primary_key_field_type_as_pg_type_ts #UpdateQueryBindSc(
                                el_323f7dfc.#primary_key_field_ident,
                                #QuerySc,
                            ) {
                                Ok(v_c40a4522) => {
                                    #QuerySc = v_c40a4522;
                                },
                                Err(#Er0) => {
                                    #pg_syn_variant_er_init_eprintln_response_creation_ts
                                }
                            }
                        }
                    };
                    let binded_query_select_only_updated_ids_query_bind_ts =
                        gen_fields_named_without_primary_key_without_comma_ts(
                            &|element: &SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let as_pg_crud_pg_type_pg_type_ts =
                                    gen_as_pg_type_ts(&element.field_type);
                                quote! {
                                    for el_a1660ed1 in &#UpdateForQueryVecSc {
                                        if let Some(v_47030ac2) = &el_a1660ed1.#field_ident {
                                            match #as_pg_crud_pg_type_pg_type_ts select_only_updated_ids_query_bind(
                                                &v_47030ac2.#ValueSc,
                                                #QuerySc
                                            ) {
                                                Ok(v_c5b79b95) => {
                                                    #QuerySc = v_c5b79b95;
                                                },
                                                Err(#Er0) => {
                                                    #pg_syn_variant_er_init_eprintln_response_creation_ts
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        );
                    quote! {
                        let mut #QuerySc = #sqlx_query_sqlx_pg_ts(&#QueryStringSc);
                        #fields_named_without_primary_key_update_assignment_ts
                        #primary_key_update_assignment_ts
                        #binded_query_select_only_updated_ids_query_bind_ts
                        #QuerySc
                    }
                };
                let pg_logic_ts = wrap_content_into_pg_transaction_begin_commit_value_ts(
                    &operation,
                    &gen_create_update_delete_many_fetch_ts(&CreateOrUpdateOrDeleteMany::Update),
                );
                impl_ident_vec_ts.push(gen_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &Ts2::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &pg_logic_ts,
                ));
            };
            quote! {
                #try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts
            }
        };
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_request_syn_variants);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &vec_ident_read_only_ids_ts,
                &ValueSc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        impl_ident_vec_ts.push(gen_operation_payload_example_ts(&operation));
        quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.update_many_content_write_into_gen_pg_table_update_many,
        "gen_pg_table_update_many",
        &update_many_ts,
        &FormatWithCargofmt::True,
    );
    let update_one_ts = {
        let operation = Operation::UpdateOne;
        let type_variants_from_request_response_syn_variants =
            gen_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = gen_parameters_pattern_ts(&operation, Ts2::new());
        let operation_ts = {
            let try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts =
                gen_ident_try_operation_logic_response_variants_ident_operation_er_convert_ts(
                    &operation,
                    &ident_read_only_ids_ucc,
                    &type_variants_from_request_response_syn_variants,
                );
            {
                let parameters_logic_ts = {
                    let parameters_logic_ts = gen_parameters_logic_ts(&operation);
                    quote! {
                        #parameters_logic_ts
                        let #UpdateForQuerySc = #ident_update_for_query_ucc::#FromHandleSc(#ParametersSc.#PayloadSc);
                    }
                };
                let query_string_ts = {
                    let additional_parameters_modification_ts =
                        gen_fields_named_without_primary_key_without_comma_ts(
                            &|element: &SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let field_ident_dq_ts = dq_ts(&field_ident);
                                let ts_9ec6b359 =
                                    gen_operation_er_init_eprintln_response_creation_ts(
                                        &operation,
                                        &query_part_syn_variant_wrapper,
                                        file!(),
                                        line!(),
                                        column!(),
                                    );
                                let gen_column_queals_value_comma_update_one_query_part_sc =
                                    GenColumnQuealsValueCommaUpdateOneQueryPartSc;
                                let update_query_part_field_ident_sc =
                                    UpdateQueryPartSelfSc::from_tokens(&field_ident);
                                quote! {
                                    if let Some(v_2d144436) = &#UpdateForQuerySc.#field_ident {
                                        acc_683e37b8.push_str(&#PgCrudSc::#gen_column_queals_value_comma_update_one_query_part_sc(
                                            #field_ident_dq_ts,
                                            &match #ident_update_for_query_ucc::#update_query_part_field_ident_sc(v_2d144436, &mut #IncrementSc) {
                                                Ok(v_1ec12051) => v_1ec12051,
                                                Err(#Er0) => {
                                                    #ts_9ec6b359
                                                }
                                            }
                                        ));
                                    }
                                }
                            },
                        );
                    let additional_parameters_primary_key_modification_ts =
                        gen_match_update_query_part_primary_key_ts(
                            &operation,
                            &quote! {#UpdateForQuerySc},
                        );
                    let ts_255ad2f1 = gen_operation_er_init_eprintln_response_creation_ts(
                        &operation,
                        &query_part_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                    quote! {
                        {
                            #increment_init_ts
                            let #ColumnsSc = {
                                let mut acc_683e37b8 = #StringTs::default();
                                #additional_parameters_modification_ts
                                let _: Option<char> = acc_683e37b8.pop();
                                acc_683e37b8
                            };
                            let #PrimaryKeyQueryPartSc = #additional_parameters_primary_key_modification_ts;
                            let return_columns = match #UpdateForQuerySc.select_only_updated_ids_query_part(&mut #IncrementSc) {
                                Ok(v_7f0d86a1) => v_7f0d86a1,
                                Err(#Er0) => {
                                    #ts_255ad2f1
                                }
                            };
                            #PgCrudSc::gen_update_one_query_string(
                                #TableSc,
                                &#ColumnsSc,
                                Self::#PrimaryKeySc(),
                                &#PrimaryKeyQueryPartSc,
                                &return_columns
                            )
                        }
                    }
                };
                let binded_query_ts = {
                    let ts_1bdf01cd = gen_operation_er_init_eprintln_response_creation_ts(
                        &operation,
                        &try_bind_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                    let binded_query_modifications_ts =
                        gen_fields_named_without_primary_key_without_comma_ts(
                            &|element: &SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let as_pg_crud_pg_type_pg_type_ts =
                                    gen_as_pg_type_ts(&element.field_type);
                                quote! {
                                    if let Some(v_ed87c152) = &#UpdateForQuerySc.#field_ident {
                                        match #as_pg_crud_pg_type_pg_type_ts #UpdateQueryBindSc(
                                            v_ed87c152.#ValueSc.clone(),//todo is there a way to remove .clone here?
                                            #QuerySc
                                        ) {
                                            Ok(v_c3c1b857) => {
                                                #QuerySc = v_c3c1b857;
                                            }
                                            Err(#Er0) => {
                                                #ts_1bdf01cd
                                            }
                                        }
                                    }
                                }
                            },
                        );
                    let binded_query_primary_key_modification_ts = quote! {
                        match #primary_key_field_type_as_pg_type_ts #UpdateQueryBindSc(
                            #UpdateForQuerySc.#primary_key_field_ident,
                            #QuerySc,
                        ) {
                            Ok(v_d64bac39) => {
                                #QuerySc = v_d64bac39;
                            },
                            Err(#Er0) => {
                                #ts_1bdf01cd
                            }
                        }
                    };
                    let binded_query_select_only_updated_ids_query_bind_ts =
                        gen_fields_named_without_primary_key_without_comma_ts(
                            &|element: &SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                let as_pg_crud_pg_type_pg_type_ts =
                                    gen_as_pg_type_ts(&element.field_type);
                                quote! {
                                    if let Some(v_b2902425) = &#UpdateForQuerySc.#field_ident {
                                        match #as_pg_crud_pg_type_pg_type_ts select_only_updated_ids_query_bind(
                                            &v_b2902425.#ValueSc,
                                            #QuerySc
                                        ) {
                                            Ok(v_cc6145f8) => {
                                                #QuerySc = v_cc6145f8;
                                            },
                                            Err(#Er0) => {
                                                #ts_1bdf01cd
                                            }
                                        }
                                    }
                                }
                            },
                        );
                    quote! {
                        let mut #QuerySc = #sqlx_query_sqlx_pg_ts(&#QueryStringSc);
                        #binded_query_modifications_ts
                        #binded_query_primary_key_modification_ts
                        #binded_query_select_only_updated_ids_query_bind_ts
                        #QuerySc
                    }
                };
                let pg_logic_ts = wrap_content_into_pg_transaction_begin_commit_value_ts(
                    &operation,
                    // &gen_create_update_delete_one_fetch_ts(&CreateOrUpdateOrDeleteOne::Update)
                    &{
                        let operation_6ab94855 =
                            Operation::from(&CreateOrUpdateOrDeleteOne::Update);
                        wrap_into_value_ts(&gen_fetch_one_ts(
                            &ExecutorSc,
                            &gen_match_ident_read_only_ids_as_from_row_from_row_ts(
                                &gen_match_pg_transaction_rollback_await_ts(
                                    &operation_6ab94855,
                                    file!(),
                                    line!(),
                                    column!(),
                                    file!(),
                                    line!(),
                                    column!(),
                                ),
                            ),
                            &gen_match_pg_transaction_rollback_await_ts(
                                &operation_6ab94855,
                                file!(),
                                line!(),
                                column!(),
                                file!(),
                                line!(),
                                column!(),
                            ),
                        ))
                    },
                );
                impl_ident_vec_ts.push(gen_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &Ts2::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &pg_logic_ts,
                ));
            };
            quote! {
                #try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts
            }
        };
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_request_syn_variants);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &ident_read_only_ids_ucc,
                &ValueSc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        impl_ident_vec_ts.push(gen_operation_payload_example_ts(&operation));
        quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.update_one_content_write_into_gen_pg_table_update_one,
        "gen_pg_table_update_one",
        &update_one_ts,
        &FormatWithCargofmt::True,
    );
    //todo return deleted rows ids vec
    let delete_many_ts = {
        let operation = Operation::DeleteMany;
        let type_variants_from_request_response_syn_variants =
            gen_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(query_part_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = gen_parameters_pattern_ts(
            &operation,
            gen_parameters_payload_and_default_ts(
                &operation,
                &quote! {{#pub_where_many_option_ident_where_many_ts}},
                &quote! {{#where_many_pg_crud_default_option_some_vec_one_el_call_ts}},
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts =
                gen_ident_try_operation_logic_response_variants_ident_operation_er_convert_ts(
                    &operation,
                    &vec_primary_key_field_type_read_ts,
                    &type_variants_from_request_response_syn_variants,
                );
            {
                let parameters_logic_ts = gen_parameters_logic_ts(&operation);
                let query_string_ts = {
                    let additional_paramaters_init_ts =
                        gen_read_or_delete_many_additional_paramaters_init_ts(
                            &ReadManyOrDeleteMany::DeleteMany,
                        );
                    quote! {#PgCrudSc::gen_delete_many_query_string(
                        #TableSc,
                        &{
                            #increment_init_ts
                            #additional_paramaters_init_ts
                        },
                        Self::#PrimaryKeySc(),
                    )}
                };
                let binded_query_ts = {
                    let query_pg_type_where_filter_query_bind_parameters_payload_where_many_query_ts = gen_query_pg_type_where_filter_query_bind_parameters_payload_where_many_query_ts(&operation);
                    quote! {
                        let mut #QuerySc = #sqlx_query_sqlx_pg_ts(&#QueryStringSc);
                        #query_pg_type_where_filter_query_bind_parameters_payload_where_many_query_ts
                        #QuerySc
                    }
                };
                let pg_logic_ts = wrap_content_into_pg_transaction_begin_commit_value_ts(
                    &operation,
                    &gen_create_update_delete_many_fetch_ts(&CreateOrUpdateOrDeleteMany::Delete),
                );
                impl_ident_vec_ts.push(gen_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &Ts2::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &pg_logic_ts,
                ));
            };
            quote! {
                #try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts
            }
        };
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_request_syn_variants);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &vec_primary_key_field_type_read_ts,
                &ValueSc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        impl_ident_vec_ts.push(gen_operation_payload_example_ts(&operation));
        quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.delete_many_content_write_into_gen_pg_table_delete_many,
        "gen_pg_table_delete_many",
        &delete_many_ts,
        &FormatWithCargofmt::True,
    );
    let delete_one_ts = {
        let operation = Operation::DeleteOne;
        let type_variants_from_request_response_syn_variants =
            gen_type_variants_from_request_response_syn_variants(
                &common_route_syn_variants
                    .iter()
                    .copied()
                    .chain(once(row_and_rollback_syn_variant_wrapper.get_syn_variant()))
                    .chain(once(try_bind_syn_variant_wrapper.get_syn_variant()))
                    .collect(),
                &operation,
            );
        let parameters_ts = gen_parameters_pattern_ts(
            &operation,
            gen_parameters_payload_and_default_ts(
                &operation,
                &{
                    let ts =
                        gen_pub_handle_primary_key_field_ident_primary_key_inner_type_handle_ts(
                            &SelfReadUcc::from_type_last_segment(primary_key_field_type),
                        );
                    quote! {{#ts}}
                },
                &{
                    let primary_key_field_with_default_option_some_vec_one_el_ts = {
                        quote! {
                            #primary_key_field_ident: #PgCrudDefaultOptionSomeVecOneElCall
                        }
                    };
                    quote! {{#primary_key_field_with_default_option_some_vec_one_el_ts}}
                },
            ),
        );
        let operation_ts = {
            let try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts =
                gen_ident_try_operation_logic_response_variants_ident_operation_er_convert_ts(
                    &operation,
                    &primary_key_field_type_as_pg_type_read_ucc,
                    &type_variants_from_request_response_syn_variants,
                );
            {
                let parameters_logic_ts = gen_parameters_logic_ts(&operation);
                let query_string_ts = quote! {#PgCrudSc::gen_delete_one_query_string(
                    #TableSc,
                    Self::#PrimaryKeySc(),
                )};
                let binded_query_ts = {
                    let ts_1319f705 = gen_operation_er_init_eprintln_response_creation_ts(
                        &operation,
                        &try_bind_syn_variant_wrapper,
                        file!(),
                        line!(),
                        column!(),
                    );
                    quote! {
                        let mut #QuerySc = #sqlx_query_sqlx_pg_ts(&#QueryStringSc);
                        match pg_crud::PgTypeWhereFilter::query_bind(
                            #ParametersSc.#PayloadSc.#primary_key_field_ident,
                            #QuerySc
                        ) {
                            Ok(v_3099ea0f) => {
                                #QuerySc = v_3099ea0f;
                            },
                            Err(#Er0) => {
                                #ts_1319f705
                            }
                        }
                        #QuerySc
                    }
                };
                let pg_logic_ts = wrap_content_into_pg_transaction_begin_commit_value_ts(
                    &operation,
                    &gen_create_update_delete_one_fetch_ts(&CreateOrUpdateOrDeleteOne::Delete),
                );
                impl_ident_vec_ts.push(gen_operation_ts(
                    &operation,
                    &common_additional_logic_ts,
                    &parameters_logic_ts,
                    &Ts2::new(),
                    &query_string_ts,
                    &binded_query_ts,
                    &pg_logic_ts,
                ));
            };
            quote! {
                #try_operation_logic_response_variants_impl_from_try_operation_logic_er_for_try_operation_logic_response_variants_try_operation_logic_er_ts
            }
        };
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_request_syn_variants);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_variants_from_request_response_syn_variants,
                &primary_key_field_type_as_pg_type_read_ucc,
                &ValueSc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        impl_ident_vec_ts.push(gen_operation_payload_example_ts(&operation));
        quote! {
            #parameters_ts
            #operation_ts
            #try_operation_ts
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.delete_one_content_write_into_gen_pg_table_delete_one,
        "gen_pg_table_delete_one",
        &delete_one_ts,
        &FormatWithCargofmt::True,
    );
    impl_ident_vec_ts.push({
        let routes_handle_ts = {
            let operation_routes_ts = [
                Operation::CreateMany,
                Operation::CreateOne,
                Operation::ReadMany,
                Operation::ReadOne,
                Operation::UpdateMany,
                Operation::UpdateOne,
                Operation::DeleteMany,
                Operation::DeleteOne
            ].into_iter().map(|operation: Operation|{
                let method_ts = match &operation {
                    Operation::CreateMany |
                    Operation::CreateOne |
                    Operation::ReadMany |
                    Operation::ReadOne => quote!{post},
                    Operation::UpdateMany |
                    Operation::UpdateOne => quote!{patch},
                    Operation::DeleteMany |
                    Operation::DeleteOne => quote!{delete},
                };
                let operation_sc_ts = operation.self_handle_sc_ts();
                let operation_payload_example_sc =
                    operation.operation_payload_example_sc();
                let (
                    slash_operation_dq_ts,
                    slash_operation_payload_example_dq_ts
                ) = {
                    let gen_slash_route_dq_ts = |
                        value: &dyn Display
                    | dq_ts(&format!("/{value}"));
                    (
                        gen_slash_route_dq_ts(&operation.self_sc_str()),
                        gen_slash_route_dq_ts(&operation_payload_example_sc)
                    )
                };
                quote!{
                    .route(#slash_operation_dq_ts, axum::routing::#method_ts({
                        let table_owned = table.to_owned();
                        async move |
                            app_state_99328dfe: axum::extract::State<std::sync::Arc<dyn pg_crud::CombinationOfAppStateLogicTraits>>,
                            request: axum::extract::Request
                        | Self::#operation_sc_ts(app_state_99328dfe, request, &table_owned).await
                    }))
                    .route(#slash_operation_payload_example_dq_ts, axum::routing::get(async move||Self::#operation_payload_example_sc()))
                }
            });
            quote!{
                #[allow(clippy::single_call_fn)]
                fn #RoutesHandleSc(#AppStateSc: #std_sync_arc_combination_of_app_state_logic_traits_ts, #TableSc: &str) -> axum::Router {
                    axum::Router::new().nest(
                        &format!("/{table}"),
                        axum::Router::new()
                        #(#operation_routes_ts)*
                        .with_state(#AppStateSc)
                    )
                }
            }
        };
        let routes_ts = quote!{
            pub fn #RoutesSc(#AppStateSc: #std_sync_arc_combination_of_app_state_logic_traits_ts) -> axum::Router {
                Self::#RoutesHandleSc(#AppStateSc, #self_table_name_call_ts)
            }
        };
        quote! {
            #routes_handle_ts
            #routes_ts
        }
    });
    let ident_tests_ts = {
        let ident_tests_sc = SelfTestsSc::from_display(&ident);
        let ident_dq_ts = dq_ts(&DisplayToScStr::case(&ident));
        let ident_create_many_parameters_ucc =
            gen_ident_operation_parameters_ucc(&Operation::CreateMany);
        let ident_read_many_parameters_ucc =
            gen_ident_operation_parameters_ucc(&Operation::ReadMany);
        let ident_create_many_payload_ucc = gen_ident_operation_payload_ucc(&Operation::CreateMany);
        let ident_read_many_payload_ucc = gen_ident_operation_payload_ucc(&Operation::ReadMany);
        let ident_create_one_parameters_ucc =
            gen_ident_operation_parameters_ucc(&Operation::CreateOne);
        let ident_read_one_parameters_ucc = gen_ident_operation_parameters_ucc(&Operation::ReadOne);
        let ident_read_one_payload_ucc = gen_ident_operation_payload_ucc(&Operation::ReadOne);
        let ident_update_one_parameters_ucc =
            gen_ident_operation_parameters_ucc(&Operation::UpdateOne);
        let config_path_ts = quote! {server_config::Config};
        let underscore_unused_ts = quote! {_unused};
        //todo maybe remove it?\
        let gen_some_pg_type_where_try_new_ts =
            |logical_operator_ts: &dyn ToTokens, ts: &dyn ToTokens| {
                quote! {
                    Some(
                        #import_path::PgTypeWhere::try_new(
                            #logical_operator_ts,
                            #ts
                        ).expect("6b0491b2"),
                    )
                }
            };
        let gen_some_pg_type_where_try_new_and_ts = |ts: &dyn ToTokens| {
            gen_some_pg_type_where_try_new_ts(&quote! {#import_path::LogicalOperator::And}, ts)
        };
        let gen_pg_type_where_try_new_primary_key_ts = quote! {
            #import_path::PgTypeWhere::try_new(
                logical_operator,
                vec
            ).expect("fd20ad6d")
        };
        let ident_create_default_fields_init_without_primary_key_ts =
            gen_fields_named_without_primary_key_with_comma_ts(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type_as_pg_type_create_ts = gen_as_pg_type_create_ts(&element.field_type);
                quote! {
                    #field_ident: <#field_type_as_pg_type_create_ts as pg_crud::DefaultOptionSomeVecOneEl>::default_option_some_vec_one_el()
                }
            });
        let fields_none_init_ts =
            gen_fields_named_without_primary_key_with_comma_ts(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                quote! {#field_ident: None}
            });
        //todo instead of first dropping table - check if its not exists. if exists Test must fail
        let select_default_all_with_max_page_size_not_empty_unique_vec_ts = {
            let ts = gen_fields_named_with_comma_ts(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type = &element.field_type;
                let field_ident_ucc = ToTokensToUccTs::case_or_panic(&field_ident);
                quote! {
                    #ident_select_ucc::#field_ident_ucc(
                        <
                            <
                                #field_type
                                as
                                pg_crud::PgType
                            >::Select
                            as
                            pg_crud::#DefaultOptionSomeVecOneElMaxPageSizeUcc
                        >::#DefaultOptionSomeVecOneElMaxPageSizeSc()
                    )
                }
            });
            quote! {
                let select_default_all_with_max_page_size = pg_crud::NotEmptyUniqueVec::try_new(vec![
                    #ts
                ]).expect("5e82ac66");
            }
        };
        let gen_primary_key_field_type_as_pg_type_primary_key_method_call_ts =
            |method_ts: &dyn ToTokens, parameters_ts: &dyn ToTokens| {
                quote! {
                    <
                        #primary_key_field_type
                        as
                        pg_crud::PgTypePrimaryKey
                    >::#method_ts(
                        #parameters_ts
                    )
                }
            };
        let primary_key_field_type_read_into_table_type_declaration_el_primary_key_field_ident_clone_ts =
            gen_primary_key_field_type_as_pg_type_primary_key_method_call_ts(
                &ReadIntoTableTypeDeclarationSc,
                &quote! {el_adcc8db3},
            );
        let (
            primary_key_field_type_read_only_ids_into_read_el_fdc88812_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_el_43ab7fb5_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_el_bf356906_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_el_80a93892_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_el_adf2b4c4_primary_key_field_ident_ts,
            primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
            primary_key_field_type_read_only_is_into_read_read_only_ids_el_primary_key_field_ident_ts_937c5af3,
            primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
        ) = {
            let gen_read_only_ids_into_read_ts = |ts: &dyn ToTokens| {
                gen_primary_key_field_type_as_pg_type_primary_key_method_call_ts(
                    &ReadOnlyIdsIntoReadSc,
                    &ts,
                )
            };
            (
                gen_read_only_ids_into_read_ts(&quote! {el_fdc88812.#primary_key_field_ident}),
                gen_read_only_ids_into_read_ts(&quote! {el_43ab7fb5.#primary_key_field_ident}),
                gen_read_only_ids_into_read_ts(&quote! {el_bf356906.#primary_key_field_ident}),
                gen_read_only_ids_into_read_ts(&quote! {el_80a93892.#primary_key_field_ident}),
                gen_read_only_ids_into_read_ts(&quote! {el_adf2b4c4.#primary_key_field_ident}),
                gen_read_only_ids_into_read_ts(
                    &quote! {read_only_ids_from_try_create_one.#primary_key_field_ident},
                ),
                gen_read_only_ids_into_read_ts(
                    &quote! {read_only_ids_element_937c5af3.#primary_key_field_ident},
                ),
                gen_read_only_ids_into_read_ts(
                    &quote! {read_only_ids_returned_from_create_one.#primary_key_field_ident},
                ),
            )
        };
        let primary_key_field_type_as_pg_type_update_as_pg_type_primary_key_read_only_ids_into_update_ts = {
            let method_call_ts = gen_primary_key_field_type_as_pg_type_primary_key_method_call_ts(
                &ReadOnlyIdsIntoUpdateSc,
                &quote! {read_only_ids_element_937c5af3.#primary_key_field_ident},
            );
            quote! {
                <
                    #primary_key_field_type
                    as
                    pg_crud::PgType
                >::Update::from(#method_call_ts)
            }
        };
        let (
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_and_create_ts,
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_from_try_create_one_ident_create_ts,
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_create_ts,
            field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_clone_ident_create_clone_ts,
        ) = {
            enum ShouldAddDotClone {
                False,
                True,
            }
            let gen_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts =
                |read_only_ids_ts: &dyn ToTokens,
                 create_ts: &dyn ToTokens,
                 should_add_dot_clone: &ShouldAddDotClone| {
                    gen_fields_named_without_primary_key_with_comma_ts(
                        &|element: &SynFieldWrapper| {
                            let field_ident_931fabfc = &element.field_ident;
                            let field_type_714e077b = &element.field_type;
                            let maybe_dot_clone_ts = match &should_add_dot_clone {
                                ShouldAddDotClone::False => Ts2::new(),
                                ShouldAddDotClone::True => quote! {.clone()},
                            };
                            quote! {
                                #field_ident_931fabfc: <#field_type_714e077b as pg_crud::PgTypeTestCases>::read_only_ids_merged_with_create_into_option_value_read(
                                    #read_only_ids_ts.#field_ident_931fabfc #maybe_dot_clone_ts.expect("f967434c"),
                                    #create_ts.#field_ident_931fabfc #maybe_dot_clone_ts
                                )
                            }
                        },
                    )
                };
            (
                gen_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts(
                    &ReadOnlyIdsSc,
                    &CreateSc,
                    &ShouldAddDotClone::False,
                ),
                gen_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts(
                    &quote! {read_only_ids_from_try_create_one},
                    &quote! {ident_create},
                    &ShouldAddDotClone::False,
                ),
                gen_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts(
                    &quote! {read_only_ids_returned_from_create_one},
                    &quote! {ident_create_default},
                    &ShouldAddDotClone::False,
                ),
                gen_field_ident_read_only_ids_merged_with_create_into_option_value_read_ts(
                    &quote! {read_only_ids_returned_from_create_one},
                    &quote! {ident_create},
                    &ShouldAddDotClone::True,
                ),
            )
        };
        let option_ident_where_many_ts_dc1232c7 =
            gen_fields_named_without_primary_key_with_comma_ts(&|element: &SynFieldWrapper| {
                let field_ident_edb35ef4 = &element.field_ident;
                quote! {
                    #field_ident_edb35ef4: None
                }
            });
        let select_default_all_with_max_page_size_clone_ts =
            quote! {select_default_all_with_max_page_size.clone()};
        let common_read_only_ids_returned_from_create_one_ts = {
            let primary_key_read_ts = quote! {primary_key_read};
            let primary_key_read_clone_ts = quote! {primary_key_read.clone()};
            let value_init_ts = gen_import_path_value_init_ts(&primary_key_read_clone_ts);
            quote! {
                let #CommonReadOnlyIdsReturnedFromCreateOneSc = {
                    let read_only_ids_from_try_create_one = gen_read_only_ids_from_try_create_one_default(&#UrlSc, &table_init).await;
                    let primary_key_read = #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts;
                    assert_eq!(
                        #ident_read_ucc {
                            #primary_key_field_ident: Some(#value_init_ts),
                            #fields_none_init_ts
                        },
                        gen_ident_try_read_one_handle_primary_key(
                            &#UrlSc,
                            #primary_key_read_clone_ts,
                            #SelectPrimaryKeySc.clone(),
                            &table_init
                        )
                        .await
                        .expect("36b95e96"),
                        "3d9f2ec0"
                    );
                    assert_eq!(
                        gen_try_delete_one_handle(
                            &url,
                            #primary_key_read_clone_ts,
                            &table_init,
                        ).await.expect("4d96d385"),
                        #primary_key_read_clone_ts,
                        "26e2058b"
                    );
                    gen_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                        &url,
                        #primary_key_read_ts,
                        #select_default_all_with_max_page_size_clone_ts,
                        &table_init,
                    ).await;
                    read_only_ids_from_try_create_one
                };
            }
        };
        let gen_ident_create_ts = |field_ident: &Ident, ts: &dyn ToTokens| {
            gen_fields_named_without_primary_key_with_comma_ts(&|element: &SynFieldWrapper| {
                let field_ident_42fe57c8 = &element.field_ident;
                let field_type_f3d6c5c7 = &element.field_type;
                if field_ident == field_ident_42fe57c8 {
                    quote! {
                        #field_ident_42fe57c8: #ts
                    }
                } else {
                    quote! {
                        #field_ident_42fe57c8: <
                            <#field_type_f3d6c5c7 as pg_crud::PgType>::Create as pg_crud::DefaultOptionSomeVecOneEl
                        >::default_option_some_vec_one_el()
                    }
                }
            })
        };
        let gen_ident_create_content_el_id_ts =
            |field_ident: &Ident, el_ts: &dyn ToTokens| gen_ident_create_ts(field_ident, &el_ts);
        let gen_ident_create_content_el_ts =
            |field_ident: &Ident| gen_ident_create_ts(field_ident, &ElementSc);
        let gen_table_test_name_field_ident_ts = |test_name: &str, field_ident: &Ident| {
            format!("table_{test_name}_{field_ident}")
                .parse::<Ts2>()
                .expect("eb30c1e4")
        };
        let mut table_field_idents_init_vec_ts = Vec::new();
        let mut table_test_name_field_idents_vec_ts = Vec::new();
        let mut fill_table_field_idents_vec_ts = |test_names: Vec<&str>| {
            for el_8f39799f in test_names {
                let gen_init_variable_name_ts = |field_ident: &Ident| {
                    format!("table_{el_8f39799f}_{field_ident}")
                        .parse::<Ts2>()
                        .expect("2003ad9f")
                };
                table_field_idents_init_vec_ts.push(
                    gen_fields_named_without_primary_key_without_comma_ts(
                        &|el_51b56762: &SynFieldWrapper| {
                            let field_ident = &el_51b56762.field_ident;
                            let init_variable_name_ts = gen_init_variable_name_ts(field_ident);
                            let format_ts = dq_ts(&format!("{el_8f39799f}_{field_ident}"));
                            quote! {
                                let #init_variable_name_ts = add_table_postfix(#format_ts);
                            }
                        },
                    ),
                );
                table_test_name_field_idents_vec_ts.push(
                    gen_fields_named_without_primary_key_without_comma_ts(
                        &|el_785024b5: &SynFieldWrapper| {
                            let field_ident = &el_785024b5.field_ident;
                            let init_variable_name_ts = gen_init_variable_name_ts(field_ident);
                            quote! {&#init_variable_name_ts,}
                        },
                    ),
                );
            }
        };
        let table_read_only_ids_merged_with_create_into_where_equal_name =
            "8e427ad7_5231_4f1e_8579_2e1aaa5da988";
        let table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name =
            "eb24448c_fa63_4259_bb05_3215802a78f6";
        let table_read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_name =
            "9ac6d79a_2673_4c07_be4a_01c5c20ff1ab";
        let table_create_into_pg_type_option_vec_where_dim_one_equal_name =
            "72940b0e_cd26_493f_9ec1_2d999d9a4401";
        let table_read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_name =
            "5a52af33_a590_403b_808e_961df6d7e7aa";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_name =
            "1f388ef8_dc28_489d_bed9_ca4e7f640dd5";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_name =
            "581c947f_9b0f_452f_8e52_524088bbb2e7";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_name =
            "de556c26_9297_4adb_9483_22d474cf1e7d";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_name =
            "35b26a97_abdd_4cf9_b4e5_aa9b47aa1a0d";
        let table_create_into_pg_json_type_option_vec_where_length_equal_name =
            "1ce53b67_1e94_413e_83cf_c6d7094289a8";
        let table_create_into_pg_json_type_option_vec_where_length_greater_than_name =
            "6b6bdfe0_c7b8_43fd_ac2e_854a47c0b64c";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_name =
            "35a01678_f7e2_482d_9803_c3b5a23d36ad";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_name =
            "33a3706a_ef28_4c80_88e0_b8e7fb720de2";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_name =
            "a3e2165c_e030_4b31_ab3d_dcd29f27f90b";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_name =
            "427ac837_383b_4af1_b956_3e64a78e1449";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_name =
            "fe3267a0_f49a_42ce_8e51_2a10e5360eb8";
        let table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_name =
            "b4504737_4463_4e47_bb30_9512275c66b1";
        fill_table_field_idents_vec_ts(vec![
            &table_read_only_ids_merged_with_create_into_where_equal_name,
            &table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name,
            &table_read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_name,
            &table_create_into_pg_type_option_vec_where_dim_one_equal_name,
            &table_read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_name,
            &table_create_into_pg_json_type_option_vec_where_length_equal_name,
            &table_create_into_pg_json_type_option_vec_where_length_greater_than_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_name,
        ]);
        let select_default_all_with_max_page_size_cloned_clone_ts =
            quote! {select_default_all_with_max_page_size_cloned.clone()};
        let read_only_ids_to_two_dimal_vec_read_inner_acc_fields_ts =
            gen_fields_named_without_primary_key_without_comma_ts(&|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_ident_read_only_ids_to_two_dimal_vec_read_inner_acc_sc =
                    SelfReadOnlyIdsToTwoDimalVecReadInnerAccSc::from_tokens(&field_ident);
                let ident_create_defaults_for_column_read_only_ids_to_two_dimal_vec_read_inner_ts =
                    gen_fields_named_without_primary_key_without_comma_ts(
                        &|el_0dfa76d6: &SynFieldWrapper| {
                            let field_ident_10070b70 = &el_0dfa76d6.field_ident;
                            let field_type_b33f54a9 = &el_0dfa76d6.field_type;
                            if field_ident == field_ident_10070b70 {
                                quote! {
                                    if let Some(v_a5f7e6cd) = &common_read_only_ids_returned_from_create_one.#field_ident_10070b70 {
                                        for el_b3522b7d in <#field_type_b33f54a9 as pg_crud::PgTypeTestCases>::read_only_ids_to_two_dimal_vec_read_inner(v_a5f7e6cd) {
                                            for _ in el_b3522b7d {
                                                acc_458cda9e.push(ident_create_default.clone());
                                            }
                                        }
                                    }
                                }
                            } else {
                                Ts2::new()
                            }
                        },
                    );
                quote! {
                    let #field_ident_read_only_ids_to_two_dimal_vec_read_inner_acc_sc = {
                        let mut acc_458cda9e = Vec::new();
                        #ident_create_defaults_for_column_read_only_ids_to_two_dimal_vec_read_inner_ts
                        acc_458cda9e
                    };
                }
            });
        let gen_read_only_ids_elements_ts_fe29ff70 = {
            let ident_read_fields_init_without_primary_key_ts =
                gen_fields_named_without_primary_key_with_comma_ts(
                    &|syn_field_wrapper: &SynFieldWrapper| {
                        let field_ident_5bea122e = &syn_field_wrapper.field_ident;
                        let field_type_f7f832df = &syn_field_wrapper.field_type;
                        let value_init_ts =
                            gen_import_path_value_init_ts(&PgCrudDefaultOptionSomeVecOneElCall);
                        quote! {
                            #field_ident_5bea122e: el_f108da5a.#field_ident_5bea122e.as_ref().map_or_else(
                                || Some(#value_init_ts),
                                <#field_type_f7f832df as pg_crud::PgTypeTestCases>::read_only_ids_to_option_value_read_default_option_some_vec_one_el
                            )
                        }
                    },
                );
            quote! {
                async fn gen_read_only_ids_elements_8a1ef027(
                    url: &str,
                    table_9c259e1c: &str,
                    select_default_all_with_max_page_size: pg_crud::NotEmptyUniqueVec<#ident_select_ucc>,
                    read_only_ids_to_two_dimal_vec_read_inner_acc: Vec<#ident_create_ucc>
                ) -> Vec<#ident_read_only_ids_ucc> {
                    let read_only_ids_elements_efeed554 = futures::StreamExt::collect::<Vec<Vec<#ident_read_only_ids_ucc>>>(
                        futures::StreamExt::buffer_unordered(
                            futures::stream::iter(
                                read_only_ids_to_two_dimal_vec_read_inner_acc
                                .chunks(25)
                                .map(Vec::from)
                                .map(|el_8e425cb1| futures::FutureExt::boxed(async move { #ident::try_create_many_handle(
                                    url,
                                    #ident_create_many_parameters_ucc {
                                        payload: #ident_create_many_payload_ucc(el_8e425cb1)
                                    },
                                    table_9c259e1c
                                ).await.expect("38a24e7a") }))
                            ),
                            5
                        )
                    )
                    .await
                    .into_iter()
                    .flatten()
                    .collect::<Vec<#ident_read_only_ids_ucc>>();
                    assert_eq!(
                        itertools::Itertools::sorted_by(
                            read_only_ids_elements_efeed554.iter().map(|el_f108da5a| {
                                #ident_read_ucc {
                                    #primary_key_field_ident: <
                                        #primary_key_field_type as pg_crud::PgTypeTestCases
                                    >::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                        &el_f108da5a.#primary_key_field_ident
                                    ),
                                    #ident_read_fields_init_without_primary_key_ts
                                }
                            }),
                            |first, second| match (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                (Some(first_handle), Some(second_handle)) => first_handle.#ValueSc.cmp(&second_handle.#ValueSc),
                                _ => panic!("0f1d45ed"),
                            }
                        ).collect::<Vec<#ident_read_ucc>>(),
                        itertools::Itertools::sorted_by(
                            gen_try_read_many_order_by_primary_key_with_big_pagination(
                                url,
                                gen_ident_where_many_pripery_key_others_none(
                                    Some(
                                        gen_pg_type_where_try_new_primary_key(
                                            pg_crud::LogicalOperator::Or,
                                            read_only_ids_elements_efeed554.iter().map(|el_43ab7fb5| #primary_key_field_type_where_ts::Equal(
                                                pg_crud::PgTypeWhereEqual {
                                                    logical_operator: pg_crud::LogicalOperator::Or,
                                                    #ValueSc: #primary_key_field_type_table_type_declaration_ts::new(
                                                        <#primary_key_field_type as pg_crud::PgType>::into_inner(
                                                            #primary_key_field_type_read_only_ids_into_read_el_43ab7fb5_primary_key_field_ident_ts
                                                        )
                                                    )
                                                }
                                            )).collect()
                                        )
                                    )
                                ),
                                #select_default_all_with_max_page_size_clone_ts,
                                table_9c259e1c
                            )
                            .await
                            .expect("097d5e7d")
                            .into_iter(),
                            |first, second| match (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                (Some(first_handle), Some(second_handle)) => first_handle.#ValueSc.cmp(&second_handle.#ValueSc),
                                _ => panic!("51e477ea"),
                            }
                        )
                        .collect::<Vec<#ident_read_ucc>>(),
                        "50198a7f"
                    );
                    read_only_ids_elements_efeed554
                }
            }
        };
        let create_many_tests_ts = {
            let create_many_tests_ts = gen_fields_named_without_primary_key_without_comma_ts(
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let ident_create_ts_910fa600 =
                        gen_ident_create_content_el_id_ts(field_ident, &quote! {el_03a4f4ee});
                    quote! {
                        for el_fce0969c in <#field_type as pg_crud::PgTypeTestCases>::#OptionVecCreateSc().unwrap_or(Vec::new())
                            .chunks(10)
                            .map(Vec::from)
                        {
                            let table_create_many_cloned = table_create_many.clone();
                            let url_cloned = url.clone();
                            let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                            acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                let ident_vec_create = {
                                    let mut acc_92d248f7 = Vec::new();
                                    for el_03a4f4ee in el_fce0969c {
                                        acc_92d248f7.push(#ident_create_ucc {
                                            #ident_create_ts_910fa600
                                        });
                                    }
                                    acc_92d248f7
                                };
                                let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                                    &url_cloned,
                                    #ident_create_many_parameters_ucc {
                                        #PayloadSc: #ident_create_many_payload_ucc(ident_vec_create.clone())
                                    },
                                    &table_create_many_cloned.clone()
                                ).await.expect("5eecedc4");
                                assert_eq!(
                                    gen_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create(
                                        read_only_ids_from_try_create_many.clone(),
                                        ident_vec_create.clone()
                                    ),
                                    gen_try_read_many_order_by_primary_key_with_big_pagination(
                                        &url_cloned,
                                        gen_ident_where_many_pripery_key_others_none(
                                            Some(
                                                gen_pg_type_where_try_new_primary_key(
                                                    pg_crud::LogicalOperator::Or,
                                                    {
                                                        let mut acc_1381c719 = Vec::new();
                                                        for el_bf356906 in &read_only_ids_from_try_create_many {
                                                            acc_1381c719.push(#primary_key_field_type_as_pg_type_where_ts::Equal(pg_crud::PgTypeWhereEqual {
                                                                logical_operator: pg_crud::LogicalOperator::Or,
                                                                //todo must use trait type instead
                                                                #ValueSc: #primary_key_field_type_table_type_declaration_ts::new(<#primary_key_field_type as pg_crud::PgType>::into_inner(
                                                                    #primary_key_field_type_read_only_ids_into_read_el_bf356906_primary_key_field_ident_ts
                                                                )),
                                                            }));
                                                        }
                                                        acc_1381c719
                                                    }
                                                )
                                            )
                                        ),
                                        #select_default_all_with_max_page_size_cloned_clone_ts,
                                        &table_create_many_cloned
                                    ).await.expect("bdb72341"),
                                    "d19bbbf6"
                                );
                                let read_only_ids_from_try_delete_many = itertools::Itertools::sorted(
                                    #ident::try_delete_many_handle(
                                        &url_cloned,
                                        #ident_delete_many_parameters_ucc {
                                            //todo rewrite it using new\try_new?
                                            payload: #ident_delete_many_payload_ucc {
                                                where_many: #option_ident_where_many_ucc(Some(
                                                    #ident_where_many_ucc {
                                                        #primary_key_field_ident: Some(gen_pg_type_where_try_new_or_primary_keys(
                                                            &read_only_ids_from_try_create_many
                                                        )),
                                                        #option_ident_where_many_ts_dc1232c7
                                                    }
                                                ))
                                            }
                                        },
                                        &table_create_many_cloned
                                    )
                                    .await
                                    .expect("716e470e")
                                    .into_iter()
                                ).collect::<Vec<<#primary_key_field_type as pg_crud::PgType>::Read>>();
                                assert_eq!(
                                    read_only_ids_from_try_delete_many,
                                    itertools::Itertools::sorted(
                                        read_only_ids_from_try_create_many
                                        .into_iter()
                                        .map(|el_80a93892| {
                                            #primary_key_field_type_read_only_ids_into_read_el_80a93892_primary_key_field_ident_ts
                                        })
                                    ).collect::<Vec<#primary_key_field_type_as_pg_type_read_ts>>(),
                                    "f58f5572"
                                );
                                assert!(
                                    gen_try_read_many_order_by_primary_key_with_big_pagination(
                                        &url_cloned,
                                        gen_ident_where_many_pripery_key_others_none(
                                            Some(
                                                gen_pg_type_where_try_new_primary_key(
                                                    pg_crud::LogicalOperator::Or,
                                                    {
                                                        let mut acc_87ea12c9 = Vec::new();
                                                        for el_a37bca54 in &read_only_ids_from_try_delete_many {
                                                            acc_87ea12c9.push(#primary_key_field_type_where_ts::Equal(pg_crud::PgTypeWhereEqual {
                                                                logical_operator: pg_crud::LogicalOperator::Or,
                                                                #ValueSc: #primary_key_field_type_table_type_declaration_ts::new(
                                                                    <#primary_key_field_type as pg_crud::PgType>::into_inner(el_a37bca54.clone())
                                                                ),
                                                            }));
                                                        }
                                                        acc_87ea12c9
                                                    }
                                                )
                                            )
                                        ),
                                        #select_default_all_with_max_page_size_cloned_clone_ts,
                                        &table_create_many_cloned
                                    ).await
                                    .expect("24ab86d6")
                                    .is_empty(),
                                    "4e88679a"
                                );
                            }));
                        }
                    }
                },
            );
            quote! {#create_many_tests_ts}
        };
        let create_one_tests_ts = {
            let create_one_tests_ts = gen_fields_named_without_primary_key_without_comma_ts(
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let ident_create_ts_f75e4ef0 =
                        gen_ident_create_content_el_id_ts(field_ident, &quote! {el_7632d698});
                    let value_init_ts = gen_import_path_value_init_ts(&primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts);
                    quote! {
                        for el_7632d698 in <#field_type as pg_crud::PgTypeTestCases>::#OptionVecCreateSc().unwrap_or(Vec::new()) {
                            let table_create_one_cloned = table_create_one.clone();
                            let url_cloned = url.clone();
                            let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                            acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                let ident_create = #ident_create_ucc {
                                    #ident_create_ts_f75e4ef0
                                };
                                let read_only_ids_from_try_create_one = gen_read_only_ids_from_try_create_one(
                                    &url_cloned,
                                    ident_create.clone(),
                                    &table_create_one_cloned
                                ).await;
                                assert_eq!(
                                    #ident_read_ucc {
                                        #primary_key_field_ident: Some(#value_init_ts),
                                        #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_from_try_create_one_ident_create_ts
                                    },
                                    gen_ident_try_read_one_handle_primary_key(
                                        &url_cloned,
                                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                                        #select_default_all_with_max_page_size_cloned_clone_ts,
                                        &table_create_one_cloned
                                    )
                                    .await
                                    .expect("f8e1cb88"),
                                    "5f2adbed"
                                );
                                assert_eq!(
                                    gen_try_delete_one_handle(
                                        &url_cloned,
                                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                                        &table_create_one_cloned
                                    ).await.expect("20d5a40a"),
                                    #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                                    "4f563faf"
                                );
                                gen_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                                    &url_cloned,
                                    #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                                    select_default_all_with_max_page_size_cloned,
                                    &table_create_one_cloned,
                                ).await;
                            }));
                        }
                    }
                },
            );
            quote! {#create_one_tests_ts}
        };
        let add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts =
            |ts: &dyn ToTokens| {
                quote! {
                    let read_only_ids_from_try_create_one = gen_read_only_ids_from_try_create_one_default(
                        &url_cloned,
                        &table_7e35b1ce
                    ).await;
                    #ts
                    let _: #primary_key_field_type_as_pg_type_read_ts = gen_try_delete_one_handle(
                        &url_cloned,
                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                        &table_7e35b1ce
                    ).await.expect("93b4bf61");
                    gen_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                        &url_cloned,
                        #primary_key_field_type_read_only_ids_into_read_read_only_ids_from_try_create_one_primary_key_field_ident_ts,
                        select_default_all_with_max_page_size_cloned,
                        &table_7e35b1ce,
                    ).await;
                }
            };
        let read_many_tests_ts = {
            //todo additional read_many checks
            let test_read_many_by_non_existent_primary_keys_ts = {
                let ts = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                    quote! {
                        assert!(
                            gen_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                gen_ident_where_many_pripery_key_others_none(
                                    Some(
                                        gen_pg_type_where_try_new_primary_key(
                                            pg_crud::LogicalOperator::Or,
                                            std::iter::repeat_with(|| #primary_key_field_type_as_pg_type_where_ts::Equal(
                                                pg_crud::PgTypeWhereEqual {
                                                    logical_operator: pg_crud::LogicalOperator::Or,
                                                    #ValueSc: #primary_key_field_type_table_type_declaration_ts::new(
                                                        uuid::Uuid::new_v4()
                                                    )
                                                }
                                            ))
                                            .take(el_30614c66)
                                            .collect::<Vec<_>>()
                                        )
                                    )
                                ),
                                select_default_all_with_max_page_size_cloned.clone(),
                                &table_7e35b1ce
                            ).await
                            .expect("e661c49b")
                            .is_empty(),
                            "06df4025"
                        );
                    }
                });
                quote! {
                    for el_30614c66 in [1,2] {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                        let table_7e35b1ce = table_test_read_many_by_non_existent_primary_keys.clone();
                        acc_9189f86e.push(futures::FutureExt::boxed(async move {
                            #ts
                        }));
                    }
                }
            };
            let test_read_many_by_equal_to_created_primary_keys_ts = {
                let ts = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                    quote! {
                        let ident_vec_create = std::iter::repeat_n(
                            ident_create_default_cloned.clone(),//todo maybe remove
                            el_a636d084
                        ).collect::<Vec<#ident_create_ucc>>();
                        let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                            &url_cloned,
                            #ident_create_many_parameters_ucc {
                                payload: #ident_create_many_payload_ucc(ident_vec_create.clone())
                            },
                            &table_7e35b1ce
                        ).await.expect("d775179f");
                        assert_eq!(
                            gen_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create(
                                read_only_ids_from_try_create_many.clone(),
                                ident_vec_create.clone()
                            ),
                            gen_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                gen_ident_where_many_pripery_key_others_none(
                                    Some(
                                        gen_pg_type_where_try_new_primary_key(
                                            pg_crud::LogicalOperator::Or,
                                            read_only_ids_from_try_create_many.iter().map(|el_adf2b4c4| {
                                                #primary_key_field_type_where_ts::Equal(
                                                    pg_crud::PgTypeWhereEqual {
                                                        logical_operator: pg_crud::LogicalOperator::Or,
                                                        #ValueSc: #primary_key_field_type_table_type_declaration_ts::new(
                                                            <#primary_key_field_type as pg_crud::PgType>::into_inner(
                                                                #primary_key_field_type_read_only_ids_into_read_el_adf2b4c4_primary_key_field_ident_ts
                                                            )
                                                        ),
                                                    },
                                                )
                                            }).collect()
                                        )
                                    )
                                ),
                                select_default_all_with_max_page_size_cloned.clone(),
                                &table_7e35b1ce
                            ).await.expect("b8efe770"),
                            "er 3b2cf1f5-2c4e-4908-ba66-f4af84fe0893"
                        );
                        let read_only_ids_from_try_delete_many = itertools::Itertools::sorted(
                            #ident::try_delete_many_handle(
                                &url_cloned,
                                #ident_delete_many_parameters_ucc {
                                    payload: #ident_delete_many_payload_ucc {
                                        where_many: #option_ident_where_many_ucc(Some(
                                            #ident_where_many_ucc {
                                                #primary_key_field_ident: Some(gen_pg_type_where_try_new_or_primary_keys(&read_only_ids_from_try_create_many)),
                                                #option_ident_where_many_ts_dc1232c7
                                            }
                                        )),
                                    },
                                },
                                &table_7e35b1ce
                            )
                            .await
                            .expect("d5c23a9d")
                            .into_iter()
                        ).collect::<Vec<<#primary_key_field_type as pg_crud::PgType>::Read>>();
                        assert_eq!(
                            read_only_ids_from_try_delete_many,
                            itertools::Itertools::sorted(
                                read_only_ids_from_try_create_many
                                .into_iter()
                                .map(|el_fdc88812| {
                                    #primary_key_field_type_read_only_ids_into_read_el_fdc88812_primary_key_field_ident_ts
                                }).collect::<Vec<<#primary_key_field_type as pg_crud::PgType>::Read>>()
                                .into_iter()
                            ).collect::<Vec<<#primary_key_field_type as pg_crud::PgType>::Read>>(),
                            "ebbbea6e"
                        );
                        assert!(
                            gen_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                gen_ident_where_many_pripery_key_others_none(
                                    Some(
                                        gen_pg_type_where_try_new_primary_key(
                                            pg_crud::LogicalOperator::Or,
                                            read_only_ids_from_try_delete_many
                                            .iter()
                                            .map(|el_1e9c87ce| #primary_key_field_type_where_ts::Equal(
                                                pg_crud::PgTypeWhereEqual {
                                                    logical_operator: pg_crud::LogicalOperator::Or,
                                                    #ValueSc: #primary_key_field_type_table_type_declaration_ts::new(
                                                        <#primary_key_field_type as pg_crud::PgType>::into_inner(
                                                            el_1e9c87ce.clone()
                                                        )
                                                    ),
                                                },
                                            ))
                                            .collect()
                                        )
                                    )
                                ),
                                select_default_all_with_max_page_size_cloned.clone(),
                                &table_7e35b1ce
                            ).await
                            .expect("1f079962")
                            .is_empty(),
                            "d79c0af3"
                        );
                    }
                });
                quote! {
                    for el_a636d084 in [1,2] {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                        let table_7e35b1ce = table_test_read_many_by_equal_to_created_primary_keys.clone();
                        let ident_create_default_cloned = ident_create_default.clone();
                        acc_9189f86e.push(futures::FutureExt::boxed(async move {
                            #ts
                        }));
                    }
                }
            };
            let gen_read_only_ids_merged_with_create_into_where_assert_eq_ts =
                |ident_where_many_try_new_parameters_ts: &dyn ToTokens| {
                    quote! {
                        assert_eq!(
                            vec![
                                #ident_read_ucc {
                                    #primary_key_field_ident: <
                                        #primary_key_field_type
                                        as
                                        pg_crud::PgTypeTestCases
                                    >::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                        &read_only_ids_returned_from_create_one.#primary_key_field_ident
                                    ),
                                    #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_clone_ident_create_clone_ts
                                }
                            ],
                            gen_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                #ident_where_many_ucc::try_new(#ident_where_many_try_new_parameters_ts).expect("83c2d430"),
                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                &table_7e35b1ce
                            ).await.expect("c3e316c0"),
                            "ee8d232d"
                        );
                    }
                };
            let gen_option_vec_create_call_unwrap_or_vec_ts = |_: &Ident, field_type: &Type| {
                quote! {
                    <#field_type as pg_crud::PgTypeTestCases>::#OptionVecCreateSc().unwrap_or(Vec::new())
                }
            };
            let gen_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts =
                |field_ident: &Ident, field_type: &Type| {
                    quote! {
                        <#field_type as pg_crud::PgTypeTestCases>::#OptionVecCreateSc()
                        .filter(|el_bba28182| !el_bba28182.is_empty())
                        .unwrap_or_else(|| vec![ident_create_default.#field_ident.clone()])
                    }
                };
            let gen_pg_type_option_vec_where_greater_than_test_unwrap_or_else_vec_call_ts =
                |_: &Ident, field_type: &Type| {
                    quote! {
                        <#field_type as #import_path::PgTypeTestCases>::#PgTypeOptionVecWhereGreaterThanTestSc()
                        .map_or_else(Vec::new, Into::into)
                    }
                };
            let gen_read_test_ts =
                |test_name: &str,
                 gen_method_call_ts: &dyn Fn(&Ident, &Type) -> Ts2,
                 gen_create_ts: &dyn Fn(&Ident) -> Ts2,
                 gen_ts: &dyn Fn(&SynFieldWrapper) -> Ts2| {
                    gen_fields_named_without_primary_key_without_comma_ts(
                        &|element: &SynFieldWrapper| {
                            let field_ident = &element.field_ident;
                            let field_type = &element.field_type;
                            let method_call_ts = gen_method_call_ts(field_ident, field_type);
                            let table_test_name_field_ident_ts =
                                gen_table_test_name_field_ident_ts(test_name, field_ident);
                            let ident_create_ts_013035e1 = gen_create_ts(field_ident);
                            let ts = gen_ts(element);
                            quote! {
                                for #ElementSc in #method_call_ts {
                                    let table_7e35b1ce = #table_test_name_field_ident_ts.clone();
                                    let url_cloned = url.clone();
                                    let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                                    acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                        let ident_create = #ident_create_ucc {
                                            #ident_create_ts_013035e1
                                        };
                                        let read_only_ids_returned_from_create_one = gen_read_only_ids_from_try_create_one(
                                            &url_cloned,
                                            ident_create.clone(),
                                            &table_7e35b1ce
                                        ).await;
                                        #ts
                                        let read_only_ids_from_try_delete_many = itertools::Itertools::sorted(
                                            #ident::try_delete_many_handle(
                                                &url_cloned,
                                                #ident_delete_many_parameters_ucc {
                                                    payload: #ident_delete_many_payload_ucc {
                                                        where_many: #option_ident_where_many_ucc(Some(
                                                            #ident_where_many_ucc {
                                                                #primary_key_field_ident: Some(
                                                                    gen_pg_type_where_try_new_primary_key(
                                                                        pg_crud::LogicalOperator::Or,
                                                                        vec![
                                                                            #primary_key_field_type_where_ts::Equal(
                                                                                pg_crud::PgTypeWhereEqual {
                                                                                    logical_operator: pg_crud::LogicalOperator::Or,
                                                                                    #ValueSc: #primary_key_field_type_table_type_declaration_ts::new(
                                                                                        <#primary_key_field_type as pg_crud::PgType>::into_inner(
                                                                                            #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts
                                                                                        )
                                                                                    )
                                                                                }
                                                                            )
                                                                        ]
                                                                    )
                                                                ),
                                                                #option_ident_where_many_ts_dc1232c7
                                                            }
                                                        )),
                                                    },
                                                },
                                                &table_7e35b1ce
                                            )
                                            .await
                                            .expect("338bcf89")
                                            .into_iter()
                                        ).collect::<Vec<<#primary_key_field_type as pg_crud::PgType>::Read>>();
                                        assert_eq!(
                                            read_only_ids_from_try_delete_many,
                                            vec![#primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts],
                                            "9fc29fa5"
                                        );
                                        assert!(
                                            gen_try_read_many_order_by_primary_key_with_big_pagination(
                                                &url_cloned,
                                                gen_ident_where_many_pripery_key_others_none(
                                                    Some(
                                                        gen_pg_type_where_try_new_primary_key(
                                                            pg_crud::LogicalOperator::Or,
                                                            vec![
                                                                #primary_key_field_type_where_ts::Equal(pg_crud::PgTypeWhereEqual {
                                                                    logical_operator: pg_crud::LogicalOperator::Or,
                                                                    #ValueSc: #primary_key_field_type_table_type_declaration_ts::new(
                                                                        <#primary_key_field_type as pg_crud::PgType>::into_inner(
                                                                            #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts
                                                                        )
                                                                    )
                                                                })
                                                            ]
                                                        )
                                                    )
                                                ),
                                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                                &table_7e35b1ce
                                            ).await
                                            .expect("1817b67a")
                                            .is_empty(),
                                            "38187925"
                                        );
                                    }));
                                }
                            }
                        },
                    )
                };
            let some_primary_key_where_init_ts = quote! {
                Some(
                    gen_pg_type_where_try_new_primary_key(
                        pg_crud::LogicalOperator::And,
                        vec![
                            <#primary_key_field_type as pg_crud::PgTypeTestCases>::read_only_ids_merged_with_create_into_where_equal(
                                read_only_ids_returned_from_create_one.#primary_key_field_ident,
                                #PgCrudDefaultOptionSomeVecOneElCall
                            )
                        ]
                    )
                )
            };
            let (
                read_only_ids_merged_with_create_into_where_equal_ts,
                read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
            ) = {
                let gen_test_read_many_by_equal_one_column_value_ts =
                    |test_name: &str, equal_or_equal_using_fields: &EqualOrEqualUsingFields| {
                        gen_read_test_ts(
                            test_name,
                            &gen_option_vec_create_call_unwrap_or_vec_ts,
                            &gen_ident_create_content_el_ts,
                            &|element: &SynFieldWrapper| {
                                let field_ident = &element.field_ident;
                                gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                                    &gen_fields_named_with_comma_ts(
                                        &|el_ac437d52: &SynFieldWrapper| {
                                            let field_ident_4fdece29 = &el_ac437d52.field_ident;
                                            let field_type_5f626ae9 = &el_ac437d52.field_type;
                                            if field_ident_4fdece29 == primary_key_field_ident {
                                                some_primary_key_where_init_ts.clone()
                                            } else if field_ident_4fdece29 == field_ident {
                                                let method_ts = {
                                                    let method_ts: &dyn ToTokens = match &equal_or_equal_using_fields {
                                            EqualOrEqualUsingFields::Equal => &ReadOnlyIdsMergedWithCreateIntoWhereEqualSc,
                                            EqualOrEqualUsingFields::EqualUsingFields => &ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc
                                        };
                                                    quote! {
                                                        <#field_type_5f626ae9 as pg_crud::PgTypeTestCases>::#method_ts(
                                                            read_only_ids_returned_from_create_one.#field_ident_4fdece29.clone().expect("11c3740b"),
                                                            ident_create.#field_ident_4fdece29.clone()
                                                        )
                                                    }
                                                };
                                                match &equal_or_equal_using_fields {
                                                    EqualOrEqualUsingFields::Equal => {
                                                        gen_some_pg_type_where_try_new_and_ts(
                                                            &quote! {
                                                                vec![#method_ts]
                                                            },
                                                        )
                                                    }
                                                    EqualOrEqualUsingFields::EqualUsingFields => {
                                                        quote! {
                                                            Some(#import_path::PgTypeWhere::new(
                                                                #import_path::LogicalOperator::And,
                                                                #method_ts
                                                            ))
                                                        }
                                                    }
                                                }
                                            } else {
                                                none_ts.clone()
                                            }
                                        },
                                    ),
                                )
                            },
                        )
                    };
                (
                    gen_test_read_many_by_equal_one_column_value_ts(table_read_only_ids_merged_with_create_into_where_equal_name, &EqualOrEqualUsingFields::Equal),
                    gen_test_read_many_by_equal_one_column_value_ts(table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name, &EqualOrEqualUsingFields::EqualUsingFields),
                )
            };
            let read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts = gen_read_test_ts(table_read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_name, &gen_option_vec_create_call_unwrap_or_vec_ts, &gen_ident_create_content_el_ts, &|element: &SynFieldWrapper| {
                let field_ident = &element.field_ident;
                let field_type = &element.field_type;
                let assert_eq_ts = gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(&gen_fields_named_with_comma_ts(&|el_97d8a089: &SynFieldWrapper| {
                    let field_ident_4557eb3f = &el_97d8a089.field_ident;
                    if field_ident_4557eb3f == primary_key_field_ident {
                        some_primary_key_where_init_ts.clone()
                    } else if field_ident_4557eb3f == field_ident {
                        gen_some_pg_type_where_try_new_and_ts(&quote! {vec![el_48a3d976]})
                    } else {
                        none_ts.clone()
                    }
                }));
                quote! {
                    if let Some(v_d5cd3c70) = <#field_type as pg_crud::PgTypeTestCases>::#ReadOnlyIdsMergedWithCreateIntoOptionVecWhereEqualToJsonFieldSc(
                        read_only_ids_returned_from_create_one.#field_ident.clone().expect("65cef584"),
                        ident_create.#field_ident.clone()
                    ) {
                        for el_48a3d976 in v_d5cd3c70.into_vec() {
                            #assert_eq_ts
                        }
                    }
                }
            });
            let create_into_pg_type_option_vec_where_dim_one_equal_ts = gen_read_test_ts(
                table_create_into_pg_type_option_vec_where_dim_one_equal_name,
                &gen_option_vec_create_call_unwrap_or_vec_ts,
                &gen_ident_create_content_el_ts,
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let assert_eq_ts = gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                        &gen_fields_named_with_comma_ts(&|el_483e5312: &SynFieldWrapper| {
                            let field_ident_65c48d59 = &el_483e5312.field_ident;
                            if primary_key_field_ident == field_ident_65c48d59 {
                                some_primary_key_where_init_ts.clone()
                            } else if field_ident_65c48d59 == field_ident {
                                gen_some_pg_type_where_try_new_and_ts(&quote! {vec![el_39d1fb5d]})
                            } else {
                                none_ts.clone()
                            }
                        }),
                    );
                    quote! {
                        if let Some(v_b02d763d) = <#field_type as pg_crud::PgTypeTestCases>::#CreateIntoPgTypeOptionVecWhereDimOneEqualSc(
                            ident_create.#field_ident.clone()
                        ) {
                            for el_39d1fb5d in v_b02d763d.into_vec() {
                                #assert_eq_ts
                            }
                        }
                    }
                },
            );
            let read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_ts = gen_read_test_ts(
                table_read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_name,
                &gen_pg_type_option_vec_where_greater_than_test_unwrap_or_else_vec_call_ts,
                &|field_ident: &Ident| {
                    gen_ident_create_ts(
                        field_ident,
                        &quote! {#ElementSc.#CreateSc},
                    )
                },
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let assert_eq_ts = gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(&gen_fields_named_with_comma_ts(&|el_a8bfc0c0: &SynFieldWrapper| {
                        let field_ident_79ff8c6e = &el_a8bfc0c0.field_ident;
                        if field_ident_79ff8c6e == primary_key_field_ident {
                            some_primary_key_where_init_ts.clone()
                        } else if field_ident_79ff8c6e == field_ident {
                            gen_some_pg_type_where_try_new_and_ts(&quote! {vec![v_60baba1f]})
                        } else {
                            none_ts.clone()
                        }
                    }));
                    quote! {
                        if let Some(v_60baba1f) = <#field_type as pg_crud::PgTypeTestCases>::#ReadOnlyIdsMergedWithTableTypeDeclarationIntoPgTypeOptionWhereGreaterThanSc(
                            #ElementSc.variant,
                            read_only_ids_returned_from_create_one.#field_ident.clone().expect("c8d34556"),
                            #ElementSc.greater_than,
                        ) {
                            #assert_eq_ts
                        }
                    }
                },
            );
            let (
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts,
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts,
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts,
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts,
            ) = {
                //todo if vec_create is empty then do different logic (for uuid). now uuid Tested using one default case
                let gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts =
                    |test_name: &str, dim: &Dim| {
                        let read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc = dim.read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc();
                        gen_read_test_ts(test_name, &gen_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts, &gen_ident_create_content_el_ts, &|element: &SynFieldWrapper| {
                        let field_ident = &element.field_ident;
                        let field_type = &element.field_type;
                        let assert_eq_ts = gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(&gen_fields_named_with_comma_ts(&|el_a9b23eca: &SynFieldWrapper| {
                            let field_ident_fa2292e0 = &el_a9b23eca.field_ident;
                            if field_ident_fa2292e0 == primary_key_field_ident {
                                some_primary_key_where_init_ts.clone()
                            } else if field_ident_fa2292e0 == field_ident {
                                gen_some_pg_type_where_try_new_and_ts(&quote! {vec![el_3efa0bb4]})
                            } else {
                                none_ts.clone()
                            }
                        }));
                        quote! {
                            if let Some(v_bb67b871) = <#field_type as pg_crud::PgTypeTestCases>::#read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_sc(
                                read_only_ids_returned_from_create_one.#field_ident.clone().expect("2ed000a5"),
                                ident_create.#field_ident.clone()
                            ) {
                                for el_3efa0bb4 in v_bb67b871.into_vec() {
                                    #assert_eq_ts
                                }
                            }
                        }
                    })
                    };
                (
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts(table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_name, &Dim::One),
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts(table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_name, &Dim::Two),
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts(table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_name, &Dim::Three),
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_number_equal_ts(table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_name, &Dim::Four),
                )
            };
            let create_into_pg_json_type_option_vec_where_length_equal_ts = gen_read_test_ts(
                table_create_into_pg_json_type_option_vec_where_length_equal_name,
                &gen_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts,
                &gen_ident_create_content_el_ts,
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let assert_eq_ts = gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                        &gen_fields_named_with_comma_ts(&|el_94f00070: &SynFieldWrapper| {
                            let field_ident_bffc985c = &el_94f00070.field_ident;
                            if field_ident_bffc985c == primary_key_field_ident {
                                some_primary_key_where_init_ts.clone()
                            } else if field_ident_bffc985c == field_ident {
                                gen_some_pg_type_where_try_new_and_ts(&quote! {vec![el_c09ef321]})
                            } else {
                                none_ts.clone()
                            }
                        }),
                    );
                    quote! {
                        if let Some(v_f825e068) = <#field_type as pg_crud::PgTypeTestCases>::#CreateIntoPgJsonTypeOptionVecWhereLengthEqualSc(
                            ident_create.#field_ident.clone()
                        ) {
                            for el_c09ef321 in v_f825e068.into_vec() {
                                #assert_eq_ts
                            }
                        }
                    }
                },
            );
            let create_into_pg_json_type_option_vec_where_length_greater_than_ts = gen_read_test_ts(
                table_create_into_pg_json_type_option_vec_where_length_greater_than_name,
                &gen_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts,
                &gen_ident_create_content_el_ts,
                &|element: &SynFieldWrapper| {
                    let field_ident = &element.field_ident;
                    let field_type = &element.field_type;
                    let assert_eq_ts = gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                        &gen_fields_named_with_comma_ts(&|el_c927ab80: &SynFieldWrapper| {
                            let field_ident_9464d51b = &el_c927ab80.field_ident;
                            if field_ident_9464d51b == primary_key_field_ident {
                                some_primary_key_where_init_ts.clone()
                            } else if field_ident_9464d51b == field_ident {
                                gen_some_pg_type_where_try_new_and_ts(&quote! {vec![el_527b546b]})
                            } else {
                                none_ts.clone()
                            }
                        }),
                    );
                    quote! {
                        if let Some(v_cd4aa374) = <#field_type as pg_crud::PgTypeTestCases>::#CreateIntoPgJsonTypeOptionVecWhereLengthGreaterThanSc(
                            ident_create.#field_ident.clone()
                        ) {
                            for el_527b546b in v_cd4aa374.into_vec() {
                                #assert_eq_ts
                            }
                        }
                    }
                },
            );
            let (
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts,
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts,
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts,
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts,
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts,
                read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts,
            ) = {
                let gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts =
                    |table_name: &str, method_ts: &dyn ToTokens| {
                        gen_read_test_ts(
                    table_name,
                    &gen_option_vec_create_call_unwrap_or_vec_ident_create_default_field_ident_clone_ts,
                    &gen_ident_create_content_el_ts,
                    &|element: &SynFieldWrapper|{
                        let field_ident = &element.field_ident;
                        let field_type = &element.field_type;
                        let assert_eq_ts = gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                            &gen_fields_named_with_comma_ts(&|el_16b8a9cc: &SynFieldWrapper|{
                                let field_ident_6b16d591 = &el_16b8a9cc.field_ident;
                                if field_ident_6b16d591 == primary_key_field_ident {
                                    some_primary_key_where_init_ts.clone()
                                }
                                else if field_ident_6b16d591 == field_ident {
                                    gen_some_pg_type_where_try_new_and_ts(&quote!{match el_feacc53b {
                                        #import_path::SingleOrMultiple::Multiple(multiple) => multiple.into_vec().into_iter().collect(),
                                        #import_path::SingleOrMultiple::Single(single) => std::iter::once(single).collect(),
                                    }})
                                } else {
                                    none_ts.clone()
                                }
                            })
                        );
                        quote!{
                            if let Some(v_0b85c066) = <#field_type as pg_crud::PgTypeTestCases>::#method_ts(
                                read_only_ids_returned_from_create_one.#field_ident.clone().expect("df01c8ac"),
                                ident_create.#field_ident.clone()
                            ) {
                                for el_feacc53b in v_0b85c066.into_vec() {
                                    #assert_eq_ts
                                }
                            }
                        }
                    }
                )
                    };
                (
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereGreaterThanSc
                    ),
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereBetweenSc
                    ),
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereInSc
                    ),
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereRegularExpressionSc
                    ),
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElGreaterThanSc
                    ),
                    gen_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_filter_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptionVecWhereContainsElRegularExpressionSc
                    )
                )
            };
            quote! {
                #test_read_many_by_non_existent_primary_keys_ts
                #test_read_many_by_equal_to_created_primary_keys_ts
                #read_only_ids_merged_with_create_into_where_equal_ts
                #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts
                #read_only_ids_merged_with_create_into_option_vec_where_equal_to_json_field_ts
                #create_into_pg_type_option_vec_where_dim_one_equal_ts
                #read_only_ids_merged_with_table_type_declaration_into_pg_type_option_where_greater_than_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_one_equal_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_two_equal_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_three_equal_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_dim_four_equal_ts
                #create_into_pg_json_type_option_vec_where_length_equal_ts
                #create_into_pg_json_type_option_vec_where_length_greater_than_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_greater_than_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_between_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_in_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_regular_expression_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_greater_than_ts
                #read_only_ids_merged_with_create_into_pg_json_type_option_vec_where_contains_el_regular_expression_ts
            }
        };
        let read_one_tests_ts = quote! {
            acc_9189f86e.push({
                let table_read_one_cloned = table_read_one.clone();
                let url_cloned = url.clone();
                let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                futures::FutureExt::boxed(async move {
                    gen_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                        &url_cloned,
                        #primary_key_field_type_as_pg_type_read_ts::new(uuid::Uuid::new_v4()),
                        #select_default_all_with_max_page_size_cloned_clone_ts,
                        &table_read_one_cloned,
                    ).await;
                })
            });
        };
        let update_many_tests_ts = {
            //todo add Test for trying to update empty vec
            let update_many_only_one_column_tests_ts =
                gen_fields_named_without_primary_key_without_comma_ts(
                    &|el_94a9ca95: &SynFieldWrapper| {
                        let field_ident = &el_94a9ca95.field_ident;
                        let field_type = &el_94a9ca95.field_type;
                        let is_fields_without_primary_key_len_greater_than_one =
                            fields_without_primary_key.len() > 1;
                        let maybe_previous_read_ts =
                            if is_fields_without_primary_key_len_greater_than_one {
                                quote! {
                                    let previous_read = itertools::Itertools::sorted_by(
                                        gen_try_read_many_order_by_primary_key_with_big_pagination(
                                            &url_cloned,
                                            gen_ident_where_many_pripery_key_others_none(
                                                Some(
                                                    gen_pg_type_where_try_new_primary_key(
                                                        pg_crud::LogicalOperator::Or,
                                                        vec![
                                                            #primary_key_field_type_as_pg_type_where_ts::Equal(
                                                                pg_crud::PgTypeWhereEqual {
                                                                    logical_operator: pg_crud::LogicalOperator::Or,
                                                                    value: #primary_key_field_type_table_type_declaration_ts::new(
                                                                        #primary_key_field_type_as_pg_type_ts into_inner(
                                                                            #primary_key_field_type_read_only_is_into_read_read_only_ids_el_primary_key_field_ident_ts_937c5af3
                                                                        )
                                                                    ),
                                                                }
                                                            )
                                                        ]
                                                    )
                                                )
                                            ),
                                            #select_default_all_with_max_page_size_cloned_clone_ts,
                                            &table_update_many_cloned
                                        )
                                        .await
                                        .expect("540ec737")
                                        .into_iter(),
                                        |first, second| {
                                            match (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                                (Some(first_handle), Some(second_handle)) => first_handle.#ValueSc.cmp(&second_handle.#ValueSc),
                                                _ => panic!("99ba9dc3"),
                                            }
                                        }
                                    );
                                }
                            } else {
                                Ts2::new()
                            };
                        let field_ident_read_only_ids_to_two_dimal_vec_read_inner_acc_sc =
                            SelfReadOnlyIdsToTwoDimalVecReadInnerAccSc::from_tokens(&field_ident);
                        let ident_read_only_ids_upper_fields_init_without_primary_key_ts =
                            gen_fields_named_without_primary_key_with_comma_ts(
                                &|syn_field_wrapper: &SynFieldWrapper| {
                                    let field_ident_867b4800 = &syn_field_wrapper.field_ident;
                                    let field_type_42b4a146 = &syn_field_wrapper.field_type;
                                    if field_ident == field_ident_867b4800 {
                                        quote! {#field_ident_867b4800: Some(<#field_type_42b4a146 as pg_crud::PgTypeTestCases>::update_to_read_only_ids(&update))}
                                    } else {
                                        quote! {#field_ident_867b4800: None}
                                    }
                                },
                            );
                        let ident_update_parameters_init_without_primary_key_ts =
                            gen_fields_named_without_primary_key_with_comma_ts(
                                &|syn_field_wrapper: &SynFieldWrapper| {
                                    let field_ident_f56e8e3f = &syn_field_wrapper.field_ident;
                                    if field_ident == field_ident_f56e8e3f {
                                        let value_init_ts =
                                            gen_import_path_value_init_ts(&quote! {
                                                #UpdateSc.clone()
                                            });
                                        quote! {Some(#value_init_ts)}
                                    } else {
                                        none_ts.clone()
                                    }
                                },
                            );
                        let ident_read_fields_init_without_primary_key_after_update_one_ts =
                            gen_fields_named_without_primary_key_with_comma_ts(
                                &|syn_field_wrapper: &SynFieldWrapper| {
                                    let field_ident_b9ec9008 = &syn_field_wrapper.field_ident;
                                    if field_ident == field_ident_b9ec9008 {
                                        let value_init_ts = gen_import_path_value_init_ts(&{
                                            let field_type_0490079a = &syn_field_wrapper.field_type;
                                            quote! {
                                                <#field_type_0490079a as pg_crud::PgTypeTestCases>::previous_read_merged_with_option_update_into_read(
                                                    <#field_type_0490079a as pg_crud::PgTypeTestCases>::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                        &read_only_ids_element_937c5af3.#field_ident_b9ec9008.clone().expect("96213542")
                                                    ).expect("bf0d6f55").#ValueSc,
                                                    Some(#UpdateSc.clone())
                                                )
                                            }
                                        });
                                        quote! {
                                            #field_ident_b9ec9008: Some(#value_init_ts)
                                        }
                                    } else {
                                        quote! {
                                            #field_ident_b9ec9008: el_a6bc6b2f.#field_ident_b9ec9008
                                        }
                                    }
                                },
                            );
                        let expected_read_many_ts =
                            if is_fields_without_primary_key_len_greater_than_one {
                                let value_init_ts = gen_import_path_value_init_ts(&primary_key_field_type_read_only_is_into_read_read_only_ids_el_primary_key_field_ident_ts_937c5af3);
                                quote! {
                                    previous_read.into_iter().map(|el_a6bc6b2f| #ident_read_ucc {
                                        #primary_key_field_ident: Some(#value_init_ts),
                                        #ident_read_fields_init_without_primary_key_after_update_one_ts
                                    }).collect::<Vec<#ident_read_ucc>>()
                                }
                            } else {
                                let value_init_ts = gen_import_path_value_init_ts(&primary_key_field_type_read_only_is_into_read_read_only_ids_el_primary_key_field_ident_ts_937c5af3);
                                quote! {
                                    vec![
                                        #ident_read_ucc {
                                            #primary_key_field_ident: Some(#value_init_ts),
                                            #ident_read_fields_init_without_primary_key_after_update_one_ts
                                        }
                                    ]
                                }
                            };
                        quote! {
                            for (index_7f181188, read_only_ids_element_937c5af3) in gen_read_only_ids_elements_8a1ef027(
                                &url,
                                &table_update_many,
                                #select_default_all_with_max_page_size_clone_ts,
                                #field_ident_read_only_ids_to_two_dimal_vec_read_inner_acc_sc.clone()
                            ).await.into_iter().enumerate() {
                                let table_update_many_cloned = table_update_many.clone();
                                let url_cloned = url.clone();
                                let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                                acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                    #maybe_previous_read_ts
                                    let update = <
                                        #field_type
                                        as
                                        pg_crud::PgTypeTestCases
                                    >::read_inner_into_update_with_new_or_try_new_unwraped({
                                        let mut index_e0c50b3e: usize = 0;
                                        let mut option_test_case = None;
                                        for el_76abae3a in <#field_type as pg_crud::PgTypeTestCases>::read_only_ids_to_two_dimal_vec_read_inner(
                                            &read_only_ids_element_937c5af3.#field_ident.clone().expect("af7d979d")
                                        ) {
                                            let mut should_break = false;
                                            for el_72f5ad12 in el_76abae3a {
                                                if index_e0c50b3e == index_7f181188 {
                                                    option_test_case = Some(el_72f5ad12);
                                                    should_break = true;
                                                    break;
                                                }
                                                index_e0c50b3e = index_e0c50b3e.checked_add(1).expect("0968dda6");
                                            }
                                            if should_break {
                                                break;
                                            }
                                        }
                                        option_test_case.expect("769983ba")
                                    });
                                    assert_eq!(
                                        vec![
                                            #ident_read_only_ids_ucc {
                                                #primary_key_field_ident: read_only_ids_element_937c5af3.#primary_key_field_ident,
                                                #ident_read_only_ids_upper_fields_init_without_primary_key_ts
                                            }
                                        ],
                                        #ident::try_update_many_handle(
                                            &url_cloned,
                                            #ident_update_many_parameters_ucc {
                                                payload: #ident_update_many_payload_ucc::try_new(vec![
                                                    #ident_update_ucc::try_new(
                                                        #primary_key_field_type_as_pg_type_update_as_pg_type_primary_key_read_only_ids_into_update_ts,
                                                        #ident_update_parameters_init_without_primary_key_ts
                                                    ).expect("42dc87b3")
                                                ]).expect("69e1bd8a")
                                            },
                                            &table_update_many_cloned
                                        ).await.expect("d2de0bd6"),
                                        "34bfb3c7"
                                    );
                                    assert_eq!(
                                        {
                                            #expected_read_many_ts
                                        },
                                        itertools::Itertools::sorted_by(
                                            gen_try_read_many_order_by_primary_key_with_big_pagination(
                                                &url_cloned,
                                                gen_ident_where_many_pripery_key_others_none(
                                                    Some(
                                                        gen_pg_type_where_try_new_primary_key(
                                                            pg_crud::LogicalOperator::Or,
                                                            vec![
                                                                #primary_key_field_type_where_ts::Equal(
                                                                    pg_crud::PgTypeWhereEqual {
                                                                        logical_operator: pg_crud::LogicalOperator::Or,
                                                                        #ValueSc: #primary_key_field_type_table_type_declaration_ts::new(
                                                                            <#primary_key_field_type as pg_crud::PgType>::into_inner(
                                                                                #primary_key_field_type_read_only_is_into_read_read_only_ids_el_primary_key_field_ident_ts_937c5af3
                                                                            )
                                                                        ),
                                                                    }
                                                                )
                                                            ]
                                                        )
                                                    )
                                                ),
                                                select_default_all_with_max_page_size_cloned,
                                                &table_update_many_cloned
                                            )
                                            .await
                                            .expect("25c561e2")
                                            .into_iter(),
                                            |first, second| match (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                                (Some(first_handle), Some(second_handle)) => first_handle.#ValueSc.cmp(&second_handle.#ValueSc),
                                                _ => panic!("3c827ad6"),
                                            }
                                        ).collect::<Vec<#ident_read_ucc>>(),
                                        "ae2a2da5"
                                    );
                                }));
                            }
                        }
                    },
                );
            quote! {#update_many_only_one_column_tests_ts}
        };
        let update_one_tests_ts = {
            let update_one_only_one_column_tests_ts =
                gen_fields_named_without_primary_key_without_comma_ts(
                    &|el_d82ff77f: &SynFieldWrapper| {
                        let field_ident = &el_d82ff77f.field_ident;
                        let field_type = &el_d82ff77f.field_type;
                        let maybe_previous_read_ts = if fields_without_primary_key.len() > 1 {
                            quote! {
                                let previous_read = gen_ident_try_read_one_handle_primary_key(
                                    &url_cloned,
                                    #primary_key_field_type_read_only_is_into_read_read_only_ids_el_primary_key_field_ident_ts_937c5af3,
                                    #select_default_all_with_max_page_size_cloned_clone_ts,
                                    &table_update_one_cloned
                                )
                                .await.expect("e6998b47");
                            }
                        } else {
                            Ts2::new()
                        };
                        let field_ident_read_only_ids_to_two_dimal_vec_read_inner_acc_sc =
                            SelfReadOnlyIdsToTwoDimalVecReadInnerAccSc::from_tokens(&field_ident);
                        let ident_read_only_ids_upper_fields_init_without_primary_key_ts =
                            gen_fields_named_without_primary_key_with_comma_ts(
                                &|element: &SynFieldWrapper| {
                                    let field_ident_6b82a8d0 = &element.field_ident;
                                    let field_type_ad043276 = &element.field_type;
                                    if field_ident == field_ident_6b82a8d0 {
                                        quote! {#field_ident_6b82a8d0: Some(<#field_type_ad043276 as pg_crud::PgTypeTestCases>::update_to_read_only_ids(&update))}
                                    } else {
                                        quote! {#field_ident_6b82a8d0: None}
                                    }
                                },
                            );
                        let ident_update_parameters_init_without_primary_key_ts =
                            gen_fields_named_without_primary_key_with_comma_ts(
                                &|element: &SynFieldWrapper| {
                                    let field_ident_6ea8afd1 = &element.field_ident;
                                    if field_ident == field_ident_6ea8afd1 {
                                        let value_init_ts = gen_import_path_value_init_ts(
                                            &quote! {#UpdateSc.clone()},
                                        );
                                        quote! {Some(#value_init_ts)}
                                    } else {
                                        none_ts.clone()
                                    }
                                },
                            );
                        let ident_read_fields_init_without_primary_key_after_update_one_ts =
                            gen_fields_named_without_primary_key_with_comma_ts(
                                &|element: &SynFieldWrapper| {
                                    let field_ident_8c7d4975 = &element.field_ident;
                                    let field_type_09e184c3 = &element.field_type;
                                    if field_ident == field_ident_8c7d4975 {
                                        let value_init_ts = gen_import_path_value_init_ts(
                                            &quote! {
                                                <
                                                    #field_type_09e184c3
                                                    as
                                                    pg_crud::PgTypeTestCases
                                                >::previous_read_merged_with_option_update_into_read(
                                                    <
                                                        #field_type_09e184c3
                                                        as
                                                        pg_crud::PgTypeTestCases
                                                    >::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                                        &read_only_ids_element_937c5af3.#field_ident_8c7d4975.clone().expect("4f19d0d2")
                                                    ).expect("c7685b19").#ValueSc,
                                                    Some(#UpdateSc.clone())
                                                )
                                            },
                                        );
                                        quote! {
                                            #field_ident_8c7d4975: Some(#value_init_ts)
                                        }
                                    } else {
                                        quote! {
                                            #field_ident_8c7d4975: previous_read.#field_ident_8c7d4975
                                        }
                                    }
                                },
                            );
                        let value_init_ts = gen_import_path_value_init_ts(&primary_key_field_type_read_only_is_into_read_read_only_ids_el_primary_key_field_ident_ts_937c5af3);
                        quote! {
                            for (index_26824592, read_only_ids_element_937c5af3) in gen_read_only_ids_elements_8a1ef027(
                                &url,
                                &table_update_one,
                                #select_default_all_with_max_page_size_clone_ts,
                                #field_ident_read_only_ids_to_two_dimal_vec_read_inner_acc_sc
                            ).await.into_iter().enumerate() {
                                let table_update_one_cloned = table_update_one.clone();
                                let url_cloned = url.clone();
                                let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                                acc_9189f86e.push(futures::FutureExt::boxed(async move {
                                    #maybe_previous_read_ts
                                    let update = <
                                        #field_type
                                        as
                                        pg_crud::PgTypeTestCases
                                    >::read_inner_into_update_with_new_or_try_new_unwraped({
                                        let mut index_e0d2f9db: usize = 0;
                                        let mut option_test_case = None;
                                        for el_3a9a65ee in <#field_type as pg_crud::PgTypeTestCases>::read_only_ids_to_two_dimal_vec_read_inner(
                                            &read_only_ids_element_937c5af3.#field_ident.clone().expect("c4d98a71")
                                        ) {
                                            let mut should_break = false;
                                            for el_bb734c11 in el_3a9a65ee {
                                                if index_e0d2f9db == index_26824592 {
                                                    option_test_case = Some(el_bb734c11);
                                                    should_break = true;
                                                    break;
                                                }
                                                index_e0d2f9db = index_e0d2f9db.checked_add(1).expect("326274d1");
                                            }
                                            if should_break {
                                                break;
                                            }
                                        }
                                        option_test_case.expect("bd79056e")
                                    });
                                    assert_eq!(
                                        #ident_read_only_ids_ucc {
                                            #primary_key_field_ident: read_only_ids_element_937c5af3.#primary_key_field_ident,
                                            #ident_read_only_ids_upper_fields_init_without_primary_key_ts
                                        },
                                        #ident::try_update_one_handle(
                                            &url_cloned,
                                            #ident_update_one_parameters_ucc {
                                                payload: #ident_update_ucc::try_new(
                                                    #primary_key_field_type_as_pg_type_update_as_pg_type_primary_key_read_only_ids_into_update_ts,
                                                    #ident_update_parameters_init_without_primary_key_ts
                                                ).expect("0e5d65a5")//todo add column ident
                                            },
                                            &table_update_one_cloned
                                        ).await.expect("4d755542"),
                                        "564de31c"
                                    );
                                    assert_eq!(
                                        #ident_read_ucc {
                                            #primary_key_field_ident: Some(#value_init_ts),
                                            #ident_read_fields_init_without_primary_key_after_update_one_ts
                                        },
                                        gen_ident_try_read_one_handle_primary_key(
                                            &url_cloned,
                                            #primary_key_field_type_read_only_is_into_read_read_only_ids_el_primary_key_field_ident_ts_937c5af3,
                                            select_default_all_with_max_page_size_cloned,
                                            &table_update_one_cloned
                                        )
                                        .await.expect("75894c76"),
                                        "d5dec823"
                                    );
                                }));
                            }
                        }
                    },
                );
            quote! {#update_one_only_one_column_tests_ts}
        };
        let delete_many_tests_ts = {
            let test_delete_many_by_non_existent_primary_keys_ts = {
                let ts = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&quote! {
                    assert!(
                        #ident::try_delete_many_handle(
                            &url_cloned,
                            #ident_delete_many_parameters_ucc {
                                payload: #ident_delete_many_payload_ucc {
                                    where_many: #option_ident_where_many_ucc(Some(#ident_where_many_ucc {
                                        #primary_key_field_ident: Some(
                                            gen_pg_type_where_try_new_primary_key(
                                                pg_crud::LogicalOperator::Or,
                                                std::iter::repeat_with(|| #primary_key_field_type_as_pg_type_where_ts::Equal(
                                                    pg_crud::PgTypeWhereEqual {
                                                        logical_operator: pg_crud::LogicalOperator::Or,
                                                        value: #primary_key_field_type_table_type_declaration_ts::new(
                                                            uuid::Uuid::new_v4()
                                                        )
                                                    }
                                                ))
                                                .take(el_39819198)
                                                .collect()
                                            )
                                        ),
                                        #fields_none_init_ts
                                    })),
                                },
                            },
                            &table_7e35b1ce
                        )
                        .await
                        .expect("0d5dec47")
                        .is_empty(),
                        "51d14103"
                    );
                });
                quote! {
                    for el_39819198 in [1,2] {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                        let table_7e35b1ce = table_test_read_many_by_equal_to_created_primary_keys.clone();
                        acc_9189f86e.push(futures::FutureExt::boxed(async move {
                            #ts
                        }));
                    };
                }
            };
            let test_delete_many_by_primary_keys_ts = {
                let ts = add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                    let primary_key_field_type_read_only_ids_into_table_type_declaration_el_primary_key_field_ident_clone_ts =
                        gen_primary_key_field_type_as_pg_type_primary_key_method_call_ts(
                            &ReadOnlyIdsIntoTableTypeDeclarationSc,
                            &quote! {el_3bb88958.#primary_key_field_ident},
                        );
                    quote! {
                        let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                            &url_cloned,
                            #ident_create_many_parameters_ucc {
                                payload: #ident_create_many_payload_ucc(
                                    std::iter::repeat_n(ident_create_default_cloned, el_56409d32).collect()
                                )
                            },
                            &table_7e35b1ce
                        ).await.expect("b8695890");
                        let read_only_ids_from_try_delete_many = #ident::try_delete_many_handle(
                            &url_cloned,
                            #ident_delete_many_parameters_ucc {
                                payload: #ident_delete_many_payload_ucc {
                                    where_many: #option_ident_where_many_ucc(Some(#ident_where_many_ucc {
                                        #primary_key_field_ident: Some(
                                            gen_pg_type_where_try_new_primary_key(
                                                pg_crud::LogicalOperator::Or,//here
                                                read_only_ids_from_try_create_many.iter().map(|el_3bb88958| #primary_key_field_type_as_pg_type_where_ts::Equal(
                                                    pg_crud::PgTypeWhereEqual {
                                                        logical_operator: pg_crud::LogicalOperator::Or,
                                                        #ValueSc: #primary_key_field_type_read_only_ids_into_table_type_declaration_el_primary_key_field_ident_clone_ts,
                                                    }
                                                )).collect()
                                            )
                                        ),
                                        #fields_none_init_ts
                                    })),
                                },
                            },
                            &table_7e35b1ce
                        ).await.expect("b80b91b8");
                        assert_eq!(
                            read_only_ids_from_try_delete_many,
                            {
                                read_only_ids_from_try_create_many.iter().map(|el_ba0f6b1c|
                                    <#primary_key_field_type as pg_crud::PgTypeTestCases>::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                        &el_ba0f6b1c.#primary_key_field_ident
                                    ).expect("3ee5ee86").#ValueSc
                                ).collect::<Vec<#primary_key_field_type_as_pg_type_read_ts>>()
                            },
                            "db5e88a6"
                        );
                        assert!(
                            gen_try_read_many_order_by_primary_key_with_big_pagination(
                                &url_cloned,
                                gen_ident_where_many_pripery_key_others_none(
                                    Some(
                                        gen_pg_type_where_try_new_primary_key(
                                            pg_crud::LogicalOperator::Or,
                                            read_only_ids_from_try_delete_many.into_iter().map(|el_adcc8db3| #primary_key_field_type_as_pg_type_where_ts::Equal(
                                                pg_crud::PgTypeWhereEqual {
                                                    logical_operator: pg_crud::LogicalOperator::Or,
                                                    #ValueSc: #primary_key_field_type_read_into_table_type_declaration_el_primary_key_field_ident_clone_ts,
                                                }
                                            )).collect()
                                        )
                                    )
                                ),
                                select_default_all_with_max_page_size_cloned.clone(),
                                &table_7e35b1ce
                            ).await
                            .expect("bcb79917")
                            .is_empty(),
                            "77f038b0"
                        );
                    }
                });
                quote! {
                    for el_56409d32 in [1,2] {
                        let url_cloned = url.clone();
                        let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                        //todo is table name correct?
                        let table_7e35b1ce = table_test_read_many_by_equal_to_created_primary_keys.clone();
                        let ident_create_default_cloned = ident_create_default.clone();
                        acc_9189f86e.push(futures::FutureExt::boxed(async move {
                            #ts
                        }));
                    };
                }
            };
            quote! {
                #test_delete_many_by_non_existent_primary_keys_ts
                #test_delete_many_by_primary_keys_ts
            }
        };
        let delete_one_tests_ts = {
            let value_init_ts = gen_import_path_value_init_ts(&primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts);
            quote! {
                acc_9189f86e.push({
                    let table_delete_one_cloned = table_delete_one.clone();
                    let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                    futures::FutureExt::boxed(async move {
                        if let Err(#ErSc) = gen_try_delete_one_handle(
                            &url,
                            #primary_key_field_type_as_pg_type_read_ts::new(uuid::Uuid::new_v4()),
                            &table_delete_one_cloned
                        ).await {
                            if let #ident_try_delete_one_er_ucc::#ident_delete_one_er_with_serde_ucc {
                                delete_one_er_with_serde,
                                ..
                            } = #ErSc {
                                if let #ident_delete_one_er_with_serde_ucc::Pg {
                                    pg,
                                    ..
                                } = delete_one_er_with_serde {
                                    assert!(pg == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row(), "c9261bb8");
                                } else {
                                    panic!("e63b27a3");
                                }
                            } else {
                                panic!("47a8e0d9")
                            }
                        } else {
                            panic!("9be62f9f")
                        }
                        let read_only_ids_returned_from_create_one = gen_read_only_ids_from_try_create_one_default(&url, &table_delete_one_cloned).await;
                        assert_eq!(
                            #ident_read_ucc {
                                #primary_key_field_ident: Some(#value_init_ts),
                                #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_returned_from_create_one_create_ts
                            },
                            gen_ident_try_read_one_handle_primary_key(
                                &url,
                                #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                &table_delete_one_cloned
                            )
                            .await.expect("c8c44c89"),
                            "86ef08ae"
                        );
                        assert_eq!(
                            gen_try_delete_one_handle(
                                &url,
                                #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
                                &table_delete_one_cloned
                            ).await.expect("7e1d1a70"),
                            #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
                            "99f81971"
                        );
                        gen_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                            &url,
                            #primary_key_field_type_read_only_ids_into_read_read_only_ids_returned_from_create_one_primary_key_field_ident_ts,
                            #select_default_all_with_max_page_size_cloned_clone_ts,
                            &table_delete_one_cloned,
                        ).await;
                    })
                });
            }
        };
        quote! {
            #[cfg(test)]
            mod #ident_tests_sc {
                use super::*;
                #[test]
                fn size_of() {
                    assert_eq!(std::mem::size_of::<#ident>(), 0);
                }
                #[test]
                fn crud() {
                    fn gen_ident_where_many_pripery_key_others_none(
                        option_pg_type_where: Option<#import_path::PgTypeWhere<#primary_key_field_type_as_pg_type_where_ts>>,
                    ) -> #ident_where_many_ucc {
                        #ident_where_many_ucc::try_new(
                            option_pg_type_where,
                            #fields_named_without_primary_key_with_comma_none_ts
                        )
                        .expect("5fb2b219")
                    }
                    fn gen_pg_type_where_try_new_primary_key(
                        logical_operator: #import_path::LogicalOperator,
                        vec: Vec<#primary_key_field_type_where_ts>
                    ) -> #import_path::PgTypeWhere<#primary_key_field_type_as_pg_type_where_ts> {
                        #gen_pg_type_where_try_new_primary_key_ts
                    }
                    fn gen_pg_type_where_try_new_or_primary_keys(
                        vec_read_only_ids: &[#ident_read_only_ids_ucc]
                    ) -> #import_path::PgTypeWhere<#primary_key_field_type_as_pg_type_where_ts> {
                        gen_pg_type_where_try_new_primary_key(
                            #import_path::LogicalOperator::Or,
                            vec_read_only_ids.iter().map(|el_9530b728| #primary_key_field_type_where_ts::Equal(#import_path::PgTypeWhereEqual {
                                logical_operator: #import_path::LogicalOperator::Or,
                                value: #primary_key_field_type_table_type_declaration_ts::new(
                                    #primary_key_field_type_as_pg_type_ts into_inner(
                                        <#primary_key_field_type as #import_path::PgTypePrimaryKey>::read_only_ids_into_read(
                                            el_9530b728.#primary_key_field_ident
                                        ),
                                    )
                                ),
                            })).collect()
                        )
                    }
                    async fn gen_try_read_many_order_by_primary_key_with_big_pagination(
                        endpoint_location: &str,
                        ident_where_many_6b1fab92: #ident_where_many_ucc,
                        select: #import_path::NotEmptyUniqueVec<#ident_select_ucc>,
                        table: &str
                    ) -> Result<Vec<#ident_read_ucc>, #ident_try_read_many_er_ucc> {
                        #ident::try_read_many_handle(
                            endpoint_location,
                            #ident_read_many_parameters_ucc {
                                payload: #ident_read_many_payload_ucc {
                                    where_many: #option_ident_where_many_ucc(Some(
                                        ident_where_many_6b1fab92
                                    )),
                                    select,
                                    order_by: #import_path::OrderBy {
                                        column: #ident_select_ucc::#primary_key_field_ident_ucc_ts(
                                            #primary_key_field_type_as_pg_type_select_ts::default()
                                        ),
                                        order: Some(#import_path::Order::Asc)
                                    },
                                    pagination: #import_path::PaginationStartsWithZero::try_new(10000, 0).expect("b0cdf0cb"),
                                }
                            },
                            table
                        )
                        .await
                    }
                    async fn gen_ident_try_read_one_handle_primary_key(
                        url: &str,
                        primary_key_column: #primary_key_field_type_as_pg_type_read_ts,
                        select: #import_path::NotEmptyUniqueVec<#ident_select_ucc>,
                        table: &str,
                    ) -> Result<#ident_read_ucc, #ident_try_read_one_er_ucc> {
                        #ident::try_read_one_handle(
                            url,
                            #ident_read_one_parameters_ucc {
                                payload: #ident_read_one_payload_ucc {
                                    primary_key_column,
                                    select,
                                },
                            },
                            table,
                        )
                        .await
                    }
                    async fn gen_check_no_rows_returned_from_ident_try_read_one_handle_primary_key(
                        url: &str,
                        primary_key_column: #primary_key_field_type_as_pg_type_read_ts,
                        select: #import_path::NotEmptyUniqueVec<#ident_select_ucc>,
                        table: &str,
                    ) {
                        if let Err(#ErSc) = gen_ident_try_read_one_handle_primary_key(
                            url,
                            primary_key_column,
                            select,
                            table
                        ).await {
                            if let #ident_try_read_one_er_ucc::#ident_read_one_er_with_serde_ucc {
                                read_one_er_with_serde,
                                ..
                            } = er {
                                if let #ident_read_one_er_with_serde_ucc::Pg { pg, .. } = read_one_er_with_serde {
                                    assert!(pg == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row(), "58b9a6a4");
                                } else {
                                    panic!("0ad0117b");
                                }
                            } else {
                                panic!("c6695392")
                            }
                        } else {
                            panic!("67e43b7a")
                        }
                    }
                    fn ident_create_default() -> #ident_create_ucc {
                        #ident_create_ucc {
                            #ident_create_default_fields_init_without_primary_key_ts
                        }
                    }
                    async fn gen_read_only_ids_from_try_create_one(
                        #UrlSc: &str,
                        #PayloadSc: #ident_create_ucc,
                        table: &str,
                    ) -> #ident_read_only_ids_ucc {
                        #ident::try_create_one_handle(
                            #UrlSc,
                            #ident_create_one_parameters_ucc {
                                #PayloadSc
                            },
                            table
                        ).await.expect("32e30b87")
                    }
                    async fn gen_read_only_ids_from_try_create_one_default(
                        #UrlSc: &str,
                        table: &str,
                    ) -> #ident_read_only_ids_ucc {
                        gen_read_only_ids_from_try_create_one(
                            #UrlSc,
                            ident_create_default(),
                            table
                        ).await
                    }
                    async fn gen_try_delete_one_handle(
                        #UrlSc: &str,
                        #primary_key_field_ident: #primary_key_field_type_as_pg_type_read_ts,
                        table: &str,
                    ) -> Result<#primary_key_field_type_as_pg_type_read_ts, #ident_try_delete_one_er_ucc> {
                        #ident::try_delete_one_handle(
                            #UrlSc,
                            #ident_delete_one_parameters_ucc {
                                payload: #ident_delete_one_payload_ucc {
                                    #primary_key_field_ident
                                }
                            },
                            table
                        ).await
                    }
                    fn no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row() -> &'static str {
                        "no rows returned by a query that expected to return at least one row"
                    }
                    fn gen_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create(
                        read_only_ids_from_try_create_many: Vec<#ident_read_only_ids_ucc>,
                        ident_vec_create: Vec<#ident_create_ucc>
                    ) -> Vec<#ident_read_ucc> {
                        let mut acc_1debe8fb = Vec::new();
                        assert_eq!(read_only_ids_from_try_create_many.len(), ident_vec_create.len(), "88fb286c");
                        for (read_only_ids, create) in read_only_ids_from_try_create_many.into_iter().zip(ident_vec_create.into_iter()) {
                            acc_1debe8fb.push(#ident_read_ucc {
                                #primary_key_field_ident: <#primary_key_field_type as pg_crud::PgTypeTestCases>::read_only_ids_to_option_value_read_default_option_some_vec_one_el(
                                    &read_only_ids.#primary_key_field_ident
                                ),
                                #field_ident_read_only_ids_merged_with_create_into_option_value_read_read_only_ids_and_create_ts
                            });
                        }
                        acc_1debe8fb.sort_by(|first, second| {
                            if let (Some(first_handle), Some(second_handle)) = (&first.#primary_key_field_ident, &second.#primary_key_field_ident) {
                                first_handle.#ValueSc.cmp(&second_handle.#ValueSc)
                            } else {
                                panic!("d760ffa3");
                            }
                        });
                        acc_1debe8fb
                    }
                    #gen_read_only_ids_elements_ts_fe29ff70
                    tracing_subscriber::fmt::init();
                    tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("38823c21").block_on(async {
                        //todo maybe refactor
                        let #ConfigSc = #config_path_ts {
                            service_socket_address: <config_lib::ServiceSocketAddress as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "127.0.0.1:8080".to_owned()
                            ).expect("b5b3915a").0,
                            database_url: <config_lib::DatabaseUrl as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=10".to_owned()
                            ).expect("f9c20f05").0,
                            timezone: <config_lib::Timezone as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "10800".to_owned()
                            ).expect("d00d8998").0,
                            tracing_level: <config_lib::TracingLevel as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "er".to_owned()
                            ).expect("957178c9").0,
                            source_place_type: <config_lib::SourcePlaceType as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "source".to_owned()
                            ).expect("bec0950e").0,
                            enable_api_git_commit_check: <config_lib::EnableApiGitCommitCheck as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "true".to_owned()
                            ).expect("31f02640").0,
                            maximum_size_of_http_body_in_bytes: <config_lib::MaximumSizeOfHttpBodyInBytes as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                                "1048576000".to_owned()
                            ).expect("93b2f818").0,
                        };
                        let #PgPoolSc = sqlx::postgres::PgPoolOptions::new()
                        .max_connections(50)
                        .connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&#ConfigSc)))
                        .await.expect("e3044bb9");
                        let #UrlSc = format!("http://{}", app_state::GetServiceSocketAddress::get_service_socket_address(&#ConfigSc));
                        let table = #ident_dq_ts;
                        let add_table_postfix = |postfix: &str|{
                            let value = format!("{table}_{postfix}");
                            assert!(value.len() <= 63, "77f9bfb7");
                            value
                        };
                        let table_init = add_table_postfix("init");
                        let table_create_many = add_table_postfix("create_many");
                        let table_create_one = add_table_postfix("create_one");
                        let table_test_read_many_by_non_existent_primary_keys = add_table_postfix("Test_read_many_by_non_existent_primary_keys");
                        let table_test_read_many_by_equal_to_created_primary_keys = add_table_postfix("Test_read_many_by_equal_to_created_primary_keys");
                        #(#table_field_idents_init_vec_ts)*
                        let table_read_one = add_table_postfix("read_one");
                        let table_update_many = add_table_postfix("update_many");
                        let table_update_one = add_table_postfix("update_one");
                        let table_delete_many = add_table_postfix("delete_many");
                        let table_delete_one = add_table_postfix("delete_one");
                        let table_names = [
                            &table_init,
                            &table_create_many,
                            &table_create_one,
                            &table_test_read_many_by_non_existent_primary_keys,
                            &table_test_read_many_by_equal_to_created_primary_keys,
                            #(#table_test_name_field_idents_vec_ts)*
                            &table_read_one,
                            &table_update_many,
                            &table_update_one,
                            &table_delete_many,
                            &table_delete_one,
                        ];
                        let drop_all_test_tables = async ||{
                            let _unused = futures::future::try_join_all(
                                table_names
                                .iter()
                                .map(|table_name|{
                                    let pg_pool_3b948340 = &pg_pool;
                                    async move {
                                        sqlx::query(&format!("drop table if exists {table_name}")).execute(pg_pool_3b948340).await
                                    }
                                })
                            )
                            .await
                            .expect("b9c1eb2e");
                        };
                        drop_all_test_tables().await;
                        #ident::prepare_extensions(&#PgPoolSc).await.expect("0633ff48");
                        //do not make it concurrent. would be pg er: "duplicate key value violates unique constraint \"pg_class_relname_nsp_index\""
                        for el_dac43b91 in table_names {
                            #ident::prepare_pg_table(
                                &#PgPoolSc,
                                el_dac43b91,
                            ).await.expect("c7952247");
                        }
                        let #PgPoolForTokioSpawnSyncMoveSc = #PgPoolSc.clone();
                        let table_names_cloned = table_names.iter().map(|el_26b304d1| (*el_26b304d1).to_owned()).collect::<Vec<String>>();
                        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
                        let #underscore_unused_ts = tokio::spawn(async move {
                            let tcp_listener = tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(&#ConfigSc)).await.expect("663ae29e");
                            let #AppStateSc = std::sync::Arc::new(server_app_state::ServerAppState {
                                #PgPoolSc: #PgPoolForTokioSpawnSyncMoveSc.clone(),
                                #ConfigSc,
                                project_git_info: &git_info::PROJECT_GIT_INFO,
                            });
                            started_tx.send(()).expect("431a6f8d");
                            axum::serve(
                                tcp_listener,
                                {
                                    let mut router = axum::Router::new()
                                        .merge(#ident::routes(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state)));
                                    for el_ef09f2b0 in table_names_cloned {
                                        router = router.merge(#ident::routes_handle(std::sync::Arc::<server_app_state::ServerAppState<'_>>::clone(&app_state), &el_ef09f2b0));
                                    }
                                    router.into_make_service()
                                },
                            )
                            .await
                            .expect("71c1bc30");
                        });
                        started_rx.await.expect("87003141");
                        let #SelectPrimaryKeySc = pg_crud::NotEmptyUniqueVec::try_new(vec![
                            #ident_select_ucc::#primary_key_field_ident_ucc_ts(
                                #primary_key_field_type_as_pg_type_select_ts::default(),
                            )
                        ])
                        .expect("0776170e");
                        let #IdentCreateDefaultSc = ident_create_default();
                        #select_default_all_with_max_page_size_not_empty_unique_vec_ts
                        #common_read_only_ids_returned_from_create_one_ts
                        #read_only_ids_to_two_dimal_vec_read_inner_acc_fields_ts
                        futures::StreamExt::for_each_concurrent(
                            futures::stream::iter({
                                let mut acc_9189f86e: Vec<futures::future::BoxFuture<'static, ()>> = Vec::new();
                                #create_many_tests_ts
                                #create_one_tests_ts
                                #read_many_tests_ts
                                #read_one_tests_ts
                                #update_many_tests_ts
                                #update_one_tests_ts
                                #delete_many_tests_ts
                                #delete_one_tests_ts
                                acc_9189f86e
                            }),
                            100,
                            async |fut| { fut.await; },
                        )
                        .await;
                        drop_all_test_tables().await;
                    });
                }
            }
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.tests_content_write_into_gen_pg_table_tests,
        "gen_pg_table_Tests",
        &ident_tests_ts,
        &FormatWithCargofmt::True,
    );
    let common_ts = quote! {
        #impl_ident_ts
        #ident_create_ts
        #ident_where_many_ts
        #option_ident_where_many_ts
        #select_ts
        #ident_read_ts
        #ident_read_only_ids_ts
        // #ident_column_read_permission_ts
        #ident_update_ts
        #ident_update_for_query_ts
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.common_content_write_into_gen_pg_table_common,
        "gen_pg_table_common",
        &common_ts,
        &FormatWithCargofmt::True,
    );
    let gend = {
        let ident_gen_pg_table_mod_sc = SelfGenPgTableModSc::from_tokens(&ident);
        let ts_1c0e3fcd = quote! {
            #AllowClippyArbitrarySourceItemOrdering
            impl #ident {
                #(#impl_ident_vec_ts)*
            }
            #common_ts
            #create_many_ts
            #create_one_ts
            #read_many_ts
            #read_one_ts
            #update_many_ts
            #update_one_ts
            #delete_many_ts
            #delete_one_ts
            #ident_tests_ts
        };
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #ident_gen_pg_table_mod_sc {
                use super::#ident;
                #ts_1c0e3fcd
            }
            pub use #ident_gen_pg_table_mod_sc::*;
        }
    };
    maybe_write_ts_into_file(
        gen_pg_table_config.whole_content_write_into_gen_pg_table,
        "gen_pg_table",
        &gend,
        &FormatWithCargofmt::True,
    );
    gend
}
