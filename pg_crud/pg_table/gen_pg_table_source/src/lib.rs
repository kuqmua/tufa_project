use gen_quotes::dq_ts;
use macros_helpers::{
    AttrIdentStr, DeriveClone, DeriveCopy, FormatWithCargofmt, LocationFieldAttr,
    ShouldWriteTokenStreamIntoFile, StatusCode, StructOrEnumDeriveTsStreamBuilder, SynFieldWrapper,
    gen_field_loc_new_ts, gen_if_write_is_err_curly_braces_ts, gen_if_write_is_err_ts,
    gen_impl_display_ts, gen_impl_pub_try_new_for_ident_ts, gen_impl_to_err_string_ts,
    gen_serde_version_of_named_syn_vrt, gen_simple_syn_punct, get_macro_attr_meta_list_ts,
    loc_syn_field, mb_write_ts_into_file,
};
use naming::{
    AppStateSc, AsRefStrEnumWithUnitFieldsToScStr, AsRefStrEnumWithUnitFieldsToUccStr,
    AsRefStrToScStr, AsRefStrToScTs, BeginSc, BindedQuerySc, BodyBytesSc, BodySc, BodySizeErUcc,
    BySc, CheckBodySizeSc, CheckBodySizeUcc, ColumnSc, ColumnsSc, CommitSc, CommonExtraErVrtsSc,
    CommonExtraLogicSc, CommonReadOnlyIdsReturnedFromCreateOneSc, ConfigSc,
    CreateExtensionIfNotExistsPgJsonschemaUcc, CreateExtensionIfNotExistsUuidOsspUcc,
    CreateIntoPgJsonTypeOptVecWhereLengthEqualSc,
    CreateIntoPgJsonTypeOptVecWhereLengthGreaterThanSc, CreateIntoPgTypeOptVecWhereDimOneEqualSc,
    CreateManyExtraErVrtsSc, CreateManyExtraLogicSc, CreateOneExtraErVrtsSc, CreateOneExtraLogicSc,
    CreateQueryBindSc, CreateQueryPartSc, CreateSc, CreateTableColumnQueryPartSc, CreateUcc,
    DefaultOptSomeVecOneElMaxPageSizeSc, DefaultOptSomeVecOneElMaxPageSizeUcc,
    DefaultOptSomeVecOneElSc, DefaultOptSomeVecOneElUcc, DeleteManyExtraErVrtsSc,
    DeleteManyExtraLogicSc, DeleteOneExtraErVrtsSc, DeleteOneExtraLogicSc, DeserializeResUcc,
    DesirableUcc, DisplayPlusToTokens, DisplayToScStr, ElSc, EndpointLocationSc, ErSc,
    ExecutorAcquireSc, ExecutorSc, ExpectedResSc, ExtraParamsSc, FailedToGetResTextUcc, FalseSc,
    FromHandleSc, FutureSc, GenColumnQuealsVCommaUpdateOneQueryPartSc, GenPgTablePkSc,
    GenSelectQueryPartSc, GenWhenColumnIdThenVUpdateManyQueryPartSc,
    HeaderContentTypeApplicationJsonNotFoundUcc, HeadersSc, IdentCreateDefaultSc, IncrSc,
    IntoSerdeVersionSc, LocSc, NoFieldsProvidedUcc, NotUniqueFieldSc, NotUniqueFieldUcc,
    NotUniquePkSc, NotUniquePkUcc, OptVecCreateSc, OrderBySc, OrderByUcc, OrderSc, PaginationSc,
    ParamsSc, PayloadSc, PayloadUcc, PgCrudSc, PgPoolForTokioSpawnSyncMoveSc, PgPoolSc, PgSc,
    PgTypeOptVecWhereGreaterThanTestSc, PgTypeUcc, PgUcc, PkQueryPartSc, PkSc, PoolConnectionSc,
    PoolSc, PrefixSc, PrepareExtensionsSc, PreparePgSc, PreparePgTableSc, PreparePgUcc,
    QueryBindSc, QueryPartErUcc, QueryPartSc, QueryPartUcc, QuerySc, QueryStringSc,
    ReadIntoTableTypeSc, ReadManyExtraErVrtsSc, ReadManyExtraLogicSc, ReadOneExtraErVrtsSc,
    ReadOneExtraLogicSc, ReadOnlyIdsIntoReadSc, ReadOnlyIdsIntoTableTypeSc,
    ReadOnlyIdsIntoUpdateSc, ReadOnlyIdsMergedWithCreateIntoOptVecWhereEqualToJsonFieldSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereBetweenSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereContainsElGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereContainsElRegexSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereGreaterThanSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereInSc,
    ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereRegexSc,
    ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc,
    ReadOnlyIdsMergedWithCreateIntoWhereEqualSc,
    ReadOnlyIdsMergedWithTableTypeIntoPgTypeOptWhereGreaterThanSc, ReadOnlyIdsSc, ReadOnlyIdsUcc,
    ReadUcc, ReqSc, ReqwestSc, ReqwestUcc, ResSc, ResTextSc, RollbackSc, RoutesHandleSc, RoutesSc,
    RowAndRollbackUcc, RowSc, RowsSc, SelectOnlyIdsQueryPartSc, SelectOnlyUpdatedIdsQueryPartSc,
    SelectPkSc, SelectQueryPartSc, SelectSc, SelectUcc, SerdeJsonSc, SerdeJsonToStringSc,
    SerdeJsonToStringUcc, SerdeJsonUcc, SerdeSc, StatusCodeSc, TableNameSc, TableSc,
    ToTokensToScStr, ToTokensToUccTs, TrueSc, TryBindSc, TryBindUcc, UpdateForQuerySc,
    UpdateForQueryUcc, UpdateForQueryVecSc, UpdateManyExtraErVrtsSc, UpdateManyExtraLogicSc,
    UpdateOneExtraErVrtsSc, UpdateOneExtraLogicSc, UpdateQueryBindSc, UpdateQueryPartPkSc,
    UpdateQueryPartSc, UpdateSc, UpdateUcc, UrlSc, VSc, VUcc, WhereManySc, WhereUcc,
    param::{
        ErSelfSc, IsSelfUpdateExistSc, SelfCreateUcc, SelfDeleteManyParamsUcc,
        SelfDeleteManyPayloadUcc, SelfDeleteOneErWithSerdeUcc, SelfDeleteOneParamsUcc,
        SelfDeleteOnePayloadUcc, SelfErWithSerdeSc, SelfGenPgTableModSc, SelfHandleSc,
        SelfPayloadExampleSc, SelfPreparePgErUcc, SelfReadOneErWithSerdeUcc,
        SelfReadOnlyIdsToTwoDimsVecReadInnerAccSc, SelfReadOnlyIdsUcc, SelfReadUcc, SelfSelectUcc,
        SelfTableTypeUcc, SelfTestsSc, SelfTryDeleteOneErUcc, SelfTryReadOneErUcc,
        SelfUpdateForQueryUcc, SelfUpdateManyParamsUcc, SelfUpdateManyPayloadUcc,
        SelfUpdateTryNewErUcc, SelfUpdateUcc, SelfWhereManyTryNewErUcc, SelfWhereManyUcc,
        StdOptOptSelfWhereManyUcc, TryFromSqlxPgPgRowWithNotEmptyUniqueVecSelfSelectSc,
        TrySelfHandleSc, TrySelfSc, UpdateQueryPartSelfSc,
    },
};
use optimal_pack::OptimalPack;
use panic_location::panic_location;
use pg_crud_macros_common::{
    ColumnParamUnderscore, Dim, EqualOrEqualUsingFields, Import, IncrParamUnderscore,
    IsNeedToAddOperatorUnderscore, IsQueryBindMutable,
    gen_impl_pg_crud_all_vrts_default_opt_some_vec_one_el_ts,
    gen_impl_pg_crud_default_opt_some_vec_one_el_ts, gen_impl_serde_deserialize_for_struct_ts,
    gen_match_try_new_in_deserialize_ts, gen_opt_type_decl_ts,
    gen_query_part_er_write_into_buffer_ts, gen_return_err_query_part_er_write_into_buffer_ts,
    gen_v_decl_ts, gen_v_init_ts, gen_vec_tokens_decl_ts, impl_pg_type_where_filter_for_ident_ts,
    mb_wrap_into_braces_ts,
};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde::Deserialize;
use serde_json::from_str;
use std::{
    fmt::{Display, Formatter, Result as StdFmtResult, Write as _},
    iter::once,
    panic::Location,
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
    Er3, F32, F64, FieldAttrSerdeSkipSerializingIfOptIsNone, I8, I16, I32, I64, MustUse,
    PgCrudCommonDefaultOptSomeVecOneEl, PgCrudCommonDefaultOptSomeVecOneElCall,
    PgCrudCommonDefaultOptSomeVecOneElMaxPageSizeCall, PgCrudDefaultOptSomeVecOneElCall, RefStr,
    SqlxAcquire, SqlxRow, StringTs, U8, U16, U32, U64,
};
//todo decide where to do er log (mb add in some places)
//todo gen route what will return columns of the table and their rust and postgersql types
//todo created at and updated at fields + created by + updated by
//todo attrs for activation generation crud methods(like gen create, update_one, delete_one)
//todo authorization for returning concrete er or just minimal info(user role)
//todo gen rules and roles
//todo mb add unnest sql types?
//todo mb add unnest to filter params if its arr ?
//todo swagger ui https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
//todo derive utoipa::ToSchema for what? original structs or with serialize deserialize?
//todo need to add utoipa::ToSchema annotation #[schema(value_type = YourToSchemaTraitImplStruct)] for all fields
//todo remove useless derives like useless serde::Serialize and Deserialize
//todo mb gen compisite type for user defined type https://docs.rs/sqlx/0.7.3/sqlx/pg/types/index.html#rust_decimal
//todo read again some interesting thoughts about sql as api https://habr.com/ru/companies/timeweb/articles/798937/
//todo reexport all crates what logic depends on (from crates.io) (use of undeclared crate or module `time`)
//todo add transaction isolation level (see pg docs)
//todo check on pg max length value of type
//todo in few cases rows affected is usefull. (update delete for example). if 0 afftected -mb its er? or mb use select then update\delete?(rewrite query)
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
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, OptimalPack)]
    struct SynVrtWrapper {
        vrt: Variant,
        status_code: Option<StatusCode>,
    }
    impl SynVrtWrapper {
        const fn get_opt_status_code(&self) -> Option<&StatusCode> {
            self.status_code.as_ref()
        }
        const fn get_syn_vrt(&self) -> &Variant {
            &self.vrt
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
        Debug,
        Clone,
        Copy,
        AsRefStrEnumWithUnitFieldsToUccStr,
        AsRefStrEnumWithUnitFieldsToScStr,
        OptimalPack,
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
        const fn gen_pg_table_attr_extra_er_vrts(self) -> GenPgTableAttr {
            match self {
                Self::CreateMany => GenPgTableAttr::CreateManyExtraErVrts,
                Self::CreateOne => GenPgTableAttr::CreateOneExtraErVrts,
                Self::ReadMany => GenPgTableAttr::ReadManyExtraErVrts,
                Self::ReadOne => GenPgTableAttr::ReadOneExtraErVrts,
                Self::UpdateMany => GenPgTableAttr::UpdateManyExtraErVrts,
                Self::UpdateOne => GenPgTableAttr::UpdateOneExtraErVrts,
                Self::DeleteMany => GenPgTableAttr::DeleteManyExtraErVrts,
                Self::DeleteOne => GenPgTableAttr::DeleteOneExtraErVrts,
            }
        }
        const fn gen_pg_table_attr_extra_logic(self) -> GenPgTableAttr {
            match self {
                Self::CreateMany => GenPgTableAttr::CreateManyExtraLogic,
                Self::CreateOne => GenPgTableAttr::CreateOneExtraLogic,
                Self::ReadMany => GenPgTableAttr::ReadManyExtraLogic,
                Self::ReadOne => GenPgTableAttr::ReadOneExtraLogic,
                Self::UpdateMany => GenPgTableAttr::UpdateManyExtraLogic,
                Self::UpdateOne => GenPgTableAttr::UpdateOneExtraLogic,
                Self::DeleteMany => GenPgTableAttr::DeleteManyExtraLogic,
                Self::DeleteOne => GenPgTableAttr::DeleteOneExtraLogic,
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
            let v = SelfHandleSc::from_tokens(&self.self_sc_ts());
            quote! {#v}
        }
        fn self_sc_str(self) -> String {
            AsRefStrToScStr::case(&self.to_string())
        }
        fn self_sc_ts(self) -> Ts2 {
            AsRefStrToScTs::case_or_panic(&self.to_string())
        }
        fn try_self_handle_sc_ts(self) -> Ts2 {
            let v = TrySelfHandleSc::from_tokens(&self.self_sc_ts());
            quote! {#v}
        }
        fn try_self_sc_ts(self) -> Ts2 {
            let v = TrySelfSc::from_tokens(&self.self_sc_ts());
            quote! {#v}
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
    #[derive(AsRefStrEnumWithUnitFieldsToScStr, OptimalPack)]
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
    #[derive(Debug, Display, OptimalPack)]
    enum GenPgTableAttr {
        CreateManyExtraErVrts,
        CreateOneExtraErVrts,
        ReadManyExtraErVrts,
        ReadOneExtraErVrts,
        UpdateManyExtraErVrts,
        UpdateOneExtraErVrts,
        DeleteManyExtraErVrts,
        DeleteOneExtraErVrts,
        CommonExtraErVrts,
        CreateManyExtraLogic,
        CreateOneExtraLogic,
        ReadManyExtraLogic,
        ReadOneExtraLogic,
        UpdateManyExtraLogic,
        UpdateOneExtraLogic,
        DeleteManyExtraLogic,
        DeleteOneExtraLogic,
        CommonExtraLogic,
    }
    impl GenPgTableAttr {
        fn gen_path_to_attr(self) -> String {
            format!(
                "{PgCrudSc}::{}",
                match self {
                    Self::CreateManyExtraErVrts => CreateManyExtraErVrtsSc.to_string(),
                    Self::CreateOneExtraErVrts => CreateOneExtraErVrtsSc.to_string(),
                    Self::ReadManyExtraErVrts => ReadManyExtraErVrtsSc.to_string(),
                    Self::ReadOneExtraErVrts => ReadOneExtraErVrtsSc.to_string(),
                    Self::UpdateManyExtraErVrts => UpdateManyExtraErVrtsSc.to_string(),
                    Self::UpdateOneExtraErVrts => UpdateOneExtraErVrtsSc.to_string(),
                    Self::DeleteManyExtraErVrts => DeleteManyExtraErVrtsSc.to_string(),
                    Self::DeleteOneExtraErVrts => DeleteOneExtraErVrtsSc.to_string(),
                    Self::CommonExtraErVrts => CommonExtraErVrtsSc.to_string(),
                    Self::CreateManyExtraLogic => CreateManyExtraLogicSc.to_string(),
                    Self::CreateOneExtraLogic => CreateOneExtraLogicSc.to_string(),
                    Self::ReadManyExtraLogic => ReadManyExtraLogicSc.to_string(),
                    Self::ReadOneExtraLogic => ReadOneExtraLogicSc.to_string(),
                    Self::UpdateManyExtraLogic => UpdateManyExtraLogicSc.to_string(),
                    Self::UpdateOneExtraLogic => UpdateOneExtraLogicSc.to_string(),
                    Self::DeleteManyExtraLogic => DeleteManyExtraLogicSc.to_string(),
                    Self::DeleteOneExtraLogic => DeleteOneExtraLogicSc.to_string(),
                    Self::CommonExtraLogic => CommonExtraLogicSc.to_string(),
                }
            )
        }
    }
    enum ShouldWrapIntoV {
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
    #[derive(Debug, Deserialize, OptimalPack)]
    struct GenPgTableConfig {
        create_many_write_into_file: ShouldWriteTokenStreamIntoFile,
        create_one_write_into_file: ShouldWriteTokenStreamIntoFile,
        read_many_write_into_file: ShouldWriteTokenStreamIntoFile,
        read_one_write_into_file: ShouldWriteTokenStreamIntoFile,
        update_many_write_into_file: ShouldWriteTokenStreamIntoFile,
        update_one_write_into_file: ShouldWriteTokenStreamIntoFile,
        delete_many_write_into_file: ShouldWriteTokenStreamIntoFile,
        delete_one_write_into_file: ShouldWriteTokenStreamIntoFile,
        tests_write_into_file: ShouldWriteTokenStreamIntoFile,
        common_write_into_file: ShouldWriteTokenStreamIntoFile,
        whole_write_into_file: ShouldWriteTokenStreamIntoFile,
    }
    panic_location();
    let import = Import::PgCrud;
    let import_ts = quote! {#import::};
    let return_err_query_part_er_write_into_buffer_ts =
        gen_return_err_query_part_er_write_into_buffer_ts(import);
    let di: DeriveInput = parse2(input).expect("991c614f");
    let gen_pg_table_config = from_str::<GenPgTableConfig>(
        &get_macro_attr_meta_list_ts(
            &di.attrs,
            &format!("{}::gen_pg_table_config", import.sc_str()),
        )
        .to_string(),
    )
    .expect("1b6adf7e");
    let ident = &di.ident;
    let ident_sc_str = ToTokensToScStr::case(&ident);
    let ident_sc_dq_ts = dq_ts(&ident_sc_str);
    let self_table_name_call_ts = quote! {Self::#TableNameSc()};
    let (pk_field, fields, fields_without_pk) = if let Data::Struct(data_struct) = &di.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            let mut opt_pk_field: Option<SynFieldWrapper> = None;
            let mut fields = Vec::new();
            let mut fields_without_pk = Vec::new();
            for el in &fields_named.named {
                let fi = el.ident.clone().expect("915ef2ce");
                let fi_len = fi.to_string().len();
                let max_pg_column_length = 63;
                //todo write runtime check
                assert!(fi_len <= max_pg_column_length, "1266ae5a");
                fields.push(SynFieldWrapper {
                    vis: el.vis.clone(),
                    type0: el.ty.clone(),
                    ident: fi.clone(),
                });
                let mut is_pk = false;
                {
                    for el0 in &el.attrs {
                        if el0.path().segments.len() == 1 {
                            let first_segment_ident =
                                &el0.path().segments.first().expect("a9c3b38b").ident;
                            let gen_pg_table_pk_sc_str = GenPgTablePkSc.to_string();
                            if first_segment_ident == &gen_pg_table_pk_sc_str {
                                if opt_pk_field.is_some() {
                                    panic!("1a75cea1");
                                } else {
                                    opt_pk_field = Some(SynFieldWrapper {
                                        vis: el.vis.clone(),
                                        type0: el.ty.clone(),
                                        ident: fi.clone(),
                                    });
                                    is_pk = true;
                                }
                            }
                        }
                    }
                }
                if !is_pk {
                    fields_without_pk.push(SynFieldWrapper {
                        vis: el.vis.clone(),
                        type0: el.ty.clone(),
                        ident: fi.clone(),
                    });
                }
            }
            // explicitly not supporting nbr of columns more than 100 so its less possibility to cause stack overflow or build process exit
            // assert!((fields.len() <= 100), "d9963f32");
            (opt_pk_field.expect("6a529a99"), fields, fields_without_pk)
        } else {
            panic!("7f31872d");
        }
    } else {
        panic!("bd4718d0");
    };
    let fields_len = fields.len();
    let fields_len_without_pk = fields_without_pk.len();
    let pk_ft = &pk_field.type0;
    //todo must remove this and use trait type instead
    let pk_ft_table_type_ts = SelfTableTypeUcc::from_type_last_segment(&pk_field.type0);
    let gen_as_pg_type_ts = |ts: &dyn ToTokens| {
        quote! {<#ts as #import_ts #PgTypeUcc>}
    };
    let gen_as_pg_type_path_ts = |ts: &dyn ToTokens| {
        let ts0 = gen_as_pg_type_ts(ts);
        quote! {#ts0::}
    };
    let pk_ft_as_pg_type_ts = gen_as_pg_type_path_ts(&pk_ft);
    let pk_ft_as_pg_type_read_ucc = quote! {#pk_ft_as_pg_type_ts #ReadUcc};
    let pk_as_pg_type_ts = gen_as_pg_type_ts(&pk_ft);
    let gen_as_pg_type_tokens_ts = |ts: &dyn ToTokens, tokens: &dyn ToTokens| {
        let as_pg_type_ts = gen_as_pg_type_path_ts(&ts);
        quote! {#as_pg_type_ts #tokens}
    };
    let gen_as_pg_type_test_cases_path_ts = |ts: &dyn ToTokens| {
        quote! {<#ts as #import_ts PgTypeTestCases>::}
    };
    let pk_as_pg_type_test_cases_path_ts = gen_as_pg_type_test_cases_path_ts(&pk_ft);
    let gen_as_pg_type_create_ts = |ts: &dyn ToTokens| gen_as_pg_type_tokens_ts(&ts, &CreateUcc);
    let gen_as_pg_type_select_ts = |ts: &dyn ToTokens| gen_as_pg_type_tokens_ts(&ts, &SelectUcc);
    let pk_ft_as_pg_type_select_ts = gen_as_pg_type_select_ts(&pk_ft);
    let gen_as_pg_type_where_ts = |ts: &dyn ToTokens| gen_as_pg_type_tokens_ts(&ts, &WhereUcc);
    let pk_ft_as_pg_type_where_ts = gen_as_pg_type_where_ts(&pk_ft);
    let gen_as_pg_type_read_ts = |ts: &dyn ToTokens| gen_as_pg_type_tokens_ts(&ts, &ReadUcc);
    let gen_as_pg_type_read_only_ids_ts =
        |ts: &dyn ToTokens| gen_as_pg_type_tokens_ts(&ts, &ReadOnlyIdsUcc);
    let pk_ft_as_pg_type_read_ts = gen_as_pg_type_read_ts(&pk_ft);
    let gen_as_pg_type_update_ts = |ts: &dyn ToTokens| gen_as_pg_type_tokens_ts(&ts, &UpdateUcc);
    let gen_as_pg_type_update_for_query_ts =
        |ts: &dyn ToTokens| gen_as_pg_type_tokens_ts(&ts, &UpdateForQueryUcc);
    let ident_read_only_ids_ucc = SelfReadOnlyIdsUcc::from_tokens(&ident);
    let ident_delete_many_params_ucc = SelfDeleteManyParamsUcc::from_tokens(&ident);
    let ident_delete_many_payload_ucc = SelfDeleteManyPayloadUcc::from_tokens(&ident);
    let ident_delete_one_params_ucc = SelfDeleteOneParamsUcc::from_tokens(&ident);
    let ident_delete_one_payload_ucc = SelfDeleteOnePayloadUcc::from_tokens(&ident);
    let ident_try_read_one_er_ucc = SelfTryReadOneErUcc::from_tokens(&ident);
    let ident_read_one_er_with_serde_ucc = SelfReadOneErWithSerdeUcc::from_tokens(&ident);
    let ident_try_delete_one_er_ucc = SelfTryDeleteOneErUcc::from_tokens(&ident);
    let ident_delete_one_er_with_serde_ucc = SelfDeleteOneErWithSerdeUcc::from_tokens(&ident);
    let vec_pk_ft_read_ts = gen_vec_tokens_decl_ts(&pk_ft_as_pg_type_read_ucc);
    let vec_ident_read_only_ids_ts = gen_vec_tokens_decl_ts(&ident_read_only_ids_ucc);
    let pk_fi = &pk_field.ident;
    let pk_fi_ucc_ts = ToTokensToUccTs::case_or_panic(&pk_fi);
    let pk_ft_update_ts = &SelfUpdateUcc::from_type_last_segment(pk_ft);
    let pk_ft_update_for_query_ts = &SelfUpdateForQueryUcc::from_type_last_segment(pk_ft);
    let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
    let gen_from_handle_ts = |ident_ts: &dyn ToTokens, ts: &dyn ToTokens| {
        quote! {
            fn #FromHandleSc(#VSc: #ident_ts) -> Self {
                #ts
            }
        }
    };
    let gen_select_pg_crud_not_empty_unique_vec_ident_select_ts =
        |should_add_borrow: &ShouldAddBorrow| {
            quote! {#SelectSc: #should_add_borrow #import_ts NotEmptyUniqueVec<#ident_select_ucc>}
        };
    let select_borrow_pg_crud_not_empty_unique_vec_ident_select_ts =
        gen_select_pg_crud_not_empty_unique_vec_ident_select_ts(&ShouldAddBorrow::True);
    let select_pg_crud_not_empty_unique_vec_ident_select_ts =
        gen_select_pg_crud_not_empty_unique_vec_ident_select_ts(&ShouldAddBorrow::False);
    let pub_select_pg_crud_not_empty_unique_vec_ident_select_ts = {
        quote! {pub #select_pg_crud_not_empty_unique_vec_ident_select_ts}
    };
    let gen_fields_named_with_comma_ts = |fn0: &dyn Fn(&SynFieldWrapper) -> Ts2| -> Ts2 {
        let fields_ts = fields.iter().map(fn0);
        quote! {#(#fields_ts),*}
    };
    let gen_fields_named_without_comma_ts = |fn0: &dyn Fn(&SynFieldWrapper) -> Ts2| -> Ts2 {
        let fields_ts = fields.iter().map(fn0);
        quote! {#(#fields_ts)*}
    };
    let gen_fields_named_without_pk_with_comma_ts = |fn0: &dyn Fn(&SynFieldWrapper) -> Ts2| -> Ts2 {
        let fields_ts = fields_without_pk.iter().map(fn0);
        quote! {#(#fields_ts),*}
    };
    let gen_fields_named_without_pk_without_comma_ts =
        |fn0: &dyn Fn(&SynFieldWrapper) -> Ts2| -> Ts2 {
            let fields_ts = fields_without_pk.iter().map(fn0);
            quote! {#(#fields_ts)*}
        };
    let gen_match_ok_err_ts = |ts0: &dyn ToTokens,
                               ts1: &dyn ToTokens,
                               ts2: &dyn ToTokens,
                               ts3: &dyn ToTokens,
                               ts4: &dyn ToTokens| {
        quote! {
            match #ts0 {
                Ok(#ts1) => #ts2,
                Err(#ts3) => #ts4
            }
        }
    };
    let gen_match_ok_err_ts_c35d87fd =
        |expr: &dyn ToTokens, ok: &dyn ToTokens, err_ts: &dyn ToTokens| {
            gen_match_ok_err_ts(&expr, &ok, &ok, &Er0, &quote! {{ #err_ts }})
        };
    let none_ts = quote! {None};
    let fields_named_with_comma_none_ts =
        gen_fields_named_with_comma_ts(&|_: &SynFieldWrapper| -> Ts2 { none_ts.clone() });
    let fields_named_without_pk_with_comma_none_ts =
        gen_fields_named_without_pk_with_comma_ts(&|_: &SynFieldWrapper| -> Ts2 {
            none_ts.clone()
        });
    let gen_acc_string_pop_ts = |acc_ts: &dyn ToTokens, ts: &dyn ToTokens| {
        let opt_char_ts = gen_opt_type_decl_ts(&Char);
        quote! {
            let mut #acc_ts = #StringTs::new();
            #ts
            let _: #opt_char_ts = #acc_ts.pop();
        }
    };
    let gen_acc_string_pop_acc_ts = |acc_ts: &dyn ToTokens, ts: &dyn ToTokens| {
        let ts0 = gen_acc_string_pop_ts(acc_ts, ts);
        quote! {
            #ts0
            #acc_ts
        }
    };
    let gen_acc_string_pop_ok_acc_ts = |acc_ts: &dyn ToTokens, ts: &dyn ToTokens| {
        let ts0 = gen_acc_string_pop_ts(acc_ts, ts);
        quote! {
            #ts0
            Ok(#acc_ts)
        }
    };
    let mut impl_ident_vec_ts = Vec::new();
    let mut operation_routes_ts = Vec::new();
    let impl_ident_ts = {
        let ident_prepare_pg_er_ucc = SelfPreparePgErUcc::from_tokens(&ident);
        let ts = quote! {
            #[eo_to_err_string]
            er: sqlx::Error,
            loc: location_lib::loc::Loc,
        };
        let ident_prepare_pg_er_ts = StructOrEnumDeriveTsStreamBuilder::new()
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
        let fn_pk_ts = {
            let pk_fi_dq_ts = dq_ts(&pk_fi);
            quote! {
                const fn #PkSc() -> &'static str {
                    #pk_fi_dq_ts
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
                let gen_ft_as_pg_crud_create_table_column_query_part_create_table_query_part_ts =
                    |ft: &Type, fi: &Ident, is_pk: bool| {
                        let is_pk_ts: &dyn ToTokens = if is_pk { &TrueSc } else { &FalseSc };
                        let fi_dq_ts = dq_ts(&fi);
                        let ft_pg_type_ts = gen_as_pg_type_path_ts(&ft);
                        quote! {
                            #ft_pg_type_ts #CreateTableColumnQueryPartSc(&#fi_dq_ts, #is_pk_ts)
                        }
                    };
                once(
                    gen_ft_as_pg_crud_create_table_column_query_part_create_table_query_part_ts(
                        pk_ft,
                        &pk_field.ident,
                        true,
                    ),
                )
                .chain(fields_without_pk.iter().map(|el| {
                    gen_ft_as_pg_crud_create_table_column_query_part_create_table_query_part_ts(
                        &el.type0, &el.ident, false,
                    )
                }))
                .collect::<Vec<Ts2>>()
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
            let vrts_ts = gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| {
                let fi_ucc_ts = ToTokensToUccTs::case_or_panic(&el.ident);
                let init_ts = {
                    let fi_string_dq_ts = dq_ts(&el.ident);
                    let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                    let ts0 = gen_match_ok_err_ts_c35d87fd(
                        &quote! {#as_pg_crud_pg_type_pg_type_ts #SelectQueryPartSc(
                            #ColumnSc,
                            #fi_string_dq_ts
                        )},
                        &quote! {v_820e1163},
                        &quote! {{
                            return Err(#Er0);
                        }},
                    );
                    quote! {=> #ts0}
                };
                quote! {#ident_select_ucc::#fi_ucc_ts(#ColumnSc) #init_ts}
            });
            let ts0 = gen_acc_string_pop_ok_acc_ts(
                &quote! {acc},
                &quote! {
                    for el in #SelectSc.to_vec() {
                        acc.push_str(&match el {
                            #vrts_ts
                        });
                        acc.push(',');
                    }
                },
            );
            quote! {
                fn #GenSelectQueryPartSc(#select_borrow_pg_crud_not_empty_unique_vec_ident_select_ts) -> Result<#StringTs, #import_ts #QueryPartErUcc> {
                    #ts0
                }
            }
        };
        impl_ident_vec_ts.push(quote! {
            #pub_fn_table_ts
            #fn_pk_ts
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
    let wrap_into_axum_res_ts =
        |axum_json_ts: &dyn ToTokens,
         status_code_ts: &dyn ToTokens,
         should_add_return: &ShouldAddReturn| {
            let return_ts = match should_add_return {
                ShouldAddReturn::False => quote! {res},
                ShouldAddReturn::True => quote! {return res;},
            };
            quote! {
                let mut res = axum::response::IntoResponse::into_response(
                    axum::Json(#axum_json_ts)
                );
                *res.status_mut() = #status_code_ts;
                #return_ts
            }
        };
    let gen_ident_operation_er_ucc = |operation: &Operation| {
        format!("{ident}{operation}Er")
            .parse::<Ts2>()
            .expect("79ab147e")
    };
    let gen_ident_operation_res_vrts_ucc = |operation: &Operation| {
        format!("{ident}{operation}ResVrts")
            .parse::<Ts2>()
            .expect("f386c0d4")
    };
    let gen_init_ts = |syn_vrt_wrapper: &SynVrtWrapper, location: &'static Location<'_>| -> Ts2 {
        let vrt_ident = &syn_vrt_wrapper.vrt.ident;
        let fields_ts = if let Fields::Named(v) = &syn_vrt_wrapper.vrt.fields {
            v.named.iter().enumerate().map(|(i, el)| {
                let fi = &el.ident;
                if *fi.as_ref().expect("edbbd08a") == LocSc.to_string() {
                    gen_field_loc_new_ts(location.file(), location.line(), location.column())
                } else {
                    let er_incr_sc = ErSelfSc::from_display(&i);
                    quote! {#fi: #er_incr_sc}
                }
            })
        } else {
            panic!("10773d36");
        };
        quote! {
            #vrt_ident {
                #(#fields_ts),*
            }
        }
    };
    let gen_operation_er_init_eprintln_res_creation_ts =
        |operation: &Operation,
         syn_vrt_wrapper: &SynVrtWrapper,
         location: &'static Location<'_>| {
            let ident_operation_er_ucc = gen_ident_operation_er_ucc(operation);
            let ident_operation_res_vrts_ucc = gen_ident_operation_res_vrts_ucc(operation);
            let syn_vrt_init_ts = gen_init_ts(syn_vrt_wrapper, location);
            let ts = wrap_into_axum_res_ts(
                &quote! {#ident_operation_res_vrts_ucc::#FromHandleSc(#ErSc)},
                &syn_vrt_wrapper
                    .get_opt_status_code()
                    .expect("81efa954")
                    .to_http_status_code_ts(),
                &ShouldAddReturn::True,
            );
            quote! {
                let #ErSc = #ident_operation_er_ucc::#syn_vrt_init_ts;
                // eprintln!("{er}");
                #ts
            }
        };
    let new_syn_vrt_wrapper = |vrt_name: &dyn Display,
                               status_code: Option<StatusCode>,
                               fields_cd1fd715: Vec<(
        LocationFieldAttr,
        &dyn Display,
        Punctuated<PathSegment, PathSep>,
    )>|
     -> SynVrtWrapper {
        SynVrtWrapper {
            vrt: Variant {
                attrs: {
                    let mut attrs = Vec::new();
                    if let Some(v) = status_code.as_ref() {
                        let mut segments = Punctuated::new();
                        segments.push(PathSegment {
                            ident: Ident::new(
                                &AsRefStrToScStr::case(v),
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
                ident: Ident::new(&vrt_name.to_string(), proc_macro2::Span::call_site()),
                fields: Fields::Named(FieldsNamed {
                    brace_token: Brace::default(),
                    named: {
                        let mut acc =
                            fields_cd1fd715
                                .into_iter()
                                .fold(Punctuated::new(), |mut acc, el| {
                                    acc.push_value(Field {
                                        attrs: vec![Attribute {
                                            pound_token: Pound {
                                                spans: [proc_macro2::Span::call_site()],
                                            },
                                            style: AttrStyle::Outer,
                                            bracket_token: Bracket::default(),
                                            meta: Meta::Path(Path {
                                                leading_colon: None,
                                                segments: {
                                                    let mut acc0 = Punctuated::new();
                                                    acc0.push(PathSegment {
                                                        ident: Ident::new(
                                                            AttrIdentStr::attr_ident_str(&el.0),
                                                            proc_macro2::Span::call_site(),
                                                        ),
                                                        arguments: PathArguments::None,
                                                    });
                                                    acc0
                                                },
                                            }),
                                        }],
                                        vis: Visibility::Inherited,
                                        mutability: FieldMutability::None,
                                        ident: Some(Ident::new(
                                            &el.1.to_string(),
                                            proc_macro2::Span::call_site(),
                                        )),
                                        colon_token: Some(Colon {
                                            spans: [proc_macro2::Span::call_site()],
                                        }),
                                        ty: Type::Path(TypePath {
                                            qself: None,
                                            path: Path {
                                                leading_colon: None,
                                                segments: el.2,
                                            },
                                        }),
                                    });
                                    acc.push_punct(Comma {
                                        spans: [proc_macro2::Span::call_site()],
                                    });
                                    acc
                                });
                        acc.push_value(loc_syn_field());
                        acc
                    },
                }),
                discriminant: None,
            },
            status_code,
        }
    };
    let query_part_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &QueryPartUcc,
        Some(StatusCode::BadReq400),
        vec![(
            LocationFieldAttr::EoLocation,
            &ErSc,
            gen_simple_syn_punct(&[&PgCrudSc.to_string(), &QueryPartErUcc.to_string()]),
        )],
    );
    let gen_select_query_part_params_payload_select_ts = |operation: &Operation| {
        gen_match_ok_err_ts_c35d87fd(
            &quote! {Self::#GenSelectQueryPartSc(&#ParamsSc.#PayloadSc.#SelectSc)},
            &quote! {v_357219fb},
            &{
                let ts = gen_operation_er_init_eprintln_res_creation_ts(
                    operation,
                    &query_part_syn_vrt_wrapper,
                    Location::caller(),
                );
                quote! {{#ts}}
            },
        )
    };
    let ident_read_ucc = SelfReadUcc::from_tokens(&ident);
    let gen_v_decl_ts0 = |ts: &dyn ToTokens| gen_v_decl_ts(&import, &ts);
    let gen_v_init_ts0 = |ts: &dyn ToTokens| gen_v_init_ts(&import, &ts);
    let gen_impl_pg_crud_default_opt_some_vec_one_el_for_tokens_no_lifetime_ts =
        |ident_4d69a809: &dyn ToTokens, ts: &dyn ToTokens| {
            gen_impl_pg_crud_default_opt_some_vec_one_el_ts(&ident_4d69a809, &Ts2::new(), &ts)
        };
    let gen_fi_default_opt_some_vec_one_el_call_ts =
        |ts: &dyn ToTokens| quote! {#ts: #PgCrudDefaultOptSomeVecOneElCall};
    let gen_match_query_bind_or_err_ts =
        |expr: &dyn ToTokens, ok_binding: &dyn ToTokens, err_ts: &dyn ToTokens| {
            gen_match_ok_err_ts(
                &expr,
                &ok_binding,
                &quote! {{
                    #QuerySc = #ok_binding;
                }},
                &Er0,
                &quote! {{#err_ts}},
            )
        };
    let gen_if_let_some_ts = |ts0: &dyn ToTokens, ts1: &dyn ToTokens, ts2: &dyn ToTokens| {
        quote! {
            if let Some(#ts0) = #ts1 {
                #ts2
            }
        }
    };
    let ident_create_ucc = SelfCreateUcc::from_tokens(&ident);
    let ident_create_ts = {
        let ident_create_ts = StructOrEnumDeriveTsStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .derive_utoipa_to_schema()
            .build_struct(&ident_create_ucc, &Ts2::new(), &{
                let ts = gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    let el_syn_field_ty_as_pg_type_create_ts = gen_as_pg_type_create_ts(&el.type0);
                    quote! {
                        pub #fi: #el_syn_field_ty_as_pg_type_create_ts
                    }
                });
                quote! {{#ts}}
            });
        let impl_ident_create_ts = {
            let pk_ft_as_default_opt_some_vec_one_el_call_ts = {
                let pk_ft_as_pg_type_create_ts = gen_as_pg_type_create_ts(&pk_ft);
                quote! {
                    <
                        #pk_ft_as_pg_type_create_ts as #import_ts #DefaultOptSomeVecOneElUcc
                    >::#DefaultOptSomeVecOneElSc()
                }
            };
            let fn_create_query_part_ts = {
                let gen_match_as_pg_crud_pg_type_pg_type_create_query_part_ts =
                    |ft: &Type, ts: &dyn ToTokens| {
                        gen_match_ok_err_ts(
                            &{
                                let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&ft);
                                quote! {#as_pg_crud_pg_type_pg_type_ts #CreateQueryPartSc(
                                    &#ts,
                                    #IncrSc
                                )}
                            },
                            &quote! {v_c3f0b59a},
                            &{
                                let if_write_is_err_ts = gen_if_write_is_err_ts(
                                    &quote! {acc, "{v_c3f0b59a},"},
                                    &return_err_query_part_er_write_into_buffer_ts,
                                );
                                quote! {{
                                    #if_write_is_err_ts
                                }}
                            },
                            &Er0,
                            &quote! {{
                                return Err(#Er0);
                            }},
                        )
                    };
                let pk_ts = gen_match_as_pg_crud_pg_type_pg_type_create_query_part_ts(
                    pk_ft,
                    &pk_ft_as_default_opt_some_vec_one_el_call_ts,
                );
                let column_incrs_ts =
                    gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                        gen_match_as_pg_crud_pg_type_pg_type_create_query_part_ts(&el.type0, &{
                            let el_fi = &el.ident;
                            quote! {self.#el_fi}
                        })
                    });
                let ts = gen_acc_string_pop_ok_acc_ts(
                    &quote! {acc},
                    &quote! {
                        #pk_ts
                        #column_incrs_ts
                    },
                );
                quote! {
                    fn #CreateQueryPartSc(&self, #IncrSc: &mut u64) -> Result<#StringTs, #import_ts #QueryPartErUcc> {
                        #ts
                    }
                }
            };
            let fn_create_query_bind_ts = {
                let gen_query_as_pg_crud_pg_type_pg_type_create_query_bind_ts =
                    |ft: &Type, ts: &dyn ToTokens| {
                        gen_match_query_bind_or_err_ts(
                            &{
                                let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&ft);
                                quote! {#as_pg_crud_pg_type_pg_type_ts #CreateQueryBindSc(#ts,#QuerySc)}
                            },
                            &quote! {v_3c55d2e1},
                            &quote! {return Err(#Er0);},
                        )
                    };
                let pk_ts = gen_query_as_pg_crud_pg_type_pg_type_create_query_bind_ts(
                    pk_ft,
                    &pk_ft_as_default_opt_some_vec_one_el_call_ts,
                );
                let binded_query_modifications_ts =
                    gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                        gen_query_as_pg_crud_pg_type_pg_type_create_query_bind_ts(&el.type0, &{
                            let fi = &el.ident;
                            quote! {self.#fi}
                        })
                    });
                quote! {
                    fn #CreateQueryBindSc(self, mut #QuerySc: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> Result<
                        sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
                        String
                    > {
                        #pk_ts
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
        let impl_pg_crud_default_opt_some_vec_one_el_for_ident_create_ts =
            gen_impl_pg_crud_default_opt_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_create_ucc,
                &{
                    let fields_init_without_pk_with_default_opt_some_vec_one_el_ts =
                        gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| {
                            gen_fi_default_opt_some_vec_one_el_call_ts(&el.ident)
                        });
                    quote! {
                        Self{#fields_init_without_pk_with_default_opt_some_vec_one_el_ts}
                    }
                },
            );
        quote! {
            #ident_create_ts
            #impl_ident_create_ts
            #impl_pg_crud_default_opt_some_vec_one_el_for_ident_create_ts
        }
    };
    let ident_where_many_ucc = SelfWhereManyUcc::from_tokens(&ident);
    let ident_where_many_try_new_er_ucc = SelfWhereManyTryNewErUcc::from_tokens(&ident);
    let ident_where_many_ts = {
        let fields_decl_ts = gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| -> Ts2 {
            let fi = &el.ident;
            let el_syn_field_ty_as_pg_type_where_ts = gen_as_pg_type_where_ts(&el.type0);
            let opt_pg_type_where_syn_field_ty_as_pg_type_where_ts = gen_opt_type_decl_ts(
                &quote! {#import_ts PgTypeWhere<#el_syn_field_ty_as_pg_type_where_ts>},
            );
            quote! {
                #fi: #opt_pg_type_where_syn_field_ty_as_pg_type_where_ts
            }
        });
        let ident_where_many_ts = {
            let ts_2ecd6da8 = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_serde_serialize()
                .derive_utoipa_to_schema()
                .build_struct(
                    &ident_where_many_ucc,
                    &Ts2::new(),
                    &quote! {{#fields_decl_ts}},
                );
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_2ecd6da8
            }
        };
        let ident_where_many_try_new_er_ts = StructOrEnumDeriveTsStreamBuilder::new()
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
            &Ts2::new(),
            &ident_where_many_ucc,
            &fields_decl_ts,
            &ident_where_many_try_new_er_ucc,
            &{
                let gen_fields_ts = |should_add_borrow: ShouldAddBorrow| {
                    gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| -> Ts2 {
                        let fi = &el.ident;
                        quote! {#should_add_borrow #fi}
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
                    .map(|el| (&el.ident, &el.type0))
                    .collect::<Vec<(&Ident, &Type)>>(),
                fields_len,
                &|_: &Ident, syn_type: &Type| {
                    let syn_type_as_pg_type_where_ts = gen_as_pg_type_where_ts(&syn_type);
                    gen_opt_type_decl_ts(
                        &quote! {#import_ts PgTypeWhere<#syn_type_as_pg_type_where_ts>},
                    )
                },
            );
        let impl_pg_crud_default_opt_some_vec_one_el_for_ident_where_many_ts =
            gen_impl_pg_crud_default_opt_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_where_many_ucc,
                &{
                    let fields_ts = gen_fields_named_without_comma_ts(&|el: &SynFieldWrapper| {
                        let fi = &el.ident;
                        quote! {
                            #fi: Some(
                                #PgCrudDefaultOptSomeVecOneElCall
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
            #impl_pg_crud_default_opt_some_vec_one_el_for_ident_where_many_ts
        }
    };
    let opt_ident_where_many_ucc = StdOptOptSelfWhereManyUcc::from_tokens(&ident);
    let opt_ident_where_many_ts = {
        let opt_ident_where_many_ts = StructOrEnumDeriveTsStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_clone()
            .derive_serde_serialize()
            .derive_serde_deserialize()
            .derive_utoipa_to_schema()
            .build_struct(&opt_ident_where_many_ucc, &Ts2::new(), &{
                let opt_ident_read_only_ids_stdrt_not_null_ts =
                    gen_opt_type_decl_ts(&ident_where_many_ucc);
                quote! {(pub #opt_ident_read_only_ids_stdrt_not_null_ts);}
            });
        let impl_pg_type_where_filter_for_opt_ident_where_many_ts =
            impl_pg_type_where_filter_for_ident_ts(
                &quote! {<'lifetime>},
                &opt_ident_where_many_ucc,
                &Ts2::new(),
                &IncrParamUnderscore::False,
                &ColumnParamUnderscore::True,
                &IsNeedToAddOperatorUnderscore::True,
                &{
                    let extra_params_modification_ts = fields.iter().enumerate().map(|(i, el)| {
                        let fi = &el.ident;
                        gen_if_let_some_ts(&quote! {v_da0f0616}, &quote! {&#VSc.#fi}, &gen_match_ok_err_ts(
                                &{
                                    let fi_dq_ts = dq_ts(&fi);
                                    quote!{#import_ts PgTypeWhereFilter::query_part(
                                        v_da0f0616,
                                        incr,
                                        &#fi_dq_ts,
                                        is_first_push_to_extra_params_already_happend,
                                    )}
                                },
                                &quote!{v_9e3f8fdd},
                                &{
                                    let ts = if i == fields_len_without_pk {
                                        Ts2::new()
                                    } else {
                                        quote! {is_first_push_to_extra_params_already_happend = true;}
                                    };
                                    quote!{{
                                        #ExtraParamsSc.push_str(&v_9e3f8fdd);
                                        #ts
                                    }}
                                },
                                &Er0,
                                &quote!{{
                                    return Err(#Er0);
                                }}
                            ))
                    });
                    quote! {
                        Ok(match &self.0 {
                            Some(#VSc) => {
                                let mut #ExtraParamsSc = #StringTs::from("where");
                                let mut is_first_push_to_extra_params_already_happend = false;
                                #(#extra_params_modification_ts)*
                                #ExtraParamsSc
                            },
                            None => #StringTs::default()
                        })
                    }
                },
                &IsQueryBindMutable::True,
                &{
                    let ts = gen_if_let_some_ts(
                        &quote! {v_27176ffb},
                        &quote! {self.0},
                        &gen_fields_named_without_comma_ts(&|el: &SynFieldWrapper| {
                            let fi = &el.ident;
                            gen_if_let_some_ts(
                                &quote! {v_b12d6fe0},
                                &quote! {v_27176ffb.#fi},
                                &gen_match_query_bind_or_err_ts(
                                    &quote! {#import_ts PgTypeWhereFilter::query_bind(v_b12d6fe0, #QuerySc)},
                                    &quote! {v_edaee3b2},
                                    &quote! {return Err(#Er0);},
                                ),
                            )
                        }),
                    );
                    quote! {
                        #ts
                        Ok(#QuerySc)
                    }
                },
                &Import::PgCrud,
            );
        let impl_pg_crud_default_opt_some_vec_one_el_for_opt_ident_where_many_ts =
            gen_impl_pg_crud_default_opt_some_vec_one_el_for_tokens_no_lifetime_ts(
                &opt_ident_where_many_ucc,
                &quote! {Self(Some(#PgCrudDefaultOptSomeVecOneElCall))},
            );
        quote! {
            #opt_ident_where_many_ts
            #impl_pg_type_where_filter_for_opt_ident_where_many_ts
            #impl_pg_crud_default_opt_some_vec_one_el_for_opt_ident_where_many_ts
        }
    };
    let pub_where_many_opt_ident_where_many_ts =
        quote! {pub #WhereManySc: #opt_ident_where_many_ucc};
    let where_many_pg_crud_default_opt_some_vec_one_el_call_ts =
        gen_fi_default_opt_some_vec_one_el_call_ts(&WhereManySc);
    let gen_read_or_delete_many_extra_params_init_ts =
        |read_many_or_delete_many: &ReadManyOrDeleteMany| {
            gen_match_ok_err_ts_c35d87fd(
                &quote! {#import_ts PgTypeWhereFilter::query_part(
                    &#ParamsSc.#PayloadSc.#WhereManySc,
                    &mut #IncrSc,
                    &"",//useless //todo check if can be optimized
                    false//useless
                )},
                &quote! {v_d1627695},
                &{
                    let ts_b34ec240 = gen_operation_er_init_eprintln_res_creation_ts(
                        &Operation::from(read_many_or_delete_many),
                        &query_part_syn_vrt_wrapper,
                        Location::caller(),
                    );
                    quote! {{
                        #ts_b34ec240
                    }}
                },
            )
        };
    let macros_helpers_location_location_field_attr_eo_to_err_string_serde =
        LocationFieldAttr::EoToErrStringSerde;
    let string_syn_punct = gen_simple_syn_punct(&["String"]);
    let try_bind_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &TryBindUcc,
        Some(StatusCode::InternalServerEr500),
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string_serde,
            &TryBindSc,
            string_syn_punct.clone(),
        )],
    );
    let gen_query_pg_type_where_filter_query_bind_params_payload_where_many_query_ts =
        |operation: &Operation| {
            gen_match_query_bind_or_err_ts(
                &quote! {#import_ts PgTypeWhereFilter::query_bind(#ParamsSc.#PayloadSc.#WhereManySc, #QuerySc)},
                &quote! {v_03a58371},
                &gen_operation_er_init_eprintln_res_creation_ts(
                    operation,
                    &try_bind_syn_vrt_wrapper,
                    Location::caller(),
                ),
            )
        };
    let try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_sc =
        TryFromSqlxPgPgRowWithNotEmptyUniqueVecSelfSelectSc::from_display(&ident);
    let simple_syn_punct_sqlx_error = gen_simple_syn_punct(&["sqlx", "Error"]);
    let macros_helpers_location_location_field_attr_eo_to_err_string =
        LocationFieldAttr::EoToErrString;
    let pg_syn_vrt_wrapper = new_syn_vrt_wrapper(
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
            gen_match_ok_err_ts_c35d87fd(
                &quote! {#ident_read_ucc::#try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_sc(
                    &v_b27d7d79,
                    &#ParamsSc.#PayloadSc.#SelectSc
                )},
                &quote! {v_90535a1d},
                &{
                    let ts_995d3d1d = gen_operation_er_init_eprintln_res_creation_ts(
                        &Operation::from(read_many_or_read_one),
                        &pg_syn_vrt_wrapper,
                        Location::caller(),
                    );
                    quote! {{
                        #ts_995d3d1d
                    }}
                },
            )
        };
    let select_ts = {
        let ident_select_ts = {
            let ts_179037cd = StructOrEnumDeriveTsStreamBuilder::new()
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
                    let vrts = gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| {
                        let serde_ident_ts = dq_ts(&el.ident);
                        let fi_ucc_ts = ToTokensToUccTs::case_or_panic(&el.ident);
                        let el_syn_field_ty_as_pg_type_select_ts = gen_as_pg_type_select_ts(&el.type0);
                        quote! {
                            #[serde(rename(serialize = #serde_ident_ts, deserialize = #serde_ident_ts))]
                            #fi_ucc_ts(#el_syn_field_ty_as_pg_type_select_ts)
                        }
                    });
                    quote!{{#vrts}}
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
        let impl_pg_crud_all_vrts_default_opt_some_vec_one_el_for_ident_select_ts =
            gen_impl_pg_crud_all_vrts_default_opt_some_vec_one_el_ts(&ident_select_ucc, &{
                let els_ts = gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| {
                    let fi_ucc_ts = ToTokensToUccTs::case_or_panic(&el.ident);
                    quote! {
                        Self::#fi_ucc_ts(#PgCrudDefaultOptSomeVecOneElCall)
                    }
                });
                quote! {vec![#els_ts]}
            });
        quote! {
            #ident_select_ts
            #impl_display_for_ident_select_ts
            #impl_location_lib_to_err_string_for_ident_select_ts
            #impl_pg_crud_all_vrts_default_opt_some_vec_one_el_for_ident_select_ts
        }
    };
    let select_pg_crud_default_opt_some_vec_one_el_call_ts =
        gen_fi_default_opt_some_vec_one_el_call_ts(&SelectSc);
    let ident_read_ts = {
        let ident_read_ts = {
            let ts_f80f1f3e = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(&ident_read_ucc, &Ts2::new(), &{
                    let field_opt_pk_ts = {
                        let opt_v_pk_ft_as_pg_type_read_ts =
                            gen_opt_type_decl_ts(&gen_v_decl_ts0(&gen_as_pg_type_read_ts(&pk_ft)));
                        quote! {
                            #FieldAttrSerdeSkipSerializingIfOptIsNone
                            pub #pk_fi: #opt_v_pk_ft_as_pg_type_read_ts
                        }
                    };
                    let fields_opts_without_pk_ts =
                        gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| -> Ts2 {
                            let field_vis = &el.vis;
                            let fi = &el.ident;
                            let opt_v_ft_as_pg_type_read_ts = gen_opt_type_decl_ts(
                                &gen_v_decl_ts0(&gen_as_pg_type_read_ts(&el.type0)),
                            );
                            quote! {
                                #FieldAttrSerdeSkipSerializingIfOptIsNone
                                #field_vis #fi: #opt_v_ft_as_pg_type_read_ts
                            }
                        });
                    quote! {{
                        #field_opt_pk_ts,
                        #fields_opts_without_pk_ts
                    }}
                });
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_f80f1f3e
            }
        };
        let impl_ident_read_ts = {
            let fn_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts = {
                let decl_pk_ts = {
                    let opt_v_pk_ft_as_pk_ts =
                        gen_opt_type_decl_ts(&gen_v_decl_ts0(&pk_ft_as_pg_type_read_ucc));
                    quote! {
                        let mut #pk_fi: #opt_v_pk_ft_as_pk_ts = None;
                    }
                };
                let decl_without_pk_ts =
                    gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                        let fi = &el.ident;
                        let opt_v_ft_as_pg_type_read_ts = gen_opt_type_decl_ts(&gen_v_decl_ts0(
                            &gen_as_pg_type_read_ts(&el.type0),
                        ));
                        quote! {
                            let mut #fi: #opt_v_ft_as_pg_type_read_ts = None;
                        }
                    });
                let (assignment_vrt_pk_ts, assignment_vrts_without_pk_ts) = {
                    let gen_assignment_ts =
                        |variant_ucc_ts: &dyn ToTokens,
                         pg_type_read_ts: &dyn ToTokens,
                         fi_string_dq_ts: &dyn ToTokens,
                         fi: &dyn ToTokens| {
                            let ts = gen_match_ok_err_ts(
                                &quote! {sqlx::Row::try_get::<
                                    #pg_type_read_ts,
                                    #RefStr
                                >(
                                    #VSc,
                                    #fi_string_dq_ts
                                )},
                                &quote! {v_470178a2},
                                &quote! {{
                                    #fi = Some(#import_ts #VUcc { #VSc: v_470178a2 });
                                }},
                                &Er0,
                                &quote! {{
                                    return Err(#Er0);
                                }},
                            );
                            quote! {#ident_select_ucc::#variant_ucc_ts(_) => #ts}
                        };
                    (
                        gen_assignment_ts(
                            &pk_fi_ucc_ts,
                            &pk_ft_as_pg_type_read_ucc,
                            &dq_ts(&pk_fi),
                            &pk_fi,
                        ),
                        fields_without_pk
                            .iter()
                            .map(|el| {
                                gen_assignment_ts(
                                    &ToTokensToUccTs::case_or_panic(&el.ident),
                                    &gen_as_pg_type_read_ts(&el.type0),
                                    &dq_ts(&el.ident),
                                    &el.ident,
                                )
                            })
                            .collect::<Vec<Ts2>>(),
                    )
                };
                let fields_init_ts = &fields.iter().map(|el| &el.ident).collect::<Vec<&Ident>>();
                quote! {
                    fn #try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_sc(
                        #VSc: &sqlx::postgres::PgRow,
                        #select_borrow_pg_crud_not_empty_unique_vec_ident_select_ts
                    ) -> Result<Self, sqlx::Error> {
                        #decl_pk_ts
                        #decl_without_pk_ts
                        for el_dca9f0b7 in #SelectSc.to_vec() {
                            match el_dca9f0b7 {
                                #assignment_vrt_pk_ts,
                                #(#assignment_vrts_without_pk_ts),*
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
            let ts_472e3ebf = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone()
                .derive_partial_eq()
                .derive_serde_serialize()
                .derive_serde_deserialize()
                .build_struct(&ident_read_only_ids_ucc, &Ts2::new(), &{
                    enum WrapIntoOpt {
                        False,
                        True,
                    }
                    let gen_field_ts =
                        |fi: &dyn ToTokens, ft: &dyn ToTokens, wrap_into_opt: &WrapIntoOpt| {
                            let ft_ts = match &wrap_into_opt {
                                WrapIntoOpt::False => gen_as_pg_type_read_only_ids_ts(&ft),
                                WrapIntoOpt::True => {
                                    gen_opt_type_decl_ts(&gen_as_pg_type_read_only_ids_ts(&ft))
                                }
                            };
                            quote! {pub #fi: #ft_ts}
                        };
                    let pk_ts = gen_field_ts(&pk_fi, &pk_ft, &WrapIntoOpt::False);
                    let ts = gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| {
                        gen_field_ts(&el.ident, &el.type0, &WrapIntoOpt::True)
                    });
                    quote! {{
                        #pk_ts,
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
            let where_fts_ts = gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| {
                let ft = &el.type0;
                let el_syn_field_ty_as_pg_type_read_only_ids_ts =
                    gen_as_pg_type_read_only_ids_ts(&ft);
                quote! {
                    #el_syn_field_ty_as_pg_type_read_only_ids_ts: ::sqlx::decode::Decode<'lifetime, R::Database>
                }
            });
            let pk_ts = {
                let el_syn_field_ty_as_pg_type_read_only_ids_ts =
                    gen_as_pg_type_read_only_ids_ts(&pk_ft);
                let fi_dq_ts = dq_ts(&pk_fi);
                let ts = gen_match_ok_err_ts_c35d87fd(
                    &quote! {sqlx::Row::try_get::<#el_syn_field_ty_as_pg_type_read_only_ids_ts, &str>(
                        #undescore_underscore_row,
                        #fi_dq_ts
                    )},
                    &quote! {v_283179dd},
                    &quote! {{
                        return Err(#Er0);
                    }},
                );
                quote! {
                    let #pk_fi = #ts;
                }
            };
            let fields_init_ts =
                gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let fi_dq_ts = dq_ts(&quote! {#fi});
                    let el_syn_field_ty_as_pg_type_read_only_ids_ts =
                        gen_as_pg_type_read_only_ids_ts(&ft);
                    quote! {
                        let #fi = sqlx::Row::try_get::<
                            #el_syn_field_ty_as_pg_type_read_only_ids_ts,
                            &str
                        >(#undescore_underscore_row, #fi_dq_ts).ok();
                    }
                });
            let self_fields_ts = gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| {
                let fi = &el.ident;
                quote! {#fi}
            });
            quote! {
                impl<'lifetime, R: ::sqlx::Row<Database = sqlx::Postgres>> ::sqlx::FromRow<'lifetime, R> for #ident_read_only_ids_ucc
                where
                    &'lifetime ::std::primitive::str: ::sqlx::ColumnIndex<R>,
                    #where_fts_ts
                {
                    fn from_row(#undescore_underscore_row: &'lifetime R) -> ::sqlx::Result<Self> {
                        #pk_ts
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
    let pg_crud_order_by_ts = quote! {#import_ts #OrderByUcc};
    let ident_update_ucc = SelfUpdateUcc::from_tokens(&ident);
    let ident_update_many_params_ucc = SelfUpdateManyParamsUcc::from_tokens(&ident);
    let ident_update_many_payload_ucc = SelfUpdateManyPayloadUcc::from_tokens(&ident);
    let ident_update_try_new_er_ucc = SelfUpdateTryNewErUcc::from_tokens(&ident);
    let ident_update_for_query_ucc = SelfUpdateForQueryUcc::from_tokens(&ident);
    let ident_update_ts = {
        let gen_opt_v_ft_as_pg_type_update_ts = |syn_type: &Type| {
            let path_v_ts = format!("{PgCrudSc}::{VUcc}")
                .parse::<Ts2>()
                .expect("dbdbb7f2");
            let syn_type_as_pg_type_update_ts = gen_as_pg_type_update_ts(&syn_type);
            gen_opt_type_decl_ts(&quote! {#path_v_ts<#syn_type_as_pg_type_update_ts>})
        };
        let fields_decl_ts = {
            let fields_named_without_pk_ts =
                gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| -> Ts2 {
                    let fi = &el.ident;
                    let opt_v_ft_as_pg_type_update_ts =
                        gen_opt_v_ft_as_pg_type_update_ts(&el.type0);
                    quote! {
                        #fi: #opt_v_ft_as_pg_type_update_ts
                    }
                });
            quote! {
                #pk_fi: #pk_ft_update_ts,
                #fields_named_without_pk_ts
            }
        };
        let ident_update_ts = {
            let ts_a09c0471 = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_serde_serialize()
                .derive_utoipa_to_schema()
                .build_struct(&ident_update_ucc, &Ts2::new(), &quote! {{#fields_decl_ts}});
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_a09c0471
            }
        };
        let ident_update_try_new_er_ts = StructOrEnumDeriveTsStreamBuilder::new()
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
            &quote! {#[allow(clippy::redundant_pattern_matching)]}, //todo check if 1 then different logic
            &ident_update_ucc,
            &fields_decl_ts,
            &ident_update_try_new_er_ucc,
            &{
                let (left_ts, right_ts) = {
                    let gen_ts =
                        |ts: &dyn ToTokens| mb_wrap_into_braces_ts(ts, fields_len_without_pk > 1);
                    (
                        gen_ts(&gen_fields_named_without_pk_with_comma_ts(
                            &|el: &SynFieldWrapper| -> Ts2 {
                                let fi = &el.ident;
                                quote! {&#fi}
                            },
                        )),
                        gen_ts(&fields_named_without_pk_with_comma_none_ts),
                    )
                };
                let fields_inialization_ts =
                    gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| -> Ts2 {
                        let fi = &el.ident;
                        quote! {#fi}
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
                .map(|el| (&el.ident, &el.type0))
                .collect::<Vec<(&Ident, &Type)>>(),
            fields_len,
            &|syn_ident: &Ident, syn_type: &Type| {
                if syn_ident == pk_fi {
                    quote! {#pk_ft_update_ts}
                } else {
                    gen_opt_v_ft_as_pg_type_update_ts(syn_type)
                }
            },
        );
        let impl_pg_crud_default_opt_some_vec_one_el_for_ident_update_ts =
            gen_impl_pg_crud_default_opt_some_vec_one_el_for_tokens_no_lifetime_ts(
                &ident_update_ucc,
                &{
                    let ts = gen_fi_default_opt_some_vec_one_el_call_ts(&pk_fi);
                    let fields_without_pk_with_default_opt_some_vec_one_el_ts =
                        gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| {
                            let fi = &el.ident;
                            let ts0 = gen_v_init_ts0(&PgCrudDefaultOptSomeVecOneElCall);
                            quote! {#fi: Some(#ts0)}
                        });
                    quote! {Self{
                        #ts,
                        #fields_without_pk_with_default_opt_some_vec_one_el_ts
                    }}
                },
            );
        quote! {
            #ident_update_ts
            #ident_update_try_new_er_ts
            #impl_pub_try_new_for_ident_update_ts
            #impl_serde_deserialize_for_ident_update_ts
            #impl_pg_crud_default_opt_some_vec_one_el_for_ident_update_ts
        }
    };
    let ident_update_for_query_ts = {
        let ident_update_for_query_ts = {
            let ts_50ae0c5f = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_serde_serialize()
                .derive_utoipa_to_schema()
                .build_struct(&ident_update_for_query_ucc, &Ts2::new(), &{
                    let fields_named_without_pk_ts =
                        gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| -> Ts2 {
                            let fi = &el.ident;
                            let opt_v_ft_as_pg_type_update_for_query_ts = {
                                let path_v_ts = format!("{PgCrudSc}::{VUcc}")
                                    .parse::<Ts2>()
                                    .expect("2b09d4ae");
                                let syn_type_as_pg_type_update_for_query_ts =
                                    gen_as_pg_type_update_for_query_ts(&el.type0);
                                gen_opt_type_decl_ts(
                                    &quote! {#path_v_ts<#syn_type_as_pg_type_update_for_query_ts>},
                                )
                            };
                            quote! {#fi: #opt_v_ft_as_pg_type_update_for_query_ts}
                        });
                    quote! {{
                        #pk_fi: #pk_ft_update_for_query_ts,
                        #fields_named_without_pk_ts
                    }}
                });
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_50ae0c5f
            }
        };
        let impl_ident_update_for_query_ts = {
            let update_query_part_pk_ts = {
                let ts = gen_match_ok_err_ts(
                    &quote! {#pk_ft_as_pg_type_ts #UpdateQueryPartSc(
                        &self.#pk_fi,
                        "",
                        #ident::#PkSc(),
                        "",
                        #IncrSc,
                    )},
                    &VSc,
                    &quote! {Ok(#VSc)},
                    &Er0,
                    &quote! {Err(#Er0)},
                );
                quote! {
                    fn #UpdateQueryPartPkSc(&self, #IncrSc: &mut u64) -> Result<#StringTs, #import_ts #QueryPartErUcc> {
                        #ts
                    }
                }
            };
            let update_query_part_fields_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    let update_query_part_fi_sc = UpdateQueryPartSelfSc::from_tokens(&fi);
                    let ft_as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                    let ts = gen_match_ok_err_ts(
                        &{
                            let fi_dq_ts = dq_ts(&fi);
                            quote! {#ft_as_pg_crud_pg_type_pg_type_ts #UpdateQueryPartSc(
                                &#VSc.#VSc,
                                #fi_dq_ts,
                                #fi_dq_ts,
                                "",
                                #IncrSc
                            )}
                        },
                        &quote! {v_f75dfd93},
                        &quote! {Ok(v_f75dfd93)},
                        &Er0,
                        &quote! {Err(#Er0)},
                    );
                    quote! {
                        fn #update_query_part_fi_sc(
                            #VSc: &#import_ts V<#ft_as_pg_crud_pg_type_pg_type_ts #UpdateForQueryUcc>,
                            #IncrSc: &mut u64
                        ) -> Result<#StringTs, #import_ts #QueryPartErUcc> {
                            #ts
                        }
                    }
                },
            );
            let select_only_updated_ids_query_part_ts = {
                let pk_ts = {
                    let pk_fi_dq_ts = dq_ts(&pk_fi);
                    let ts = gen_match_ok_err_ts_c35d87fd(
                        &quote! {#pk_as_pg_type_ts::#SelectOnlyUpdatedIdsQueryPartSc(
                            &self.#pk_fi,
                            #pk_fi_dq_ts,
                            incr,
                        )},
                        &quote! {v},
                        &quote! {{
                            return Err(#Er0);
                        }},
                    );
                    quote! {acc.push_str(&#ts);}
                };
                let ts = fields_without_pk.iter().map(|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    gen_if_let_some_ts(
                        &quote!{v_90f79b11},
                        &quote!{&self.#fi},
                        &{
                            let ts = gen_match_ok_err_ts_c35d87fd(
                                &{
                                    let fi_dq_ts = dq_ts(&fi);
                                    let ft_as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                                    quote!{#ft_as_pg_crud_pg_type_pg_type_ts #SelectOnlyUpdatedIdsQueryPartSc(
                                        &v_90f79b11.#VSc,
                                        #fi_dq_ts,
                                        incr,
                                    )}
                                },
                                &quote!{v_47a6f597},
                                &quote!{{
                                    return Err(#Er0);
                                }}
                            );
                            quote!{acc.push_str(&#ts);}
                        }
                    )
                });
                let ts0 = gen_acc_string_pop_ok_acc_ts(
                    &quote! {acc},
                    &quote! {
                        #pk_ts
                        #(#ts)*
                    },
                );
                quote! {
                    fn #SelectOnlyUpdatedIdsQueryPartSc(&self, #IncrSc: &mut u64) -> Result<#StringTs, #import_ts QueryPartEr> {
                        #ts0
                    }
                }
            };
            let update_handle_ts = gen_from_handle_ts(&ident_update_ucc, &{
                let pk_ft_as_pg_type_update_for_query_ts =
                    gen_as_pg_type_update_for_query_ts(&pk_ft);
                let fields_named_without_pk_ts =
                    gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| -> Ts2 {
                        let fi = &el.ident;
                        let ts = gen_v_init_ts0(&{
                            let ft_as_pg_type_update_for_query_ts =
                                gen_as_pg_type_update_for_query_ts(&el.type0);
                            quote! {#ft_as_pg_type_update_for_query_ts::from(v_0e64c53a.#VSc)}
                        });
                        quote! {#fi: #VSc.#fi.map(|v_0e64c53a| #ts)}
                    });
                quote! {
                    Self {
                        #pk_fi: #pk_ft_as_pg_type_update_for_query_ts::from(#VSc.#pk_fi),
                        #fields_named_without_pk_ts
                    }
                }
            });
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                impl #ident_update_for_query_ucc {
                    #update_query_part_pk_ts
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
    let gen_match_update_query_part_pk_ts = |operation: &Operation, ts: &dyn ToTokens| {
        gen_match_ok_err_ts_c35d87fd(
            &quote! {#ts.#UpdateQueryPartPkSc(&mut #IncrSc)},
            &quote! {v_f269a3b2},
            &{
                let ts_75b4019b = gen_operation_er_init_eprintln_res_creation_ts(
                    operation,
                    &query_part_syn_vrt_wrapper,
                    Location::caller(),
                );
                quote! {{
                    #ts_75b4019b
                }}
            },
        )
    };
    let row_and_rollback_syn_vrt_wrapper = new_syn_vrt_wrapper(
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
        let gen_ts = |ts: &dyn ToTokens| quote! {#import_ts PgTypeWhereFilter::#ts};
        (gen_ts(&QueryPartSc), gen_ts(&QueryBindSc))
    };
    let vec_struct_opts_ident_ts = gen_vec_tokens_decl_ts(&ident_read_ucc);
    let not_unique_field_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &NotUniqueFieldUcc,
        Some(StatusCode::BadReq400),
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string_serde,
            &NotUniqueFieldSc,
            gen_simple_syn_punct(&[&ident_select_ucc.to_string()]),
        )],
    );
    let simple_syn_punct_serde_error = gen_simple_syn_punct(&["serde_json", "Error"]);
    let serde_json_to_string_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &SerdeJsonToStringUcc,
        None,
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string,
            &SerdeJsonToStringSc,
            simple_syn_punct_serde_error.clone(),
        )],
    );
    let simple_syn_punct_reqwest_error = gen_simple_syn_punct(&["reqwest", "Error"]);
    let failed_to_get_res_text_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &FailedToGetResTextUcc,
        Some(StatusCode::BadReq400),
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
    let deserialize_res_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &DeserializeResUcc,
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
                &ResTextSc,
                string_syn_punct,
            ),
            (
                macros_helpers_location_location_field_attr_eo_to_err_string,
                &SerdeSc,
                simple_syn_punct_serde_error.clone(),
            ),
        ],
    );
    let reqwest_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &ReqwestUcc,
        None,
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string,
            &ReqwestSc,
            simple_syn_punct_reqwest_error,
        )],
    );
    let check_body_size_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &CheckBodySizeUcc,
        Some(StatusCode::BadReq400),
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
    let serde_json_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &SerdeJsonUcc,
        Some(StatusCode::BadReq400),
        vec![(
            macros_helpers_location_location_field_attr_eo_to_err_string,
            &SerdeJsonSc,
            simple_syn_punct_serde_error,
        )],
    );
    let header_content_type_application_json_not_found_syn_vrt_wrapper = new_syn_vrt_wrapper(
        &HeaderContentTypeApplicationJsonNotFoundUcc,
        Some(StatusCode::BadReq400),
        Vec::<(
            LocationFieldAttr,
            &'static dyn Display,
            Punctuated<PathSegment, PathSep>,
        )>::default(),
    );
    let common_http_req_syn_vrts = {
        vec![
            serde_json_to_string_syn_vrt_wrapper.get_syn_vrt().clone(),
            failed_to_get_res_text_syn_vrt_wrapper.get_syn_vrt().clone(),
            deserialize_res_syn_vrt_wrapper.get_syn_vrt().clone(),
            reqwest_syn_vrt_wrapper.get_syn_vrt().clone(),
        ]
    };
    let gen_extra_er_vrts = |di_bde7efb1: &DeriveInput,
                             gen_pg_table_attr: GenPgTableAttr|
     -> Vec<Variant> {
        let gen_pg_table_attr_str = gen_pg_table_attr.to_string();
        let common_extra_er_vrts_attr_ts =
            get_macro_attr_meta_list_ts(&di_bde7efb1.attrs, &gen_pg_table_attr.gen_path_to_attr());
        let di_894e3269: DeriveInput =
            parse2((*common_extra_er_vrts_attr_ts).clone()).expect("1b80783d");
        assert!(di_894e3269.ident == gen_pg_table_attr_str, "8a66c852");
        let vrts = if let Data::Enum(data_enum) = di_894e3269.data {
            data_enum.variants
        } else {
            panic!("f3ddc78c");
        };
        vrts.into_iter().collect()
    };
    let common_extra_er_vrts = gen_extra_er_vrts(&di, GenPgTableAttr::CommonExtraErVrts);
    let common_route_syn_vrts = {
        let mut acc = vec![
            check_body_size_syn_vrt_wrapper.get_syn_vrt(),
            pg_syn_vrt_wrapper.get_syn_vrt(),
            serde_json_syn_vrt_wrapper.get_syn_vrt(),
            header_content_type_application_json_not_found_syn_vrt_wrapper.get_syn_vrt(),
        ];
        for el in &common_extra_er_vrts {
            acc.push(el);
        }
        acc
    };
    let gen_pub_handle_ts = |is_pub: bool| {
        if is_pub {
            quote! {pub}
        } else {
            Ts2::new()
        }
    };
    let gen_pub_handle_pk_fi_pk_inner_type_handle_ts = |ts: &dyn ToTokens| {
        let is_pub = true;
        let pub_handle_ts = gen_pub_handle_ts(is_pub);
        quote! {#pub_handle_ts #pk_fi: #ts}
    };
    let gen_match_pg_transaction_rollback_await_ts =
        |operation: &Operation, location: &'static Location<'_>| {
            let ts_91f19090 = gen_operation_er_init_eprintln_res_creation_ts(
                operation,
                &pg_syn_vrt_wrapper,
                location,
            );
            let row_and_rollback_syn_vrt_er_init_eprintln_res_creation_ts =
                gen_operation_er_init_eprintln_res_creation_ts(
                    operation,
                    &row_and_rollback_syn_vrt_wrapper,
                    location,
                );
            quote! {{
                if let Err(#Er1) = #ExecutorSc.#RollbackSc().await {
                    #row_and_rollback_syn_vrt_er_init_eprintln_res_creation_ts
                }
                #ts_91f19090
            }}
        };
    let gen_drop_rows_match_pg_transaction_rollback_await_handle_ts =
        |operation: &Operation, location: &'static Location<'_>| {
            let match_pg_transaction_rollback_await_ts =
                gen_match_pg_transaction_rollback_await_ts(operation, location);
            quote! {
                drop(#RowsSc);
                #match_pg_transaction_rollback_await_ts
            }
        };
    let wrap_into_v_ts = |ts: &dyn ToTokens| {
        quote! {
            let #VSc = {
                #ts
            };
        }
    };
    let gen_fetch_ts = |fetch_ts: &dyn ToTokens,
                        some_ts: &dyn ToTokens,
                        er_ts: &dyn ToTokens,
                        should_wrap_into_v: &ShouldWrapIntoV| {
        let ts = {
            let ts = gen_match_ok_err_ts(
                &quote! {#import_ts TryStreamExt::try_next(&mut #RowsSc).await},
                &quote! {v_19f3d6e1},
                &quote! {match v_19f3d6e1 {
                    Some(v_b27d7d79) => #some_ts,
                    None => None,
                }},
                &Er0,
                &quote! {{
                    #er_ts
                }},
            );
            quote! {
                let mut #RowsSc = #BindedQuerySc.fetch(#fetch_ts.as_mut());
                let mut acc_d16ac269 = Vec::new();
                while let Some(v_d9cc2c36) = #ts {
                    acc_d16ac269.push(v_d9cc2c36);
                }
                acc_d16ac269
            }
        };
        match should_wrap_into_v {
            ShouldWrapIntoV::False => ts,
            ShouldWrapIntoV::True => wrap_into_v_ts(&ts),
        }
    };
    let gen_fetch_one_ts = |fetch_ts: &dyn ToTokens, ok_ts: &dyn ToTokens, er_ts: &dyn ToTokens| {
        gen_match_ok_err_ts(
            &quote! {#BindedQuerySc.fetch_one(#fetch_ts.as_mut()).await},
            &quote! {v_b27d7d79},
            &quote! {{
                #ok_ts
            }},
            &Er0,
            &quote! {{
                #er_ts
            }},
        )
    };
    let gen_sqlx_row_try_get_pk_ts =
        |sqlx_row_try_get_type_ts: &dyn ToTokens, ok_ts: &dyn ToTokens, err_ts: &dyn ToTokens| {
            gen_match_ok_err_ts(
                &quote! {#SqlxRow::try_get::<
                    #sqlx_row_try_get_type_ts,
                    #RefStr
                >(&v_b27d7d79, Self::#PkSc())},
                &quote! {v_69ecb6a9},
                &ok_ts,
                &Er0,
                &quote! {{
                    #err_ts
                }},
            )
        };
    let wrap_into_pg_transaction_begin_commit_ts = |operation: &Operation, ts: &dyn ToTokens| {
        let pg_transaction_begin_ts = {
            let ts_efebc55b = gen_operation_er_init_eprintln_res_creation_ts(
                operation,
                &pg_syn_vrt_wrapper,
                Location::caller(),
            );
            let ts0 = gen_match_ok_err_ts_c35d87fd(
                &quote! {#SqlxAcquire::#BeginSc(#ExecutorAcquireSc).await},
                &quote! {v_1aaca28f},
                &quote! {{#ts_efebc55b}},
            );
            quote! {let mut #ExecutorSc = #ts0;}
        };
        let pg_transaction_commit_ts = {
            let pg_syn_vrt_er_init_eprintln_res_creation_ts =
                gen_operation_er_init_eprintln_res_creation_ts(
                    operation,
                    &pg_syn_vrt_wrapper,
                    Location::caller(),
                );
            quote! {
                if let Err(#Er0) = #ExecutorSc.#CommitSc().await {
                    #pg_syn_vrt_er_init_eprintln_res_creation_ts
                }
            }
        };
        quote! {
            #pg_transaction_begin_ts
            #ts
            #pg_transaction_commit_ts
            #VSc
        }
    };
    let gen_location_vrt_ts = |er_vrt: &Variant| -> Ts2 {
        let vrt_ident = &er_vrt.ident;
        let Fields::Named(fields_named) = &er_vrt.fields else {
            panic!("2acd4725");
        };
        let fields_mapped_into_ts = fields_named.named.iter().map(|field| {
            let fi = field.ident.as_ref().expect("a21dc807");
            let location_attr = if *fi == *LocSc.to_string() {
                Ts2::new()
            } else {
                let mut location_attr: Option<LocationFieldAttr> = None;
                for el in &field.attrs {
                    if el.path().segments.len() == 1 {
                        let segment = el.path().segments.first().expect("5bd7ed8d");
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
            let ft = &field.ty;
            quote! {
                #location_attr
                #fi: #ft
            }
        });
        quote! {
            #vrt_ident {
                #(#fields_mapped_into_ts),*
            }
        }
    };
    let gen_ident_try_operation_logic_res_vrts_ident_operation_er_convert_ts =
        |operation: &Operation,
         desirable_type_ts: &dyn ToTokens,
         type_vrts_from_req_res_syn_vrts: &Vec<Variant>|
         -> Ts2 {
            let ident_operation_res_vrts_ucc = gen_ident_operation_res_vrts_ucc(operation);
            let ident_try_operation_logic_res_vrts_ts = {
                let ts_c997a274 = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .build_enum(&ident_operation_res_vrts_ucc, &Ts2::new(), &{
                        let vrts_ts = type_vrts_from_req_res_syn_vrts
                            .iter()
                            .map(gen_serde_version_of_named_syn_vrt);
                        quote! {{
                            #DesirableUcc(#desirable_type_ts),
                            #(#vrts_ts),*
                        }}
                    });
                quote! {
                    #AllowClippyArbitrarySourceItemOrdering
                    #ts_c997a274
                }
            };
            let ident_operation_er_ucc = gen_ident_operation_er_ucc(operation);
            let impl_ident_operation_res_vrts_ts = {
                let from_handle_ts = gen_from_handle_ts(&ident_operation_er_ucc, &{
                    let vrts_ts = type_vrts_from_req_res_syn_vrts.iter().map(|el| {
                        let vrt_ident = &el.ident;
                        let Fields::Named(fields_named) = &el.fields else {
                            panic!("10764d2b");
                        };
                        let fields_mapped_into_ts = {
                            let fields_ts = fields_named.named.iter().map(|field| &field.ident);
                            quote! {#(#fields_ts),*}
                        };
                        let ident_operation_er_with_serde_ucc =
                            gen_ident_operation_er_with_serde_ucc(operation);
                        quote! {
                            #ident_operation_er_with_serde_ucc::#vrt_ident {
                                #fields_mapped_into_ts
                            } => Self::#vrt_ident {
                                #fields_mapped_into_ts
                            }
                        }
                    });
                    quote! {
                        match #VSc.#IntoSerdeVersionSc() {
                            #(#vrts_ts),*
                        }
                    }
                });
                quote! {
                    impl #ident_operation_res_vrts_ucc {
                        #from_handle_ts
                    }
                }
            };
            let ident_operation_er_ts = {
                let ts_685e0be8 = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_thiserror_error()
                    .derive_location_lib_location()
                    .build_enum(&ident_operation_er_ucc, &Ts2::new(), &{
                        let vrts_ts = type_vrts_from_req_res_syn_vrts
                            .iter()
                            .map(gen_location_vrt_ts);
                        quote! {{#(#vrts_ts),*}}
                    });
                quote! {
                    #AllowClippyArbitrarySourceItemOrdering
                    #ts_685e0be8
                }
            };
            quote! {
                #ident_try_operation_logic_res_vrts_ts
                #impl_ident_operation_res_vrts_ts
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
    let gen_ident_operation_params_ucc = |operation: &Operation| {
        format!("{ident}{operation}Params")
            .parse::<Ts2>()
            .expect("c1203fc6")
    };
    let gen_params_pattern_ts = |operation: &Operation, payload_ts: Ts2| -> Ts2 {
        let params_ts = {
            let (derive_clone, derive_copy) = operation.derive_clone_and_copy();
            let ts_0d032fce = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_clone_if(derive_clone)
                .derive_copy_if(derive_copy)
                .build_struct(&gen_ident_operation_params_ucc(operation), &Ts2::new(), &{
                    let ident_operation_payload_ucc = gen_ident_operation_payload_ucc(operation);
                    quote! {{
                        pub #PayloadSc: #ident_operation_payload_ucc,
                    }}
                });
            quote! {
                #AllowClippyArbitrarySourceItemOrdering
                #ts_0d032fce
            }
        };
        quote! {
            #payload_ts
            #params_ts
        }
    };
    let gen_params_payload_and_default_ts =
        |operation: &Operation, decl_ts: &dyn ToTokens, default_init_ts: &dyn ToTokens| {
            let ident_operation_payload_ucc = gen_ident_operation_payload_ucc(operation);
            let ident_operation_payload_ts = {
                let (derive_clone, derive_copy) = operation.derive_clone_and_copy();
                let ts_ec5b096c = StructOrEnumDeriveTsStreamBuilder::new()
                    .make_pub()
                    .derive_debug()
                    .derive_clone_if(derive_clone)
                    .derive_copy_if(derive_copy)
                    .derive_serde_serialize()
                    .derive_serde_deserialize()
                    .derive_utoipa_to_schema()
                    .build_struct(&ident_operation_payload_ucc, &Ts2::new(), &decl_ts);
                quote! {
                    #AllowClippyArbitrarySourceItemOrdering
                    #ts_ec5b096c
                }
            };
            let impl_pg_crud_default_opt_some_vec_one_el_for_operation_payload_ts =
                gen_impl_pg_crud_default_opt_some_vec_one_el_for_tokens_no_lifetime_ts(
                    &ident_operation_payload_ucc,
                    &quote! {Self #default_init_ts},
                );
            quote! {
                #ident_operation_payload_ts
                #impl_pg_crud_default_opt_some_vec_one_el_for_operation_payload_ts
            }
        };
    let gen_type_vrts_from_req_res_syn_vrts =
        |syn_vrts: &Vec<&Variant>, operation: &Operation| -> Vec<Variant> {
            let mut type_vrts_from_req_res_syn_vrts = Vec::new();
            for el in syn_vrts {
                type_vrts_from_req_res_syn_vrts.push((*el).clone());
            }
            for el in gen_extra_er_vrts(&di, operation.gen_pg_table_attr_extra_er_vrts()) {
                type_vrts_from_req_res_syn_vrts.push(el.clone());
            }
            type_vrts_from_req_res_syn_vrts
        };
    let gen_ident_try_operation_er_ts = |operation: &Operation, syn_vrts: &Vec<Variant>| -> Ts2 {
        let ts_930e1a93 = StructOrEnumDeriveTsStreamBuilder::new()
            .make_pub()
            .derive_debug()
            .derive_thiserror_error()
            .derive_location_lib_location()
            .build_enum(&gen_ident_try_operation_er_ucc(operation), &Ts2::new(), &{
                let vrts = syn_vrts
                    .iter()
                    .cloned()
                    .chain(once({
                        let ident_operation_er_with_serde_ucc =
                            gen_ident_operation_er_with_serde_ucc(operation);
                        new_syn_vrt_wrapper(
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
                        .get_syn_vrt()
                        .clone()
                    }))
                    .collect::<Vec<Variant>>();
                let vrts_ts = vrts.iter().map(gen_location_vrt_ts);
                quote! {{#(#vrts_ts),*}}
            });
        quote! {
            #AllowClippyArbitrarySourceItemOrdering
            #ts_930e1a93
        }
    };
    let std_sync_arc_combination_of_app_state_logic_traits_ts =
        quote! {std::sync::Arc<dyn #import_ts CombinationOfAppStateLogicTraits>};
    impl_ident_vec_ts.push({
        let ts = [
            Operation::CreateMany,
            Operation::CreateOne,
            Operation::ReadMany,
            Operation::ReadOne,
            Operation::UpdateMany,
            Operation::UpdateOne,
            Operation::DeleteMany,
            Operation::DeleteOne,
        ].iter().map(|operation: &Operation|{
            let operation_handle_sc_ts = operation.self_handle_sc_ts();
            let operation_sc_ts = operation.self_sc_ts();
            let req_parts_preparation_ts = {
                let ts0 =
                    &gen_operation_er_init_eprintln_res_creation_ts(
                        operation,
                        &header_content_type_application_json_not_found_syn_vrt_wrapper,
                        Location::caller(),
                    );
                let ts1 = gen_match_ok_err_ts_c35d87fd(
                    &quote!{#import_ts check_body_size::check_body_size(#BodySc, *#AppStateSc.get_maximum_size_of_http_body_in_bytes()).await},
                    &quote!{v_cfac9140},
                    &{
                        let ts = gen_operation_er_init_eprintln_res_creation_ts(
                            operation,
                            &check_body_size_syn_vrt_wrapper,
                            Location::caller(),
                        );
                        quote!{{#ts}}
                    }
                );
                quote! {
                    let (parts, #BodySc) = #ReqSc.into_parts();
                    let headers = parts.headers;
                    if !matches!(
                        headers.get(http::header::CONTENT_TYPE),
                        Some(v_e3f6eecd) if v_e3f6eecd == http::header::HeaderValue::from_static("application/json")
                    ) {
                        #ts0
                    }
                    //todo
                    // match axum::body::HttpBody::size_hint(&#BodySc).exact() {
                    //     Some(v) => {
                    //         println!(
                    //             "HttpBody::size_hint {v} byte or {} kilobyte or {} megabyte",
                    //             v
                    //                 .checked_div(1024)
                    //                 .expect("111fd01a"),
                    //             v
                    //                 .checked_div(1_048_576)
                    //                 .expect("efbe0db4"), //(1024*1024)
                    //         );
                    //     }
                    //     None => {
                    //         println!("HttpBody::size_hint is None");
                    //     }
                    // }
                    let body_bytes = #ts1;
                }
            };
            let extra_validators_ts = {
                let (
                    common_extra_logic_ts,
                    operation_extra_logic_ts
                ) = {
                    let gen_ts = |v: &String|get_macro_attr_meta_list_ts(
                        &di.attrs,
                        v
                    );
                    (
                        gen_ts(&GenPgTableAttr::CommonExtraLogic.gen_path_to_attr()),
                        gen_ts(&operation.gen_pg_table_attr_extra_logic().gen_path_to_attr())
                    )
                };
                quote! {
                    #common_extra_logic_ts
                    #operation_extra_logic_ts
                }
            };
            let params_logic_ts = {
                let params_logic_ts0 = {
                    let ident_operation_params_ucc = gen_ident_operation_params_ucc(operation);
                    //todo in case of large type there is a stackoverflow. for example it was a 3.5md json file gend by create_many_payload_example. 3400 fields = success. 16000 = stackoverflow
                    let ts = gen_match_ok_err_ts_c35d87fd(
                        &{
                            let ident_operation_payload_ucc = gen_ident_operation_payload_ucc(operation);
                            quote!{serde_json::from_slice::<#ident_operation_payload_ucc>(&#BodyBytesSc)}
                        },
                        &quote!{v_9e6fcd2d},
                        &{
                            let ts =
                                gen_operation_er_init_eprintln_res_creation_ts(
                                    operation,
                                    &serde_json_syn_vrt_wrapper,
                                    Location::caller(),
                                );
                            quote!{{#ts}}
                        }
                    );
                    quote! {
                        let #ParamsSc = #ident_operation_params_ucc {
                            #PayloadSc: #ts
                        };
                    }
                };
                match &operation {
                    Operation::CreateMany
                    | Operation::CreateOne
                    | Operation::ReadMany
                    | Operation::ReadOne
                    | Operation::DeleteMany
                    | Operation::DeleteOne => params_logic_ts0,
                    Operation::UpdateMany => quote! {
                        #params_logic_ts0
                        let #UpdateForQueryVecSc = #ParamsSc.#PayloadSc.0.into_iter()
                        .map(#ident_update_for_query_ucc::#FromHandleSc)
                        .collect::<Vec<#ident_update_for_query_ucc>>();
                    },
                    Operation::UpdateOne => quote! {
                        #params_logic_ts0
                        let #UpdateForQuerySc = #ident_update_for_query_ucc::#FromHandleSc(#ParamsSc.#PayloadSc);
                    },
                }
            };
            let gen_for_el_in_update_for_query_vec_ts = |ts: &dyn ToTokens|quote!{
                for el_a72f3eac in &#UpdateForQueryVecSc {
                    #ts
                }
            };
            let ts_fa8795ea = gen_operation_er_init_eprintln_res_creation_ts(
                operation,
                &query_part_syn_vrt_wrapper,
                Location::caller(),
            );
            let gen_match_ok_err_ts_85a5eace = |
                ts0: &dyn ToTokens,
                ts1: &dyn ToTokens,
            |gen_match_ok_err_ts_c35d87fd(
                &ts0,
                &ts1,
                &quote!{{#ts_fa8795ea}}
            );
            let gen_for_el_in_update_for_query_vec_ts_03fc0945 = |
                fi: &dyn ToTokens,
                ts0: &dyn ToTokens,
                ts1: &dyn ToTokens
            |gen_for_el_in_update_for_query_vec_ts(&gen_if_let_some_ts(
                &ts0,
                &quote!{&el_a72f3eac.#fi},
                &ts1
            ));
            let query_string_ts = {
                let gen_match_ok_err_ts_dd5366af = |
                    ts0: &dyn ToTokens,
                    ts1: &dyn ToTokens,
                    ts2: &dyn ToTokens,
                    ts3: &dyn ToTokens
                |gen_match_ok_err_ts(
                    &ts0,
                    &ts1,
                    &ts2,
                    &ts3,
                    &quote!{{#ts_fa8795ea}}
                );
                let write_into_buffer_query_part_syn_vrt_er_init_eprintln_res_creation_ts = {
                    let query_part_er_write_into_buffer_ts =
                        gen_query_part_er_write_into_buffer_ts(import);
                    quote! {
                        let #Er0 = #query_part_er_write_into_buffer_ts;
                        #ts_fa8795ea
                    }
                };
                let incr_init_ts = quote! {let mut #IncrSc: u64 = 0;};
                let column_names_dq_ts = dq_ts(&{
                    let mut acc = fields.iter().fold(String::default(), |mut acc0, el| {
                        assert!(write!(acc0, "{}", &el.ident).is_ok(), "b9fe50dc");
                        acc0.push(',');
                        acc0
                    });
                    let _: Option<char> = acc.pop();
                    acc
                });
                let select_only_ids_query_part_ts = {
                    let select_only_ids_query_part_init_ts = fields.iter().map(|el: &SynFieldWrapper| gen_match_ok_err_ts_dd5366af(
                        &{
                            let fi_dq_ts = dq_ts(&el.ident);
                            let ft_as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                            quote!{#ft_as_pg_crud_pg_type_pg_type_ts #SelectOnlyIdsQueryPartSc(#fi_dq_ts)}
                        },
                        &quote!{v_aa341baf},
                        &quote!{{
                            acc_a35168d8.push_str(&v_aa341baf);
                        }},
                        &Er0
                    ));
                    let ts0 = gen_acc_string_pop_acc_ts(
                        &quote!{acc_a35168d8},
                        &quote!{#(#select_only_ids_query_part_init_ts)*}
                    );
                    quote! {{#ts0}}
                };
                let gen_if_write_is_err_ts_f22b2dd2 = |ts: &dyn ToTokens|gen_if_write_is_err_ts(
                    &ts,
                    &write_into_buffer_query_part_syn_vrt_er_init_eprintln_res_creation_ts
                );
                let gen_select_only_updated_ids_query_part_ts = |ts: &dyn ToTokens|quote!{#ts.#SelectOnlyUpdatedIdsQueryPartSc(&mut #IncrSc)};
                match &operation {
                    Operation::CreateMany => {
                        let if_write_is_err_ts = gen_if_write_is_err_ts_f22b2dd2(
                            &quote! {
                                acc_8a58994e,
                                "({v_f4fdd10d}),"
                            }
                        );
                        let ts0 = gen_acc_string_pop_acc_ts(
                            &quote!{acc_8a58994e},
                            &{
                                let ts = gen_match_ok_err_ts_dd5366af(
                                    &quote!{el_1651705d.#CreateQueryPartSc(&mut #IncrSc)},
                                    &quote!{v_f4fdd10d},
                                    &quote!{{
                                        #if_write_is_err_ts
                                    }},
                                    &Er0,
                                );
                                quote!{
                                    for el_1651705d in &#ParamsSc.#PayloadSc.0 {
                                        #ts
                                    }
                                }
                            }
                        );
                        quote! {#import_ts gen_create_many_query_string(
                            #TableSc,
                            #column_names_dq_ts,
                            &{
                                #incr_init_ts
                                #ts0
                            },
                            &#select_only_ids_query_part_ts
                        )}
                    }
                    Operation::CreateOne => {
                        let ts = gen_match_ok_err_ts_85a5eace(
                            &quote!{#ParamsSc.#PayloadSc.#CreateQueryPartSc(&mut 0)},
                            &quote!{v_3267d57d},
                        );
                        quote! {
                            #import_ts gen_create_one_query_string(
                                #TableSc,
                                #column_names_dq_ts,
                                &#ts,
                                &#select_only_ids_query_part_ts
                            )
                        }
                    }
                    Operation::ReadMany => {
                        let select_query_part_params_payload_select_ts =
                            gen_select_query_part_params_payload_select_ts(operation);
                        let extra_params_init_ts = gen_read_or_delete_many_extra_params_init_ts(
                            &ReadManyOrDeleteMany::ReadMany,
                        );
                        let extra_params_order_by_handle_ts =
                            dq_ts(&format!("{{}}{OrderSc} {BySc} {{}} {{}}"));
                        let order_by_column_match_ts =
                            gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| {
                                let fi_ucc = ToTokensToUccTs::case_or_panic(&el.ident);
                                let fi_dq_ts = dq_ts(&el.ident);
                                quote! {
                                    #ident_select_ucc::#fi_ucc(_) => #fi_dq_ts
                                }
                            });
                        let (
                            if_write_is_err_curly_braces_0_ts,
                            if_write_is_err_curly_braces_1_ts
                        ) = {
                            let gen_if_write_is_err_curly_braces_ts_f9cf9cf2 = |ts: &dyn ToTokens|gen_if_write_is_err_curly_braces_ts(
                                &ts,
                                &write_into_buffer_query_part_syn_vrt_er_init_eprintln_res_creation_ts
                            );
                            (
                                gen_if_write_is_err_curly_braces_ts_f9cf9cf2(
                                    &quote! {
                                        #ExtraParamsSc,
                                        #extra_params_order_by_handle_ts,
                                        #PrefixSc,
                                        &match &#ParamsSc.#PayloadSc.#OrderBySc.#ColumnSc {
                                            #order_by_column_match_ts
                                        },
                                        #ParamsSc.#PayloadSc.#OrderBySc.#OrderSc.as_ref().map_or_else(
                                            || #import_ts Order::default().to_sc_str(),
                                            #import_ts Order::to_sc_str
                                        )
                                    }
                                ),
                                gen_if_write_is_err_curly_braces_ts_f9cf9cf2(
                                    &{
                                        let ts = gen_match_ok_err_ts_85a5eace(
                                            &quote!{#pg_crud_pg_type_where_filter_query_part_ts(
                                                &#ParamsSc.#PayloadSc.pagination,
                                                &mut #IncrSc,
                                                &"",
                                                bool::default()
                                            )},
                                            &quote!{v_742be6cf},
                                        );
                                        quote! {
                                            #ExtraParamsSc,
                                            "{prefix}{}",
                                            #ts
                                        }
                                    }
                                )
                            )
                        };
                        quote! {#import_ts gen_read_many_query_string(
                            #TableSc,
                            &#select_query_part_params_payload_select_ts,
                            &{
                                #incr_init_ts
                                let mut #ExtraParamsSc = #extra_params_init_ts;
                                let #PrefixSc = if extra_params.is_empty() {""} else {" "};
                                #if_write_is_err_curly_braces_0_ts
                                #if_write_is_err_curly_braces_1_ts
                                #ExtraParamsSc
                            }
                        )}
                    }
                    Operation::ReadOne => {
                        let select_query_part_params_payload_select_ts =
                            gen_select_query_part_params_payload_select_ts(operation);
                        let ts = gen_match_ok_err_ts_85a5eace(
                            &quote!{#pg_crud_pg_type_where_filter_query_part_ts(
                                &#ParamsSc.#PayloadSc.#pk_fi,
                                &mut 0,
                                &Self::#PkSc(),
                                false
                            )},
                            &quote!{v_be9e7b7d},
                        );
                        quote! {#import_ts gen_read_one_query_string(
                            #TableSc,
                            &#select_query_part_params_payload_select_ts,
                            &#ts
                        )}
                    }
                    Operation::UpdateMany => {
                        let gen_match_update_query_part_pk_operation_ts =
                            |ts: &dyn ToTokens| gen_match_update_query_part_pk_ts(operation, &ts);
                        let ts0 = gen_acc_string_pop_acc_ts(
                            &quote!{acc_b86a253a},
                            &gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                                let fi = &el.ident;
                                let fi_dq_ts = dq_ts(&fi);
                                let is_fi_update_exists_sc = IsSelfUpdateExistSc::from_tokens(&fi);
                                let update_query_part_fi_sc = UpdateQueryPartSelfSc::from_tokens(&fi);
                                let ts_ee27d6ff = gen_for_el_in_update_for_query_vec_ts(&quote!{
                                    if el_a72f3eac.#fi.is_some() {
                                        #is_fi_update_exists_sc = true;
                                        break;
                                    }
                                });
                                let ts_33401696 = gen_for_el_in_update_for_query_vec_ts_03fc0945(
                                    &fi,
                                    &quote!{v_3ea04126},
                                    &{
                                        let ts0 = gen_match_ok_err_ts_85a5eace(
                                            &quote!{el_a72f3eac.#UpdateQueryPartPkSc(&mut #IncrSc)},
                                            &quote!{v_00890100},
                                        );
                                        let ts1 = gen_match_ok_err_ts_85a5eace(
                                            &quote!{#ident_update_for_query_ucc::#update_query_part_fi_sc(v_3ea04126, &mut #IncrSc)},
                                            &quote!{v_8797585c},
                                        );
                                        quote!{
                                            acc_8ad06c8c.push_str(&#import_ts #GenWhenColumnIdThenVUpdateManyQueryPartSc(
                                                Self::#PkSc(),
                                                &#ts0,
                                                &#ts1
                                            ));
                                        }
                                    }
                                );
                                quote! {
                                    {
                                        let mut #is_fi_update_exists_sc = false;
                                        #ts_ee27d6ff
                                        if #is_fi_update_exists_sc {
                                            acc_b86a253a.push_str(&
                                                #import_ts gen_column_equals_case_acc_else_column_end_comma_update_many_query_part(
                                                    #fi_dq_ts,
                                                    &{
                                                        let mut acc_8ad06c8c = #StringTs::default();
                                                        #ts_33401696
                                                        acc_8ad06c8c
                                                    }
                                                )
                                            );
                                        }
                                    }
                                }
                            })
                        );
                        let ts1 = gen_acc_string_pop_acc_ts(
                            &quote!{acc_a95eb175},
                            &gen_for_el_in_update_for_query_vec_ts(&gen_if_write_is_err_ts_f22b2dd2(
                                &{
                                    let match_update_query_part_pk_operation_ts =
                                        gen_match_update_query_part_pk_operation_ts(&quote! {el_a72f3eac});
                                    quote! {
                                        acc_a95eb175,
                                        "{},",
                                        #match_update_query_part_pk_operation_ts
                                    }
                                }
                            ))
                        );
                        let ts_5abb9ece = gen_for_el_in_update_for_query_vec_ts(&gen_match_ok_err_ts_dd5366af(
                            &gen_select_only_updated_ids_query_part_ts(&quote!{el_a72f3eac}),
                            &quote!{v_4f536654},
                            &quote!{{
                                acc_fd44b0aa.push_str(&v_4f536654);
                            }},
                            &Er0,
                        ));
                        quote! {
                            {
                                #incr_init_ts
                                let els = {
                                    #ts0
                                };
                                let pks = {
                                    #ts1
                                };
                                let return_columns = {
                                    let mut acc_fd44b0aa = String::new();
                                    #ts_5abb9ece
                                    acc_fd44b0aa
                                };
                                #import_ts gen_update_many_query_string(
                                    #TableSc,
                                    &els,
                                    Self::#PkSc(),
                                    &pks,
                                    &return_columns
                                )
                            }
                        }
                    }
                    Operation::UpdateOne => {
                        let extra_params_modification_ts = gen_fields_named_without_pk_without_comma_ts(
                            &|el: &SynFieldWrapper| {
                                let fi = &el.ident;
                                let fi_dq_ts = dq_ts(&fi);
                                let gen_column_queals_v_comma_update_one_query_part_sc =
                                    GenColumnQuealsVCommaUpdateOneQueryPartSc;
                                let update_query_part_fi_sc = UpdateQueryPartSelfSc::from_tokens(&fi);
                                gen_if_let_some_ts(
                                    &quote!{v_2d144436},
                                    &quote!{&#UpdateForQuerySc.#fi},
                                    &{
                                        let ts = gen_match_ok_err_ts_85a5eace(
                                            &quote!{#ident_update_for_query_ucc::#update_query_part_fi_sc(v_2d144436, &mut #IncrSc)},
                                            &quote!{v_1ec12051},
                                        );
                                        quote!{
                                            acc_683e37b8.push_str(&#import_ts #gen_column_queals_v_comma_update_one_query_part_sc(
                                                #fi_dq_ts,
                                                &#ts
                                            ));
                                        }
                                    }
                                )
                            },
                        );
                        let extra_params_pk_modification_ts =
                            gen_match_update_query_part_pk_ts(operation, &quote! {#UpdateForQuerySc});
                        let ts_a6ae3308 = gen_acc_string_pop_acc_ts(
                            &quote!{acc_683e37b8},
                            &extra_params_modification_ts
                        );
                        let ts = gen_match_ok_err_ts_85a5eace(
                            &gen_select_only_updated_ids_query_part_ts(&UpdateForQuerySc),
                            &quote!{v_7f0d86a1},
                        );
                        quote! {
                            {
                                #incr_init_ts
                                let #ColumnsSc = {
                                    #ts_a6ae3308
                                };
                                let #PkQueryPartSc = #extra_params_pk_modification_ts;
                                let return_columns = #ts;
                                #import_ts gen_update_one_query_string(
                                    #TableSc,
                                    &#ColumnsSc,
                                    Self::#PkSc(),
                                    &#PkQueryPartSc,
                                    &return_columns
                                )
                            }
                        }
                    }
                    Operation::DeleteMany => {
                        let extra_params_init_ts = gen_read_or_delete_many_extra_params_init_ts(
                            &ReadManyOrDeleteMany::DeleteMany,
                        );
                        quote! {#import_ts gen_delete_many_query_string(
                            #TableSc,
                            &{
                                #incr_init_ts
                                #extra_params_init_ts
                            },
                            Self::#PkSc(),
                        )}
                    }
                    Operation::DeleteOne => quote! {#import_ts gen_delete_one_query_string(
                        #TableSc,
                        Self::#PkSc(),
                    )},
                }
            };
            let binded_query_ts = {
                let ts_2795ebdc = gen_operation_er_init_eprintln_res_creation_ts(
                    operation,
                    &try_bind_syn_vrt_wrapper,
                    Location::caller(),
                );
                let gen_match_query_bind_or_err_ts_519a3119 = |
                    ts0: &dyn ToTokens,
                    ts1: &dyn ToTokens,
                |gen_match_query_bind_or_err_ts(
                    &ts0,
                    &ts1,
                    &ts_2795ebdc
                );
                match &operation {
                    Operation::CreateMany => {
                        let ts = gen_match_query_bind_or_err_ts_519a3119(
                            &quote!{el_7f862135.#CreateQueryBindSc(#QuerySc)},
                            &quote!{v_011a3eb4}
                        );
                        quote! {
                            for el_7f862135 in #ParamsSc.#PayloadSc.0 {
                                #ts
                            }
                        }
                    }
                    Operation::CreateOne => gen_match_query_bind_or_err_ts_519a3119(
                        &quote!{#ParamsSc.#PayloadSc.#CreateQueryBindSc(#QuerySc)},
                        &quote!{v_06f852cd}
                    ),
                    Operation::ReadMany => {
                        let query_pg_type_where_filter_query_bind_params_payload_where_many_query_ts = gen_query_pg_type_where_filter_query_bind_params_payload_where_many_query_ts(operation);
                        let ts = gen_match_query_bind_or_err_ts_519a3119(
                            &quote!{#pg_crud_pg_type_where_filter_query_bind_ts(
                                #ParamsSc.#PayloadSc.pagination,
                                #QuerySc,
                            )},
                            &quote!{v_9f7e487b},
                        );
                        quote! {
                            #query_pg_type_where_filter_query_bind_params_payload_where_many_query_ts
                            #ts
                        }
                    }
                    Operation::ReadOne => gen_match_query_bind_or_err_ts_519a3119(
                        &quote!{#pg_crud_pg_type_where_filter_query_bind_ts(#ParamsSc.#PayloadSc.#pk_fi, #QuerySc)},
                        &quote!{v_80ee6983}
                    ),
                    Operation::UpdateMany => {
                        let fields_named_without_pk_update_assignment_ts =
                            gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                                gen_for_el_in_update_for_query_vec_ts_03fc0945(
                                    &el.ident,
                                    &quote!{v_2edaa480},
                                    &{
                                        let ts = gen_match_query_bind_or_err_ts_519a3119(
                                            &{
                                                let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                                                quote!{#as_pg_crud_pg_type_pg_type_ts #UpdateQueryBindSc(
                                                    v_2edaa480.#VSc.clone(),
                                                    #QuerySc,
                                                )}
                                            },
                                            &quote!{v_600e67dc},
                                        );
                                        quote!{
                                            if let Err(er_981062db) = #QuerySc.try_bind(el_a72f3eac.#pk_fi) {
                                                let #Er0 = er_981062db.to_string();
                                                #ts_2795ebdc
                                            }
                                            #ts
                                        }
                                    }
                                )
                            });
                        let pk_update_assignment_ts = gen_for_el_in_update_for_query_vec_ts(&gen_match_query_bind_or_err_ts_519a3119(
                            &quote!{#pk_ft_as_pg_type_ts #UpdateQueryBindSc(
                                el_a72f3eac.#pk_fi,
                                #QuerySc,
                            )},
                            &quote!{v_c40a4522},
                        ));
                        let binded_query_select_only_updated_ids_query_bind_ts =
                            gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                                gen_for_el_in_update_for_query_vec_ts_03fc0945(
                                    &el.ident,
                                    &quote!{v_47030ac2},
                                    &gen_match_query_bind_or_err_ts_519a3119(
                                        &{
                                            let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                                            quote!{#as_pg_crud_pg_type_pg_type_ts select_only_updated_ids_query_bind(
                                                &v_47030ac2.#VSc,
                                                #QuerySc
                                            )}
                                        },
                                        &quote!{v_c5b79b95},
                                    )
                                )
                            });
                        quote! {
                            #fields_named_without_pk_update_assignment_ts
                            #pk_update_assignment_ts
                            #binded_query_select_only_updated_ids_query_bind_ts
                        }
                    }
                    Operation::UpdateOne => {
                        let gen_binded_query_ts = |var_name: proc_macro2::TokenStream, method_name: proc_macro2::TokenStream| {
                            gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                                gen_if_let_some_ts(
                                    &var_name,
                                    &{
                                        let fi = &el.ident;
                                        quote!{&#UpdateForQuerySc.#fi}
                                    },
                                    &gen_match_query_bind_or_err_ts_519a3119(
                                        &{
                                            let as_pg_crud_pg_type_pg_type_ts = gen_as_pg_type_path_ts(&el.type0);
                                            quote!{#as_pg_crud_pg_type_pg_type_ts #method_name}
                                        },
                                        &quote!{v_result},
                                    )
                                )
                            })
                        };
                        let binded_query_modifications_ts = gen_binded_query_ts(
                            quote!{v_ed87c152},
                            quote!{#UpdateQueryBindSc(v_ed87c152.#VSc.clone(), #QuerySc)},
                        );
                        let binded_query_pk_modification_ts = gen_match_query_bind_or_err_ts_519a3119(
                            &quote!{#pk_ft_as_pg_type_ts #UpdateQueryBindSc(
                                #UpdateForQuerySc.#pk_fi,
                                #QuerySc,
                            )},
                            &quote!{v_d64bac39},
                        );
                        let binded_query_select_only_updated_ids_query_bind_ts = gen_binded_query_ts(
                            quote!{v_b2902425},
                            quote!{select_only_updated_ids_query_bind(&v_b2902425.#VSc, #QuerySc)},
                        );
                        quote! {
                            #binded_query_modifications_ts
                            #binded_query_pk_modification_ts
                            #binded_query_select_only_updated_ids_query_bind_ts
                        }
                    }
                    Operation::DeleteMany => {
                        gen_query_pg_type_where_filter_query_bind_params_payload_where_many_query_ts(
                            operation,
                        )
                    }
                    Operation::DeleteOne => gen_match_query_bind_or_err_ts_519a3119(
                        &quote!{#import_ts PgTypeWhereFilter::query_bind(
                            #ParamsSc.#PayloadSc.#pk_fi,
                            #QuerySc
                        )},
                        &quote!{v_3099ea0f}
                    )
                }
            };
            let acquire_pool_and_connection_ts = {
                let pg_syn_vrt_wrapper_er_init_eprintln_res_creation_ts =
                    gen_operation_er_init_eprintln_res_creation_ts(
                        operation,
                        &pg_syn_vrt_wrapper,
                        Location::caller(),
                    );
                let ts = gen_match_ok_err_ts_c35d87fd(
                    &quote!{#AppStateSc.get_pg_pool().acquire().await},
                    &quote!{v_4535ee48},
                    &quote!{{
                        #pg_syn_vrt_wrapper_er_init_eprintln_res_creation_ts
                    }}
                );
                let ts0 = gen_match_ok_err_ts_c35d87fd(
                    &quote!{sqlx::Acquire::acquire(&mut #PoolConnectionSc).await},
                    &quote!{v_61ae8f84},
                    &quote!{{
                        #pg_syn_vrt_wrapper_er_init_eprintln_res_creation_ts
                    }}
                );
                quote! {
                    let mut #PoolConnectionSc = #ts;
                    let #ExecutorAcquireSc = #ts0;
                }
            };
            let wraped_into_axum_res_ts = wrap_into_axum_res_ts(
                &{
                    let ident_operation_res_vrts_ucc = gen_ident_operation_res_vrts_ucc(operation);
                    quote! {#ident_operation_res_vrts_ucc::#DesirableUcc(#VSc)}
                },
                &operation.desirable_status_code().to_http_status_code_ts(),
                &ShouldAddReturn::False,
            );
            let pg_logic_ts = {
                let gen_match_ident_read_only_ids_as_from_row_from_row_ts = |ts: &dyn ToTokens| gen_match_ok_err_ts_c35d87fd(
                    &quote!{<#ident_read_only_ids_ucc as sqlx::FromRow<'_, sqlx::postgres::PgRow>>::from_row(&v_b27d7d79)},
                    &quote!{v_33759463},
                    &ts
                );
                let gen_create_update_delete_many_fetch_ts =
                    |create_or_update_or_delete_many: &CreateOrUpdateOrDeleteMany| {
                        let operation_d1960edc = Operation::from(create_or_update_or_delete_many);
                        gen_fetch_ts(
                            &ExecutorSc,
                            &match &create_or_update_or_delete_many {
                                CreateOrUpdateOrDeleteMany::Create
                                | CreateOrUpdateOrDeleteMany::Update => {
                                    let ts = gen_match_ident_read_only_ids_as_from_row_from_row_ts(&{
                                        let ts =
                                            gen_drop_rows_match_pg_transaction_rollback_await_handle_ts(
                                                &operation_d1960edc,
                                                Location::caller(),
                                            );
                                        quote! {{#ts}}
                                    });
                                    quote! {Some(#ts)}
                                }
                                CreateOrUpdateOrDeleteMany::Delete => gen_sqlx_row_try_get_pk_ts(
                                    &pk_ft_as_pg_type_read_ucc,
                                    &quote! {Some(v_69ecb6a9)},
                                    &gen_drop_rows_match_pg_transaction_rollback_await_handle_ts(
                                        &operation_d1960edc,
                                        Location::caller(),
                                    ),
                                ),
                            },
                            &gen_drop_rows_match_pg_transaction_rollback_await_handle_ts(
                                &operation_d1960edc,
                                Location::caller(),
                            ),
                            &ShouldWrapIntoV::True,
                        )
                    };
                let gen_create_update_delete_one_fetch_ts =
                    |create_or_update_or_delete_one: &CreateOrUpdateOrDeleteOne| {
                        wrap_into_v_ts(&{
                            let operation0 = Operation::from(create_or_update_or_delete_one);
                            let ts = gen_match_pg_transaction_rollback_await_ts(
                                &operation0,
                                Location::caller(),
                            );
                            gen_fetch_one_ts(
                                &ExecutorSc,
                                &match create_or_update_or_delete_one {
                                    CreateOrUpdateOrDeleteOne::Create => {
                                        gen_match_ident_read_only_ids_as_from_row_from_row_ts(
                                            &quote! {{#ts}},
                                        )
                                    }
                                    CreateOrUpdateOrDeleteOne::Update => {
                                        gen_match_ident_read_only_ids_as_from_row_from_row_ts(&ts)
                                    }
                                    CreateOrUpdateOrDeleteOne::Delete => gen_sqlx_row_try_get_pk_ts(
                                        &quote! {#pk_ft_as_pg_type_read_ucc},
                                        &quote! {v_69ecb6a9},
                                        &ts,
                                    ),
                                },
                                &ts,
                            )
                        })
                    };
                match &operation {
                    Operation::CreateMany => wrap_into_pg_transaction_begin_commit_ts(
                        operation,
                        &gen_create_update_delete_many_fetch_ts(&CreateOrUpdateOrDeleteMany::Create),
                    ),
                    Operation::CreateOne => wrap_into_pg_transaction_begin_commit_ts(
                        operation,
                        &gen_create_update_delete_one_fetch_ts(&CreateOrUpdateOrDeleteOne::Create),
                    ),
                    Operation::ReadMany => {
                        let fetch_ts = gen_fetch_ts(
                            &ExecutorAcquireSc,
                            &{
                                let match_ident_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts = gen_match_ident_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts(&ReadManyOrReadOne::ReadMany);
                                quote! {Some(#match_ident_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts)}
                            },
                            &gen_operation_er_init_eprintln_res_creation_ts(
                                operation,
                                &pg_syn_vrt_wrapper,
                                Location::caller(),
                            ),
                            &ShouldWrapIntoV::False,
                        );
                        quote! {{
                            #fetch_ts
                        }}
                    },
                    Operation::ReadOne => gen_fetch_one_ts(
                        &ExecutorAcquireSc,
                        &gen_match_ident_read_try_from_sqlx_pg_pg_row_with_not_empty_unique_vec_ident_select_ts(&ReadManyOrReadOne::ReadOne),
                        &gen_operation_er_init_eprintln_res_creation_ts(operation, &pg_syn_vrt_wrapper, Location::caller()),
                    ),
                    Operation::UpdateMany => wrap_into_pg_transaction_begin_commit_ts(
                        operation,
                        &gen_create_update_delete_many_fetch_ts(&CreateOrUpdateOrDeleteMany::Update),
                    ),
                    Operation::UpdateOne => wrap_into_pg_transaction_begin_commit_ts(
                        operation,
                        &gen_create_update_delete_one_fetch_ts(&CreateOrUpdateOrDeleteOne::Update),
                    ),
                    Operation::DeleteMany => wrap_into_pg_transaction_begin_commit_ts(
                        operation,
                        &gen_create_update_delete_many_fetch_ts(&CreateOrUpdateOrDeleteMany::Delete),
                    ),
                    Operation::DeleteOne => wrap_into_pg_transaction_begin_commit_ts(
                        operation,
                        &gen_create_update_delete_one_fetch_ts(&CreateOrUpdateOrDeleteOne::Delete),
                    ),
                }
            };
            let operation_payload_example_ts = {
                let operation_payload_example_sc = operation.operation_payload_example_sc();
                let ts = wrap_into_axum_res_ts(
                    &{
                        let ident_operation_payload_ucc = gen_ident_operation_payload_ucc(operation);
                        quote! {<#ident_operation_payload_ucc as #import_ts #DefaultOptSomeVecOneElUcc>::#DefaultOptSomeVecOneElSc()}
                    },
                    &quote! {http::StatusCode::OK},
                    &ShouldAddReturn::False,
                );
                quote! {
                    #MustUse
                    pub fn #operation_payload_example_sc() -> axum::response::Response {
                        #ts
                    }
                }
            };
            // let type_vrts_from_req_res_syn_vrts = gen_type_vrts_from_req_res_syn_vrts(
            //     &{
            //         let mut acc = common_route_syn_vrts.clone();
            //         if let Operation::ReadMany | Operation::ReadOne = &operation {
            //             acc.push(not_unique_field_syn_vrt_wrapper.get_syn_vrt());
            //         }
            //         if let Operation::CreateMany | Operation::ReadMany | Operation::ReadOne | Operation::CreateOne | Operation::UpdateMany | Operation::UpdateOne | Operation::DeleteMany = &operation {
            //             acc.push(query_part_syn_vrt_wrapper.get_syn_vrt());
            //         }
            //         if let Operation::CreateMany | Operation::DeleteOne | Operation::CreateOne | Operation::UpdateMany | Operation::UpdateOne | Operation::DeleteMany = &operation {
            //             acc.push(row_and_rollback_syn_vrt_wrapper.get_syn_vrt());
            //         }
            //         acc.push(try_bind_syn_vrt_wrapper.get_syn_vrt());
            //         acc
            //     },
            //     operation
            // );
            operation_routes_ts.push({
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
                let operation_payload_example_sc =
                    operation.operation_payload_example_sc();
                let (
                    slash_operation_dq_ts,
                    slash_operation_payload_example_dq_ts
                ) = {
                    let gen_ts = |
                        v: &dyn Display
                    | dq_ts(&format!("/{v}"));
                    (
                        gen_ts(&operation.self_sc_str()),
                        gen_ts(&operation_payload_example_sc)
                    )
                };
                quote!{
                    .route(#slash_operation_dq_ts, axum::routing::#method_ts({
                        let table_owned = table.to_owned();
                        async move |
                            app_state_99328dfe: axum::extract::State<std::sync::Arc<dyn #import_ts CombinationOfAppStateLogicTraits>>,
                            req: axum::extract::Request
                        | Self::#operation_handle_sc_ts(app_state_99328dfe, req, &table_owned).await
                    }))
                    .route(#slash_operation_payload_example_dq_ts, axum::routing::get(async move||Self::#operation_payload_example_sc()))
                }
            });
            quote! {
                #[allow(clippy::single_call_fn)]
                async fn #operation_handle_sc_ts(
                    #AppStateSc: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_ts>,
                    #ReqSc: axum::extract::Request,
                    #TableSc: &str,
                ) -> axum::response::Response {
                    #req_parts_preparation_ts
                    #extra_validators_ts
                    #params_logic_ts
                    let #QueryStringSc = #query_string_ts;
                    //println!("{}", #QueryStringSc);
                    let #BindedQuerySc = {
                        let mut #QuerySc = #sqlx_query_sqlx_pg_ts(&#QueryStringSc);
                        #binded_query_ts
                        #QuerySc
                    };
                    #acquire_pool_and_connection_ts
                    let #VSc = {
                        #pg_logic_ts
                    };
                    #wraped_into_axum_res_ts
                }
                #[allow(clippy::single_call_fn)]
                pub async fn #operation_sc_ts(
                    #AppStateSc: axum::extract::State<#std_sync_arc_combination_of_app_state_logic_traits_ts>,
                    #ReqSc: axum::extract::Request,
                ) -> axum::response::Response {
                    Self::#operation_handle_sc_ts(#AppStateSc, #ReqSc, #self_table_name_call_ts).await
                }
                #operation_payload_example_ts
            }
        });
        quote!{#(#ts)*}
    });
    let gen_try_operation_ts = |operation: &Operation,
                                type_vrts_from_req_res_syn_vrts: &[Variant],
                                result_ok_type_ts: &dyn ToTokens| {
        let try_operation_sc_ts = operation.try_self_sc_ts();
        let try_operation_handle_sc_ts = operation.try_self_handle_sc_ts();
        let ident_try_operation_er_ucc = gen_ident_try_operation_er_ucc(operation);
        let ident_operation_params_ucc = gen_ident_operation_params_ucc(operation);
        let payload_ts = {
            let ts = gen_match_ok_err_ts_c35d87fd(
                &quote! {serde_json::to_string(&#ParamsSc.#PayloadSc)},
                &quote! {v_1772a83e},
                &{
                    let ts = gen_init_ts(&serde_json_to_string_syn_vrt_wrapper, Location::caller());
                    quote! {{
                        return Err(#ident_try_operation_er_ucc::#ts);
                    }}
                },
            );
            quote! {
                let #PayloadSc = {
                    #ts
                };
            }
        };
        let url_ts = {
            let format_ts = dq_ts(&format!(
                "{{endpoint_location}}/{{table}}/{}",
                operation.self_sc_str()
            ));
            quote! {let #UrlSc = format!(#format_ts);}
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
        let res_ts = {
            let ts =
                gen_match_ok_err_ts_c35d87fd(&quote! {#FutureSc.await}, &quote! {v_180559e9}, &{
                    let ts = gen_init_ts(&reqwest_syn_vrt_wrapper, Location::caller());
                    quote! {{
                        return Err(#ident_try_operation_er_ucc::#ts);
                    }}
                });
            quote! {let #ResSc = #ts;}
        };
        let er_0_res_status_ts = quote! {
            let #Er0 = #ResSc.status();
        };
        let headers_ts = quote! {
            let #Er1 = #ResSc.headers().clone();
        };
        let res_text_ts = {
            let ts = gen_match_ok_err_ts(
                &quote! {#ResSc.text().await},
                &quote! {v_6a62b2b9},
                &quote! {v_6a62b2b9},
                &Er2,
                &{
                    let failed_to_get_res_text_syn_vrt_init_ts =
                        gen_init_ts(&failed_to_get_res_text_syn_vrt_wrapper, Location::caller());
                    quote! {{
                        return Err(#ident_try_operation_er_ucc::#failed_to_get_res_text_syn_vrt_init_ts);
                    }}
                },
            );
            quote! {let #Er2 = #ts;}
        };
        let ident_operation_res_vrts_ucc = gen_ident_operation_res_vrts_ucc(operation);
        let expected_res_ts = {
            let deserialize_res_syn_vrt_init_ts =
                gen_init_ts(&deserialize_res_syn_vrt_wrapper, Location::caller());
            let ts = gen_match_ok_err_ts(
                &quote! {serde_json::from_str::<#ident_operation_res_vrts_ucc>(&#Er2)},
                &quote! {v_563d2a75},
                &quote! {v_563d2a75},
                &Er3,
                &quote! {{
                    return Err(#ident_try_operation_er_ucc::#deserialize_res_syn_vrt_init_ts);
                }},
            );
            quote! {let #ExpectedResSc = #ts;}
        };
        let try_operation_logic_er_with_serde_ucc =
            gen_ident_operation_er_with_serde_ucc(operation);
        let operation_er_with_serde_sc = &operation.operation_er_with_serde_sc();
        let try_operation_logic_er_with_serde_ts = {
            let try_operation_logic_res_vrts_to_try_operation_logic_er_with_serde = type_vrts_from_req_res_syn_vrts.iter().map(|el| {
                    let vrt_ident = &el.ident;
                    let fields_idents_ts = if let Fields::Named(fields_named) = &el.fields {
                        let fields_idents = fields_named.named.iter().map(|field| &field.ident);
                        quote! {#(#fields_idents),*}
                    } else {
                        panic!("8dcafc1c");
                    };
                    quote! {
                        #ident_operation_res_vrts_ucc::#vrt_ident {
                            #fields_idents_ts
                        } => #try_operation_logic_er_with_serde_ucc::#vrt_ident { #fields_idents_ts }
                    }
                });
            quote! {
                let #operation_er_with_serde_sc = match #ExpectedResSc {
                    #ident_operation_res_vrts_ucc::#DesirableUcc(#VSc) => {
                        return Ok(#VSc);
                    },
                    #(#try_operation_logic_res_vrts_to_try_operation_logic_er_with_serde),*
                };
            }
        };
        let return_er_ts = {
            let ts_6ac7b78e = gen_field_loc_new_ts(file!(), line!(), column!());
            quote! {
                Err(#ident_try_operation_er_ucc::#try_operation_logic_er_with_serde_ucc {
                    #operation_er_with_serde_sc,
                    #ts_6ac7b78e,
                })
            }
        };
        quote! {
            #[allow(clippy::single_call_fn)]
            async fn #try_operation_handle_sc_ts(
                #EndpointLocationSc: #RefStr,
                #ParamsSc: #ident_operation_params_ucc,
                #TableSc: &str,
            ) -> Result<#result_ok_type_ts, #ident_try_operation_er_ucc> {
                #payload_ts
                #url_ts
                #future_ts
                #res_ts
                #er_0_res_status_ts
                #headers_ts
                #res_text_ts
                #expected_res_ts
                #try_operation_logic_er_with_serde_ts
                #return_er_ts
            }
            pub async fn #try_operation_sc_ts(
                #EndpointLocationSc: #RefStr,
                #ParamsSc: #ident_operation_params_ucc
            ) -> Result<#result_ok_type_ts, #ident_try_operation_er_ucc> {
                Self::#try_operation_handle_sc_ts(
                    #EndpointLocationSc,
                    #ParamsSc,
                    #self_table_name_call_ts
                ).await
            }
        }
    };
    let create_many_ts = {
        let operation = Operation::CreateMany;
        let type_vrts_from_req_res_syn_vrts = gen_type_vrts_from_req_res_syn_vrts(
            &common_route_syn_vrts
                .iter()
                .copied()
                .chain(once(query_part_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(row_and_rollback_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(try_bind_syn_vrt_wrapper.get_syn_vrt()))
                .collect(),
            &operation,
        );
        let params_ts = gen_params_pattern_ts(
            &operation,
            gen_params_payload_and_default_ts(
                &operation,
                &{
                    let vec_ident_create_ts = gen_vec_tokens_decl_ts(&ident_create_ucc);
                    quote! {(pub #vec_ident_create_ts);}
                },
                &quote! {(vec![#PgCrudDefaultOptSomeVecOneElCall])},
            ),
        );
        let operation_ts = gen_ident_try_operation_logic_res_vrts_ident_operation_er_convert_ts(
            &operation,
            &vec_ident_read_only_ids_ts,
            &type_vrts_from_req_res_syn_vrts,
        );
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_req_syn_vrts);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_vrts_from_req_res_syn_vrts,
                &vec_ident_read_only_ids_ts,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        quote! {
            #params_ts
            #operation_ts
            #try_operation_ts
        }
    };
    mb_write_ts_into_file(
        gen_pg_table_config.create_many_write_into_file,
        "gen_pg_table_create_many",
        &create_many_ts,
        &FormatWithCargofmt::True,
    );
    let create_one_ts = {
        let operation = Operation::CreateOne;
        let type_vrts_from_req_res_syn_vrts = gen_type_vrts_from_req_res_syn_vrts(
            &common_route_syn_vrts
                .iter()
                .copied()
                .chain(once(row_and_rollback_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(query_part_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(try_bind_syn_vrt_wrapper.get_syn_vrt()))
                .collect(),
            &operation,
        );
        let params_ts = gen_params_pattern_ts(&operation, Ts2::new());
        let operation_ts = gen_ident_try_operation_logic_res_vrts_ident_operation_er_convert_ts(
            &operation,
            &ident_read_only_ids_ucc,
            &type_vrts_from_req_res_syn_vrts,
        );
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_req_syn_vrts);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_vrts_from_req_res_syn_vrts,
                &ident_read_only_ids_ucc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        quote! {
            #params_ts
            #operation_ts
            #try_operation_ts
        }
    };
    mb_write_ts_into_file(
        gen_pg_table_config.create_one_write_into_file,
        "gen_pg_table_create_one",
        &create_one_ts,
        &FormatWithCargofmt::True,
    );
    let read_many_ts = {
        let operation = Operation::ReadMany;
        let type_vrts_from_req_res_syn_vrts = gen_type_vrts_from_req_res_syn_vrts(
            &common_route_syn_vrts
                .iter()
                .copied()
                .chain(once(not_unique_field_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(query_part_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(try_bind_syn_vrt_wrapper.get_syn_vrt()))
                .collect(),
            &operation,
        );
        let params_ts = gen_params_pattern_ts(
            &operation,
            gen_params_payload_and_default_ts(
                &operation,
                &quote! {{
                    #pub_where_many_opt_ident_where_many_ts,
                    #pub_select_pg_crud_not_empty_unique_vec_ident_select_ts,
                    pub #OrderBySc: #pg_crud_order_by_ts<#ident_select_ucc>,
                    pub #PaginationSc: #import_ts PaginationStartsWithZero,
                }},
                &{
                    let ts = gen_fi_default_opt_some_vec_one_el_call_ts(&PaginationSc);
                    quote! {{
                        #where_many_pg_crud_default_opt_some_vec_one_el_call_ts,
                        #select_pg_crud_default_opt_some_vec_one_el_call_ts,
                        #OrderBySc: #import_ts OrderBy {
                            #ColumnSc: #ident_select_ucc::#pk_fi_ucc_ts(
                                #PgCrudDefaultOptSomeVecOneElCall
                            ),
                            #OrderSc: Some(
                                #PgCrudDefaultOptSomeVecOneElCall
                            ),
                        },
                        #ts,
                    }}
                },
            ),
        );
        let operation_ts = gen_ident_try_operation_logic_res_vrts_ident_operation_er_convert_ts(
            &operation,
            &vec_struct_opts_ident_ts,
            &type_vrts_from_req_res_syn_vrts,
        );
        let try_operation_ts = {
            let try_operation_er_ts = gen_ident_try_operation_er_ts(&operation, &{
                let mut acc = common_http_req_syn_vrts.clone();
                acc.push(not_unique_field_syn_vrt_wrapper.get_syn_vrt().clone());
                acc
            });
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_vrts_from_req_res_syn_vrts,
                &vec_struct_opts_ident_ts,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        quote! {
            #params_ts
            #operation_ts
            #try_operation_ts
        }
    };
    mb_write_ts_into_file(
        gen_pg_table_config.read_many_write_into_file,
        "gen_pg_table_read_many",
        &read_many_ts,
        &FormatWithCargofmt::True,
    );
    let read_one_ts = {
        let operation = Operation::ReadOne;
        let type_vrts_from_req_res_syn_vrts = gen_type_vrts_from_req_res_syn_vrts(
            &common_route_syn_vrts
                .iter()
                .copied()
                .chain(once(not_unique_field_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(query_part_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(try_bind_syn_vrt_wrapper.get_syn_vrt()))
                .collect(),
            &operation,
        );
        let params_ts = gen_params_pattern_ts(
            &operation,
            gen_params_payload_and_default_ts(
                &operation,
                &{
                    let pub_handle_pk_fi_pk_inner_type_handle_ts =
                        gen_pub_handle_pk_fi_pk_inner_type_handle_ts(
                            &SelfReadUcc::from_type_last_segment(pk_ft),
                        );
                    quote! {{
                        #pub_handle_pk_fi_pk_inner_type_handle_ts,
                        #pub_select_pg_crud_not_empty_unique_vec_ident_select_ts,
                    }}
                },
                &{
                    let ts = gen_fi_default_opt_some_vec_one_el_call_ts(&pk_fi);
                    quote! {{
                        #ts,
                        #select_pg_crud_default_opt_some_vec_one_el_call_ts
                    }}
                },
            ),
        );
        let operation_ts = gen_ident_try_operation_logic_res_vrts_ident_operation_er_convert_ts(
            &operation,
            &ident_read_ucc,
            &type_vrts_from_req_res_syn_vrts,
        );
        let try_operation_ts = {
            let try_operation_er_ts = gen_ident_try_operation_er_ts(&operation, &{
                let mut acc = common_http_req_syn_vrts.clone();
                acc.push(not_unique_field_syn_vrt_wrapper.get_syn_vrt().clone());
                acc
            });
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_vrts_from_req_res_syn_vrts,
                &ident_read_ucc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        quote! {
            #params_ts
            #operation_ts
            #try_operation_ts
        }
    };
    mb_write_ts_into_file(
        gen_pg_table_config.read_one_write_into_file,
        "gen_pg_table_read_one",
        &read_one_ts,
        &FormatWithCargofmt::True,
    );
    //todo update not only with arr of objects with ids but with WHERE and one object
    let update_many_ts = {
        let operation = Operation::UpdateMany;
        let type_vrts_from_req_res_syn_vrts = gen_type_vrts_from_req_res_syn_vrts(
            &common_route_syn_vrts
                .iter()
                .copied()
                .chain(once(row_and_rollback_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(query_part_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(try_bind_syn_vrt_wrapper.get_syn_vrt()))
                .collect(),
            &operation,
        );
        let params_ts = gen_params_pattern_ts(&operation, {
            let ident_operation_payload_ucc = gen_ident_operation_payload_ucc(&operation);
            let vec_ident_update_ts = gen_vec_tokens_decl_ts(&ident_update_ucc);
            let ident_operation_payload_vec_ts = StructOrEnumDeriveTsStreamBuilder::new()
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
            let ident_operation_payload_try_new_er_ts = StructOrEnumDeriveTsStreamBuilder::new()
                .make_pub()
                .derive_debug()
                .derive_thiserror_error()
                .derive_location_lib_location()
                .build_enum(
                    &ident_operation_payload_try_new_er_ucc,
                    &Ts2::new(),
                    &quote! {{
                        #NotUniquePkUcc {
                            #[eo_to_err_string]
                            #NotUniquePkSc: #pk_ft_update_ts,
                            #[eo_to_err_string]
                            loc: location_lib::loc::Loc,
                        }
                    }},
                );
            let impl_pub_try_new_for_ident_operation_payload_ts = gen_impl_pub_try_new_for_ident_ts(
                &Ts2::new(),
                &gen_ident_operation_payload_ucc(&operation),
                &quote! {#VSc: #vec_ident_update_ts},
                &ident_operation_payload_try_new_er_ucc,
                &quote! {
                    let mut acc_6bf275fc = Vec::new();
                    for el_35facc3a in &#VSc {
                        if acc_6bf275fc.contains(&&el_35facc3a.#pk_fi) {
                            return Err(#ident_operation_payload_try_new_er_ucc::#NotUniquePkUcc {
                                #NotUniquePkSc: el_35facc3a.#pk_fi,
                                loc: location_lib::loc!(),
                            });
                        }
                        acc_6bf275fc.push(&el_35facc3a.#pk_fi);
                    }
                    Ok(Self(#VSc))
                },
            );
            let impl_serde_deserialize_for_ident_update_many_payload_ts = {
                let tuple_struct_ident_operation_payload_dq_ts =
                    dq_ts(&format!("tuple struct {ident_operation_payload_ucc}"));
                let tuple_struct_ident_operation_payload_with_1_el_dq_ts = dq_ts(&format!(
                    "tuple struct {ident_operation_payload_ucc} with 1 el"
                ));
                let match_ident_update_many_payload_try_new_field0_ts =
                    gen_match_try_new_in_deserialize_ts(&ident_operation_payload_ucc, &quote! {f0});
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
                                        let f0: #vec_ident_update_ts = <#vec_ident_update_ts as _serde::Deserialize>::deserialize(__e)?;
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
                                        let Some(f0) = _serde::de::SeqAccess::next_element::<#vec_ident_update_ts>(&mut __seq)? else {
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
            let impl_pg_crud_default_opt_some_vec_one_el_for_operation_payload_ts =
                gen_impl_pg_crud_default_opt_some_vec_one_el_for_tokens_no_lifetime_ts(
                    &ident_operation_payload_ucc,
                    &quote! {
                        Self(vec![#PgCrudDefaultOptSomeVecOneElCall])
                    },
                );
            quote! {
                #ident_operation_payload_vec_ts
                #ident_operation_payload_try_new_er_ts
                #impl_pub_try_new_for_ident_operation_payload_ts
                #impl_serde_deserialize_for_ident_update_many_payload_ts
                #impl_pg_crud_default_opt_some_vec_one_el_for_operation_payload_ts
            }
        });
        let operation_ts = gen_ident_try_operation_logic_res_vrts_ident_operation_er_convert_ts(
            &operation,
            &vec_ident_read_only_ids_ts,
            &type_vrts_from_req_res_syn_vrts,
        );
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_req_syn_vrts);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_vrts_from_req_res_syn_vrts,
                &vec_ident_read_only_ids_ts,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        quote! {
            #params_ts
            #operation_ts
            #try_operation_ts
        }
    };
    mb_write_ts_into_file(
        gen_pg_table_config.update_many_write_into_file,
        "gen_pg_table_update_many",
        &update_many_ts,
        &FormatWithCargofmt::True,
    );
    let update_one_ts = {
        let operation = Operation::UpdateOne;
        let type_vrts_from_req_res_syn_vrts = gen_type_vrts_from_req_res_syn_vrts(
            &common_route_syn_vrts
                .iter()
                .copied()
                .chain(once(row_and_rollback_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(query_part_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(try_bind_syn_vrt_wrapper.get_syn_vrt()))
                .collect(),
            &operation,
        );
        let params_ts = gen_params_pattern_ts(&operation, Ts2::new());
        let operation_ts = gen_ident_try_operation_logic_res_vrts_ident_operation_er_convert_ts(
            &operation,
            &ident_read_only_ids_ucc,
            &type_vrts_from_req_res_syn_vrts,
        );
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_req_syn_vrts);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_vrts_from_req_res_syn_vrts,
                &ident_read_only_ids_ucc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        quote! {
            #params_ts
            #operation_ts
            #try_operation_ts
        }
    };
    mb_write_ts_into_file(
        gen_pg_table_config.update_one_write_into_file,
        "gen_pg_table_update_one",
        &update_one_ts,
        &FormatWithCargofmt::True,
    );
    //todo return deleted rows ids vec
    let delete_many_ts = {
        let operation = Operation::DeleteMany;
        let type_vrts_from_req_res_syn_vrts = gen_type_vrts_from_req_res_syn_vrts(
            &common_route_syn_vrts
                .iter()
                .copied()
                .chain(once(row_and_rollback_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(query_part_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(try_bind_syn_vrt_wrapper.get_syn_vrt()))
                .collect(),
            &operation,
        );
        let params_ts = gen_params_pattern_ts(
            &operation,
            gen_params_payload_and_default_ts(
                &operation,
                &quote! {{#pub_where_many_opt_ident_where_many_ts}},
                &quote! {{#where_many_pg_crud_default_opt_some_vec_one_el_call_ts}},
            ),
        );
        let operation_ts = gen_ident_try_operation_logic_res_vrts_ident_operation_er_convert_ts(
            &operation,
            &vec_pk_ft_read_ts,
            &type_vrts_from_req_res_syn_vrts,
        );
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_req_syn_vrts);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_vrts_from_req_res_syn_vrts,
                &vec_pk_ft_read_ts,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        quote! {
            #params_ts
            #operation_ts
            #try_operation_ts
        }
    };
    mb_write_ts_into_file(
        gen_pg_table_config.delete_many_write_into_file,
        "gen_pg_table_delete_many",
        &delete_many_ts,
        &FormatWithCargofmt::True,
    );
    let delete_one_ts = {
        let operation = Operation::DeleteOne;
        let type_vrts_from_req_res_syn_vrts = gen_type_vrts_from_req_res_syn_vrts(
            &common_route_syn_vrts
                .iter()
                .copied()
                .chain(once(row_and_rollback_syn_vrt_wrapper.get_syn_vrt()))
                .chain(once(try_bind_syn_vrt_wrapper.get_syn_vrt()))
                .collect(),
            &operation,
        );
        let params_ts = gen_params_pattern_ts(
            &operation,
            gen_params_payload_and_default_ts(
                &operation,
                &{
                    let ts = gen_pub_handle_pk_fi_pk_inner_type_handle_ts(
                        &SelfReadUcc::from_type_last_segment(pk_ft),
                    );
                    quote! {{#ts}}
                },
                &{
                    let ts = gen_fi_default_opt_some_vec_one_el_call_ts(&pk_fi);
                    quote! {{#ts}}
                },
            ),
        );
        let operation_ts = gen_ident_try_operation_logic_res_vrts_ident_operation_er_convert_ts(
            &operation,
            &pk_ft_as_pg_type_read_ucc,
            &type_vrts_from_req_res_syn_vrts,
        );
        let try_operation_ts = {
            let try_operation_er_ts =
                gen_ident_try_operation_er_ts(&operation, &common_http_req_syn_vrts);
            impl_ident_vec_ts.push(gen_try_operation_ts(
                &operation,
                &type_vrts_from_req_res_syn_vrts,
                &pk_ft_as_pg_type_read_ucc,
            ));
            quote! {
                #try_operation_er_ts
            }
        };
        quote! {
            #params_ts
            #operation_ts
            #try_operation_ts
        }
    };
    mb_write_ts_into_file(
        gen_pg_table_config.delete_one_write_into_file,
        "gen_pg_table_delete_one",
        &delete_one_ts,
        &FormatWithCargofmt::True,
    );
    impl_ident_vec_ts.push({
        let routes_ts = quote!{
            pub fn #RoutesSc(#AppStateSc: #std_sync_arc_combination_of_app_state_logic_traits_ts) -> axum::Router {
                Self::#RoutesHandleSc(#AppStateSc, #self_table_name_call_ts)
            }
        };
        quote! {
            #routes_ts
        }
    });
    let (operator_or_ts, operator_and_ts) = {
        let operator_ts = quote! {#import_ts Operator::};
        (quote! {#operator_ts Or}, quote! {#operator_ts And})
    };
    let ident_tests_ts = {
        fn gen_assert_ts(ts0: &dyn ToTokens, ts1: &dyn ToTokens) -> Ts2 {
            quote! {assert!(#ts0,#ts1);}
        }
        fn gen_assert_eq_ts(ts0: &dyn ToTokens, ts1: &dyn ToTokens, ts2: &dyn ToTokens) -> Ts2 {
            quote! {assert_eq!(#ts0,#ts1,#ts2);}
        }
        let gen_pk_where_equal_ts = |ts0: &dyn ToTokens| {
            quote! {
                #pk_ft_as_pg_type_where_ts::Equal(
                    #import_ts PgTypeWhereEqual {
                        operator: #operator_or_ts,
                        #VSc: #ts0,
                    },
                )
            }
        };
        let gen_pk_where_equal_new_ts =
            |ts0: &dyn ToTokens| gen_pk_where_equal_ts(&quote! {#pk_ft_table_type_ts::new(#ts0)});
        let pk_where_equal_uuid_new_v_ts =
            gen_pk_where_equal_new_ts(&quote! {uuid::Uuid::new_v4()});
        let gen_pk_where_equal_into_inner_ts = |ts0: &dyn ToTokens| {
            gen_pk_where_equal_new_ts(&quote! {#pk_as_pg_type_ts::into_inner(#ts0)})
        };
        let ident_tests_sc = SelfTestsSc::from_display(&ident);
        let ident_dq_ts = dq_ts(&DisplayToScStr::case(&ident));
        let ident_create_many_params_ucc = gen_ident_operation_params_ucc(&Operation::CreateMany);
        let ident_read_many_params_ucc = gen_ident_operation_params_ucc(&Operation::ReadMany);
        let ident_create_many_payload_ucc = gen_ident_operation_payload_ucc(&Operation::CreateMany);
        let ident_read_many_payload_ucc = gen_ident_operation_payload_ucc(&Operation::ReadMany);
        let ident_create_one_params_ucc = gen_ident_operation_params_ucc(&Operation::CreateOne);
        let ident_read_one_params_ucc = gen_ident_operation_params_ucc(&Operation::ReadOne);
        let ident_read_one_payload_ucc = gen_ident_operation_payload_ucc(&Operation::ReadOne);
        let ident_update_one_params_ucc = gen_ident_operation_params_ucc(&Operation::UpdateOne);
        let config_path_ts = quote! {server_config::Config};
        let underscore_unused_ts = quote! {_unused};
        //todo mb remove it?\
        let gen_some_pg_type_where_try_new_ts = |operator_ts: &dyn ToTokens, ts: &dyn ToTokens| {
            quote! {
                Some(
                    #import_ts PgTypeWhere::try_new(
                        #operator_ts,
                        #ts
                    ).expect("6b0491b2"),
                )
            }
        };
        let gen_some_pg_type_where_try_new_and_ts =
            |ts: &dyn ToTokens| gen_some_pg_type_where_try_new_ts(&operator_and_ts, ts);
        let gen_pg_type_where_try_new_pk_ts = quote! {
            #import_ts PgTypeWhere::try_new(
                operator,
                vec
            ).expect("fd20ad6d")
        };
        let ident_create_default_fields_init_without_pk_ts =
            gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| {
                let fi = &el.ident;
                let ft_as_pg_type_create_ts = gen_as_pg_type_create_ts(&el.type0);
                quote! {
                    #fi: <#ft_as_pg_type_create_ts as #import_ts DefaultOptSomeVecOneEl>::default_opt_some_vec_one_el()
                }
            });
        let fields_none_init_ts =
            gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| {
                let fi = &el.ident;
                quote! {#fi: None}
            });
        //todo instead of first dropping table - check if its not exists. if exists Test must fail
        let select_default_all_with_max_page_size_not_empty_unique_vec_ts = {
            let ts = gen_fields_named_with_comma_ts(&|el: &SynFieldWrapper| {
                let fi = &el.ident;
                let ft = &el.type0;
                let fi_ucc = ToTokensToUccTs::case_or_panic(&fi);
                quote! {
                    #ident_select_ucc::#fi_ucc(
                        <<#ft as #import_ts PgType>::Select as #import_ts #DefaultOptSomeVecOneElMaxPageSizeUcc>::#DefaultOptSomeVecOneElMaxPageSizeSc()
                    )
                }
            });
            quote! {
                let select_default_all_with_max_page_size = #import_ts NotEmptyUniqueVec::try_new(vec![
                    #ts
                ]).expect("5e82ac66");
            }
        };
        let pk_ft_as_pg_type_pk_ts = quote! {<#pk_ft as #import_ts PgTypePk>::};
        let gen_pk_ft_as_pg_type_pk_method_call_ts =
            |method_ts: &dyn ToTokens, params_ts: &dyn ToTokens| {
                quote! {#pk_ft_as_pg_type_pk_ts #method_ts(#params_ts)}
            };
        let pk_ft_read_into_table_type_el_pk_fi_clone_ts =
            gen_pk_ft_as_pg_type_pk_method_call_ts(&ReadIntoTableTypeSc, &quote! {el_adcc8db3});
        let read_only_ids_el_937c5af3_pk_fi = quote! {read_only_ids_el_937c5af3.#pk_fi};
        let (
            pk_ft_read_only_ids_into_read_el_43ab7fb5_pk_fi_ts,
            pk_ft_read_only_ids_into_read_read_only_ids_from_try_create_one_pk_fi_ts,
            pk_ft_read_only_is_into_read_read_only_ids_el_pk_fi_ts_937c5af3,
            pk_ft_read_only_ids_into_read_read_only_ids_returned_from_create_one_pk_fi_ts,
        ) = {
            let gen_ts = |ts: &dyn ToTokens| {
                gen_pk_ft_as_pg_type_pk_method_call_ts(&ReadOnlyIdsIntoReadSc, &ts)
            };
            (
                gen_ts(&quote! {el_43ab7fb5.#pk_fi}),
                gen_ts(&quote! {read_only_ids_from_try_create_one.#pk_fi}),
                gen_ts(&read_only_ids_el_937c5af3_pk_fi),
                gen_ts(&quote! {read_only_ids_returned_from_create_one.#pk_fi}),
            )
        };
        let ts_ffb964de = {
            let ts = gen_pk_where_equal_into_inner_ts(
                &pk_ft_read_only_ids_into_read_el_43ab7fb5_pk_fi_ts,
            );
            quote! {.iter().map(|el_43ab7fb5| #ts).collect()}
        };
        let pk_ft_as_pg_type_update_as_pg_type_pk_read_only_ids_into_update_ts = {
            let method_call_ts = gen_pk_ft_as_pg_type_pk_method_call_ts(
                &ReadOnlyIdsIntoUpdateSc,
                &read_only_ids_el_937c5af3_pk_fi,
            );
            quote! {#pk_as_pg_type_ts::Update::from(#method_call_ts)}
        };
        let (
            fi_read_only_ids_merged_with_create_into_opt_v_read_read_only_ids_and_create_ts,
            fi_read_only_ids_merged_with_create_into_opt_v_read_read_only_ids_from_try_create_one_ident_create_ts,
            fi_read_only_ids_merged_with_create_into_opt_v_read_read_only_ids_returned_from_create_one_create_ts,
            fi_read_only_ids_merged_with_create_into_opt_v_read_read_only_ids_returned_from_create_one_clone_ident_create_clone_ts,
        ) = {
            enum ShouldAddDotClone {
                False,
                True,
            }
            let gen_ts = |read_only_ids_ts: &dyn ToTokens,
                          create_ts: &dyn ToTokens,
                          should_add_dot_clone: &ShouldAddDotClone| {
                gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    let mb_dot_clone_ts = match &should_add_dot_clone {
                        ShouldAddDotClone::False => Ts2::new(),
                        ShouldAddDotClone::True => quote! {.clone()},
                    };
                    let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                    quote! {
                        #fi: #ft_ts read_only_ids_merged_with_create_into_opt_v_read(
                            #read_only_ids_ts.#fi #mb_dot_clone_ts.expect("f967434c"),
                            #create_ts.#fi #mb_dot_clone_ts
                        )
                    }
                })
            };
            let ident_create_name_ts = quote! {ident_create};
            let read_only_ids_returned_from_create_one_name_ts =
                quote! {read_only_ids_returned_from_create_one};
            (
                gen_ts(&ReadOnlyIdsSc, &CreateSc, &ShouldAddDotClone::False),
                gen_ts(
                    &quote! {read_only_ids_from_try_create_one},
                    &ident_create_name_ts,
                    &ShouldAddDotClone::False,
                ),
                gen_ts(
                    &read_only_ids_returned_from_create_one_name_ts,
                    &quote! {ident_create_default},
                    &ShouldAddDotClone::False,
                ),
                gen_ts(
                    &read_only_ids_returned_from_create_one_name_ts,
                    &ident_create_name_ts,
                    &ShouldAddDotClone::True,
                ),
            )
        };
        let opt_ident_where_many_ts_dc1232c7 =
            gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| {
                let fi = &el.ident;
                quote! {#fi: None}
            });
        let select_default_all_with_max_page_size_clone_ts =
            quote! {select_default_all_with_max_page_size.clone()};
        let common_read_only_ids_returned_from_create_one_ts = {
            let pk_read_ts = quote! {pk_read};
            let pk_read_clone_ts = quote! {pk_read.clone()};
            let ts = gen_v_init_ts0(&pk_read_clone_ts);
            let assert_eq_ts_4f6bbe8a = gen_assert_eq_ts(
                &quote! {
                    #ident_read_ucc {
                        #pk_fi: Some(#ts),
                        #fields_none_init_ts
                    }
                },
                &quote! {
                    gen_ident_try_read_one_handle_pk(
                        &#UrlSc,
                        #pk_read_clone_ts,
                        #SelectPkSc.clone(),
                        &table_init
                    )
                    .await
                    .expect("36b95e96")
                },
                &quote! {"3d9f2ec0"},
            );
            let assert_eq_ts_947d2096 = gen_assert_eq_ts(
                &quote! {
                    gen_try_delete_one_handle(
                        &url,
                        #pk_read_clone_ts,
                        &table_init,
                    ).await.expect("4d96d385")
                },
                &quote! {#pk_read_clone_ts},
                &quote! {"26e2058b"},
            );
            quote! {
                let #CommonReadOnlyIdsReturnedFromCreateOneSc = {
                    let read_only_ids_from_try_create_one = gen_read_only_ids_from_try_create_one_default(&#UrlSc, &table_init).await;
                    let pk_read = #pk_ft_read_only_ids_into_read_read_only_ids_from_try_create_one_pk_fi_ts;
                    #assert_eq_ts_4f6bbe8a
                    #assert_eq_ts_947d2096
                    gen_check_no_rows_returned_from_ident_try_read_one_handle_pk(
                        &url,
                        #pk_read_ts,
                        #select_default_all_with_max_page_size_clone_ts,
                        &table_init,
                    ).await;
                    read_only_ids_from_try_create_one
                };
            }
        };
        let gen_ident_create_ts = |fi: &Ident, ts: &dyn ToTokens| {
            gen_fields_named_without_pk_with_comma_ts(&|el: &SynFieldWrapper| {
                let fi0 = &el.ident;
                let ft0 = &el.type0;
                let ts0 = if fi == fi0 {
                    quote! {#ts}
                } else {
                    let ts1 = gen_as_pg_type_path_ts(&ft0);
                    quote! {<#ts1 Create as #import_ts DefaultOptSomeVecOneEl>::default_opt_some_vec_one_el()}
                };
                quote! {#fi0: #ts0}
            })
        };
        let gen_ident_create_content_el_id_ts =
            |fi: &Ident, el_ts: &dyn ToTokens| gen_ident_create_ts(fi, &el_ts);
        let gen_ident_create_content_el_ts = |fi: &Ident| gen_ident_create_ts(fi, &ElSc);
        let gen_table_test_name_fi_ts = |test_name: &str, fi: &Ident| {
            format!("table_{test_name}_{fi}")
                .parse::<Ts2>()
                .expect("eb30c1e4")
        };
        let mut table_fis_init_vec_ts = Vec::new();
        let mut table_test_name_fis_vec_ts = Vec::new();
        let mut fill_table_fis_vec_ts = |test_names: Vec<&str>| {
            for el0 in test_names {
                let gen_init_variable_name_ts = |fi: &Ident| {
                    format!("table_{el0}_{fi}")
                        .parse::<Ts2>()
                        .expect("2003ad9f")
                };
                table_fis_init_vec_ts.push(gen_fields_named_without_pk_without_comma_ts(
                    &|el: &SynFieldWrapper| {
                        let fi = &el.ident;
                        let init_variable_name_ts = gen_init_variable_name_ts(fi);
                        let format_ts = dq_ts(&format!("{el0}_{fi}"));
                        quote! {
                            let #init_variable_name_ts = add_table_postfix(#format_ts);
                        }
                    },
                ));
                table_test_name_fis_vec_ts.push(gen_fields_named_without_pk_without_comma_ts(
                    &|el1: &SynFieldWrapper| {
                        let fi = &el1.ident;
                        let init_variable_name_ts = gen_init_variable_name_ts(fi);
                        quote! {&#init_variable_name_ts,}
                    },
                ));
            }
        };
        let table_read_only_ids_merged_with_create_into_where_equal_name = "8e427ad7";
        let table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name =
            "eb24448c";
        let table_read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_name =
            "9ac6d79a";
        let table_create_into_pg_type_opt_vec_where_dim_one_equal_name = "72940b0e";
        let table_read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_name =
            "5a52af33";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_name =
            "1f388ef8";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_name =
            "581c947f";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_name =
            "de556c26";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_name =
            "35b26a97";
        let table_create_into_pg_json_type_opt_vec_where_length_equal_name = "1ce53b67";
        let table_create_into_pg_json_type_opt_vec_where_length_greater_than_name = "6b6bdfe0";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_name =
            "35a01678";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_name =
            "33a3706a";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_name =
            "a3e2165c";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_name =
            "427ac837";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_name =
            "fe3267a0";
        let table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_name =
            "b4504737";
        fill_table_fis_vec_ts(vec![
            &table_read_only_ids_merged_with_create_into_where_equal_name,
            &table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name,
            &table_read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_name,
            &table_create_into_pg_type_opt_vec_where_dim_one_equal_name,
            &table_read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_name,
            &table_create_into_pg_json_type_opt_vec_where_length_equal_name,
            &table_create_into_pg_json_type_opt_vec_where_length_greater_than_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_name,
            &table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_name,
        ]);
        let select_default_all_with_max_page_size_cloned_clone_ts =
            quote! {select_default_all_with_max_page_size_cloned.clone()};
        let read_only_ids_to_two_dims_vec_read_inner_acc_fields_ts =
            gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                let fi = &el.ident;
                let fi_read_only_ids_to_two_dims_vec_read_inner_acc_sc =
                    SelfReadOnlyIdsToTwoDimsVecReadInnerAccSc::from_tokens(&fi);
                let ident_create_defaults_for_column_read_only_ids_to_two_dims_vec_read_inner_ts =
                    gen_fields_named_without_pk_without_comma_ts(&|el0: &SynFieldWrapper| {
                        let fi0 = &el0.ident;
                        let ft0 = &el0.type0;
                        if fi == fi0 {
                            gen_if_let_some_ts(
                                &quote! {v_a5f7e6cd},
                                &quote! {&common_read_only_ids_returned_from_create_one.#fi0},
                                &{
                                    let ft_ts = gen_as_pg_type_test_cases_path_ts(&ft0);
                                    quote! {
                                        for el_b3522b7d in #ft_ts read_only_ids_to_two_dims_vec_read_inner(v_a5f7e6cd) {
                                            for _ in el_b3522b7d {
                                                acc_458cda9e.push(ident_create_default.clone());
                                            }
                                        }
                                    }
                                },
                            )
                        } else {
                            Ts2::new()
                        }
                    });
                quote! {
                    let #fi_read_only_ids_to_two_dims_vec_read_inner_acc_sc = {
                        let mut acc_458cda9e = Vec::new();
                        #ident_create_defaults_for_column_read_only_ids_to_two_dims_vec_read_inner_ts
                        acc_458cda9e
                    };
                }
            });
        let gen_read_only_ids_els_ts_fe29ff70 = {
            let ident_read_fields_init_without_pk_ts = gen_fields_named_without_pk_with_comma_ts(
                &|syn_field_wrapper: &SynFieldWrapper| {
                    let fi = &syn_field_wrapper.ident;
                    let ts = gen_v_init_ts0(&PgCrudDefaultOptSomeVecOneElCall);
                    let ts0 = gen_as_pg_type_test_cases_path_ts(&syn_field_wrapper.type0);
                    quote! {
                        #fi: el_f108da5a.#fi.as_ref().map_or_else(
                            || Some(#ts),
                            #ts0 read_only_ids_to_opt_v_read_default_opt_some_vec_one_el
                        )
                    }
                },
            );
            let assert_eq_ts_d7cc4bd8 = gen_assert_eq_ts(
                &quote! {
                    itertools::Itertools::sorted_by(
                        read_only_ids_els_efeed554.iter().map(|el_f108da5a| {
                            #ident_read_ucc {
                                #pk_fi: #pk_as_pg_type_test_cases_path_ts read_only_ids_to_opt_v_read_default_opt_some_vec_one_el(
                                    &el_f108da5a.#pk_fi
                                ),
                                #ident_read_fields_init_without_pk_ts
                            }
                        }),
                        |first, second| match (&first.#pk_fi, &second.#pk_fi) {
                            (Some(first_handle), Some(second_handle)) => first_handle.#VSc.cmp(&second_handle.#VSc),
                            _ => panic!("0f1d45ed"),
                        }
                    ).collect::<Vec<#ident_read_ucc>>()
                },
                &quote! {
                    itertools::Itertools::sorted_by(
                        gen_try_read_many_order_by_pk_with_big_pagination(
                            url,
                            gen_ident_where_many_pripery_k_others_none(
                                Some(
                                    gen_pg_type_where_try_new_pk(
                                        #operator_or_ts,
                                        read_only_ids_els_efeed554 #ts_ffb964de
                                    )
                                )
                            ),
                            #select_default_all_with_max_page_size_clone_ts,
                            table_9c259e1c
                        )
                        .await
                        .expect("097d5e7d")
                        .into_iter(),
                        |first, second| match (&first.#pk_fi, &second.#pk_fi) {
                            (Some(first_handle), Some(second_handle)) => first_handle.#VSc.cmp(&second_handle.#VSc),
                            _ => panic!("51e477ea"),
                        }
                    )
                    .collect::<Vec<#ident_read_ucc>>()
                },
                &quote! {"50198a7f"},
            );
            quote! {
                async fn gen_read_only_ids_els_8a1ef027(
                    url: &str,
                    table_9c259e1c: &str,
                    select_default_all_with_max_page_size: #import_ts NotEmptyUniqueVec<#ident_select_ucc>,
                    read_only_ids_to_two_dims_vec_read_inner_acc: Vec<#ident_create_ucc>
                ) -> Vec<#ident_read_only_ids_ucc> {
                    let read_only_ids_els_efeed554 = futures::StreamExt::collect::<Vec<Vec<#ident_read_only_ids_ucc>>>(
                        futures::StreamExt::buffer_unordered(
                            futures::stream::iter(
                                read_only_ids_to_two_dims_vec_read_inner_acc
                                .chunks(25)
                                .map(Vec::from)
                                .map(|el_8e425cb1| futures::FutureExt::boxed(async move { #ident::try_create_many_handle(
                                    url,
                                    #ident_create_many_params_ucc {
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
                    #assert_eq_ts_d7cc4bd8
                    read_only_ids_els_efeed554
                }
            }
        };
        let gen_ft_opt_vec_create_ts = |ft: &Type| {
            let ts = gen_as_pg_type_test_cases_path_ts(ft);
            quote! {#ts #OptVecCreateSc()}
        };
        let gen_ft_opt_vec_create_or_vec_ts = |ft: &Type| {
            let ts = gen_ft_opt_vec_create_ts(ft);
            quote! {#ts.unwrap_or(Vec::new())}
        };
        let gen_ident_ft_opt_vec_create_or_vec_ts =
            |_: &Ident, ft: &Type| gen_ft_opt_vec_create_or_vec_ts(ft);
        let gen_for_in_1_2_ts = |el_ts: &dyn ToTokens, ts: &dyn ToTokens| {
            quote! {
                for #el_ts in [1,2] {
                    #ts
                }
            }
        };
        let gen_vec_pk_sorted_read_ts = |ts: &dyn ToTokens| {
            quote! {itertools::Itertools::sorted(#ts).collect::<Vec<#pk_ft_as_pg_type_read_ts>>()}
        };
        let vec_pk_sorted_read_ts = gen_vec_pk_sorted_read_ts(&quote! {
            read_only_ids_from_try_create_many
            .into_iter()
            .map(|el_43ab7fb5| {
                #pk_ft_read_only_ids_into_read_el_43ab7fb5_pk_fi_ts
            })
        });
        let gen_try_delete_many_handle_ts = |ts: &dyn ToTokens, table_ts: &dyn ToTokens| {
            quote! {
                #ident::try_delete_many_handle(
                    &url_cloned,
                    #ident_delete_many_params_ucc {
                        //todo rewrite it using new\try_new?
                        payload: #ident_delete_many_payload_ucc {
                            where_many: #opt_ident_where_many_ucc(Some(
                                #ident_where_many_ucc {
                                    #ts
                                }
                            ))
                        }
                    },
                    &#table_ts
                )
                .await
                .expect("716e470e")
            }
        };
        let gen_read_only_ids_from_try_delete_many_ts = |ts: &dyn ToTokens| {
            quote! {
                let read_only_ids_from_try_delete_many = #ts;
            }
        };
        let gen_read_only_ids_from_try_delete_many_sorted_pk_ts =
            |table_ts: &dyn ToTokens, some_ts: &dyn ToTokens| {
                gen_read_only_ids_from_try_delete_many_ts(&gen_vec_pk_sorted_read_ts(&{
                    let ts = gen_try_delete_many_handle_ts(
                        &quote! {
                            #pk_fi: Some(#some_ts),
                            #opt_ident_where_many_ts_dc1232c7
                        },
                        &table_ts,
                    );
                    quote! {#ts.into_iter()}
                }))
            };
        let gen_acc_push_future_ts = |ts0: &dyn ToTokens,
                                      ts1: &dyn ToTokens,
                                      ts2: &dyn ToTokens| {
            quote! {
                let #ts0 = #ts1.clone();
                let url_cloned = url.clone();
                let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                acc_9189f86e.push(futures::FutureExt::boxed(async move {
                    #ts2
                }));
            }
        };
        let ts_611ddc2e = quote! {
            gen_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create(
                read_only_ids_from_try_create_many.clone(),
                ident_vec_create.clone()
            )
        };
        let create_many_tests_ts = {
            let create_many_tests_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let ident_create_ts_910fa600 =
                        gen_ident_create_content_el_id_ts(fi, &quote! {el_03a4f4ee});
                    let ft_opt_vec_create_or_vec_ts = gen_ft_opt_vec_create_or_vec_ts(ft);
                    let assert_eq_ts_b47328e3 = gen_assert_eq_ts(
                        &ts_611ddc2e,
                        &quote! {
                            gen_try_read_many_order_by_pk_with_big_pagination(
                                &url_cloned,
                                gen_ident_where_many_pripery_k_others_none(
                                    Some(
                                        gen_pg_type_where_try_new_pk(
                                            #operator_or_ts,
                                            read_only_ids_from_try_create_many #ts_ffb964de
                                        )
                                    )
                                ),
                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                &table_create_many_cloned
                            ).await.expect("bdb72341")
                        },
                        &quote! {"d19bbbf6"},
                    );
                    let assert_eq_ts_78d9a1bd = gen_assert_eq_ts(
                        &quote! {read_only_ids_from_try_delete_many},
                        &vec_pk_sorted_read_ts,
                        &quote! {"f58f5572"},
                    );
                    let assert_ts_56d830a6 = gen_assert_ts(
                        &{
                            let ts =
                                gen_pk_where_equal_into_inner_ts(&quote! {el_a37bca54.clone()});
                            quote! {
                                gen_try_read_many_order_by_pk_with_big_pagination(
                                    &url_cloned,
                                    gen_ident_where_many_pripery_k_others_none(
                                        Some(
                                            gen_pg_type_where_try_new_pk(
                                                #operator_or_ts,
                                                {
                                                    let mut acc_87ea12c9 = Vec::new();
                                                    for el_a37bca54 in &read_only_ids_from_try_delete_many {
                                                        acc_87ea12c9.push(#ts);
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
                                .is_empty()
                            }
                        },
                        &quote! {"4e88679a"},
                    );
                    let ts_08a18039 = gen_read_only_ids_from_try_delete_many_sorted_pk_ts(
                        &quote! {table_create_many_cloned},
                        &quote! {
                            gen_pg_type_where_try_new_or_pks(&read_only_ids_from_try_create_many)
                        },
                    );
                    let ts_f318a803 = gen_acc_push_future_ts(
                        &quote! {table_create_many_cloned},
                        &quote! {table_create_many},
                        &quote! {
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
                                #ident_create_many_params_ucc {
                                    #PayloadSc: #ident_create_many_payload_ucc(ident_vec_create.clone())
                                },
                                &table_create_many_cloned.clone()
                            ).await.expect("5eecedc4");
                            #assert_eq_ts_b47328e3
                            #ts_08a18039
                            #assert_eq_ts_78d9a1bd
                            #assert_ts_56d830a6
                        },
                    );
                    quote! {
                        for el_fce0969c in #ft_opt_vec_create_or_vec_ts
                            .chunks(10)
                            .map(Vec::from)
                        {
                            #ts_f318a803
                        }
                    }
                },
            );
            quote! {#create_many_tests_ts}
        };
        let create_one_tests_ts = {
            let create_one_tests_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let ident_create_ts_f75e4ef0 =
                        gen_ident_create_content_el_id_ts(fi, &quote! {el_7632d698});
                    let ts = gen_v_init_ts0(
                        &pk_ft_read_only_ids_into_read_read_only_ids_from_try_create_one_pk_fi_ts,
                    );
                    let ft_opt_vec_create_or_vec_ts = gen_ft_opt_vec_create_or_vec_ts(ft);
                    let assert_eq_ts_e2916686 = gen_assert_eq_ts(
                        &quote! {
                            #ident_read_ucc {
                                #pk_fi: Some(#ts),
                                #fi_read_only_ids_merged_with_create_into_opt_v_read_read_only_ids_from_try_create_one_ident_create_ts
                            }
                        },
                        &quote! {
                            gen_ident_try_read_one_handle_pk(
                                &url_cloned,
                                #pk_ft_read_only_ids_into_read_read_only_ids_from_try_create_one_pk_fi_ts,
                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                &table_create_one_cloned
                            )
                            .await
                            .expect("f8e1cb88")
                        },
                        &quote! {"5f2adbed"},
                    );
                    let assert_eq_ts_f5d5140f = gen_assert_eq_ts(
                        &quote! {
                            gen_try_delete_one_handle(
                                &url_cloned,
                                #pk_ft_read_only_ids_into_read_read_only_ids_from_try_create_one_pk_fi_ts,
                                &table_create_one_cloned
                            ).await.expect("20d5a40a")
                        },
                        &quote! {#pk_ft_read_only_ids_into_read_read_only_ids_from_try_create_one_pk_fi_ts},
                        &quote! {"4f563faf"},
                    );
                    let ts_eb57f4ce = gen_acc_push_future_ts(
                        &quote! {table_create_one_cloned},
                        &quote! {table_create_one},
                        &quote! {
                            let ident_create = #ident_create_ucc {
                                #ident_create_ts_f75e4ef0
                            };
                            let read_only_ids_from_try_create_one = gen_read_only_ids_from_try_create_one(
                                &url_cloned,
                                ident_create.clone(),
                                &table_create_one_cloned
                            ).await;
                            #assert_eq_ts_e2916686
                            #assert_eq_ts_f5d5140f
                            gen_check_no_rows_returned_from_ident_try_read_one_handle_pk(
                                &url_cloned,
                                #pk_ft_read_only_ids_into_read_read_only_ids_from_try_create_one_pk_fi_ts,
                                select_default_all_with_max_page_size_cloned,
                                &table_create_one_cloned,
                            ).await;
                        },
                    );
                    quote! {
                        for el_7632d698 in #ft_opt_vec_create_or_vec_ts {
                            #ts_eb57f4ce
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
                    let _: #pk_ft_as_pg_type_read_ts = gen_try_delete_one_handle(
                        &url_cloned,
                        #pk_ft_read_only_ids_into_read_read_only_ids_from_try_create_one_pk_fi_ts,
                        &table_7e35b1ce
                    ).await.expect("93b4bf61");
                    gen_check_no_rows_returned_from_ident_try_read_one_handle_pk(
                        &url_cloned,
                        #pk_ft_read_only_ids_into_read_read_only_ids_from_try_create_one_pk_fi_ts,
                        select_default_all_with_max_page_size_cloned,
                        &table_7e35b1ce,
                    ).await;
                }
            };
        let read_many_tests_ts = {
            //todo extra read_many checks
            let test_read_many_by_non_existent_pks_ts = gen_for_in_1_2_ts(
                &quote! {el_30614c66},
                &gen_acc_push_future_ts(
                    &quote!{table_7e35b1ce},
                    &quote!{table_test_read_many_by_non_existent_pks},
                    &add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&gen_assert_ts(
                        &quote!{
                            gen_try_read_many_order_by_pk_with_big_pagination(
                                &url_cloned,
                                gen_ident_where_many_pripery_k_others_none(
                                    Some(
                                        gen_pg_type_where_try_new_pk(
                                            #operator_or_ts,
                                            std::iter::repeat_with(|| #pk_where_equal_uuid_new_v_ts)
                                            .take(el_30614c66)
                                            .collect::<Vec<_>>()
                                        )
                                    )
                                ),
                                select_default_all_with_max_page_size_cloned.clone(),
                                &table_7e35b1ce
                            ).await
                            .expect("e661c49b")
                            .is_empty()
                        },
                        &quote!{"06df4025"}
                    ))
                )
            );
            let test_read_many_by_equal_to_created_pks_ts = gen_for_in_1_2_ts(
                &quote! {el_a636d084},
                &{
                    let ts = gen_acc_push_future_ts(
                        &quote!{table_7e35b1ce},
                        &quote!{table_test_read_many_by_equal_to_created_pks},
                        &add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                            let assert_eq_ts_03eaa791 = gen_assert_eq_ts(
                                &ts_611ddc2e,
                                &quote! {
                                    gen_try_read_many_order_by_pk_with_big_pagination(
                                        &url_cloned,
                                        gen_ident_where_many_pripery_k_others_none(
                                            Some(
                                                gen_pg_type_where_try_new_pk(
                                                    #operator_or_ts,
                                                    read_only_ids_from_try_create_many #ts_ffb964de
                                                )
                                            )
                                        ),
                                        select_default_all_with_max_page_size_cloned.clone(),
                                        &table_7e35b1ce
                                    ).await.expect("b8efe770")
                                },
                                &quote! {"er 3b2cf1f5-2c4e-4908-ba66-f4af84fe0893"},
                            );
                            let assert_eq_ts_10f06d56 = gen_assert_eq_ts(
                                &quote! {read_only_ids_from_try_delete_many},
                                &vec_pk_sorted_read_ts,
                                &quote! {"ebbbea6e"},
                            );
                            let assert_ts_a5027b61 = gen_assert_ts(
                                &{
                                    let ts = gen_pk_where_equal_into_inner_ts(&quote!{el_1e9c87ce.clone()});
                                    quote!{
                                        gen_try_read_many_order_by_pk_with_big_pagination(
                                            &url_cloned,
                                            gen_ident_where_many_pripery_k_others_none(
                                                Some(
                                                    gen_pg_type_where_try_new_pk(
                                                        #operator_or_ts,
                                                        read_only_ids_from_try_delete_many
                                                        .iter()
                                                        .map(|el_1e9c87ce| #ts)
                                                        .collect()
                                                    )
                                                )
                                            ),
                                            select_default_all_with_max_page_size_cloned.clone(),
                                            &table_7e35b1ce
                                        ).await
                                        .expect("1f079962")
                                        .is_empty()
                                    }
                                },
                                &quote!{"d79c0af3"}
                            );
                            let ts_83ae6884 = gen_read_only_ids_from_try_delete_many_sorted_pk_ts(
                                &quote!{table_7e35b1ce},
                                &quote!{
                                    gen_pg_type_where_try_new_or_pks(&read_only_ids_from_try_create_many)
                                }
                            );
                            quote! {
                                let ident_vec_create = std::iter::repeat_n(
                                    ident_create_default_cloned.clone(),//todo mb remove
                                    el_a636d084
                                ).collect::<Vec<#ident_create_ucc>>();
                                let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                                    &url_cloned,
                                    #ident_create_many_params_ucc {
                                        payload: #ident_create_many_payload_ucc(ident_vec_create.clone())
                                    },
                                    &table_7e35b1ce
                                ).await.expect("d775179f");
                                #assert_eq_ts_03eaa791
                                #ts_83ae6884
                                #assert_eq_ts_10f06d56
                                #assert_ts_a5027b61
                            }
                        })
                    );
                    quote! {
                        let ident_create_default_cloned = ident_create_default.clone();
                        #ts
                    }
                },
            );
            let gen_read_only_ids_merged_with_create_into_where_assert_eq_ts =
                |ts: &dyn ToTokens| {
                    gen_assert_eq_ts(
                        &quote! {vec![
                            #ident_read_ucc {
                                #pk_fi: #pk_as_pg_type_test_cases_path_ts read_only_ids_to_opt_v_read_default_opt_some_vec_one_el(
                                    &read_only_ids_returned_from_create_one.#pk_fi
                                ),
                                #fi_read_only_ids_merged_with_create_into_opt_v_read_read_only_ids_returned_from_create_one_clone_ident_create_clone_ts
                            }
                        ]},
                        &quote! {
                            gen_try_read_many_order_by_pk_with_big_pagination(
                                &url_cloned,
                                #ident_where_many_ucc::try_new(#ts).expect("83c2d430"),
                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                &table_7e35b1ce
                            ).await.expect("c3e316c0")
                        },
                        &quote! {"ee8d232d"},
                    )
                };
            let gen_fi_ft_opt_vec_create_filter_not_empty_or_vec_fi_default_ts =
                |fi: &Ident, ft: &Type| {
                    let ts = gen_ft_opt_vec_create_ts(ft);
                    quote! {
                        #ts
                        .filter(|el_bba28182| !el_bba28182.is_empty())
                        .unwrap_or_else(|| vec![ident_create_default.#fi.clone()])
                    }
                };
            let gen_read_test_ts =
                |test_name: &str,
                 gen_method_call_ts: &dyn Fn(&Ident, &Type) -> Ts2,
                 gen_create_ts: &dyn Fn(&Ident) -> Ts2,
                 gen_ts: &dyn Fn(&SynFieldWrapper) -> Ts2| {
                    gen_fields_named_without_pk_without_comma_ts(&|el: &SynFieldWrapper| {
                        let fi = &el.ident;
                        let ft = &el.type0;
                        let method_call_ts = gen_method_call_ts(fi, ft);
                        let table_test_name_fi_ts = gen_table_test_name_fi_ts(test_name, fi);
                        let ident_create_ts_013035e1 = gen_create_ts(fi);
                        let ts = gen_ts(el);
                        let assert_eq_ts_b444d33d = gen_assert_eq_ts(
                            &quote! {read_only_ids_from_try_delete_many},
                            &quote! {vec![#pk_ft_read_only_ids_into_read_read_only_ids_returned_from_create_one_pk_fi_ts]},
                            &quote! {"9fc29fa5"},
                        );
                        let assert_ts_87ec2ac1 = gen_assert_ts(
                            &{
                                let ts_75998fa9 = gen_pk_where_equal_into_inner_ts(
                                    &pk_ft_read_only_ids_into_read_read_only_ids_returned_from_create_one_pk_fi_ts
                                );
                                quote! {
                                    gen_try_read_many_order_by_pk_with_big_pagination(
                                        &url_cloned,
                                        gen_ident_where_many_pripery_k_others_none(
                                            Some(
                                                gen_pg_type_where_try_new_pk(
                                                    #operator_or_ts,
                                                    vec![#ts_75998fa9]
                                                )
                                            )
                                        ),
                                        #select_default_all_with_max_page_size_cloned_clone_ts,
                                        &table_7e35b1ce
                                    ).await
                                    .expect("1817b67a")
                                    .is_empty()
                                }
                            },
                            &quote! {"38187925"},
                        );
                        let ts_490b2aa3 = gen_pk_where_equal_into_inner_ts(
                            &pk_ft_read_only_ids_into_read_read_only_ids_returned_from_create_one_pk_fi_ts
                        );
                        let ts_7a8c2dcc = gen_read_only_ids_from_try_delete_many_sorted_pk_ts(
                            &quote! {table_7e35b1ce},
                            &quote! {
                                gen_pg_type_where_try_new_pk(
                                    #operator_or_ts,
                                    vec![#ts_490b2aa3]
                                )
                            },
                        );
                        let ts_4c3245ed = gen_acc_push_future_ts(
                            &quote! {table_7e35b1ce},
                            &table_test_name_fi_ts,
                            &quote! {
                                let ident_create = #ident_create_ucc {
                                    #ident_create_ts_013035e1
                                };
                                let read_only_ids_returned_from_create_one = gen_read_only_ids_from_try_create_one(
                                    &url_cloned,
                                    ident_create.clone(),
                                    &table_7e35b1ce
                                ).await;
                                #ts
                                #ts_7a8c2dcc
                                #assert_eq_ts_b444d33d
                                #assert_ts_87ec2ac1
                            },
                        );
                        quote! {
                            for #ElSc in #method_call_ts {
                                #ts_4c3245ed
                            }
                        }
                    })
                };
            let some_pk_where_init_ts = quote! {
                Some(
                    gen_pg_type_where_try_new_pk(
                        #operator_and_ts,
                        vec![
                            #pk_as_pg_type_test_cases_path_ts read_only_ids_merged_with_create_into_where_equal(
                                read_only_ids_returned_from_create_one.#pk_fi,
                                #PgCrudDefaultOptSomeVecOneElCall
                            )
                        ]
                    )
                )
            };
            let gen_ts_ccbfdac5 = |fi: &Ident, ts: &dyn ToTokens| {
                gen_fields_named_with_comma_ts(&|el0: &SynFieldWrapper| {
                    let fi0 = &el0.ident;
                    if pk_fi == fi0 {
                        some_pk_where_init_ts.clone()
                    } else if fi0 == fi {
                        gen_some_pg_type_where_try_new_and_ts(&ts)
                    } else {
                        none_ts.clone()
                    }
                })
            };
            let (
                read_only_ids_merged_with_create_into_where_equal_ts,
                read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts,
            ) = {
                let gen_ts =
                    |test_name: &str, equal_or_equal_using_fields: &EqualOrEqualUsingFields| {
                        gen_read_test_ts(
                            test_name,
                            &gen_ident_ft_opt_vec_create_or_vec_ts,
                            &gen_ident_create_content_el_ts,
                            &|el: &SynFieldWrapper| {
                                let fi = &el.ident;
                                gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                                    &gen_fields_named_with_comma_ts(&|el0: &SynFieldWrapper| {
                                        let fi0 = &el0.ident;
                                        let ft0 = &el0.type0;
                                        if fi0 == pk_fi {
                                            some_pk_where_init_ts.clone()
                                        } else if fi0 == fi {
                                            let method_ts = {
                                                let method_ts: &dyn ToTokens = match &equal_or_equal_using_fields {
                                                    EqualOrEqualUsingFields::Equal => &ReadOnlyIdsMergedWithCreateIntoWhereEqualSc,
                                                    EqualOrEqualUsingFields::EqualUsingFields => &ReadOnlyIdsMergedWithCreateIntoVecWhereEqualUsingFieldsSc
                                                };
                                                let ts0 = gen_as_pg_type_test_cases_path_ts(&ft0);
                                                quote! {
                                                    #ts0 #method_ts(
                                                        read_only_ids_returned_from_create_one.#fi0.clone().expect("11c3740b"),
                                                        ident_create.#fi0.clone()
                                                    )
                                                }
                                            };
                                            match &equal_or_equal_using_fields {
                                                EqualOrEqualUsingFields::Equal => {
                                                    gen_some_pg_type_where_try_new_and_ts(&quote! {
                                                        vec![#method_ts]
                                                    })
                                                }
                                                EqualOrEqualUsingFields::EqualUsingFields => {
                                                    quote! {
                                                        Some(#import_ts PgTypeWhere::new(
                                                            #operator_and_ts,
                                                            #method_ts
                                                        ))
                                                    }
                                                }
                                            }
                                        } else {
                                            none_ts.clone()
                                        }
                                    }),
                                )
                            },
                        )
                    };
                (
                    gen_ts(table_read_only_ids_merged_with_create_into_where_equal_name, &EqualOrEqualUsingFields::Equal),
                    gen_ts(table_read_only_ids_merged_with_create_into_vec_where_equal_using_fields_name, &EqualOrEqualUsingFields::EqualUsingFields),
                )
            };
            let read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_ts = gen_read_test_ts(table_read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_name, &gen_ident_ft_opt_vec_create_or_vec_ts, &gen_ident_create_content_el_ts, &|el: &SynFieldWrapper| {
                let fi = &el.ident;
                gen_if_let_some_ts(
                    &quote!{v_d5cd3c70},
                    &{
                        let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                        quote!{#ft_ts #ReadOnlyIdsMergedWithCreateIntoOptVecWhereEqualToJsonFieldSc(
                            read_only_ids_returned_from_create_one.#fi.clone().expect("65cef584"),
                            ident_create.#fi.clone()
                        )}
                    },
                    &{
                        let assert_eq_ts = gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                            &gen_ts_ccbfdac5(fi, &quote! {vec![el_48a3d976]})
                        );
                        quote!{
                            for el_48a3d976 in v_d5cd3c70.into_vec() {
                                #assert_eq_ts
                            }
                        }
                    }
                )
            });
            let create_into_pg_type_opt_vec_where_dim_one_equal_ts = gen_read_test_ts(
                table_create_into_pg_type_opt_vec_where_dim_one_equal_name,
                &gen_ident_ft_opt_vec_create_or_vec_ts,
                &gen_ident_create_content_el_ts,
                &|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    gen_if_let_some_ts(
                        &quote! {v_b02d763d},
                        &{
                            let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                            quote! {#ft_ts #CreateIntoPgTypeOptVecWhereDimOneEqualSc(ident_create.#fi.clone())}
                        },
                        &{
                            let assert_eq_ts =
                                gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                                    &gen_ts_ccbfdac5(fi, &quote! {vec![el_39d1fb5d]}),
                                );
                            quote! {
                                for el_39d1fb5d in v_b02d763d.into_vec() {
                                    #assert_eq_ts
                                }
                            }
                        },
                    )
                },
            );
            let read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_ts = gen_read_test_ts(
                table_read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_name,
                &|_: &Ident, ft: &Type| {
                    quote! {
                        <#ft as #import_ts PgTypeTestCases>::#PgTypeOptVecWhereGreaterThanTestSc()
                        .map_or_else(Vec::new, Into::into)
                    }
                },
                &|fi: &Ident| {
                    gen_ident_create_ts(
                        fi,
                        &quote! {#ElSc.#CreateSc},
                    )
                },
                &|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    gen_if_let_some_ts(
                        &quote!{v_60baba1f},
                        &{
                            let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                            quote!{#ft_ts #ReadOnlyIdsMergedWithTableTypeIntoPgTypeOptWhereGreaterThanSc(
                                #ElSc.vrt,
                                read_only_ids_returned_from_create_one.#fi.clone().expect("c8d34556"),
                                #ElSc.greater_than,
                            )}
                        },
                        &gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                            &gen_ts_ccbfdac5(fi,&quote!{vec![v_60baba1f]})
                        )
                    )
                },
            );
            let (
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts,
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts,
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts,
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts,
            ) = {
                //todo if vec_create is empty then do different logic (for uuid). now uuid Tested using one default case
                let gen_ts = |test_name: &str, dim: &Dim| {
                    let fn_ts = dim.read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_nbr_equal_sc();
                    gen_read_test_ts(
                        test_name,
                        &gen_fi_ft_opt_vec_create_filter_not_empty_or_vec_fi_default_ts,
                        &gen_ident_create_content_el_ts,
                        &|el: &SynFieldWrapper| {
                            let fi = &el.ident;
                            let ft = &el.type0;
                            let ft_ts = gen_as_pg_type_test_cases_path_ts(ft);
                            gen_if_let_some_ts(
                                &quote! {v_bb67b871},
                                &quote! {#ft_ts #fn_ts(
                                    read_only_ids_returned_from_create_one.#fi.clone().expect("2ed000a5"),
                                    ident_create.#fi.clone()
                                )},
                                &{
                                    let assert_eq_ts =
                                        gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                                            &gen_ts_ccbfdac5(fi, &quote! {vec![el_3efa0bb4]}),
                                        );
                                    quote! {
                                        for el_3efa0bb4 in v_bb67b871.into_vec() {
                                            #assert_eq_ts
                                        }
                                    }
                                },
                            )
                        },
                    )
                };
                (
                    gen_ts(table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_name, &Dim::One),
                    gen_ts(table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_name, &Dim::Two),
                    gen_ts(table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_name, &Dim::Three),
                    gen_ts(table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_name, &Dim::Four),
                )
            };
            let create_into_pg_json_type_opt_vec_where_length_equal_ts = gen_read_test_ts(
                table_create_into_pg_json_type_opt_vec_where_length_equal_name,
                &gen_fi_ft_opt_vec_create_filter_not_empty_or_vec_fi_default_ts,
                &gen_ident_create_content_el_ts,
                &|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    gen_if_let_some_ts(
                        &quote! {v_f825e068},
                        &{
                            let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                            quote! {#ft_ts #CreateIntoPgJsonTypeOptVecWhereLengthEqualSc(
                                ident_create.#fi.clone()
                            )}
                        },
                        &{
                            let assert_eq_ts =
                                gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                                    &gen_ts_ccbfdac5(fi, &quote! {vec![el_c09ef321]}),
                                );
                            quote! {
                                for el_c09ef321 in v_f825e068.into_vec() {
                                    #assert_eq_ts
                                }
                            }
                        },
                    )
                },
            );
            let create_into_pg_json_type_opt_vec_where_length_greater_than_ts = gen_read_test_ts(
                table_create_into_pg_json_type_opt_vec_where_length_greater_than_name,
                &gen_fi_ft_opt_vec_create_filter_not_empty_or_vec_fi_default_ts,
                &gen_ident_create_content_el_ts,
                &|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    gen_if_let_some_ts(
                        &quote! {v_cd4aa374},
                        &{
                            let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                            quote! {#ft_ts #CreateIntoPgJsonTypeOptVecWhereLengthGreaterThanSc(
                                ident_create.#fi.clone()
                            )}
                        },
                        &{
                            let assert_eq_ts =
                                gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                                    &gen_ts_ccbfdac5(fi, &quote! {vec![el_527b546b]}),
                                );
                            quote! {
                                for el_527b546b in v_cd4aa374.into_vec() {
                                    #assert_eq_ts
                                }
                            }
                        },
                    )
                },
            );
            let (
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts,
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts,
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts,
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts,
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts,
                read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts,
            ) = {
                let gen_ts = |table_name: &str, method_ts: &dyn ToTokens| {
                    gen_read_test_ts(
                        table_name,
                        &gen_fi_ft_opt_vec_create_filter_not_empty_or_vec_fi_default_ts,
                        &gen_ident_create_content_el_ts,
                        &|el: &SynFieldWrapper| {
                            let fi = &el.ident;
                            gen_if_let_some_ts(
                                &quote! {v_0b85c066},
                                &{
                                    let ft_ts = gen_as_pg_type_test_cases_path_ts(&el.type0);
                                    quote! {#ft_ts #method_ts(
                                        read_only_ids_returned_from_create_one.#fi.clone().expect("df01c8ac"),
                                        ident_create.#fi.clone()
                                    )}
                                },
                                &{
                                    let ts = gen_read_only_ids_merged_with_create_into_where_assert_eq_ts(
                                        &gen_ts_ccbfdac5(
                                            fi,
                                            &quote! {match el_feacc53b {
                                                #import_ts SingleOrMultiple::Multiple(multiple) => multiple.into_vec().into_iter().collect(),
                                                #import_ts SingleOrMultiple::Single(single) => std::iter::once(single).collect(),
                                            }},
                                        ),
                                    );
                                    quote! {
                                        for el_feacc53b in v_0b85c066.into_vec() {
                                            #ts
                                        }
                                    }
                                },
                            )
                        },
                    )
                };
                (
                    gen_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereGreaterThanSc
                    ),
                    gen_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereBetweenSc
                    ),
                    gen_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereInSc
                    ),
                    gen_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereRegexSc
                    ),
                    gen_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereContainsElGreaterThanSc
                    ),
                    gen_ts(
                        table_read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_name,
                        &ReadOnlyIdsMergedWithCreateIntoPgJsonTypeOptVecWhereContainsElRegexSc
                    )
                )
            };
            quote! {
                #test_read_many_by_non_existent_pks_ts
                #test_read_many_by_equal_to_created_pks_ts
                #read_only_ids_merged_with_create_into_where_equal_ts
                #read_only_ids_merged_with_create_into_vec_where_equal_using_fields_ts
                #read_only_ids_merged_with_create_into_opt_vec_where_equal_to_json_field_ts
                #create_into_pg_type_opt_vec_where_dim_one_equal_ts
                #read_only_ids_merged_with_table_type_into_pg_type_opt_where_greater_than_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_one_equal_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_two_equal_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_three_equal_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_dim_four_equal_ts
                #create_into_pg_json_type_opt_vec_where_length_equal_ts
                #create_into_pg_json_type_opt_vec_where_length_greater_than_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_greater_than_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_between_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_in_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_regex_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_greater_than_ts
                #read_only_ids_merged_with_create_into_pg_json_type_opt_vec_where_contains_el_regex_ts
            }
        };
        let read_one_tests_ts = quote! {
            acc_9189f86e.push({
                let table_read_one_cloned = table_read_one.clone();
                let url_cloned = url.clone();
                let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                futures::FutureExt::boxed(async move {
                    gen_check_no_rows_returned_from_ident_try_read_one_handle_pk(
                        &url_cloned,
                        #pk_ft_as_pg_type_read_ts::new(uuid::Uuid::new_v4()),
                        #select_default_all_with_max_page_size_cloned_clone_ts,
                        &table_read_one_cloned,
                    ).await;
                })
            });
        };
        let gen_ident_read_init_ts = |ts: &dyn ToTokens| {
            let ts0 =
                gen_v_init_ts0(&pk_ft_read_only_is_into_read_read_only_ids_el_pk_fi_ts_937c5af3);
            quote! {#ident_read_ucc {
                #pk_fi: Some(#ts0),
                #ts
            }}
        };
        let gen_read_inner_into_update_ts =
            |fi: &dyn ToTokens, ft: &dyn ToTokens, ft_ts: &dyn ToTokens, i_ts: &dyn ToTokens| {
                let ts = gen_as_pg_type_test_cases_path_ts(&ft);
                quote! {
                    let update = #ts read_inner_into_update_with_new_or_try_new_unwraped({
                        let mut i_e0d2f9db: usize = 0;
                        let mut opt_test_case = None;
                        for el_3a9a65ee in #ft_ts read_only_ids_to_two_dims_vec_read_inner(
                            &read_only_ids_el_937c5af3.#fi.clone().expect("c4d98a71")
                        ) {
                            let mut should_break = false;
                            for el_bb734c11 in el_3a9a65ee {
                                if i_e0d2f9db == #i_ts {
                                    opt_test_case = Some(el_bb734c11);
                                    should_break = true;
                                    break;
                                }
                                i_e0d2f9db = i_e0d2f9db.checked_add(1).expect("326274d1");
                            }
                            if should_break {
                                break;
                            }
                        }
                        opt_test_case.expect("bd79056e")
                    });
                }
            };
        let update_many_tests_ts = {
            //todo add Test for trying to update empty vec
            let update_many_only_one_column_tests_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let ft_ts = gen_as_pg_type_test_cases_path_ts(ft);
                    let is_fields_without_pk_len_greater_than_one = fields_without_pk.len() > 1;
                    let mb_previous_read_ts = if is_fields_without_pk_len_greater_than_one {
                        let ts = gen_pk_where_equal_into_inner_ts(
                            &pk_ft_read_only_is_into_read_read_only_ids_el_pk_fi_ts_937c5af3,
                        );
                        quote! {
                            let previous_read = itertools::Itertools::sorted_by(
                                gen_try_read_many_order_by_pk_with_big_pagination(
                                    &url_cloned,
                                    gen_ident_where_many_pripery_k_others_none(
                                        Some(
                                            gen_pg_type_where_try_new_pk(
                                                #operator_or_ts,
                                                vec![#ts]
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
                                    match (&first.#pk_fi, &second.#pk_fi) {
                                        (Some(first_handle), Some(second_handle)) => first_handle.#VSc.cmp(&second_handle.#VSc),
                                        _ => panic!("99ba9dc3"),
                                    }
                                }
                            );
                        }
                    } else {
                        Ts2::new()
                    };
                    let fi_read_only_ids_to_two_dims_vec_read_inner_acc_sc =
                        SelfReadOnlyIdsToTwoDimsVecReadInnerAccSc::from_tokens(&fi);
                    let ident_read_only_ids_upper_fields_init_without_pk_ts =
                        gen_fields_named_without_pk_with_comma_ts(
                            &|syn_field_wrapper: &SynFieldWrapper| {
                                let fi0 = &syn_field_wrapper.ident;
                                let ft0 = &syn_field_wrapper.type0;
                                let ts = if fi == fi0 {
                                    let ts0 = gen_as_pg_type_test_cases_path_ts(&ft0);
                                    quote! {Some(#ts0 update_to_read_only_ids(&update))}
                                } else {
                                    quote! {None}
                                };
                                quote! {#fi0: #ts}
                            },
                        );
                    let ident_update_params_init_without_pk_ts =
                        gen_fields_named_without_pk_with_comma_ts(
                            &|syn_field_wrapper: &SynFieldWrapper| {
                                let fi0 = &syn_field_wrapper.ident;
                                if fi == fi0 {
                                    let ts = gen_v_init_ts0(&quote! {#UpdateSc.clone()});
                                    quote! {Some(#ts)}
                                } else {
                                    none_ts.clone()
                                }
                            },
                        );
                    let ident_read_fields_init_without_pk_after_update_one_ts =
                        gen_fields_named_without_pk_with_comma_ts(
                            &|syn_field_wrapper: &SynFieldWrapper| {
                                let fi0 = &syn_field_wrapper.ident;
                                let ts = if fi == fi0 {
                                    let ts0 = gen_v_init_ts0(&{
                                        let ts1 = gen_as_pg_type_test_cases_path_ts(
                                            &syn_field_wrapper.type0,
                                        );
                                        quote! {
                                            #ts1 previous_read_merged_with_opt_update_into_read(
                                                #ts1 read_only_ids_to_opt_v_read_default_opt_some_vec_one_el(
                                                    &read_only_ids_el_937c5af3.#fi0.clone().expect("96213542")
                                                ).expect("bf0d6f55").#VSc,
                                                Some(#UpdateSc.clone())
                                            )
                                        }
                                    });
                                    quote! {Some(#ts0)}
                                } else {
                                    quote! {el_a6bc6b2f.#fi0}
                                };
                                quote::quote! {#fi0: #ts}
                            },
                        );
                    let expected_read_many_ts = {
                        let ts = gen_ident_read_init_ts(
                            &ident_read_fields_init_without_pk_after_update_one_ts,
                        );
                        if is_fields_without_pk_len_greater_than_one {
                            quote! {previous_read.into_iter().map(|el_a6bc6b2f|#ts).collect::<Vec<#ident_read_ucc>>()}
                        } else {
                            quote! {vec![#ts]}
                        }
                    };
                    let ts_b13210d0 =
                        gen_read_inner_into_update_ts(&fi, &ft, &ft_ts, &quote! {i_7f181188});
                    let assert_eq_ts_b61aac03 = gen_assert_eq_ts(
                        &quote! {vec![
                            #ident_read_only_ids_ucc {
                                #pk_fi: read_only_ids_el_937c5af3.#pk_fi,
                                #ident_read_only_ids_upper_fields_init_without_pk_ts
                            }
                        ]},
                        &quote! {
                            #ident::try_update_many_handle(
                                &url_cloned,
                                #ident_update_many_params_ucc {
                                    payload: #ident_update_many_payload_ucc::try_new(vec![
                                        #ident_update_ucc::try_new(
                                            #pk_ft_as_pg_type_update_as_pg_type_pk_read_only_ids_into_update_ts,
                                            #ident_update_params_init_without_pk_ts
                                        ).expect("42dc87b3")
                                    ]).expect("69e1bd8a")
                                },
                                &table_update_many_cloned
                            ).await.expect("d2de0bd6")
                        },
                        &quote! {"34bfb3c7"},
                    );
                    let assert_eq_ts_10783673 = gen_assert_eq_ts(
                        &quote! {{#expected_read_many_ts}},
                        &{
                            let ts = gen_pk_where_equal_into_inner_ts(
                                &pk_ft_read_only_is_into_read_read_only_ids_el_pk_fi_ts_937c5af3,
                            );
                            quote! {
                                itertools::Itertools::sorted_by(
                                    gen_try_read_many_order_by_pk_with_big_pagination(
                                        &url_cloned,
                                        gen_ident_where_many_pripery_k_others_none(
                                            Some(
                                                gen_pg_type_where_try_new_pk(
                                                    #operator_or_ts,
                                                    vec![#ts]
                                                )
                                            )
                                        ),
                                        select_default_all_with_max_page_size_cloned,
                                        &table_update_many_cloned
                                    )
                                    .await
                                    .expect("25c561e2")
                                    .into_iter(),
                                    |first, second| match (&first.#pk_fi, &second.#pk_fi) {
                                        (Some(first_handle), Some(second_handle)) => first_handle.#VSc.cmp(&second_handle.#VSc),
                                        _ => panic!("3c827ad6"),
                                    }
                                ).collect::<Vec<#ident_read_ucc>>()
                            }
                        },
                        &quote! {"ae2a2da5"},
                    );
                    let ts_2a6601e1 = gen_acc_push_future_ts(
                        &quote! {table_update_many_cloned},
                        &quote! {table_update_many},
                        &quote! {
                            #mb_previous_read_ts
                            #ts_b13210d0
                            #assert_eq_ts_b61aac03
                            #assert_eq_ts_10783673
                        },
                    );
                    quote! {
                        for (i_7f181188, read_only_ids_el_937c5af3) in gen_read_only_ids_els_8a1ef027(
                            &url,
                            &table_update_many,
                            #select_default_all_with_max_page_size_clone_ts,
                            #fi_read_only_ids_to_two_dims_vec_read_inner_acc_sc.clone()
                        ).await.into_iter().enumerate() {
                            #ts_2a6601e1
                        }
                    }
                },
            );
            quote! {#update_many_only_one_column_tests_ts}
        };
        let update_one_tests_ts = {
            let update_one_only_one_column_tests_ts = gen_fields_named_without_pk_without_comma_ts(
                &|el: &SynFieldWrapper| {
                    let fi = &el.ident;
                    let ft = &el.type0;
                    let ft_ts = gen_as_pg_type_test_cases_path_ts(ft);
                    let mb_previous_read_ts = if fields_without_pk.len() > 1 {
                        quote! {
                            let previous_read = gen_ident_try_read_one_handle_pk(
                                &url_cloned,
                                #pk_ft_read_only_is_into_read_read_only_ids_el_pk_fi_ts_937c5af3,
                                #select_default_all_with_max_page_size_cloned_clone_ts,
                                &table_update_one_cloned
                            )
                            .await.expect("e6998b47");
                        }
                    } else {
                        Ts2::new()
                    };
                    let fi_read_only_ids_to_two_dims_vec_read_inner_acc_sc =
                        SelfReadOnlyIdsToTwoDimsVecReadInnerAccSc::from_tokens(&fi);
                    let ident_read_only_ids_upper_fields_init_without_pk_ts =
                        gen_fields_named_without_pk_with_comma_ts(&|el0: &SynFieldWrapper| {
                            let fi0 = &el0.ident;
                            let ts = if fi == fi0 {
                                let ts0 = gen_as_pg_type_test_cases_path_ts(&el0.type0);
                                quote! {Some(#ts0 update_to_read_only_ids(&update))}
                            } else {
                                quote! {None}
                            };
                            quote! {#fi0: #ts}
                        });
                    let ident_update_params_init_without_pk_ts =
                        gen_fields_named_without_pk_with_comma_ts(&|el0: &SynFieldWrapper| {
                            let fi0 = &el0.ident;
                            if fi == fi0 {
                                let ts = gen_v_init_ts0(&quote! {#UpdateSc.clone()});
                                quote! {Some(#ts)}
                            } else {
                                none_ts.clone()
                            }
                        });
                    let ident_read_fields_init_without_pk_after_update_one_ts =
                        gen_fields_named_without_pk_with_comma_ts(&|el0: &SynFieldWrapper| {
                            let fi0 = &el0.ident;
                            let ts = if fi == fi0 {
                                let ts_0ec756e2 = gen_v_init_ts0(&{
                                    let ts0 = gen_as_pg_type_test_cases_path_ts(&el0.type0);
                                    quote! {
                                        #ts0 previous_read_merged_with_opt_update_into_read(
                                            #ts0 read_only_ids_to_opt_v_read_default_opt_some_vec_one_el(
                                                &read_only_ids_el_937c5af3.#fi0.clone().expect("4f19d0d2")
                                            ).expect("c7685b19").#VSc,
                                            Some(#UpdateSc.clone())
                                        )
                                    }
                                });
                                quote! {Some(#ts_0ec756e2)}
                            } else {
                                quote! {previous_read.#fi0}
                            };
                            quote! {#fi0: #ts}
                        });
                    let ts_a903994d =
                        gen_read_inner_into_update_ts(&fi, &ft, &ft_ts, &quote! {i_26824592});
                    let assert_eq_ts_0ba29fe7 = gen_assert_eq_ts(
                        &quote! {#ident_read_only_ids_ucc {
                            #pk_fi: read_only_ids_el_937c5af3.#pk_fi,
                            #ident_read_only_ids_upper_fields_init_without_pk_ts
                        }},
                        &quote! {
                            #ident::try_update_one_handle(
                                &url_cloned,
                                #ident_update_one_params_ucc {
                                    payload: #ident_update_ucc::try_new(
                                        #pk_ft_as_pg_type_update_as_pg_type_pk_read_only_ids_into_update_ts,
                                        #ident_update_params_init_without_pk_ts
                                    ).expect("0e5d65a5")//todo add column ident
                                },
                                &table_update_one_cloned
                            ).await.expect("4d755542")
                        },
                        &quote! {"564de31c"},
                    );
                    let assert_eq_ts_35a86616 = gen_assert_eq_ts(
                        &gen_ident_read_init_ts(
                            &ident_read_fields_init_without_pk_after_update_one_ts,
                        ),
                        &quote! {
                            gen_ident_try_read_one_handle_pk(
                                &url_cloned,
                                #pk_ft_read_only_is_into_read_read_only_ids_el_pk_fi_ts_937c5af3,
                                select_default_all_with_max_page_size_cloned,
                                &table_update_one_cloned
                            )
                            .await.expect("75894c76")
                        },
                        &quote! {"d5dec823"},
                    );
                    let ts_fedea8c3 = gen_acc_push_future_ts(
                        &quote! {table_update_one_cloned},
                        &quote! {table_update_one},
                        &quote! {
                            #mb_previous_read_ts
                            #ts_a903994d
                            #assert_eq_ts_0ba29fe7
                            #assert_eq_ts_35a86616
                        },
                    );
                    quote! {
                        for (i_26824592, read_only_ids_el_937c5af3) in gen_read_only_ids_els_8a1ef027(
                            &url,
                            &table_update_one,
                            #select_default_all_with_max_page_size_clone_ts,
                            #fi_read_only_ids_to_two_dims_vec_read_inner_acc_sc
                        ).await.into_iter().enumerate() {
                            #ts_fedea8c3
                        }
                    }
                },
            );
            quote! {#update_one_only_one_column_tests_ts}
        };
        let delete_many_tests_ts = {
            let test_delete_many_by_non_existent_pks_ts = gen_for_in_1_2_ts(
                &quote! {el_39819198},
                &gen_acc_push_future_ts(
                    &quote!{table_7e35b1ce},
                    &quote!{table_test_read_many_by_equal_to_created_pks},
                    &add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&gen_assert_ts(
                        &{
                            let ts = gen_try_delete_many_handle_ts(
                                &quote!{
                                    #pk_fi: Some(
                                        gen_pg_type_where_try_new_pk(
                                            #operator_or_ts,
                                            std::iter::repeat_with(|| #pk_where_equal_uuid_new_v_ts)
                                            .take(el_39819198)
                                            .collect()
                                        )
                                    ),
                                    #fields_none_init_ts
                                },
                                &quote!{table_7e35b1ce}
                            );
                            quote!{#ts.is_empty()}
                        },
                        &quote!{"51d14103"}
                    ))
                )
            );
            let test_delete_many_by_pks_ts = gen_for_in_1_2_ts(&quote! {el_56409d32}, &{
                let ts_3240261f = gen_acc_push_future_ts(
                    &quote!{table_7e35b1ce},
                    &quote!{table_test_read_many_by_equal_to_created_pks},//todo is table name correct?
                    &add_create_one_default_and_delete_after_just_to_add_some_data_to_be_sure_it_will_not_return_from_the_test_query_ts(&{
                        let pk_ft_read_only_ids_into_table_type_el_pk_fi_clone_ts =
                            gen_pk_ft_as_pg_type_pk_method_call_ts(
                                &ReadOnlyIdsIntoTableTypeSc,
                                &quote! {el_3bb88958.#pk_fi},
                            );
                        let assert_eq_ts_ea7edbc4 = gen_assert_eq_ts(
                            &quote!{read_only_ids_from_try_delete_many},
                            &quote!{{
                                read_only_ids_from_try_create_many.iter().map(|el_ba0f6b1c|
                                    #pk_as_pg_type_test_cases_path_ts read_only_ids_to_opt_v_read_default_opt_some_vec_one_el(
                                        &el_ba0f6b1c.#pk_fi
                                    ).expect("3ee5ee86").#VSc
                                ).collect::<Vec<#pk_ft_as_pg_type_read_ts>>()
                            }},
                            &quote!{"db5e88a6"}
                        );
                        let assert_ts_d6ec39a3 = gen_assert_ts(
                            &{
                                let ts = gen_pk_where_equal_ts(&pk_ft_read_into_table_type_el_pk_fi_clone_ts);
                                quote!{
                                    gen_try_read_many_order_by_pk_with_big_pagination(
                                        &url_cloned,
                                        gen_ident_where_many_pripery_k_others_none(
                                            Some(
                                                gen_pg_type_where_try_new_pk(
                                                    #operator_or_ts,
                                                    read_only_ids_from_try_delete_many.into_iter().map(|el_adcc8db3| #ts).collect()
                                                )
                                            )
                                        ),
                                        select_default_all_with_max_page_size_cloned.clone(),
                                        &table_7e35b1ce
                                    ).await
                                    .expect("bcb79917")
                                    .is_empty()
                                }
                            },
                            &quote!{"77f038b0"}
                        );
                        let ts_212f8aca = gen_pk_where_equal_ts(&pk_ft_read_only_ids_into_table_type_el_pk_fi_clone_ts);
                        let ts_6f76ccd4 = gen_read_only_ids_from_try_delete_many_ts(&gen_try_delete_many_handle_ts(
                            &quote!{
                                #pk_fi: Some(
                                    gen_pg_type_where_try_new_pk(
                                        #operator_or_ts,
                                        read_only_ids_from_try_create_many.iter().map(|el_3bb88958| #ts_212f8aca).collect()
                                    )
                                ),
                                #fields_none_init_ts
                            },
                            &quote!{table_7e35b1ce}
                        ));
                        quote! {
                            let read_only_ids_from_try_create_many = #ident::try_create_many_handle(
                                &url_cloned,
                                #ident_create_many_params_ucc {
                                    payload: #ident_create_many_payload_ucc(
                                        std::iter::repeat_n(ident_create_default_cloned, el_56409d32).collect()
                                    )
                                },
                                &table_7e35b1ce
                            ).await.expect("b8695890");
                            #ts_6f76ccd4
                            #assert_eq_ts_ea7edbc4
                            #assert_ts_d6ec39a3
                        }
                    })
                );
                quote! {
                    let ident_create_default_cloned = ident_create_default.clone();
                    #ts_3240261f
                }
            });
            quote! {
                #test_delete_many_by_non_existent_pks_ts
                #test_delete_many_by_pks_ts
            }
        };
        let delete_one_tests_ts = {
            let ts = gen_v_init_ts0(
                &pk_ft_read_only_ids_into_read_read_only_ids_returned_from_create_one_pk_fi_ts,
            );
            let assert_eq_ts_6322435c = gen_assert_eq_ts(
                &quote! {#ident_read_ucc {
                    #pk_fi: Some(#ts),
                    #fi_read_only_ids_merged_with_create_into_opt_v_read_read_only_ids_returned_from_create_one_create_ts
                }},
                &quote! {
                    gen_ident_try_read_one_handle_pk(
                        &url,
                        #pk_ft_read_only_ids_into_read_read_only_ids_returned_from_create_one_pk_fi_ts,
                        #select_default_all_with_max_page_size_cloned_clone_ts,
                        &table_delete_one_cloned
                    )
                    .await.expect("c8c44c89")
                },
                &quote! {"86ef08ae"},
            );
            let assert_eq_ts_8812d778 = gen_assert_eq_ts(
                &quote! {
                    gen_try_delete_one_handle(
                        &url,
                        #pk_ft_read_only_ids_into_read_read_only_ids_returned_from_create_one_pk_fi_ts,
                        &table_delete_one_cloned
                    ).await.expect("7e1d1a70")
                },
                &quote! {#pk_ft_read_only_ids_into_read_read_only_ids_returned_from_create_one_pk_fi_ts},
                &quote! {"99f81971"},
            );
            let assert_ts_9c8cb81a = gen_assert_ts(
                &quote! {pg == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row()},
                &quote! {"c9261bb8"},
            );
            quote! {
                acc_9189f86e.push({
                    let table_delete_one_cloned = table_delete_one.clone();
                    let select_default_all_with_max_page_size_cloned = #select_default_all_with_max_page_size_clone_ts;
                    futures::FutureExt::boxed(async move {
                        if let Err(#ErSc) = gen_try_delete_one_handle(
                            &url,
                            #pk_ft_as_pg_type_read_ts::new(uuid::Uuid::new_v4()),
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
                                    #assert_ts_9c8cb81a
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
                        #assert_eq_ts_6322435c
                        #assert_eq_ts_8812d778
                        gen_check_no_rows_returned_from_ident_try_read_one_handle_pk(
                            &url,
                            #pk_ft_read_only_ids_into_read_read_only_ids_returned_from_create_one_pk_fi_ts,
                            #select_default_all_with_max_page_size_cloned_clone_ts,
                            &table_delete_one_cloned,
                        ).await;
                    })
                });
            }
        };
        let assert_ts_48ca54b1 = gen_assert_ts(&quote! {v.len() <= 63}, &quote! {"77f9bfb7"});
        let ts_e698d734 = gen_pk_where_equal_into_inner_ts(&quote! {
            #pk_ft_as_pg_type_pk_ts read_only_ids_into_read(el_9530b728.#pk_fi)
        });
        let size_of_ts = {
            let ts = gen_assert_eq_ts(
                &quote! {std::mem::size_of::<#ident>()},
                &quote! {0},
                &quote! {"e8eed4b3"},
            );
            quote! {
                #[test]
                fn size_of() {
                    #ts
                }
            }
        };
        let gen_ident_where_many_pripery_k_others_none_fn_ts = quote! {
            fn gen_ident_where_many_pripery_k_others_none(
                opt_pg_type_where: Option<#import_ts PgTypeWhere<#pk_ft_as_pg_type_where_ts>>,
            ) -> #ident_where_many_ucc {
                #ident_where_many_ucc::try_new(
                    opt_pg_type_where,
                    #fields_named_without_pk_with_comma_none_ts
                )
                .expect("5fb2b219")
            }
        };
        let gen_pg_type_where_try_new_pk_fn_ts = quote! {
            fn gen_pg_type_where_try_new_pk(
                operator: #import_ts Operator,
                vec: Vec<#pk_ft_as_pg_type_where_ts>
            ) -> #import_ts PgTypeWhere<#pk_ft_as_pg_type_where_ts> {
                #gen_pg_type_where_try_new_pk_ts
            }
        };
        let gen_pg_type_where_try_new_or_pks_fn_ts = quote! {
            fn gen_pg_type_where_try_new_or_pks(
                vec_read_only_ids: &[#ident_read_only_ids_ucc]
            ) -> #import_ts PgTypeWhere<#pk_ft_as_pg_type_where_ts> {
                gen_pg_type_where_try_new_pk(
                    #operator_or_ts,
                    vec_read_only_ids.iter().map(|el_9530b728| #ts_e698d734).collect()
                )
            }
        };
        let gen_try_read_many_order_by_pk_with_big_pagination_fn_ts = quote! {
            async fn gen_try_read_many_order_by_pk_with_big_pagination(
                endpoint_location: &str,
                ident_where_many_6b1fab92: #ident_where_many_ucc,
                select: #import_ts NotEmptyUniqueVec<#ident_select_ucc>,
                table: &str
            ) -> Result<Vec<#ident_read_ucc>, #ident_try_read_many_er_ucc> {
                #ident::try_read_many_handle(
                    endpoint_location,
                    #ident_read_many_params_ucc {
                        payload: #ident_read_many_payload_ucc {
                            where_many: #opt_ident_where_many_ucc(Some(
                                ident_where_many_6b1fab92
                            )),
                            select,
                            order_by: #import_ts OrderBy {
                                column: #ident_select_ucc::#pk_fi_ucc_ts(
                                    #pk_ft_as_pg_type_select_ts::default()
                                ),
                                order: Some(#import_ts Order::Asc)
                            },
                            pagination: #import_ts PaginationStartsWithZero::try_new(10000, 0).expect("b0cdf0cb"),
                        }
                    },
                    table
                )
                .await
            }
        };
        let gen_ident_try_read_one_handle_pk_fn_ts = quote! {
            async fn gen_ident_try_read_one_handle_pk(
                url: &str,
                pk_column: #pk_ft_as_pg_type_read_ts,
                select: #import_ts NotEmptyUniqueVec<#ident_select_ucc>,
                table: &str,
            ) -> Result<#ident_read_ucc, #ident_try_read_one_er_ucc> {
                #ident::try_read_one_handle(
                    url,
                    #ident_read_one_params_ucc {
                        payload: #ident_read_one_payload_ucc {
                            pk_column,
                            select,
                        },
                    },
                    table,
                )
                .await
            }
        };
        let gen_check_no_rows_returned_from_ident_try_read_one_handle_pk_fn_ts = {
            let ts = gen_assert_ts(
                &quote! {pg == no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row()},
                &quote! {"58b9a6a4"},
            );
            quote! {
                async fn gen_check_no_rows_returned_from_ident_try_read_one_handle_pk(
                    url: &str,
                    pk_column: #pk_ft_as_pg_type_read_ts,
                    select: #import_ts NotEmptyUniqueVec<#ident_select_ucc>,
                    table: &str,
                ) {
                    if let Err(#ErSc) = gen_ident_try_read_one_handle_pk(
                        url,
                        pk_column,
                        select,
                        table
                    ).await {
                        if let #ident_try_read_one_er_ucc::#ident_read_one_er_with_serde_ucc {
                            read_one_er_with_serde,
                            ..
                        } = er {
                            if let #ident_read_one_er_with_serde_ucc::Pg { pg, .. } = read_one_er_with_serde {
                                #ts
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
            }
        };
        let ident_create_default_fn_ts = quote! {
            fn ident_create_default() -> #ident_create_ucc {
                #ident_create_ucc {
                    #ident_create_default_fields_init_without_pk_ts
                }
            }
        };
        let gen_read_only_ids_from_try_create_one_fn_ts = quote! {
            async fn gen_read_only_ids_from_try_create_one(
                #UrlSc: &str,
                #PayloadSc: #ident_create_ucc,
                table: &str,
            ) -> #ident_read_only_ids_ucc {
                #ident::try_create_one_handle(
                    #UrlSc,
                    #ident_create_one_params_ucc {
                        #PayloadSc
                    },
                    table
                ).await.expect("32e30b87")
            }
        };
        let gen_read_only_ids_from_try_create_one_default_fn_ts = quote! {
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
        };
        let gen_try_delete_one_handle_fn_ts = quote! {
            async fn gen_try_delete_one_handle(
                #UrlSc: &str,
                #pk_fi: #pk_ft_as_pg_type_read_ts,
                table: &str,
            ) -> Result<#pk_ft_as_pg_type_read_ts, #ident_try_delete_one_er_ucc> {
                #ident::try_delete_one_handle(
                    #UrlSc,
                    #ident_delete_one_params_ucc {
                        payload: #ident_delete_one_payload_ucc {
                            #pk_fi
                        }
                    },
                    table
                ).await
            }
        };
        let no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row_fn_ts = quote! {
            fn no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row() -> &'static str {
                "no rows returned by a query that expected to return at least one row"
            }
        };
        let gen_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create_fn_ts = {
            let ts = gen_assert_eq_ts(
                &quote! {read_only_ids_from_try_create_many.len()},
                &quote! {ident_vec_create.len()},
                &quote! {"88fb286c"},
            );
            quote! {
                fn gen_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create(
                    read_only_ids_from_try_create_many: Vec<#ident_read_only_ids_ucc>,
                    ident_vec_create: Vec<#ident_create_ucc>
                ) -> Vec<#ident_read_ucc> {
                    let mut acc_1debe8fb = Vec::new();
                    #ts
                    for (read_only_ids, create) in read_only_ids_from_try_create_many.into_iter().zip(ident_vec_create) {
                        acc_1debe8fb.push(#ident_read_ucc {
                            #pk_fi: #pk_as_pg_type_test_cases_path_ts read_only_ids_to_opt_v_read_default_opt_some_vec_one_el(
                                &read_only_ids.#pk_fi
                            ),
                            #fi_read_only_ids_merged_with_create_into_opt_v_read_read_only_ids_and_create_ts
                        });
                    }
                    acc_1debe8fb.sort_by(|first, second| {
                        if let (Some(first_handle), Some(second_handle)) = (&first.#pk_fi, &second.#pk_fi) {
                            first_handle.#VSc.cmp(&second_handle.#VSc)
                        } else {
                            panic!("d760ffa3");
                        }
                    });
                    acc_1debe8fb
                }
            }
        };
        quote! {
            #[cfg(test)]
            mod #ident_tests_sc {
                use super::*;
                #size_of_ts
                #[test]
                fn crud() {
                    #gen_ident_where_many_pripery_k_others_none_fn_ts
                    #gen_pg_type_where_try_new_pk_fn_ts
                    #gen_pg_type_where_try_new_or_pks_fn_ts
                    #gen_try_read_many_order_by_pk_with_big_pagination_fn_ts
                    #gen_ident_try_read_one_handle_pk_fn_ts
                    #gen_check_no_rows_returned_from_ident_try_read_one_handle_pk_fn_ts
                    #ident_create_default_fn_ts
                    #gen_read_only_ids_from_try_create_one_fn_ts
                    #gen_read_only_ids_from_try_create_one_default_fn_ts
                    #gen_try_delete_one_handle_fn_ts
                    #no_rows_returned_by_a_query_that_expected_to_return_at_least_one_row_fn_ts
                    #gen_vec_ident_read_from_vec_ident_read_only_ids_with_vec_ident_create_fn_ts
                    #gen_read_only_ids_els_ts_fe29ff70
                    tracing_subscriber::fmt::init();
                    tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build().expect("38823c21").block_on(async {
                        //todo mb refactor
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
                            let v = format!("{table}_{postfix}");
                            #assert_ts_48ca54b1
                            v
                        };
                        let table_init = add_table_postfix("init");
                        let table_create_many = add_table_postfix("create_many");
                        let table_create_one = add_table_postfix("create_one");
                        let table_test_read_many_by_non_existent_pks = add_table_postfix("Test_read_many_by_non_existent_pks");
                        let table_test_read_many_by_equal_to_created_pks = add_table_postfix("Test_read_many_by_equal_to_created_pks");
                        #(#table_fis_init_vec_ts)*
                        let table_read_one = add_table_postfix("read_one");
                        let table_update_many = add_table_postfix("update_many");
                        let table_update_one = add_table_postfix("update_one");
                        let table_delete_many = add_table_postfix("delete_many");
                        let table_delete_one = add_table_postfix("delete_one");
                        let table_names = [
                            &table_init,
                            &table_create_many,
                            &table_create_one,
                            &table_test_read_many_by_non_existent_pks,
                            &table_test_read_many_by_equal_to_created_pks,
                            #(#table_test_name_fis_vec_ts)*
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
                        //do not make it concurrent. would be pg er: "duplicate k v violates unique constraint \"pg_class_relname_nsp_index\""
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
                        let #SelectPkSc = #import_ts NotEmptyUniqueVec::try_new(vec![
                            #ident_select_ucc::#pk_fi_ucc_ts(
                                #pk_ft_as_pg_type_select_ts::default(),
                            )
                        ])
                        .expect("0776170e");
                        let #IdentCreateDefaultSc = ident_create_default();
                        #select_default_all_with_max_page_size_not_empty_unique_vec_ts
                        #common_read_only_ids_returned_from_create_one_ts
                        #read_only_ids_to_two_dims_vec_read_inner_acc_fields_ts
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
    mb_write_ts_into_file(
        gen_pg_table_config.tests_write_into_file,
        "gen_pg_table_Tests",
        &ident_tests_ts,
        &FormatWithCargofmt::True,
    );
    let common_ts = quote! {
        #impl_ident_ts
        #ident_create_ts
        #ident_where_many_ts
        #opt_ident_where_many_ts
        #select_ts
        #ident_read_ts
        #ident_read_only_ids_ts
        // #ident_column_read_permission_ts
        #ident_update_ts
        #ident_update_for_query_ts
    };
    mb_write_ts_into_file(
        gen_pg_table_config.common_write_into_file,
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
    mb_write_ts_into_file(
        gen_pg_table_config.whole_write_into_file,
        "gen_pg_table",
        &gend,
        &FormatWithCargofmt::True,
    );
    gend
}
