use enum_extension_lib::EnumExtension;
use gen_quotes::dq_ts;
use macros_helpers::{
    DSerdeDeserialize, DTsBuilder, FormatWithCargofmt, ShouldWriteTsIntoFile, SynField,
    gen_if_write_is_err_curly_braces_ts, gen_if_write_is_err_ts, gen_impl_display_ts,
    gen_impl_from_ts, gen_impl_pub_const_new_for_ident_ts, gen_impl_pub_new_for_ident_ts,
    gen_impl_to_err_string_ts, gen_pub_const_new_ts, gen_pub_new_ts, gen_pub_try_new_ts,
    gen_pub_type_al_ts, gen_simple_syn_punct, get_macro_attr_meta_list_ts, mb_write_ts_into_file,
};
use naming::{
    AddOprtrSc, AllFieldsAreNoneUcc, ArrOfUcc, AsRefStrToUccTs, AsUcc, ColumnFieldSc, ColumnSc,
    ContainsAllElsOfArrUcc, CrIntoPgJsonOptVecWhereLengthEqualSc,
    CrIntoPgJsonOptVecWhereLengthGreaterThanSc, CrSc, CrUpdDelAreEmptyUcc, DelSc,
    DfltOptSomeVecOneElSc, DfltOptSomeVecOneElUcc, DimOneEqualUcc, DimOneInUcc,
    DisplayPlusToTokens, EqualUcc, ErSc, FieldsSc, GenJsonbSetTargetSc, IdSc, IdsAreNotUniqueUcc,
    InUcc, IncrSc, JsonbObjUcc, JsonbSetAccumulatorSc, JsonbSetPathSc, JsonbSetTargetSc,
    LengthEqualUcc, LengthGreaterThanUcc, NotUniqueIdInJsonDelArrUcc,
    NotUniqueIdInJsonUpdAndDelArrsUcc, OptUpdSc, OptVecCrSc, OverlapsWithArrUcc,
    PgJsonTestCasesUcc, PgJsonUcc, PgTypeTestCasesUcc, PgTypeUcc, PreviousReadAndOptUpdIntoReadSc,
    QpErUcc, QpSc, QuerySc, ReadIdsAndCrIntoOptVReadSc, ReadIdsAndCrIntoPgJsonOptVecWhereBetweenSc,
    ReadIdsAndCrIntoPgJsonOptVecWhereContainsElGreaterThanSc,
    ReadIdsAndCrIntoPgJsonOptVecWhereContainsElRegexSc,
    ReadIdsAndCrIntoPgJsonOptVecWhereGreaterThanSc, ReadIdsAndCrIntoPgJsonOptVecWhereInSc,
    ReadIdsAndCrIntoPgJsonOptVecWhereRegexSc, ReadIdsAndCrIntoReadSc, ReadIdsAndCrIntoTableTypeSc,
    ReadIdsAndCrIntoVecWhereEqualToJsonFieldSc, ReadIdsAndCrIntoVecWhereEqualUsingFieldsSc,
    ReadIdsAndCrIntoWhereEqualSc, ReadIdsIntoOptVReadInnSc, ReadIdsSc, ReadIdsTo2DimsVecReadInnSc,
    ReadIdsToOptVReadDfltOptSomeVecOneElSc, ReadInnIntoReadWithNewOrTryNewUnwrapedSc,
    ReadInnIntoUpdWithNewOrTryNewUnwrapedSc, ReadSc, SelectOnlyCrdIdsQbSc, SelectOnlyCrdIdsQpSc,
    SelectOnlyIdsQpSc, SelectOnlyUpddIdsQbSc, SelectOnlyUpddIdsQpSc, SelectQpPgTypeSc, SelectQpSc,
    SelfSc, SelfUcc, StdOptOptObjAccSc, ToTokensToUccTs, UpdQbSc, UpdQpSc, UpdSc, UpdToReadIdsSc,
    UuidUuidAsNotNullJsonbStringUcc, VSc, ValueSc, VecOfUcc, WithIdUcc,
    param::{
        ElSelfUcc, SelfCrForQueryUcc, SelfCrUcc, SelfCurrentSc, SelfGenPgJsonObjModSc, SelfLastSc,
        SelfReadIdsHUcc, SelfReadIdsUcc, SelfReadInnUcc, SelfReadTryFromErUcc, SelfReadUcc,
        SelfSelectElUcc, SelfSelectSc, SelfSelectUcc, SelfTableTypeUcc, SelfUpdElUcc,
        SelfUpdForQueryElUcc, SelfUpdForQueryUcc, SelfUpdTryNewErUcc, SelfUpdUcc, SelfWhereUcc,
    },
};
use optml::Optml;
use panic_location::panic_location;
use pg_crud_macros_common::{
    AddOprtrUndrscr, ColumnParamUndrscr, CrQbValueUndrscr, CrQpIncrUndrscr, CrQpValueUndrscr,
    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize, Dim, Import, IncrParamUndrscr, IsCrQbMut,
    IsNullable, IsPkUndrscr, IsQbMut, IsSelectOnlyCrdIdsQbMut, IsSelectOnlyUpddIdsQbMut,
    IsSelectQpColumnFieldForErMessageUsed, IsSelectQpIsPgTypeUsed, IsSelectQpSelfSelectUsed,
    IsUpdQbMut, IsUpdQpJsonbSetTargetUsed, IsUpdQpSelfUpdUsed, PgTypeOrPgJson,
    SelectQpValueUndrscr, UpdQpJsonbSetAccumulatorUndrscr, UpdQpJsonbSetPathUndrscr,
    UpdQpJsonbSetTargetUndrscr, UpdQpValueUndrscr, gen_impl_de_for_struct_ts,
    gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_max_page_size_ts,
    gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_ts,
    gen_impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_ts,
    gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts, gen_impl_pg_json_test_cases_for_ident_ts,
    gen_impl_pg_json_ts, gen_impl_pg_type_not_pk_for_ident_ts,
    gen_impl_pg_type_test_cases_for_ident_ts, gen_impl_pg_type_ts,
    gen_impl_sqlx_decode_sqlx_pg_for_ident_ts, gen_impl_sqlx_encode_sqlx_pg_for_ident_ts,
    gen_impl_sqlx_type_for_ident_ts, gen_jsonb_build_obj, gen_jsonb_build_obj_v,
    gen_match_try_new_in_de_ts, gen_opt_type_dcl_ts,
    gen_read_ids_and_cr_into_vec_where_equal_to_json_field_ts,
    gen_read_ids_and_cr_into_vec_where_equal_using_fields_ts,
    gen_read_ids_and_cr_into_where_equal_ts, gen_return_err_qp_er_write_into_buffer_ts,
    gen_sqlx_types_json_type_dcl_ts, gen_v_dcl_ts, gen_v_init_ts, gen_vec_tokens_dcl_ts,
    impl_pg_type_where_filter_for_ident_ts, mb_wrap_into_braces_ts, wrap_into_scopes_ts,
};
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt::Display;
use std::iter::repeat_with;
use strum_macros::{Display, EnumIter};
use syn::{
    Data, DeriveInput, Field, FieldMutability, Fields, Ident, Path, Type, TypePath, Visibility,
    parse2,
    token::{Colon, Pub},
};
use token_patterns::{
    AllowClippyArbitrarySrcItemOrdering, MustUse, PgCrudDfltOptSomeVecOneElCall,
    PgCrudDfltOptSomeVecOneElMaxPageSizeCall, StringTs,
};
//todo gen authorization rights enum for json fields
//todo bug in upd if updating arr and creating el in jsonb arr without anything - read_ids generation logic of vec returns wrong query part
#[must_use]
pub fn gen_pg_json_obj(input_ts: Ts2) -> Ts2 {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Optml)]
    enum TraitGen {
        PgJson,
        PgTypeAndPgJson,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, PartialEq, Serialize, Deserialize, Display, EnumIter, EnumExtension, Optml,
    )]
    enum Pattern {
        Stdrt,
        Arr,
    }
    #[derive(Debug, PartialEq, Serialize, Deserialize, Optml)]
    struct PgJsonObjectRecord {
        is_nullable: IsNullable,
        pattern: Pattern,
        trait_gen: TraitGen,
    }
    #[derive(Debug, Deserialize, Optml)]
    struct GenPgJsonsConfig {
        pg_table_columns_write_into_pg_table_columns_using_pg_json_objs: ShouldWriteTsIntoFile,
        vrt: PgJsonObjectRecord,
        whole_write_into_gen_pg_json_obj: ShouldWriteTsIntoFile,
    }
    panic_location();
    let di: DeriveInput = parse2(input_ts).expect("e5f0e27b");
    let import = Import::PgCrud;
    let gen_pg_json_obj_config = from_str::<GenPgJsonsConfig>(
        &get_macro_attr_meta_list_ts(
            &di.attrs,
            &format!("{}::pg_json_obj_config", import.sc_str()),
        )
        .to_string(),
    )
    .expect("246de453");
    let (fields_ts, pg_json_obj_arr) = {
        let pg_json_obj_record = gen_pg_json_obj_config.vrt;
        match (&pg_json_obj_record.is_nullable, &pg_json_obj_record.pattern) {
            (IsNullable::False, Pattern::Stdrt) => vec![pg_json_obj_record],
            (IsNullable::True, Pattern::Stdrt) |
            (IsNullable::False, Pattern::Arr) => vec![
                PgJsonObjectRecord {
                    is_nullable: IsNullable::False,
                    pattern: Pattern::Stdrt,
                    trait_gen: pg_json_obj_record.trait_gen.clone(),
                },
                pg_json_obj_record
            ],
            (IsNullable::True, Pattern::Arr) => vec![
                PgJsonObjectRecord {
                    is_nullable: IsNullable::False,
                    pattern: Pattern::Stdrt,
                    trait_gen: pg_json_obj_record.trait_gen.clone(),
                },
                PgJsonObjectRecord {
                    is_nullable: IsNullable::True,
                    pattern: Pattern::Stdrt,
                    trait_gen: pg_json_obj_record.trait_gen.clone(),
                },
                PgJsonObjectRecord {
                    is_nullable: IsNullable::False,
                    pattern: Pattern::Arr,
                    trait_gen: pg_json_obj_record.trait_gen.clone(),
                },
                pg_json_obj_record
            ]
        }
    }
    .into_iter()
    .enumerate()
    .map(|(i, el)| {
        #[derive(Debug, Display, EnumIter, EnumExtension, Optml)]
        enum IsStdrtWithId {
            False,
            True,
        }
        #[allow(clippy::arbitrary_source_item_ordering)]
        enum IdentPattern {
            StdrtNotNullWithoutId,
            StdrtNotNullWithId,
            StdrtNullableWithoutId,
            ArrNotNullWithId,
            ArrNullableWithIdentifier,//Identifier instead of Id - just to fix clippy lint
        }
        #[allow(clippy::arbitrary_source_item_ordering)]
        #[derive(Debug, Clone, Display, Optml)]
        enum PgJsonSubtype {
            TableType,
            Cr,
            CrForQuery,
            Select,
            Where,
            Read,
            ReadIds,
            ReadInn,
            Upd,
            UpdForQuery,
        }
        impl ToTokens for PgJsonSubtype {
            fn to_tokens(&self, tokens: &mut Ts2) {
                self.to_string().parse::<Ts2>().expect("43ac0b62").to_tokens(tokens);
            }
        }
        #[derive(Debug, Clone, Display, Optml)]
        enum PgTypeSubtype {
            // TableType,
            // Cr,
            // Select,
            // Where,
            Read,
            // ReadIds,
            // ReadInn,
            Upd,
        }
        impl ToTokens for PgTypeSubtype {
            fn to_tokens(&self, tokens: &mut Ts2) {
                self.to_string().parse::<Ts2>().expect("5825d4b7").to_tokens(tokens);
            }
        }
        #[allow(clippy::arbitrary_source_item_ordering)]
        enum PgJsonSubtypeTableTypeOrCr {
            TableType,
            Cr,
        }
        impl From<&PgJsonSubtypeTableTypeOrCr> for PgJsonSubtype {
            fn from(v: &PgJsonSubtypeTableTypeOrCr) -> Self {
                match &v {
                    PgJsonSubtypeTableTypeOrCr::TableType => Self::TableType,
                    PgJsonSubtypeTableTypeOrCr::Cr => Self::Cr,
                }
            }
        }
        enum ReadWithOrWithoutAnnOrInn {
            Inn,
            WithSerdeOptIsNoneAnn,
            WithoutSerdeOptIsNoneAnn,
        }
        enum AddSerdeSkipSerializingIfVecIsEmptyAnn {
            False,
            True,
        }
        enum NewTypeOrStructDcl {
            NewType,
            StructDcl,
        }
        let is_nullable = &el.is_nullable;
        let pattern = &el.pattern;
        let trait_gen = &el.trait_gen;
        let self_field_vec_ts = quote! {.0.to_vec()};
        let cfg_feature_test_utils = quote! {#[cfg(feature = "test-utils")]};
        let return_err_qp_er_write_into_buffer_ts = gen_return_err_qp_er_write_into_buffer_ts(import);
        let none_ts = quote!{None};
        let import_qp_er_ts = quote! {#import::#QpErUcc};
        let vec_pg_crud_dflt_opt_some_vec_one_el_call_ts = quote!{vec![#PgCrudDfltOptSomeVecOneElCall]};
        let dflt_but_opt_is_some_ts = quote!{
            #import::#DfltOptSomeVecOneElUcc::#DfltOptSomeVecOneElSc
        };
        let dflt_but_opt_is_some_call_ts = quote!{#dflt_but_opt_is_some_ts()};
        let gen_ident_as_dflt_but_opt_is_some_ts = |ts: &dyn ToTokens|quote!{
            <#ts as #import::#DfltOptSomeVecOneElUcc>::#DfltOptSomeVecOneElSc
        };
        let gen_ident_as_dflt_but_opt_is_some_call_ts = |ts: &dyn ToTokens|{
            let ts0 = gen_ident_as_dflt_but_opt_is_some_ts(ts);
            quote!{#ts0()}
        };
        let gen_v_dcl_ts0 = |ts: &dyn ToTokens|{
            gen_v_dcl_ts(&import, ts)
        };
        let gen_v_init_ts0 = |ts: &dyn ToTokens|{
            gen_v_init_ts(&import, ts)
        };
        let di_ident = &di.ident;
        let vec_syn_field = if let Data::Struct(data_struct) = &di.data {
            if let Fields::Named(fields_named) = &data_struct.fields {
                fields_named.named.iter()
                .collect::<Vec<&Field>>()
                .iter()
                .map(|el0|SynField {
                    vis: el0.vis.clone(),
                    type0: el0.ty.clone(),
                    ident: el0.ident.clone().expect("3ac7f263"),
                })
                .collect::<Vec<SynField>>()
            } else {
                panic!("4c305996");
            }
        } else {
            panic!("a4fc18a1");
        };
        let is_stdrt_with_id_false = IsStdrtWithId::False;
        let is_stdrt_with_id_true = IsStdrtWithId::True;
        let gen_ident_ucc = |ident_pattern: &IdentPattern| {
            let (rust_part, pg_part, is_nullable_325dc715) = {
                let di_ident_str = di_ident.to_string();
                let vec_of_di_ident_with_id = format!("{VecOfUcc}{di_ident}{WithIdUcc}");
                let jsonb_obj_ucc_str = JsonbObjUcc.to_string();
                let arr_of_not_null_jsonb_obj_with_id = format!("{ArrOfUcc}{}{JsonbObjUcc}{WithIdUcc}", IsNullable::False.not_null_or_nullable_str());
                match &ident_pattern {
                    IdentPattern::StdrtNotNullWithoutId => (di_ident_str, jsonb_obj_ucc_str, IsNullable::False),
                    IdentPattern::StdrtNotNullWithId => (format!("{di_ident}{WithIdUcc}"), format!("{JsonbObjUcc}{WithIdUcc}"), IsNullable::False),
                    IdentPattern::StdrtNullableWithoutId => (di_ident_str, jsonb_obj_ucc_str, IsNullable::True),
                    IdentPattern::ArrNotNullWithId => (vec_of_di_ident_with_id, arr_of_not_null_jsonb_obj_with_id, IsNullable::False),
                    IdentPattern::ArrNullableWithIdentifier => (vec_of_di_ident_with_id, arr_of_not_null_jsonb_obj_with_id, IsNullable::True),
                }
            };
            let is_nullable_325dc715_rust = is_nullable_325dc715.rust();
            let not_null_or_nullable_str = is_nullable_325dc715.not_null_or_nullable_str();
            format!("{is_nullable_325dc715_rust}{rust_part}{AsUcc}{not_null_or_nullable_str}{pg_part}").parse::<Ts2>().expect("43784dd3")
        };
        let ident = &gen_ident_ucc(&match (&is_nullable, &pattern) {
            (IsNullable::False, Pattern::Stdrt) => IdentPattern::StdrtNotNullWithoutId,
            (IsNullable::False, Pattern::Arr) => IdentPattern::ArrNotNullWithId,
            (IsNullable::True, Pattern::Stdrt) => IdentPattern::StdrtNullableWithoutId,
            (IsNullable::True, Pattern::Arr) => IdentPattern::ArrNullableWithIdentifier,
        });
        let ident_stdrt_not_null_ucc = &gen_ident_ucc(&IdentPattern::StdrtNotNullWithoutId);
        let ident_arr_not_null_ucc = &gen_ident_ucc(&IdentPattern::ArrNotNullWithId);
        let ident_with_id_stdrt_not_null_ucc = &gen_ident_ucc(&IdentPattern::StdrtNotNullWithId);
        let ident_with_id_arr_not_null_ucc = &gen_ident_ucc(&IdentPattern::ArrNotNullWithId);
        let is_stdrt_not_null = matches!((&is_nullable, pattern), (IsNullable::False, Pattern::Stdrt));
        let gen_type_as_import_ts = |first_type_ts: &dyn ToTokens, second_type_ts: &dyn ToTokens|{
            quote! {<#first_type_ts as #import::#second_type_ts>}
        };
        let gen_type_as_pg_json_ts = |ts: &dyn ToTokens| {
            gen_type_as_import_ts(&ts, &PgJsonUcc)
        };
        let ident_as_import_pg_json_ts = gen_type_as_pg_json_ts(&ident);
        let ident_stdrt_not_null_as_import_pg_json_ts = gen_type_as_pg_json_ts(&ident_stdrt_not_null_ucc);
        let ident_arr_not_null_as_import_pg_json_ts = gen_type_as_pg_json_ts(&ident_arr_not_null_ucc);
        let uuid_uuid_as_not_null_jsonb_string_ts = quote!{#import::#UuidUuidAsNotNullJsonbStringUcc};
        let uuid_uuid_as_not_null_jsonb_string_table_type_ucc = {
            let uuid_uuid_as_not_null_jsonb_string_table_type_ucc = SelfTableTypeUcc::from_display(&UuidUuidAsNotNullJsonbStringUcc);
            quote!{#import::#uuid_uuid_as_not_null_jsonb_string_table_type_ucc}
        };
        let uuid_uuid_as_not_null_jsonb_string_upd_ucc = {
            let uuid_uuid_as_not_null_jsonb_string_upd_ucc = SelfUpdUcc::from_display(&UuidUuidAsNotNullJsonbStringUcc);
            quote!{#import::#uuid_uuid_as_not_null_jsonb_string_upd_ucc}
        };
        let uuid_uuid_as_not_null_jsonb_string_where_ucc = {
            let uuid_uuid_as_not_null_jsonb_string_where_ucc = SelfWhereUcc::from_display(&UuidUuidAsNotNullJsonbStringUcc);
            quote!{#import::#uuid_uuid_as_not_null_jsonb_string_where_ucc}
        };
        let uuid_uuid_as_not_null_jsonb_string_as_import_pg_json_ts = gen_type_as_pg_json_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
        let uuid_uuid_as_not_null_jsonb_string_as_pg_json_upd_ts = quote!{
            #uuid_uuid_as_not_null_jsonb_string_as_import_pg_json_ts::Upd
        };
        let uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts = quote!{
            <#uuid_uuid_as_not_null_jsonb_string_ts as #import::PgJsonObjectVecElId>
        };
        let id_syn_field = {
            let v = Field {
                attrs: Vec::new(),
                vis: Visibility::Public(Pub { span: proc_macro2::Span::call_site() }),
                mutability: FieldMutability::None,
                ident: Some(Ident::new(&IdSc.to_string(), proc_macro2::Span::call_site())),
                colon_token: Some(Colon { spans: [proc_macro2::Span::call_site()] }),
                ty: Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: gen_simple_syn_punct(
                            &[import.to_path(), &UuidUuidAsNotNullJsonbStringUcc.to_string()]
                        ),
                    },
                }),
            };
            SynField {
                vis: v.vis.clone(),
                type0: v.ty,
                ident: v.ident.expect("3550d755"),
            }
        };
        let vec_syn_field_with_id: Vec<SynField> = vec_syn_field.clone().into_iter().fold(vec![id_syn_field], |mut acc, el0| {
            acc.push(el0);
            acc
        });
        let get_vec_syn_field = |is_stdrt_with_id: &IsStdrtWithId| -> &Vec<SynField> {
            match &is_stdrt_with_id {
                IsStdrtWithId::False => &vec_syn_field,
                IsStdrtWithId::True => &vec_syn_field_with_id,
            }
        };
        let gen_type_as_pg_type_ts = |ts: &dyn ToTokens| {
            gen_type_as_import_ts(&ts, &PgTypeUcc)
        };
        let gen_type_as_pg_json_test_cases_ts = |ts: &dyn ToTokens| {
            gen_type_as_import_ts(&ts, &PgJsonTestCasesUcc)
        };
        let gen_type_as_pg_type_test_cases_ts = |ts: &dyn ToTokens| {
            gen_type_as_import_ts(&ts, &PgTypeTestCasesUcc)
        };
        let self_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&SelfUcc);
        let ident_stdrt_not_null_as_pg_json_ts = gen_type_as_pg_json_ts(
            &ident_stdrt_not_null_ucc
        );
        let ident_stdrt_not_null_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ident_stdrt_not_null_ucc);
        let import_pg_json_uuid_uuid_as_not_null_jsonb_string_as_pg_json_ts = gen_type_as_pg_json_ts(
            &uuid_uuid_as_not_null_jsonb_string_ts
        );
        let ident_with_id_stdrt_not_null_table_type_ucc = SelfTableTypeUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let ident_with_id_stdrt_not_null_cr_ucc = SelfCrUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let ident_with_id_stdrt_not_null_read_ids_ucc = SelfReadIdsUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let ident_with_id_stdrt_not_null_where_ucc = SelfWhereUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let ident_ts = {
            let gen_struct_ident_ts = |ts: &dyn ToTokens| DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_copy()
                .build_struct(
                    &ts,
                    &Ts2::new(),
                    &quote!{;}
                );
            let ident_ts = gen_struct_ident_ts(&ident);
            let mb_ident_with_id_stdrt_not_null_ts = if is_stdrt_not_null {
                let ident_with_id_stdrt_not_null_ts = gen_struct_ident_ts(&ident_with_id_stdrt_not_null_ucc);
                let cfg_feature_test_utils_impl_ident_with_id_stdrt_not_null_ts = {
                    let read_ids_and_cr_into_where_equal_ts = gen_read_ids_and_cr_into_where_equal_ts(
                        &ident_with_id_stdrt_not_null_read_ids_ucc,
                        &ident_with_id_stdrt_not_null_cr_ucc,
                        &ident_with_id_stdrt_not_null_where_ucc,
                        &{
                            let gen_ts = |
                                fi: &dyn ToTokens,
                                ft: &dyn ToTokens,
                                second_argument_ts: &dyn ToTokens,
                            |{
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                                quote!{
                                    #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoTableTypeSc(
                                        #ReadIdsSc.0.#VSc.#fi,
                                        #second_argument_ts
                                    )
                                }
                            };
                            let ident_ts_e06846af = gen_ts(
                                &IdSc,
                                &uuid_uuid_as_not_null_jsonb_string_ts,
                                &PgCrudDfltOptSomeVecOneElCall
                            );
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                gen_ts(
                                    &fi,
                                    &el0.type0,
                                    &quote!{#CrSc.#fi}
                                )
                            });
                            quote!{
                                #ident_with_id_stdrt_not_null_where_ucc::#EqualUcc(pg_crud::PgJsonWhereEqual {
                                    oprtr: pg_crud::Oprtr::Or,
                                    #VSc: #ident_with_id_stdrt_not_null_table_type_ucc::new(
                                        #ident_ts_e06846af,
                                        #(#ts),*
                                    ),
                                })
                            }
                        },
                    );
                    let read_ids_and_cr_into_vec_where_equal_using_fields_ts = gen_read_ids_and_cr_into_vec_where_equal_using_fields_ts(
                        &import,
                        &ident_with_id_stdrt_not_null_read_ids_ucc,
                        &ident_with_id_stdrt_not_null_cr_ucc,
                        &ident_with_id_stdrt_not_null_where_ucc,
                        &{
                            let gen_ts = |
                                fi: &dyn ToTokens,
                                ft: &dyn ToTokens,
                                second_argument_ts: &dyn ToTokens,
                            |{
                                let fi_ucc = ToTokensToUccTs::case_or_panic(&fi);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                                quote!{
                                    #ident_with_id_stdrt_not_null_where_ucc::#fi_ucc(
                                        pg_crud::PgTypeWhere::new(
                                            pg_crud::Oprtr::And,
                                            #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoVecWhereEqualUsingFieldsSc(
                                                #ReadIdsSc.0.#VSc.#fi,
                                                #second_argument_ts
                                            ),
                                        ),
                                    )
                                }
                            };
                            let id_ts = gen_ts(
                                &IdSc,
                                &uuid_uuid_as_not_null_jsonb_string_ts,
                                &PgCrudDfltOptSomeVecOneElCall
                            );
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                gen_ts(
                                    &fi,
                                    &el0.type0,
                                    &quote!{#CrSc.#fi}
                                )
                            });
                            quote!{
                                #import::NotEmptyUniqueVec::try_new(vec![
                                    #id_ts,
                                    #(#ts),*
                                ]).expect("5473d8c4")
                            }
                        },
                    );
                    let read_ids_and_cr_into_vec_where_equal_to_json_field_ts = gen_read_ids_and_cr_into_vec_where_equal_to_json_field_ts(
                        import,
                        &ident_with_id_stdrt_not_null_read_ids_ucc,
                        &ident_with_id_stdrt_not_null_cr_ucc,
                        &ident_with_id_stdrt_not_null_where_ucc,
                        &{
                            let gen_ts = |
                                fi: &dyn ToTokens,
                                ft: &dyn ToTokens,
                                second_argument_ts: &dyn ToTokens,
                            |{
                                let fi_ucc = ToTokensToUccTs::case_or_panic(&fi);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                                quote!{
                                    #ident_with_id_stdrt_not_null_where_ucc::#fi_ucc(
                                        #import::PgTypeWhere::new(
                                            #import::Oprtr::Or,
                                            #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoVecWhereEqualToJsonFieldSc(
                                                #ReadIdsSc.0.#VSc.#fi,
                                                #second_argument_ts
                                            ),
                                        ),
                                    )
                                }
                            };
                            let id_ts = gen_ts(
                                &IdSc,
                                &uuid_uuid_as_not_null_jsonb_string_ts,
                                &PgCrudDfltOptSomeVecOneElCall
                            );
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                gen_ts(
                                    &fi,
                                    &el0.type0,
                                    &quote!{#CrSc.#fi}
                                )
                            });
                            quote!{
                                #import::NotEmptyUniqueVec::try_new(vec![
                                    #id_ts,
                                    #(#ts),*
                                ]).expect("221a4c55")
                            }
                        },
                    );
                    quote! {
                        #AllowClippyArbitrarySrcItemOrdering
                        #[cfg(feature = "test-utils")]
                        impl #ident_with_id_stdrt_not_null_ucc {
                            #read_ids_and_cr_into_where_equal_ts
                            #read_ids_and_cr_into_vec_where_equal_using_fields_ts
                            #read_ids_and_cr_into_vec_where_equal_to_json_field_ts
                        }
                    }
                };
                quote! {
                    #ident_with_id_stdrt_not_null_ts
                    #cfg_feature_test_utils_impl_ident_with_id_stdrt_not_null_ts
                }
            }
            else {
                Ts2::new()
            };
            quote! {
                #ident_ts
                #mb_ident_with_id_stdrt_not_null_ts
            }
        };
        let ident_arr_not_null_as_pg_json_ts = gen_type_as_pg_json_ts(&ident_arr_not_null_ucc);
        let ident_with_id_arr_not_null_as_pg_json_ts = gen_type_as_pg_json_ts(&ident_with_id_arr_not_null_ucc);
        let pg_json_subtype_table_type = PgJsonSubtype::TableType;
        let pg_json_subtype_cr = PgJsonSubtype::Cr;
        let pg_json_subtype_cr_for_query = PgJsonSubtype::CrForQuery;
        let pg_json_subtype_select = PgJsonSubtype::Select;
        let pg_json_subtype_where = PgJsonSubtype::Where;
        let pg_json_subtype_read = PgJsonSubtype::Read;
        let pg_json_subtype_read_inn = PgJsonSubtype::ReadInn;
        let pg_json_subtype_upd = PgJsonSubtype::Upd;
        let pg_json_subtype_upd_for_query = PgJsonSubtype::UpdForQuery;
        let gen_type_as_pg_json_subtype_ts = |type_ts: &dyn ToTokens, pg_json_subtype: &PgJsonSubtype| {
            let type_as_pg_json_ts = gen_type_as_pg_json_ts(&type_ts);
            quote! {#type_as_pg_json_ts::#pg_json_subtype}
        };
        let gen_type_as_pg_type_subtype_ts = |type_ts: &dyn ToTokens, pg_type_subtype: &PgTypeSubtype| {
            let type_as_pg_type_ts = gen_type_as_pg_type_ts(&type_ts);
            quote! {#type_as_pg_type_ts::#pg_type_subtype}
        };
        let gen_ft_as_crud_pg_json_from_field_ts = |
            syn_field: &SynField
        | gen_type_as_pg_json_ts(
            &syn_field.type0
        );
        let gen_gen_impl_location_lib_to_err_string_w_ts = |ts: &dyn ToTokens| gen_impl_to_err_string_ts(
            &Ts2::new(),
            &ts,
            &Ts2::new(),
            &quote! {format!("{self:?}")}
        );
        let ident_as_pg_json_table_type_ts = gen_type_as_pg_json_subtype_ts(&ident, &pg_json_subtype_table_type);
        let self_v_ts = quote! {Self(#VSc)};
        let pg_type_where_filter_qb_v_query_ts = quote!{#import::PgTypeWhereFilter::qb(#VSc, #QuerySc)};
        let ident_table_type_ucc = SelfTableTypeUcc::from_tokens(&ident);
        let ident_cr_ucc = SelfCrUcc::from_tokens(&ident);
        let ident_arr_not_null_upd_for_query_ucc = SelfUpdForQueryUcc::from_tokens(&ident_arr_not_null_ucc);
        let ident_stdrt_not_null_read_inn_ucc = SelfReadInnUcc::from_tokens(&ident_stdrt_not_null_ucc);
        let ident_with_id_stdrt_not_null_cr_for_query_ucc = SelfCrForQueryUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let wrap_into_scopes_dot_comma_ts = |ts: &dyn ToTokens| {
            let scopes_ts = wrap_into_scopes_ts(&ts);
            quote! {#scopes_ts;}
        };
        let gen_ident_table_type_or_ident_cr_common_ts = |pg_json_subtype_table_type_or_cr: &PgJsonSubtypeTableTypeOrCr| {
            let ident_table_type_or_ident_cr_ucc: &dyn DisplayPlusToTokens = match &pg_json_subtype_table_type_or_cr {
                PgJsonSubtypeTableTypeOrCr::TableType => &ident_table_type_ucc,
                PgJsonSubtypeTableTypeOrCr::Cr => &ident_cr_ucc,
            };
            let gen_ident_table_type_or_cr_ts = |
                attrs_ts: &dyn ToTokens,
                ident_ts_20954cb5: &dyn ToTokens,
                ts: &dyn ToTokens
            | {
                let ts_44f35e48 = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .d_schemars_json_schema()
                .build_struct(
                    &ident_ts_20954cb5,
                    &Ts2::new(),
                    &ts
                );
                quote!{
                    #attrs_ts
                    #ts_44f35e48
                }
            };
            let new_type_or_struct_dcl_struct_dcl = NewTypeOrStructDcl::StructDcl;
            let new_type_or_struct_dcl_new_type = NewTypeOrStructDcl::NewType;
            let gen_ident_table_type_or_cr_or_ident_with_id_table_type_or_cr_stdrt_not_null_ts = |
                is_stdrt_with_id: &IsStdrtWithId,
                pg_json_subtype_table_type_or_cr_925a7214: &PgJsonSubtypeTableTypeOrCr,
                new_type_or_struct_dcl: &NewTypeOrStructDcl
            | {
                let ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                    let fi = &el0.ident;
                    let type_as_pg_json_subtype_table_type_ts = gen_type_as_pg_json_subtype_ts(
                        &el0.type0,
                        &PgJsonSubtype::from(pg_json_subtype_table_type_or_cr_925a7214)
                    );
                    quote! {#fi: #type_as_pg_json_subtype_table_type_ts}
                });
                let fields_ts = quote! {#(#ts),*};
                match &new_type_or_struct_dcl {
                    NewTypeOrStructDcl::StructDcl => quote! {{#fields_ts}},
                    NewTypeOrStructDcl::NewType => fields_ts,
                }
            };
            let gen_tokens_table_type_or_cr_ts = |ts: &dyn ToTokens| {
                let ts0: &dyn ToTokens = match &pg_json_subtype_table_type_or_cr {
                    PgJsonSubtypeTableTypeOrCr::TableType => &SelfTableTypeUcc::from_tokens(&ts),
                    PgJsonSubtypeTableTypeOrCr::Cr => &SelfCrUcc::from_tokens(&ts),
                };
                quote!{#ts0}
            };
            let ident_table_type_or_ident_cr_ts = gen_ident_table_type_or_cr_ts(
                &match &pg_json_subtype_table_type_or_cr {
                    PgJsonSubtypeTableTypeOrCr::TableType => quote!{#AllowClippyArbitrarySrcItemOrdering},
                    PgJsonSubtypeTableTypeOrCr::Cr => Ts2::new(),
                },
                &ident_table_type_or_ident_cr_ucc,
                &match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => gen_ident_table_type_or_cr_or_ident_with_id_table_type_or_cr_stdrt_not_null_ts(&is_stdrt_with_id_false, pg_json_subtype_table_type_or_cr, &new_type_or_struct_dcl_struct_dcl),
                        IsNullable::True => wrap_into_scopes_dot_comma_ts(&gen_opt_type_dcl_ts(&gen_tokens_table_type_or_cr_ts(ident_stdrt_not_null_ucc))),
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => wrap_into_scopes_dot_comma_ts(&gen_vec_tokens_dcl_ts(
                            &gen_tokens_table_type_or_cr_ts(&ident_with_id_stdrt_not_null_ucc)
                        )),
                        IsNullable::True => wrap_into_scopes_dot_comma_ts(&gen_opt_type_dcl_ts(&gen_tokens_table_type_or_cr_ts(&ident_with_id_arr_not_null_ucc))),
                    },
                }
            );
            let gen_self_cnt_for_ident_or_ident_with_id_table_type_or_cr_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                let ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0|&el0.ident);
                quote! {Self {#(#ts),*}}
            };
            let impl_pub_new_for_ident_table_type_or_ident_cr_ts = {
                let params_ts = {
                    let gen_wrap_into_v_param_ts = |ts: &dyn ToTokens| {
                        quote! {#VSc: #ts}
                    };
                    match &pattern {
                        Pattern::Stdrt => match &is_nullable {
                            IsNullable::False => gen_ident_table_type_or_cr_or_ident_with_id_table_type_or_cr_stdrt_not_null_ts(&is_stdrt_with_id_false, pg_json_subtype_table_type_or_cr, &new_type_or_struct_dcl_new_type),
                            IsNullable::True => gen_wrap_into_v_param_ts(&gen_opt_type_dcl_ts(&gen_tokens_table_type_or_cr_ts(ident_stdrt_not_null_ucc))),
                        },
                        Pattern::Arr => match &is_nullable {
                            IsNullable::False => gen_wrap_into_v_param_ts(&gen_vec_tokens_dcl_ts(&gen_tokens_table_type_or_cr_ts(&ident_with_id_stdrt_not_null_ucc))),
                            IsNullable::True => gen_wrap_into_v_param_ts(&gen_opt_type_dcl_ts(&gen_vec_tokens_dcl_ts(&gen_tokens_table_type_or_cr_ts(
                                &ident_with_id_stdrt_not_null_ucc,
                            )))),
                        },
                    }
                };
                let ts = match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => gen_self_cnt_for_ident_or_ident_with_id_table_type_or_cr_ts(&is_stdrt_with_id_false),
                        IsNullable::True => self_v_ts.clone(),
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => self_v_ts.clone(),
                        IsNullable::True => {
                            let ident_arr_not_null_with_id_postfix_ucc = gen_tokens_table_type_or_cr_ts(&gen_ident_ucc(&IdentPattern::ArrNotNullWithId));
                            quote! {Self(#VSc.map(#ident_arr_not_null_with_id_postfix_ucc::new))}
                        }
                    },
                };
                if matches!(&pattern, Pattern::Arr) && matches!(&is_nullable, IsNullable::True) {
                    gen_impl_pub_new_for_ident_ts(
                        &ident_table_type_or_ident_cr_ucc,
                        &MustUse,
                        &params_ts,
                        &ts,
                    )
                }
                else {
                    gen_impl_pub_const_new_for_ident_ts(
                        &ident_table_type_or_ident_cr_ucc,
                        &MustUse,
                        &params_ts,
                        &ts,
                    )
                }
            };
            let gen_impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_cr_ts = |
                ident_ts_36ce1199: &dyn ToTokens,
                ts: &dyn ToTokens
            | gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(
                &ident_ts_36ce1199,
                &Ts2::new(),
                &quote! {Self #ts}
            );
            let gen_impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_cr_stdrt_not_null_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                let ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                    let fi = &el0.ident;
                    quote! {#fi: #PgCrudDfltOptSomeVecOneElCall}
                });
                quote! {{#(#ts),*}}
            };
            let impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_cr_stdrt_not_null_ts = gen_impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_cr_stdrt_not_null_ts(&is_stdrt_with_id_false);
            let scopes_vec_pg_crud_dflt_opt_some_vec_one_el_call_ts = quote! {(#vec_pg_crud_dflt_opt_some_vec_one_el_call_ts)};
            let scopes_some_pg_crud_dflt_opt_some_vec_one_el_call_ts = quote! {
                (Some(#PgCrudDfltOptSomeVecOneElCall))
            };
            let impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_ident_cr_ts = gen_impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_cr_ts(
                &ident_table_type_or_ident_cr_ucc,
                match &is_nullable {
                    IsNullable::False => match &pattern {
                        Pattern::Stdrt => &impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_cr_stdrt_not_null_ts,
                        Pattern::Arr => &scopes_vec_pg_crud_dflt_opt_some_vec_one_el_call_ts,
                    },
                    IsNullable::True => &scopes_some_pg_crud_dflt_opt_some_vec_one_el_call_ts,
                },
            );
            let impl_sqlx_encode_sqlx_pg_for_ident_table_type_or_ident_cr_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(
                &ident_table_type_or_ident_cr_ucc,
                &quote!{sqlx::types::Json(#SelfSc)}
            );
            let impl_sqlx_type_for_ident_table_type_or_ident_cr_ts = gen_impl_sqlx_type_for_ident_ts(
                &ident_table_type_or_ident_cr_ucc,
                &quote!{sqlx::types::Json<#SelfUcc>}
            );
            let mb_ident_with_id_table_type_or_ident_with_id_cr_stdrt_not_null_ts = if is_stdrt_not_null {
                let ident_with_id_table_type_or_ident_with_id_stdrt_not_null_cr_ucc: &dyn DisplayPlusToTokens = match &pg_json_subtype_table_type_or_cr {
                    PgJsonSubtypeTableTypeOrCr::TableType => &ident_with_id_stdrt_not_null_table_type_ucc,
                    PgJsonSubtypeTableTypeOrCr::Cr => &ident_with_id_stdrt_not_null_cr_ucc,
                };
                let is_stdrt_with_id_240dfa72 = match &pg_json_subtype_table_type_or_cr {
                    PgJsonSubtypeTableTypeOrCr::TableType => &is_stdrt_with_id_true,
                    PgJsonSubtypeTableTypeOrCr::Cr => &is_stdrt_with_id_false,
                };
                let ident_with_id_table_type_or_ident_with_id_cr_stdrt_not_null_ts = gen_ident_table_type_or_cr_ts(
                    &AllowClippyArbitrarySrcItemOrdering,
                    &ident_with_id_table_type_or_ident_with_id_stdrt_not_null_cr_ucc,
                    &gen_ident_table_type_or_cr_or_ident_with_id_table_type_or_cr_stdrt_not_null_ts(is_stdrt_with_id_240dfa72, pg_json_subtype_table_type_or_cr, &new_type_or_struct_dcl_struct_dcl),
                );
                let impl_pub_const_new_for_ident_with_id_table_type_or_ident_with_id_cr_stdrt_not_null_ts = gen_impl_pub_const_new_for_ident_ts(
                    &ident_with_id_table_type_or_ident_with_id_stdrt_not_null_cr_ucc,
                    &MustUse,
                    &gen_ident_table_type_or_cr_or_ident_with_id_table_type_or_cr_stdrt_not_null_ts(is_stdrt_with_id_240dfa72, pg_json_subtype_table_type_or_cr, &new_type_or_struct_dcl_new_type),
                    &gen_self_cnt_for_ident_or_ident_with_id_table_type_or_cr_ts(is_stdrt_with_id_240dfa72),
                );
                let impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_with_id_table_type_or_ident_with_id_cr_stdrt_not_null_ts = gen_impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_cr_ts(
                    &ident_with_id_table_type_or_ident_with_id_stdrt_not_null_cr_ucc,
                    &match &pg_json_subtype_table_type_or_cr {
                        PgJsonSubtypeTableTypeOrCr::TableType => gen_impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_cr_stdrt_not_null_ts(&is_stdrt_with_id_true),
                        PgJsonSubtypeTableTypeOrCr::Cr => impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_cr_stdrt_not_null_ts,
                    },
                );
                quote! {
                    #ident_with_id_table_type_or_ident_with_id_cr_stdrt_not_null_ts
                    #impl_pub_const_new_for_ident_with_id_table_type_or_ident_with_id_cr_stdrt_not_null_ts
                    #impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_with_id_table_type_or_ident_with_id_cr_stdrt_not_null_ts
                }
            } else {
                Ts2::new()
            };
            quote! {
                #ident_table_type_or_ident_cr_ts
                #impl_pub_new_for_ident_table_type_or_ident_cr_ts
                #impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_table_type_or_ident_cr_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_table_type_or_ident_cr_ts
                #impl_sqlx_type_for_ident_table_type_or_ident_cr_ts
                #mb_ident_with_id_table_type_or_ident_with_id_cr_stdrt_not_null_ts
            }
        };
        let ident_table_type_ts = {
            let ident_table_type_common_ts = gen_ident_table_type_or_ident_cr_common_ts(&PgJsonSubtypeTableTypeOrCr::TableType);
            quote! {
                #ident_table_type_common_ts
            }
        };
        let gen_type_as_pg_json_cr_ts = |ts: &dyn ToTokens| gen_type_as_pg_json_subtype_ts(&ts, &pg_json_subtype_cr);
        let gen_type_as_pg_json_cr_for_query_ts = |ts: &dyn ToTokens| gen_type_as_pg_json_subtype_ts(&ts, &pg_json_subtype_cr_for_query);
        let ident_cr_ts = {
            let ident_cr_common_ts = gen_ident_table_type_or_ident_cr_common_ts(&PgJsonSubtypeTableTypeOrCr::Cr);
            let gen_impl_display_for_ident_cr_ts = |ts: &dyn ToTokens| gen_impl_display_ts(
                &Ts2::new(),
                &ts,
                &Ts2::new(),
                &quote! {write!(f, "{self:?}")}
            );
            let impl_display_for_ident_cr_ts = gen_impl_display_for_ident_cr_ts(&ident_cr_ucc);
            let impl_location_lib_to_err_string_for_ident_cr_ts = gen_gen_impl_location_lib_to_err_string_w_ts(&ident_cr_ucc);
            let mb_ident_with_id_cr_stdrt_not_null_ts = if is_stdrt_not_null {
                let impl_display_for_ident_with_id_cr_stdrt_not_null_ts = gen_impl_display_for_ident_cr_ts(&ident_with_id_stdrt_not_null_cr_ucc);
                let impl_location_lib_to_err_string_for_ident_with_id_cr_stdrt_not_null_ts = gen_gen_impl_location_lib_to_err_string_w_ts(&ident_with_id_stdrt_not_null_cr_ucc);
                quote! {
                    #impl_display_for_ident_with_id_cr_stdrt_not_null_ts
                    #impl_location_lib_to_err_string_for_ident_with_id_cr_stdrt_not_null_ts
                }
            } else {
                Ts2::new()
            };
            quote! {
                #ident_cr_common_ts
                #impl_display_for_ident_cr_ts
                #impl_location_lib_to_err_string_for_ident_cr_ts
                #mb_ident_with_id_cr_stdrt_not_null_ts
            }
        };
        let ident_cr_for_query_ucc = SelfCrForQueryUcc::from_tokens(&ident);
        let self_as_pg_json_cr_ts = gen_type_as_pg_json_cr_ts(&SelfUcc);
        let ident_stdrt_not_null_as_pg_json_cr_for_query_ts = gen_type_as_pg_json_cr_for_query_ts(&ident_stdrt_not_null_ucc);
        let ident_arr_not_null_as_pg_json_cr_for_query_ts = gen_type_as_pg_json_cr_for_query_ts(&ident_arr_not_null_ucc);
        let ident_arr_not_null_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ident_arr_not_null_ucc);
        let pg_crud_path_pg_json_uuid_uuid_cr_for_query_ts = gen_type_as_pg_json_cr_for_query_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
        let gen_debug_clone_partialeq_serialize_pub_struct_ts = |
            attrs_ts: &dyn ToTokens,
            ident_ts_d8fa00d3: &dyn ToTokens,
            ts_153ac202: &dyn ToTokens
        | {
            let ts_6ea2da58 = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .d_serde_serialize()
                .build_struct(
                    &ident_ts_d8fa00d3,
                    &Ts2::new(),
                    &ts_153ac202
                );
            quote!{
                #attrs_ts
                #ts_6ea2da58
            }
        };
        let ident_cr_for_query_ts = {
            let gen_struct_stdrt_not_null_ts = |is_stdrt_with_id: &IsStdrtWithId|{
                let ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                    let fi = &el0.ident;
                    let type_as_pg_json_subtype_crate_for_query_ts = gen_type_as_pg_json_subtype_ts(
                        &el0.type0,
                        &PgJsonSubtype::CrForQuery
                    );
                    quote! {#fi: #type_as_pg_json_subtype_crate_for_query_ts}
                });
                quote! {{#(#ts),*}}
            };
            let impl_from_stdrt_not_null_without_id_ts = {
                let ts = vec_syn_field.iter().map(|el0| {
                    let fi = &el0.ident;
                    let type_as_pg_json_subtype_crate_for_query_ts = gen_type_as_pg_json_subtype_ts(
                        &el0.type0,
                        &PgJsonSubtype::CrForQuery
                    );
                    quote! {#fi: #type_as_pg_json_subtype_crate_for_query_ts::from(#VSc.#fi)}
                });
                quote! {#(#ts),*}
            };
            let ident_cr_for_query_ts = {
                let ident_cr_for_query_ts = gen_debug_clone_partialeq_serialize_pub_struct_ts(
                    &AllowClippyArbitrarySrcItemOrdering,
                    &ident_cr_for_query_ucc,
                    &match &pattern {
                        Pattern::Stdrt => match &is_nullable {
                            IsNullable::False => gen_struct_stdrt_not_null_ts(&is_stdrt_with_id_false),
                            IsNullable::True => {
                                wrap_into_scopes_dot_comma_ts(
                                    &gen_opt_type_dcl_ts(
                                        &gen_type_as_pg_json_subtype_ts(
                                            &ident_stdrt_not_null_ucc,
                                            &pg_json_subtype_cr_for_query,
                                        )
                                    )
                                )
                            },
                        },
                        Pattern::Arr => match &is_nullable {
                            IsNullable::False => wrap_into_scopes_dot_comma_ts(
                                &gen_vec_tokens_dcl_ts(
                                    &ident_with_id_stdrt_not_null_cr_for_query_ucc
                                )
                            ),
                            IsNullable::True => wrap_into_scopes_dot_comma_ts(
                                &gen_opt_type_dcl_ts(
                                    &gen_type_as_pg_json_subtype_ts(
                                        &ident_arr_not_null_ucc,
                                        &pg_json_subtype_cr_for_query,
                                    )
                                )
                            ),
                        },
                    }
                );
                let impl_from_ident_cr_for_ident_cr_for_query_ts = gen_impl_from_ts(
                    &ident_cr_ucc,
                    &ident_cr_for_query_ucc,
                    &{
                        let ts = match &is_nullable {
                            IsNullable::False => match &pattern {
                                Pattern::Stdrt => quote! {{#impl_from_stdrt_not_null_without_id_ts}},
                                Pattern::Arr => quote!{(
                                    #VSc.0.into_iter().map(#ident_with_id_stdrt_not_null_cr_for_query_ucc::from).collect()
                                )},
                            },
                            IsNullable::True => {
                                let ts: &dyn ToTokens = match &pattern {
                                    Pattern::Stdrt => &ident_stdrt_not_null_as_pg_json_cr_for_query_ts,
                                    Pattern::Arr => &ident_arr_not_null_as_pg_json_cr_for_query_ts,
                                };
                                quote!{(#VSc.0.map(#ts::from))}
                            },
                        };
                        quote! {Self #ts}
                    }
                );
                let impl_sqlx_encode_sqlx_pg_for_ident_cr_for_query_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(
                    &ident_cr_for_query_ucc,
                    &quote!{sqlx::types::Json(#SelfSc)}
                );
                let impl_sqlx_type_for_ident_cr_for_query_ts = gen_impl_sqlx_type_for_ident_ts(
                    &ident_cr_for_query_ucc,
                    &quote!{sqlx::types::Json<#SelfUcc>}
                );
                quote! {
                    #ident_cr_for_query_ts
                    #impl_from_ident_cr_for_ident_cr_for_query_ts
                    #impl_sqlx_encode_sqlx_pg_for_ident_cr_for_query_ts
                    #impl_sqlx_type_for_ident_cr_for_query_ts
                }
            };
            let mb_ident_with_id_stdrt_not_null_cr_for_query_ts = if is_stdrt_not_null {
                let ident_with_id_stdrt_not_null_cr_for_query_ts = gen_debug_clone_partialeq_serialize_pub_struct_ts(
                    &AllowClippyArbitrarySrcItemOrdering,
                    &ident_with_id_stdrt_not_null_cr_for_query_ucc,
                    &gen_struct_stdrt_not_null_ts(&is_stdrt_with_id_true)
                );
                let impl_from_ident_with_id_stdrt_not_null_cr_for_ident_with_id_stdrt_not_null_cr_for_query_ts = gen_impl_from_ts(
                    &ident_with_id_stdrt_not_null_cr_ucc,
                    &ident_with_id_stdrt_not_null_cr_for_query_ucc,
                    &quote! {Self {
                        #IdSc: #pg_crud_path_pg_json_uuid_uuid_cr_for_query_ts::new(
                            uuid::Uuid::new_v4()
                        ),
                        #impl_from_stdrt_not_null_without_id_ts
                    }}
                );
                quote! {
                    #ident_with_id_stdrt_not_null_cr_for_query_ts
                    #impl_from_ident_with_id_stdrt_not_null_cr_for_ident_with_id_stdrt_not_null_cr_for_query_ts
                }
            } else {
                Ts2::new()
            };
            quote! {
                #ident_cr_for_query_ts
                #mb_ident_with_id_stdrt_not_null_cr_for_query_ts
            }
        };
        let gen_sqlx_types_json_type_dcl_w_ts = |ts: &dyn ToTokens| gen_impl_sqlx_type_for_ident_ts(
            &ts,
            &gen_sqlx_types_json_type_dcl_ts(&SelfUcc)
        );
        let gen_impl_sqlx_decode_sqlx_pg_for_ident_w_ts = |ts: &dyn ToTokens| gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
            &ts,
            &gen_sqlx_types_json_type_dcl_ts(&SelfUcc),
            &quote! {Ok(v.0)}
        );
        let gen_v_type_ts = |ts: &dyn ToTokens| {
            quote! {#VSc: #ts}
        };
        let gen_pub_const_new_v_type_cnt_self_v_ts = |ts: &dyn ToTokens|gen_pub_const_new_ts(
            &MustUse,
            &gen_v_type_ts(&ts),
            &self_v_ts
        );
        let gen_unique_vec_w_ts = |ts: &dyn ToTokens| {
            quote! {#import::NotEmptyUniqueVec<#ts>}
        };
        let self_some_pg_crud_dflt_opt_some_vec_one_el_call_ts = quote! {
            Self(Some(#PgCrudDfltOptSomeVecOneElCall))
        };
        let self_some_pg_crud_dflt_opt_some_vec_one_el_max_page_size_call_ts = quote! {
            Self(Some(#PgCrudDfltOptSomeVecOneElMaxPageSizeCall))
        };
        let gen_type_as_pg_json_upd_ts = |ts: &dyn ToTokens| gen_type_as_pg_json_subtype_ts(&ts, &pg_json_subtype_upd);
        let gen_type_as_pg_json_upd_for_query_ts = |ts: &dyn ToTokens| gen_type_as_pg_json_subtype_ts(&ts, &pg_json_subtype_upd_for_query);
        let self_as_pg_json_ts = gen_type_as_pg_json_ts(&SelfUcc);
        let self_as_pg_json_upd_ts = gen_type_as_pg_json_upd_ts(&SelfUcc);
        let self_as_pg_json_cr_for_query_ts = gen_type_as_pg_json_cr_for_query_ts(&SelfUcc);
        let pg_crud_path_pg_json_uuid_uuid_upd_ts = gen_type_as_pg_json_upd_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
        let pg_crud_path_pg_json_uuid_uuid_upd_for_query_ts = gen_type_as_pg_json_upd_for_query_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
        let ident_select_ucc = SelfSelectUcc::from_tokens(&ident);
        let ident_with_id_stdrt_not_null_select_ucc = SelfSelectUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let gen_type_as_pg_json_select_ts = |ts: &dyn ToTokens| gen_type_as_pg_json_subtype_ts(&ts, &pg_json_subtype_select);
        let ident_stdrt_not_null_as_pg_json_select_ts = gen_type_as_pg_json_select_ts(&ident_stdrt_not_null_ucc);
        let ident_with_id_arr_not_null_as_pg_json_select_ts = gen_type_as_pg_json_select_ts(&ident_with_id_arr_not_null_ucc);
        let ident_with_id_stdrt_not_null_select_sc = SelfSelectSc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let dim1_pgn_ts = quote! {dim1_pgn};
        let ident_stdrt_not_null_select_el_ucc = SelfSelectElUcc::from_tokens(&ident_stdrt_not_null_ucc);
        let ident_with_id_stdrt_not_null_select_el_ucc = SelfSelectElUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let gen_select_qp_for_loop_ts = |
            acc_ts: &dyn ToTokens,
            is_stdrt_with_id: &IsStdrtWithId,
            in_ts: &dyn ToTokens,
            column_field_fi_ts: &dyn ToTokens,
            column_field_for_er_message_fi_ts: &dyn ToTokens,
        |{
            let ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                let fi_str = el0.ident.to_string();
                let vrt_name_ts: &dyn ToTokens = &AsRefStrToUccTs::case_or_panic(&fi_str);
                let fi_dq_ts: &dyn ToTokens = &dq_ts(&fi_str);
                let ft_as_crud_pg_json_from_field_ts = gen_type_as_pg_json_ts(&el0.type0);
                let ident_or_ident_with_id_stdrt_not_null_select_el_ucc: &dyn ToTokens = match &is_stdrt_with_id {
                    IsStdrtWithId::False => &ident_stdrt_not_null_select_el_ucc,
                    IsStdrtWithId::True => &ident_with_id_stdrt_not_null_select_el_ucc,
                };
                quote! {
                    #ident_or_ident_with_id_stdrt_not_null_select_el_ucc::#vrt_name_ts(v_3c8acf6a) => match #ft_as_crud_pg_json_from_field_ts::#SelectQpSc(
                        v_3c8acf6a,
                        #fi_dq_ts,
                        #column_field_fi_ts,
                        #column_field_for_er_message_fi_ts,
                        false,
                    ) {
                        Ok(v_d54cf786) => v_d54cf786,
                        Err(#ErSc) => {
                            return Err(#ErSc);
                        }
                    }
                }
            });
            let if_write_is_err_ts = gen_if_write_is_err_ts(
                &quote!{
                    #acc_ts,
                    "{}||",
                    match el {
                        #(#ts),*
                    }
                },
                &return_err_qp_er_write_into_buffer_ts
            );
            quote!{
                for el in #in_ts #self_field_vec_ts {
                    #if_write_is_err_ts
                }
            }
        };
        let ident_select_ts = {
            let gen_pub_struct_ident_select_ts = |
                attrs_ts: &dyn ToTokens,
                ident_ts_6fce2985: &dyn ToTokens,
                ts_fc7ad384: &dyn ToTokens
            | {
                let ts_83d3ad18 = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_utoipa_to_schema()
                .d_schemars_json_schema()
                .build_struct(
                    &ident_ts_6fce2985,
                    &Ts2::new(),
                    &ts_fc7ad384
                );
                quote!{
                    #attrs_ts
                    #ts_83d3ad18
                }
            };
            let gen_ident_select_stdrt_not_null_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                let ident_stdrt_not_null_select_ucc = SelfSelectUcc::from_tokens(&ident_stdrt_not_null_ucc);
                gen_pub_struct_ident_select_ts(
                    &AllowClippyArbitrarySrcItemOrdering,
                    match &is_stdrt_with_id {
                        IsStdrtWithId::False => &ident_stdrt_not_null_select_ucc,
                        IsStdrtWithId::True => &ident_with_id_stdrt_not_null_select_ucc,
                    },
                    &wrap_into_scopes_dot_comma_ts(&gen_unique_vec_w_ts(match &is_stdrt_with_id {
                        IsStdrtWithId::False => &ident_stdrt_not_null_select_el_ucc,
                        IsStdrtWithId::True => &ident_with_id_stdrt_not_null_select_el_ucc,
                    })),
                )
            };
            let import_pgn_ts = quote! {#import::PgnStartsWithZero};
            let ident_select_ts = match &is_nullable {
                IsNullable::False => match &pattern {
                    Pattern::Stdrt => gen_ident_select_stdrt_not_null_ts(&is_stdrt_with_id_false),
                    Pattern::Arr => gen_pub_struct_ident_select_ts(
                        &AllowClippyArbitrarySrcItemOrdering,
                        &ident_select_ucc,
                        &quote! {{
                            #ident_with_id_stdrt_not_null_select_sc: #ident_with_id_stdrt_not_null_select_ucc,
                            #dim1_pgn_ts: #import_pgn_ts
                        }},
                    ),
                },
                IsNullable::True => gen_pub_struct_ident_select_ts(
                    &AllowClippyArbitrarySrcItemOrdering,
                    &ident_select_ucc,
                    &wrap_into_scopes_dot_comma_ts(&gen_opt_type_dcl_ts(&match &pattern {
                        Pattern::Stdrt => &ident_stdrt_not_null_as_pg_json_select_ts,
                        Pattern::Arr => &ident_with_id_arr_not_null_as_pg_json_select_ts,
                    })),
                ),
            };
            let impl_ident_select_ts = {
                let pub_new_ts = {
                    let params_ts = {
                        let unique_vec_ident_select_el_stdrt_not_null_ts = gen_unique_vec_w_ts(&ident_stdrt_not_null_select_el_ucc);
                        match &pattern {
                            Pattern::Stdrt => match &is_nullable {
                                IsNullable::False => gen_v_type_ts(&unique_vec_ident_select_el_stdrt_not_null_ts),
                                IsNullable::True => gen_v_type_ts(&gen_opt_type_dcl_ts(&unique_vec_ident_select_el_stdrt_not_null_ts)),
                            },
                            Pattern::Arr => match &is_nullable {
                                IsNullable::False => quote! {
                                    #ident_with_id_stdrt_not_null_select_sc: #ident_with_id_stdrt_not_null_select_ucc,
                                    #dim1_pgn_ts: #import_pgn_ts
                                },
                                IsNullable::True => gen_v_type_ts(&gen_opt_type_dcl_ts(&ident_with_id_arr_not_null_as_pg_json_select_ts)),
                            },
                        }
                    };
                    let ts = match &pattern {
                        Pattern::Stdrt => match &is_nullable {
                            IsNullable::False => self_v_ts.clone(),
                            IsNullable::True => quote! {
                                Self(#VSc.map(#ident_stdrt_not_null_as_pg_json_select_ts::new))
                            },
                        },
                        Pattern::Arr => match &is_nullable {
                            IsNullable::False => {
                                quote! {Self {
                                    #ident_with_id_stdrt_not_null_select_sc,
                                    #dim1_pgn_ts,
                                }}
                            }
                            IsNullable::True => self_v_ts.clone(),
                        },
                    };
                    if matches!(&pattern, Pattern::Stdrt) && matches!(&is_nullable, IsNullable::True) {
                        gen_pub_new_ts(
                            &MustUse,
                            &params_ts,
                            &ts
                        )
                    }
                    else {
                         gen_pub_const_new_ts(
                            &MustUse,
                            &params_ts,
                            &ts
                        )
                    }
                };
                let mb_select_qp_ts = if matches!(&pattern, Pattern::Stdrt) &&
                matches!(&is_nullable, IsNullable::False) {
                    let acc_ac57d097_ts = quote!{acc_ac57d097};
                    let select_qp_for_loop_ts = gen_select_qp_for_loop_ts(
                        &acc_ac57d097_ts,
                        &is_stdrt_with_id_false,
                        &SelfSc,
                        &quote!{column_field},
                        &quote!{column_field_for_er_message},
                    );
                    quote! {
                        fn #SelectQpSc(
                            &self,
                            column_field: &str,
                            column_field_for_er_message: &str,
                        ) -> Result<#StringTs, #import_qp_er_ts> {
                            let mut #acc_ac57d097_ts = #StringTs::default();
                            #select_qp_for_loop_ts
                            let _: Option<char> = #acc_ac57d097_ts.pop();
                            let _: Option<char> = #acc_ac57d097_ts.pop();
                            Ok(#acc_ac57d097_ts)
                        }
                    }
                }
                else {
                    Ts2::new()
                };
                let select_qp_pg_type_ts = {
                    let ts = match &pattern {
                        Pattern::Stdrt => match &is_nullable {
                            IsNullable::False => quote! {
                                #SelfSc.#SelectQpSc(
                                    #ColumnSc,
                                    #ColumnSc,
                                )
                            },
                            IsNullable::True => {
                                let ident_740c9034 = match &pattern {
                                    Pattern::Stdrt => &ident_stdrt_not_null_as_pg_json_select_ts,
                                    Pattern::Arr => &ident_with_id_arr_not_null_as_pg_json_select_ts,
                                };
                                quote! {
                                    let #VSc = self.0.as_ref().map_or_else(
                                        <#ident_740c9034 as pg_crud::DfltOptSomeVecOneEl>::dflt_opt_some_vec_one_el,
                                        Clone::clone
                                    );
                                    match #VSc.#SelectQpPgTypeSc(#ColumnSc) {
                                        Ok(v_c69f1ffe) => Ok(format!("case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({v_c69f1ffe}) end")),
                                        Err(#ErSc) => Err(#ErSc)
                                    }
                                }
                            },
                        },
                        Pattern::Arr => match &is_nullable {
                            IsNullable::False => {
                                let acc_399d9786_ts = quote!{acc_399d9786};
                                let select_qp_for_loop_ts = gen_select_qp_for_loop_ts(
                                    &acc_399d9786_ts,
                                    &is_stdrt_with_id_true,
                                    &quote!{#SelfSc.#ident_with_id_stdrt_not_null_select_sc},
                                    &dq_ts(&ValueSc),
                                    &ColumnSc
                                );
                                let format_ts = dq_ts(&format!(
                                    "(case when (jsonb_array_length({{column}}) = 0) then '[]'::jsonb else (select jsonb_agg(({{{ident_with_id_stdrt_not_null_select_sc}}})) from jsonb_array_elements((select {{column}})) with ordinality where ordinality between {{dim1_start}} and {{dim1_end}}) end)"
                                ));
                                quote! {
                                    let #ident_with_id_stdrt_not_null_select_sc = {
                                        let mut #acc_399d9786_ts = #StringTs::default();
                                        #select_qp_for_loop_ts
                                        let _: Option<char> = #acc_399d9786_ts.pop();
                                        let _: Option<char> = #acc_399d9786_ts.pop();
                                        #acc_399d9786_ts
                                    };
                                    let dim1_start = self.#dim1_pgn_ts.start();
                                    let dim1_end = self.#dim1_pgn_ts.end();
                                    Ok(format!(#format_ts))
                                }
                            }
                            IsNullable::True => {
                                let format_ts = dq_ts(&"case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({v_c2ca032e}) end");
                                let ident_with_id_arr_not_null_as_pg_json_select_as_dflt_but_opt_is_some_ts = gen_ident_as_dflt_but_opt_is_some_ts(
                                    &ident_with_id_arr_not_null_as_pg_json_select_ts
                                );
                                quote! {
                                    let #VSc = self.0.as_ref().map_or_else(
                                        #ident_with_id_arr_not_null_as_pg_json_select_as_dflt_but_opt_is_some_ts,
                                        Clone::clone
                                    );
                                    match #VSc.#SelectQpPgTypeSc(column) {
                                        Ok(v_c2ca032e) => Ok(format!(#format_ts)),
                                        Err(#ErSc) => Err(#ErSc)
                                    }
                                }
                            },
                        },
                    };
                    quote! {
                        fn #SelectQpPgTypeSc(
                            &self,
                            #ColumnSc: &str,
                        ) -> Result<#StringTs, #import_qp_er_ts> {
                            #ts
                        }
                    }
                };
                quote! {
                    impl #ident_select_ucc {
                        #pub_new_ts
                        #mb_select_qp_ts
                        #select_qp_pg_type_ts
                    }
                }
            };
            let impl_sqlx_type_for_ident_select_ts = gen_sqlx_types_json_type_dcl_w_ts(&ident_select_ucc);
            let impl_sqlx_decode_sqlx_pg_for_ident_select_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_w_ts(&ident_select_ucc);
            let impl_pg_crud_dflt_opt_some_vec_one_el_stdrt_not_null_ts = quote! {
                Self(#PgCrudDfltOptSomeVecOneElCall)
            };
            let impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_stdrt_not_null_ts = quote! {
                Self(#PgCrudDfltOptSomeVecOneElMaxPageSizeCall)
            };
            let (
                impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_select_ts,
                impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_for_ident_select_ts
            ) = {
                let gen_ts = |dflt_some_one_or_dflt_some_one_with_max_page_size: &DefaultSomeOneOrDefaultSomeOneWithMaxPageSize|{
                    match &pattern {
                        Pattern::Stdrt => match &is_nullable {
                            IsNullable::False => match &dflt_some_one_or_dflt_some_one_with_max_page_size {
                                DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => quote! {#impl_pg_crud_dflt_opt_some_vec_one_el_stdrt_not_null_ts},
                                DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => quote! {#impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_stdrt_not_null_ts},
                            },
                            IsNullable::True => match &dflt_some_one_or_dflt_some_one_with_max_page_size {
                                DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => self_some_pg_crud_dflt_opt_some_vec_one_el_call_ts.clone(),
                                DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => self_some_pg_crud_dflt_opt_some_vec_one_el_max_page_size_call_ts.clone(),
                            },
                        },
                        Pattern::Arr => match &is_nullable {
                            IsNullable::False => {
                                let (
                                    ident_with_id_stdrt_not_null_select_ts,
                                    dim1_pgn_ts_364caa54
                                ): (
                                    &dyn ToTokens,
                                    &dyn ToTokens
                                ) = match &dflt_some_one_or_dflt_some_one_with_max_page_size {
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => (
                                        &PgCrudDfltOptSomeVecOneElCall,
                                        &PgCrudDfltOptSomeVecOneElCall
                                    ),
                                    DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => (
                                        &PgCrudDfltOptSomeVecOneElMaxPageSizeCall,
                                        &PgCrudDfltOptSomeVecOneElMaxPageSizeCall
                                    ),
                                };
                                quote! {
                                    Self {
                                        #ident_with_id_stdrt_not_null_select_sc: #ident_with_id_stdrt_not_null_select_ts,
                                        #dim1_pgn_ts: #dim1_pgn_ts_364caa54,
                                    }
                                }
                            },
                            IsNullable::True => match &dflt_some_one_or_dflt_some_one_with_max_page_size {
                                DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => self_some_pg_crud_dflt_opt_some_vec_one_el_call_ts.clone(),
                                DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => self_some_pg_crud_dflt_opt_some_vec_one_el_max_page_size_call_ts.clone(),
                            },
                        },
                    }
                };
                (
                    gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(
                        &ident_select_ucc,
                        &Ts2::new(),
                        &gen_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)
                    ),
                    gen_impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_ts(
                        &ident_select_ucc,
                        &Ts2::new(),
                        &gen_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)
                    )
                )
            };
            let gen_ident_select_el_or_ident_with_id_stdrt_not_null_select_el_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                let ident_select_el_or_ident_with_id_select_el_ucc: &dyn ToTokens = match &is_stdrt_with_id {
                    IsStdrtWithId::False => &ident_stdrt_not_null_select_el_ucc,
                    IsStdrtWithId::True => &ident_with_id_stdrt_not_null_select_el_ucc,
                };
                let ident_select_el_or_ident_with_id_stdrt_not_null_select_el_ts = {
                    let ts_bf3bd19e = DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone()
                    .d_partial_eq()
                    .d_serde_serialize()
                    .d_serde_deserialize()
                    .d_utoipa_to_schema()
                    .d_schemars_json_schema()
                    .build_enum(
                        &ident_select_el_or_ident_with_id_select_el_ucc,
                        &Ts2::new(),
                        &{
                            let ts_ecc4a666 = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                                let fi = &el0.ident;
                                let serde_fi_dq_ts = dq_ts(&fi);
                                let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                                let ft_as_json_type_select_ts = gen_type_as_pg_json_select_ts(&el0.type0);
                                quote! {
                                    #[serde(rename(serialize = #serde_fi_dq_ts, deserialize = #serde_fi_dq_ts))]
                                    #vrt_ident_ucc_ts(#ft_as_json_type_select_ts)
                                }
                            });
                            quote!{{#(#ts_ecc4a666),*}}
                        }
                    );
                    quote!{
                        #AllowClippyArbitrarySrcItemOrdering
                        #ts_bf3bd19e
                    }
                };
                let impl_location_lib_to_err_string_for_ident_select_el_or_ident_with_id_stdrt_not_null_select_el_ts = gen_gen_impl_location_lib_to_err_string_w_ts(&ident_select_el_or_ident_with_id_select_el_ucc);
                let (
                    impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_select_el_or_ident_with_id_stdrt_not_null_select_el_ts,
                    impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_select_el_or_ident_with_id_stdrt_not_null_select_el_with_max_page_size_ts
                ) = {
                    let gen_ts = |dflt_some_one_or_dflt_some_one_with_max_page_size: &DefaultSomeOneOrDefaultSomeOneWithMaxPageSize|{
                        let vec_ts = {
                            let ts: &dyn ToTokens = match &dflt_some_one_or_dflt_some_one_with_max_page_size {
                                DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne => &PgCrudDfltOptSomeVecOneElCall,
                                DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize => &PgCrudDfltOptSomeVecOneElMaxPageSizeCall,
                            };
                            let els_ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                                let fi = &el0.ident;
                                let fi_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                                quote! {#SelfUcc::#fi_ucc_ts(#ts)}
                            });
                            quote! {#(#els_ts),*}
                        };
                        quote! {vec![#vec_ts]}
                    };
                    (
                        gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_ts(
                            &ident_select_el_or_ident_with_id_select_el_ucc,
                            &gen_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOne)
                        ),
                        gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_max_page_size_ts(
                            &ident_select_el_or_ident_with_id_select_el_ucc,
                            &gen_ts(&DefaultSomeOneOrDefaultSomeOneWithMaxPageSize::DefaultSomeOneWithMaxPageSize)
                        )
                    )
                };
                quote! {
                    #ident_select_el_or_ident_with_id_stdrt_not_null_select_el_ts
                    #impl_location_lib_to_err_string_for_ident_select_el_or_ident_with_id_stdrt_not_null_select_el_ts
                    #impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_select_el_or_ident_with_id_stdrt_not_null_select_el_ts
                    #impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_select_el_or_ident_with_id_stdrt_not_null_select_el_with_max_page_size_ts
                }
            };
            let mb_ident_select_el_ts = if is_stdrt_not_null { gen_ident_select_el_or_ident_with_id_stdrt_not_null_select_el_ts(&is_stdrt_with_id_false) } else { Ts2::new() };
            let mb_ident_with_id_stdrt_not_null_select_ts = if is_stdrt_not_null {
                let ident_with_id_stdrt_not_null_select_ts = gen_ident_select_stdrt_not_null_ts(&is_stdrt_with_id_true);
                let impl_ident_with_id_stdrt_not_null_select_ts = {
                    let impl_new_for_ident_with_id_stdrt_not_null_select_ts = gen_pub_const_new_v_type_cnt_self_v_ts(
                        &gen_unique_vec_w_ts(
                            &ident_with_id_stdrt_not_null_select_el_ucc
                        )
                    );
                    quote!{
                        impl #ident_with_id_stdrt_not_null_select_ucc {
                            #impl_new_for_ident_with_id_stdrt_not_null_select_ts
                        }
                    }
                };
                let impl_sqlx_type_for_ident_with_id_stdrt_not_null_select_ts = gen_sqlx_types_json_type_dcl_w_ts(&ident_with_id_stdrt_not_null_select_ucc);
                let impl_sqlx_decode_sqlx_pg_for_ident_with_id_stdrt_not_null_select_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_w_ts(&ident_with_id_stdrt_not_null_select_ucc);
                let impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_with_id_stdrt_not_null_select_ts = gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(
                    &ident_with_id_stdrt_not_null_select_ucc,
                    &Ts2::new(),
                    &impl_pg_crud_dflt_opt_some_vec_one_el_stdrt_not_null_ts
                );
                let impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_for_ident_with_id_stdrt_not_null_select_ts = gen_impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_ts(
                    &ident_with_id_stdrt_not_null_select_ucc,
                    &Ts2::new(),
                    &impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_stdrt_not_null_ts
                );
                let ident_with_id_select_el_ts = gen_ident_select_el_or_ident_with_id_stdrt_not_null_select_el_ts(&is_stdrt_with_id_true);
                quote! {
                    #ident_with_id_stdrt_not_null_select_ts
                    #impl_ident_with_id_stdrt_not_null_select_ts
                    #impl_sqlx_type_for_ident_with_id_stdrt_not_null_select_ts
                    #impl_sqlx_decode_sqlx_pg_for_ident_with_id_stdrt_not_null_select_ts
                    #impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_with_id_stdrt_not_null_select_ts
                    #impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_for_ident_with_id_stdrt_not_null_select_ts
                    #ident_with_id_select_el_ts
                }
            } else {
                Ts2::new()
            };
            quote! {
                #ident_select_ts
                #impl_ident_select_ts
                #impl_sqlx_type_for_ident_select_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_select_ts
                #impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_select_ts
                #impl_pg_crud_dflt_opt_some_vec_one_el_max_page_size_for_ident_select_ts
                #mb_ident_select_el_ts
                #mb_ident_with_id_stdrt_not_null_select_ts
            }
        };
        let ident_where_ucc = SelfWhereUcc::from_tokens(&ident);
        let ident_where_ts = match &is_nullable {
            IsNullable::False => {
                let gen_ident_where_field_vrts_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                    let vrts_ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                        let fi_ucc_ts = AsRefStrToUccTs::case_or_panic(&el0.ident.to_string());
                        let ft_as_json_type_where_ts = gen_type_as_pg_json_subtype_ts(
                            &el0.type0,
                            &pg_json_subtype_where
                        );
                        quote! {
                            #fi_ucc_ts(#import::PgTypeWhere<
                                #ft_as_json_type_where_ts
                            >)
                        }
                    });
                    quote! {#(#vrts_ts),*}
                };
                let gen_ident_where_ts = |
                    attrs_ts: &dyn ToTokens,
                    ident_ts_90ecb979: &dyn ToTokens,
                    ts_e1af2d89: &dyn ToTokens
                | {
                    let ts_60d5d187 = DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone()
                    .d_partial_eq()
                    .d_serde_serialize()
                    .d_serde_deserialize()
                    .d_utoipa_to_schema()
                    .d_schemars_json_schema()
                    .build_enum(
                        &ident_ts_90ecb979,
                        &Ts2::new(),
                        &quote!{{#ts_e1af2d89}}
                    );
                    quote!{
                        #attrs_ts
                        #ts_60d5d187
                    }
                };
                let equal_vrt_ident_ts = quote! {#EqualUcc(#import::PgJsonWhereEqual<#ident_as_pg_json_table_type_ts>)};
                let equal_vrt_qp_ts = quote!{
                    #SelfUcc::#EqualUcc(v_6781c7e3) => #import::PgTypeWhereFilter::qp(
                        v_6781c7e3,
                        #IncrSc,
                        &#ColumnSc,
                        add_oprtr
                    )
                };
                let equal_vrt_qb_ts = quote!{
                    #SelfUcc::#EqualUcc(#VSc) => #pg_type_where_filter_qb_v_query_ts
                };
                let mb_ident_where_ts = {
                    let gen_ident_where_w_ts = |ts: &dyn ToTokens| gen_ident_where_ts(
                        &AllowClippyArbitrarySrcItemOrdering,
                        &ident_where_ucc,
                        &ts
                    );
                    match &is_nullable {
                        IsNullable::False => match &pattern {
                            Pattern::Stdrt => gen_ident_where_w_ts(&{
                                let ident_where_field_vrts_ts = gen_ident_where_field_vrts_ts(&is_stdrt_with_id_false);
                                quote!{
                                    #ident_where_field_vrts_ts,
                                    #equal_vrt_ident_ts,
                                }
                            }),
                            Pattern::Arr => gen_ident_where_w_ts(&{
                                let dim_one_equal_ts = quote! {
                                    DimOneEqual(#import::PgJsonWhereDimOneEqual<#ident_with_id_stdrt_not_null_table_type_ucc>),
                                };
                                let length_equal_ts = quote! {
                                    LengthEqual(#import::PgJsonWhereLengthEqual),
                                };
                                let length_greater_than_ts = quote! {
                                    LengthGreaterThan(#import::PgJsonWhereLengthGreaterThan),
                                };
                                let in_ts = quote! {
                                    In(#import::PgJsonWhereIn<#ident_as_pg_json_table_type_ts>),
                                };
                                let dim_one_in_ts = quote! {
                                    DimOneIn(#import::PgJsonWhereDimOneIn<#ident_with_id_stdrt_not_null_table_type_ucc>),
                                };
                                let contains_all_els_of_arr_ts = quote! {
                                    ContainsAllElsOfArr(#import::PgJsonWhereContainsAllElsOfArr<#ident_with_id_stdrt_not_null_table_type_ucc>),
                                };
                                let overlaps_with_arr_ts = quote! {
                                    OverlapsWithArr(#import::PgJsonWhereOverlapsWithArr<#ident_with_id_stdrt_not_null_table_type_ucc>),
                                };
                                let el_filters_ts = vec_syn_field_with_id.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let el_fi_ucc = ElSelfUcc::from_tokens(&fi);
                                    let el_type_as_pg_json_where_ts = gen_type_as_pg_json_subtype_ts(
                                        &el0.type0,
                                        &PgJsonSubtype::Where
                                    );
                                    quote! {
                                        #el_fi_ucc(#import::PgTypeWhere<
                                            #el_type_as_pg_json_where_ts
                                        >)
                                    }
                                });
                                quote! {
                                    #equal_vrt_ident_ts,
                                    #dim_one_equal_ts
                                    #length_equal_ts
                                    #length_greater_than_ts
                                    #in_ts
                                    #dim_one_in_ts
                                    #contains_all_els_of_arr_ts
                                    #overlaps_with_arr_ts
                                    #(#el_filters_ts),*
                                }
                            }),
                        },
                        IsNullable::True => Ts2::new(),
                    }
                };
                let gen_where_filter_qp_fields_cnt_stdrt_not_null_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                    let qp_vrts_ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                        let fi_str = el0.ident.to_string();
                        let fi_ucc_ts = AsRefStrToUccTs::case_or_panic(&fi_str);
                        let ts = dq_ts(&format!("{{column}}->'{fi_str}'"));
                        quote! {
                            Self::#fi_ucc_ts(v_b93ffc1d) => #import::PgTypeWhereFilter::#QpSc(
                                v_b93ffc1d,
                                incr,
                                &format!(#ts),
                                add_oprtr,
                            )
                        }
                    });
                    quote! {#(#qp_vrts_ts),*}
                };
                let gen_where_filter_qb_fields_cnt_stdrt_not_null_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                    let qb_vrts_ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                        let fi_ucc_ts = AsRefStrToUccTs::case_or_panic(&el0.ident.to_string());
                        quote! {Self::#fi_ucc_ts(#VSc) => #pg_type_where_filter_qb_v_query_ts}
                    });
                    quote! {#(#qb_vrts_ts),*}
                };
                let gen_impl_pg_type_where_filter_ts = |
                    ident_ts_e0f20014: &dyn ToTokens,
                    qp_ts: &dyn ToTokens,
                    is_qb_mut: IsQbMut,
                    qb_ts: &dyn ToTokens
                | {
                    impl_pg_type_where_filter_for_ident_ts(
                        &quote! {<'lt>},
                        &ident_ts_e0f20014,
                        &Ts2::new(),
                        &IncrParamUndrscr::False,
                        &ColumnParamUndrscr::False,
                        &AddOprtrUndrscr::False,
                        &qp_ts,
                        &is_qb_mut,
                        &qb_ts,
                        &Import::PgCrud,
                    )
                };
                let mb_impl_pg_crud_pg_type_pg_type_where_filter_for_ident_where_ts = {
                    let gen_impl_pg_type_where_filter_for_ident_ts = |qp_ts: &dyn ToTokens, is_qb_mut: IsQbMut, qb_ts: &dyn ToTokens| gen_impl_pg_type_where_filter_ts(&ident_where_ucc, &qp_ts, is_qb_mut, &qb_ts);
                    match &pattern {
                        Pattern::Stdrt => match &is_nullable {
                            IsNullable::False => gen_impl_pg_type_where_filter_for_ident_ts(
                                &{
                                    let fields_ts = gen_where_filter_qp_fields_cnt_stdrt_not_null_ts(&is_stdrt_with_id_false);
                                    quote!{
                                        match &self {
                                            #fields_ts,
                                            #equal_vrt_qp_ts,
                                        }
                                    }
                                },
                                IsQbMut::False,
                                &{
                                    let fields_ts = gen_where_filter_qb_fields_cnt_stdrt_not_null_ts(&is_stdrt_with_id_false);
                                    quote!{
                                        match self {
                                            #fields_ts,
                                            #equal_vrt_qb_ts,
                                        }
                                    }
                                }
                            ),
                            IsNullable::True => Ts2::new(),
                        },
                        Pattern::Arr => gen_impl_pg_type_where_filter_for_ident_ts(
                            &{
                                let el_filters_ts = vec_syn_field_with_id.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let el_fi_ucc = ElSelfUcc::from_tokens(&fi);
                                    let fi_dq_ts = dq_ts(&fi);
                                    quote! {
                                        Self::#el_fi_ucc(v_5ff7ccdf) => gen_el_query(
                                            v_5ff7ccdf.get_oprtr(),
                                            v_5ff7ccdf,
                                            #fi_dq_ts
                                        )
                                    }
                                });
                                let concrete_filters_ts = [
                                    quote!{#EqualUcc},
                                    quote!{#DimOneEqualUcc},
                                    quote!{#LengthEqualUcc},
                                    quote!{#LengthGreaterThanUcc},
                                    quote!{#InUcc},
                                    quote!{#DimOneInUcc},
                                    quote!{#ContainsAllElsOfArrUcc},
                                    quote!{#OverlapsWithArrUcc}
                                ].into_iter().map(|el_ts|quote!{
                                    Self::#el_ts(v_df049001) => #import::PgTypeWhereFilter::#QpSc(
                                        v_df049001,
                                        #IncrSc,
                                        #ColumnSc,
                                        #AddOprtrSc
                                    ),
                                });
                                quote! {
                                    let mut gen_el_query = |
                                        oprtr: &#import::Oprtr,
                                        v_637adcbd: &dyn #import::PgTypeWhereFilter<'_>,
                                        field: &str
                                    | -> Result<#StringTs, #import_qp_er_ts> {
                                        let oprtr_qp = oprtr.to_qp(add_oprtr);
                                        let elem = "elem";
                                        let v_9696ee60 = match #import::PgTypeWhereFilter::#QpSc(
                                            v_637adcbd,
                                            #IncrSc,
                                            &format!("{elem}->'{field}'"),
                                            false
                                        ) {
                                            Ok(v_c7ec4e53) => v_c7ec4e53,
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            }
                                        };
                                        Ok(format!("{oprtr_qp}(exists (select 1 from jsonb_array_elements({column}) as {elem} where {v_9696ee60}))"))
                                    };
                                    match &self {
                                        #(#concrete_filters_ts)*
                                        #(#el_filters_ts),*
                                    }
                                }
                            },
                            IsQbMut::False,
                            &{
                                let el_filters_ts = vec_syn_field_with_id.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let el_fi_ucc = ElSelfUcc::from_tokens(&fi);
                                    quote! {Self::#el_fi_ucc(#VSc) => #pg_type_where_filter_qb_v_query_ts}
                                });
                                quote! {
                                    match self {
                                        Self::Equal(#VSc) => #pg_type_where_filter_qb_v_query_ts,
                                        Self::DimOneEqual(#VSc) => #pg_type_where_filter_qb_v_query_ts,
                                        Self::LengthEqual(#VSc) => #pg_type_where_filter_qb_v_query_ts,
                                        Self::LengthGreaterThan(#VSc) => #pg_type_where_filter_qb_v_query_ts,
                                        Self::In(#VSc) => #pg_type_where_filter_qb_v_query_ts,
                                        Self::DimOneIn(#VSc) => #pg_type_where_filter_qb_v_query_ts,
                                        Self::ContainsAllElsOfArr(#VSc) => #pg_type_where_filter_qb_v_query_ts,
                                        Self::OverlapsWithArr(#VSc) => #pg_type_where_filter_qb_v_query_ts,
                                        #(#el_filters_ts),*
                                    }
                                }
                            },
                        ),
                    }
                };
                let mb_impl_location_lib_to_err_string_for_ident_where_ts = if matches!((&pattern, &is_nullable), (Pattern::Stdrt, IsNullable::True)) {
                    Ts2::new()
                } else {
                    gen_gen_impl_location_lib_to_err_string_w_ts(&ident_where_ucc)
                };
                let gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_cnt_stdrt_not_null_where = |is_stdrt_with_id: &IsStdrtWithId| {
                    let gen_self_vrt_dflt_some_one_ts = |ts: &dyn ToTokens|quote!{
                        Self::#ts(#PgCrudDfltOptSomeVecOneElCall)
                    };
                    let vrts_ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                        gen_self_vrt_dflt_some_one_ts(&AsRefStrToUccTs::case_or_panic(&el0.ident.to_string()))
                    });
                    let self_equal_dflt_some_one_ts = gen_self_vrt_dflt_some_one_ts(&EqualUcc);
                    quote! {vec![
                        #(#vrts_ts),*,
                        #self_equal_dflt_some_one_ts
                    ]}
                };
                let mb_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_where_ts = match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_ts(&ident_where_ucc, &gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_cnt_stdrt_not_null_where(&is_stdrt_with_id_false)),
                        IsNullable::True => Ts2::new(),
                    },
                    Pattern::Arr => gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_ts(&ident_where_ucc, &{
                        let el_filters_ts = vec_syn_field_with_id.iter().map(|el0| {
                            let fi = &el0.ident;
                            let el_fi_ucc = ElSelfUcc::from_tokens(&fi);
                            quote! {Self::#el_fi_ucc(#dflt_but_opt_is_some_call_ts)}
                        });
                        quote! {
                            vec![
                                Self::Equal(#dflt_but_opt_is_some_call_ts),
                                Self::DimOneEqual(#dflt_but_opt_is_some_call_ts),
                                Self::LengthEqual(#dflt_but_opt_is_some_call_ts),
                                Self::LengthGreaterThan(#dflt_but_opt_is_some_call_ts),
                                Self::In(#dflt_but_opt_is_some_call_ts),
                                Self::DimOneIn(#dflt_but_opt_is_some_call_ts),
                                Self::ContainsAllElsOfArr(#dflt_but_opt_is_some_call_ts),
                                Self::OverlapsWithArr(#dflt_but_opt_is_some_call_ts),
                                #(#el_filters_ts),*
                            ]
                        }
                    }),
                };
                let mb_ident_with_id_stdrt_not_null_where_ts = if is_stdrt_not_null {
                    let ident_with_id_stdrt_not_null_where_ts = gen_ident_where_ts(
                        &AllowClippyArbitrarySrcItemOrdering,
                        &ident_with_id_stdrt_not_null_where_ucc,
                        &{
                            let ident_where_field_vrts_ts = gen_ident_where_field_vrts_ts(&is_stdrt_with_id_true);
                            quote!{
                                #ident_where_field_vrts_ts,
                                #EqualUcc(#import::PgJsonWhereEqual<#ident_with_id_stdrt_not_null_table_type_ucc>),//todo mb reuse? vrt generation
                            }
                        }
                    );
                    let impl_pg_crud_pg_type_pg_type_where_filter_for_ident_with_id_stdrt_not_null_where_ts = gen_impl_pg_type_where_filter_ts(
                        &ident_with_id_stdrt_not_null_where_ucc,
                        &{
                            let fields_ts = gen_where_filter_qp_fields_cnt_stdrt_not_null_ts(&is_stdrt_with_id_true);
                            quote!{
                                match &self {
                                    #fields_ts,
                                    Self::#EqualUcc(v_31e7fe47) => pg_crud::PgTypeWhereFilter::qp(
                                        v_31e7fe47,
                                        #IncrSc,
                                        &#ColumnSc,
                                        add_oprtr
                                    ),//todo mb reuse? vrt generation
                                }
                            }
                        },
                        IsQbMut::False,
                        &{
                            let fields_ts = gen_where_filter_qb_fields_cnt_stdrt_not_null_ts(&is_stdrt_with_id_true);
                            quote!{
                                match self {
                                    #fields_ts,
                                    Self::#EqualUcc(v_45b5a7f0) => pg_crud::PgTypeWhereFilter::qb(v_45b5a7f0, #QuerySc),//todo mb reuse? vrt generation
                                }
                            }
                        },
                    );
                    let impl_location_lib_to_err_string_for_ident_with_id_stdrt_not_null_where_ts = gen_gen_impl_location_lib_to_err_string_w_ts(&ident_with_id_stdrt_not_null_where_ucc);
                    let impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_with_id_stdrt_not_null_where_ts = gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_ts(
                        &ident_with_id_stdrt_not_null_where_ucc,
                        &gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_cnt_stdrt_not_null_where(&is_stdrt_with_id_true)
                    );
                    quote! {
                        #ident_with_id_stdrt_not_null_where_ts
                        #impl_pg_crud_pg_type_pg_type_where_filter_for_ident_with_id_stdrt_not_null_where_ts
                        #impl_location_lib_to_err_string_for_ident_with_id_stdrt_not_null_where_ts
                        #impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_with_id_stdrt_not_null_where_ts
                    }
                } else {
                    Ts2::new()
                };
                quote! {
                    #mb_ident_where_ts
                    #mb_impl_pg_crud_pg_type_pg_type_where_filter_for_ident_where_ts
                    #mb_impl_location_lib_to_err_string_for_ident_where_ts
                    #mb_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_where_ts
                    #mb_ident_with_id_stdrt_not_null_where_ts
                }
            }
            IsNullable::True => {
                let ident_stdrt_or_ident_with_id_arr_as_pg_json_where_ts = gen_type_as_pg_json_subtype_ts(
                    &match &pattern {
                        Pattern::Stdrt => &ident_stdrt_not_null_ucc,
                        Pattern::Arr => &ident_with_id_arr_not_null_ucc,
                    },
                    &pg_json_subtype_where
                );
                quote! {
                    pub type #ident_where_ucc = #import::NullableJsonObjectPgTypeWhereFilter<
                        #ident_stdrt_or_ident_with_id_arr_as_pg_json_where_ts
                    >;
                }
            }
        };
        let gen_fi_dq_ts = |v: &SynField| {
            dq_ts(&v.ident)
        };
        let gen_type_as_pg_json_read_ts = |ts: &dyn ToTokens| gen_type_as_pg_json_subtype_ts(&ts, &pg_json_subtype_read);
        let gen_type_as_pg_json_read_inn_ts = |ts: &dyn ToTokens| gen_type_as_pg_json_subtype_ts(&ts, &pg_json_subtype_read_inn);
        let gen_ident_or_ident_with_id_read_or_read_inn_fields_dcl_ts = |
            is_stdrt_with_id: &IsStdrtWithId,
            read_with_or_without_ann_or_inn: &ReadWithOrWithoutAnnOrInn
        | {
            let ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                let mb_serde_skip_serializing_if_opt_is_none_ts = match &read_with_or_without_ann_or_inn {
                    ReadWithOrWithoutAnnOrInn::WithSerdeOptIsNoneAnn => quote! {#[serde(skip_serializing_if = "Option::is_none")]},
                    ReadWithOrWithoutAnnOrInn::WithoutSerdeOptIsNoneAnn |
                    ReadWithOrWithoutAnnOrInn::Inn => Ts2::new(),
                };
                let fi = &el0.ident;
                let ft_as_json_type_read_ts = match &read_with_or_without_ann_or_inn {
                    ReadWithOrWithoutAnnOrInn::Inn => gen_type_as_pg_json_read_inn_ts(
                        &el0.type0
                    ),
                    ReadWithOrWithoutAnnOrInn::WithSerdeOptIsNoneAnn |
                    ReadWithOrWithoutAnnOrInn::WithoutSerdeOptIsNoneAnn => gen_type_as_pg_json_read_ts(
                        &el0.type0
                    ),
                };
                let opt_v_ft_as_json_type_read_ts = gen_opt_type_dcl_ts(
                    &gen_v_dcl_ts0(&ft_as_json_type_read_ts)
                );
                quote! {
                    #mb_serde_skip_serializing_if_opt_is_none_ts
                    #fi: #opt_v_ft_as_json_type_read_ts
                }
            });
            quote! {#(#ts),*}
        };
        let ident_read_ucc = SelfReadUcc::from_tokens(&ident);
        let ident_with_id_stdrt_not_null_read_ucc = SelfReadUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let ident_read_inn_ucc = SelfReadInnUcc::from_tokens(&ident);
        let ident_with_id_stdrt_not_null_read_inn_ucc = SelfReadInnUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let ident_read_ts = {
            let ident_read_try_from_er_ucc = SelfReadTryFromErUcc::from_tokens(&ident);
            let ident_with_id_stdrt_not_null_read_try_from_er_ucc = SelfReadTryFromErUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
            let ident_stdrt_not_null_as_pg_json_read_ts = gen_type_as_pg_json_read_ts(&ident_stdrt_not_null_ucc);
            let ident_with_id_arr_not_null_as_pg_json_read_ts = gen_type_as_pg_json_read_ts(&ident_with_id_arr_not_null_ucc);
            let gen_ident_read_ts = |
                ident_ts_fc625e96: &dyn ToTokens,
                ts_1c85ea2c: &dyn ToTokens,
                derive_serde_deserialize: DSerdeDeserialize
            | {
                let ts_3a67b41f = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize_if(derive_serde_deserialize)
                .d_utoipa_to_schema()
                .d_schemars_json_schema()
                .build_struct(
                    &ident_ts_fc625e96,
                    &Ts2::new(),
                    &ts_1c85ea2c
                );
                quote!{
                    #AllowClippyArbitrarySrcItemOrdering
                    #ts_3a67b41f
                }
            };
            let ident_read_ts = {
                let (ts, derive_serde_deserialize) = match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => (
                            {
                                let ts = gen_ident_or_ident_with_id_read_or_read_inn_fields_dcl_ts(
                                    &is_stdrt_with_id_false,
                                    &ReadWithOrWithoutAnnOrInn::WithSerdeOptIsNoneAnn
                                );
                                quote! {{#ts}}
                            },
                            DSerdeDeserialize::False,
                        ),
                        IsNullable::True => (wrap_into_scopes_dot_comma_ts(&gen_opt_type_dcl_ts(&ident_stdrt_not_null_as_pg_json_read_ts)), DSerdeDeserialize::True),
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => (wrap_into_scopes_dot_comma_ts(&gen_vec_tokens_dcl_ts(&ident_with_id_stdrt_not_null_read_ucc)), DSerdeDeserialize::True),
                        IsNullable::True => (wrap_into_scopes_dot_comma_ts(&gen_opt_type_dcl_ts(&ident_with_id_arr_not_null_as_pg_json_read_ts)), DSerdeDeserialize::True),
                    },
                };
                gen_ident_read_ts(&ident_read_ucc, &ts, derive_serde_deserialize)
            };
            let gen_ident_read_try_from_er_ts = |ts: &dyn ToTokens|DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_serde_serialize()
                .d_serde_deserialize()
                .d_thiserror_error()
                .d_location_lib_location()
                .build_enum(
                    &ts,
                    &Ts2::new(),
                    &quote!{{
                        #AllFieldsAreNoneUcc {
                            loc: location_lib::loc::Loc,
                        },
                    }}
                );
            let mb_ident_read_try_from_er_ts = match &pattern {
                Pattern::Stdrt => match &is_nullable {
                    IsNullable::False => gen_ident_read_try_from_er_ts(&ident_read_try_from_er_ucc),
                    IsNullable::True => Ts2::new(),
                },
                Pattern::Arr => Ts2::new(),
            };
            let gen_ident_read_or_ident_with_id_stdrt_not_null_read_ucc = |is_stdrt_with_id: &IsStdrtWithId| match &is_stdrt_with_id {
                IsStdrtWithId::False => &ident_read_ucc,
                IsStdrtWithId::True => &ident_with_id_stdrt_not_null_read_ucc,
            };
            let gen_pub_try_new_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                let ident_read_try_from_er_or_ident_with_id_stdrt_not_null_read_try_from_er_ucc: &dyn ToTokens = match &is_stdrt_with_id {
                    IsStdrtWithId::False => &ident_read_try_from_er_ucc,
                    IsStdrtWithId::True => &ident_with_id_stdrt_not_null_read_try_from_er_ucc,
                };
                gen_pub_try_new_ts(
                    &Ts2::new(),
                    &gen_ident_or_ident_with_id_read_or_read_inn_fields_dcl_ts(
                        is_stdrt_with_id,
                        &ReadWithOrWithoutAnnOrInn::WithoutSerdeOptIsNoneAnn
                    ),
                    &ident_read_try_from_er_or_ident_with_id_stdrt_not_null_read_try_from_er_ucc,
                    &{
                        let vec_syn_field_19e98ce1 = get_vec_syn_field(is_stdrt_with_id);
                        let (fields_reference_ts, fields_ts) = {
                            enum WithReference {
                                False,
                                True,
                            }
                            let gen_ts = |with_reference: &WithReference| {
                                let mb_reference_symbol_ts = match &with_reference {
                                    WithReference::False => Ts2::new(),
                                    WithReference::True => quote! {&},
                                };
                                let fields_ts = vec_syn_field_19e98ce1.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    quote! {#mb_reference_symbol_ts #fi}
                                });
                                quote! {#(#fields_ts),*}
                            };
                            (gen_ts(&WithReference::True), gen_ts(&WithReference::False))
                        };
                        let check_if_all_fields_are_none_ts = {
                            let vec_syn_field_19e98ce1_len = vec_syn_field_19e98ce1.len();
                            let mb_wrap_into_braces_h_ts = |ts: &dyn ToTokens| mb_wrap_into_braces_ts(
                                ts,
                                vec_syn_field_19e98ce1_len > 1
                            );
                            let left_ts = mb_wrap_into_braces_h_ts(&fields_reference_ts);
                            let right_ts = mb_wrap_into_braces_h_ts(&{
                                let nones_ts = repeat_with(||quote!{None}).take(vec_syn_field_19e98ce1_len);
                                quote! {#(#nones_ts),*}
                            });
                            let ts = if vec_syn_field_19e98ce1_len == 1 {
                                let ts = mb_wrap_into_braces_h_ts(&fields_ts);
                                quote! {#ts.is_none()}
                            }
                            else {
                                quote! {matches!(#left_ts, #right_ts)}
                            };
                            quote! {
                                if #ts {
                                    return Err(#ident_read_try_from_er_or_ident_with_id_stdrt_not_null_read_try_from_er_ucc::#AllFieldsAreNoneUcc {
                                        loc: location_lib::loc!()
                                    });
                                }
                            }
                        };
                        quote!{
                            #check_if_all_fields_are_none_ts
                            Ok(Self{#fields_ts})
                        }
                    }
                )
            };
            let impl_ident_read_ts = {
                let pub_new_or_try_new_ts = {
                    let vec_ident_with_id_stdrt_not_null_read_ts = gen_vec_tokens_dcl_ts(&ident_with_id_stdrt_not_null_read_ucc);
                    match &pattern {
                        Pattern::Stdrt => match &is_nullable {
                            IsNullable::False => gen_pub_try_new_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts(&is_stdrt_with_id_false),
                            IsNullable::True => gen_pub_const_new_ts(
                                &MustUse,
                                &gen_v_type_ts(
                                    &gen_opt_type_dcl_ts(
                                        &ident_stdrt_not_null_as_pg_json_read_ts
                                    )
                                ),
                                &self_v_ts
                            ),
                        },
                        Pattern::Arr => match &is_nullable {
                            IsNullable::False => gen_pub_const_new_ts(
                                &MustUse,
                                &gen_v_type_ts(
                                    &vec_ident_with_id_stdrt_not_null_read_ts
                                ),
                                &self_v_ts
                            ),
                            IsNullable::True => gen_pub_new_ts(
                                &MustUse,
                                &gen_v_type_ts(
                                    &gen_opt_type_dcl_ts(
                                        &vec_ident_with_id_stdrt_not_null_read_ts
                                    )
                                ),
                                &quote! {Self(#VSc.map(#ident_with_id_arr_not_null_as_pg_json_read_ts::new))},
                            ),
                        },
                    }
                };
                quote!{
                    impl #ident_read_ucc {
                        #pub_new_or_try_new_ts
                    }
                }
            };
            let gen_impl_de_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                let vec_syn_field_13a9b1ed = get_vec_syn_field(is_stdrt_with_id);
                gen_impl_de_for_struct_ts(
                    &gen_ident_read_or_ident_with_id_stdrt_not_null_read_ucc(is_stdrt_with_id),
                    &vec_syn_field_13a9b1ed.iter().map(|el0|
                        (&el0.ident, &el0.type0)
                    ).collect::<Vec<(&Ident, &Type)>>(),
                    vec_syn_field_13a9b1ed.len(),
                    &|_: &Ident, syn_type: &Type| {
                        let type_read_ts = gen_type_as_pg_json_read_ts(&syn_type);
                        gen_opt_type_dcl_ts(
                            &gen_v_dcl_ts0(&type_read_ts)
                        )
                    }
                )
            };
            let mb_impl_de_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts = match &pattern {
                Pattern::Stdrt => match &is_nullable {
                    IsNullable::False => gen_impl_de_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts(&is_stdrt_with_id_false),
                    IsNullable::True => Ts2::new(),
                },
                Pattern::Arr => Ts2::new(),
            };
            let gen_impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(&gen_ident_read_or_ident_with_id_stdrt_not_null_read_ucc(is_stdrt_with_id), &Ts2::new(), &{
                    let fields_ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                        let fi = &el0.ident;
                        let ts = gen_v_init_ts0(
                            &PgCrudDfltOptSomeVecOneElCall
                        );
                        quote! {#fi: Some(#ts)}
                    });
                    quote! {Self{#(#fields_ts),*}}
                })
            };
            let impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts = match &pattern {
                Pattern::Stdrt => match &is_nullable {
                    IsNullable::False => gen_impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts(&is_stdrt_with_id_false),
                    IsNullable::True => gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(&ident_read_ucc, &Ts2::new(), &self_some_pg_crud_dflt_opt_some_vec_one_el_call_ts),
                },
                Pattern::Arr => match &is_nullable {
                    IsNullable::False => gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(
                        &ident_read_ucc,
                        &Ts2::new(),
                        &quote! {
                            Self(#vec_pg_crud_dflt_opt_some_vec_one_el_call_ts)
                        },
                    ),
                    IsNullable::True => gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(&ident_read_ucc, &Ts2::new(), &self_some_pg_crud_dflt_opt_some_vec_one_el_call_ts),
                },
            };
            let impl_sqlx_type_for_ident_read_ts = gen_sqlx_types_json_type_dcl_w_ts(&ident_read_ucc);
            let impl_sqlx_encode_sqlx_pg_for_ident_read_ts = gen_impl_sqlx_encode_sqlx_pg_for_ident_ts(
                &ident_read_ucc,
                &quote!{sqlx::types::Json(#SelfSc)}
            );
            let impl_sqlx_decode_sqlx_pg_for_ident_read_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_w_ts(&ident_read_ucc);
            let mb_ident_with_id_read_ts = if is_stdrt_not_null {
                let ident_with_id_stdrt_not_null_read_ts = gen_ident_read_ts(
                    &ident_with_id_stdrt_not_null_read_ucc,
                    &{
                        let ts = gen_ident_or_ident_with_id_read_or_read_inn_fields_dcl_ts(
                            &is_stdrt_with_id_true,
                            &ReadWithOrWithoutAnnOrInn::WithSerdeOptIsNoneAnn
                        );
                        quote! {{#ts}}
                    },
                    DSerdeDeserialize::False,
                );
                let ident_with_id_stdrt_not_null_read_try_from_er_ts = gen_ident_read_try_from_er_ts(&ident_with_id_stdrt_not_null_read_try_from_er_ucc);
                let impl_ident_with_id_stdrt_not_null_read_ts = {
                    let pub_try_new_ts = gen_pub_try_new_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts(&is_stdrt_with_id_true);
                    quote!{
                        impl #ident_with_id_stdrt_not_null_read_ucc {
                            #pub_try_new_ts
                        }
                    }
                };
                let impl_de_for_ident_with_id_stdrt_not_null_read_ts = gen_impl_de_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts(&is_stdrt_with_id_true);
                let impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_with_id_stdrt_not_null_read_ts = gen_impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts(&is_stdrt_with_id_true);
                let impl_sqlx_type_for_ident_with_id_stdrt_not_null_read_ts = gen_sqlx_types_json_type_dcl_w_ts(&ident_with_id_stdrt_not_null_read_ucc);
                let impl_sqlx_decode_sqlx_pg_for_ident_with_id_stdrt_not_null_read_ts = gen_impl_sqlx_decode_sqlx_pg_for_ident_w_ts(&ident_with_id_stdrt_not_null_read_ucc);
                quote! {
                    #ident_with_id_stdrt_not_null_read_ts
                    #ident_with_id_stdrt_not_null_read_try_from_er_ts
                    #impl_ident_with_id_stdrt_not_null_read_ts
                    #impl_de_for_ident_with_id_stdrt_not_null_read_ts
                    #impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_with_id_stdrt_not_null_read_ts
                    #impl_sqlx_type_for_ident_with_id_stdrt_not_null_read_ts
                    #impl_sqlx_decode_sqlx_pg_for_ident_with_id_stdrt_not_null_read_ts
                }
            } else {
                Ts2::new()
            };
            quote! {
                #ident_read_ts
                #mb_ident_read_try_from_er_ts
                #impl_ident_read_ts
                #mb_impl_de_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts
                #impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts
                #impl_sqlx_type_for_ident_read_ts
                #impl_sqlx_encode_sqlx_pg_for_ident_read_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_read_ts
                #mb_ident_with_id_read_ts
            }
        };
        let ident_with_id_stdrt_not_null_read_ids_h_ucc = SelfReadIdsHUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let ident_stdrt_not_null_read_ids_ucc = SelfReadIdsUcc::from_tokens(&ident_stdrt_not_null_ucc);
        let ident_read_ids_ucc = SelfReadIdsUcc::from_tokens(&ident);
        let ident_read_ids_h_ucc = SelfReadIdsHUcc::from_tokens(&ident);
        let gen_ident_read_ids_or_ident_with_id_read_ids_ts = |is_stdrt_with_id: &IsStdrtWithId| {
            let ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                let fi = &el0.ident;
                let ft_as_pg_json_read_ids_ts = gen_type_as_pg_json_subtype_ts(
                    &el0.type0,
                    &PgJsonSubtype::ReadIds
                );
                quote! {#fi: #ft_as_pg_json_read_ids_ts}
            });
            quote! {{#(#ts),*}}
        };
        let gen_impl_sqlx_decode_ts = |ts: &dyn ToTokens|{
            gen_impl_sqlx_decode_sqlx_pg_for_ident_ts(
                &ts,
                &quote!{sqlx::types::Json<Self>},
                &quote!{Ok(v.0)}
            )
        };
        let gen_impl_sqlx_type_ts = |ts: &dyn ToTokens|{
            gen_impl_sqlx_type_for_ident_ts(
                &ts,
                &quote!{sqlx::types::Json<Self>}
            )
        };
        let gen_fields_read_ids_into_opt_v_read_inn_ts = |is_stdrt_with_id: &IsStdrtWithId, params_ts: &dyn ToTokens|{
            let ts = gen_v_init_ts0(&{
                let ident_ts_6ad36393: &dyn ToTokens = match &is_stdrt_with_id {
                    IsStdrtWithId::False => &ident_stdrt_not_null_read_inn_ucc,
                    IsStdrtWithId::True => &ident_with_id_stdrt_not_null_read_inn_ucc,
                };
                let ts0 = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                    let fi = &el0.ident;
                    let ft = &el0.type0;
                    let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&ft);
                    let ft_as_pg_json_read_ts = gen_type_as_pg_json_subtype_ts(&ft, &PgJsonSubtype::Read);
                    let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                    let ts1 = gen_v_init_ts0(&{
                        let dflt_but_opt_is_some_call_ts_f378bbab = gen_ident_as_dflt_but_opt_is_some_call_ts(
                            &ft_as_pg_json_read_ts
                        );
                        quote!{#ft_as_pg_json_ts::into_inn(#dflt_but_opt_is_some_call_ts_f378bbab)}
                    });
                    quote! {
                        #fi: #ft_as_pg_json_test_cases_ts::read_ids_into_opt_v_read_inn(
                            #params_ts.0.#VSc.#fi
                        ).map_or_else(|| Some(#ts1), Some)
                    }
                });
                quote!{ #ident_ts_6ad36393 { #(#ts0),* } }
            });
            quote!{Some(#ts)}
        };
        let ident_read_ids_ts = {
            let mb_ident_read_ids_h_ts = if is_stdrt_not_null {
                let ts_1e087f4d = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .build_struct(
                    &ident_read_ids_h_ucc,
                    &Ts2::new(),
                    &gen_ident_read_ids_or_ident_with_id_read_ids_ts(&IsStdrtWithId::False)
                );
                quote!{
                    #AllowClippyArbitrarySrcItemOrdering
                    #ts_1e087f4d
                }
            }
            else {
                Ts2::new()
            };
            let ident_read_ids_ts = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize()
                .build_struct(
                    &ident_read_ids_ucc,
                    &Ts2::new(),
                    &{
                        let ts = gen_v_dcl_ts0(&match &is_nullable {
                            IsNullable::False => match &pattern {
                                Pattern::Stdrt => quote!{#ident_read_ids_h_ucc},
                                Pattern::Arr => gen_vec_tokens_dcl_ts(
                                    &ident_with_id_stdrt_not_null_read_ids_ucc
                                ),
                            },
                            IsNullable::True => gen_opt_type_dcl_ts(&{
                                let ts0: &dyn ToTokens = match &pattern {
                                    Pattern::Stdrt => &ident_stdrt_not_null_read_ids_ucc,
                                    Pattern::Arr => &SelfReadIdsUcc::from_tokens(&gen_ident_ucc(&IdentPattern::ArrNotNullWithId)),
                                };
                                quote!{#ts0}
                            })
                        });
                        quote!{(#ts);}
                    }
                );
            let impl_sqlx_decode_sqlx_pg_for_ident_read_ids_ts = gen_impl_sqlx_decode_ts(&ident_read_ids_ucc);
            let impl_sqlx_type_for_ident_read_ids_ts = gen_impl_sqlx_type_ts(&ident_read_ids_ucc);
            let mb_ident_with_id_stdrt_not_null_read_ids_ts = if is_stdrt_not_null {
                let ident_with_id_stdrt_not_null_read_ids_ts = {
                    let ident_with_id_stdrt_not_null_read_ids_h_ts = {
                        let ts_fe644945 = DTsBuilder::new()
                        .make_pub()
                        .d_debug()
                        .d_clone()
                        .d_partial_eq()
                        .d_serde_serialize()
                        .d_serde_deserialize()
                        .build_struct(
                            &ident_with_id_stdrt_not_null_read_ids_h_ucc,
                            &Ts2::new(),
                            &gen_ident_read_ids_or_ident_with_id_read_ids_ts(&IsStdrtWithId::True)
                        );
                        quote!{
                            #AllowClippyArbitrarySrcItemOrdering
                            #ts_fe644945
                        }
                    };
                    let ident_with_id_stdrt_not_null_read_ids_ts = DTsBuilder::new()
                        .make_pub()
                        .d_debug()
                        .d_clone()
                        .d_partial_eq()
                        .d_serde_serialize()
                        .d_serde_deserialize()
                        .build_struct(
                            &ident_with_id_stdrt_not_null_read_ids_ucc,
                            &Ts2::new(),
                            &{
                                let ts = gen_v_dcl_ts0(
                                    &ident_with_id_stdrt_not_null_read_ids_h_ucc
                                );
                                quote!{(pub #ts);}
                            }
                        );
                    quote! {
                        #ident_with_id_stdrt_not_null_read_ids_h_ts
                        #ident_with_id_stdrt_not_null_read_ids_ts
                    }
                };
                let impl_sqlx_decode_sqlx_pg_for_ident_with_id_stdrt_not_null_read_ids_ts = gen_impl_sqlx_decode_ts(&ident_with_id_stdrt_not_null_read_ids_ucc);
                let impl_sqlx_type_for_ident_with_id_stdrt_not_null_read_ids_ts = gen_impl_sqlx_type_ts(&ident_with_id_stdrt_not_null_read_ids_ucc);
                quote! {
                    #ident_with_id_stdrt_not_null_read_ids_ts
                    #impl_sqlx_decode_sqlx_pg_for_ident_with_id_stdrt_not_null_read_ids_ts
                    #impl_sqlx_type_for_ident_with_id_stdrt_not_null_read_ids_ts
                }
            } else {
                Ts2::new()
            };
            quote! {
                #mb_ident_read_ids_h_ts
                #ident_read_ids_ts
                #impl_sqlx_decode_sqlx_pg_for_ident_read_ids_ts
                #impl_sqlx_type_for_ident_read_ids_ts
                #mb_ident_with_id_stdrt_not_null_read_ids_ts
            }
        };
        let ident_read_inn_ts = {
            let gen_ident_read_inn_or_ident_with_id_stdrt_not_null_read_inn_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                let ts_3d7e760e = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .build_struct(
                    match &is_stdrt_with_id {
                        IsStdrtWithId::False => &ident_read_inn_ucc,
                        IsStdrtWithId::True => &ident_with_id_stdrt_not_null_read_inn_ucc,
                    },
                    &Ts2::new(),
                    &{
                        let ts = gen_ident_or_ident_with_id_read_or_read_inn_fields_dcl_ts(
                            is_stdrt_with_id,
                            &ReadWithOrWithoutAnnOrInn::Inn
                        );
                        quote!{{#ts}}
                    }
                );
                quote!{
                    #AllowClippyArbitrarySrcItemOrdering
                    #ts_3d7e760e
                }
            };
            let ident_read_inn_ts = {
                let gen_pub_type_ident_read_inn_al_ts = |ts: &dyn ToTokens| gen_pub_type_al_ts(&ident_read_inn_ucc, &ts);
                match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => gen_ident_read_inn_or_ident_with_id_stdrt_not_null_read_inn_ts(&IsStdrtWithId::False),
                        IsNullable::True => gen_pub_type_ident_read_inn_al_ts(&gen_opt_type_dcl_ts(&gen_type_as_pg_json_read_inn_ts(&ident_stdrt_not_null_ucc))),
                    },
                    Pattern::Arr => gen_pub_type_ident_read_inn_al_ts(&match &is_nullable {
                        IsNullable::False => gen_vec_tokens_dcl_ts(
                            &ident_with_id_stdrt_not_null_read_inn_ucc
                        ),
                        IsNullable::True => gen_opt_type_dcl_ts(&gen_type_as_pg_json_read_inn_ts(&ident_with_id_arr_not_null_ucc)),
                    }),
                }
            };
            let mb_ident_with_id_read_inn_ts = if is_stdrt_not_null {
                let ident_with_id_read_inn_ts = gen_ident_read_inn_or_ident_with_id_stdrt_not_null_read_inn_ts(&IsStdrtWithId::True);
                quote! {
                    #ident_with_id_read_inn_ts
                }
            } else {
                Ts2::new()
            };
            quote! {
                #ident_read_inn_ts
                #mb_ident_with_id_read_inn_ts
            }
        };
        let ident_upd_ucc = SelfUpdUcc::from_tokens(&ident);
        let ident_stdrt_not_null_upd_el_ucc = &SelfUpdElUcc::from_tokens(&ident_stdrt_not_null_ucc);
        let ident_stdrt_not_null_upd_for_query_el_ucc = &SelfUpdForQueryElUcc::from_tokens(&ident_stdrt_not_null_ucc);
        let ident_upd_el_ucc = &SelfUpdElUcc::from_tokens(&ident);
        let ident_upd_for_query_el_ucc = &SelfUpdForQueryElUcc::from_tokens(&ident);
        let ident_stdrt_not_null_as_pg_json_upd_ts = gen_type_as_pg_json_upd_ts(&ident_stdrt_not_null_ucc);
        let ident_stdrt_not_null_as_pg_json_upd_for_query_ts = gen_type_as_pg_json_upd_for_query_ts(&ident_stdrt_not_null_ucc);
        let ident_with_id_arr_not_null_as_pg_json_upd_ts = gen_type_as_pg_json_upd_ts(&ident_with_id_arr_not_null_ucc);
        let ident_with_id_arr_not_null_as_pg_json_upd_for_query_ts = gen_type_as_pg_json_upd_for_query_ts(&ident_with_id_arr_not_null_ucc);
        let ident_with_id_stdrt_not_null_upd_el_ucc = &SelfUpdElUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let ident_with_id_stdrt_not_null_upd_for_query_el_ucc = &SelfUpdForQueryElUcc::from_tokens(&ident_with_id_stdrt_not_null_ucc);
        let import_unique_vec_ident_with_id_stdrt_not_null_upd_el_ts = quote!{
            #import::UniqueVec::<#ident_with_id_stdrt_not_null_upd_el_ucc>
        };
        let import_unique_vec_ident_with_id_stdrt_not_null_upd_for_query_el_ts = quote!{
            #import::UniqueVec::<#ident_with_id_stdrt_not_null_upd_for_query_el_ucc>
        };
        let gen_cr_upd_del_fields_ts_043c4057 = |
            add_serde_skip_serializing_if_vec_is_empty_ann: &AddSerdeSkipSerializingIfVecIsEmptyAnn,
            cr_ts: &dyn ToTokens,
            upd_ts: &dyn ToTokens,
            del_ts: &dyn ToTokens
        | {
            let mb_serde_skip_serializing_if_vec_is_empty_ts = match &add_serde_skip_serializing_if_vec_is_empty_ann {
                AddSerdeSkipSerializingIfVecIsEmptyAnn::False => Ts2::new(),
                AddSerdeSkipSerializingIfVecIsEmptyAnn::True => quote! {#[serde(skip_serializing_if = "Vec::is_empty")]},
            };
            quote! {
                #mb_serde_skip_serializing_if_vec_is_empty_ts
                #CrSc: #cr_ts,
                #UpdSc: #upd_ts,
                #mb_serde_skip_serializing_if_vec_is_empty_ts
                #DelSc: #del_ts,
            }
        };
        let ident_upd_ts = {
            let gen_ident_upd_stdrt_not_null_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                gen_unique_vec_w_ts(match &is_stdrt_with_id {
                    IsStdrtWithId::False => &ident_upd_el_ucc,
                    IsStdrtWithId::True => &ident_with_id_stdrt_not_null_upd_el_ucc,
                })
            };
            let vec_ident_with_id_stdrt_not_null_cr_ts = gen_vec_tokens_dcl_ts(
                &ident_with_id_stdrt_not_null_cr_ucc
            );
            let vec_pg_crud_path_pg_json_uuid_uuid_upd_ts = gen_vec_tokens_dcl_ts(
                &pg_crud_path_pg_json_uuid_uuid_upd_ts
            );
            let gen_cr_upd_del_fields_ts_ffcbdaf0 = |v: &AddSerdeSkipSerializingIfVecIsEmptyAnn| {
                gen_cr_upd_del_fields_ts_043c4057(
                    v,
                    &vec_ident_with_id_stdrt_not_null_cr_ts,
                    &import_unique_vec_ident_with_id_stdrt_not_null_upd_el_ts,
                    &vec_pg_crud_path_pg_json_uuid_uuid_upd_ts
                )
            };
            let ident_upd_ts = {
                let gen_opt_ident_type_ts = |ts: &dyn ToTokens| wrap_into_scopes_dot_comma_ts(
                    &gen_opt_type_dcl_ts(&ts)
                );
                let (
                    derive_serde_deserialize,
                    ts_975df5c5
                ) = match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => (
                            DSerdeDeserialize::True,
                            &wrap_into_scopes_dot_comma_ts(
                                &gen_ident_upd_stdrt_not_null_ts(&is_stdrt_with_id_false)
                            )
                        ),
                        IsNullable::True => (
                            DSerdeDeserialize::True,
                            &gen_opt_ident_type_ts(&ident_stdrt_not_null_as_pg_json_upd_ts)
                        ),
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => (
                            DSerdeDeserialize::False,
                            &{
                                let fields_ts = gen_cr_upd_del_fields_ts_ffcbdaf0(&AddSerdeSkipSerializingIfVecIsEmptyAnn::True);
                                quote! {{#fields_ts}}
                            }
                        ),
                        IsNullable::True => (
                            DSerdeDeserialize::True,
                            &gen_opt_ident_type_ts(&ident_with_id_arr_not_null_as_pg_json_upd_ts)
                        ),
                    },
                };
                let ts_c9a843aa = DTsBuilder::new()
                .make_pub()
                .d_debug()
                .d_clone()
                .d_partial_eq()
                .d_serde_serialize()
                .d_serde_deserialize_if(derive_serde_deserialize)
                .d_utoipa_to_schema()
                .d_schemars_json_schema()
                .build_struct(
                    &ident_upd_ucc,
                    &Ts2::new(),
                    &ts_975df5c5
                );
                quote!{
                    #AllowClippyArbitrarySrcItemOrdering
                    #ts_c9a843aa
                }
            };
            let ident_upd_try_new_er_ucc = &SelfUpdTryNewErUcc::from_tokens(&ident);
            let mb_ident_upd_try_new_er_ts = match &pattern {
                Pattern::Stdrt => Ts2::new(),
                Pattern::Arr => match &is_nullable {
                    IsNullable::False => DTsBuilder::new()
                        .make_pub()
                        .d_debug()
                        .d_serde_serialize()
                        .d_serde_deserialize()
                        .d_thiserror_error()
                        .d_location_lib_location()
                        .build_enum(
                            &ident_upd_try_new_er_ucc,
                            &Ts2::new(),
                            &quote!{{
                                #CrUpdDelAreEmptyUcc {
                                    loc: location_lib::loc::Loc,
                                },
                                #IdsAreNotUniqueUcc {
                                    #[eo_to_err_string_serde]
                                    duplicate: #StringTs,
                                    loc: location_lib::loc::Loc,
                                },
                                #NotUniqueIdInJsonDelArrUcc {
                                    #[eo_to_err_string_serde]
                                    er: #StringTs,
                                    loc: location_lib::loc::Loc,
                                },
                                #NotUniqueIdInJsonUpdAndDelArrsUcc {
                                    #[eo_to_err_string_serde]
                                    er: #StringTs,
                                    loc: location_lib::loc::Loc,
                                },
                            }}
                        ),
                    IsNullable::True => Ts2::new(),
                },
            };
            let impl_ident_upd_ts = {
                let mb_pub_new_or_try_new_for_ident_upd_ts = match &pattern {
                    Pattern::Stdrt => gen_pub_const_new_ts(
                        &MustUse,
                        &gen_v_type_ts(&match &is_nullable {
                            IsNullable::False => gen_unique_vec_w_ts(&ident_stdrt_not_null_upd_el_ucc),
                            IsNullable::True => gen_opt_type_dcl_ts(&ident_stdrt_not_null_as_pg_json_upd_ts)
                        }),
                        &self_v_ts
                    ),
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => gen_pub_try_new_ts(
                            &Ts2::new(),
                            &gen_cr_upd_del_fields_ts_ffcbdaf0(&AddSerdeSkipSerializingIfVecIsEmptyAnn::False),
                            &ident_upd_try_new_er_ucc,
                            &{
                                let custom_serde_er_deserializing_ident_upd_str = format!("custom serde er deserializing {ident_upd_ucc}");
                                let check_if_all_empty_ts = {
                                    quote! {
                                        if cr.is_empty() && upd.is_empty() && del.is_empty() {
                                            return Err(#ident_upd_try_new_er_ucc::#CrUpdDelAreEmptyUcc {
                                                loc: location_lib::loc!()
                                            });
                                        }
                                    }
                                };
                                let check_if_ids_are_unique_ts = {
                                    let (
                                        uuid_as_pg_json_upd_to_err_string_el_id_ts,
                                        uuid_as_pg_json_upd_to_err_string_el_ts,
                                    ) = {
                                        #[allow(clippy::arbitrary_source_item_ordering)]
                                        enum UpdOrDel {
                                            Upd,
                                            Del
                                        }
                                        let gen_ts = |
                                            upd_or_del: &UpdOrDel,
                                            el_ts: &dyn ToTokens,
                                        |{
                                            let ts: &dyn ToTokens = match &upd_or_del {
                                                UpdOrDel::Upd => &quote!{&#el_ts.#IdSc},
                                                UpdOrDel::Del => &el_ts
                                            };
                                            quote!{
                                                <#uuid_uuid_as_not_null_jsonb_string_as_pg_json_upd_ts as location_lib::ToErrString>::to_err_string(
                                                    #ts
                                                )
                                            }
                                        };
                                        (
                                            gen_ts(
                                                &UpdOrDel::Upd,
                                                &quote!{el}
                                            ),
                                            gen_ts(
                                                &UpdOrDel::Del,
                                                &quote!{el}
                                            )
                                        )
                                    };
                                    quote!{{
                                        let mut acc_2bf4e098 = Vec::new();
                                        for el in upd.to_vec() {
                                            if acc_2bf4e098.contains(&&el.#IdSc) {
                                                return Err(#ident_upd_try_new_er_ucc::#IdsAreNotUniqueUcc {
                                                    duplicate: #uuid_as_pg_json_upd_to_err_string_el_id_ts,
                                                    loc: location_lib::loc!()
                                                });
                                            }
                                            acc_2bf4e098.push(&el.#IdSc);
                                        }
                                        for el in &del {
                                            if acc_2bf4e098.contains(&el) {
                                                return Err(#ident_upd_try_new_er_ucc::#IdsAreNotUniqueUcc {
                                                    duplicate: #uuid_as_pg_json_upd_to_err_string_el_ts,
                                                    loc: location_lib::loc!()
                                                });
                                            }
                                            acc_2bf4e098.push(el);
                                        }
                                    }}
                                };
                                let check_not_unique_id_ts = {
                                    let check_not_unique_id_in_upd_arr_ts = quote! {
                                        let upd_acc = #UpdSc.to_vec().iter()
                                        .map(|el|&el.#IdSc)
                                        .collect::<Vec<&#uuid_uuid_as_not_null_jsonb_string_as_pg_json_upd_ts>>();
                                    };
                                    let check_not_unique_id_in_del_aray_ts = {
                                        let not_unique_id_in_json_del_arr_dq_ts = dq_ts(&format!("{custom_serde_er_deserializing_ident_upd_str}: not unique {IdSc} in json del arr: {{}}"));
                                        quote! {
                                            let del_acc = {
                                                let mut del_acc = Vec::new();
                                                for el in &del {
                                                    if del_acc.contains(&el) {
                                                        return Err(#ident_upd_try_new_er_ucc::#NotUniqueIdInJsonDelArrUcc {
                                                            er: format!(
                                                                #not_unique_id_in_json_del_arr_dq_ts,
                                                                #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::get_inn(
                                                                    &el.clone().into()
                                                                )
                                                            ),
                                                            loc: location_lib::loc!()
                                                        });
                                                    }
                                                    del_acc.push(el);
                                                }
                                                del_acc
                                            };
                                        }
                                    };
                                    let check_not_unique_id_in_upd_and_del_arrs_ts = {
                                        let not_unique_id_in_json_upd_and_del_arrs_dq_ts = dq_ts(&format!("{custom_serde_er_deserializing_ident_upd_str}: not unique {IdSc} in json upd and del arrs: {{}}"));
                                        quote! {
                                            for el in upd_acc {
                                                if del_acc.contains(&el) {
                                                    return Err(#ident_upd_try_new_er_ucc::#NotUniqueIdInJsonUpdAndDelArrsUcc {
                                                        er: format!(
                                                            #not_unique_id_in_json_upd_and_del_arrs_dq_ts,
                                                            #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::get_inn(
                                                                &el.clone().into()
                                                            )
                                                        ),
                                                        loc: location_lib::loc!()
                                                    });
                                                }
                                            }
                                        }
                                    };
                                    quote! {
                                        {
                                            #check_not_unique_id_in_upd_arr_ts
                                            #check_not_unique_id_in_del_aray_ts
                                            #check_not_unique_id_in_upd_and_del_arrs_ts
                                        }
                                    }
                                };
                                quote!{
                                    #check_if_all_empty_ts
                                    #check_if_ids_are_unique_ts
                                    #check_not_unique_id_ts
                                    Ok(Self {
                                        #CrSc,
                                        #UpdSc,
                                        #DelSc
                                    })
                                }
                            }
                        ),
                        IsNullable::True => gen_pub_const_new_v_type_cnt_self_v_ts(
                            &gen_opt_type_dcl_ts(
                                &ident_with_id_arr_not_null_as_pg_json_upd_ts
                            )
                        )
                    },
                };
                quote!{
                    impl #ident_upd_ucc {
                        #mb_pub_new_or_try_new_for_ident_upd_ts
                    }
                }
            };
            let mb_impl_de_for_ident_upd_ts = match &pattern {
                Pattern::Stdrt => Ts2::new(),
                Pattern::Arr => match &is_nullable {
                    IsNullable::False => {
                        //todo mb reuse?
                        let tuple_struct_ident_upd_dq_ts = dq_ts(&format!("tuple struct {ident_upd_ucc}"));
                        let ident_upd_dq_ts = dq_ts(&ident_upd_ucc);
                        let match_try_new_in_de_ts = gen_match_try_new_in_de_ts(
                            &ident_upd_ucc,
                            &quote! {f0_v, f1_v, f2_v}
                        );
                        quote! {
                            #[allow(clippy::absolute_paths)]
                            #AllowClippyArbitrarySrcItemOrdering
                            impl<'de> serde::Deserialize<'de> for #ident_upd_ucc {
                                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                                where
                                    __D: serde::Deserializer<'de>,
                                {
                                    #[allow(non_camel_case_types)]
                                    #[doc(hidden)]
                                    enum __Field {
                                        f0,
                                        f1,
                                        f2,
                                        __ignore,
                                    }
                                    #[doc(hidden)]
                                    struct __FieldVisitor;
                                    impl serde::de::Visitor<'_> for __FieldVisitor {
                                        type Value = __Field;
                                        fn expecting(&self, __f: &mut serde::__private228::Formatter<'_>) -> serde::__private228::fmt::Result {
                                            serde::__private228::Formatter::write_str(__f, "field identifier")
                                        }
                                        fn visit_u64<__E>(self, v: u64) -> Result<Self::Value, __E>
                                        where
                                            __E: serde::de::Error,
                                        {
                                            match v {
                                                0u64 => Ok(__Field::f0),
                                                1u64 => Ok(__Field::f1),
                                                2u64 => Ok(__Field::f2),
                                                _ => Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_str<__E>(self, v: &str) -> Result<Self::Value, __E>
                                        where
                                            __E: serde::de::Error,
                                        {
                                            match v {
                                                "cr" => Ok(__Field::f0),
                                                "upd" => Ok(__Field::f1),
                                                "del" => Ok(__Field::f2),
                                                _ => Ok(__Field::__ignore),
                                            }
                                        }
                                        fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
                                        where
                                            __E: serde::de::Error,
                                        {
                                            match v {
                                                b"cr" => Ok(__Field::f0),
                                                b"upd" => Ok(__Field::f1),
                                                b"del" => Ok(__Field::f2),
                                                _ => Ok(__Field::__ignore),
                                            }
                                        }
                                    }
                                    impl<'de> serde::Deserialize<'de> for __Field {
                                        #[inline]
                                        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                                        where
                                            __D: serde::Deserializer<'de>,
                                        {
                                            serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                                        }
                                    }
                                    #[doc(hidden)]
                                    struct __Visitor<'de> {
                                        marker: serde::__private228::PhantomData<#ident_upd_ucc>,
                                        lt: serde::__private228::PhantomData<&'de ()>,
                                    }
                                    impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = #ident_upd_ucc;
                                        fn expecting(&self, __f: &mut serde::__private228::Formatter<'_>) -> serde::__private228::fmt::Result {
                                            serde::__private228::Formatter::write_str(__f, #tuple_struct_ident_upd_dq_ts)
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                                        where
                                            __A: serde::de::SeqAccess<'de>,
                                        {
                                            let f0_v = serde::de::SeqAccess::next_element::<#vec_ident_with_id_stdrt_not_null_cr_ts>(&mut __seq)?.unwrap_or_default();
                                            let f1_v = serde::de::SeqAccess::next_element::<#import_unique_vec_ident_with_id_stdrt_not_null_upd_el_ts>(&mut __seq)?.unwrap_or_default();
                                            let f2_v = serde::de::SeqAccess::next_element::<#vec_pg_crud_path_pg_json_uuid_uuid_upd_ts>(&mut __seq)?.unwrap_or_default();
                                            #match_try_new_in_de_ts
                                        }
                                        #[inline]
                                        fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                                        where
                                            __A: serde::de::MapAccess<'de>,
                                        {
                                            let mut f0: Option<#vec_ident_with_id_stdrt_not_null_cr_ts> = None;
                                            let mut f1: Option<#import_unique_vec_ident_with_id_stdrt_not_null_upd_el_ts> = None;
                                            let mut f2: Option<#vec_pg_crud_path_pg_json_uuid_uuid_upd_ts> = None;
                                            while let Some(__k) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                                match __k {
                                                    __Field::f0 => {
                                                        if Option::is_some(&f0) {
                                                            return Err(<__A::Error as serde::de::Error>::duplicate_field("cr"));
                                                        }
                                                        f0 = Some(serde::de::MapAccess::next_value::<#vec_ident_with_id_stdrt_not_null_cr_ts>(&mut __map)?);
                                                    }
                                                    __Field::f1 => {
                                                        if Option::is_some(&f1) {
                                                            return Err(<__A::Error as serde::de::Error>::duplicate_field("upd"));
                                                        }
                                                        f1 = Some(serde::de::MapAccess::next_value::<#import_unique_vec_ident_with_id_stdrt_not_null_upd_el_ts>(&mut __map)?);
                                                    }
                                                    __Field::f2 => {
                                                        if Option::is_some(&f2) {
                                                            return Err(<__A::Error as serde::de::Error>::duplicate_field("del"));
                                                        }
                                                        f2 = Some(serde::de::MapAccess::next_value::<#vec_pg_crud_path_pg_json_uuid_uuid_upd_ts>(&mut __map)?);
                                                    }
                                                    __Field::__ignore => {
                                                        let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut __map)?;
                                                    }
                                                }
                                            }
                                            let f0_v = f0.unwrap_or_default();
                                            let f1_v = f1.unwrap_or_default();
                                            let f2_v = f2.unwrap_or_default();
                                            #match_try_new_in_de_ts
                                        }
                                    }
                                    #[doc(hidden)]
                                    const FIELDS: &[&str] = &["cr", "upd", "del"];
                                    serde::Deserializer::deserialize_struct(
                                        __deserializer,
                                        #ident_upd_dq_ts,
                                        FIELDS,
                                        __Visitor {
                                            marker: serde::__private228::PhantomData::<#SelfUcc>,
                                            lt: serde::__private228::PhantomData,
                                        },
                                    )
                                }
                            }
                        }
                    }
                    IsNullable::True => Ts2::new(),
                },
            };
            let impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_upd_ts = gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(&ident_upd_ucc, &Ts2::new(), &{
                let v = match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => quote! {(#PgCrudDfltOptSomeVecOneElCall)},
                        IsNullable::True => quote! {(Some(#PgCrudDfltOptSomeVecOneElCall))},
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => quote! {{
                            #CrSc: #vec_pg_crud_dflt_opt_some_vec_one_el_call_ts,
                            #UpdSc: #PgCrudDfltOptSomeVecOneElCall,
                            #DelSc: #vec_pg_crud_dflt_opt_some_vec_one_el_call_ts,
                        }},
                        IsNullable::True => quote! {
                            (Some(#PgCrudDfltOptSomeVecOneElCall))
                        },
                    },
                };
                quote! {Self #v}
            });
            let mb_ident_upd_el_ts = if is_stdrt_not_null {
                let ident_upd_el_ts = {
                    let ts_b258e2eb = DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone()
                    .d_partial_eq()
                    .d_serde_serialize()
                    .d_serde_deserialize()
                    .d_utoipa_to_schema()
                    .d_schemars_json_schema()
                    .build_enum(
                        &ident_stdrt_not_null_upd_el_ucc,
                        &Ts2::new(),
                        &{
                            let vrts_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                                let fi_dq_ts = gen_fi_dq_ts(el0);
                                let v_ft_as_json_type_upd_ts = gen_v_dcl_ts0(
                                    &gen_type_as_pg_json_upd_ts(&el0.type0)
                                );
                                quote! {
                                    #[serde(rename(serialize = #fi_dq_ts, deserialize = #fi_dq_ts))]
                                    #vrt_ident_ucc_ts(#v_ft_as_json_type_upd_ts)
                                }
                            });
                            quote!{{#(#vrts_ts),*}}
                        }
                    );
                    quote!{
                        #AllowClippyArbitrarySrcItemOrdering
                        #ts_b258e2eb
                    }
                };
                let impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_upd_el_ts = gen_impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_ts(&ident_stdrt_not_null_upd_el_ucc, &{
                    let ts = vec_syn_field.iter().map(|el0| {
                        let fi = &el0.ident;
                        let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                        let ts0 = gen_v_init_ts0(
                            &PgCrudDfltOptSomeVecOneElCall
                        );
                        quote! {#SelfUcc::#vrt_ident_ucc_ts(#ts0)}
                    });
                    quote! {vec![#(#ts),*]}
                });
                quote! {
                    #ident_upd_el_ts
                    #impl_pg_crud_all_vrts_dflt_opt_some_vec_one_el_for_ident_upd_el_ts
                }
            } else {
                Ts2::new()
            };
            let mb_ident_with_id_stdrt_not_null_upd_el_ts = if is_stdrt_not_null {
                //thought it can be reused as struct with generic param, but turns out its too painfull
                // pub trait MyTrait {
                //     type ExtraType: PartialEq;
                // }
                // pub struct MyStruct;
                // #[derive(PartialEq, Optml)]
                // pub struct MyStructExtraType(String);
                // impl MyTrait for MyStruct {
                //     type ExtraType = MyStructExtraType;
                // }
                // #[derive(PartialEq, Optml)]
                // pub struct WOfMyTrait<T: MyTrait>(<T as MyTrait>::ExtraType);
                // pub type WOfMyTraitAl = WOfMyTrait<MyStruct>;
                // #[derive(PartialEq, Optml)]
                // pub struct WOfWOfMyTraitAl(WOfMyTraitAl);
                // // er[E0369]: binary op `==` cannot be applied to type `WOfMyTrait<MyStruct>`
                let ident_with_id_stdrt_not_null_upd_el_fields_dcl_ts = quote! {
                    #IdSc: #pg_crud_path_pg_json_uuid_uuid_upd_ts,
                    #FieldsSc: #ident_stdrt_not_null_as_pg_json_upd_ts
                };
                let ident_with_id_stdrt_not_null_upd_el_ts = {
                    let ts_d18600a2 = DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone()
                    .d_partial_eq()
                    .d_serde_serialize()
                    .d_serde_deserialize()
                    .d_utoipa_to_schema()
                    .d_schemars_json_schema()
                    .build_struct(
                        &ident_with_id_stdrt_not_null_upd_el_ucc,
                        &Ts2::new(),
                        &quote!{{#ident_with_id_stdrt_not_null_upd_el_fields_dcl_ts}}
                    );
                    quote!{
                        #AllowClippyArbitrarySrcItemOrdering
                        #ts_d18600a2
                    }
                };
                let impl_pub_new_for_ident_with_id_stdrt_not_null_upd_el_ts = gen_impl_pub_const_new_for_ident_ts(
                    &ident_with_id_stdrt_not_null_upd_el_ucc,
                    &MustUse,
                    &ident_with_id_stdrt_not_null_upd_el_fields_dcl_ts,
                    &quote! {Self {
                        #IdSc,
                        #FieldsSc
                    }},
                );
                let impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_with_stdrt_not_null_upd_el_ts = gen_impl_pg_crud_dflt_opt_some_vec_one_el_ts(
                    &ident_with_id_stdrt_not_null_upd_el_ucc,
                    &Ts2::new(),
                    &quote! {Self {
                        #IdSc: #PgCrudDfltOptSomeVecOneElCall,
                        #FieldsSc: #PgCrudDfltOptSomeVecOneElCall,
                    }},
                );
                quote! {
                    #ident_with_id_stdrt_not_null_upd_el_ts
                    #impl_pub_new_for_ident_with_id_stdrt_not_null_upd_el_ts
                    #impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_with_stdrt_not_null_upd_el_ts
                }
            } else {
                Ts2::new()
            };
            quote! {
                #ident_upd_ts
                #mb_ident_upd_try_new_er_ts
                #impl_ident_upd_ts
                #mb_impl_de_for_ident_upd_ts
                #impl_pg_crud_dflt_opt_some_vec_one_el_for_ident_upd_ts
                #mb_ident_upd_el_ts
                #mb_ident_with_id_stdrt_not_null_upd_el_ts
            }
        };
        let gen_jsonb_build_obj_or_dq_ts = |v: &dyn Display|dq_ts(&format!("{}||", gen_jsonb_build_obj(v)));
        let ident_upd_for_query_ucc = SelfUpdForQueryUcc::from_tokens(&ident);
        let ident_upd_for_query_ts = {
            let ident_upd_for_query_ts = {
                let gen_ident_upd_for_query_ts = |ts: &dyn ToTokens| {
                    gen_debug_clone_partialeq_serialize_pub_struct_ts(
                        &AllowClippyArbitrarySrcItemOrdering,
                        &ident_upd_for_query_ucc,
                        &ts
                    )
                };
                let gen_opt_ident_type_ts = |ts: &dyn ToTokens| wrap_into_scopes_dot_comma_ts(
                    &gen_opt_type_dcl_ts(&ts)
                );
                let gen_ident_upd_for_query_stdrt_not_null_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                    gen_unique_vec_w_ts(match &is_stdrt_with_id {
                        IsStdrtWithId::False => &ident_upd_for_query_el_ucc,
                        IsStdrtWithId::True => &ident_with_id_stdrt_not_null_upd_for_query_el_ucc,
                    })
                };
                let vec_ident_with_id_stdrt_not_null_cr_for_query_ts = gen_vec_tokens_dcl_ts(
                    &ident_with_id_stdrt_not_null_cr_for_query_ucc
                );
                let vec_pg_crud_path_pg_json_uuid_uuid_upd_for_query_ts = gen_vec_tokens_dcl_ts(
                    &pg_crud_path_pg_json_uuid_uuid_upd_for_query_ts
                );
                match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => gen_ident_upd_for_query_ts(
                            &wrap_into_scopes_dot_comma_ts(
                                &gen_ident_upd_for_query_stdrt_not_null_ts(
                                    &is_stdrt_with_id_false
                                )
                            )
                        ),
                        IsNullable::True => gen_ident_upd_for_query_ts(
                            &gen_opt_ident_type_ts(
                                &ident_stdrt_not_null_as_pg_json_upd_for_query_ts
                            )
                        ),
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => gen_ident_upd_for_query_ts(
                            &{
                                let fields_ts = gen_cr_upd_del_fields_ts_043c4057(
                                    &AddSerdeSkipSerializingIfVecIsEmptyAnn::True,
                                    &vec_ident_with_id_stdrt_not_null_cr_for_query_ts,
                                    &import_unique_vec_ident_with_id_stdrt_not_null_upd_for_query_el_ts,
                                    &vec_pg_crud_path_pg_json_uuid_uuid_upd_for_query_ts,//todo mb expand logic with where cases
                                );
                                quote! {{#fields_ts}}
                            }
                        ),
                        IsNullable::True => gen_ident_upd_for_query_ts(
                            &gen_opt_ident_type_ts(&ident_with_id_arr_not_null_as_pg_json_upd_for_query_ts)
                        ),
                    },
                }
            };
            let impl_ident_upd_for_query_ts = {
                let select_only_updd_ids_qp_ts = {
                    let ts = match &pattern {
                        Pattern::Stdrt => match &is_nullable {
                            IsNullable::False => {
                                let match_vrts_ts = vec_syn_field.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let fi_ucc = ToTokensToUccTs::case_or_panic(&fi);
                                    let fi_dq_ts = dq_ts(&fi);
                                    let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                    let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                        &{
                                            let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&"{v_c3ae3be4}");
                                            quote!{acc_8e628eaf,#dq_ts0}
                                        },
                                        &return_err_qp_er_write_into_buffer_ts
                                    );
                                    quote! {
                                        #ident_stdrt_not_null_upd_for_query_el_ucc::#fi_ucc(v_939e13d6) => {
                                            match #ft_as_pg_json_ts::#SelectOnlyUpddIdsQpSc(
                                                &v_939e13d6.#VSc,
                                                #fi_dq_ts,
                                                column_field,
                                                #IncrSc
                                            ) {
                                                Ok(mut v_c3ae3be4) => {
                                                    let _: Option<char> = v_c3ae3be4.pop();
                                                    #if_write_is_err_curly_braces_ts
                                                },
                                                Err(#ErSc) => {
                                                    return Err(#ErSc);
                                                }
                                            }
                                        }
                                    }
                                });
                                quote!{
                                    let mut acc_8e628eaf = #StringTs::default();
                                    for el in self.0.to_vec() {
                                        match &el {
                                            #(#match_vrts_ts),*
                                        }
                                    }
                                    let _: Option<char> = acc_8e628eaf.pop();
                                    let _: Option<char> = acc_8e628eaf.pop();
                                    Ok(acc_8e628eaf)
                                }
                            },
                            IsNullable::True => {
                                let match_ts = vec_syn_field.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let fi_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                                    let fi_dq_ts = dq_ts(&fi);
                                    let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                    let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                        &{
                                            let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&"{v_a9da8905}");
                                            quote!{acc_f7537df2, #dq_ts0}
                                        },
                                        &return_err_qp_er_write_into_buffer_ts
                                    );
                                    quote! {
                                        #ident_stdrt_not_null_upd_for_query_el_ucc::#fi_ucc_ts(
                                            v_92d002a5
                                        ) => match #ft_as_pg_json_ts::#SelectOnlyUpddIdsQpSc(
                                            &v_92d002a5.#VSc,
                                            #fi_dq_ts,
                                            column_field,
                                            #IncrSc
                                        ) {
                                            Ok(mut v_a9da8905) => {
                                                let _: Option<char> = v_a9da8905.pop();
                                                #if_write_is_err_curly_braces_ts
                                            }
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            }
                                        }
                                    }
                                });
                                let dq_ts0 = dq_ts(&gen_jsonb_build_obj_v(&"{acc_f7537df2}"));
                                quote!{
                                    Ok(match &self.0 {
                                        Some(v_9570957e) => {
                                            let mut acc_f7537df2 = #StringTs::default();
                                            for el in v_9570957e.0.to_vec() {
                                                match &el {
                                                    #(#match_ts),*
                                                }
                                            }
                                            let _: Option<char> = acc_f7537df2.pop();
                                            let _: Option<char> = acc_f7537df2.pop();
                                            format!(#dq_ts0)
                                        },
                                        None => "'null'::jsonb".to_owned()//todo mb reuse
                                    })
                                }
                            },
                        },
                        Pattern::Arr => match &is_nullable {
                            IsNullable::False => {
                                let match_vrts_ts = vec_syn_field.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let fi_ucc = ToTokensToUccTs::case_or_panic(&fi);
                                    let fi_dq_ts = dq_ts(&fi);
                                    let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                    let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                        &{
                                            let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&"{v_33d3b52e}");
                                            quote!{acc_892857b1, #dq_ts0}
                                        },
                                        &return_err_qp_er_write_into_buffer_ts
                                    );
                                    quote! {
                                        #ident_stdrt_not_null_upd_for_query_el_ucc::#fi_ucc(v_40a8d7a1) => match #ft_as_pg_json_ts::#SelectOnlyUpddIdsQpSc(
                                            &v_40a8d7a1.#VSc,
                                            #fi_dq_ts,
                                            "elem",
                                            #IncrSc
                                        ) {
                                            Ok(mut v_33d3b52e) => {
                                                let _: Option<char> = v_33d3b52e.pop();
                                                #if_write_is_err_curly_braces_ts
                                            }
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            }
                                        }
                                    }
                                });
                                let select_only_crd_ids_qp_ts = vec_syn_field_with_id.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let fi_dq_ts = dq_ts(&fi);
                                    let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                    let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                        &{
                                            let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&"{v_549a93c8}");
                                            quote!{acc_57cd0744, #dq_ts0}
                                        },
                                        &return_err_qp_er_write_into_buffer_ts
                                    );
                                    quote! {
                                        match #ft_as_pg_json_ts::#SelectOnlyCrdIdsQpSc(
                                            &el_b1359d90.#fi,
                                            #fi_dq_ts,
                                            "elem",
                                            #IncrSc
                                        ) {
                                            Ok(mut v_549a93c8) => {
                                                let _: Option<char> = v_549a93c8.pop();
                                                #if_write_is_err_curly_braces_ts
                                            },
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            }
                                        }
                                    }
                                });
                                let if_write_is_err_curly_braces_0_ts = gen_if_write_is_err_curly_braces_ts(
                                    &{
                                        let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&"{v_6bac798d}");
                                        quote!{acc_892857b1, #dq_ts0}
                                    },
                                    &return_err_qp_er_write_into_buffer_ts
                                );
                                let if_write_is_err_curly_braces_1_ts = gen_if_write_is_err_curly_braces_ts(
                                    &quote!{acc_57cd0744, "{acc_892857b1}||"},
                                    &return_err_qp_er_write_into_buffer_ts
                                );
                                let if_write_is_err_0_ts = gen_if_write_is_err_ts(
                                    &quote!{acc_d497e8a5, "${v_c31cb081},"},
                                    &return_err_qp_er_write_into_buffer_ts
                                );
                                let if_write_is_err_1_ts = gen_if_write_is_err_ts(
                                    &quote!{acc_d497e8a5, "${v_b52c3fe1},"},
                                    &return_err_qp_er_write_into_buffer_ts
                                );
                                let dq_ts0 = dq_ts(&gen_jsonb_build_obj_v(&"{acc_57cd0744}"));
                                quote!{
                                    Ok(format!(
                                        "(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))",
                                        {
                                            let mut acc_57cd0744 = #StringTs::new();
                                            for el_d7561f40 in self.#UpdSc.to_vec() {
                                                //todo mb wrong for multiple upds by id?
                                                let mut acc_892857b1 = #StringTs::new();
                                                match #import_pg_json_uuid_uuid_as_not_null_jsonb_string_as_pg_json_ts ::select_only_updd_ids_qp(
                                                    &el_d7561f40.id,
                                                    "id",
                                                    "elem",
                                                    #IncrSc
                                                ) {
                                                    Ok(mut v_6bac798d) => {
                                                        let _: Option<char> = v_6bac798d.pop();
                                                        #if_write_is_err_curly_braces_0_ts
                                                    }
                                                    Err(#ErSc) => {
                                                        return Err(#ErSc);
                                                    }
                                                }
                                                for el_738b2a83 in el_d7561f40.fields.0.to_vec() {
                                                    match &el_738b2a83 {
                                                        #(#match_vrts_ts),*
                                                    }
                                                }
                                                let _: Option<char> = acc_892857b1.pop();
                                                let _: Option<char> = acc_892857b1.pop();
                                                #if_write_is_err_curly_braces_1_ts
                                            }
                                            for el_b1359d90 in &self.cr {
                                                #(#select_only_crd_ids_qp_ts)*
                                            }
                                            let _: Option<char> = acc_57cd0744.pop();
                                            let _: Option<char> = acc_57cd0744.pop();
                                            format!(#dq_ts0)
                                        },
                                        column_field,
                                        {
                                            let mut acc_d497e8a5 = #StringTs::new();
                                            for _ in self.#UpdSc.to_vec() {
                                                match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                                    Ok(v_c31cb081) => {
                                                        #if_write_is_err_0_ts
                                                    },
                                                    Err(#ErSc) => {
                                                        return Err(#ErSc);
                                                    },
                                                }
                                            }
                                            for _ in &self.#CrSc {
                                                match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                                    Ok(v_b52c3fe1) => {
                                                        #if_write_is_err_1_ts
                                                    },
                                                    Err(#ErSc) => {
                                                        return Err(#ErSc);
                                                    },
                                                }
                                            }
                                            let _: Option<char> = acc_d497e8a5.pop();
                                            acc_d497e8a5
                                        }
                                    ))
                                }
                            },
                            IsNullable::True => {
                                let dq_ts0 = dq_ts(&gen_jsonb_build_obj_v(&"{}"));
                                quote!{
                                    Ok(match &self.0 {
                                        Some(v_bc509c9a) => format!(
                                            #dq_ts0,
                                            match #ident_arr_not_null_upd_for_query_ucc::#SelectOnlyUpddIdsQpSc(
                                                v_bc509c9a,
                                                column_field,
                                                #IncrSc
                                            ) {
                                                Ok(v_1e016751) => v_1e016751,
                                                Err(#ErSc) => {
                                                    return Err(#ErSc);
                                                }
                                            }
                                        ),
                                        None => "'null'::jsonb".to_owned(),
                                    })
                                }
                            },
                        },
                    };
                    quote!{
                        #[allow(clippy::single_call_fn)]//for some reason lint ignoring this fn call in other struct trait methonds(arr not null)
                        fn #SelectOnlyUpddIdsQpSc(
                            &self,
                            column_field: &str,
                            #IncrSc: &mut u64
                        ) -> Result<#StringTs, #import_qp_er_ts> {
                            #ts
                        }
                    }
                };
                quote!{
                    impl #ident_upd_for_query_ucc {
                        #select_only_updd_ids_qp_ts
                    }
                }
            };
            let impl_from_ident_stdrt_not_null_upd_for_ident_stdrt_not_null_upd_for_query_ts = gen_impl_from_ts(
                &quote!{#ident_as_import_pg_json_ts::Upd},
                &quote!{#ident_as_import_pg_json_ts::UpdForQuery},
                &match &is_nullable {
                    IsNullable::False => match &pattern {
                        Pattern::Stdrt => quote!{
                            Self(#import::NotEmptyUniqueVec::from_t1_impl_from_t2(#VSc.0))
                        },
                        Pattern::Arr => quote!{
                            Self {
                                #CrSc: #VSc.#CrSc.into_iter().map(#ident_with_id_stdrt_not_null_cr_for_query_ucc::from).collect(),
                                #UpdSc: #import::UniqueVec::from_t1_impl_from_t2(#VSc.#UpdSc),
                                #DelSc: #VSc.#DelSc.into_iter().map(Into::into).collect(),
                            }
                        }
                    },
                    IsNullable::True => {
                        let ts: &dyn ToTokens = match &pattern {
                            Pattern::Stdrt => &ident_stdrt_not_null_as_import_pg_json_ts,
                            Pattern::Arr => &ident_arr_not_null_as_import_pg_json_ts
                        };
                        quote!{Self(#VSc.0.map(#ts::UpdForQuery::from))}
                    }
                }
            );
            let mb_ident_upd_for_query_el_ts = if is_stdrt_not_null {
                let ident_stdrt_not_null_upd_for_query_el_ts = DTsBuilder::new()
                    .make_pub()
                    .d_debug()
                    .d_clone()
                    .d_partial_eq()
                    .d_serde_serialize()
                    .build_enum(
                        &ident_stdrt_not_null_upd_for_query_el_ucc,
                        &Ts2::new(),
                        &{
                            let vrts_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                                let fi_dq_ts = gen_fi_dq_ts(el0);
                                let v_ft_as_json_type_upd_for_query_ts = gen_v_dcl_ts0(&gen_type_as_pg_json_upd_for_query_ts(&el0.type0));
                                quote! {
                                    #[serde(rename(serialize = #fi_dq_ts, deserialize = #fi_dq_ts))]
                                    #vrt_ident_ucc_ts(#v_ft_as_json_type_upd_for_query_ts)
                                }
                            });
                            quote!{{#(#vrts_ts),*}}
                        }
                    );
                let impl_from_ident_stdrt_not_null_upd_el_for_ident_stdrt_not_null_upd_for_query_el_ts = gen_impl_from_ts(
                    &ident_stdrt_not_null_upd_el_ucc,
                    &ident_stdrt_not_null_upd_for_query_el_ucc,
                    &{
                        let vrts_ts = vec_syn_field.iter().map(|el0| {
                            let fi = &el0.ident;
                            let vrt_ident_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                            let ts = gen_v_init_ts0(&{
                                let ft_as_json_type_upd_for_query_ts = gen_type_as_pg_json_upd_for_query_ts(
                                    &el0.type0
                                );
                                quote!{
                                    #ft_as_json_type_upd_for_query_ts::from(v_121f1c54.#VSc)
                                }
                            });
                            quote! {
                                #ident_stdrt_not_null_upd_el_ucc::#vrt_ident_ucc_ts(v_121f1c54) => #SelfUcc::#vrt_ident_ucc_ts(#ts)
                            }
                        });
                        quote!{
                            match #VSc {
                                #(#vrts_ts),*
                            }
                        }
                    }
                );
                quote! {
                    #ident_stdrt_not_null_upd_for_query_el_ts
                    #impl_from_ident_stdrt_not_null_upd_el_for_ident_stdrt_not_null_upd_for_query_el_ts
                }
            } else {
                Ts2::new()
            };
            let mb_ident_with_id_stdrt_not_null_upd_for_query_el_ts = if is_stdrt_not_null {
                let ident_with_id_stdrt_not_null_upd_for_query_el_fields_dcl_ts = quote! {
                    #IdSc: #pg_crud_path_pg_json_uuid_uuid_upd_for_query_ts,
                    #FieldsSc: #ident_stdrt_not_null_as_pg_json_upd_for_query_ts
                };
                let ident_with_id_stdrt_not_null_upd_for_query_el_ts = gen_debug_clone_partialeq_serialize_pub_struct_ts(
                    &AllowClippyArbitrarySrcItemOrdering,
                    &ident_with_id_stdrt_not_null_upd_for_query_el_ucc,
                    &quote!{{#ident_with_id_stdrt_not_null_upd_for_query_el_fields_dcl_ts}}
                );
                let impl_pub_const_new_for_ident_with_id_stdrt_not_null_upd_for_query_el_ts = gen_impl_pub_const_new_for_ident_ts(
                    &ident_with_id_stdrt_not_null_upd_for_query_el_ucc,
                    &MustUse,
                    &ident_with_id_stdrt_not_null_upd_for_query_el_fields_dcl_ts,
                    &quote! {Self {
                        #IdSc,
                        #FieldsSc
                    }},
                );
                let impl_from_ident_with_id_stdrt_not_null_upd_el_for_ident_with_id_stdrt_not_null_upd_for_query_el_ts = gen_impl_from_ts(
                    &ident_with_id_stdrt_not_null_upd_el_ucc,
                    &ident_with_id_stdrt_not_null_upd_for_query_el_ucc,
                    &quote! {Self {
                        #IdSc: #uuid_uuid_as_not_null_jsonb_string_as_import_pg_json_ts::UpdForQuery::from(
                            #VSc.#IdSc
                        ),
                        fields: #ident_stdrt_not_null_as_import_pg_json_ts::UpdForQuery::from(
                            #VSc.fields
                        ),
                    }}
                );
                quote! {
                    #ident_with_id_stdrt_not_null_upd_for_query_el_ts
                    #impl_pub_const_new_for_ident_with_id_stdrt_not_null_upd_for_query_el_ts
                    #impl_from_ident_with_id_stdrt_not_null_upd_el_for_ident_with_id_stdrt_not_null_upd_for_query_el_ts
                }
            } else {
                Ts2::new()
            };
            quote!{
                #ident_upd_for_query_ts
                #impl_ident_upd_for_query_ts
                #impl_from_ident_stdrt_not_null_upd_for_ident_stdrt_not_null_upd_for_query_ts
                #mb_ident_upd_for_query_el_ts
                #mb_ident_with_id_stdrt_not_null_upd_for_query_el_ts
            }
        };
        let (impl_pg_crud_pg_json_for_ident_ts, mb_impl_pg_crud_pg_types_pg_type_pg_type_ts) = {
            let pg_type_or_pg_json_pg_type = PgTypeOrPgJson::PgType;
            let pg_type_or_pg_json_pg_json = PgTypeOrPgJson::PgJson;
            let gen_upd_qp_stdrt_nullable_ts = |pg_type_or_pg_json: &PgTypeOrPgJson|{
                let format_ts = dq_ts(&match &pg_type_or_pg_json {
                    PgTypeOrPgJson::PgType => format!("jsonb_set({{{JsonbSetAccumulatorSc}}},'{{{{{{{JsonbSetPathSc}}}}}}}',${{v_27b8537f}})"),
                    PgTypeOrPgJson::PgJson => "${v_27b8537f}".to_owned(),
                });
                quote! {
                    match &#VSc.0 {
                        Some(v_92f34435) => #ident_stdrt_not_null_as_pg_json_ts::#UpdQpSc(
                            v_92f34435,
                            jsonb_set_accumulator,
                            jsonb_set_target,
                            jsonb_set_path,
                            incr,
                        ),
                        None => match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                            Ok(v_27b8537f) => Ok(format!(#format_ts)),
                            Err(#ErSc) => Err(#ErSc),
                        }
                    }
                }
            };
            let gen_upd_del_cr_arr_ts = |ts: &dyn ToTokens|{
                let if_write_is_err_ts = gen_if_write_is_err_ts(
                    &quote!{acc_2e2ad041, "{v_8333f8f4}"},
                    &return_err_qp_er_write_into_buffer_ts
                );
                let if_write_is_err_curly_braces_0_ts = gen_if_write_is_err_curly_braces_ts(
                    &quote!{acc_5b4cd920, "{mb_space_and_space}elem->>'id' <> ${incr_cb6ba4a7}"},
                    &return_err_qp_er_write_into_buffer_ts
                );
                let if_write_is_err_curly_braces_1_ts = gen_if_write_is_err_curly_braces_ts(
                    &quote!{acc_8554f572, "${incr_5bbc4961},"},
                    &return_err_qp_er_write_into_buffer_ts
                );
                quote! {
                    let upd_qp_acc = {
                        if v_58d685d3.#UpdSc.is_empty() {
                            #StringTs::from("elem")
                        } else {
                            let mut acc_2e2ad041 = #StringTs::default();
                            for el in v_58d685d3.#UpdSc.to_vec() {
                                let ident_with_id_h = {
                                    let id_incr = match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::incr_checked_add_one(#IncrSc) {
                                        Ok(v_15b44b54) => v_15b44b54,
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    };
                                    match #ident_stdrt_not_null_as_pg_json_ts::#UpdQpSc(
                                        &el.fields,
                                        "",
                                        "elem",
                                        "",
                                        #IncrSc
                                    ) {
                                        Ok(v_56c44461) => Ok(format!("when elem->>'id' = ${id_incr} then {v_56c44461} ")),
                                        Err(#ErSc) => Err(#ErSc)
                                    }
                                };
                                match ident_with_id_h {
                                    Ok(v_8333f8f4) => {
                                        #if_write_is_err_ts
                                    }
                                    Err(#ErSc) => {
                                        return Err(#ErSc);
                                    }
                                }
                            }
                            let _: Option<char> = acc_2e2ad041.pop();
                            format!("case {acc_2e2ad041} else elem end")
                        }
                    };
                    let del_qp_acc = {
                        let mut acc_5b4cd920 = #StringTs::default();
                        for _ in &v_58d685d3.#DelSc {
                            let incr_cb6ba4a7 = match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::incr_checked_add_one(#IncrSc) {
                                Ok(v_110650cc) => v_110650cc,
                                Err(#ErSc) => {
                                    return Err(#ErSc);
                                }
                            };
                            let mb_space_and_space = if acc_5b4cd920.is_empty() { "" } else { " and " };
                            #if_write_is_err_curly_braces_0_ts
                        }
                        acc_5b4cd920
                    };
                    let cr_qp_acc = {
                        let mut acc_8554f572 = #StringTs::default();
                        for _ in &v_58d685d3.#CrSc {
                            let incr_5bbc4961 = match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::incr_checked_add_one(#IncrSc) {
                                Ok(v_27487842) => v_27487842,
                                Err(#ErSc) => {
                                    return Err(#ErSc);
                                }
                            };
                            #if_write_is_err_curly_braces_1_ts
                        }
                        let _: Option<char> = acc_8554f572.pop();
                        acc_8554f572
                    };
                    let mb_where = if v_58d685d3.#DelSc.is_empty() {
                        #StringTs::default()
                    } else {
                        format!(" where {del_qp_acc}")
                    };
                    let mb_jsonb_build_arr = if v_58d685d3.#CrSc.is_empty() {
                        #StringTs::default()
                    } else {
                        format!(" || jsonb_build_arr({cr_qp_acc})")
                    };
                    Ok (format!(#ts))
                }
            };
            let gen_upd_qp_arr_not_null_ts = |pg_type_or_pg_json: &PgTypeOrPgJson|{
                let ts_c75c3cd1 = gen_upd_del_cr_arr_ts(&dq_ts(&match &pg_type_or_pg_json {
                    PgTypeOrPgJson::PgType => "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({upd_qp_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {mb_where}),'[]'::jsonb)) end {mb_jsonb_build_arr})",
                    PgTypeOrPgJson::PgJson => "((select coalesce((select jsonb_agg({upd_qp_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {mb_where}),'[]'::jsonb)) {mb_jsonb_build_arr})",
                }));
                quote!{
                    let v_58d685d3 = #VSc;
                    #ts_c75c3cd1
                }
            };
            let impl_pg_crud_pg_json_for_ident_ts = gen_impl_pg_json_ts(
                &Import::PgCrud,
                &ident,
                &ident_table_type_ucc,
                &ident_cr_ucc,
                &ident_cr_for_query_ucc,
                &ident_select_ucc,
                &IsSelectQpSelfSelectUsed::True,
                &IsSelectQpColumnFieldForErMessageUsed::True,
                &IsSelectQpIsPgTypeUsed::True,
                &match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let dq_ts0 = dq_ts(&gen_jsonb_build_obj(&format!("'{{fi}}',{}", gen_jsonb_build_obj_v(&"{v_156121ad}"))));
                            quote! {
                                match #VSc.#SelectQpSc(
                                    &if is_pg_type {
                                        column_field.to_owned()
                                    } else {
                                        format!("{column_field}->'{fi}'")
                                    },
                                    &format!("{column_field_for_er_message}.{fi}"),
                                ) {
                                    Ok(v_156121ad) => Ok(
                                        if is_pg_type {
                                            v_156121ad
                                        } else {
                                            format!(#dq_ts0)
                                        }
                                    ),
                                    Err(#ErSc) => Err(#ErSc)
                                }
                            }
                        },
                        IsNullable::True => {
                            let ident_stdrt_not_null_as_pg_json_select_as_dflt_but_opt_is_some_ts = gen_ident_as_dflt_but_opt_is_some_ts(
                                &ident_stdrt_not_null_as_pg_json_select_ts
                            );
                            let dq_ts0 = dq_ts(
                                &gen_jsonb_build_obj(
                                    &format!("'{{fi}}',{}", gen_jsonb_build_obj_v(&"case when jsonb_typeof({column_field_fi}) = 'null' then 'null'::jsonb else ({v_1f8de96a}) end"))
                                )
                            );
                            quote! {
                                let column_field_fi = format!("{column_field}->'{fi}'");
                                let v_46039f0e = #VSc.0.as_ref().map_or_else(
                                    #ident_stdrt_not_null_as_pg_json_select_as_dflt_but_opt_is_some_ts,
                                    Clone::clone
                                );
                                match #ident_stdrt_not_null_as_pg_json_ts::#SelectQpSc(
                                    &v_46039f0e,
                                    fi,
                                    &column_field_fi,
                                    column_field_for_er_message,
                                    true
                                ) {
                                    Ok(v_1f8de96a) => Ok(
                                        format!(#dq_ts0)
                                    ),
                                    Err(#ErSc) => Err(#ErSc)
                                }
                            }
                        },
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => {
                            let acc_41dea548_ts = quote!{acc_41dea548};
                            let select_qp_for_loop_ts = {
                                let v_dq_ts = dq_ts(&ValueSc);
                                gen_select_qp_for_loop_ts(
                                    &acc_41dea548_ts,
                                    &is_stdrt_with_id_true,
                                    &quote!{#VSc.#ident_with_id_stdrt_not_null_select_sc},
                                    &v_dq_ts,
                                    &v_dq_ts,
                                )
                            };
                            let format_ts = dq_ts(
                                &gen_jsonb_build_obj(
                                    &format!("'{{fi}}',{}", gen_jsonb_build_obj_v(
                                        &format!("case when (jsonb_array_length({{column_field}}->'{{fi}}') = 0) then '[]'::jsonb else (select jsonb_agg(({{{ident_with_id_stdrt_not_null_select_sc}}})) from jsonb_array_elements((select {{column_field}}->'{{fi}}')) with ordinality where ordinality between {{dim1_start}} and {{dim1_end}}) end "
                                    )))
                                )
                            );
                            quote! {
                                let #ident_with_id_stdrt_not_null_select_sc = {
                                    let mut #acc_41dea548_ts = #StringTs::default();
                                    #select_qp_for_loop_ts
                                    let _: Option<char> = #acc_41dea548_ts.pop();
                                    let _: Option<char> = #acc_41dea548_ts.pop();
                                    #acc_41dea548_ts
                                };
                                let dim1_start = #VSc.#dim1_pgn_ts.start();
                                let dim1_end = #VSc.#dim1_pgn_ts.end();
                                Ok(format!(#format_ts))
                            }
                        }
                        IsNullable::True => {
                            let format_ts = dq_ts(
                                &format!(
                                    "case when jsonb_typeof({{column_field}}->'{{fi}}') = 'null' then {} else ({{v_d7bbd03c}}) end",
                                    gen_jsonb_build_obj(&format!(
                                        "'{{fi}}',{}",
                                        gen_jsonb_build_obj_v(&"'null'::jsonb")
                                    ))
                                )
                            );
                            let ident_with_id_arr_not_null_as_pg_json_select_as_dflt_but_opt_is_some_ts = gen_ident_as_dflt_but_opt_is_some_ts(
                                &ident_with_id_arr_not_null_as_pg_json_select_ts
                            );
                            quote! {
                                let v_174d33cd = #VSc.0.as_ref().map_or_else(
                                    #ident_with_id_arr_not_null_as_pg_json_select_as_dflt_but_opt_is_some_ts,
                                    Clone::clone
                                );
                                match #ident_with_id_arr_not_null_as_pg_json_ts::#SelectQpSc(
                                    &v_174d33cd,
                                    fi,
                                    column_field,
                                    column_field_for_er_message,
                                    true
                                ) {
                                    Ok(v_d7bbd03c) => Ok(format!(#format_ts)),
                                    Err(#ErSc) => Err(#ErSc)
                                }
                            }
                        }
                    },
                },
                &ident_where_ucc,
                &ident_read_ucc,
                &ident_read_ids_ucc,
                &match &is_nullable {
                    IsNullable::False => {
                        let ts_a5fb2689 = {
                            let ts = {
                                let acc_push_ts = get_vec_syn_field(match &pattern {
                                    Pattern::Stdrt => &is_stdrt_with_id_false,
                                    Pattern::Arr => &is_stdrt_with_id_true
                                }).iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&format!("'{fi}',{{}}"));
                                    let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                    let ts = match &pattern {
                                        Pattern::Stdrt => {
                                            let format_ts_14808143 = dq_ts(&format!("{{column_field}}->'{fi}'"));
                                            quote! {&format!(#format_ts_14808143)}
                                        },
                                        Pattern::Arr => dq_ts(&format!("elem->'{fi}'"))
                                    };
                                    gen_if_write_is_err_curly_braces_ts(
                                        &quote!{
                                            acc_2912b128,
                                            #dq_ts0,
                                            match #ft_as_pg_json_ts::#SelectOnlyIdsQpSc(#ts) {
                                                Ok(v_2317e0af) => v_2317e0af,
                                                Err(#ErSc) => {
                                                    return Err(#ErSc);
                                                }
                                            }
                                        },
                                        &return_err_qp_er_write_into_buffer_ts
                                    )
                                });
                                let dq_ts0 = dq_ts(&gen_jsonb_build_obj_v(&"{acc_2912b128}"));
                                quote! {{
                                    let mut acc_2912b128 = #StringTs::default();
                                    #(#acc_push_ts)*
                                    let _: Option<char> = acc_2912b128.pop();
                                    let _: Option<char> = acc_2912b128.pop();
                                    format!(#dq_ts0)
                                }}
                            };
                            match &pattern {
                                Pattern::Stdrt => ts,
                                Pattern::Arr => {
                                    let format_ts = dq_ts(
                                        &gen_jsonb_build_obj_v(&format!("(select jsonb_agg({{}}) from jsonb_array_elements({{{ColumnFieldSc}}}) as elem)"))
                                    );
                                    quote! {format!(#format_ts, #ts)}
                                },
                            }
                        };
                        quote! {Ok(#ts_a5fb2689)}
                    },
                    IsNullable::True => {
                        let ts: &dyn ToTokens = match &pattern {
                            Pattern::Stdrt => &ident_stdrt_not_null_as_pg_json_ts,
                            Pattern::Arr => &ident_with_id_arr_not_null_as_pg_json_ts,
                        };
                        let case_null_format_ts = dq_ts(
                            &gen_jsonb_build_obj_v(&format!("case when jsonb_typeof({{{ColumnFieldSc}}})='null' then 'null'::jsonb else {{v_21000130}} end"))
                        );
                        quote! {
                            match #ts::#SelectOnlyIdsQpSc(#ColumnFieldSc) {
                                Ok(v_21000130) => Ok(format!(#case_null_format_ts)),
                                Err(#ErSc) => Err(#ErSc)
                            }
                        }
                    }
                },
                &ident_read_inn_ucc,
                &{
                    let gen_impl_into_inn_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts = |is_stdrt_with_id: &IsStdrtWithId| {
                        let ident_ts_df0c096c: &dyn ToTokens = match &is_stdrt_with_id {
                            IsStdrtWithId::False => &ident_read_inn_ucc,
                            IsStdrtWithId::True => &ident_with_id_stdrt_not_null_read_inn_ucc,
                        };
                        let ts = get_vec_syn_field(is_stdrt_with_id).iter().map(|el0| {
                            let fi = &el0.ident;
                            let ts = gen_v_init_ts0(&{
                                let ident_ts_c89f002f = gen_type_as_pg_json_ts(&el0.type0);
                                let params_ts = quote!{v_6e5af985.#VSc};
                                quote!{#ident_ts_c89f002f::into_inn(#params_ts)}
                            });
                            let param_ts: &dyn ToTokens = match &is_stdrt_with_id {
                                IsStdrtWithId::False => &VSc,
                                IsStdrtWithId::True => &quote!{el_34d57236},
                            };
                            quote! {#fi: #param_ts.#fi.map(|v_6e5af985| #ts)}
                        });
                        quote! {
                            #ident_ts_df0c096c {
                                #(#ts),*
                            }
                        }
                    };
                    match &is_nullable {
                        IsNullable::False => match &pattern {
                            Pattern::Stdrt => gen_impl_into_inn_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts(&IsStdrtWithId::False),
                            Pattern::Arr => {
                                let ts = gen_impl_into_inn_for_ident_read_or_ident_with_id_stdrt_not_null_read_ts(&IsStdrtWithId::True);
                                quote! {#VSc.0.into_iter().map(|el_34d57236|#ts).collect()}
                            },
                        },
                        IsNullable::True => {
                            let ident_ddcdad63 = gen_type_as_pg_json_ts(&match &pattern {
                                Pattern::Stdrt => ident_stdrt_not_null_ucc,
                                Pattern::Arr => ident_arr_not_null_ucc,
                            });
                            quote! {#VSc.0.map(#ident_ddcdad63::into_inn)}
                        }
                    }
                },
                &ident_upd_ucc,
                &ident_upd_for_query_ucc,
                &match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let format_ts = dq_ts(&format!("jsonb_set({{{JsonbSetAccumulatorSc}}},'{{{{{{{JsonbSetPathSc}}}}}}}',{{{StdOptOptObjAccSc}}})"));
                            let qp_vrts_ts = vec_syn_field.iter().map(|el0| {
                                let vrt_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el0.ident.to_string());
                                let fi_dq_ts = gen_fi_dq_ts(el0);
                                let ft_as_crud_pg_json_from_field_ts = gen_ft_as_crud_pg_json_from_field_ts(el0);
                                quote! {
                                    #ident_upd_for_query_el_ucc::#vrt_ident_ucc_ts(v_3b3fae4c) => {
                                        match #ft_as_crud_pg_json_from_field_ts::#UpdQpSc(
                                            &v_3b3fae4c.#VSc,
                                            &#StdOptOptObjAccSc,
                                            &#GenJsonbSetTargetSc(#fi_dq_ts),
                                            #fi_dq_ts,
                                            #IncrSc,
                                        ) {
                                            Ok(v_5edc1648) => {
                                                #StdOptOptObjAccSc = v_5edc1648;
                                            }
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            }
                                        }
                                    }
                                }
                            });
                            let some_format_ts = dq_ts(&format!("case when jsonb_typeof({{{JsonbSetTargetSc}}}) = 'object' then ({{{JsonbSetTargetSc}}})::jsonb else '{{{{}}}}'::jsonb end"));
                            let gen_jsonb_set_target_ts = {
                                let ts = dq_ts(&format!("{{{JsonbSetTargetSc}}}->'{{v_12d082b5}}'"));
                                quote! {
                                    let #GenJsonbSetTargetSc = |v_12d082b5: &str|{
                                        format!(#ts)
                                    };
                                }
                            };
                            quote! {
                                let mut #StdOptOptObjAccSc = format!(#some_format_ts);
                                #gen_jsonb_set_target_ts
                                for el in #VSc.0.to_vec() {
                                    match el {
                                        #(#qp_vrts_ts),*
                                    }
                                }
                                if #JsonbSetPathSc.is_empty() {
                                    Ok(#StdOptOptObjAccSc)
                                }
                                else {
                                    Ok(format!(#format_ts))
                                }
                            }
                        },
                        IsNullable::True => gen_upd_qp_stdrt_nullable_ts(
                            &pg_type_or_pg_json_pg_type
                        )
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => gen_upd_qp_arr_not_null_ts(
                            &pg_type_or_pg_json_pg_type
                        ),
                        IsNullable::True => quote! {
                            match &#VSc.0 {
                                Some(v_3245b79f) => #ident_arr_not_null_as_pg_json_ts::#UpdQpSc(
                                    v_3245b79f,
                                    jsonb_set_accumulator,
                                    jsonb_set_target,
                                    jsonb_set_path,
                                    #IncrSc,
                                ),
                                None => match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                    Ok(v_87e08bec) => Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${v_87e08bec})")),
                                    Err(#ErSc) => Err(#ErSc)
                                }
                            }
                        },
                    },
                },
                &IsUpdQpSelfUpdUsed::True,
                &IsUpdQpJsonbSetTargetUsed::True,
                &IsUpdQbMut::True,
                &match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let upd_qb_vrts_ts = vec_syn_field.iter().map(|el0| {
                                let vrt_ident_ucc_ts = AsRefStrToUccTs::case_or_panic(&el0.ident.to_string());
                                let ft_as_crud_pg_json_from_field_ts = gen_ft_as_crud_pg_json_from_field_ts(
                                    el0
                                );
                                quote! {
                                    #ident_upd_for_query_el_ucc::#vrt_ident_ucc_ts(v_b27f5b09) => {
                                        match #ft_as_crud_pg_json_from_field_ts::#UpdQbSc(
                                            v_b27f5b09.#VSc,
                                            #QuerySc
                                        ) {
                                            Ok(v_a4870bad) => {
                                                #QuerySc = v_a4870bad;
                                            },
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            }
                                        }
                                    }
                                }
                            });
                            quote! {
                                for el in #VSc.0.into_vec() {
                                    match el {
                                        #(#upd_qb_vrts_ts),*
                                    }
                                }
                                Ok(#QuerySc)
                            }
                        },
                        IsNullable::True => quote! {
                            match #VSc.0 {
                                Some(v_269a0d34) => #ident_stdrt_not_null_as_pg_json_ts::upd_qb(
                                    v_269a0d34,
                                    #QuerySc
                                ),
                                None => if let Err(#ErSc) = #QuerySc.try_bind(sqlx::types::Json(#self_as_pg_json_upd_ts::new(None))) {
                                    Err(#ErSc.to_string())
                                }
                                else {
                                    Ok(#QuerySc)
                                },
                            }
                        }
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => quote! {
                            for el in #VSc.#UpdSc.into_vec() {
                                match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::qb_string_as_pg_text_upd_for_query(
                                    el.#IdSc,
                                    #QuerySc
                                ) {
                                    Ok(v_7633dc9b) => {
                                        #QuerySc = v_7633dc9b;
                                    },
                                    Err(#ErSc) => {
                                        return Err(#ErSc);
                                    }
                                }
                                match #ident_stdrt_not_null_as_pg_json_ts::upd_qb(
                                    el.#FieldsSc,
                                    #QuerySc
                                ) {
                                    Ok(v_2073f07a) => {
                                        #QuerySc = v_2073f07a;
                                    },
                                    Err(#ErSc) => {
                                        return Err(#ErSc);
                                    }
                                }
                            }
                            for el in #VSc.del {
                                match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::qb_string_as_pg_text_upd_for_query(
                                    el,
                                    #QuerySc
                                ) {
                                    Ok(v_31262d92) => {
                                        #QuerySc = v_31262d92;
                                    },
                                    Err(#ErSc) => {
                                        return Err(#ErSc);
                                    }
                                }
                            }
                            for el in #VSc.#CrSc {
                                if let Err(#ErSc) = #QuerySc.try_bind(sqlx::types::Json(el)) {
                                    return Err(#ErSc.to_string());
                                }
                            }
                            Ok(#QuerySc)
                        },
                        IsNullable::True => quote! {
                            match #VSc.0 {
                                Some(v_a2156b3e) => #ident_arr_not_null_as_pg_json_ts::upd_qb(
                                    v_a2156b3e,
                                    #QuerySc
                                ),
                                None => if let Err(#ErSc) = #QuerySc.try_bind(sqlx::types::Json(#self_as_pg_json_upd_ts::new(None))) {
                                    Err(#ErSc.to_string())
                                }
                                else {
                                    Ok(#QuerySc)
                                },
                            }
                        },
                    },
                },
                &{
                    let dq_ts0 = dq_ts(&format!("'{{fi}}',{},", gen_jsonb_build_obj_v(&"{v_e137951b}")));
                    quote!{
                        match #VSc.#SelectOnlyUpddIdsQpSc(
                            &format!("{column_field}->'{fi}'"),
                            #IncrSc
                        ) {
                            Ok(v_e137951b) => Ok(format!(#dq_ts0)),
                            Err(#ErSc) => Err(#ErSc)
                        }
                    }
                },
                &IsSelectOnlyUpddIdsQbMut::True,
                &match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let match_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                let fi_ucc = ToTokensToUccTs::case_or_panic(&fi);
                                quote! {
                                    #ident_stdrt_not_null_upd_for_query_el_ucc::#fi_ucc(v_b79c2851) => {
                                        match #ft_as_pg_json_ts::#SelectOnlyUpddIdsQbSc(
                                            &v_b79c2851.#VSc,
                                            #QuerySc
                                        ) {
                                            Ok(v_e8914f75) => {
                                                #QuerySc = v_e8914f75;
                                            },
                                            Err(#ErSc) => {
                                                return Err(#ErSc);
                                            }
                                        }
                                    }
                                }
                            });
                            quote!{
                                for el in #VSc.0.to_vec() {
                                    match el {
                                        #(#match_ts),*
                                    }
                                }
                                Ok(#QuerySc)
                            }
                        },
                        IsNullable::True => quote!{
                            if let Some(v_6334d77d) = &#VSc.0 {
                                match #ident_stdrt_not_null_as_pg_json_ts::#SelectOnlyUpddIdsQbSc(v_6334d77d, #QuerySc) {
                                    Ok(v_0bd3ba6f) => {
                                        #QuerySc = v_0bd3ba6f;
                                    },
                                    Err(#ErSc) => {
                                        return Err(#ErSc);
                                    }
                                }
                            }
                            Ok(#QuerySc)
                        },
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => {
                            let select_only_crd_ids_qb_ts = vec_syn_field_with_id.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                quote! {
                                    match #ft_as_pg_json_ts::#SelectOnlyCrdIdsQbSc(
                                        &el_5fba4c1f.#fi,
                                        #QuerySc
                                    ) {
                                        Ok(v_cb81ec2c) => {
                                            #QuerySc = v_cb81ec2c;
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                            });
                            quote!{
                                for el in #VSc.#UpdSc.to_vec() {
                                    match #import_pg_json_uuid_uuid_as_not_null_jsonb_string_as_pg_json_ts::#SelectOnlyUpddIdsQbSc(
                                        &el.#IdSc,
                                        #QuerySc
                                    ) {
                                        Ok(v_0fd735de) => {
                                            #QuerySc = v_0fd735de;
                                        },
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                    match #ident_stdrt_not_null_as_pg_json_ts::#SelectOnlyUpddIdsQbSc(
                                        &el.fields,
                                        #QuerySc
                                    ) {
                                        Ok(v_4b52fa4f) => {
                                            #QuerySc = v_4b52fa4f;
                                        },
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                                for el_5fba4c1f in &#VSc.cr {
                                    #(#select_only_crd_ids_qb_ts)*
                                }
                                for el in #VSc.#UpdSc.to_vec() {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::qb_string_as_pg_text_upd_for_query(
                                        el.#IdSc.clone(),
                                        #QuerySc
                                    ) {
                                        Ok(v_b0da764b) => {
                                            #QuerySc = v_b0da764b;
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                                for el in &#VSc.#CrSc {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::qb_string_as_pg_text_cr_for_query(
                                        el.#IdSc.clone(),
                                        #QuerySc
                                    ) {
                                        Ok(v_dd8932e8) => {
                                            #QuerySc = v_dd8932e8;
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                                Ok(#QuerySc)
                            }
                        },
                        IsNullable::True => quote!{
                            if let Some(v_107e6639) = &#VSc.0 {
                                match #ident_arr_not_null_as_pg_json_ts::#SelectOnlyUpddIdsQbSc(v_107e6639, #QuerySc) {
                                    Ok(v_ecf1b8de) => {
                                        #QuerySc = v_ecf1b8de;
                                    },
                                    Err(#ErSc) => {
                                        return Err(#ErSc);
                                    }
                                }
                            }
                            Ok(#QuerySc)
                        },
                    },
                },
                &match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                let fi_dq_ts = &dq_ts(&fi);
                                let column_field_fi_dq_ts = &dq_ts(
                                    &format!("{{{ColumnFieldSc}}}->'{fi}'")
                                );
                                let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                    &{
                                        let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&"{v_cddc8a0a}");
                                        quote!{acc_0fe559fa, #dq_ts0}
                                    },
                                    &return_err_qp_er_write_into_buffer_ts
                                );
                                quote! {
                                    match #ft_as_pg_json_ts::#SelectOnlyCrdIdsQpSc(
                                        &#VSc.#fi,
                                        #fi_dq_ts,
                                        &format!(#column_field_fi_dq_ts),
                                        #IncrSc
                                    ) {
                                        Ok(mut v_cddc8a0a) => {
                                            let _: Option<char> = v_cddc8a0a.pop();
                                            #if_write_is_err_curly_braces_ts
                                        },
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                            });
                            let dq_ts0 = dq_ts(&format!("'{{fi}}',{},", gen_jsonb_build_obj_v(&"{}")));
                            quote!{
                                Ok(format!(
                                    #dq_ts0,
                                    {
                                        let mut acc_0fe559fa = #StringTs::new();
                                        #(#ts)*
                                        let _: Option<char> = acc_0fe559fa.pop();
                                        let _: Option<char> = acc_0fe559fa.pop();
                                        acc_0fe559fa
                                    }
                                ))
                            }
                        },
                        IsNullable::True => {
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                let fi_dq_ts = &dq_ts(&fi);
                                let column_field_fi_dq_ts = &dq_ts(
                                    &format!("{{{ColumnFieldSc}}}->'{fi}'")
                                );
                                let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                    &{
                                        let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&"{v_93015133}");
                                        quote!{acc_0e9170a3, #dq_ts0}
                                    },
                                    &return_err_qp_er_write_into_buffer_ts
                                );
                                quote! {
                                    match #ft_as_pg_json_ts::#SelectOnlyCrdIdsQpSc(
                                        &v_90219286.#fi,
                                        #fi_dq_ts,
                                        &format!(#column_field_fi_dq_ts),
                                        #IncrSc
                                    ) {
                                        Ok(mut v_93015133) => {
                                            let _: Option<char> = v_93015133.pop();
                                            #if_write_is_err_curly_braces_ts
                                        },
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                            });
                            let dq_ts0 = dq_ts(&format!("'{{fi}}'{},", gen_jsonb_build_obj_v(&"{}")));
                            let dq_ts1 = dq_ts(&gen_jsonb_build_obj_v(&"{}"));
                            quote!{
                                Ok(format!(
                                    #dq_ts0,
                                    match &#VSc.0 {
                                        Some(v_90219286) => format!(
                                            #dq_ts1,
                                            {
                                                let mut acc_0e9170a3 = #StringTs::new();
                                                #(#ts)*
                                                let _: Option<char> = acc_0e9170a3.pop();
                                                let _: Option<char> = acc_0e9170a3.pop();
                                                acc_0e9170a3
                                            }
                                        ),
                                        None => "'null'::jsonb".to_owned(),
                                    }
                                ))
                            }
                        },
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => {
                            let ts = vec_syn_field_with_id.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                let fi_dq_ts = &dq_ts(&fi);
                                let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                    &{
                                        let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&"{v_6d76c065}");
                                        quote!{acc_0f2b92d0, #dq_ts0}
                                    },
                                    &return_err_qp_er_write_into_buffer_ts
                                );
                                quote! {
                                    match #ft_as_pg_json_ts::#SelectOnlyCrdIdsQpSc(
                                        &el.#fi,
                                        #fi_dq_ts,
                                        "elem",
                                        #IncrSc
                                    ) {
                                        Ok(mut v_6d76c065) => {
                                            let _: Option<char> = v_6d76c065.pop();
                                            #if_write_is_err_curly_braces_ts
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                            });
                            let if_write_is_err_ts = gen_if_write_is_err_ts(
                                &quote!{acc_44b1f772, "${v_73b58d3a},"},
                                &return_err_qp_er_write_into_buffer_ts
                            );
                            let dq_ts0 = dq_ts(&format!("'{{fi}}',{},", gen_jsonb_build_obj_v(&"(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))")));
                            let dq_ts1 = dq_ts(&gen_jsonb_build_obj_v(&"{acc_0f2b92d0}"));
                            quote!{
                                Ok(format!(
                                    #dq_ts0,
                                    {
                                        let mut acc_0f2b92d0 = #StringTs::new();
                                        for el in &#VSc.0 {
                                            #(#ts)*
                                        }
                                        let _: Option<char> = acc_0f2b92d0.pop();
                                        let _: Option<char> = acc_0f2b92d0.pop();
                                        format!(#dq_ts1)
                                    },
                                    &format!("{column_field}->'{fi}'"),
                                    {
                                        let mut acc_44b1f772 = #StringTs::new();
                                        for _ in &#VSc.0 {
                                            match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                                Ok(v_73b58d3a) => {
                                                    #if_write_is_err_ts
                                                },
                                                Err(#ErSc) => {
                                                    return Err(#ErSc);
                                                },
                                            }
                                        }
                                        let _: Option<char> = acc_44b1f772.pop();
                                        acc_44b1f772
                                    }
                                ))
                            }
                        },
                        IsNullable::True => {
                            let ts = vec_syn_field_with_id.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                let fi_dq_ts = &dq_ts(&fi);
                                let if_write_is_err_curly_braces_ts = gen_if_write_is_err_curly_braces_ts(
                                    &{
                                        let dq_ts0 = gen_jsonb_build_obj_or_dq_ts(&"{v_d49fe9d8}");
                                        quote!{acc_1a91bdc7, #dq_ts0}
                                    },
                                    &return_err_qp_er_write_into_buffer_ts
                                );
                                quote! {
                                    match #ft_as_pg_json_ts::#SelectOnlyCrdIdsQpSc(
                                        &el_9bdcd847.#fi,
                                        #fi_dq_ts,
                                        "elem",
                                        #IncrSc
                                    ) {
                                        Ok(mut v_d49fe9d8) => {
                                            let _: Option<char> = v_d49fe9d8.pop();
                                            #if_write_is_err_curly_braces_ts
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                            });
                            let if_write_is_err_ts = gen_if_write_is_err_ts(
                                &quote!{acc_857ce631, "${v_7f11bec0},"},
                                &return_err_qp_er_write_into_buffer_ts
                            );
                            let dq_ts0 = dq_ts(&format!("'{{fi}}',{},", gen_jsonb_build_obj_v(&"{}")));
                            let dq_ts1 = dq_ts(&gen_jsonb_build_obj_v(&"(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))"));
                            let dq_ts2 = dq_ts(&gen_jsonb_build_obj_v(&"{acc_1a91bdc7}"));
                            quote!{
                                Ok(format!(
                                    #dq_ts0,
                                    match &#VSc.0 {
                                        Some(v_3c415c92) => format!(
                                            #dq_ts1,
                                            {
                                                let mut acc_1a91bdc7 = #StringTs::new();
                                                for el_9bdcd847 in &v_3c415c92.0 {
                                                    #(#ts)*
                                                }
                                                let _: Option<char> = acc_1a91bdc7.pop();
                                                let _: Option<char> = acc_1a91bdc7.pop();
                                                format!(#dq_ts2)
                                            },
                                            &format!("{column_field}->'{fi}'"),
                                            {
                                                let mut acc_857ce631 = #StringTs::new();
                                                for _ in &v_3c415c92.0 {
                                                    match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                                        Ok(v_7f11bec0) => {
                                                            #if_write_is_err_ts
                                                        },
                                                        Err(#ErSc) => {
                                                            return Err(#ErSc);
                                                        },
                                                    }
                                                }
                                                let _: Option<char> = acc_857ce631.pop();
                                                acc_857ce631
                                            }
                                        ),
                                        None => "'null'::jsonb".to_owned(),
                                    }
                                ))
                            }
                        },
                    },
                },
                &IsSelectOnlyCrdIdsQbMut::True,
                &match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                quote! {
                                    match #ft_as_pg_json_ts::#SelectOnlyCrdIdsQbSc(
                                        &#VSc.#fi,
                                        #QuerySc
                                    ) {
                                        Ok(v_231618d9) => {
                                            #QuerySc = v_231618d9;
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                            });
                            quote!{
                                #(#ts)*
                                Ok(#QuerySc)
                            }
                        },
                        IsNullable::True => {
                            quote!{
                                if let Some(v_a1ccd526) = &#VSc.0 {
                                    match #ident_stdrt_not_null_as_import_pg_json_ts::#SelectOnlyCrdIdsQbSc(
                                        v_a1ccd526,
                                        #QuerySc
                                    ) {
                                        Ok(v_70ed6013) => {
                                            #QuerySc = v_70ed6013;
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                                Ok(#QuerySc)
                            }
                        },
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => {
                            let ts = vec_syn_field_with_id.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&el0.type0);
                                quote! {
                                    match #ft_as_pg_json_ts::#SelectOnlyCrdIdsQbSc(&el_9bdcd847.#fi, #QuerySc) {
                                        Ok(v_ade27463) => {
                                            #QuerySc = v_ade27463;
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                            });
                            quote!{
                                for el_9bdcd847 in &#VSc.0 {
                                    #(#ts)*
                                }
                                for el in &#VSc.0 {
                                    match #uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::qb_string_as_pg_text_cr_for_query(
                                        el.#IdSc.clone(),
                                        #QuerySc
                                    ) {
                                        Ok(v_a3749ea8) => {
                                            #QuerySc = v_a3749ea8;
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                                Ok(#QuerySc)
                            }
                        },
                        IsNullable::True => {
                            quote!{
                                if let Some(v_0b55a65a) = &#VSc.0 {
                                    match #ident_arr_not_null_as_import_pg_json_ts::#SelectOnlyCrdIdsQbSc(v_0b55a65a, #QuerySc) {
                                        Ok(v_ad6a1ac5) => {
                                            #QuerySc = v_ad6a1ac5;
                                        }
                                        Err(#ErSc) => {
                                            return Err(#ErSc);
                                        }
                                    }
                                }
                                Ok(#QuerySc)
                            }
                        },
                    },
                },
            );
            let impl_pg_crud_pg_types_pg_type_pg_type_ts = gen_impl_pg_type_ts(
                &Import::PgCrud,
                &ident,
                &ident_table_type_ucc,
                &IsPkUndrscr::True,
                &{
                    let format_ts = dq_ts(&"{column} jsonb not null check (jsonb_matches_schema('{}', {column}))".to_owned());
                    quote! {
                        format!(#format_ts, serde_json::to_string(&schemars::schema_for!(#ident_table_type_ucc)).expect("59a1654b"))
                    }
                },
                &ident_cr_ucc,
                &CrQpValueUndrscr::True,
                &CrQpIncrUndrscr::False,
                &quote!{
                    match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                        Ok(v_7df9eb00) => Ok(format!("${v_7df9eb00}")),
                        Err(#ErSc) => Err(#ErSc)
                    }
                },
                &CrQbValueUndrscr::False,
                &IsCrQbMut::True,
                &quote!{
                    if let Err(#ErSc) = #QuerySc.try_bind(
                        #self_as_pg_json_cr_for_query_ts::from(#VSc)
                    ) {
                        return Err(#ErSc.to_string());
                    }
                    Ok(#QuerySc)
                },
                &ident_select_ucc,
                &SelectQpValueUndrscr::False,
                &quote! {
                    match #VSc.#SelectQpPgTypeSc(#ColumnSc) {
                        Ok(v_d91c19a6) => Ok(format!("{v_d91c19a6} as {column}")),
                        Err(#ErSc) => Err(#ErSc)
                    }
                },
                &ident_where_ucc,
                &ident_read_ucc,
                &VSc,
                &ident_read_ids_ucc,
                &quote! {
                    match #self_as_pg_json_ts::#SelectOnlyIdsQpSc(#ColumnSc) {
                        Ok(v_e776e9fa) => Ok(format!("{v_e776e9fa} as {column},")),
                        Err(#ErSc) => Err(#ErSc)
                    }
                },
                &ident_read_inn_ucc,
                &quote!{#self_as_pg_json_ts::into_inn(#VSc)},
                &ident_upd_ucc,
                &ident_upd_for_query_ucc,
                &UpdQpValueUndrscr::False,
                &UpdQpJsonbSetAccumulatorUndrscr::False,
                &UpdQpJsonbSetTargetUndrscr::False,
                &UpdQpJsonbSetPathUndrscr::False,
                &match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => quote!{#self_as_pg_json_ts::#UpdQpSc(
                            #VSc,
                            jsonb_set_accumulator,
                            jsonb_set_target,
                            jsonb_set_path,
                            incr
                        )},
                        IsNullable::True => gen_upd_qp_stdrt_nullable_ts(
                            &pg_type_or_pg_json_pg_json
                        )
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => gen_upd_qp_arr_not_null_ts(
                            &pg_type_or_pg_json_pg_json
                        ),
                        IsNullable::True => {
                            let ts = gen_upd_del_cr_arr_ts(&quote!{
                                "(case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({upd_qp_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {mb_where}),'[]'::jsonb)) end {mb_jsonb_build_arr})"
                            });
                            quote! {
                                match &#VSc.0 {
                                    Some(v_58d685d3) => {
                                        #ts
                                    },
                                    None => match #import::incr_checked_add_one_returning_incr(#IncrSc) {
                                        Ok(v_d31ab6f0) => Ok(format!("${v_d31ab6f0}")),
                                        Err(#ErSc) => Err(#ErSc)
                                    }
                                }
                            }
                        },
                    },
                },
                &IsUpdQbMut::False,
                &quote!{#self_as_pg_json_ts::#UpdQbSc(
                    #VSc,
                    #QuerySc
                )},
                &{
                    let dq_ts0 = dq_ts(&format!("{} as {{column}},", gen_jsonb_build_obj_v(&"{v_f0787243}")));
                    quote!{
                        match #VSc.#SelectOnlyUpddIdsQpSc(
                            #ColumnSc,
                            #IncrSc
                        ) {
                            Ok(v_f0787243) => Ok(format!(#dq_ts0)),
                            Err(#ErSc) => Err(#ErSc)
                        }
                    }
                },
                &IsSelectOnlyUpddIdsQbMut::False,
                &quote!{#self_as_pg_json_ts::#SelectOnlyUpddIdsQbSc(
                    #VSc,
                    #QuerySc
                )},
            );
            match &trait_gen {
                TraitGen::PgTypeAndPgJson => (impl_pg_crud_pg_json_for_ident_ts, impl_pg_crud_pg_types_pg_type_pg_type_ts),
                TraitGen::PgJson => (impl_pg_crud_pg_json_for_ident_ts, Ts2::new()),
            }
        };
        let self_pg_json_ts = quote!{#SelfUcc::#PgJsonUcc};
        let impl_pg_json_test_cases_for_ident_ts = {
            let opt_vec_cr_ts = {
                let ts = match &is_nullable {
                    IsNullable::False => match &pattern {
                        Pattern::Stdrt => {
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_type_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                let params_ts = vec_syn_field.iter().map(|el1| {
                                    let fi0 = &el1.ident;
                                    if fi == fi0 {
                                        quote! {el_37154498}
                                    } else {
                                        quote! {
                                            #PgCrudDfltOptSomeVecOneElCall
                                        }
                                    }
                                });
                                quote! {
                                    if let Some(v_0296b347) = #ft_type_as_pg_json_test_cases_ts::#OptVecCrSc() {
                                        for el_37154498 in v_0296b347 {
                                            let #VSc = #self_as_pg_json_cr_ts::new(
                                                #(#params_ts),*
                                            );
                                            if !acc_ccd79a32.contains(&#VSc) {
                                                acc_ccd79a32.push(#VSc);
                                            }
                                        }
                                    }
                                }
                            });
                            quote! {#(#ts)*}
                        },
                        Pattern::Arr => {
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_type_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                let (
                                    opt_extra_params_ts,
                                    params_ts
                                ) = {
                                    #[derive(Clone, Optml)]
                                    enum AddDotClone {
                                        False,
                                        True,
                                    }
                                    let gen_ts = |
                                        add_dot_clone: AddDotClone,
                                        el_ts: &dyn ToTokens,
                                    |{
                                        vec_syn_field.iter().map(|el1| {
                                            let fi0 = &el1.ident;
                                            if fi == fi0 {
                                                let mb_dot_clone_ts = match add_dot_clone.clone() {
                                                    AddDotClone::False => Ts2::new(),
                                                    AddDotClone::True => quote! { .clone() },
                                                };
                                                quote! {#el_ts #mb_dot_clone_ts}
                                            } else {
                                                quote! {#PgCrudDfltOptSomeVecOneElCall}
                                            }
                                        }).collect::<Vec<Ts2>>()
                                    };
                                    (
                                        gen_ts(
                                            AddDotClone::True,
                                            &quote!{el_37154498}
                                        ),
                                        gen_ts(
                                            AddDotClone::False,
                                            &quote!{el_37154498}
                                        )
                                    )
                                };
                                quote! {
                                    if let Some(vec_cr) = #ft_type_as_pg_json_test_cases_ts::#OptVecCrSc() {
                                        let mut acc_6a886d56 = Vec::new();
                                        let opt_extra = {
                                            let mut opt_extra = None;
                                            for el_37154498 in &vec_cr {
                                                if opt_extra.is_none() {
                                                    let #VSc = #ident_with_id_stdrt_not_null_cr_ucc::new(
                                                        #(#opt_extra_params_ts),*
                                                    );
                                                    opt_extra = Some((
                                                        #ident_cr_ucc::new(vec![#VSc.clone()]),
                                                        #ident_cr_ucc::new(vec![#VSc.clone(), #VSc])
                                                    ));
                                                }
                                                else {
                                                    break;
                                                }
                                            }
                                            opt_extra
                                        };
                                        let has_len_greater_than_one = vec_cr.len() > 1;
                                        for el_37154498 in vec_cr {
                                            acc_6a886d56.push(#ident_with_id_stdrt_not_null_cr_ucc::new(
                                                #(#params_ts),*
                                            ));
                                        }
                                        {
                                            let v_07c0c08c = #ident_cr_ucc::new(acc_6a886d56);
                                            if !acc_ccd79a32.contains(&v_07c0c08c) {
                                                acc_ccd79a32.push(v_07c0c08c);
                                            }
                                        }
                                        if let Some(v_f6686d5d) = opt_extra {
                                            if has_len_greater_than_one {
                                                let v_60116463 = v_f6686d5d.0;
                                                if !acc_ccd79a32.contains(&v_60116463) {
                                                    acc_ccd79a32.push(v_60116463);
                                                }
                                            }
                                            else {
                                                let v_7a843059 = v_f6686d5d.1;
                                                if !acc_ccd79a32.contains(&v_7a843059) {
                                                    acc_ccd79a32.push(v_7a843059);
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                            quote! {#(#ts)*}
                        },
                    },
                    IsNullable::True => {
                        let (
                            ident_not_null_as_pg_json_test_cases_ts_e690553a,
                            ts
                        ): (
                            &dyn ToTokens,
                            &dyn ToTokens
                        ) = match &pattern {
                            Pattern::Stdrt => (
                                &ident_stdrt_not_null_as_pg_json_test_cases_ts,
                                &Ts2::new()
                            ),
                            Pattern::Arr => (
                                &ident_arr_not_null_as_pg_json_test_cases_ts,
                                &quote!{.0}
                            ),
                        };
                        quote! {
                            if let Some(v_399e6a50) = #ident_not_null_as_pg_json_test_cases_ts_e690553a::#OptVecCrSc() {
                                for el in v_399e6a50 {
                                    let #VSc = #self_as_pg_json_ts::Cr::new(Some(el #ts));
                                    if !acc_ccd79a32.contains(&#VSc) {
                                        acc_ccd79a32.push(#VSc);
                                    }
                                }
                            }
                            {
                                let #VSc = #self_as_pg_json_ts::Cr::new(None);
                                if !acc_ccd79a32.contains(&#VSc) {
                                    acc_ccd79a32.push(#VSc);
                                }
                            }
                        }
                    }
                };
                quote!{Some({
                    let mut acc_ccd79a32 = Vec::new();
                    #ts
                    acc_ccd79a32
                })}
            };
            let read_ids_to_2_dims_vec_read_inn_ts = match &pattern {
                Pattern::Stdrt => match &is_nullable {
                    IsNullable::False => {
                        let fields_last_init_ts = {
                            if vec_syn_field.len() == 1 {
                                Ts2::new()
                            }
                            else {
                                let ts = vec_syn_field.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let fi_last_sc = SelfLastSc::from_display(&fi);
                                    let ft_type_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                    quote! {
                                        let mut #fi_last_sc = #ft_type_as_pg_json_test_cases_ts::#ReadIdsIntoOptVReadInnSc(
                                            read_ids.0.#VSc.#fi.clone()
                                        );
                                    }
                                });
                                quote!{#(#ts)*}
                            }
                        };
                        let ts = vec_syn_field.iter().map(|el0| {
                            let fi = &el0.ident;
                            let fi_current_sc = SelfCurrentSc::from_display(&fi);
                            let fi_last_sc = SelfLastSc::from_display(&fi);
                            let mb_fi_last_clone_from_fi_current = if vec_syn_field.len() == 1 {
                                Ts2::new()
                            }
                            else {
                                quote!{#fi_last_sc.clone_from(&#fi_current_sc);}
                            };
                            let fields_ts = vec_syn_field.iter().map(|el1| {//todo rename
                                let fi0 = &el1.ident;
                                let fi0_current_sc = SelfCurrentSc::from_display(&fi0);
                                let fi0_last_sc = SelfLastSc::from_display(&fi0);
                                let ts: &dyn ToTokens = if fi == fi0 {
                                    &fi0_current_sc
                                } else {
                                    &fi0_last_sc
                                };
                                quote! {#fi0: #ts.clone()}
                            });
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                            let ts0 = gen_v_init_ts0(&quote!{el_2720df8a});
                            quote! {
                                for el_7bf83754 in #ft_as_pg_json_test_cases_ts::#ReadIdsTo2DimsVecReadInnSc(&#ReadIdsSc.0.#VSc.#fi) {
                                    for el_2720df8a in el_7bf83754 {
                                        let #fi_current_sc = Some(#ts0);
                                        #mb_fi_last_clone_from_fi_current
                                        acc_ef081dc3.push(
                                            vec![
                                                #ident_stdrt_not_null_read_inn_ucc {
                                                    #(#fields_ts),*
                                                }
                                            ]
                                        );
                                    }
                                }
                            }
                        });
                        quote! {
                            let mut acc_ef081dc3 = Vec::new();
                            #fields_last_init_ts
                            #(#ts)*
                            acc_ef081dc3
                        }
                    }
                    IsNullable::True => {
                        quote! {
                            #ReadIdsSc.0.#VSc.as_ref().into_iter().flat_map(|v_5fa0668c| {
                                #ident_stdrt_not_null_as_pg_json_test_cases_ts::
                                    #ReadIdsTo2DimsVecReadInnSc(v_5fa0668c)
                                    .into_iter()
                                    .flat_map(|el0| {
                                        el0.into_iter().map(|el1| vec![Some(el1)])
                                    })
                            })
                            .chain(std::iter::once(vec![None]))
                            .collect()
                        }
                    }
                },
                Pattern::Arr => match &is_nullable {
                    IsNullable::False => {
                        let ts = vec_syn_field.iter().map(|el0| {
                            let fi = &el0.ident;
                            let fields_ts = vec_syn_field.iter().map(|el1| {
                                let fi_dd46e0cb = &el1.ident;
                                if fi == fi_dd46e0cb {
                                    let ts0 = gen_v_init_ts0(&quote!{el_18d1f553});
                                    quote! {
                                        #fi_dd46e0cb: Some(#ts0)
                                    }
                                } else {
                                    let ft_as_pg_json_test_cases_ts_64dc25bd = gen_type_as_pg_json_test_cases_ts(&el1.type0);
                                    quote! {
                                        #fi_dd46e0cb: #ft_as_pg_json_test_cases_ts_64dc25bd::#ReadIdsIntoOptVReadInnSc(
                                            el.0.#VSc.#fi_dd46e0cb.clone()
                                        )
                                    }
                                }
                            });
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                            let ts0 = gen_v_init_ts0(&quote!{el.0.#VSc.#IdSc.0.#VSc});
                            quote! {
                                for el_4b4da5aa in #ft_as_pg_json_test_cases_ts::#ReadIdsTo2DimsVecReadInnSc(
                                    &el.0.#VSc.#fi.clone()
                                ) {
                                    for el_18d1f553 in el_4b4da5aa {
                                        acc_00b3df88.push(
                                            vec![
                                                #ident_with_id_stdrt_not_null_read_inn_ucc {
                                                    #IdSc: Some(#ts0),
                                                    #(#fields_ts),*
                                                }
                                            ]
                                        );
                                    }
                                }
                            }
                        });
                        quote! {
                            #ReadIdsSc.0.#VSc.iter().map(|el|{
                                let mut acc_00b3df88 = Vec::new();
                                #(#ts)*
                                acc_00b3df88
                            })
                            .collect()
                        }
                    }
                    IsNullable::True => {
                        quote! {
                            let mut acc_fb5111f1 = Vec::new();
                            if let Some(v_6ee5750e) = &#ReadIdsSc.0.#VSc {
                                for el_4a5a4b09 in #ident_arr_not_null_as_pg_json_test_cases_ts::#ReadIdsTo2DimsVecReadInnSc(v_6ee5750e) {
                                    for el_264980ec in el_4a5a4b09 {
                                        acc_fb5111f1.push(vec![Some(el_264980ec)]);
                                    }
                                }
                            }
                            acc_fb5111f1.push(vec![None]);
                            acc_fb5111f1
                        }
                    }
                },
            };
            let read_inn_into_read_with_new_or_try_new_unwraped_ts = match &pattern {
                Pattern::Stdrt => match &is_nullable {
                    IsNullable::False => {
                        let self_el_as_pg_type_read_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_ts, &PgTypeSubtype::Read);
                        let params_ts = vec_syn_field.iter().map(|el0| {
                            let fi = &el0.ident;
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                            let ts = gen_v_init_ts0(&quote!{
                                #ft_as_pg_json_test_cases_ts::#ReadInnIntoReadWithNewOrTryNewUnwrapedSc(v_8ff65e09.#VSc)
                            });
                            quote! {#VSc.#fi.map(|v_8ff65e09|#ts)}
                        });
                        quote! {#self_el_as_pg_type_read_ts::try_new(#(#params_ts),*).expect("3aeeabba")}
                    }
                    IsNullable::True => {
                        let self_el_as_pg_type_read_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_ts, &PgTypeSubtype::Read);
                        quote! {
                            #self_el_as_pg_type_read_ts::new(
                                #VSc.map(#ident_stdrt_not_null_as_pg_json_test_cases_ts::#ReadInnIntoReadWithNewOrTryNewUnwrapedSc)
                            )
                        }
                    }
                },
                Pattern::Arr => match &is_nullable {
                    IsNullable::False => {
                        let ts = vec_syn_field_with_id.iter().map(|el0| {
                            let fi = &el0.ident;
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                            let ts0 = gen_v_init_ts0(&quote!{
                                #ft_as_pg_json_test_cases_ts::#ReadInnIntoReadWithNewOrTryNewUnwrapedSc(v_3ac52220.#VSc)
                            });
                            quote! {#fi: el_ffed1bfc.#fi.map(|v_3ac52220|#ts0)}
                        });
                        quote!{
                            #ident_read_ucc::new(
                                #VSc.into_iter().map(|el_ffed1bfc| #ident_with_id_stdrt_not_null_read_ucc {
                                    #(#ts),*
                                }).collect()
                            )
                        }
                    }
                    IsNullable::True => {
                        let ts = vec_syn_field_with_id.iter().map(|el0| {
                            let fi = &el0.ident;
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                            // let mb_dot_clone_ts = if vec_syn_field.len() == 1 {
                            //     Ts2::new()
                            // }
                            // else {
                            //     quote!{.clone()}
                            // };
                            let ts0 = gen_v_init_ts0(&quote!{
                                #ft_as_pg_json_test_cases_ts::#ReadInnIntoReadWithNewOrTryNewUnwrapedSc(
                                    el_5c1f7f63.#VSc
                                    // #mb_dot_clone_ts
                                    .clone()
                                )
                            });
                            quote! {
                                #fi: el_ffed1bfc.#fi.as_ref().map(|el_5c1f7f63| #ts0)
                            }
                        });
                        let self_el_as_pg_type_read_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_ts, &PgTypeSubtype::Read);
                        quote! {
                            #self_el_as_pg_type_read_ts::new(
                                #VSc.map(|v_189e3c07|
                                    v_189e3c07
                                    .into_iter()
                                    .map(|el_ffed1bfc|#ident_with_id_stdrt_not_null_read_ucc {
                                        #(#ts),*
                                    }).collect()
                                )
                            )
                        }
                    }
                },
            };
            let read_inn_into_upd_with_new_or_try_new_unwraped_ts = match &is_nullable {
                IsNullable::False => match &pattern {
                    Pattern::Stdrt => {
                        let self_el_as_pg_type_upd_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_ts, &PgTypeSubtype::Upd);
                        let params_ts = vec_syn_field.iter().map(|el0| {
                            let fi = &el0.ident;
                            let fi_ucc = &ToTokensToUccTs::case_or_panic(&fi);
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                            let ts = gen_v_init_ts0(&quote!{
                                #ft_as_pg_json_test_cases_ts::#ReadInnIntoUpdWithNewOrTryNewUnwrapedSc(el_23bdfe1e.#VSc)
                            });
                            quote! {
                                acc_ebea163e.extend(#VSc.#fi.map(|el_23bdfe1e| {
                                    #ident_stdrt_not_null_upd_el_ucc::#fi_ucc(#ts)
                                }));
                            }
                        });
                        quote! {
                            #self_el_as_pg_type_upd_ts::new(
                                #import::NotEmptyUniqueVec::try_new({
                                    let mut acc_ebea163e = Vec::new();
                                    #(#params_ts)*
                                    acc_ebea163e
                                }).expect("a06dbdc5")
                            )
                        }
                    },
                    Pattern::Arr => {
                        let fields_ts = vec_syn_field.iter().map(|el0| {
                            let fi = &el0.ident;
                            quote! {#fi: el_ffed1bfc.#fi}
                        });
                        quote! {
                            #ident_upd_ucc::try_new(
                                Vec::new(),
                                #import_unique_vec_ident_with_id_stdrt_not_null_upd_el_ts::try_new(
                                    #VSc.into_iter().map(|el_ffed1bfc| #ident_with_id_stdrt_not_null_upd_el_ucc {
                                        #IdSc: #uuid_uuid_as_not_null_jsonb_string_upd_ucc::new(el_ffed1bfc.#IdSc.clone().expect("f04a2c6d").#VSc),
                                        fields: #ident_stdrt_not_null_as_pg_json_test_cases_ts::#ReadInnIntoUpdWithNewOrTryNewUnwrapedSc(
                                            #ident_stdrt_not_null_read_inn_ucc {
                                                #(#fields_ts),*
                                            }
                                        ),
                                    })
                                    .collect(),
                                )
                                .expect("ca51d559"),
                                Vec::new(),
                            )
                            .expect("0449fe82")
                        }
                    }
                },
                IsNullable::True => {
                    let ts = gen_type_as_pg_type_test_cases_ts(match &pattern {
                        Pattern::Stdrt => &ident_stdrt_not_null_ucc,
                        Pattern::Arr => &ident_with_id_arr_not_null_ucc,
                    });
                    let self_el_as_pg_type_upd_ts = gen_type_as_pg_type_subtype_ts(&self_pg_json_ts, &PgTypeSubtype::Upd);
                    quote! {
                        #self_el_as_pg_type_upd_ts::new(
                            #VSc.map(#ts::#ReadInnIntoUpdWithNewOrTryNewUnwrapedSc)
                        )
                    }
                },
            };
            let read_ids_into_opt_v_read_inn_ts = match &pattern {
                Pattern::Stdrt => match &is_nullable {
                    IsNullable::False => gen_fields_read_ids_into_opt_v_read_inn_ts(&is_stdrt_with_id_false, &VSc),
                    IsNullable::True => {
                        let ts = gen_v_init_ts0(&quote!{
                            #VSc.0.#VSc.and_then(|v_5d7e3961| match #ident_stdrt_not_null_as_pg_json_test_cases_ts::read_ids_into_opt_v_read_inn(
                                v_5d7e3961
                            ) {
                                Some(v_cfca0099) => Some(v_cfca0099.#VSc),
                                None => None,
                            })
                        });
                        quote! {Some(#ts)}
                    }
                },
                Pattern::Arr => match &is_nullable {
                    IsNullable::False => {
                        let ts = gen_v_init_ts0(&{
                            let ts0 = vec_syn_field_with_id.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft = &el0.type0;
                                let ft_as_pg_json_ts = gen_type_as_pg_json_ts(&ft);
                                let ft_as_pg_json_read_ts = gen_type_as_pg_json_subtype_ts(&ft, &PgJsonSubtype::Read);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                                let ts1 = gen_v_init_ts0(&{
                                    let dflt_but_opt_is_some_call_ts_a3f714b3 = gen_ident_as_dflt_but_opt_is_some_call_ts(
                                        &ft_as_pg_json_read_ts
                                    );
                                    quote!{#ft_as_pg_json_ts::into_inn(#dflt_but_opt_is_some_call_ts_a3f714b3)}
                                });
                                quote! {
                                    #fi: #ft_as_pg_json_test_cases_ts::read_ids_into_opt_v_read_inn(
                                        el_6603f209.0.#VSc.#fi
                                    ).map_or_else(|| Some(#ts1), Some)
                                }
                            });
                            quote!{
                                #VSc.0.#VSc.into_iter().fold(Vec::new(), |mut acc_cf4743b1, el_6603f209| {
                                    acc_cf4743b1.push(#ident_with_id_stdrt_not_null_read_inn_ucc {
                                        #(#ts0),*
                                    });
                                    acc_cf4743b1
                                })
                            }
                        });
                        quote! {Some(#ts)}
                    }
                    IsNullable::True => {
                        let ts = gen_v_init_ts0(&quote!{
                            #VSc.0.#VSc.and_then(|v_f816032d| match #ident_arr_not_null_as_pg_json_test_cases_ts::#ReadIdsIntoOptVReadInnSc(
                                v_f816032d
                            ) {
                                Some(v_d0549423) => Some(v_d0549423.#VSc),
                                None => None,
                            })
                        });
                        quote! {Some(#ts)}
                    }
                },
            };
            let upd_to_read_ids_ts = {
                let ts = match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let fields_init_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                quote! {let mut #fi = None;}
                            });
                            let match_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let fi_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    #ident_upd_el_ucc::#fi_ucc_ts(v_0f4d677e) => {
                                        #fi = Some(#ft_as_pg_json_test_cases_ts::#UpdToReadIdsSc(&v_0f4d677e.#VSc));
                                    }
                                }
                            });
                            let struct_fields_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                quote! {#fi: #fi.expect("106f16f2")}
                            });
                            let ts0 = gen_v_init_ts0(&quote!{
                                #ident_read_ids_h_ucc{
                                    #(#struct_fields_ts),*
                                }
                            });
                            quote! {{
                                #(#fields_init_ts)*
                                for el_b3974846 in #VSc.0.to_vec() {
                                    match el_b3974846 {
                                        #(#match_ts),*
                                    }
                                }
                                #ts0
                            }}
                        }
                        IsNullable::True => gen_v_init_ts0(&{
                            quote!{
                                #VSc.0.as_ref().map(#ident_stdrt_not_null_as_pg_json_test_cases_ts::#UpdToReadIdsSc)
                            }
                        })
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => gen_v_init_ts0(&{
                            let init_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                quote! {let mut #fi = None;}
                            });
                            let for_loop_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let fi_ts = {
                                    let fi_ucc_ts_fe80e842 = ToTokensToUccTs::case_or_panic(&fi);
                                    quote!{
                                        #ident_stdrt_not_null_upd_el_ucc::#fi_ucc_ts_fe80e842(v_d2a6daf8) => {
                                            #fi = Some(v_d2a6daf8.#VSc.clone());
                                        },
                                    }
                                };
                                //todo wtf
                                let fields_without_ident_ts_11eab22a = if vec_syn_field.is_empty() {
                                    Ts2::new()
                                }
                                else {
                                    let ts_e0bf4e67 = vec_syn_field
                                    .iter()
                                    .filter(|el1| el1.ident != *fi)
                                    .map(|el1| {
                                        let el1_fi = &el1.ident;
                                        let el1_fi_ucc_ts =
                                            ToTokensToUccTs::case_or_panic(
                                                &el1_fi,
                                            );
                                        quote! {
                                            #ident_stdrt_not_null_upd_el_ucc::#el1_fi_ucc_ts(_)
                                        }
                                    })
                                    .fold(None, |acc_bbf653f7, el1| Some(match acc_bbf653f7 {
                                        None => el1,
                                        Some(v_5b375af0) => quote! { #v_5b375af0 | #el1 },
                                    }));
                                    ts_e0bf4e67.map_or_else(
                                        Ts2::new,
                                        |v_5c826b8c| quote!{#v_5c826b8c => (),}
                                    )
                                };
                                quote! {
                                    for el_da177c5a in el_4634bb8a.fields.0.to_vec() {
                                        match &el_da177c5a {
                                            #fi_ts
                                            #fields_without_ident_ts_11eab22a
                                        }
                                    }
                                }
                            });
                            let ts1 = gen_v_init_ts0(&{
                                let uuid_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&uuid_uuid_as_not_null_jsonb_string_ts);
                                let fields_ts = vec_syn_field.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                    quote! {
                                        #fi: #ft_as_pg_json_test_cases_ts::#UpdToReadIdsSc(&#fi.expect("a3ec7cae"))
                                    }
                                });
                                quote!{
                                    #ident_with_id_stdrt_not_null_read_ids_h_ucc {
                                        #IdSc: #uuid_as_pg_json_test_cases_ts::#UpdToReadIdsSc(&el_4634bb8a.#IdSc),
                                        #(#fields_ts),*
                                    }
                                }
                            });
                            quote!{
                                #VSc.#UpdSc.to_vec().iter().map(|el_4634bb8a|{
                                    #(#init_ts)*
                                    #(#for_loop_ts)*
                                    #ident_with_id_stdrt_not_null_read_ids_ucc(#ts1)
                                }).collect()
                            }
                        }),
                        IsNullable::True => gen_v_init_ts0(&quote!{
                            #VSc.0.as_ref().map(#ident_arr_not_null_as_pg_json_test_cases_ts::#UpdToReadIdsSc)
                        })
                    },
                };
                quote!{#ident_read_ids_ucc(#ts)}
            };
            let read_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts = {
                let ts = gen_v_init_ts0(&match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let params_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    #ft_as_pg_json_test_cases_ts::read_ids_to_opt_v_read_dflt_opt_some_vec_one_el(
                                        &#VSc.0.#VSc.#fi
                                    )
                                }
                            });
                            quote! {
                                #ident_read_ucc::try_new(
                                    #(#params_ts),*
                                ).expect("57820868")
                            }
                        }
                        IsNullable::True => quote! {
                            #ident_read_ucc::new(
                                #VSc.0.#VSc.as_ref().and_then(|v_dfa7815e| match #ident_stdrt_not_null_as_pg_json_test_cases_ts::read_ids_to_opt_v_read_dflt_opt_some_vec_one_el(
                                    v_dfa7815e
                                ) {
                                    Some(v_02cef266) => Some(v_02cef266.#VSc),
                                    None => None,
                                })
                            )
                        }
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => {
                            let params_ts = vec_syn_field_with_id.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    #ft_as_pg_json_test_cases_ts::read_ids_to_opt_v_read_dflt_opt_some_vec_one_el(
                                        &el_629b1544.0.#VSc.#fi
                                    )
                                }
                            });
                            quote! {
                                #ident_read_ucc::new({
                                    let mut acc_5f587d35 = #VSc.0.#VSc.clone().into_iter().map(|el_629b1544|{
                                        #ident_with_id_stdrt_not_null_read_ucc::try_new(
                                            #(#params_ts),*
                                        ).expect("8f6fb6b6")
                                    }).collect::<Vec<#ident_with_id_stdrt_not_null_read_ucc>>();
                                    acc_5f587d35.sort_by(|first, second| {
                                        if let (Some(v_first), Some(v_second)) = (&first.id, &second.id) {
                                            //mb remove .clone - add .get by ref method
                                            #import_pg_json_uuid_uuid_as_not_null_jsonb_string_as_pg_json_ts::into_inn(
                                                v_first.#VSc.clone()
                                            )
                                            .cmp(&#import_pg_json_uuid_uuid_as_not_null_jsonb_string_as_pg_json_ts::into_inn(
                                                v_second.#VSc.clone()
                                            ))
                                        }
                                        else {
                                            panic!("0bdf0f44");
                                        }
                                    });
                                    acc_5f587d35
                                })
                            }
                        }
                        IsNullable::True => quote! {
                            #ident_read_ucc::new(
                                #VSc.0.#VSc.as_ref().and_then(|v_16ab4136| match #ident_arr_not_null_as_pg_json_test_cases_ts::read_ids_to_opt_v_read_dflt_opt_some_vec_one_el(
                                    v_16ab4136
                                ) {
                                    Some(v_71a66429) => Some(v_71a66429.#VSc.0),
                                    None => None,
                                })
                            )
                        }
                    },
                });
                quote!{Some(#ts)}
            };
            let previous_read_and_opt_upd_into_read_ts = {
                let fields_init_ts = vec_syn_field.iter().map(|el0| {
                    let fi = &el0.ident;
                    quote! {let mut #fi = None;}
                });
                let match_ts = vec_syn_field.iter().map(|el0| {
                    let fi = &el0.ident;
                    let fi_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                    quote! {
                        #ident_stdrt_not_null_upd_el_ucc::#fi_ucc_ts(v_730a4dde) => {
                            #fi = Some(v_730a4dde.#VSc);
                        }
                    }
                });
                let gen_struct_init_ts = |fn0: &dyn Fn(&dyn ToTokens) -> Ts2|{
                    let ts = vec_syn_field.iter().map(|el0| {
                        let fi = &el0.ident;
                        let ts0 = gen_v_init_ts0(&{
                            let ts = fn0(&fi);
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                            quote!{
                                #ft_as_pg_json_test_cases_ts::previous_read_and_opt_upd_into_read(
                                    #ts,
                                    #fi
                                )
                            }
                        });
                        quote! {#fi: Some(#ts0)}
                    });
                    quote!{#(#ts),*}
                };
                let gen_opt_ts = |pattern_53bdff8d: &Pattern|{
                    let ident_as_pg_json_test_cases_ts_53bdff8d = gen_type_as_pg_json_test_cases_ts(match &pattern_53bdff8d {
                        Pattern::Stdrt => &ident_stdrt_not_null_ucc,
                        Pattern::Arr => &ident_arr_not_null_ucc
                    });
                    quote! {
                        match #OptUpdSc {
                            Some(v_fca601b5) => #ident_read_ucc(
                                match v_fca601b5.0 {
                                    Some(v_8d7747f1) => Some(
                                        #ident_as_pg_json_test_cases_ts_53bdff8d::previous_read_and_opt_upd_into_read(
                                            #ReadSc.0.unwrap_or_else(#dflt_but_opt_is_some_ts),
                                            Some(v_8d7747f1),
                                        )
                                    ),
                                    None => None,
                                }
                            ),
                            None => #ReadSc,
                        }
                    }
                };
                match &is_nullable {
                    IsNullable::False => match &pattern {
                        Pattern::Stdrt => {
                            let struct_init_ts = gen_struct_init_ts(&|ts: &dyn ToTokens|{
                                quote!{
                                    #ReadSc.#ts.expect("a2d26e36").#VSc
                                }
                            });
                            quote!{
                                match #OptUpdSc {
                                    Some(v_e5377436) => {
                                        #(#fields_init_ts)*
                                        for el_629b1544 in v_e5377436.0.into_vec() {
                                            match el_629b1544 {
                                                #(#match_ts),*
                                            }
                                        }
                                        #ident_read_ucc {
                                            #struct_init_ts
                                        }
                                    },
                                    None => #ReadSc
                                }
                            }
                        },
                        Pattern::Arr => {
                            let struct_init_ts = gen_struct_init_ts(&|ts: &dyn ToTokens|{
                                quote!{
                                    found_read_el.#ts.expect("2e8229f7").#VSc
                                }
                            });
                            quote! {
                                match #OptUpdSc {
                                    Some(v_47f5a20b) => #ident_read_ucc({
                                        let mut acc_04a67ef2 = Vec::new();
                                        for el_377d1bb4 in v_47f5a20b.#UpdSc.into_vec() {
                                            let mut opt_read_el = None;
                                            for el in &#ReadSc.0 {
                                                if *#uuid_uuid_as_not_null_jsonb_string_as_pg_json_obj_vec_el_id_ts::get_inn(&el_377d1bb4.#IdSc.clone().into())
                                                ==
                                                #uuid_uuid_as_not_null_jsonb_string_as_import_pg_json_ts::into_inn(
                                                    el.#IdSc.clone().expect("df2413fe").#VSc
                                                )
                                                {
                                                    opt_read_el = Some(el.clone());
                                                    break;
                                                }
                                            }
                                            let found_read_el = opt_read_el.expect("139882b9");
                                            #(#fields_init_ts)*
                                            for el_629b1544 in el_377d1bb4.fields.0.into_vec() {
                                                match el_629b1544 {
                                                    #(#match_ts),*
                                                }
                                            }
                                            acc_04a67ef2.push(#ident_with_id_stdrt_not_null_read_ucc {
                                                #IdSc: found_read_el.#IdSc,
                                                #struct_init_ts
                                            });
                                        }
                                        acc_04a67ef2
                                    }),
                                    None => #ReadSc
                                }
                            }
                        },
                    },
                    IsNullable::True => gen_opt_ts(pattern)
                }
            };
            let read_ids_and_cr_into_read_ts = {
                let gen_nullable_ts = |ident_ts_3eb17505: &dyn ToTokens, ts: &dyn ToTokens|{
                    let ident_as_pg_json_test_cases_ts_3eb17505 = gen_type_as_pg_json_test_cases_ts(&ident_ts_3eb17505);
                    quote! {
                        #ident_read_ucc::new(
                            match (#ReadIdsSc.0.#VSc, #CrSc.0) {
                                (Some(read_ids_2b2ab8a1), Some(cr_4a1adaa3)) => {
                                    Some(
                                        #ident_as_pg_json_test_cases_ts_3eb17505::#ReadIdsAndCrIntoOptVReadSc(
                                            read_ids_2b2ab8a1,
                                            cr_4a1adaa3
                                        ).expect("56ac4450").#VSc #ts
                                    )
                                },
                                (Some(_), None) => panic!("75be9ae0"),
                                (None, Some(_)) => panic!("6a95d7ae"),
                                (None, None) => None,
                            }
                        )
                    }
                };
                match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let params_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoOptVReadSc(
                                        #ReadIdsSc.0.#VSc.#fi,
                                        #CrSc.#fi
                                    )
                                }
                            });
                            quote! {
                                #ident_read_ucc::try_new(
                                    #(#params_ts),*
                                ).expect("52ad3994")
                            }
                        }
                        IsNullable::True => gen_nullable_ts(
                            &ident_stdrt_not_null_ucc,
                            &Ts2::new()
                        )
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => {
                            let gen_param_ts = |ft: &dyn ToTokens, fi: &dyn ToTokens, ts: &dyn ToTokens|{
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                                quote! {
                                    #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoOptVReadSc(
                                        read_ids_225e2b76.0.#VSc.#fi,
                                        #ts,
                                    )
                                }
                            };
                            let id_param_ts = gen_param_ts(
                                &uuid_uuid_as_not_null_jsonb_string_ts,
                                &IdSc,
                                &dflt_but_opt_is_some_call_ts
                            );
                            let params_ts = vec_syn_field.iter().map(|el0|{
                                let fi = &el0.ident;
                                gen_param_ts(
                                    &el0.type0,
                                    &fi,
                                    &quote! {cr_3c660445.#fi}
                                )
                            });
                            quote! {
                                #ident_read_ucc::new({
                                    assert_eq!(#ReadIdsSc.0.#VSc.len(), #CrSc.0.len(), "90d33ddd");
                                    let mut acc_37909420 = Vec::new();
                                    for (read_ids_225e2b76, cr_3c660445) in #ReadIdsSc.0.#VSc.into_iter().zip(#CrSc.0) {
                                        acc_37909420.push(#ident_with_id_stdrt_not_null_read_ucc::try_new(
                                            #id_param_ts,
                                            #(#params_ts),*
                                        ).expect("1330ac8d"));
                                    }
                                    acc_37909420
                                })
                            }
                        }
                        IsNullable::True => gen_nullable_ts(
                            &ident_arr_not_null_ucc,
                            &quote!{.0}
                        )
                    },
                }
            };
            let read_ids_and_cr_into_opt_v_read_ts = {
                let ts = gen_v_init_ts0(&quote!{
                    <#SelfUcc as #import::PgJsonTestCases>::#ReadIdsAndCrIntoReadSc(
                        #ReadIdsSc,
                        #CrSc
                    )
                });
                quote!{Some(#ts)}
            };
            let read_ids_and_cr_into_table_type_ts = {
                let gen_nullable_ts = |ident_ts_971139d7: &dyn ToTokens, ts: &dyn ToTokens|{
                    let ident_as_pg_json_test_cases_ts_971139d7 = gen_type_as_pg_json_test_cases_ts(&ident_ts_971139d7);
                    quote! {
                        #ident_table_type_ucc::new(
                            match (#ReadIdsSc.0.#VSc, #CrSc.0) {
                                (Some(read_ids_fb2ec2e4), Some(cr_2f615d4f)) => {
                                    Some(
                                        #ident_as_pg_json_test_cases_ts_971139d7::#ReadIdsAndCrIntoTableTypeSc(
                                            read_ids_fb2ec2e4,
                                            cr_2f615d4f
                                        ) #ts
                                    )
                                },
                                (Some(_), None) => panic!("9349dcd5"),
                                (None, Some(_)) => panic!("45f8e70a"),
                                (None, None) => None,
                            }
                        )
                    }
                };
                match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let params_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoTableTypeSc(
                                        #ReadIdsSc.0.#VSc.#fi,
                                        #CrSc.#fi
                                    )
                                }
                            });
                            quote! {
                                #ident_table_type_ucc::new(
                                    #(#params_ts),*
                                )
                            }
                        }
                        IsNullable::True => gen_nullable_ts(
                            &ident_stdrt_not_null_ucc,
                            &Ts2::new()
                        )
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => {
                            let gen_param_ts = |ft: &dyn ToTokens, fi: &dyn ToTokens, ts: &dyn ToTokens|{
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                                quote! {
                                    #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoTableTypeSc(
                                        read_ids_94b49496.0.#VSc.#fi,
                                        #ts,
                                    )
                                }
                            };
                            let id_param_ts = gen_param_ts(
                                &uuid_uuid_as_not_null_jsonb_string_ts,
                                &IdSc,
                                &dflt_but_opt_is_some_call_ts
                            );
                            let params_ts = vec_syn_field.iter().map(|el0|{
                                let fi = &el0.ident;
                                gen_param_ts(
                                    &el0.type0,
                                    &fi,
                                    &quote! {cr_24629087.#fi}
                                )
                            });
                            quote! {
                                #ident_table_type_ucc::new({
                                    assert_eq!(#ReadIdsSc.0.#VSc.len(), #CrSc.0.len(), "7776a146");
                                    let mut acc_319e1fb1 = Vec::new();
                                    for (read_ids_94b49496, cr_24629087) in #ReadIdsSc.0.#VSc.into_iter().zip(#CrSc.0) {
                                        acc_319e1fb1.push(#ident_with_id_stdrt_not_null_table_type_ucc::new(
                                            #id_param_ts,
                                            #(#params_ts),*
                                        ));
                                    }
                                    acc_319e1fb1
                                })
                            }
                        }
                        IsNullable::True => gen_nullable_ts(
                            &ident_arr_not_null_ucc,
                            &quote!{.0}
                        )
                    },
                }
            };
            let read_ids_and_cr_into_where_equal_ts = match &is_nullable {
                IsNullable::False => match &pattern {
                    Pattern::Stdrt => {
                        let params_ts = vec_syn_field.iter().map(|el0| {
                            let fi = &el0.ident;
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                            quote! {
                                #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoTableTypeSc(
                                    #ReadIdsSc.0.#VSc.#fi,
                                    #CrSc.#fi
                                )
                            }
                        });
                        quote! {
                            #ident_where_ucc::#EqualUcc(
                                #import::PgJsonWhereEqual {
                                    oprtr: #import::Oprtr::Or,
                                    #VSc: #ident_table_type_ucc::new(
                                        #(#params_ts),*
                                    )
                                }
                            )
                        }
                    },
                    Pattern::Arr => {
                        let gen_read_ids_and_cr_into_table_type_ts = |
                            fi: &dyn ToTokens,
                            ft: &dyn ToTokens,
                            ts: &dyn ToTokens
                        |{
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                            quote!{
                                #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoTableTypeSc(
                                    read_ids_ea32954c.0.#VSc.#fi,
                                    #ts
                                )
                            }
                        };
                        let ident_ts_978daa48 = gen_read_ids_and_cr_into_table_type_ts(
                            &IdSc,
                            &uuid_uuid_as_not_null_jsonb_string_ts,
                            &dflt_but_opt_is_some_call_ts
                        );
                        let params_ts = vec_syn_field.iter().map(|el0| {
                            let fi = &el0.ident;
                            gen_read_ids_and_cr_into_table_type_ts(
                                &fi,
                                &el0.type0,
                                &quote!{cr_3cbe8967.#fi}
                            )
                        });
                        quote! {
                            #ident_where_ucc::#EqualUcc(
                                #import::PgJsonWhereEqual {
                                    oprtr: #import::Oprtr::And,
                                    #VSc: #ident_table_type_ucc::new({
                                        let mut acc_321b3fcd = Vec::new();
                                        for (read_ids_ea32954c, cr_3cbe8967) in #ReadIdsSc.0.#VSc.into_iter().zip(#CrSc.0) {
                                            acc_321b3fcd.push(
                                                #ident_with_id_stdrt_not_null_table_type_ucc::new(
                                                    #ident_ts_978daa48,
                                                    #(#params_ts),*
                                                )
                                            );
                                        }
                                        acc_321b3fcd
                                    })
                                }
                            )
                        }
                    }
                },
                IsNullable::True => {
                    let ts = {
                        let ident_ts_bdc5fdf7 = gen_type_as_pg_json_test_cases_ts(&gen_ident_ucc(&match &pattern {
                            Pattern::Stdrt => IdentPattern::StdrtNotNullWithoutId,
                            Pattern::Arr => IdentPattern::ArrNotNullWithId,
                        }));
                        quote!{
                            vec![
                                #ident_ts_bdc5fdf7::#ReadIdsAndCrIntoWhereEqualSc(
                                    read_ids_ce30c0fe,
                                    cr_8fd81ed8
                                )
                            ]
                        }
                    };
                    quote! {
                        #import::NullableJsonObjectPgTypeWhereFilter(
                            match (#ReadIdsSc.0.#VSc, #CrSc.0) {
                                (Some(read_ids_ce30c0fe), Some(cr_8fd81ed8)) => match #import::NotEmptyUniqueVec::try_new(#ts) {
                                    Ok(v_7a9cd49b) => Some(v_7a9cd49b),
                                    Err(er) => match er {
                                        #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("463769fc")
                                    }
                                },
                                (Some(_), None) => panic!("1a2b314c"),
                                (None, Some(_)) => panic!("9faea0f9"),
                                (None, None) => None,
                            }
                        )
                    }
                },
            };
            let read_ids_and_cr_into_vec_where_equal_using_fields_ts = {
                let ts = match &is_nullable {
                    IsNullable::False => match &pattern {
                        Pattern::Stdrt => {
                            let els_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let fi_ucc_ts = ToTokensToUccTs::case_or_panic(&fi);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    #ident_where_ucc::#fi_ucc_ts(
                                        #import::PgTypeWhere::new(
                                            #import::Oprtr::And,
                                            #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoVecWhereEqualUsingFieldsSc(
                                                #ReadIdsSc.0.#VSc.#fi,
                                                #CrSc.#fi
                                            )
                                        )
                                    )
                                }
                            });
                            quote! {#(#els_ts),*}
                        },
                        Pattern::Arr => {
                            let gen_read_ids_and_cr_into_table_type_ts = |
                                fi: &dyn ToTokens,
                                ft: &dyn ToTokens,
                                ts: &dyn ToTokens
                            |{
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                                quote!{
                                    #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoTableTypeSc(
                                        read_ids_319c9e78.0.#VSc.#fi,
                                        #ts
                                    )
                                }
                            };
                            let ident_ts_76fd9d28 = gen_read_ids_and_cr_into_table_type_ts(
                                &IdSc,
                                &uuid_uuid_as_not_null_jsonb_string_ts,
                                &dflt_but_opt_is_some_call_ts
                            );
                            let params_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                gen_read_ids_and_cr_into_table_type_ts(
                                    &fi,
                                    &el0.type0,
                                    &quote!{cr_00ae06d2.#fi}
                                )
                            });
                            quote! {
                                #ident_where_ucc::#EqualUcc(
                                    #import::PgJsonWhereEqual {
                                        oprtr: #import::Oprtr::And,
                                        #VSc: #ident_table_type_ucc::new({
                                            let mut acc_97ebf7d6 = Vec::new();
                                            for (read_ids_319c9e78, cr_00ae06d2) in #ReadIdsSc.0.#VSc.into_iter().zip(#CrSc.0) {
                                                acc_97ebf7d6.push(
                                                    #ident_with_id_stdrt_not_null_table_type_ucc::new(
                                                        #ident_ts_76fd9d28,
                                                        #(#params_ts),*
                                                    )
                                                );
                                            }
                                            acc_97ebf7d6
                                        })
                                    }
                                )
                            }
                        }
                    },
                    IsNullable::True => {
                        let ts = {
                            let ident_ts_b9e61412 = gen_type_as_pg_json_test_cases_ts(&gen_ident_ucc(&match &pattern {
                                Pattern::Stdrt => IdentPattern::StdrtNotNullWithoutId,
                                Pattern::Arr => IdentPattern::ArrNotNullWithId,
                            }));
                            quote! {
                                #ident_ts_b9e61412::#ReadIdsAndCrIntoVecWhereEqualUsingFieldsSc(
                                    read_ids_2898c440,
                                    cr_f1c4667c
                                )
                            }
                        };
                        quote! {
                            #import::NullableJsonObjectPgTypeWhereFilter(
                                match (#ReadIdsSc.0.#VSc, #CrSc.0) {
                                    (Some(read_ids_2898c440), Some(cr_f1c4667c)) => Some(#ts),
                                    (Some(_), None) => panic!("49e4c289"),
                                    (None, Some(_)) => panic!("ad71caa2"),
                                    (None, None) => None,
                                }
                            )
                        }
                    },
                };
                quote!{
                    #import::NotEmptyUniqueVec::try_new(vec![
                        #ts
                    ]).expect("ba9c52c1")
                }
            };
            let read_ids_and_cr_into_vec_where_equal_to_json_field_ts = match &pattern {
                Pattern::Stdrt => match &is_nullable {
                    IsNullable::False => {
                        let ts = vec_syn_field.iter().map(|el0| {
                            let fi = &el0.ident;
                            let fi_ucc = &ToTokensToUccTs::case_or_panic(&fi);
                            let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                            quote! {
                                for el_d830c061 in #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoVecWhereEqualToJsonFieldSc(
                                    #ReadIdsSc.0.#VSc.#fi,
                                    #CrSc.#fi
                                ).into_vec() {
                                    acc_89ec072c.push(
                                        #ident_where_ucc::#fi_ucc(
                                            #import::PgTypeWhere::try_new(
                                                #import::Oprtr::Or,
                                                vec![el_d830c061],
                                            )
                                            .expect("0c6ccad1"),
                                        )
                                    );
                                }
                            }
                        });
                        quote!{
                            #import::NotEmptyUniqueVec::try_new({
                                let mut acc_89ec072c = Vec::new();
                                #(#ts)*
                                acc_89ec072c
                            }).expect("9c50391c")
                        }
                    },
                    IsNullable::True => quote!{
                        #import::NotEmptyUniqueVec::try_new({
                            let mut acc_12b6f16d = Vec::new();
                            match (#ReadIdsSc.0.#VSc, #CrSc.0) {
                                (Some(read_ids_2f024927), Some(cr_120c1dad)) => {
                                    for el_a8b181a0 in #ident_stdrt_not_null_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoVecWhereEqualToJsonFieldSc(
                                        read_ids_2f024927,
                                        cr_120c1dad
                                    ).into_vec() {
                                        match #import::NotEmptyUniqueVec::try_new(vec![el_a8b181a0]) {
                                            Ok(v_8e72cfd7) => {
                                                acc_12b6f16d.push(#import::NullableJsonObjectPgTypeWhereFilter(Some(v_8e72cfd7)));
                                            },
                                            Err(er) => match er {
                                                #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("2a88b17f")
                                            }
                                        }
                                    }
                                },
                                (Some(_), None) => panic!("b4507b4c"),
                                (None, Some(_)) => panic!("8f458c1d"),
                                (None, None) => {
                                    acc_12b6f16d.push(#import::NullableJsonObjectPgTypeWhereFilter(None));
                                },
                            }
                            acc_12b6f16d
                        }).expect("7efc9aae")
                    }
                },
                Pattern::Arr => quote!{
                    #self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoVecWhereEqualUsingFieldsSc(
                        #ReadIdsSc,
                        #CrSc
                    )
                }
            };
            let (
                read_ids_and_cr_into_pg_json_opt_vec_where_dim_one_equal_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_dim_two_equal_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_dim_three_equal_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_dim_four_equal_ts,
            ) = {
                let gen_ts = |dim: &Dim|{
                    let read_ids_and_cr_into_pg_json_opt_vec_where_dim_nbr_equal_sc = dim.read_ids_and_cr_into_pg_json_opt_vec_where_dim_nbr_equal_sc();
                    let gen_nullable_ts = |ts: &dyn ToTokens|quote! {
                        match #import::NotEmptyUniqueVec::try_new(
                            match (#ReadIdsSc.0.#VSc, #CrSc.0) {
                                (Some(read_ids_cdcb6239), Some(cr_fdd53941)) => match <
                                    #ts
                                    as
                                    #import::PgJsonTestCases
                                >::#read_ids_and_cr_into_pg_json_opt_vec_where_dim_nbr_equal_sc(
                                    read_ids_cdcb6239,
                                    cr_fdd53941
                                ) {
                                    Some(v_d6124e21) => {
                                        let mut acc_bd78dc08 = Vec::new();
                                        for el in v_d6124e21.clone().into_vec() {
                                            match #import::NotEmptyUniqueVec::try_new(
                                                vec![el]
                                            ) {
                                                Ok(v_7ed84f3b) => {
                                                    acc_bd78dc08.push(
                                                        #import::NullableJsonObjectPgTypeWhereFilter(Some(v_7ed84f3b))
                                                    );
                                                },
                                                Err(er) => match er {
                                                    #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                    #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("23dca12f")
                                                }
                                            }
                                        }
                                        let v_e48110ec = #import::NullableJsonObjectPgTypeWhereFilter(Some(v_d6124e21));
                                        if !acc_bd78dc08.contains(&v_e48110ec) {
                                            acc_bd78dc08.push(v_e48110ec);
                                        }
                                        acc_bd78dc08
                                    },
                                    None => {
                                        return None;
                                    }
                                },
                                (Some(_), None) => panic!("6abeac7b"),
                                (None, Some(_)) => panic!("a2761cd2"),
                                (None, None) => vec![#import::NullableJsonObjectPgTypeWhereFilter(None)]
                            }
                        ) {
                            Ok(v_55f2dc3d) => Some(v_55f2dc3d),
                            Err(er) => match er {
                                #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("88912e24")
                            }
                        }
                    };
                    match &pattern {
                        Pattern::Stdrt => match &is_nullable {
                            IsNullable::False => {
                                let ts = vec_syn_field.iter().map(|el0| {
                                    let fi = &el0.ident;
                                    let fi_ucc = &ToTokensToUccTs::case_or_panic(&fi);
                                    let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                    quote! {
                                        if let Some(v_2bbd2c96) = #ft_as_pg_json_test_cases_ts::#read_ids_and_cr_into_pg_json_opt_vec_where_dim_nbr_equal_sc(
                                            #ReadIdsSc.0.#VSc.#fi,
                                            #CrSc.#fi
                                        ) {
                                            for el in v_2bbd2c96.clone().into_vec() {
                                                acc_2fe1cca8.push(
                                                    #ident_where_ucc::#fi_ucc(
                                                        #import::PgTypeWhere::try_new(
                                                            #import::Oprtr::And,
                                                            vec![el]
                                                        ).expect("9a25e058")
                                                    )
                                                );
                                            }
                                            let v_c45bab0d = #ident_where_ucc::#fi_ucc(
                                                #import::PgTypeWhere::new(
                                                    #import::Oprtr::And,
                                                    v_2bbd2c96
                                                )
                                            );
                                            if !acc_2fe1cca8.contains(&v_c45bab0d) {
                                                acc_2fe1cca8.push(v_c45bab0d);
                                            }
                                        }
                                    }
                                });
                                quote! {
                                    match #import::NotEmptyUniqueVec::try_new({
                                        let mut acc_2fe1cca8 = Vec::new();
                                        #(#ts)*
                                        acc_2fe1cca8
                                    }) {
                                        Ok(v_a5fa471d) => Some(v_a5fa471d),
                                        Err(er) => match er {
                                            #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                            #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("89e719cf")
                                        }
                                    }
                                }
                            }
                            IsNullable::True => gen_nullable_ts(&ident_stdrt_not_null_ucc)
                        },
                        Pattern::Arr => match &is_nullable {
                            IsNullable::False => {
                                let ts_f0710cd9 = {
                                    let ts_57d244f8 = vec_syn_field.iter().map(|el0| {
                                        let fi = &el0.ident;
                                        let el_fi_ucc = ElSelfUcc::from_tokens(&fi);
                                        let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                        quote! {
                                            if let Some(v_bf84026e) = #ft_as_pg_json_test_cases_ts::#read_ids_and_cr_into_pg_json_opt_vec_where_dim_nbr_equal_sc(
                                                read_ids_420d38ca.0.#VSc.#fi.clone(),
                                                cr_76f032c1.#fi.clone()
                                            ) {
                                                for el in v_bf84026e.clone().into_vec() {
                                                    let v_592e6b5f = #ident_where_ucc::#el_fi_ucc(
                                                        #import::PgTypeWhere::try_new(
                                                            #import::Oprtr::And,
                                                            vec![el]
                                                        ).expect("1f7ae335")
                                                    );
                                                    if !acc_dd377eb1.contains(&v_592e6b5f) {
                                                        acc_dd377eb1.push(v_592e6b5f);
                                                    }
                                                }
                                                let v_03205172 = #ident_where_ucc::#el_fi_ucc(
                                                    #import::PgTypeWhere::new(
                                                        #import::Oprtr::And,
                                                        v_bf84026e
                                                    )
                                                );
                                                if !acc_dd377eb1.contains(&v_03205172) {
                                                    acc_dd377eb1.push(v_03205172);
                                                }
                                            }
                                        }
                                    });
                                    quote!{#(#ts_57d244f8)*}
                                };
                                let ts_2cc4a40e = match &dim {
                                    Dim::One => {
                                        let dim_one_ts = {
                                            let ts_91a09fe2 = vec_syn_field.iter().map(|el0| {
                                                let fi = &el0.ident;
                                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                                quote! {
                                                    #ft_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoTableTypeSc(
                                                        read_ids_420d38ca.0.#VSc.#fi,
                                                        cr_76f032c1.#fi
                                                    )
                                                }
                                            });
                                            quote!{
                                                acc_dd377eb1.push(#ident_where_ucc::DimOneEqual(#import::PgJsonWhereDimOneEqual {
                                                    oprtr: #import::Oprtr::And,
                                                    dims: #import::BoundedVec::try_from(
                                                        vec![
                                                            #import::UnsignedPartOfI32::try_from(
                                                                i32::try_from(i_47620dcf).expect("5341936f")
                                                            ).expect("76906f3c")
                                                        ]
                                                    ).expect("8a624c70"),
                                                    #VSc: #ident_with_id_stdrt_not_null_table_type_ucc::new(
                                                        <#uuid_uuid_as_not_null_jsonb_string_ts as #import::PgJsonTestCases>::#ReadIdsAndCrIntoTableTypeSc(
                                                            read_ids_420d38ca.0.#VSc.#IdSc,
                                                            #PgCrudDfltOptSomeVecOneElCall
                                                        ),
                                                        #(#ts_91a09fe2),*
                                                    ),
                                                }));
                                            }
                                        };
                                        quote!{
                                            for (i_47620dcf, (read_ids_420d38ca, cr_76f032c1)) in #ReadIdsSc.0.#VSc.into_iter()
                                                .zip(#CrSc.0)
                                                .enumerate()
                                            {
                                                #ts_f0710cd9
                                                #dim_one_ts
                                            }
                                        }
                                    },
                                    Dim::Two |
                                    Dim::Three |
                                    Dim::Four => quote!{
                                        for (read_ids_420d38ca, cr_76f032c1) in #ReadIdsSc.0.#VSc.into_iter()
                                            .zip(#CrSc.0)
                                        {
                                            #ts_f0710cd9
                                        }
                                    },
                                };
                                quote! {
                                    match #import::NotEmptyUniqueVec::try_new({
                                        let mut acc_dd377eb1 = Vec::new();
                                        #ts_2cc4a40e
                                        acc_dd377eb1
                                    }) {
                                        Ok(v_dfac36e4) => Some(v_dfac36e4),
                                        Err(er) => match er {
                                            #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                            #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("93390f1a")
                                        },
                                    }
                                }
                            }
                            IsNullable::True => gen_nullable_ts(&ident_arr_not_null_ucc)
                        },
                    }
                };
                (
                    gen_ts(&Dim::One),
                    gen_ts(&Dim::Two),
                    gen_ts(&Dim::Three),
                    gen_ts(&Dim::Four)
                )
            };
            let cr_into_pg_json_opt_vec_where_length_equal_ts = {
                let gen_nullable_ts = |ts: &dyn ToTokens|quote! {
                    match #import::NotEmptyUniqueVec::try_new(
                        match #CrSc.0 {
                            Some(cr_09a81dae) => match <
                                #ts
                                as
                                #import::PgJsonTestCases
                            >::#CrIntoPgJsonOptVecWhereLengthEqualSc(cr_09a81dae) {
                                Some(v_3680a4c9) => {
                                    let mut acc_5c441d3a = Vec::new();
                                    for el_a8b181a0 in v_3680a4c9.clone().into_vec() {
                                        match #import::NotEmptyUniqueVec::try_new(vec![el_a8b181a0]) {
                                            Ok(v_15097b27) => {
                                                acc_5c441d3a.push(
                                                    #import::NullableJsonObjectPgTypeWhereFilter(
                                                        Some(v_15097b27)
                                                    )
                                                );
                                            },
                                            Err(er) => match er {
                                                #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("6c4da72e")
                                            }
                                        }
                                    }
                                    let v_84ea8e4c = #import::NullableJsonObjectPgTypeWhereFilter(Some(v_3680a4c9));
                                    if !acc_5c441d3a.contains(&v_84ea8e4c) {
                                        acc_5c441d3a.push(v_84ea8e4c);
                                    }
                                    acc_5c441d3a
                                },
                                None => {
                                    return None;
                                }
                            },
                            None => vec![#import::NullableJsonObjectPgTypeWhereFilter(None)],
                        }
                    ) {
                        Ok(v_72dbefbc) => Some(v_72dbefbc),
                        Err(er) => match er {
                            #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                            #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("d41bcbca")
                        }
                    }
                };
                match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let fi_ucc = &ToTokensToUccTs::case_or_panic(&fi);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    if let Some(v_927601a4) = #ft_as_pg_json_test_cases_ts::#CrIntoPgJsonOptVecWhereLengthEqualSc(
                                        #CrSc.#fi
                                    ) {
                                        for el_194a660a in v_927601a4.clone().into_vec() {
                                            acc_587bf907.push(
                                                #ident_where_ucc::#fi_ucc(
                                                    #import::PgTypeWhere::try_new(
                                                        #import::Oprtr::And,
                                                        vec![el_194a660a]
                                                    ).expect("2f437949")
                                                )
                                            );
                                        }
                                        let v_84ea8e4c = #ident_where_ucc::#fi_ucc(
                                            #import::PgTypeWhere::new(
                                                #import::Oprtr::And,
                                                v_927601a4
                                            )
                                        );
                                        if !acc_587bf907.contains(&v_84ea8e4c) {
                                            acc_587bf907.push(v_84ea8e4c);
                                        }
                                    }
                                }
                            });
                            quote! {
                                match #import::NotEmptyUniqueVec::try_new({
                                    let mut acc_587bf907 = Vec::new();
                                    #(#ts)*
                                    acc_587bf907
                                }) {
                                    Ok(v_ea661a62) => Some(v_ea661a62),
                                    Err(er) => match er {
                                        #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("7786dfd4")
                                    },
                                }
                            }
                        }
                        IsNullable::True => gen_nullable_ts(&ident_stdrt_not_null_ucc)
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => {
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let el_fi_ucc = ElSelfUcc::from_tokens(&fi);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    for cr_e06a9fe2 in #CrSc.0.clone() {
                                        if let Some(v_ee015fcc) = #ft_as_pg_json_test_cases_ts::#CrIntoPgJsonOptVecWhereLengthEqualSc(
                                            cr_e06a9fe2.#fi
                                        ) {
                                            for el in v_ee015fcc.clone().into_vec() {
                                                let v_0ae29f5f = #ident_where_ucc::#el_fi_ucc(
                                                    #import::PgTypeWhere::try_new(
                                                        #import::Oprtr::And,
                                                        vec![el]
                                                    )
                                                    .expect("38ca88dc"),
                                                );
                                                if !acc_480d72e5.contains(&v_0ae29f5f) {
                                                    acc_480d72e5.push(v_0ae29f5f);
                                                }
                                            }
                                            let v_4e4cfda3 = #ident_where_ucc::#el_fi_ucc(
                                                #import::PgTypeWhere::new(
                                                    #import::Oprtr::And,
                                                    v_ee015fcc
                                                )
                                            );
                                            if !acc_480d72e5.contains(&v_4e4cfda3) {
                                                acc_480d72e5.push(v_4e4cfda3);
                                            }
                                        }
                                    }
                                }
                            });
                            quote! {
                                match #import::NotEmptyUniqueVec::try_new({
                                    let mut acc_480d72e5 = Vec::new();
                                    #(#ts)*
                                    acc_480d72e5.push(#ident_where_ucc::LengthEqual(
                                        #import::PgJsonWhereLengthEqual {
                                            oprtr: #import::Oprtr::And,
                                            #VSc: #import::UnsignedPartOfI32::try_from(
                                                i32::try_from(#CrSc.0.len()).expect("1811faf7")
                                            ).expect("a590f39b"),
                                        }
                                    ));
                                    acc_480d72e5
                                }) {
                                    Ok(v_cc01db9a) => Some(v_cc01db9a),
                                    Err(er) => match er {
                                        #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("bad01dd0")
                                    },
                                }
                            }
                        }
                        IsNullable::True => gen_nullable_ts(&ident_arr_not_null_ucc)
                    },
                }
            };
            let cr_into_pg_json_opt_vec_where_length_greater_than_ts = {
                let gen_nullable_ts = |ts: &dyn ToTokens|quote! {
                    #CrSc.0.map_or_else(|| None, |cr_612f2a61| <
                        #ts
                        as
                        #import::PgJsonTestCases
                    >::cr_into_pg_json_opt_vec_where_length_greater_than(cr_612f2a61).map_or_else(
                        || None,
                        |v_1ea95b5d| match #import::NotEmptyUniqueVec::try_new({
                            let mut acc_87f84b5c = Vec::new();
                            for el_9bbf8527 in v_1ea95b5d.clone().into_vec() {
                                match #import::NotEmptyUniqueVec::try_new(vec![el_9bbf8527]) {
                                    Ok(v_1d0202fc) => {
                                        acc_87f84b5c.push(#import::NullableJsonObjectPgTypeWhereFilter(Some(v_1d0202fc)));
                                    }
                                    Err(er) => match er {
                                        #import::NotEmptyUniqueVecTryNewEr::IsEmpty { .. } => (),
                                        #import::NotEmptyUniqueVecTryNewEr::NotUnique { .. } => panic!("bdb0a112"),
                                    },
                                }
                            }
                            let v_4e4cfda3 = #import::NullableJsonObjectPgTypeWhereFilter(Some(v_1ea95b5d));
                            if !acc_87f84b5c.contains(&v_4e4cfda3) {
                                acc_87f84b5c.push(v_4e4cfda3);
                            }
                            acc_87f84b5c
                        }) {
                            Ok(v_ea4ca151) => Some(v_ea4ca151),
                            Err(er) => match er {
                                #import::NotEmptyUniqueVecTryNewEr::IsEmpty { .. } => None,
                                #import::NotEmptyUniqueVecTryNewEr::NotUnique { .. } => panic!("c7ecc36f"),
                            },
                        },
                    ))
                };
                match &pattern {
                    Pattern::Stdrt => match &is_nullable {
                        IsNullable::False => {
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let fi_ucc = &ToTokensToUccTs::case_or_panic(&fi);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    if let Some(v_3432b965) = #ft_as_pg_json_test_cases_ts::#CrIntoPgJsonOptVecWhereLengthGreaterThanSc(
                                        #CrSc.#fi
                                    ) {
                                        for el_9bbf8527 in v_3432b965.clone().into_vec() {
                                            acc_f5866fb6.push(
                                                #ident_where_ucc::#fi_ucc(
                                                    #import::PgTypeWhere::try_new(
                                                        #import::Oprtr::And,
                                                        vec![el_9bbf8527]
                                                    ).expect("479db858")
                                                )
                                            );
                                        }
                                        let el_4a00ab02 = #ident_where_ucc::#fi_ucc(
                                            #import::PgTypeWhere::new(
                                                #import::Oprtr::And,
                                                v_3432b965
                                            )
                                        );
                                        if !acc_f5866fb6.contains(&el_4a00ab02) {
                                            acc_f5866fb6.push(el_4a00ab02);
                                        }
                                    }
                                }
                            });
                            quote! {
                                match #import::NotEmptyUniqueVec::try_new({
                                    let mut acc_f5866fb6 = Vec::new();
                                    #(#ts)*
                                    acc_f5866fb6
                                }) {
                                    Ok(v_c4c01cd9) => Some(v_c4c01cd9),
                                    Err(er) => match er {
                                        #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("91d713b5")
                                    },
                                }
                            }
                        }
                        IsNullable::True => gen_nullable_ts(&ident_stdrt_not_null_ucc)
                    },
                    Pattern::Arr => match &is_nullable {
                        IsNullable::False => {
                            let ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let el_fi_ucc = ElSelfUcc::from_tokens(&fi);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&el0.type0);
                                quote! {
                                    for cr_34a1e540 in #CrSc.0.clone() {
                                        if let Some(v_51fe384b) = #ft_as_pg_json_test_cases_ts::#CrIntoPgJsonOptVecWhereLengthGreaterThanSc(
                                            cr_34a1e540.#fi
                                        ) {
                                            for el_4a00ab02 in v_51fe384b.clone().into_vec() {
                                                let el_938f8b34 = #ident_where_ucc::#el_fi_ucc(
                                                    #import::PgTypeWhere::try_new(
                                                        #import::Oprtr::And,
                                                        vec![el_4a00ab02]
                                                    )
                                                    .expect("955c6c27"),
                                                );
                                                if !acc_acceb7eb.contains(&el_938f8b34) {
                                                    acc_acceb7eb.push(el_938f8b34);
                                                }
                                            }
                                            let el_e17d9fba = #ident_where_ucc::#el_fi_ucc(
                                                #import::PgTypeWhere::new(
                                                    #import::Oprtr::And,
                                                    v_51fe384b
                                                )
                                            );
                                            if !acc_acceb7eb.contains(&el_e17d9fba) {
                                                acc_acceb7eb.push(el_e17d9fba);
                                            }
                                        }
                                    }
                                }
                            });
                            quote! {
                                match #import::NotEmptyUniqueVec::try_new({
                                    let mut acc_acceb7eb = Vec::new();
                                    #(#ts)*
                                    acc_acceb7eb.push(#ident_where_ucc::LengthGreaterThan(
                                        #import::PgJsonWhereLengthGreaterThan {
                                            oprtr: #import::Oprtr::And,
                                            #VSc: #import::UnsignedPartOfI32::try_from(
                                                i32::try_from(
                                                    #CrSc.0.len().checked_sub(1).unwrap_or_else(|| {
                                                        panic!("e411b8ca");
                                                    })
                                                ).expect("1fbbd897")
                                            ).expect("0eb5d915"),
                                        }
                                    ));
                                    acc_acceb7eb
                                }) {
                                    Ok(v_a889de37) => Some(v_a889de37),
                                    Err(er) => match er {
                                        #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("a9e99f81")
                                    },
                                }
                            }
                        }
                        IsNullable::True => gen_nullable_ts(&ident_arr_not_null_ucc)
                    },
                }
            };
            let (
                read_ids_and_cr_into_pg_json_opt_vec_where_greater_than_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_between_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_in_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_regex_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_contains_el_greater_than_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_contains_el_regex_ts
            ) = {
                let gen_ts = |ts: &dyn ToTokens|match &is_nullable {
                    IsNullable::False => match &pattern {
                        Pattern::Stdrt => {
                            let ts0 = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft = &el0.type0;
                                let fi_ucc = &ToTokensToUccTs::case_or_panic(&fi);
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                                quote! {
                                    if let Some(v_a2900ac9) = #ft_as_pg_json_test_cases_ts::#ts(
                                        #ReadIdsSc.0.#VSc.#fi,
                                        #CrSc.#fi
                                    ) {
                                        let and = #import::Oprtr::And;
                                        for el_3e86d33d in v_a2900ac9.clone().into_vec() {
                                            match el_3e86d33d {
                                                #import::SingleOrMultiple::Multiple(multiple) => {
                                                    acc_a94bd7fb.push(
                                                        #import::SingleOrMultiple::Single(
                                                            #ident_where_ucc::#fi_ucc(#import::PgTypeWhere::new(
                                                                and,
                                                                multiple
                                                            ))
                                                        )
                                                    );
                                                },
                                                #import::SingleOrMultiple::Single(single) => {
                                                    acc_a94bd7fb.push(
                                                        #import::SingleOrMultiple::Single(
                                                            #ident_where_ucc::#fi_ucc(#import::PgTypeWhere::try_new(
                                                                and,
                                                                vec![single]
                                                            ).expect("2635ede5"))
                                                        )
                                                    );
                                                },
                                            }
                                        }
                                        let v_3e75a2f2 = #import::SingleOrMultiple::Single(
                                            #ident_where_ucc::#fi_ucc(#import::PgTypeWhere::try_new(
                                                and,
                                                v_a2900ac9.into_vec().into_iter().flat_map(|el_9efefcdc| match el_9efefcdc {
                                                    #import::SingleOrMultiple::Multiple(multiple) => multiple.into_vec(),
                                                    #import::SingleOrMultiple::Single(single) => {
                                                        std::iter::once(single).collect()
                                                    }
                                                })
                                                .fold(Vec::new(), |mut acc_be2a6606, el_7ae146ee| {
                                                    if !acc_be2a6606.contains(&el_7ae146ee) {
                                                        acc_be2a6606.push(el_7ae146ee);
                                                    }
                                                    acc_be2a6606
                                                })
                                            ).expect("e3e5b4ab"))
                                        );
                                        if !acc_a94bd7fb.contains(&v_3e75a2f2) {
                                            acc_a94bd7fb.push(v_3e75a2f2);
                                        }
                                    }
                                }
                            });
                            quote! {
                                match #import::NotEmptyUniqueVec::try_new({
                                    let mut acc_a94bd7fb = Vec::new();
                                    #(#ts0)*
                                    acc_a94bd7fb
                                }) {
                                    Ok(v_ebe930f0) => Some(v_ebe930f0),
                                    Err(er) => match er {
                                        #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("b877e9c0")
                                    }
                                }
                            }
                        },
                        Pattern::Arr => {
                            let init_ts = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let ft = &el0.type0;
                                let ft_as_pg_json_test_cases_ts = gen_type_as_pg_json_test_cases_ts(&ft);
                                quote! {
                                    let #fi = #ft_as_pg_json_test_cases_ts::#ts(
                                        read_ids_629675e2.0.#VSc.#fi,
                                        cr_82796400.#fi
                                    );
                                }
                            });
                            let if_some_ts = {
                                let (last, rest) = vec_syn_field.split_last().expect("a8e7b6d6");
                                let gen_fi_is_some_ts = |fi: &Ident|quote!{#fi.is_some()};
                                let rest_ts = rest.iter().map(|el0| {
                                    let fi_is_some_ts = gen_fi_is_some_ts(&el0.ident);
                                    quote!{#fi_is_some_ts || }
                                });
                                let last_ts = gen_fi_is_some_ts(&last.ident);
                                quote! {#(#rest_ts)* #last_ts}
                            };
                            let ts0 = vec_syn_field.iter().map(|el0| {
                                let fi = &el0.ident;
                                let el_fi_ucc = ElSelfUcc::from_tokens(&fi);
                                quote! {
                                    if let Some(v_f190793e) = #fi {
                                        for el_22ac4087 in v_f190793e.clone().into_vec() {
                                            let where_f8a4319c = #ident_where_ucc::#el_fi_ucc(
                                                match el_22ac4087 {
                                                    #import::SingleOrMultiple::Multiple(multiple) => #import::PgTypeWhere::new(
                                                        and,
                                                        multiple.clone()
                                                    ),
                                                    #import::SingleOrMultiple::Single(single) => #import::PgTypeWhere::try_new(
                                                        and,
                                                        vec![single]
                                                    ).expect("2ed4dc5e"),
                                                }
                                            );
                                            all_fields_acc.push(where_f8a4319c.clone());
                                            match #import::NotEmptyUniqueVec::try_new(vec![
                                                #IdSc.clone(),
                                                where_f8a4319c
                                            ]) {
                                                Ok(v_fdd1b3eb) => {
                                                    let multiple_where_with_id_f8a4319c = #import::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                                    if !acc_359c0b3f.contains(&multiple_where_with_id_f8a4319c) {
                                                        acc_359c0b3f.push(multiple_where_with_id_f8a4319c);
                                                    }
                                                },
                                                Err(er) => match er {
                                                    #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                    #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("f0e3d01b")
                                                }
                                            }
                                        }
                                        match #import::NotEmptyUniqueVec::try_new(
                                            v_f190793e.into_vec().into_iter().flat_map(|el| match el {
                                                #import::SingleOrMultiple::Multiple(multiple) => multiple.into_vec(),
                                                #import::SingleOrMultiple::Single(single) => {
                                                    std::iter::once(single).collect()
                                                }
                                            })
                                            .fold(Vec::new(), |mut acc_01265629, el| {
                                                if !acc_01265629.contains(&el) {
                                                    acc_01265629.push(el);
                                                }
                                                acc_01265629
                                            })
                                        ) {
                                            Ok(v_a4000d70) => {
                                                let v_d6218307 = #ident_where_ucc::#el_fi_ucc(
                                                    #import::PgTypeWhere::new(
                                                        and,
                                                        v_a4000d70
                                                    )
                                                );
                                                if !all_fields_acc.contains(&v_d6218307) {
                                                    all_fields_acc.push(v_d6218307);
                                                }
                                            },
                                            Err(er) => match er {
                                                #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("f8fcc434")
                                            }
                                        }
                                    }
                                }
                            });
                            quote! {
                                match #import::NotEmptyUniqueVec::try_new({
                                    let mut acc_359c0b3f = Vec::new();
                                    for (read_ids_629675e2, cr_82796400) in #ReadIdsSc.0.#VSc.into_iter().zip(#CrSc.0) {
                                        let and = #import::Oprtr::And;
                                        let #IdSc = #ident_where_ucc::ElId(
                                            #import::PgTypeWhere::try_new(
                                                and,
                                                vec![
                                                    #uuid_uuid_as_not_null_jsonb_string_where_ucc::Equal(#import::PgJsonWhereEqual {
                                                        oprtr: #import::Oprtr::Or,
                                                        #VSc: #uuid_uuid_as_not_null_jsonb_string_table_type_ucc::new(
                                                            read_ids_629675e2.0.#VSc.#IdSc.0.#VSc
                                                        ),
                                                    })
                                                ],
                                            )
                                            .expect("31db8e1e"),
                                        );
                                        #(#init_ts)*
                                        if #if_some_ts {
                                            let mut all_fields_acc = vec![];
                                            #(#ts0)*
                                            match #import::NotEmptyUniqueVec::try_new({
                                                all_fields_acc.push(#IdSc);
                                                all_fields_acc
                                            }) {
                                                Ok(v_80199720) => {
                                                    acc_359c0b3f.push(#import::SingleOrMultiple::Multiple(v_80199720));
                                                },
                                                Err(er) => match er {
                                                    #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => (),
                                                    #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("32a3da97")
                                                }
                                            }
                                        }
                                    }
                                    acc_359c0b3f
                                }) {
                                    Ok(v_752f0e8d) => Some(v_752f0e8d),
                                    Err(er) => match er {
                                        #import::NotEmptyUniqueVecTryNewEr::IsEmpty {..} => None,
                                        #import::NotEmptyUniqueVecTryNewEr::NotUnique {..} => panic!("76542a11")
                                    }
                                }
                            }
                        }
                    },
                    IsNullable::True => {
                        let ident_ts_a8bc30fc = gen_type_as_pg_json_test_cases_ts(&gen_ident_ucc(&match &pattern {
                            Pattern::Stdrt => IdentPattern::StdrtNotNullWithoutId,
                            Pattern::Arr => IdentPattern::ArrNotNullWithId,
                        }));
                        quote! {
                            match (#ReadIdsSc.0.#VSc, #CrSc.0) {
                                (Some(read_ids_3e2e30c8), Some(cr_79039a2f)) => #ident_ts_a8bc30fc::#ts(
                                    read_ids_3e2e30c8,
                                    cr_79039a2f
                                ).map_or_else(|| None, |v_35662b3a| match #import::NotEmptyUniqueVec::try_new({
                                    let mut acc_e0d72451 = vec![];
                                    for el in v_35662b3a.into_vec() {
                                        match el {
                                            #import::SingleOrMultiple::Multiple(multiple) => {
                                                acc_e0d72451.push(#import::SingleOrMultiple::Single(#import::NullableJsonObjectPgTypeWhereFilter(Some(multiple))));
                                            },
                                            #import::SingleOrMultiple::Single(single) => match #import::NotEmptyUniqueVec::try_new(vec![single]) {
                                                Ok(v_4ce6ecd3) => {
                                                    acc_e0d72451.push(#import::SingleOrMultiple::Single(#import::NullableJsonObjectPgTypeWhereFilter(Some(v_4ce6ecd3))));
                                                }
                                                Err(er) => match er {
                                                    #import::NotEmptyUniqueVecTryNewEr::IsEmpty { .. } => (),
                                                    #import::NotEmptyUniqueVecTryNewEr::NotUnique { .. } => panic!("626ffa77"),
                                                },
                                            },
                                        }
                                    }
                                    acc_e0d72451
                                }) {
                                    Ok(v_5d381053) => Some(v_5d381053),
                                    Err(er) => match er {
                                        #import::NotEmptyUniqueVecTryNewEr::IsEmpty { .. } => None,
                                        #import::NotEmptyUniqueVecTryNewEr::NotUnique { .. } => panic!("23a17416"),
                                    },
                                }),
                                (Some(_), None) => panic!("994082bf"),
                                (None, Some(_)) => panic!("04f4d016"),
                                (None, None) => None,
                            }
                        }
                    }
                };
                (
                    gen_ts(
                        &ReadIdsAndCrIntoPgJsonOptVecWhereGreaterThanSc
                    ),
                    gen_ts(
                        &ReadIdsAndCrIntoPgJsonOptVecWhereBetweenSc
                    ),
                    gen_ts(
                        &ReadIdsAndCrIntoPgJsonOptVecWhereInSc
                    ),
                    gen_ts(
                        &ReadIdsAndCrIntoPgJsonOptVecWhereRegexSc
                    ),
                    gen_ts(
                        &ReadIdsAndCrIntoPgJsonOptVecWhereContainsElGreaterThanSc
                    ),
                    gen_ts(
                        &ReadIdsAndCrIntoPgJsonOptVecWhereContainsElRegexSc
                    )
                )
            };
            gen_impl_pg_json_test_cases_for_ident_ts(
                &cfg_feature_test_utils,
                &import,
                &ident_read_inn_ucc,
                &ident,
                &opt_vec_cr_ts,
                &read_ids_to_2_dims_vec_read_inn_ts,
                &read_inn_into_read_with_new_or_try_new_unwraped_ts,
                &read_inn_into_upd_with_new_or_try_new_unwraped_ts,
                &read_ids_into_opt_v_read_inn_ts,
                &upd_to_read_ids_ts,
                &read_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts,
                &previous_read_and_opt_upd_into_read_ts,
                &read_ids_and_cr_into_read_ts,
                &read_ids_and_cr_into_opt_v_read_ts,
                &read_ids_and_cr_into_table_type_ts,
                &read_ids_and_cr_into_where_equal_ts,
                &read_ids_and_cr_into_vec_where_equal_using_fields_ts,
                &read_ids_and_cr_into_vec_where_equal_to_json_field_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_dim_one_equal_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_dim_two_equal_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_dim_three_equal_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_dim_four_equal_ts,
                &cr_into_pg_json_opt_vec_where_length_equal_ts,
                &cr_into_pg_json_opt_vec_where_length_greater_than_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_greater_than_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_between_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_in_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_regex_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_contains_el_greater_than_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_contains_el_regex_ts,
            )
        };
        let impl_pg_type_test_cases_for_ident_ts = {
            let opt_vec_cr_ts = quote! {#self_as_pg_json_test_cases_ts::#OptVecCrSc()};
            let read_ids_to_2_dims_vec_read_inn_ts = quote! {#self_as_pg_json_test_cases_ts::#ReadIdsTo2DimsVecReadInnSc(#ReadIdsSc)};
            let read_inn_into_read_with_new_or_try_new_unwraped_ts = quote! {#self_as_pg_json_test_cases_ts::#ReadInnIntoReadWithNewOrTryNewUnwrapedSc(#VSc)};
            let read_inn_into_upd_with_new_or_try_new_unwraped_ts = quote! {#self_as_pg_json_test_cases_ts::#ReadInnIntoUpdWithNewOrTryNewUnwrapedSc(#VSc)};
            let upd_to_read_ids_ts = quote! {#self_as_pg_json_test_cases_ts::#UpdToReadIdsSc(#VSc)};
            let read_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts = quote! {#self_as_pg_json_test_cases_ts::#ReadIdsToOptVReadDfltOptSomeVecOneElSc(#VSc)};
            let previous_read_and_opt_upd_into_read_ts = quote! {#self_as_pg_json_test_cases_ts::#PreviousReadAndOptUpdIntoReadSc(#ReadSc, #OptUpdSc)};
            let read_ids_and_cr_into_read_ts = quote! {#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoReadSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_opt_v_read_ts = quote! {#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoOptVReadSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_table_type_ts = quote! {#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoTableTypeSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_where_equal_ts = quote! {#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoWhereEqualSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_vec_where_equal_using_fields_ts = quote! {#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoVecWhereEqualUsingFieldsSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_opt_vec_where_equal_to_json_field_ts = quote!{Some(#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoVecWhereEqualToJsonFieldSc(
                #ReadIdsSc,
                #CrSc
            ))};
            let cr_into_pg_type_opt_vec_where_dim_one_equal_ts = &none_ts;
            let pg_type_opt_vec_where_greater_than_test_ts = &none_ts;
            let read_ids_and_table_type_into_pg_type_opt_where_greater_than_ts = &none_ts;
            let (
                read_ids_and_cr_into_pg_json_opt_vec_where_dim_one_equal_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_dim_two_equal_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_dim_three_equal_ts,
                read_ids_and_cr_into_pg_json_opt_vec_where_dim_four_equal_ts
            ) = {
                let gen_ts = |dim: &Dim|{
                    let fn_ts = dim.read_ids_and_cr_into_pg_json_opt_vec_where_dim_nbr_equal_sc();
                    quote!{#self_as_pg_json_test_cases_ts::#fn_ts(
                        #ReadIdsSc,
                        #CrSc
                    )}
                };
                (
                    gen_ts(&Dim::One),
                    gen_ts(&Dim::Two),
                    gen_ts(&Dim::Three),
                    gen_ts(&Dim::Four)
                )
            };
            let cr_into_pg_json_opt_vec_where_length_equal_ts = quote!{#self_as_pg_json_test_cases_ts::#CrIntoPgJsonOptVecWhereLengthEqualSc(
                #CrSc
            )};
            let cr_into_pg_json_opt_vec_where_length_greater_than_ts = quote!{#self_as_pg_json_test_cases_ts::#CrIntoPgJsonOptVecWhereLengthGreaterThanSc(
                #CrSc
            )};
            let read_ids_and_cr_into_pg_json_opt_vec_where_greater_than_ts = quote!{#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoPgJsonOptVecWhereGreaterThanSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_pg_json_opt_vec_where_between_ts = quote!{#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoPgJsonOptVecWhereBetweenSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_pg_json_opt_vec_where_in_ts = quote!{#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoPgJsonOptVecWhereInSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_pg_json_opt_vec_where_regex_ts = quote!{#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoPgJsonOptVecWhereRegexSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_pg_json_opt_vec_where_contains_el_greater_than_ts = quote!{#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoPgJsonOptVecWhereContainsElGreaterThanSc(
                #ReadIdsSc,
                #CrSc
            )};
            let read_ids_and_cr_into_pg_json_opt_vec_where_contains_el_regex_ts = quote!{#self_as_pg_json_test_cases_ts::#ReadIdsAndCrIntoPgJsonOptVecWhereContainsElRegexSc(
                #ReadIdsSc,
                #CrSc
            )};
            gen_impl_pg_type_test_cases_for_ident_ts(
                &cfg_feature_test_utils,
                &import,
                &ident_read_inn_ucc,
                &ident,
                &opt_vec_cr_ts,
                &read_ids_to_2_dims_vec_read_inn_ts,
                &read_inn_into_read_with_new_or_try_new_unwraped_ts,
                &read_inn_into_upd_with_new_or_try_new_unwraped_ts,
                &upd_to_read_ids_ts,
                &read_ids_to_opt_v_read_dflt_opt_some_vec_one_el_ts,
                &previous_read_and_opt_upd_into_read_ts,
                &read_ids_and_cr_into_read_ts,
                &read_ids_and_cr_into_opt_v_read_ts,
                &read_ids_and_cr_into_table_type_ts,
                &read_ids_and_cr_into_where_equal_ts,
                &read_ids_and_cr_into_vec_where_equal_using_fields_ts,
                &read_ids_and_cr_into_opt_vec_where_equal_to_json_field_ts,
                &cr_into_pg_type_opt_vec_where_dim_one_equal_ts,
                &pg_type_opt_vec_where_greater_than_test_ts,
                &read_ids_and_table_type_into_pg_type_opt_where_greater_than_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_dim_one_equal_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_dim_two_equal_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_dim_three_equal_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_dim_four_equal_ts,
                &cr_into_pg_json_opt_vec_where_length_equal_ts,
                &cr_into_pg_json_opt_vec_where_length_greater_than_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_greater_than_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_between_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_in_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_regex_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_contains_el_greater_than_ts,
                &read_ids_and_cr_into_pg_json_opt_vec_where_contains_el_regex_ts,
            )
        };
        let impl_pg_type_not_pk_for_ident_ts = gen_impl_pg_type_not_pk_for_ident_ts(&import, &ident);
        let generated = quote! {
            #ident_ts
            #ident_table_type_ts
            #ident_cr_ts
            #ident_cr_for_query_ts
            #ident_select_ts
            #ident_where_ts
            #ident_read_ts
            #ident_read_ids_ts
            #ident_read_inn_ts
            #ident_upd_ts
            #ident_upd_for_query_ts
            #impl_pg_crud_pg_json_for_ident_ts
            #mb_impl_pg_crud_pg_types_pg_type_pg_type_ts
            #impl_pg_json_test_cases_for_ident_ts
            #impl_pg_type_test_cases_for_ident_ts
            #impl_pg_type_not_pk_for_ident_ts
        };
        (
            {
                let fi = format!("field_{i}").parse::<Ts2>().expect("7f9a06a5");
                quote! {
                    pub #fi: #ident,
                }
            },
            generated,
        )
    })
    .collect::<(Vec<Ts2>, Vec<Ts2>)>();
    mb_write_ts_into_file(
        gen_pg_json_obj_config.pg_table_columns_write_into_pg_table_columns_using_pg_json_objs,
        "pg_table_columns_using_pg_json_objects",
        &quote! {
            pub struct PgTableColumnsContentWriteIntoPgTableColumnsUsingPgJsonObjects {
                #(#fields_ts)*
            }
        },
        &FormatWithCargofmt::True,
    );
    let generated: Ts2 = {
        let ident_gen_pg_json_obj_mod = SelfGenPgJsonObjModSc::from_tokens(&di.ident);
        quote! {
            #[allow(unused_qualifications)]
            #[allow(clippy::absolute_paths)]
            mod #ident_gen_pg_json_obj_mod {
                #(#pg_json_obj_arr)*
            }
            pub use #ident_gen_pg_json_obj_mod::*;
        }
    };
    mb_write_ts_into_file(
        gen_pg_json_obj_config.whole_write_into_gen_pg_json_obj,
        "gen_pg_json_obj",
        &generated,
        &FormatWithCargofmt::True,
    );
    generated
}
